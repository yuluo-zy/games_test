
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1426235d0: 41 57                       	pushq	%r15
1426235d2: 41 56                       	pushq	%r14
1426235d4: 41 55                       	pushq	%r13
1426235d6: 41 54                       	pushq	%r12
1426235d8: 56                          	pushq	%rsi
1426235d9: 57                          	pushq	%rdi
1426235da: 55                          	pushq	%rbp
1426235db: 53                          	pushq	%rbx
1426235dc: 48 81 ec 08 02 00 00        	subq	$0x208, %rsp            # imm = 0x208
1426235e3: 0f 29 b4 24 f0 01 00 00     	movaps	%xmm6, 0x1f0(%rsp)
1426235eb: 4c 89 cf                    	movq	%r9, %rdi
1426235ee: 44 89 c3                    	movl	%r8d, %ebx
1426235f1: 49 89 d6                    	movq	%rdx, %r14
1426235f4: 48 89 ce                    	movq	%rcx, %rsi
1426235f7: 66 c7 84 24 d8 01 00 00 00 00       	movw	$0x0, 0x1d8(%rsp)
142623601: 0f 10 05 30 f1 59 00        	movups	0x59f130(%rip), %xmm0   # 0x142bc2738
142623608: 0f 11 84 24 48 01 00 00     	movups	%xmm0, 0x148(%rsp)
142623610: 0f 10 05 31 f1 59 00        	movups	0x59f131(%rip), %xmm0   # 0x142bc2748
142623617: 0f 11 84 24 58 01 00 00     	movups	%xmm0, 0x158(%rsp)
14262361f: 0f 10 05 32 f1 59 00        	movups	0x59f132(%rip), %xmm0   # 0x142bc2758
142623626: 0f 11 84 24 68 01 00 00     	movups	%xmm0, 0x168(%rsp)
14262362e: 0f 10 05 33 f1 59 00        	movups	0x59f133(%rip), %xmm0   # 0x142bc2768
142623635: 0f 11 84 24 78 01 00 00     	movups	%xmm0, 0x178(%rsp)
14262363d: 0f 10 05 34 f1 59 00        	movups	0x59f134(%rip), %xmm0   # 0x142bc2778
142623644: 0f 11 84 24 88 01 00 00     	movups	%xmm0, 0x188(%rsp)
14262364c: 48 c7 84 24 f0 00 00 00 00 00 00 00 	movq	$0x0, 0xf0(%rsp)
142623658: 0f 10 05 29 f1 59 00        	movups	0x59f129(%rip), %xmm0   # 0x142bc2788
14262365f: 0f 11 84 24 98 01 00 00     	movups	%xmm0, 0x198(%rsp)
142623667: 0f 10 05 2a f1 59 00        	movups	0x59f12a(%rip), %xmm0   # 0x142bc2798
14262366e: 0f 11 84 24 a8 01 00 00     	movups	%xmm0, 0x1a8(%rsp)
142623676: 0f 10 05 7b f0 59 00        	movups	0x59f07b(%rip), %xmm0   # 0x142bc26f8
14262367d: 0f 11 84 24 08 01 00 00     	movups	%xmm0, 0x108(%rsp)
142623685: 0f 10 05 5c f0 59 00        	movups	0x59f05c(%rip), %xmm0   # 0x142bc26e8
14262368c: 0f 11 84 24 f8 00 00 00     	movups	%xmm0, 0xf8(%rsp)
142623694: 0f 10 05 7d f0 59 00        	movups	0x59f07d(%rip), %xmm0   # 0x142bc2718
14262369b: 0f 11 84 24 28 01 00 00     	movups	%xmm0, 0x128(%rsp)
1426236a3: 0f 10 05 5e f0 59 00        	movups	0x59f05e(%rip), %xmm0   # 0x142bc2708
1426236aa: 0f 11 84 24 18 01 00 00     	movups	%xmm0, 0x118(%rsp)
1426236b2: c7 84 24 dd 01 00 00 04 07 07 07    	movl	$0x7070704, 0x1dd(%rsp) # imm = 0x7070704
1426236bd: 66 c7 84 24 e1 01 00 00 09 04       	movw	$0x409, 0x1e1(%rsp) # imm = 0x409
1426236c7: 0f 10 05 5a f0 59 00        	movups	0x59f05a(%rip), %xmm0   # 0x142bc2728
1426236ce: 0f 11 84 24 38 01 00 00     	movups	%xmm0, 0x138(%rsp)
1426236d6: 66 c7 84 24 da 01 00 00 00 00       	movw	$0x0, 0x1da(%rsp)
1426236e0: 48 c7 84 24 b8 01 00 00 02 00 00 00 	movq	$0x2, 0x1b8(%rsp)
1426236ec: f2 0f 10 05 bc 0e 31 00     	movsd	0x310ebc(%rip), %xmm0   # 0x1429345b0
1426236f4: f2 0f 11 84 24 d0 01 00 00  	movsd	%xmm0, 0x1d0(%rsp)
1426236fd: 0f 10 05 7c ef 59 00        	movups	0x59ef7c(%rip), %xmm0   # 0x142bc2680
142623704: 0f 29 84 24 90 00 00 00     	movaps	%xmm0, 0x90(%rsp)
14262370c: 48 8b 05 7d ef 59 00        	movq	0x59ef7d(%rip), %rax    # 0x142bc2690
142623713: 48 89 84 24 a0 00 00 00     	movq	%rax, 0xa0(%rsp)
14262371b: 48 8b 05 86 ef 59 00        	movq	0x59ef86(%rip), %rax    # 0x142bc26a8
142623722: 48 89 84 24 b8 00 00 00     	movq	%rax, 0xb8(%rsp)
14262372a: 0f 10 05 67 ef 59 00        	movups	0x59ef67(%rip), %xmm0   # 0x142bc2698
142623731: 0f 11 84 24 a8 00 00 00     	movups	%xmm0, 0xa8(%rsp)
142623739: 48 8b 05 80 ef 59 00        	movq	0x59ef80(%rip), %rax    # 0x142bc26c0
142623740: 48 89 84 24 d0 00 00 00     	movq	%rax, 0xd0(%rsp)
142623748: 0f 10 05 61 ef 59 00        	movups	0x59ef61(%rip), %xmm0   # 0x142bc26b0
14262374f: 0f 29 84 24 c0 00 00 00     	movaps	%xmm0, 0xc0(%rsp)
142623757: 48 8b 05 7a ef 59 00        	movq	0x59ef7a(%rip), %rax    # 0x142bc26d8
14262375e: 48 89 84 24 e8 00 00 00     	movq	%rax, 0xe8(%rsp)
142623766: 0f 10 05 5b ef 59 00        	movups	0x59ef5b(%rip), %xmm0   # 0x142bc26c8
14262376d: 0f 11 84 24 d8 00 00 00     	movups	%xmm0, 0xd8(%rsp)
142623775: c6 84 24 dc 01 00 00 00     	movb	$0x0, 0x1dc(%rsp)
14262377d: 0f 57 c0                    	xorps	%xmm0, %xmm0
142623780: 0f 29 84 24 c0 01 00 00     	movaps	%xmm0, 0x1c0(%rsp)
142623788: 48 8d 4c 24 70              	leaq	0x70(%rsp), %rcx
14262378d: 4c 8d 84 24 90 00 00 00     	leaq	0x90(%rsp), %r8
142623795: e8 66 3e 9c fe              	callq	0x140fe7600 <.text+0xfe6600>
14262379a: 83 7c 24 70 04              	cmpl	$0x4, 0x70(%rsp)
14262379f: 0f 85 6b 04 00 00           	jne	0x142623c10 <.text+0x2622c10>
1426237a5: 8b 6c 24 74                 	movl	0x74(%rsp), %ebp
1426237a9: 44 8b 6c 24 78              	movl	0x78(%rsp), %r13d
1426237ae: 44 8b 46 20                 	movl	0x20(%rsi), %r8d
1426237b2: 44 8b 4e 24                 	movl	0x24(%rsi), %r9d
1426237b6: 44 89 6c 24 28              	movl	%r13d, 0x28(%rsp)
1426237bb: 89 6c 24 20                 	movl	%ebp, 0x20(%rsp)
1426237bf: 48 8d 4c 24 40              	leaq	0x40(%rsp), %rcx
1426237c4: 4c 89 f2                    	movq	%r14, %rdx
1426237c7: e8 c4 3f 9c fe              	callq	0x140fe7790 <.text+0xfe6790>
1426237cc: 83 7c 24 40 04              	cmpl	$0x4, 0x40(%rsp)
1426237d1: 0f 85 79 04 00 00           	jne	0x142623c50 <.text+0x2622c50>
1426237d7: 8b 84 24 88 02 00 00        	movl	0x288(%rsp), %eax
1426237de: 4c 8b b4 24 70 02 00 00     	movq	0x270(%rsp), %r14
1426237e6: f3 0f 10 35 0e ab a2 00     	movss	0xa2ab0e(%rip), %xmm6   # 0x14304e2fc
1426237ee: a8 01                       	testb	$0x1, %al
1426237f0: 74 09                       	je	0x1426237fb <.text+0x26227fb>
1426237f2: f3 0f 59 b4 24 90 02 00 00  	mulss	0x290(%rsp), %xmm6
1426237fb: 0f 57 c0                    	xorps	%xmm0, %xmm0
1426237fe: 0f 11 84 24 c0 00 00 00     	movups	%xmm0, 0xc0(%rsp)
142623806: 4d 85 f6                    	testq	%r14, %r14
142623809: 0f 88 87 04 00 00           	js	0x142623c96 <.text+0x2622c96>
14262380f: 74 28                       	je	0x142623839 <.text+0x2622839>
142623811: 0f b6 05 f1 13 01 01        	movzbl	0x10113f1(%rip), %eax   # 0x143634c09
142623818: 41 bc 01 00 00 00           	movl	$0x1, %r12d
14262381e: ba 01 00 00 00              	movl	$0x1, %edx
142623823: 4c 89 f1                    	movq	%r14, %rcx
142623826: e8 e5 03 ff fd              	callq	0x140613c10 <.text+0x612c10>
14262382b: 48 85 c0                    	testq	%rax, %rax
14262382e: 0f 84 65 04 00 00           	je	0x142623c99 <.text+0x2622c99>
142623834: 49 89 c7                    	movq	%rax, %r15
142623837: eb 06                       	jmp	0x14262383f <.text+0x262283f>
142623839: 41 bf 01 00 00 00           	movl	$0x1, %r15d
14262383f: 4c 8b a4 24 78 02 00 00     	movq	0x278(%rsp), %r12
142623847: 4c 89 f9                    	movq	%r15, %rcx
14262384a: 48 89 fa                    	movq	%rdi, %rdx
14262384d: 4d 89 f0                    	movq	%r14, %r8
142623850: e8 ab 03 30 00              	callq	0x142923c00 <.text+0x2922c00>
142623855: 4c 89 b4 24 90 00 00 00     	movq	%r14, 0x90(%rsp)
14262385d: 4c 89 bc 24 98 00 00 00     	movq	%r15, 0x98(%rsp)
142623865: 4c 89 b4 24 a0 00 00 00     	movq	%r14, 0xa0(%rsp)
14262386d: c7 84 24 a8 00 00 00 00 00 00 00    	movl	$0x0, 0xa8(%rsp)
142623878: 89 6c 24 38                 	movl	%ebp, 0x38(%rsp)
14262387c: 89 ac 24 d0 00 00 00        	movl	%ebp, 0xd0(%rsp)
142623883: 44 89 ac 24 d4 00 00 00     	movl	%r13d, 0xd4(%rsp)
14262388b: f3 0f 11 b4 24 d8 00 00 00  	movss	%xmm6, 0xd8(%rsp)
142623894: f3 0f 11 b4 24 dc 00 00 00  	movss	%xmm6, 0xdc(%rsp)
14262389d: 66 c7 84 24 e8 00 00 00 01 00       	movw	$0x1, 0xe8(%rsp)
1426238a7: c6 84 24 ea 00 00 00 00     	movb	$0x0, 0xea(%rsp)
1426238af: f3 0f 10 05 4d 20 30 00     	movss	0x30204d(%rip), %xmm0   # 0x142925904
1426238b7: 0f 13 84 24 b0 00 00 00     	movlps	%xmm0, 0xb0(%rsp)
1426238bf: c7 84 24 b8 00 00 00 00 00 00 00    	movl	$0x0, 0xb8(%rsp)
1426238ca: c6 84 24 bc 00 00 00 02     	movb	$0x2, 0xbc(%rsp)
1426238d2: f2 0f 10 05 36 bd 96 00     	movsd	0x96bd36(%rip), %xmm0   # 0x142f8f610
1426238da: f2 0f 11 84 24 e0 00 00 00  	movsd	%xmm0, 0xe0(%rsp)
1426238e3: 48 c7 84 24 f0 00 00 00 00 00 00 00 	movq	$0x0, 0xf0(%rsp)
1426238ef: 4c 8d 0d ca bd 96 00        	leaq	0x96bdca(%rip), %r9     # 0x142f8f6c0
1426238f6: 48 8d 4c 24 40              	leaq	0x40(%rsp), %rcx
1426238fb: 4c 8d 84 24 90 00 00 00     	leaq	0x90(%rsp), %r8
142623903: 4c 89 e2                    	movq	%r12, %rdx
142623906: e8 65 cb 9b fe              	callq	0x140fe0470 <.text+0xfdf470>
14262390b: 48 8b 7c 24 48              	movq	0x48(%rsp), %rdi
142623910: 48 85 ff                    	testq	%rdi, %rdi
142623913: 75 05                       	jne	0x14262391a <.text+0x262291a>
142623915: 48 8b 7c 24 50              	movq	0x50(%rsp), %rdi
14262391a: 44 89 6c 24 3c              	movl	%r13d, 0x3c(%rsp)
14262391f: 48 8b ac 24 80 02 00 00     	movq	0x280(%rsp), %rbp
142623927: 4c 8b 64 24 60              	movq	0x60(%rsp), %r12
14262392c: 48 8b 07                    	movq	(%rdi), %rax
14262392f: 4c 8b 77 10                 	movq	0x10(%rdi), %r14
142623933: 4c 29 f0                    	subq	%r14, %rax
142623936: 48 83 f8 1f                 	cmpq	$0x1f, %rax
14262393a: 0f 86 f8 01 00 00           	jbe	0x142623b38 <.text+0x2622b38>
142623940: 48 8b 47 08                 	movq	0x8(%rdi), %rax
142623944: 48 8d 0d 55 b2 9b fe        	leaq	-0x1644dab(%rip), %rcx  # 0x140fdeba0 <.text+0xfddba0>
14262394b: 4a 89 0c 30                 	movq	%rcx, (%rax,%r14)
14262394f: 4c 8d 2d 0a bf 82 ff        	leaq	-0x7d40f6(%rip), %r13   # 0x141e4f860 <.text+0x1e4e860>
142623956: 4e 89 6c 30 08              	movq	%r13, 0x8(%rax,%r14)
14262395b: 4e 89 64 30 10              	movq	%r12, 0x10(%rax,%r14)
142623960: 42 c7 44 30 18 00 00 00 00  	movl	$0x0, 0x18(%rax,%r14)
142623969: 42 c6 44 30 1c 00           	movb	$0x0, 0x1c(%rax,%r14)
14262396f: 4d 8d 7e 20                 	leaq	0x20(%r14), %r15
142623973: 4c 89 7f 10                 	movq	%r15, 0x10(%rdi)
142623977: 0f b6 6d 00                 	movzbl	(%rbp), %ebp
14262397b: 83 fd 05                    	cmpl	$0x5, %ebp
14262397e: 74 27                       	je	0x1426239a7 <.text+0x26229a7>
142623980: 83 fd 08                    	cmpl	$0x8, %ebp
142623983: 74 22                       	je	0x1426239a7 <.text+0x26229a7>
142623985: 83 fd 0b                    	cmpl	$0xb, %ebp
142623988: 75 5a                       	jne	0x1426239e4 <.text+0x26229e4>
14262398a: 48 8b 07                    	movq	(%rdi), %rax
14262398d: 4c 29 f8                    	subq	%r15, %rax
142623990: 48 83 f8 1f                 	cmpq	$0x1f, %rax
142623994: 0f 86 51 02 00 00           	jbe	0x142623beb <.text+0x2622beb>
14262399a: 48 8b 47 08                 	movq	0x8(%rdi), %rax
14262399e: 48 8d 0d fb b2 9b fe        	leaq	-0x1644d05(%rip), %rcx  # 0x140fdeca0 <.text+0xfddca0>
1426239a5: eb 27                       	jmp	0x1426239ce <.text+0x26229ce>
1426239a7: 48 8b 05 ea 86 00 01        	movq	0x10086ea(%rip), %rax   # 0x14362c098
1426239ae: 80 38 00                    	cmpb	$0x0, (%rax)
1426239b1: 74 31                       	je	0x1426239e4 <.text+0x26229e4>
1426239b3: 48 8b 07                    	movq	(%rdi), %rax
1426239b6: 4c 29 f8                    	subq	%r15, %rax
1426239b9: 48 83 f8 1f                 	cmpq	$0x1f, %rax
1426239bd: 0f 86 03 02 00 00           	jbe	0x142623bc6 <.text+0x2622bc6>
1426239c3: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1426239c7: 48 8d 0d 32 bb 9b fe        	leaq	-0x16444ce(%rip), %rcx  # 0x140fdf500 <.text+0xfde500>
1426239ce: 4a 89 0c 38                 	movq	%rcx, (%rax,%r15)
1426239d2: 4e 89 6c 38 08              	movq	%r13, 0x8(%rax,%r15)
1426239d7: 4e 89 64 38 10              	movq	%r12, 0x10(%rax,%r15)
1426239dc: 42 c6 44 38 18 00           	movb	$0x0, 0x18(%rax,%r15)
1426239e2: eb 34                       	jmp	0x142623a18 <.text+0x2622a18>
1426239e4: 48 8b 07                    	movq	(%rdi), %rax
1426239e7: 4c 29 f8                    	subq	%r15, %rax
1426239ea: 48 83 f8 1f                 	cmpq	$0x1f, %rax
1426239ee: 0f 86 ad 01 00 00           	jbe	0x142623ba1 <.text+0x2622ba1>
1426239f4: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1426239f8: 48 8d 0d 01 a7 9b fe        	leaq	-0x16458ff(%rip), %rcx  # 0x140fde100 <.text+0xfdd100>
1426239ff: 4a 89 0c 38                 	movq	%rcx, (%rax,%r15)
142623a03: 4e 89 6c 38 08              	movq	%r13, 0x8(%rax,%r15)
142623a08: 4e 89 64 38 10              	movq	%r12, 0x10(%rax,%r15)
142623a0d: 42 c6 44 38 18 00           	movb	$0x0, 0x18(%rax,%r15)
142623a13: 42 88 6c 38 19              	movb	%bpl, 0x19(%rax,%r15)
142623a18: 8b 6c 24 38                 	movl	0x38(%rsp), %ebp
142623a1c: 49 8d 56 40                 	leaq	0x40(%r14), %rdx
142623a20: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
142623a24: 48 8b 07                    	movq	(%rdi), %rax
142623a27: 48 29 d0                    	subq	%rdx, %rax
142623a2a: 48 83 f8 1f                 	cmpq	$0x1f, %rax
142623a2e: 0f 86 29 01 00 00           	jbe	0x142623b5d <.text+0x2622b5d>
142623a34: 48 8b 47 08                 	movq	0x8(%rdi), %rax
142623a38: 48 8d 0d c1 b5 9b fe        	leaq	-0x1644a3f(%rip), %rcx  # 0x140fdf000 <.text+0xfde000>
142623a3f: 4a 89 4c 30 40              	movq	%rcx, 0x40(%rax,%r14)
142623a44: 4e 89 6c 30 48              	movq	%r13, 0x48(%rax,%r14)
142623a49: 4e 89 64 30 50              	movq	%r12, 0x50(%rax,%r14)
142623a4e: 42 c6 44 30 58 00           	movb	$0x0, 0x58(%rax,%r14)
142623a54: 49 8d 56 60                 	leaq	0x60(%r14), %rdx
142623a58: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
142623a5c: 48 8b 07                    	movq	(%rdi), %rax
142623a5f: 48 29 d0                    	subq	%rdx, %rax
142623a62: 48 83 f8 1f                 	cmpq	$0x1f, %rax
142623a66: 0f 86 13 01 00 00           	jbe	0x142623b7f <.text+0x2622b7f>
142623a6c: 48 8b 47 08                 	movq	0x8(%rdi), %rax
142623a70: 48 8d 0d 49 ae 9b fe        	leaq	-0x16451b7(%rip), %rcx  # 0x140fde8c0 <.text+0xfdd8c0>
142623a77: 4a 89 4c 30 60              	movq	%rcx, 0x60(%rax,%r14)
142623a7c: 4e 89 6c 30 68              	movq	%r13, 0x68(%rax,%r14)
142623a81: 4e 89 64 30 70              	movq	%r12, 0x70(%rax,%r14)
142623a86: 42 c6 44 30 78 00           	movb	$0x0, 0x78(%rax,%r14)
142623a8c: 49 83 ee 80                 	subq	$-0x80, %r14
142623a90: 4c 89 77 10                 	movq	%r14, 0x10(%rdi)
142623a94: 48 8b 46 08                 	movq	0x8(%rsi), %rax
142623a98: 48 8b 7e 10                 	movq	0x10(%rsi), %rdi
142623a9c: 48 89 f9                    	movq	%rdi, %rcx
142623a9f: 48 c1 e1 05                 	shlq	$0x5, %rcx
142623aa3: 48 89 c2                    	movq	%rax, %rdx
142623aa6: 44 8b 74 24 3c              	movl	0x3c(%rsp), %r14d
142623aab: 0f 1f 44 00 00              	nopl	(%rax,%rax)
142623ab0: 48 85 c9                    	testq	%rcx, %rcx
142623ab3: 74 28                       	je	0x142623add <.text+0x2622add>
142623ab5: 4c 8d 42 20                 	leaq	0x20(%rdx), %r8
142623ab9: 48 83 c1 e0                 	addq	$-0x20, %rcx
142623abd: 38 5a 18                    	cmpb	%bl, 0x18(%rdx)
142623ac0: 4c 89 c2                    	movq	%r8, %rdx
142623ac3: 75 eb                       	jne	0x142623ab0 <.text+0x2622ab0>
142623ac5: 48 8d 0d 0c 5d b2 00        	leaq	0xb25d0c(%rip), %rcx    # 0x1431497d8
142623acc: 4c 8d 05 65 5d b2 00        	leaq	0xb25d65(%rip), %r8     # 0x143149838
142623ad3: ba 59 00 00 00              	movl	$0x59, %edx
142623ad8: e8 53 59 2b 00              	callq	0x1428d9430 <.text+0x28d8430>
142623add: 48 3b 3e                    	cmpq	(%rsi), %rdi
142623ae0: 75 13                       	jne	0x142623af5 <.text+0x2622af5>
142623ae2: 48 8d 15 d7 5c b2 00        	leaq	0xb25cd7(%rip), %rdx    # 0x1431497c0
142623ae9: 48 89 f1                    	movq	%rsi, %rcx
142623aec: e8 3f f1 f3 fd              	callq	0x140562c30 <.text+0x561c30>
142623af1: 48 8b 46 08                 	movq	0x8(%rsi), %rax
142623af5: 48 89 f9                    	movq	%rdi, %rcx
142623af8: 48 c1 e1 05                 	shlq	$0x5, %rcx
142623afc: 89 2c 08                    	movl	%ebp, (%rax,%rcx)
142623aff: 44 89 74 08 04              	movl	%r14d, 0x4(%rax,%rcx)
142623b04: 4c 89 64 08 08              	movq	%r12, 0x8(%rax,%rcx)
142623b09: c7 44 08 10 00 00 00 00     	movl	$0x0, 0x10(%rax,%rcx)
142623b11: 88 5c 08 18                 	movb	%bl, 0x18(%rax,%rcx)
142623b15: 48 ff c7                    	incq	%rdi
142623b18: 48 89 7e 10                 	movq	%rdi, 0x10(%rsi)
142623b1c: 0f 28 b4 24 f0 01 00 00     	movaps	0x1f0(%rsp), %xmm6
142623b24: 48 81 c4 08 02 00 00        	addq	$0x208, %rsp            # imm = 0x208
142623b2b: 5b                          	popq	%rbx
142623b2c: 5d                          	popq	%rbp
142623b2d: 5f                          	popq	%rdi
142623b2e: 5e                          	popq	%rsi
142623b2f: 41 5c                       	popq	%r12
142623b31: 41 5d                       	popq	%r13
142623b33: 41 5e                       	popq	%r14
142623b35: 41 5f                       	popq	%r15
142623b37: c3                          	retq
142623b38: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
142623b41: 41 b8 20 00 00 00           	movl	$0x20, %r8d
142623b47: 41 b9 01 00 00 00           	movl	$0x1, %r9d
142623b4d: 48 89 f9                    	movq	%rdi, %rcx
142623b50: 4c 89 f2                    	movq	%r14, %rdx
142623b53: e8 78 1d 28 00              	callq	0x1428a58d0 <.text+0x28a48d0>
142623b58: e9 e3 fd ff ff              	jmp	0x142623940 <.text+0x2622940>
142623b5d: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
142623b66: 41 b8 20 00 00 00           	movl	$0x20, %r8d
142623b6c: 41 b9 01 00 00 00           	movl	$0x1, %r9d
142623b72: 48 89 f9                    	movq	%rdi, %rcx
142623b75: e8 56 1d 28 00              	callq	0x1428a58d0 <.text+0x28a48d0>
142623b7a: e9 b5 fe ff ff              	jmp	0x142623a34 <.text+0x2622a34>
142623b7f: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
142623b88: 41 b8 20 00 00 00           	movl	$0x20, %r8d
142623b8e: 41 b9 01 00 00 00           	movl	$0x1, %r9d
142623b94: 48 89 f9                    	movq	%rdi, %rcx
142623b97: e8 34 1d 28 00              	callq	0x1428a58d0 <.text+0x28a48d0>
142623b9c: e9 cb fe ff ff              	jmp	0x142623a6c <.text+0x2622a6c>
142623ba1: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
142623baa: 41 b8 20 00 00 00           	movl	$0x20, %r8d
142623bb0: 41 b9 01 00 00 00           	movl	$0x1, %r9d
142623bb6: 48 89 f9                    	movq	%rdi, %rcx
142623bb9: 4c 89 fa                    	movq	%r15, %rdx
142623bbc: e8 0f 1d 28 00              	callq	0x1428a58d0 <.text+0x28a48d0>
142623bc1: e9 2e fe ff ff              	jmp	0x1426239f4 <.text+0x26229f4>
142623bc6: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
142623bcf: 41 b8 20 00 00 00           	movl	$0x20, %r8d
142623bd5: 41 b9 01 00 00 00           	movl	$0x1, %r9d
142623bdb: 48 89 f9                    	movq	%rdi, %rcx
142623bde: 4c 89 fa                    	movq	%r15, %rdx
142623be1: e8 ea 1c 28 00              	callq	0x1428a58d0 <.text+0x28a48d0>
142623be6: e9 d8 fd ff ff              	jmp	0x1426239c3 <.text+0x26229c3>
142623beb: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
142623bf4: 41 b8 20 00 00 00           	movl	$0x20, %r8d
142623bfa: 41 b9 01 00 00 00           	movl	$0x1, %r9d
142623c00: 48 89 f9                    	movq	%rdi, %rcx
142623c03: 4c 89 fa                    	movq	%r15, %rdx
142623c06: e8 c5 1c 28 00              	callq	0x1428a58d0 <.text+0x28a48d0>
142623c0b: e9 8a fd ff ff              	jmp	0x14262399a <.text+0x262299a>
142623c10: 0f 10 44 24 70              	movups	0x70(%rsp), %xmm0
142623c15: 0f 10 8c 24 80 00 00 00     	movups	0x80(%rsp), %xmm1
142623c1d: 0f 29 4c 24 50              	movaps	%xmm1, 0x50(%rsp)
142623c22: 0f 29 44 24 40              	movaps	%xmm0, 0x40(%rsp)
142623c27: 48 8d 05 ba b8 96 00        	leaq	0x96b8ba(%rip), %rax    # 0x142f8f4e8
142623c2e: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
142623c33: 48 8d 0d 1e b7 96 00        	leaq	0x96b71e(%rip), %rcx    # 0x142f8f358
142623c3a: 4c 8d 0d f7 b6 96 00        	leaq	0x96b6f7(%rip), %r9     # 0x142f8f338
142623c41: 4c 8d 44 24 40              	leaq	0x40(%rsp), %r8
142623c46: ba 2b 00 00 00              	movl	$0x2b, %edx
142623c4b: e8 10 5b 2b 00              	callq	0x1428d9760 <.text+0x28d8760>
142623c50: 0f 10 44 24 40              	movups	0x40(%rsp), %xmm0
142623c55: 0f 10 4c 24 50              	movups	0x50(%rsp), %xmm1
142623c5a: 0f 29 8c 24 a0 00 00 00     	movaps	%xmm1, 0xa0(%rsp)
142623c62: 0f 29 84 24 90 00 00 00     	movaps	%xmm0, 0x90(%rsp)
142623c6a: 48 8d 05 47 b9 96 00        	leaq	0x96b947(%rip), %rax    # 0x142f8f5b8
142623c71: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
142623c76: 48 8d 0d db b6 96 00        	leaq	0x96b6db(%rip), %rcx    # 0x142f8f358
142623c7d: 4c 8d 0d b4 b6 96 00        	leaq	0x96b6b4(%rip), %r9     # 0x142f8f338
142623c84: 4c 8d 84 24 90 00 00 00     	leaq	0x90(%rsp), %r8
142623c8c: ba 2b 00 00 00              	movl	$0x2b, %edx
142623c91: e8 ca 5a 2b 00              	callq	0x1428d9760 <.text+0x28d8760>
142623c96: 45 31 e4                    	xorl	%r12d, %r12d
142623c99: 4c 8d 05 88 8b 96 00        	leaq	0x968b88(%rip), %r8     # 0x142f8c828
142623ca0: 4c 89 e1                    	movq	%r12, %rcx
142623ca3: 4c 89 f2                    	movq	%r14, %rdx
142623ca6: e8 38 53 2b 00              	callq	0x1428d8fe3 <.text+0x28d7fe3>
142623cab: cc                          	int3
