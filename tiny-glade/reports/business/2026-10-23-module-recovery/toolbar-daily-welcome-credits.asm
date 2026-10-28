
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140fc3550: 41 57                       	pushq	%r15
140fc3552: 41 56                       	pushq	%r14
140fc3554: 41 55                       	pushq	%r13
140fc3556: 41 54                       	pushq	%r12
140fc3558: 56                          	pushq	%rsi
140fc3559: 57                          	pushq	%rdi
140fc355a: 53                          	pushq	%rbx
140fc355b: 48 81 ec 80 00 00 00        	subq	$0x80, %rsp
140fc3562: 0f 29 74 24 70              	movaps	%xmm6, 0x70(%rsp)
140fc3567: 48 89 d6                    	movq	%rdx, %rsi
140fc356a: 48 89 cf                    	movq	%rcx, %rdi
140fc356d: 48 8b 19                    	movq	(%rcx), %rbx
140fc3570: 4c 8b 71 08                 	movq	0x8(%rcx), %r14
140fc3574: 48 89 5c 24 60              	movq	%rbx, 0x60(%rsp)
140fc3579: 4c 89 74 24 68              	movq	%r14, 0x68(%rsp)
140fc357e: 48 8d 4c 24 60              	leaq	0x60(%rsp), %rcx
140fc3583: e8 18 a8 5f ff              	callq	0x1405bdda0 <.text+0x5bcda0>
140fc3588: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140fc358d: 49 bc b2 80 93 1b 95 3d 60 dc       	movabsq	$-0x239fc26ae46c7f4e, %r12 # imm = 0xDC603D951B9380B2
140fc3597: 4c 89 64 24 28              	movq	%r12, 0x28(%rsp)
140fc359c: 49 bd a2 3d e1 ae f0 db 69 ce       	movabsq	$-0x3196240f511ec25e, %r13 # imm = 0xCE69DBF0AEE13DA2
140fc35a6: 4c 89 6c 24 30              	movq	%r13, 0x30(%rsp)
140fc35ab: 48 8d 4c 24 20              	leaq	0x20(%rsp), %rcx
140fc35b0: e8 6b a9 5f ff              	callq	0x1405bdf20 <.text+0x5bcf20>
140fc35b5: 48 8b 4e 08                 	movq	0x8(%rsi), %rcx
140fc35b9: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
140fc35be: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140fc35c3: 48 8d 4c 24 20              	leaq	0x20(%rsp), %rcx
140fc35c8: e8 43 a3 5f ff              	callq	0x1405bd910 <.text+0x5bc910>
140fc35cd: 4c 8d 7c 24 20              	leaq	0x20(%rsp), %r15
140fc35d2: 4c 89 f9                    	movq	%r15, %rcx
140fc35d5: 48 89 f2                    	movq	%rsi, %rdx
140fc35d8: 49 89 c0                    	movq	%rax, %r8
140fc35db: e8 d0 2b 6c ff              	callq	0x1406861b0 <.text+0x6851b0>
140fc35e0: 4c 89 f9                    	movq	%r15, %rcx
140fc35e3: 31 d2                       	xorl	%edx, %edx
140fc35e5: e8 46 a3 fd ff              	callq	0x140f9d930 <.text+0xf9c930>
140fc35ea: 48 89 c1                    	movq	%rax, %rcx
140fc35ed: 48 89 da                    	movq	%rbx, %rdx
140fc35f0: 4d 89 f0                    	movq	%r14, %r8
140fc35f3: e8 c8 8e fe ff              	callq	0x140fac4c0 <.text+0xfab4c0>
140fc35f8: 48 8b 44 24 30              	movq	0x30(%rsp), %rax
140fc35fd: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
140fc3602: 0f 10 44 24 20              	movups	0x20(%rsp), %xmm0
140fc3607: 0f 29 44 24 40              	movaps	%xmm0, 0x40(%rsp)
140fc360c: 48 8d 4c 24 40              	leaq	0x40(%rsp), %rcx
140fc3611: b2 01                       	movb	$0x1, %dl
140fc3613: e8 18 a3 fd ff              	callq	0x140f9d930 <.text+0xf9c930>
140fc3618: c7 80 58 02 00 00 ff 51 5a 7b       	movl	$0x7b5a51ff, 0x258(%rax) # imm = 0x7B5A51FF
140fc3622: 0f 28 44 24 40              	movaps	0x40(%rsp), %xmm0
140fc3627: 0f 29 44 24 20              	movaps	%xmm0, 0x20(%rsp)
140fc362c: 48 8b 44 24 50              	movq	0x50(%rsp), %rax
140fc3631: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
140fc3636: 48 8d 4c 24 20              	leaq	0x20(%rsp), %rcx
140fc363b: b2 01                       	movb	$0x1, %dl
140fc363d: e8 ee a2 fd ff              	callq	0x140f9d930 <.text+0xf9c930>
140fc3642: f3 0f 10 35 8a 9c a0 01     	movss	0x1a09c8a(%rip), %xmm6  # 0x1429cd2d4
140fc364a: 48 89 c1                    	movq	%rax, %rcx
140fc364d: 0f 28 ce                    	movaps	%xmm6, %xmm1
140fc3650: e8 fb 8a fe ff              	callq	0x140fac150 <.text+0xfab150>
140fc3655: 48 8b 44 24 30              	movq	0x30(%rsp), %rax
140fc365a: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
140fc365f: 0f 28 44 24 20              	movaps	0x20(%rsp), %xmm0
140fc3664: 0f 29 44 24 40              	movaps	%xmm0, 0x40(%rsp)
140fc3669: 48 8d 4c 24 40              	leaq	0x40(%rsp), %rcx
140fc366e: b2 01                       	movb	$0x1, %dl
140fc3670: e8 bb a2 fd ff              	callq	0x140f9d930 <.text+0xf9c930>
140fc3675: 48 89 c1                    	movq	%rax, %rcx
140fc3678: 66 ba 2c 01                 	movw	$0x12c, %dx             # imm = 0x12C
140fc367c: e8 1f d8 bc ff              	callq	0x140b90ea0 <.text+0xb8fea0>
140fc3681: 48 8b 44 24 50              	movq	0x50(%rsp), %rax
140fc3686: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
140fc368b: 0f 28 44 24 40              	movaps	0x40(%rsp), %xmm0
140fc3690: 0f 29 44 24 20              	movaps	%xmm0, 0x20(%rsp)
140fc3695: 48 8d 4c 24 20              	leaq	0x20(%rsp), %rcx
140fc369a: b2 01                       	movb	$0x1, %dl
140fc369c: e8 8f a2 fd ff              	callq	0x140f9d930 <.text+0xf9c930>
140fc36a1: 48 89 c1                    	movq	%rax, %rcx
140fc36a4: b2 02                       	movb	$0x2, %dl
140fc36a6: e8 95 d8 bc ff              	callq	0x140b90f40 <.text+0xb8ff40>
140fc36ab: 48 8b 44 24 30              	movq	0x30(%rsp), %rax
140fc36b0: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
140fc36b5: 0f 28 44 24 20              	movaps	0x20(%rsp), %xmm0
140fc36ba: 0f 29 44 24 40              	movaps	%xmm0, 0x40(%rsp)
140fc36bf: 48 8d 4c 24 40              	leaq	0x40(%rsp), %rcx
140fc36c4: b2 01                       	movb	$0x1, %dl
140fc36c6: e8 65 a2 fd ff              	callq	0x140f9d930 <.text+0xf9c930>
140fc36cb: 48 89 c1                    	movq	%rax, %rcx
140fc36ce: b2 03                       	movb	$0x3, %dl
140fc36d0: e8 ab d5 bc ff              	callq	0x140b90c80 <.text+0xb8fc80>
140fc36d5: 48 8b 47 10                 	movq	0x10(%rdi), %rax
140fc36d9: 48 8b 78 08                 	movq	0x8(%rax), %rdi
140fc36dd: 48 8b 58 10                 	movq	0x10(%rax), %rbx
140fc36e1: 48 89 7c 24 60              	movq	%rdi, 0x60(%rsp)
140fc36e6: 48 89 5c 24 68              	movq	%rbx, 0x68(%rsp)
140fc36eb: 48 8d 4c 24 60              	leaq	0x60(%rsp), %rcx
140fc36f0: e8 ab a6 5f ff              	callq	0x1405bdda0 <.text+0x5bcda0>
140fc36f5: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140fc36fa: 4c 89 64 24 28              	movq	%r12, 0x28(%rsp)
140fc36ff: 4c 89 6c 24 30              	movq	%r13, 0x30(%rsp)
140fc3704: 48 8d 4c 24 20              	leaq	0x20(%rsp), %rcx
140fc3709: e8 12 a8 5f ff              	callq	0x1405bdf20 <.text+0x5bcf20>
140fc370e: 48 8b 4e 08                 	movq	0x8(%rsi), %rcx
140fc3712: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
140fc3717: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140fc371c: 48 8d 4c 24 20              	leaq	0x20(%rsp), %rcx
140fc3721: e8 ea a1 5f ff              	callq	0x1405bd910 <.text+0x5bc910>
140fc3726: 4c 8d 74 24 20              	leaq	0x20(%rsp), %r14
140fc372b: 4c 89 f1                    	movq	%r14, %rcx
140fc372e: 48 89 f2                    	movq	%rsi, %rdx
140fc3731: 49 89 c0                    	movq	%rax, %r8
140fc3734: e8 77 2a 6c ff              	callq	0x1406861b0 <.text+0x6851b0>
140fc3739: 4c 89 f1                    	movq	%r14, %rcx
140fc373c: 31 d2                       	xorl	%edx, %edx
140fc373e: e8 ed a1 fd ff              	callq	0x140f9d930 <.text+0xf9c930>
140fc3743: 48 89 c1                    	movq	%rax, %rcx
140fc3746: 48 89 fa                    	movq	%rdi, %rdx
140fc3749: 49 89 d8                    	movq	%rbx, %r8
140fc374c: e8 6f 8d fe ff              	callq	0x140fac4c0 <.text+0xfab4c0>
140fc3751: 48 8b 44 24 30              	movq	0x30(%rsp), %rax
140fc3756: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
140fc375b: 0f 10 44 24 20              	movups	0x20(%rsp), %xmm0
140fc3760: 0f 29 44 24 40              	movaps	%xmm0, 0x40(%rsp)
140fc3765: 48 8d 4c 24 40              	leaq	0x40(%rsp), %rcx
140fc376a: b2 01                       	movb	$0x1, %dl
140fc376c: e8 bf a1 fd ff              	callq	0x140f9d930 <.text+0xf9c930>
140fc3771: c7 80 58 02 00 00 ff 51 5a 7b       	movl	$0x7b5a51ff, 0x258(%rax) # imm = 0x7B5A51FF
140fc377b: 0f 28 44 24 40              	movaps	0x40(%rsp), %xmm0
140fc3780: 0f 29 44 24 20              	movaps	%xmm0, 0x20(%rsp)
140fc3785: 48 8b 44 24 50              	movq	0x50(%rsp), %rax
140fc378a: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
140fc378f: 48 8d 4c 24 20              	leaq	0x20(%rsp), %rcx
140fc3794: b2 01                       	movb	$0x1, %dl
140fc3796: e8 95 a1 fd ff              	callq	0x140f9d930 <.text+0xf9c930>
140fc379b: 48 89 c1                    	movq	%rax, %rcx
140fc379e: 0f 28 ce                    	movaps	%xmm6, %xmm1
140fc37a1: e8 aa 89 fe ff              	callq	0x140fac150 <.text+0xfab150>
140fc37a6: 48 8b 44 24 30              	movq	0x30(%rsp), %rax
140fc37ab: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
140fc37b0: 0f 28 44 24 20              	movaps	0x20(%rsp), %xmm0
140fc37b5: 0f 29 44 24 40              	movaps	%xmm0, 0x40(%rsp)
140fc37ba: 48 8d 4c 24 40              	leaq	0x40(%rsp), %rcx
140fc37bf: b2 01                       	movb	$0x1, %dl
140fc37c1: e8 6a a1 fd ff              	callq	0x140f9d930 <.text+0xf9c930>
140fc37c6: 48 89 c1                    	movq	%rax, %rcx
140fc37c9: 66 ba 2c 01                 	movw	$0x12c, %dx             # imm = 0x12C
140fc37cd: e8 ce d6 bc ff              	callq	0x140b90ea0 <.text+0xb8fea0>
140fc37d2: 48 8b 44 24 50              	movq	0x50(%rsp), %rax
140fc37d7: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
140fc37dc: 0f 28 44 24 40              	movaps	0x40(%rsp), %xmm0
140fc37e1: 0f 29 44 24 20              	movaps	%xmm0, 0x20(%rsp)
140fc37e6: 48 8d 4c 24 20              	leaq	0x20(%rsp), %rcx
140fc37eb: b2 01                       	movb	$0x1, %dl
140fc37ed: e8 3e a1 fd ff              	callq	0x140f9d930 <.text+0xf9c930>
140fc37f2: 48 89 c1                    	movq	%rax, %rcx
140fc37f5: b2 02                       	movb	$0x2, %dl
140fc37f7: e8 44 d7 bc ff              	callq	0x140b90f40 <.text+0xb8ff40>
140fc37fc: 48 8b 44 24 30              	movq	0x30(%rsp), %rax
140fc3801: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
140fc3806: 0f 28 44 24 20              	movaps	0x20(%rsp), %xmm0
140fc380b: 0f 29 44 24 40              	movaps	%xmm0, 0x40(%rsp)
140fc3810: 48 8d 4c 24 40              	leaq	0x40(%rsp), %rcx
140fc3815: b2 01                       	movb	$0x1, %dl
140fc3817: e8 14 a1 fd ff              	callq	0x140f9d930 <.text+0xf9c930>
140fc381c: 48 89 c1                    	movq	%rax, %rcx
140fc381f: b2 03                       	movb	$0x3, %dl
140fc3821: e8 5a d4 bc ff              	callq	0x140b90c80 <.text+0xb8fc80>
140fc3826: 0f 28 74 24 70              	movaps	0x70(%rsp), %xmm6
140fc382b: 48 81 c4 80 00 00 00        	addq	$0x80, %rsp
140fc3832: 5b                          	popq	%rbx
140fc3833: 5f                          	popq	%rdi
140fc3834: 5e                          	popq	%rsi
140fc3835: 41 5c                       	popq	%r12
140fc3837: 41 5d                       	popq	%r13
140fc3839: 41 5e                       	popq	%r14
140fc383b: 41 5f                       	popq	%r15
140fc383d: c3                          	retq
