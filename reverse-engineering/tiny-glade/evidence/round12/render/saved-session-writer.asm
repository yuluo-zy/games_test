
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141328260: 55                          	pushq	%rbp
141328261: 41 56                       	pushq	%r14
141328263: 56                          	pushq	%rsi
141328264: 57                          	pushq	%rdi
141328265: 53                          	pushq	%rbx
141328266: 48 81 ec 00 04 00 00        	subq	$0x400, %rsp            # imm = 0x400
14132826d: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141328275: 0f 29 b5 70 03 00 00        	movaps	%xmm6, 0x370(%rbp)
14132827c: 48 c7 85 68 03 00 00 fe ff ff ff    	movq	$-0x2, 0x368(%rbp)
141328287: 48 89 ce                    	movq	%rcx, %rsi
14132828a: 49 8b 38                    	movq	(%r8), %rdi
14132828d: 48 85 ff                    	testq	%rdi, %rdi
141328290: 0f 84 ed 01 00 00           	je	0x141328483 <.text+0x1327483>
141328296: 48 8b 02                    	movq	(%rdx), %rax
141328299: 4c 8b 30                    	movq	(%rax), %r14
14132829c: 48 8d 4d b8                 	leaq	-0x48(%rbp), %rcx
1413282a0: 49 83 c0 08                 	addq	$0x8, %r8
1413282a4: 4c 89 c2                    	movq	%r8, %rdx
1413282a7: 41 b8 50 02 00 00           	movl	$0x250, %r8d            # imm = 0x250
1413282ad: e8 4e b9 5f 01              	callq	0x142923c00 <.text+0x2922c00>
1413282b2: 48 89 7d b0                 	movq	%rdi, -0x50(%rbp)
1413282b6: 48 8d 8d 10 03 00 00        	leaq	0x310(%rbp), %rcx
1413282bd: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
1413282c1: e8 6a 1e a3 ff              	callq	0x140d5a130 <.text+0xd59130>
1413282c6: 48 8d 8d 10 03 00 00        	leaq	0x310(%rbp), %rcx
1413282cd: e8 fe eb a4 ff              	callq	0x140d76ed0 <.text+0xd75ed0>
1413282d2: 49 89 d0                    	movq	%rdx, %r8
1413282d5: 48 8d 8d 88 02 00 00        	leaq	0x288(%rbp), %rcx
1413282dc: 48 89 c2                    	movq	%rax, %rdx
1413282df: e8 8c 0b a3 ff              	callq	0x140d58e70 <.text+0xd57e70>
1413282e4: f6 85 88 02 00 00 01        	testb	$0x1, 0x288(%rbp)
1413282eb: 0f 85 5b 01 00 00           	jne	0x14132844c <.text+0x132744c>
1413282f1: 48 8b 95 90 02 00 00        	movq	0x290(%rbp), %rdx
1413282f8: 4c 8b 85 98 02 00 00        	movq	0x298(%rbp), %r8
1413282ff: 48 8d 8d 88 02 00 00        	leaq	0x288(%rbp), %rcx
141328306: e8 25 7f 8e ff              	callq	0x140c10230 <.text+0xc0f230>
14132830b: 83 bd 88 02 00 00 08        	cmpl	$0x8, 0x288(%rbp)
141328312: 0f 85 34 01 00 00           	jne	0x14132844c <.text+0x132744c>
141328318: 0f 10 85 8c 02 00 00        	movups	0x28c(%rbp), %xmm0
14132831f: 0f 29 85 60 02 00 00        	movaps	%xmm0, 0x260(%rbp)
141328326: 48 8d 8d f0 02 00 00        	leaq	0x2f0(%rbp), %rcx
14132832d: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
141328331: e8 da 1b a3 ff              	callq	0x140d59f10 <.text+0xd58f10>
141328336: 48 8d 8d f0 02 00 00        	leaq	0x2f0(%rbp), %rcx
14132833d: e8 8e eb a4 ff              	callq	0x140d76ed0 <.text+0xd75ed0>
141328342: 48 89 c1                    	movq	%rax, %rcx
141328345: e8 c6 aa a3 ff              	callq	0x140d62e10 <.text+0xd61e10>
14132834a: 88 c3                       	movb	%al, %bl
14132834c: 48 8d 8d 88 02 00 00        	leaq	0x288(%rbp), %rcx
141328353: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
141328357: e8 b4 1b a3 ff              	callq	0x140d59f10 <.text+0xd58f10>
14132835c: 48 8d 8d 88 02 00 00        	leaq	0x288(%rbp), %rcx
141328363: e8 68 eb a4 ff              	callq	0x140d76ed0 <.text+0xd75ed0>
141328368: 49 89 d0                    	movq	%rdx, %r8
14132836b: 48 c7 44 24 20 0c 00 00 00  	movq	$0xc, 0x20(%rsp)
141328374: 4c 8d 0d 1d 24 91 01        	leaq	0x191241d(%rip), %r9    # 0x142c3a798
14132837b: 48 8d 8d 30 03 00 00        	leaq	0x330(%rbp), %rcx
141328382: 48 89 c2                    	movq	%rax, %rdx
141328385: e8 86 a4 a3 ff              	callq	0x140d62810 <.text+0xd61810>
14132838a: 48 8b 95 88 02 00 00        	movq	0x288(%rbp), %rdx
141328391: 48 85 d2                    	testq	%rdx, %rdx
141328394: 74 12                       	je	0x1413283a8 <.text+0x13273a8>
141328396: 48 8b 8d 90 02 00 00        	movq	0x290(%rbp), %rcx
14132839d: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1413283a3: e8 78 b8 2e ff              	callq	0x140613c20 <.text+0x612c20>
1413283a8: 48 8d 8d 30 03 00 00        	leaq	0x330(%rbp), %rcx
1413283af: e8 1c eb a4 ff              	callq	0x140d76ed0 <.text+0xd75ed0>
1413283b4: 48 89 c1                    	movq	%rax, %rcx
1413283b7: e8 34 a9 a3 ff              	callq	0x140d62cf0 <.text+0xd61cf0>
1413283bc: 40 88 c7                    	movb	%al, %dil
1413283bf: 0f 28 85 60 02 00 00        	movaps	0x260(%rbp), %xmm0
1413283c6: 0f 29 85 70 02 00 00        	movaps	%xmm0, 0x270(%rbp)
1413283cd: 48 8d 8d 30 03 00 00        	leaq	0x330(%rbp), %rcx
1413283d4: e8 f7 ea a4 ff              	callq	0x140d76ed0 <.text+0xd75ed0>
1413283d9: 49 89 d0                    	movq	%rdx, %r8
1413283dc: 48 8d 8d 88 02 00 00        	leaq	0x288(%rbp), %rcx
1413283e3: 48 89 c2                    	movq	%rax, %rdx
1413283e6: e8 25 23 a4 ff              	callq	0x140d6a710 <.text+0xd69710>
1413283eb: 48 8b 8d 88 02 00 00        	movq	0x288(%rbp), %rcx
1413283f2: 48 8b 85 90 02 00 00        	movq	0x290(%rbp), %rax
1413283f9: 48 83 f9 02                 	cmpq	$0x2, %rcx
1413283fd: 0f 85 a8 00 00 00           	jne	0x1413284ab <.text+0x13274ab>
141328403: 89 c1                       	movl	%eax, %ecx
141328405: 83 e1 03                    	andl	$0x3, %ecx
141328408: 83 f9 01                    	cmpl	$0x1, %ecx
14132840b: 0f 85 88 01 00 00           	jne	0x141328599 <.text+0x1327599>
141328411: 48 8d 48 ff                 	leaq	-0x1(%rax), %rcx
141328415: 48 89 8d 50 03 00 00        	movq	%rcx, 0x350(%rbp)
14132841c: 48 8b 48 ff                 	movq	-0x1(%rax), %rcx
141328420: 48 89 8d 58 03 00 00        	movq	%rcx, 0x358(%rbp)
141328427: 48 8b 40 07                 	movq	0x7(%rax), %rax
14132842b: 48 89 85 60 03 00 00        	movq	%rax, 0x360(%rbp)
141328432: 48 8b 00                    	movq	(%rax), %rax
141328435: 48 85 c0                    	testq	%rax, %rax
141328438: 0f 84 24 01 00 00           	je	0x141328562 <.text+0x1327562>
14132843e: 48 8b 8d 58 03 00 00        	movq	0x358(%rbp), %rcx
141328445: ff d0                       	callq	*%rax
141328447: e9 16 01 00 00              	jmp	0x141328562 <.text+0x1327562>
14132844c: c6 46 18 03                 	movb	$0x3, 0x18(%rsi)
141328450: 48 8b 95 10 03 00 00        	movq	0x310(%rbp), %rdx
141328457: 48 85 d2                    	testq	%rdx, %rdx
14132845a: 74 12                       	je	0x14132846e <.text+0x132746e>
14132845c: 48 8b 8d 18 03 00 00        	movq	0x318(%rbp), %rcx
141328463: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141328469: e8 b2 b7 2e ff              	callq	0x140613c20 <.text+0x612c20>
14132846e: 48 8b 45 b0                 	movq	-0x50(%rbp), %rax
141328472: f0                          	lock
141328473: 48 ff 08                    	decq	(%rax)
141328476: 75 1b                       	jne	0x141328493 <.text+0x1327493>
141328478: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
14132847c: e8 3f b7 a2 ff              	callq	0x140d53bc0 <.text+0xd52bc0>
141328481: eb 10                       	jmp	0x141328493 <.text+0x1327493>
141328483: 49 83 c0 08                 	addq	$0x8, %r8
141328487: 4c 89 c1                    	movq	%r8, %rcx
14132848a: e8 71 f5 2b ff              	callq	0x1405e7a00 <.text+0x5e6a00>
14132848f: c6 46 18 03                 	movb	$0x3, 0x18(%rsi)
141328493: 48 89 f0                    	movq	%rsi, %rax
141328496: 0f 28 b5 70 03 00 00        	movaps	0x370(%rbp), %xmm6
14132849d: 48 81 c4 00 04 00 00        	addq	$0x400, %rsp            # imm = 0x400
1413284a4: 5b                          	popq	%rbx
1413284a5: 5f                          	popq	%rdi
1413284a6: 5e                          	popq	%rsi
1413284a7: 41 5e                       	popq	%r14
1413284a9: 5d                          	popq	%rbp
1413284aa: c3                          	retq
1413284ab: 48 8b 95 d8 02 00 00        	movq	0x2d8(%rbp), %rdx
1413284b2: 48 89 95 58 02 00 00        	movq	%rdx, 0x258(%rbp)
1413284b9: 0f 10 85 98 02 00 00        	movups	0x298(%rbp), %xmm0
1413284c0: 0f 10 8d a8 02 00 00        	movups	0x2a8(%rbp), %xmm1
1413284c7: 0f 10 95 b8 02 00 00        	movups	0x2b8(%rbp), %xmm2
1413284ce: 0f 10 9d c8 02 00 00        	movups	0x2c8(%rbp), %xmm3
1413284d5: 0f 11 9d 48 02 00 00        	movups	%xmm3, 0x248(%rbp)
1413284dc: 0f 11 95 38 02 00 00        	movups	%xmm2, 0x238(%rbp)
1413284e3: 0f 11 8d 28 02 00 00        	movups	%xmm1, 0x228(%rbp)
1413284ea: 0f 11 85 18 02 00 00        	movups	%xmm0, 0x218(%rbp)
1413284f1: 48 89 85 10 02 00 00        	movq	%rax, 0x210(%rbp)
1413284f8: 48 89 8d 08 02 00 00        	movq	%rcx, 0x208(%rbp)
1413284ff: 48 8d 8d e0 02 00 00        	leaq	0x2e0(%rbp), %rcx
141328506: 48 8d 95 08 02 00 00        	leaq	0x208(%rbp), %rdx
14132850d: e8 4e 19 a3 ff              	callq	0x140d59e60 <.text+0xd58e60>
141328512: 83 bd e0 02 00 00 01        	cmpl	$0x1, 0x2e0(%rbp)
141328519: 0f 85 c3 00 00 00           	jne	0x1413285e2 <.text+0x13275e2>
14132851f: 48 8b 85 e8 02 00 00        	movq	0x2e8(%rbp), %rax
141328526: 89 c1                       	movl	%eax, %ecx
141328528: 83 e1 03                    	andl	$0x3, %ecx
14132852b: 83 f9 01                    	cmpl	$0x1, %ecx
14132852e: 75 69                       	jne	0x141328599 <.text+0x1327599>
141328530: 48 8d 48 ff                 	leaq	-0x1(%rax), %rcx
141328534: 48 89 8d 50 03 00 00        	movq	%rcx, 0x350(%rbp)
14132853b: 48 8b 48 ff                 	movq	-0x1(%rax), %rcx
14132853f: 48 89 8d 58 03 00 00        	movq	%rcx, 0x358(%rbp)
141328546: 48 8b 40 07                 	movq	0x7(%rax), %rax
14132854a: 48 89 85 60 03 00 00        	movq	%rax, 0x360(%rbp)
141328551: 48 8b 00                    	movq	(%rax), %rax
141328554: 48 85 c0                    	testq	%rax, %rax
141328557: 74 09                       	je	0x141328562 <.text+0x1327562>
141328559: 48 8b 8d 58 03 00 00        	movq	0x358(%rbp), %rcx
141328560: ff d0                       	callq	*%rax
141328562: 48 8b 8d 58 03 00 00        	movq	0x358(%rbp), %rcx
141328569: 48 8b 85 60 03 00 00        	movq	0x360(%rbp), %rax
141328570: 48 8b 50 08                 	movq	0x8(%rax), %rdx
141328574: 48 85 d2                    	testq	%rdx, %rdx
141328577: 74 09                       	je	0x141328582 <.text+0x1327582>
141328579: 4c 8b 40 10                 	movq	0x10(%rax), %r8
14132857d: e8 9e b6 2e ff              	callq	0x140613c20 <.text+0x612c20>
141328582: ba 18 00 00 00              	movl	$0x18, %edx
141328587: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14132858d: 48 8b 8d 50 03 00 00        	movq	0x350(%rbp), %rcx
141328594: e8 87 b6 2e ff              	callq	0x140613c20 <.text+0x612c20>
141328599: c6 46 18 03                 	movb	$0x3, 0x18(%rsi)
14132859d: 48 8b 95 30 03 00 00        	movq	0x330(%rbp), %rdx
1413285a4: 48 85 d2                    	testq	%rdx, %rdx
1413285a7: 74 12                       	je	0x1413285bb <.text+0x13275bb>
1413285a9: 48 8b 8d 38 03 00 00        	movq	0x338(%rbp), %rcx
1413285b0: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1413285b6: e8 65 b6 2e ff              	callq	0x140613c20 <.text+0x612c20>
1413285bb: 48 8b 95 f0 02 00 00        	movq	0x2f0(%rbp), %rdx
1413285c2: 48 85 d2                    	testq	%rdx, %rdx
1413285c5: 0f 84 85 fe ff ff           	je	0x141328450 <.text+0x1327450>
1413285cb: 48 8b 8d f8 02 00 00        	movq	0x2f8(%rbp), %rcx
1413285d2: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1413285d8: e8 43 b6 2e ff              	callq	0x140613c20 <.text+0x612c20>
1413285dd: e9 6e fe ff ff              	jmp	0x141328450 <.text+0x1327450>
1413285e2: f2 0f 10 b5 e4 02 00 00     	movsd	0x2e4(%rbp), %xmm6
1413285ea: 49 8b 16                    	movq	(%r14), %rdx
1413285ed: 48 8d 8d 70 02 00 00        	leaq	0x270(%rbp), %rcx
1413285f4: e8 87 74 77 ff              	callq	0x140a9fa80 <.text+0xa9ea80>
1413285f9: b1 03                       	movb	$0x3, %cl
1413285fb: 40 20 fb                    	andb	%dil, %bl
1413285fe: 74 10                       	je	0x141328610 <.text+0x1327610>
141328600: 0f 28 85 70 02 00 00        	movaps	0x270(%rbp), %xmm0
141328607: 0f 11 06                    	movups	%xmm0, (%rsi)
14132860a: 0f 13 76 10                 	movlps	%xmm6, 0x10(%rsi)
14132860e: 89 c1                       	movl	%eax, %ecx
141328610: 88 4e 18                    	movb	%cl, 0x18(%rsi)
141328613: 48 8b 95 30 03 00 00        	movq	0x330(%rbp), %rdx
14132861a: 48 85 d2                    	testq	%rdx, %rdx
14132861d: 75 8a                       	jne	0x1413285a9 <.text+0x13275a9>
14132861f: eb 9a                       	jmp	0x1413285bb <.text+0x13275bb>
141328621: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
141328630: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141328635: 55                          	pushq	%rbp
141328636: 41 56                       	pushq	%r14
141328638: 56                          	pushq	%rsi
141328639: 57                          	pushq	%rdi
14132863a: 53                          	pushq	%rbx
14132863b: 48 83 ec 40                 	subq	$0x40, %rsp
14132863f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141328646: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
14132864b: 48 8b 45 b0                 	movq	-0x50(%rbp), %rax
14132864f: f0                          	lock
141328650: 48 ff 08                    	decq	(%rax)
141328653: 75 09                       	jne	0x14132865e <.text+0x132765e>
141328655: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
141328659: e8 62 b5 a2 ff              	callq	0x140d53bc0 <.text+0xd52bc0>
14132865e: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
141328663: 48 83 c4 40                 	addq	$0x40, %rsp
141328667: 5b                          	popq	%rbx
141328668: 5f                          	popq	%rdi
141328669: 5e                          	popq	%rsi
14132866a: 41 5e                       	popq	%r14
14132866c: 5d                          	popq	%rbp
14132866d: c3                          	retq
14132866e: 66 90                       	nop
141328670: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141328675: 55                          	pushq	%rbp
141328676: 41 56                       	pushq	%r14
141328678: 56                          	pushq	%rsi
141328679: 57                          	pushq	%rdi
14132867a: 53                          	pushq	%rbx
14132867b: 48 83 ec 40                 	subq	$0x40, %rsp
14132867f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141328686: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
14132868b: 48 8b 95 10 03 00 00        	movq	0x310(%rbp), %rdx
141328692: 48 85 d2                    	testq	%rdx, %rdx
141328695: 74 12                       	je	0x1413286a9 <.text+0x13276a9>
141328697: 48 8b 8d 18 03 00 00        	movq	0x318(%rbp), %rcx
14132869e: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1413286a4: e8 77 b5 2e ff              	callq	0x140613c20 <.text+0x612c20>
1413286a9: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
1413286ae: 48 83 c4 40                 	addq	$0x40, %rsp
1413286b2: 5b                          	popq	%rbx
1413286b3: 5f                          	popq	%rdi
1413286b4: 5e                          	popq	%rsi
1413286b5: 41 5e                       	popq	%r14
1413286b7: 5d                          	popq	%rbp
1413286b8: c3                          	retq
1413286b9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1413286c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1413286c5: 55                          	pushq	%rbp
1413286c6: 41 56                       	pushq	%r14
1413286c8: 56                          	pushq	%rsi
1413286c9: 57                          	pushq	%rdi
1413286ca: 53                          	pushq	%rbx
1413286cb: 48 83 ec 40                 	subq	$0x40, %rsp
1413286cf: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1413286d6: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
1413286db: 48 8b 95 f0 02 00 00        	movq	0x2f0(%rbp), %rdx
1413286e2: 48 85 d2                    	testq	%rdx, %rdx
1413286e5: 74 12                       	je	0x1413286f9 <.text+0x13276f9>
1413286e7: 48 8b 8d f8 02 00 00        	movq	0x2f8(%rbp), %rcx
1413286ee: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1413286f4: e8 27 b5 2e ff              	callq	0x140613c20 <.text+0x612c20>
1413286f9: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
1413286fe: 48 83 c4 40                 	addq	$0x40, %rsp
141328702: 5b                          	popq	%rbx
141328703: 5f                          	popq	%rdi
141328704: 5e                          	popq	%rsi
141328705: 41 5e                       	popq	%r14
141328707: 5d                          	popq	%rbp
141328708: c3                          	retq
141328709: 0f 1f 80 00 00 00 00        	nopl	(%rax)
141328710: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141328715: 55                          	pushq	%rbp
141328716: 41 56                       	pushq	%r14
141328718: 56                          	pushq	%rsi
141328719: 57                          	pushq	%rdi
14132871a: 53                          	pushq	%rbx
14132871b: 48 83 ec 40                 	subq	$0x40, %rsp
14132871f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141328726: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
14132872b: 48 8b 95 88 02 00 00        	movq	0x288(%rbp), %rdx
141328732: 48 85 d2                    	testq	%rdx, %rdx
141328735: 74 12                       	je	0x141328749 <.text+0x1327749>
141328737: 48 8b 8d 90 02 00 00        	movq	0x290(%rbp), %rcx
14132873e: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141328744: e8 d7 b4 2e ff              	callq	0x140613c20 <.text+0x612c20>
141328749: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
14132874e: 48 83 c4 40                 	addq	$0x40, %rsp
141328752: 5b                          	popq	%rbx
141328753: 5f                          	popq	%rdi
141328754: 5e                          	popq	%rsi
141328755: 41 5e                       	popq	%r14
141328757: 5d                          	popq	%rbp
141328758: c3                          	retq
141328759: 0f 1f 80 00 00 00 00        	nopl	(%rax)
141328760: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141328765: 55                          	pushq	%rbp
141328766: 41 56                       	pushq	%r14
141328768: 56                          	pushq	%rsi
141328769: 57                          	pushq	%rdi
14132876a: 53                          	pushq	%rbx
14132876b: 48 83 ec 40                 	subq	$0x40, %rsp
14132876f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141328776: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
14132877b: 48 8b 95 30 03 00 00        	movq	0x330(%rbp), %rdx
141328782: 48 85 d2                    	testq	%rdx, %rdx
141328785: 74 12                       	je	0x141328799 <.text+0x1327799>
141328787: 48 8b 8d 38 03 00 00        	movq	0x338(%rbp), %rcx
14132878e: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141328794: e8 87 b4 2e ff              	callq	0x140613c20 <.text+0x612c20>
141328799: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
14132879e: 48 83 c4 40                 	addq	$0x40, %rsp
1413287a2: 5b                          	popq	%rbx
1413287a3: 5f                          	popq	%rdi
1413287a4: 5e                          	popq	%rsi
1413287a5: 41 5e                       	popq	%r14
1413287a7: 5d                          	popq	%rbp
1413287a8: c3                          	retq
1413287a9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1413287b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1413287b5: 55                          	pushq	%rbp
1413287b6: 41 56                       	pushq	%r14
1413287b8: 56                          	pushq	%rsi
1413287b9: 57                          	pushq	%rdi
1413287ba: 53                          	pushq	%rbx
1413287bb: 48 83 ec 40                 	subq	$0x40, %rsp
1413287bf: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1413287c6: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
1413287cb: 48 8b 8d 58 03 00 00        	movq	0x358(%rbp), %rcx
1413287d2: 48 8b 85 60 03 00 00        	movq	0x360(%rbp), %rax
1413287d9: 48 8b 50 08                 	movq	0x8(%rax), %rdx
1413287dd: 48 85 d2                    	testq	%rdx, %rdx
1413287e0: 74 10                       	je	0x1413287f2 <.text+0x13277f2>
1413287e2: 48 8b 85 60 03 00 00        	movq	0x360(%rbp), %rax
1413287e9: 4c 8b 40 10                 	movq	0x10(%rax), %r8
1413287ed: e8 2e b4 2e ff              	callq	0x140613c20 <.text+0x612c20>
1413287f2: ba 18 00 00 00              	movl	$0x18, %edx
1413287f7: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1413287fd: 48 8b 8d 50 03 00 00        	movq	0x350(%rbp), %rcx
141328804: e8 17 b4 2e ff              	callq	0x140613c20 <.text+0x612c20>
141328809: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
14132880e: 48 83 c4 40                 	addq	$0x40, %rsp
141328812: 5b                          	popq	%rbx
141328813: 5f                          	popq	%rdi
141328814: 5e                          	popq	%rsi
141328815: 41 5e                       	popq	%r14
141328817: 5d                          	popq	%rbp
141328818: c3                          	retq
141328819: 0f 1f 80 00 00 00 00        	nopl	(%rax)
141328820: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141328825: 55                          	pushq	%rbp
141328826: 41 56                       	pushq	%r14
141328828: 56                          	pushq	%rsi
141328829: 57                          	pushq	%rdi
14132882a: 53                          	pushq	%rbx
14132882b: 48 83 ec 40                 	subq	$0x40, %rsp
14132882f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141328836: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
14132883b: 48 8b 8d 58 03 00 00        	movq	0x358(%rbp), %rcx
141328842: 48 8b 85 60 03 00 00        	movq	0x360(%rbp), %rax
141328849: 48 8b 50 08                 	movq	0x8(%rax), %rdx
14132884d: 48 85 d2                    	testq	%rdx, %rdx
141328850: 74 10                       	je	0x141328862 <.text+0x1327862>
141328852: 48 8b 85 60 03 00 00        	movq	0x360(%rbp), %rax
141328859: 4c 8b 40 10                 	movq	0x10(%rax), %r8
14132885d: e8 be b3 2e ff              	callq	0x140613c20 <.text+0x612c20>
141328862: ba 18 00 00 00              	movl	$0x18, %edx
141328867: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14132886d: 48 8b 8d 50 03 00 00        	movq	0x350(%rbp), %rcx
141328874: e8 a7 b3 2e ff              	callq	0x140613c20 <.text+0x612c20>
141328879: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
14132887e: 48 83 c4 40                 	addq	$0x40, %rsp
141328882: 5b                          	popq	%rbx
141328883: 5f                          	popq	%rdi
141328884: 5e                          	popq	%rsi
141328885: 41 5e                       	popq	%r14
141328887: 5d                          	popq	%rbp
141328888: c3                          	retq
