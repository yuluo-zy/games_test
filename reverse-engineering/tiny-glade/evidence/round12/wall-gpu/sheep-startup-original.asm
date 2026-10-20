
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a093e0: 55                          	pushq	%rbp
140a093e1: 41 57                       	pushq	%r15
140a093e3: 41 56                       	pushq	%r14
140a093e5: 41 55                       	pushq	%r13
140a093e7: 41 54                       	pushq	%r12
140a093e9: 56                          	pushq	%rsi
140a093ea: 57                          	pushq	%rdi
140a093eb: 53                          	pushq	%rbx
140a093ec: 48 81 ec 18 0a 00 00        	subq	$0xa18, %rsp            # imm = 0xA18
140a093f3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140a093fb: 44 0f 29 ad 80 09 00 00     	movaps	%xmm13, 0x980(%rbp)
140a09403: 44 0f 29 a5 70 09 00 00     	movaps	%xmm12, 0x970(%rbp)
140a0940b: 44 0f 29 9d 60 09 00 00     	movaps	%xmm11, 0x960(%rbp)
140a09413: 44 0f 29 95 50 09 00 00     	movaps	%xmm10, 0x950(%rbp)
140a0941b: 44 0f 29 8d 40 09 00 00     	movaps	%xmm9, 0x940(%rbp)
140a09423: 44 0f 29 85 30 09 00 00     	movaps	%xmm8, 0x930(%rbp)
140a0942b: 66 0f 7f bd 20 09 00 00     	movdqa	%xmm7, 0x920(%rbp)
140a09433: 66 0f 7f b5 10 09 00 00     	movdqa	%xmm6, 0x910(%rbp)
140a0943b: 48 c7 85 08 09 00 00 fe ff ff ff    	movq	$-0x2, 0x908(%rbp)
140a09446: 4d 89 cc                    	movq	%r9, %r12
140a09449: 48 89 d3                    	movq	%rdx, %rbx
140a0944c: 49 89 cf                    	movq	%rcx, %r15
140a0944f: 4d 8b 30                    	movq	(%r8), %r14
140a09452: 48 8d 15 bf d5 0b 02        	leaq	0x20bd5bf(%rip), %rdx   # 0x142ac6a18
140a09459: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
140a0945f: 4c 89 f1                    	movq	%r14, %rcx
140a09462: e8 09 41 f2 ff              	callq	0x14092d570 <.text+0x92c570>
140a09467: 48 89 95 f8 08 00 00        	movq	%rdx, 0x8f8(%rbp)
140a0946e: 48 83 f8 01                 	cmpq	$0x1, %rax
140a09472: 0f 85 be 08 00 00           	jne	0x140a09d36 <.text+0xa08d36>
140a09478: 48 8b 33                    	movq	(%rbx), %rsi
140a0947b: 48 8b 85 10 0a 00 00        	movq	0xa10(%rbp), %rax
140a09482: 48 8b 95 08 0a 00 00        	movq	0xa08(%rbp), %rdx
140a09489: 48 8b 0e                    	movq	(%rsi), %rcx
140a0948c: 48 83 c1 10                 	addq	$0x10, %rcx
140a09490: 48 8b 00                    	movq	(%rax), %rax
140a09493: 48 8b 3a                    	movq	(%rdx), %rdi
140a09496: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
140a0949b: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140a094a0: 48 c7 44 24 20 0c 00 00 00  	movq	$0xc, 0x20(%rsp)
140a094a9: 48 8d 15 40 d6 0b 02        	leaq	0x20bd640(%rip), %rdx   # 0x142ac6af0
140a094b0: 4c 8d 0d 4b d6 0b 02        	leaq	0x20bd64b(%rip), %r9    # 0x142ac6b02
140a094b7: 41 b8 12 00 00 00           	movl	$0x12, %r8d
140a094bd: e8 7e 69 13 00              	callq	0x140b3fe40 <.text+0xb3ee40>
140a094c2: 48 89 85 b0 08 00 00        	movq	%rax, 0x8b0(%rbp)
140a094c9: 48 8d 15 3e d6 0b 02        	leaq	0x20bd63e(%rip), %rdx   # 0x142ac6b0e
140a094d0: 48 8d 8d d0 03 00 00        	leaq	0x3d0(%rbp), %rcx
140a094d7: 41 b8 1e 00 00 00           	movl	$0x1e, %r8d
140a094dd: e8 ce 59 10 00              	callq	0x140b0eeb0 <.text+0xb0deb0>
140a094e2: 31 c0                       	xorl	%eax, %eax
140a094e4: 48 3b 85 d0 03 00 00        	cmpq	0x3d0(%rbp), %rax
140a094eb: 0f 80 54 13 00 00           	jo	0x140a0a845 <.text+0xa09845>
140a094f1: 48 8b 85 40 04 00 00        	movq	0x440(%rbp), %rax
140a094f8: 48 89 85 60 02 00 00        	movq	%rax, 0x260(%rbp)
140a094ff: 0f 10 85 30 04 00 00        	movups	0x430(%rbp), %xmm0
140a09506: 0f 29 85 50 02 00 00        	movaps	%xmm0, 0x250(%rbp)
140a0950d: 0f 10 85 20 04 00 00        	movups	0x420(%rbp), %xmm0
140a09514: 0f 29 85 40 02 00 00        	movaps	%xmm0, 0x240(%rbp)
140a0951b: 0f 10 85 10 04 00 00        	movups	0x410(%rbp), %xmm0
140a09522: 0f 29 85 30 02 00 00        	movaps	%xmm0, 0x230(%rbp)
140a09529: f3 0f 6f 85 d0 03 00 00     	movdqu	0x3d0(%rbp), %xmm0
140a09531: f3 0f 6f 8d e0 03 00 00     	movdqu	0x3e0(%rbp), %xmm1
140a09539: f3 0f 6f 95 f0 03 00 00     	movdqu	0x3f0(%rbp), %xmm2
140a09541: f3 0f 6f 9d 00 04 00 00     	movdqu	0x400(%rbp), %xmm3
140a09549: 66 0f 7f 9d 20 02 00 00     	movdqa	%xmm3, 0x220(%rbp)
140a09551: 66 0f 7f 95 10 02 00 00     	movdqa	%xmm2, 0x210(%rbp)
140a09559: 66 0f 7f 8d 00 02 00 00     	movdqa	%xmm1, 0x200(%rbp)
140a09561: 66 0f 7f 85 f0 01 00 00     	movdqa	%xmm0, 0x1f0(%rbp)
140a09569: 49 8b 34 24                 	movq	(%r12), %rsi
140a0956d: 4c 89 74 24 20              	movq	%r14, 0x20(%rsp)
140a09572: 48 8d 15 cf d5 0b 02        	leaq	0x20bd5cf(%rip), %rdx   # 0x142ac6b48
140a09579: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
140a09580: 41 b8 0b 00 00 00           	movl	$0xb, %r8d
140a09586: 49 89 f1                    	movq	%rsi, %r9
140a09589: e8 12 17 11 00              	callq	0x140b1aca0 <.text+0xb19ca0>
140a0958e: 48 8d 15 be d5 0b 02        	leaq	0x20bd5be(%rip), %rdx   # 0x142ac6b53
140a09595: 48 8d 8d d0 03 00 00        	leaq	0x3d0(%rbp), %rcx
140a0959c: 41 b8 1b 00 00 00           	movl	$0x1b, %r8d
140a095a2: e8 09 59 10 00              	callq	0x140b0eeb0 <.text+0xb0deb0>
140a095a7: 31 c0                       	xorl	%eax, %eax
140a095a9: 48 3b 85 d0 03 00 00        	cmpq	0x3d0(%rbp), %rax
140a095b0: 0f 80 cd 12 00 00           	jo	0x140a0a883 <.text+0xa09883>
140a095b6: 48 8b 85 40 04 00 00        	movq	0x440(%rbp), %rax
140a095bd: 48 89 85 60 02 00 00        	movq	%rax, 0x260(%rbp)
140a095c4: 0f 10 85 30 04 00 00        	movups	0x430(%rbp), %xmm0
140a095cb: 0f 29 85 50 02 00 00        	movaps	%xmm0, 0x250(%rbp)
140a095d2: 0f 10 85 20 04 00 00        	movups	0x420(%rbp), %xmm0
140a095d9: 0f 29 85 40 02 00 00        	movaps	%xmm0, 0x240(%rbp)
140a095e0: 0f 10 85 10 04 00 00        	movups	0x410(%rbp), %xmm0
140a095e7: 0f 29 85 30 02 00 00        	movaps	%xmm0, 0x230(%rbp)
140a095ee: f3 0f 6f 85 d0 03 00 00     	movdqu	0x3d0(%rbp), %xmm0
140a095f6: f3 0f 6f 8d e0 03 00 00     	movdqu	0x3e0(%rbp), %xmm1
140a095fe: f3 0f 6f 95 f0 03 00 00     	movdqu	0x3f0(%rbp), %xmm2
140a09606: f3 0f 6f 9d 00 04 00 00     	movdqu	0x400(%rbp), %xmm3
140a0960e: 66 0f 7f 9d 20 02 00 00     	movdqa	%xmm3, 0x220(%rbp)
140a09616: 66 0f 7f 95 10 02 00 00     	movdqa	%xmm2, 0x210(%rbp)
140a0961e: 66 0f 7f 8d 00 02 00 00     	movdqa	%xmm1, 0x200(%rbp)
140a09626: 66 0f 7f 85 f0 01 00 00     	movdqa	%xmm0, 0x1f0(%rbp)
140a0962e: 4c 89 74 24 20              	movq	%r14, 0x20(%rsp)
140a09633: 48 8d 15 4e d5 0b 02        	leaq	0x20bd54e(%rip), %rdx   # 0x142ac6b88
140a0963a: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
140a09641: 41 b8 0d 00 00 00           	movl	$0xd, %r8d
140a09647: 49 89 f1                    	movq	%rsi, %r9
140a0964a: e8 51 16 11 00              	callq	0x140b1aca0 <.text+0xb19ca0>
140a0964f: 48 89 85 b8 08 00 00        	movq	%rax, 0x8b8(%rbp)
140a09656: 48 8d 15 38 d5 0b 02        	leaq	0x20bd538(%rip), %rdx   # 0x142ac6b95
140a0965d: 48 8d 8d d0 03 00 00        	leaq	0x3d0(%rbp), %rcx
140a09664: 41 b8 1a 00 00 00           	movl	$0x1a, %r8d
140a0966a: e8 41 58 10 00              	callq	0x140b0eeb0 <.text+0xb0deb0>
140a0966f: 31 c0                       	xorl	%eax, %eax
140a09671: 48 3b 85 d0 03 00 00        	cmpq	0x3d0(%rbp), %rax
140a09678: 0f 80 43 12 00 00           	jo	0x140a0a8c1 <.text+0xa098c1>
140a0967e: 48 8b 85 40 04 00 00        	movq	0x440(%rbp), %rax
140a09685: 48 89 85 60 02 00 00        	movq	%rax, 0x260(%rbp)
140a0968c: 0f 10 85 30 04 00 00        	movups	0x430(%rbp), %xmm0
140a09693: 0f 29 85 50 02 00 00        	movaps	%xmm0, 0x250(%rbp)
140a0969a: 0f 10 85 20 04 00 00        	movups	0x420(%rbp), %xmm0
140a096a1: 0f 29 85 40 02 00 00        	movaps	%xmm0, 0x240(%rbp)
140a096a8: 0f 10 85 10 04 00 00        	movups	0x410(%rbp), %xmm0
140a096af: 0f 29 85 30 02 00 00        	movaps	%xmm0, 0x230(%rbp)
140a096b6: 0f 10 85 d0 03 00 00        	movups	0x3d0(%rbp), %xmm0
140a096bd: f3 0f 6f 8d e0 03 00 00     	movdqu	0x3e0(%rbp), %xmm1
140a096c5: f3 0f 6f 95 f0 03 00 00     	movdqu	0x3f0(%rbp), %xmm2
140a096cd: f3 0f 6f 9d 00 04 00 00     	movdqu	0x400(%rbp), %xmm3
140a096d5: 66 0f 7f 9d 20 02 00 00     	movdqa	%xmm3, 0x220(%rbp)
140a096dd: 66 0f 7f 95 10 02 00 00     	movdqa	%xmm2, 0x210(%rbp)
140a096e5: 66 0f 7f 8d 00 02 00 00     	movdqa	%xmm1, 0x200(%rbp)
140a096ed: 0f 29 85 f0 01 00 00        	movaps	%xmm0, 0x1f0(%rbp)
140a096f4: 4c 89 b5 c8 08 00 00        	movq	%r14, 0x8c8(%rbp)
140a096fb: 4c 89 74 24 20              	movq	%r14, 0x20(%rsp)
140a09700: 48 8d 15 c1 d4 0b 02        	leaq	0x20bd4c1(%rip), %rdx   # 0x142ac6bc8
140a09707: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
140a0970e: 41 b8 0c 00 00 00           	movl	$0xc, %r8d
140a09714: 49 89 f1                    	movq	%rsi, %r9
140a09717: e8 84 15 11 00              	callq	0x140b1aca0 <.text+0xb19ca0>
140a0971c: 48 89 85 c0 08 00 00        	movq	%rax, 0x8c0(%rbp)
140a09723: 48 8d 15 aa d4 0b 02        	leaq	0x20bd4aa(%rip), %rdx   # 0x142ac6bd4
140a0972a: 41 b8 12 00 00 00           	movl	$0x12, %r8d
140a09730: 48 89 bd d0 08 00 00        	movq	%rdi, 0x8d0(%rbp)
140a09737: 48 89 f9                    	movq	%rdi, %rcx
140a0973a: e8 61 e3 f2 ff              	callq	0x140937aa0 <.text+0x936aa0>
140a0973f: a8 01                       	testb	$0x1, %al
140a09741: 0f 84 b8 11 00 00           	je	0x140a0a8ff <.text+0xa098ff>
140a09747: 48 89 d6                    	movq	%rdx, %rsi
140a0974a: 48 8b 85 18 0a 00 00        	movq	0xa18(%rbp), %rax
140a09751: f2 0f 10 05 07 c6 0b 02     	movsd	0x20bc607(%rip), %xmm0  # 0x142ac5d60
140a09759: f2 0f 11 85 88 08 00 00     	movsd	%xmm0, 0x888(%rbp)
140a09761: c7 85 90 08 00 00 66 66 26 3f       	movl	$0x3f266666, 0x890(%rbp) # imm = 0x3F266666
140a0976b: f2 0f 10 05 ed c1 f1 01     	movsd	0x1f1c1ed(%rip), %xmm0  # 0x142925960
140a09773: f2 0f 11 85 10 08 00 00     	movsd	%xmm0, 0x810(%rbp)
140a0977b: c7 85 18 08 00 00 00 00 80 3f       	movl	$0x3f800000, 0x818(%rbp) # imm = 0x3F800000
140a09785: 48 c7 85 d8 08 00 00 5c 6e 01 00    	movq	$0x16e5c, 0x8d8(%rbp) # imm = 0x16E5C
140a09790: 49 8b 0f                    	movq	(%r15), %rcx
140a09793: 48 89 8d a0 08 00 00        	movq	%rcx, 0x8a0(%rbp)
140a0979a: 8b 48 1c                    	movl	0x1c(%rax), %ecx
140a0979d: 89 8d f0 08 00 00           	movl	%ecx, 0x8f0(%rbp)
140a097a3: 4c 8b 30                    	movq	(%rax), %r14
140a097a6: 48 8b 40 10                 	movq	0x10(%rax), %rax
140a097aa: 48 89 85 a8 08 00 00        	movq	%rax, 0x8a8(%rbp)
140a097b1: 4c 8d a5 40 04 00 00        	leaq	0x440(%rbp), %r12
140a097b8: 31 db                       	xorl	%ebx, %ebx
140a097ba: f3 44 0f 10 0d f5 d4 00 02  	movss	0x200d4f5(%rip), %xmm9  # 0x142a16cb8
140a097c3: 48 8d bd d8 08 00 00        	leaq	0x8d8(%rbp), %rdi
140a097ca: f3 44 0f 10 15 15 09 f5 01  	movss	0x1f50915(%rip), %xmm10 # 0x14295a0e8
140a097d3: f3 44 0f 10 1d ac 6f f9 01  	movss	0x1f96fac(%rip), %xmm11 # 0x1429a0788
140a097dc: 4c 8d bd d0 03 00 00        	leaq	0x3d0(%rbp), %r15
140a097e3: f3 44 0f 10 25 80 33 0b 02  	movss	0x20b3380(%rip), %xmm12 # 0x142abcb6c
140a097ec: 48 83 fb 03                 	cmpq	$0x3, %rbx
140a097f0: 0f 83 b1 04 00 00           	jae	0x140a09ca7 <.text+0xa08ca7>
140a097f6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140a09800: f3 44 0f 10 ac 9d 88 08 00 00       	movss	0x888(%rbp,%rbx,4), %xmm13
140a0980a: f3 45 0f 59 e9              	mulss	%xmm9, %xmm13
140a0980f: 48 8b 85 a0 08 00 00        	movq	0x8a0(%rbp), %rax
140a09816: 48 8b 10                    	movq	(%rax), %rdx
140a09819: 48 85 d2                    	testq	%rdx, %rdx
140a0981c: 74 32                       	je	0x140a09850 <.text+0xa08850>
140a0981e: 48 83 c2 10                 	addq	$0x10, %rdx
140a09822: 41 b9 15 00 00 00           	movl	$0x15, %r9d
140a09828: 4c 89 f9                    	movq	%r15, %rcx
140a0982b: 4c 8d 05 e6 d3 0b 02        	leaq	0x20bd3e6(%rip), %r8    # 0x142ac6c18
140a09832: e8 09 ac 04 00              	callq	0x140a54440 <.text+0xa53440>
140a09837: 48 83 bd d0 03 00 00 02     	cmpq	$0x2, 0x3d0(%rbp)
140a0983f: 75 75                       	jne	0x140a098b6 <.text+0xa088b6>
140a09841: e9 89 0f 00 00              	jmp	0x140a0a7cf <.text+0xa097cf>
140a09846: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140a09850: 0f b6 05 b2 b3 c2 02        	movzbl	0x2c2b3b2(%rip), %eax   # 0x143634c09
140a09857: b9 15 00 00 00              	movl	$0x15, %ecx
140a0985c: ba 01 00 00 00              	movl	$0x1, %edx
140a09861: e8 aa a3 c0 ff              	callq	0x140613c10 <.text+0x612c10>
140a09866: 48 85 c0                    	testq	%rax, %rax
140a09869: 0f 84 9e 0f 00 00           	je	0x140a0a80d <.text+0xa0980d>
140a0986f: 0f 10 05 a2 d3 0b 02        	movups	0x20bd3a2(%rip), %xmm0  # 0x142ac6c18
140a09876: 0f 11 00                    	movups	%xmm0, (%rax)
140a09879: 48 b9 65 65 70 2f 42 61 61 68       	movabsq	$0x686161422f706565, %rcx # imm = 0x686161422F706565
140a09883: 48 89 48 0d                 	movq	%rcx, 0xd(%rax)
140a09887: 48 c7 85 d0 03 00 00 00 00 00 00    	movq	$0x0, 0x3d0(%rbp)
140a09892: 48 c7 85 e0 03 00 00 15 00 00 00    	movq	$0x15, 0x3e0(%rbp)
140a0989d: 48 89 85 e8 03 00 00        	movq	%rax, 0x3e8(%rbp)
140a098a4: 48 c7 85 f0 03 00 00 15 00 00 00    	movq	$0x15, 0x3f0(%rbp)
140a098af: c6 85 f8 03 00 00 02        	movb	$0x2, 0x3f8(%rbp)
140a098b6: 48 8b 85 d0 03 00 00        	movq	0x3d0(%rbp), %rax
140a098bd: 48 8b 8d d8 03 00 00        	movq	0x3d8(%rbp), %rcx
140a098c4: 48 8b 95 e0 03 00 00        	movq	0x3e0(%rbp), %rdx
140a098cb: 4c 8b 85 e8 03 00 00        	movq	0x3e8(%rbp), %r8
140a098d2: 4c 8b 8d f0 03 00 00        	movq	0x3f0(%rbp), %r9
140a098d9: 4c 89 8d 10 02 00 00        	movq	%r9, 0x210(%rbp)
140a098e0: 44 0f b6 8d f8 03 00 00     	movzbl	0x3f8(%rbp), %r9d
140a098e8: 44 88 8d 18 02 00 00        	movb	%r9b, 0x218(%rbp)
140a098ef: 44 8b 8d f9 03 00 00        	movl	0x3f9(%rbp), %r9d
140a098f6: 44 89 8d 19 02 00 00        	movl	%r9d, 0x219(%rbp)
140a098fd: 44 0f b7 8d fd 03 00 00     	movzwl	0x3fd(%rbp), %r9d
140a09905: 66 44 89 8d 1d 02 00 00     	movw	%r9w, 0x21d(%rbp)
140a0990d: 44 0f b6 8d ff 03 00 00     	movzbl	0x3ff(%rbp), %r9d
140a09915: 44 88 8d 1f 02 00 00        	movb	%r9b, 0x21f(%rbp)
140a0991c: 48 89 95 00 02 00 00        	movq	%rdx, 0x200(%rbp)
140a09923: 4c 89 85 08 02 00 00        	movq	%r8, 0x208(%rbp)
140a0992a: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
140a09931: 48 89 8d f8 01 00 00        	movq	%rcx, 0x1f8(%rbp)
140a09938: 66 0f 6e b4 9d 10 08 00 00  	movd	0x810(%rbp,%rbx,4), %xmm6
140a09941: c6 85 06 09 00 00 01        	movb	$0x1, 0x906(%rbp)
140a09948: 41 b8 05 00 00 00           	movl	$0x5, %r8d
140a0994e: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
140a09955: 48 8d 15 ec d2 0b 02        	leaq	0x20bd2ec(%rip), %rdx   # 0x142ac6c48
140a0995c: 66 0f 6f de                 	movdqa	%xmm6, %xmm3
140a09960: e8 0b dd 04 00              	callq	0x140a57670 <.text+0xa56670>
140a09965: c6 85 06 09 00 00 01        	movb	$0x1, 0x906(%rbp)
140a0996c: 48 89 f9                    	movq	%rdi, %rcx
140a0996f: e8 0c 5a 2a 00              	callq	0x140caf380 <.text+0xcae380>
140a09974: 0f 28 f8                    	movaps	%xmm0, %xmm7
140a09977: c6 85 06 09 00 00 01        	movb	$0x1, 0x906(%rbp)
140a0997e: 48 89 f9                    	movq	%rdi, %rcx
140a09981: e8 fa 59 2a 00              	callq	0x140caf380 <.text+0xcae380>
140a09986: 44 0f 28 c0                 	movaps	%xmm0, %xmm8
140a0998a: 8b 85 f0 08 00 00           	movl	0x8f0(%rbp), %eax
140a09990: 48 8b 8d a8 08 00 00        	movq	0x8a8(%rbp), %rcx
140a09997: 89 01                       	movl	%eax, (%rcx)
140a09999: 45 8b 6e 08                 	movl	0x8(%r14), %r13d
140a0999d: 41 8d 45 01                 	leal	0x1(%r13), %eax
140a099a1: 41 89 46 08                 	movl	%eax, 0x8(%r14)
140a099a5: 0f b6 05 5d b2 c2 02        	movzbl	0x2c2b25d(%rip), %eax   # 0x143634c09
140a099ac: b9 18 00 00 00              	movl	$0x18, %ecx
140a099b1: ba 08 00 00 00              	movl	$0x8, %edx
140a099b6: e8 55 a2 c0 ff              	callq	0x140613c10 <.text+0x612c10>
140a099bb: 48 85 c0                    	testq	%rax, %rax
140a099be: 0f 84 f0 0d 00 00           	je	0x140a0a7b4 <.text+0xa097b4>
140a099c4: f3 41 0f 58 fa              	addss	%xmm10, %xmm7
140a099c9: f3 41 0f 59 fb              	mulss	%xmm11, %xmm7
140a099ce: f3 45 0f 58 c2              	addss	%xmm10, %xmm8
140a099d3: f3 45 0f 59 c3              	mulss	%xmm11, %xmm8
140a099d8: c7 40 04 00 00 00 00        	movl	$0x0, 0x4(%rax)
140a099df: f3 0f 11 78 08              	movss	%xmm7, 0x8(%rax)
140a099e4: c7 40 0c 00 00 00 00        	movl	$0x0, 0xc(%rax)
140a099eb: f3 44 0f 11 40 10           	movss	%xmm8, 0x10(%rax)
140a099f1: 44 89 68 14                 	movl	%r13d, 0x14(%rax)
140a099f5: 48 c7 85 d0 03 00 00 01 00 00 00    	movq	$0x1, 0x3d0(%rbp)
140a09a00: 48 89 85 d8 03 00 00        	movq	%rax, 0x3d8(%rbp)
140a09a07: 48 c7 85 e0 03 00 00 01 00 00 00    	movq	$0x1, 0x3e0(%rbp)
140a09a12: c6 85 06 09 00 00 01        	movb	$0x1, 0x906(%rbp)
140a09a19: 48 8d 8d 90 07 00 00        	leaq	0x790(%rbp), %rcx
140a09a20: 4c 89 f2                    	movq	%r14, %rdx
140a09a23: 4d 89 f8                    	movq	%r15, %r8
140a09a26: e8 15 c9 57 00              	callq	0x140f86340 <.text+0xf85340>
140a09a2b: c6 85 02 09 00 00 01        	movb	$0x1, 0x902(%rbp)
140a09a32: 4c 89 f9                    	movq	%r15, %rcx
140a09a35: e8 46 37 29 00              	callq	0x140c9d180 <.text+0xc9c180>
140a09a3a: 66 0f 6f 8d d0 03 00 00     	movdqa	0x3d0(%rbp), %xmm1
140a09a42: 0f 28 85 f0 01 00 00        	movaps	0x1f0(%rbp), %xmm0
140a09a49: 0f 28 95 00 02 00 00        	movaps	0x200(%rbp), %xmm2
140a09a50: 66 0f 6f 9d 10 02 00 00     	movdqa	0x210(%rbp), %xmm3
140a09a58: f3 41 0f 7f 5c 24 20        	movdqu	%xmm3, 0x20(%r12)
140a09a5f: 41 0f 11 54 24 10           	movups	%xmm2, 0x10(%r12)
140a09a65: 41 0f 11 04 24              	movups	%xmm0, (%r12)
140a09a6a: 41 0f 28 c5                 	movaps	%xmm13, %xmm0
140a09a6e: f3 41 0f 59 c4              	mulss	%xmm12, %xmm0
140a09a73: 48 8b 85 a0 07 00 00        	movq	0x7a0(%rbp), %rax
140a09a7a: 49 89 84 24 20 01 00 00     	movq	%rax, 0x120(%r12)
140a09a82: f3 0f 6f 95 90 07 00 00     	movdqu	0x790(%rbp), %xmm2
140a09a8a: f3 41 0f 7f 94 24 10 01 00 00       	movdqu	%xmm2, 0x110(%r12)
140a09a94: 66 0f 7f 8d d0 03 00 00     	movdqa	%xmm1, 0x3d0(%rbp)
140a09a9c: f3 0f 11 bd e0 03 00 00     	movss	%xmm7, 0x3e0(%rbp)
140a09aa4: c7 85 e4 03 00 00 00 00 00 00       	movl	$0x0, 0x3e4(%rbp)
140a09aae: f3 44 0f 11 85 e8 03 00 00  	movss	%xmm8, 0x3e8(%rbp)
140a09ab7: f3 44 0f 11 ad ec 03 00 00  	movss	%xmm13, 0x3ec(%rbp)
140a09ac0: f3 44 0f 11 ad f0 03 00 00  	movss	%xmm13, 0x3f0(%rbp)
140a09ac9: f3 44 0f 11 ad f4 03 00 00  	movss	%xmm13, 0x3f4(%rbp)
140a09ad2: 66 0f 7f 8d 00 04 00 00     	movdqa	%xmm1, 0x400(%rbp)
140a09ada: f3 0f 11 bd 10 04 00 00     	movss	%xmm7, 0x410(%rbp)
140a09ae2: c7 85 14 04 00 00 00 00 00 00       	movl	$0x0, 0x414(%rbp)
140a09aec: f3 44 0f 11 85 18 04 00 00  	movss	%xmm8, 0x418(%rbp)
140a09af5: f3 44 0f 11 ad 1c 04 00 00  	movss	%xmm13, 0x41c(%rbp)
140a09afe: f3 44 0f 11 ad 20 04 00 00  	movss	%xmm13, 0x420(%rbp)
140a09b07: f3 44 0f 11 ad 24 04 00 00  	movss	%xmm13, 0x424(%rbp)
140a09b10: 48 8b 85 f8 08 00 00        	movq	0x8f8(%rbp), %rax
140a09b17: 48 89 85 30 04 00 00        	movq	%rax, 0x430(%rbp)
140a09b1e: 48 8b 85 b0 08 00 00        	movq	0x8b0(%rbp), %rax
140a09b25: 48 89 85 38 04 00 00        	movq	%rax, 0x438(%rbp)
140a09b2c: c7 85 78 04 00 00 00 00 00 00       	movl	$0x0, 0x478(%rbp)
140a09b36: c7 85 80 04 00 00 00 00 00 00       	movl	$0x0, 0x480(%rbp)
140a09b40: c7 85 88 04 00 00 00 00 00 00       	movl	$0x0, 0x488(%rbp)
140a09b4a: c7 85 90 04 00 00 00 00 00 00       	movl	$0x0, 0x490(%rbp)
140a09b54: 48 89 9d 98 04 00 00        	movq	%rbx, 0x498(%rbp)
140a09b5b: 48 c7 85 a0 04 00 00 00 00 00 00    	movq	$0x0, 0x4a0(%rbp)
140a09b66: c6 85 a8 04 00 00 00        	movb	$0x0, 0x4a8(%rbp)
140a09b6d: c7 85 ac 04 00 00 00 00 00 00       	movl	$0x0, 0x4ac(%rbp)
140a09b77: f3 44 0f 11 ad b4 04 00 00  	movss	%xmm13, 0x4b4(%rbp)
140a09b80: f3 44 0f 11 ad b8 04 00 00  	movss	%xmm13, 0x4b8(%rbp)
140a09b89: f3 44 0f 11 ad bc 04 00 00  	movss	%xmm13, 0x4bc(%rbp)
140a09b92: c7 85 c0 04 00 00 00 00 00 00       	movl	$0x0, 0x4c0(%rbp)
140a09b9c: f3 0f 11 bd c4 04 00 00     	movss	%xmm7, 0x4c4(%rbp)
140a09ba4: c7 85 c8 04 00 00 00 00 00 00       	movl	$0x0, 0x4c8(%rbp)
140a09bae: f3 44 0f 11 85 cc 04 00 00  	movss	%xmm8, 0x4cc(%rbp)
140a09bb7: 48 c7 85 d0 04 00 00 00 00 00 00    	movq	$0x0, 0x4d0(%rbp)
140a09bc2: c6 85 d8 04 00 00 01        	movb	$0x1, 0x4d8(%rbp)
140a09bc9: 66 0f 7e b5 70 04 00 00     	movd	%xmm6, 0x470(%rbp)
140a09bd1: c7 85 74 04 00 00 00 00 00 00       	movl	$0x0, 0x474(%rbp)
140a09bdb: c7 85 e0 04 00 00 00 00 00 00       	movl	$0x0, 0x4e0(%rbp)
140a09be5: c7 85 f0 04 00 00 02 00 00 00       	movl	$0x2, 0x4f0(%rbp)
140a09bef: 41 c7 84 24 c0 00 00 00 00 00 00 00 	movl	$0x0, 0xc0(%r12)
140a09bfb: 49 c7 84 24 b8 00 00 00 00 00 00 00 	movq	$0x0, 0xb8(%r12)
140a09c07: c7 85 04 05 00 00 0a d7 23 3c       	movl	$0x3c23d70a, 0x504(%rbp) # imm = 0x3C23D70A
140a09c11: 41 c7 84 24 d0 00 00 00 00 00 00 00 	movl	$0x0, 0xd0(%r12)
140a09c1d: 49 c7 84 24 c8 00 00 00 00 00 00 00 	movq	$0x0, 0xc8(%r12)
140a09c29: c7 85 14 05 00 00 00 00 80 3f       	movl	$0x3f800000, 0x514(%rbp) # imm = 0x3F800000
140a09c33: f3 0f 11 85 48 05 00 00     	movss	%xmm0, 0x548(%rbp)
140a09c3b: 48 8b 85 b8 08 00 00        	movq	0x8b8(%rbp), %rax
140a09c42: 48 89 85 18 05 00 00        	movq	%rax, 0x518(%rbp)
140a09c49: 48 89 b5 20 05 00 00        	movq	%rsi, 0x520(%rbp)
140a09c50: c7 85 2c 05 00 00 00 00 00 00       	movl	$0x0, 0x52c(%rbp)
140a09c5a: 48 8b 85 c0 08 00 00        	movq	0x8c0(%rbp), %rax
140a09c61: 48 89 85 30 05 00 00        	movq	%rax, 0x530(%rbp)
140a09c68: 48 89 b5 38 05 00 00        	movq	%rsi, 0x538(%rbp)
140a09c6f: c7 85 44 05 00 00 00 00 00 00       	movl	$0x0, 0x544(%rbp)
140a09c79: c6 85 02 09 00 00 00        	movb	$0x0, 0x902(%rbp)
140a09c80: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140a09c84: 48 8b 95 20 0a 00 00        	movq	0xa20(%rbp), %rdx
140a09c8b: 4d 89 f8                    	movq	%r15, %r8
140a09c8e: 4c 8d 0d bb cf 0b 02        	leaq	0x20bcfbb(%rip), %r9    # 0x142ac6c50
140a09c95: e8 f6 84 8d 00              	callq	0x1412e2190 <.text+0x12e1190>
140a09c9a: 48 ff c3                    	incq	%rbx
140a09c9d: 48 83 fb 03                 	cmpq	$0x3, %rbx
140a09ca1: 0f 82 59 fb ff ff           	jb	0x140a09800 <.text+0xa08800>
140a09ca7: 48 8d 75 c0                 	leaq	-0x40(%rbp), %rsi
140a09cab: 48 89 f1                    	movq	%rsi, %rcx
140a09cae: 48 8b 95 c8 08 00 00        	movq	0x8c8(%rbp), %rdx
140a09cb5: 4c 8b 85 d0 08 00 00        	movq	0x8d0(%rbp), %r8
140a09cbc: 48 8b bd 20 0a 00 00        	movq	0xa20(%rbp), %rdi
140a09cc3: 49 89 f9                    	movq	%rdi, %r9
140a09cc6: e8 d5 71 ce 00              	callq	0x1416f0ea0 <.text+0x16efea0>
140a09ccb: 4c 8d 0d 2e cf 0b 02        	leaq	0x20bcf2e(%rip), %r9    # 0x142ac6c00
140a09cd2: 48 8d 8d d0 03 00 00        	leaq	0x3d0(%rbp), %rcx
140a09cd9: 48 89 fa                    	movq	%rdi, %rdx
140a09cdc: 49 89 f0                    	movq	%rsi, %r8
140a09cdf: e8 fc 86 8d 00              	callq	0x1412e23e0 <.text+0x12e13e0>
140a09ce4: 0f 28 b5 10 09 00 00        	movaps	0x910(%rbp), %xmm6
140a09ceb: 0f 28 bd 20 09 00 00        	movaps	0x920(%rbp), %xmm7
140a09cf2: 44 0f 28 85 30 09 00 00     	movaps	0x930(%rbp), %xmm8
140a09cfa: 44 0f 28 8d 40 09 00 00     	movaps	0x940(%rbp), %xmm9
140a09d02: 44 0f 28 95 50 09 00 00     	movaps	0x950(%rbp), %xmm10
140a09d0a: 44 0f 28 9d 60 09 00 00     	movaps	0x960(%rbp), %xmm11
140a09d12: 44 0f 28 a5 70 09 00 00     	movaps	0x970(%rbp), %xmm12
140a09d1a: 44 0f 28 ad 80 09 00 00     	movaps	0x980(%rbp), %xmm13
140a09d22: 48 81 c4 18 0a 00 00        	addq	$0xa18, %rsp            # imm = 0xA18
140a09d29: 5b                          	popq	%rbx
140a09d2a: 5f                          	popq	%rdi
140a09d2b: 5e                          	popq	%rsi
140a09d2c: 41 5c                       	popq	%r12
140a09d2e: 41 5d                       	popq	%r13
140a09d30: 41 5e                       	popq	%r14
140a09d32: 41 5f                       	popq	%r15
140a09d34: 5d                          	popq	%rbp
140a09d35: c3                          	retq
140a09d36: 48 8d 15 e5 cc 0b 02        	leaq	0x20bcce5(%rip), %rdx   # 0x142ac6a22
140a09d3d: 48 8d 8d d0 03 00 00        	leaq	0x3d0(%rbp), %rcx
140a09d44: 41 b8 24 00 00 00           	movl	$0x24, %r8d
140a09d4a: e8 61 51 10 00              	callq	0x140b0eeb0 <.text+0xb0deb0>
140a09d4f: 31 c0                       	xorl	%eax, %eax
140a09d51: 48 3b 85 d0 03 00 00        	cmpq	0x3d0(%rbp), %rax
140a09d58: 0f 80 ad 0b 00 00           	jo	0x140a0a90b <.text+0xa0990b>
140a09d5e: 48 8b 85 40 04 00 00        	movq	0x440(%rbp), %rax
140a09d65: 48 89 85 80 08 00 00        	movq	%rax, 0x880(%rbp)
140a09d6c: 0f 10 85 30 04 00 00        	movups	0x430(%rbp), %xmm0
140a09d73: 0f 29 85 70 08 00 00        	movaps	%xmm0, 0x870(%rbp)
140a09d7a: 0f 10 85 20 04 00 00        	movups	0x420(%rbp), %xmm0
140a09d81: 0f 29 85 60 08 00 00        	movaps	%xmm0, 0x860(%rbp)
140a09d88: 0f 10 85 10 04 00 00        	movups	0x410(%rbp), %xmm0
140a09d8f: 0f 29 85 50 08 00 00        	movaps	%xmm0, 0x850(%rbp)
140a09d96: f3 0f 6f 85 d0 03 00 00     	movdqu	0x3d0(%rbp), %xmm0
140a09d9e: f3 0f 6f 8d e0 03 00 00     	movdqu	0x3e0(%rbp), %xmm1
140a09da6: f3 0f 6f 95 f0 03 00 00     	movdqu	0x3f0(%rbp), %xmm2
140a09dae: f3 0f 6f 9d 00 04 00 00     	movdqu	0x400(%rbp), %xmm3
140a09db6: 66 0f 7f 9d 40 08 00 00     	movdqa	%xmm3, 0x840(%rbp)
140a09dbe: 66 0f 7f 95 30 08 00 00     	movdqa	%xmm2, 0x830(%rbp)
140a09dc6: 66 0f 7f 8d 20 08 00 00     	movdqa	%xmm1, 0x820(%rbp)
140a09dce: 66 0f 7f 85 10 08 00 00     	movdqa	%xmm0, 0x810(%rbp)
140a09dd6: c6 85 05 09 00 00 01        	movb	$0x1, 0x905(%rbp)
140a09ddd: 48 8d 15 ac cc 0b 02        	leaq	0x20bccac(%rip), %rdx   # 0x142ac6a90
140a09de4: 48 8d 8d d0 03 00 00        	leaq	0x3d0(%rbp), %rcx
140a09deb: 41 b8 2c 00 00 00           	movl	$0x2c, %r8d
140a09df1: e8 ba 50 10 00              	callq	0x140b0eeb0 <.text+0xb0deb0>
140a09df6: 31 c0                       	xorl	%eax, %eax
140a09df8: 48 3b 85 d0 03 00 00        	cmpq	0x3d0(%rbp), %rax
140a09dff: 0f 80 44 0b 00 00           	jo	0x140a0a949 <.text+0xa09949>
140a09e05: 48 8b 85 40 04 00 00        	movq	0x440(%rbp), %rax
140a09e0c: 48 89 85 00 08 00 00        	movq	%rax, 0x800(%rbp)
140a09e13: 0f 10 85 30 04 00 00        	movups	0x430(%rbp), %xmm0
140a09e1a: 0f 29 85 f0 07 00 00        	movaps	%xmm0, 0x7f0(%rbp)
140a09e21: 0f 10 85 20 04 00 00        	movups	0x420(%rbp), %xmm0
140a09e28: 0f 29 85 e0 07 00 00        	movaps	%xmm0, 0x7e0(%rbp)
140a09e2f: 0f 10 85 10 04 00 00        	movups	0x410(%rbp), %xmm0
140a09e36: 0f 29 85 d0 07 00 00        	movaps	%xmm0, 0x7d0(%rbp)
140a09e3d: f3 0f 6f 85 d0 03 00 00     	movdqu	0x3d0(%rbp), %xmm0
140a09e45: f3 0f 6f 8d e0 03 00 00     	movdqu	0x3e0(%rbp), %xmm1
140a09e4d: f3 0f 6f 95 f0 03 00 00     	movdqu	0x3f0(%rbp), %xmm2
140a09e55: f3 0f 6f 9d 00 04 00 00     	movdqu	0x400(%rbp), %xmm3
140a09e5d: 66 0f 7f 9d c0 07 00 00     	movdqa	%xmm3, 0x7c0(%rbp)
140a09e65: 66 0f 7f 95 b0 07 00 00     	movdqa	%xmm2, 0x7b0(%rbp)
140a09e6d: 66 0f 7f 8d a0 07 00 00     	movdqa	%xmm1, 0x7a0(%rbp)
140a09e75: 66 0f 7f 85 90 07 00 00     	movdqa	%xmm0, 0x790(%rbp)
140a09e7d: 48 83 bd 60 08 00 00 00     	cmpq	$0x0, 0x860(%rbp)
140a09e85: 0f 84 8f 08 00 00           	je	0x140a0a71a <.text+0xa0971a>
140a09e8b: 48 8d 8d 68 08 00 00        	leaq	0x868(%rbp), %rcx
140a09e92: 48 8d 15 ba c6 0b 02        	leaq	0x20bc6ba(%rip), %rdx   # 0x142ac6553
140a09e99: 41 b8 0f 00 00 00           	movl	$0xf, %r8d
140a09e9f: e8 cc 7b ba ff              	callq	0x1405b1a70 <.text+0x5b0a70>
140a09ea4: 48 8b 8d 48 08 00 00        	movq	0x848(%rbp), %rcx
140a09eab: 48 8b 95 50 08 00 00        	movq	0x850(%rbp), %rdx
140a09eb2: 49 89 d0                    	movq	%rdx, %r8
140a09eb5: 49 21 c0                    	andq	%rax, %r8
140a09eb8: 48 c1 e8 39                 	shrq	$0x39, %rax
140a09ebc: 66 0f 6e c0                 	movd	%eax, %xmm0
140a09ec0: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140a09ec4: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140a09ec9: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140a09ece: 48 8d 41 c8                 	leaq	-0x38(%rcx), %rax
140a09ed2: 45 31 c9                    	xorl	%r9d, %r9d
140a09ed5: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
140a09ed9: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
140a09edf: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
140a09ee3: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
140a09ee7: 66 44 0f d7 d3              	pmovmskb	%xmm3, %r10d
140a09eec: 45 85 d2                    	testl	%r10d, %r10d
140a09eef: 74 55                       	je	0x140a09f46 <.text+0xa08f46>
140a09ef1: f3 45 0f bc da              	tzcntl	%r10d, %r11d
140a09ef6: 4d 01 c3                    	addq	%r8, %r11
140a09ef9: 49 21 d3                    	andq	%rdx, %r11
140a09efc: 49 f7 db                    	negq	%r11
140a09eff: 4d 6b db 38                 	imulq	$0x38, %r11, %r11
140a09f03: 4a 83 7c 18 10 0f           	cmpq	$0xf, 0x10(%rax,%r11)
140a09f09: 75 2e                       	jne	0x140a09f39 <.text+0xa08f39>
140a09f0b: 4a 8d 34 18                 	leaq	(%rax,%r11), %rsi
140a09f0f: 48 8b 76 08                 	movq	0x8(%rsi), %rsi
140a09f13: 48 8b 3e                    	movq	(%rsi), %rdi
140a09f16: 49 bd 56 65 72 74 65 78 5f 50       	movabsq	$0x505f786574726556, %r13 # imm = 0x505F786574726556
140a09f20: 4c 31 ef                    	xorq	%r13, %rdi
140a09f23: 48 8b 76 07                 	movq	0x7(%rsi), %rsi
140a09f27: 49 bd 50 6f 73 69 74 69 6f 6e       	movabsq	$0x6e6f697469736f50, %r13 # imm = 0x6E6F697469736F50
140a09f31: 4c 31 ee                    	xorq	%r13, %rsi
140a09f34: 48 09 fe                    	orq	%rdi, %rsi
140a09f37: 74 32                       	je	0x140a09f6b <.text+0xa08f6b>
140a09f39: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
140a09f3d: 66 45 21 d3                 	andw	%r10w, %r11w
140a09f41: 45 89 da                    	movl	%r11d, %r10d
140a09f44: 75 ab                       	jne	0x140a09ef1 <.text+0xa08ef1>
140a09f46: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
140a09f4a: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
140a09f4f: 45 85 d2                    	testl	%r10d, %r10d
140a09f52: 0f 85 c2 07 00 00           	jne	0x140a0a71a <.text+0xa0971a>
140a09f58: 4d 01 c8                    	addq	%r9, %r8
140a09f5b: 49 83 c0 10                 	addq	$0x10, %r8
140a09f5f: 49 83 c1 10                 	addq	$0x10, %r9
140a09f63: 49 21 d0                    	andq	%rdx, %r8
140a09f66: e9 6e ff ff ff              	jmp	0x140a09ed9 <.text+0xa08ed9>
140a09f6b: 4e 8b 6c 19 f8              	movq	-0x8(%rcx,%r11), %r13
140a09f70: 48 c7 85 d8 08 00 00 00 00 00 00    	movq	$0x0, 0x8d8(%rbp)
140a09f7b: 48 c7 85 e0 08 00 00 04 00 00 00    	movq	$0x4, 0x8e0(%rbp)
140a09f86: 48 c7 85 e8 08 00 00 00 00 00 00    	movq	$0x0, 0x8e8(%rbp)
140a09f91: 48 c7 85 d0 03 00 00 01 00 00 00    	movq	$0x1, 0x3d0(%rbp)
140a09f9c: 48 c7 85 d8 03 00 00 1e 00 00 00    	movq	$0x1e, 0x3d8(%rbp)
140a09fa7: c6 85 e0 03 00 00 00        	movb	$0x0, 0x3e0(%rbp)
140a09fae: c6 85 04 09 00 00 01        	movb	$0x1, 0x904(%rbp)
140a09fb5: 4c 8d 05 6c c3 0b 02        	leaq	0x20bc36c(%rip), %r8    # 0x142ac6328
140a09fbc: 48 8d 8d 88 08 00 00        	leaq	0x888(%rbp), %rcx
140a09fc3: 48 8d 95 d0 03 00 00        	leaq	0x3d0(%rbp), %rdx
140a09fca: e8 11 50 90 00              	callq	0x14130efe0 <.text+0x130dfe0>
140a09fcf: 48 8d 85 c0 03 00 00        	leaq	0x3c0(%rbp), %rax
140a09fd6: 66 0f ef f6                 	pxor	%xmm6, %xmm6
140a09fda: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140a09fde: 66 0f 76 ff                 	pcmpeqd	%xmm7, %xmm7
140a09fe2: 45 31 c9                    	xorl	%r9d, %r9d
140a09fe5: 4d 39 e9                    	cmpq	%r13, %r9
140a09fe8: 0f 83 06 06 00 00           	jae	0x140a0a5f4 <.text+0xa095f4>
140a09fee: 4c 89 ad b0 08 00 00        	movq	%r13, 0x8b0(%rbp)
140a09ff5: 48 89 9d c0 08 00 00        	movq	%rbx, 0x8c0(%rbp)
140a09ffc: 4c 89 a5 a8 08 00 00        	movq	%r12, 0x8a8(%rbp)
140a0a003: 4c 89 bd b8 08 00 00        	movq	%r15, 0x8b8(%rbp)
140a0a00a: 4c 89 b5 c8 08 00 00        	movq	%r14, 0x8c8(%rbp)
140a0a011: f3 0f 7f 30                 	movdqu	%xmm6, (%rax)
140a0a015: 41 b8 e0 01 00 00           	movl	$0x1e0, %r8d            # imm = 0x1E0
140a0a01b: 31 d2                       	xorl	%edx, %edx
140a0a01d: 4c 89 ce                    	movq	%r9, %rsi
140a0a020: e8 fb 9b f1 01              	callq	0x142923c20 <.text+0x2922c20>
140a0a025: 48 89 b5 f8 08 00 00        	movq	%rsi, 0x8f8(%rbp)
140a0a02c: 48 8d 46 01                 	leaq	0x1(%rsi), %rax
140a0a030: 48 89 85 d0 08 00 00        	movq	%rax, 0x8d0(%rbp)
140a0a037: 41 b8 d0 01 00 00           	movl	$0x1d0, %r8d            # imm = 0x1D0
140a0a03d: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
140a0a044: 31 d2                       	xorl	%edx, %edx
140a0a046: e8 d5 9b f1 01              	callq	0x142923c20 <.text+0x2922c20>
140a0a04b: 4c 8b ad 90 08 00 00        	movq	0x890(%rbp), %r13
140a0a052: 48 8b 85 98 08 00 00        	movq	0x898(%rbp), %rax
140a0a059: 31 db                       	xorl	%ebx, %ebx
140a0a05b: 49 89 c7                    	movq	%rax, %r15
140a0a05e: 48 39 c3                    	cmpq	%rax, %rbx
140a0a061: 0f 84 1d 09 00 00           	je	0x140a0a984 <.text+0xa09984>
140a0a067: 48 6b c3 78                 	imulq	$0x78, %rbx, %rax
140a0a06b: 49 83 7c 05 50 00           	cmpq	$0x0, 0x50(%r13,%rax)
140a0a071: 0f 84 bb 06 00 00           	je	0x140a0a732 <.text+0xa09732>
140a0a077: 48 8d 4b 01                 	leaq	0x1(%rbx), %rcx
140a0a07b: 48 89 8d a0 08 00 00        	movq	%rcx, 0x8a0(%rbp)
140a0a082: 4e 8d 34 28                 	leaq	(%rax,%r13), %r14
140a0a086: 49 83 c6 38                 	addq	$0x38, %r14
140a0a08a: 49 8d 7e 20                 	leaq	0x20(%r14), %rdi
140a0a08e: 41 b8 0f 00 00 00           	movl	$0xf, %r8d
140a0a094: 48 89 f9                    	movq	%rdi, %rcx
140a0a097: 48 8d 15 b5 c4 0b 02        	leaq	0x20bc4b5(%rip), %rdx   # 0x142ac6553
140a0a09e: e8 cd 79 ba ff              	callq	0x1405b1a70 <.text+0x5b0a70>
140a0a0a3: 49 8b 36                    	movq	(%r14), %rsi
140a0a0a6: 4d 8b 66 08                 	movq	0x8(%r14), %r12
140a0a0aa: 4c 89 e1                    	movq	%r12, %rcx
140a0a0ad: 48 21 c1                    	andq	%rax, %rcx
140a0a0b0: 48 c1 e8 39                 	shrq	$0x39, %rax
140a0a0b4: 66 0f 6e c0                 	movd	%eax, %xmm0
140a0a0b8: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140a0a0bc: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140a0a0c1: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140a0a0c6: 4c 8d 76 c8                 	leaq	-0x38(%rsi), %r14
140a0a0ca: 31 c0                       	xorl	%eax, %eax
140a0a0cc: f3 0f 6f 0c 0e              	movdqu	(%rsi,%rcx), %xmm1
140a0a0d1: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
140a0a0d5: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
140a0a0d9: 66 0f d7 d2                 	pmovmskb	%xmm2, %edx
140a0a0dd: 85 d2                       	testl	%edx, %edx
140a0a0df: 74 55                       	je	0x140a0a136 <.text+0xa09136>
140a0a0e1: f3 44 0f bc c2              	tzcntl	%edx, %r8d
140a0a0e6: 49 01 c8                    	addq	%rcx, %r8
140a0a0e9: 4d 21 e0                    	andq	%r12, %r8
140a0a0ec: 49 f7 d8                    	negq	%r8
140a0a0ef: 4d 6b c0 38                 	imulq	$0x38, %r8, %r8
140a0a0f3: 4b 83 7c 06 10 0f           	cmpq	$0xf, 0x10(%r14,%r8)
140a0a0f9: 75 2e                       	jne	0x140a0a129 <.text+0xa09129>
140a0a0fb: 4f 8d 0c 06                 	leaq	(%r14,%r8), %r9
140a0a0ff: 4d 8b 49 08                 	movq	0x8(%r9), %r9
140a0a103: 4d 8b 11                    	movq	(%r9), %r10
140a0a106: 49 bb 56 65 72 74 65 78 5f 50       	movabsq	$0x505f786574726556, %r11 # imm = 0x505F786574726556
140a0a110: 4d 31 da                    	xorq	%r11, %r10
140a0a113: 4d 8b 49 07                 	movq	0x7(%r9), %r9
140a0a117: 49 bb 50 6f 73 69 74 69 6f 6e       	movabsq	$0x6e6f697469736f50, %r11 # imm = 0x6E6F697469736F50
140a0a121: 4d 31 d9                    	xorq	%r11, %r9
140a0a124: 4d 09 d1                    	orq	%r10, %r9
140a0a127: 74 37                       	je	0x140a0a160 <.text+0xa09160>
140a0a129: 44 8d 42 ff                 	leal	-0x1(%rdx), %r8d
140a0a12d: 66 41 21 d0                 	andw	%dx, %r8w
140a0a131: 44 89 c2                    	movl	%r8d, %edx
140a0a134: 75 ab                       	jne	0x140a0a0e1 <.text+0xa090e1>
140a0a136: 66 0f 74 cf                 	pcmpeqb	%xmm7, %xmm1
140a0a13a: 66 0f d7 d1                 	pmovmskb	%xmm1, %edx
140a0a13e: 85 d2                       	testl	%edx, %edx
140a0a140: 0f 85 ec 05 00 00           	jne	0x140a0a732 <.text+0xa09732>
140a0a146: 48 01 c1                    	addq	%rax, %rcx
140a0a149: 48 83 c1 10                 	addq	$0x10, %rcx
140a0a14d: 48 83 c0 10                 	addq	$0x10, %rax
140a0a151: 4c 21 e1                    	andq	%r12, %rcx
140a0a154: e9 73 ff ff ff              	jmp	0x140a0a0cc <.text+0xa090cc>
140a0a159: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140a0a160: 42 83 7c 06 e0 03           	cmpl	$0x3, -0x20(%rsi,%r8)
140a0a166: 0f 85 26 06 00 00           	jne	0x140a0a792 <.text+0xa09792>
140a0a16c: 49 01 f0                    	addq	%rsi, %r8
140a0a16f: 49 8b 50 f8                 	movq	-0x8(%r8), %rdx
140a0a173: 48 8b 8d f8 08 00 00        	movq	0x8f8(%rbp), %rcx
140a0a17a: 48 39 d1                    	cmpq	%rdx, %rcx
140a0a17d: 0f 83 1c 08 00 00           	jae	0x140a0a99f <.text+0xa0999f>
140a0a183: 49 8b 40 f0                 	movq	-0x10(%r8), %rax
140a0a187: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140a0a18b: f3 0f 10 44 88 08           	movss	0x8(%rax,%rcx,4), %xmm0
140a0a191: 48 c1 e3 04                 	shlq	$0x4, %rbx
140a0a195: 48 89 8d f0 08 00 00        	movq	%rcx, 0x8f0(%rbp)
140a0a19c: f2 0f 10 0c 88              	movsd	(%rax,%rcx,4), %xmm1
140a0a1a1: f2 0f 11 4c 1d 10           	movsd	%xmm1, 0x10(%rbp,%rbx)
140a0a1a7: f3 0f 11 44 1d 18           	movss	%xmm0, 0x18(%rbp,%rbx)
140a0a1ad: c7 44 1d 1c 00 00 00 00     	movl	$0x0, 0x1c(%rbp,%rbx)
140a0a1b5: 41 b8 0d 00 00 00           	movl	$0xd, %r8d
140a0a1bb: 48 89 f9                    	movq	%rdi, %rcx
140a0a1be: 48 8d 15 9d c3 0b 02        	leaq	0x20bc39d(%rip), %rdx   # 0x142ac6562
140a0a1c5: e8 a6 78 ba ff              	callq	0x1405b1a70 <.text+0x5b0a70>
140a0a1ca: 48 89 c1                    	movq	%rax, %rcx
140a0a1cd: 48 c1 e9 39                 	shrq	$0x39, %rcx
140a0a1d1: 66 0f 6e c1                 	movd	%ecx, %xmm0
140a0a1d5: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140a0a1d9: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140a0a1de: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140a0a1e3: 31 c9                       	xorl	%ecx, %ecx
140a0a1e5: 4c 21 e0                    	andq	%r12, %rax
140a0a1e8: f3 0f 6f 0c 06              	movdqu	(%rsi,%rax), %xmm1
140a0a1ed: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
140a0a1f1: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
140a0a1f5: 66 0f d7 d2                 	pmovmskb	%xmm2, %edx
140a0a1f9: 85 d2                       	testl	%edx, %edx
140a0a1fb: 74 55                       	je	0x140a0a252 <.text+0xa09252>
140a0a1fd: f3 44 0f bc c2              	tzcntl	%edx, %r8d
140a0a202: 49 01 c0                    	addq	%rax, %r8
140a0a205: 4d 21 e0                    	andq	%r12, %r8
140a0a208: 49 f7 d8                    	negq	%r8
140a0a20b: 4d 6b c0 38                 	imulq	$0x38, %r8, %r8
140a0a20f: 4b 83 7c 06 10 0d           	cmpq	$0xd, 0x10(%r14,%r8)
140a0a215: 75 2e                       	jne	0x140a0a245 <.text+0xa09245>
140a0a217: 4f 8d 0c 06                 	leaq	(%r14,%r8), %r9
140a0a21b: 4d 8b 49 08                 	movq	0x8(%r9), %r9
140a0a21f: 4d 8b 11                    	movq	(%r9), %r10
140a0a222: 49 bb 56 65 72 74 65 78 5f 4e       	movabsq	$0x4e5f786574726556, %r11 # imm = 0x4E5F786574726556
140a0a22c: 4d 31 da                    	xorq	%r11, %r10
140a0a22f: 4d 8b 49 05                 	movq	0x5(%r9), %r9
140a0a233: 49 bb 78 5f 4e 6f 72 6d 61 6c       	movabsq	$0x6c616d726f4e5f78, %r11 # imm = 0x6C616D726F4E5F78
140a0a23d: 4d 31 d9                    	xorq	%r11, %r9
140a0a240: 4d 09 d1                    	orq	%r10, %r9
140a0a243: 74 2d                       	je	0x140a0a272 <.text+0xa09272>
140a0a245: 44 8d 42 ff                 	leal	-0x1(%rdx), %r8d
140a0a249: 66 41 21 d0                 	andw	%dx, %r8w
140a0a24d: 44 89 c2                    	movl	%r8d, %edx
140a0a250: 75 ab                       	jne	0x140a0a1fd <.text+0xa091fd>
140a0a252: 66 0f 74 cf                 	pcmpeqb	%xmm7, %xmm1
140a0a256: 66 0f d7 d1                 	pmovmskb	%xmm1, %edx
140a0a25a: 85 d2                       	testl	%edx, %edx
140a0a25c: 0f 85 e8 04 00 00           	jne	0x140a0a74a <.text+0xa0974a>
140a0a262: 48 01 c8                    	addq	%rcx, %rax
140a0a265: 48 83 c0 10                 	addq	$0x10, %rax
140a0a269: 48 83 c1 10                 	addq	$0x10, %rcx
140a0a26d: e9 73 ff ff ff              	jmp	0x140a0a1e5 <.text+0xa091e5>
140a0a272: 42 83 7c 06 e0 03           	cmpl	$0x3, -0x20(%rsi,%r8)
140a0a278: 0f 85 25 05 00 00           	jne	0x140a0a7a3 <.text+0xa097a3>
140a0a27e: 4c 01 c6                    	addq	%r8, %rsi
140a0a281: 48 8b 56 f8                 	movq	-0x8(%rsi), %rdx
140a0a285: 48 39 95 f8 08 00 00        	cmpq	%rdx, 0x8f8(%rbp)
140a0a28c: 0f 83 22 07 00 00           	jae	0x140a0a9b4 <.text+0xa099b4>
140a0a292: 48 8b 46 f0                 	movq	-0x10(%rsi), %rax
140a0a296: 48 8b 8d f0 08 00 00        	movq	0x8f0(%rbp), %rcx
140a0a29d: 66 0f 6e 44 88 08           	movd	0x8(%rax,%rcx,4), %xmm0
140a0a2a3: f3 0f 7e 0c 88              	movq	(%rax,%rcx,4), %xmm1
140a0a2a8: 66 0f d6 8c 1d f0 01 00 00  	movq	%xmm1, 0x1f0(%rbp,%rbx)
140a0a2b1: 66 0f 7e 84 1d f8 01 00 00  	movd	%xmm0, 0x1f8(%rbp,%rbx)
140a0a2ba: c7 84 1d fc 01 00 00 00 00 00 00    	movl	$0x0, 0x1fc(%rbp,%rbx)
140a0a2c5: 48 8b 85 a0 08 00 00        	movq	0x8a0(%rbp), %rax
140a0a2cc: 48 89 c3                    	movq	%rax, %rbx
140a0a2cf: 48 83 f8 1e                 	cmpq	$0x1e, %rax
140a0a2d3: 4c 89 f8                    	movq	%r15, %rax
140a0a2d6: 0f 85 7f fd ff ff           	jne	0x140a0a05b <.text+0xa0905b>
140a0a2dc: 48 83 bd e0 07 00 00 00     	cmpq	$0x0, 0x7e0(%rbp)
140a0a2e4: 4c 8b a5 a8 08 00 00        	movq	0x8a8(%rbp), %r12
140a0a2eb: 4c 8b ad b0 08 00 00        	movq	0x8b0(%rbp), %r13
140a0a2f2: 4c 8d b5 e8 07 00 00        	leaq	0x7e8(%rbp), %r14
140a0a2f9: 0f 84 63 04 00 00           	je	0x140a0a762 <.text+0xa09762>
140a0a2ff: 41 b8 0f 00 00 00           	movl	$0xf, %r8d
140a0a305: 4c 89 f1                    	movq	%r14, %rcx
140a0a308: 48 8d 15 44 c2 0b 02        	leaq	0x20bc244(%rip), %rdx   # 0x142ac6553
140a0a30f: e8 5c 77 ba ff              	callq	0x1405b1a70 <.text+0x5b0a70>
140a0a314: 48 8b b5 c8 07 00 00        	movq	0x7c8(%rbp), %rsi
140a0a31b: 48 8b bd d0 07 00 00        	movq	0x7d0(%rbp), %rdi
140a0a322: 48 89 f9                    	movq	%rdi, %rcx
140a0a325: 48 21 c1                    	andq	%rax, %rcx
140a0a328: 48 c1 e8 39                 	shrq	$0x39, %rax
140a0a32c: 66 0f 6e c0                 	movd	%eax, %xmm0
140a0a330: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140a0a334: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140a0a339: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140a0a33e: 48 8d 5e c8                 	leaq	-0x38(%rsi), %rbx
140a0a342: 31 c0                       	xorl	%eax, %eax
140a0a344: f3 0f 6f 0c 0e              	movdqu	(%rsi,%rcx), %xmm1
140a0a349: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
140a0a34d: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
140a0a351: 66 0f d7 d2                 	pmovmskb	%xmm2, %edx
140a0a355: 85 d2                       	testl	%edx, %edx
140a0a357: 74 55                       	je	0x140a0a3ae <.text+0xa093ae>
140a0a359: f3 44 0f bc c2              	tzcntl	%edx, %r8d
140a0a35e: 49 01 c8                    	addq	%rcx, %r8
140a0a361: 49 21 f8                    	andq	%rdi, %r8
140a0a364: 49 f7 d8                    	negq	%r8
140a0a367: 4d 6b c0 38                 	imulq	$0x38, %r8, %r8
140a0a36b: 4a 83 7c 03 10 0f           	cmpq	$0xf, 0x10(%rbx,%r8)
140a0a371: 75 2e                       	jne	0x140a0a3a1 <.text+0xa093a1>
140a0a373: 4e 8d 0c 03                 	leaq	(%rbx,%r8), %r9
140a0a377: 4d 8b 49 08                 	movq	0x8(%r9), %r9
140a0a37b: 4d 8b 11                    	movq	(%r9), %r10
140a0a37e: 49 bb 56 65 72 74 65 78 5f 50       	movabsq	$0x505f786574726556, %r11 # imm = 0x505F786574726556
140a0a388: 4d 31 da                    	xorq	%r11, %r10
140a0a38b: 4d 8b 49 07                 	movq	0x7(%r9), %r9
140a0a38f: 49 bb 50 6f 73 69 74 69 6f 6e       	movabsq	$0x6e6f697469736f50, %r11 # imm = 0x6E6F697469736F50
140a0a399: 4d 31 d9                    	xorq	%r11, %r9
140a0a39c: 4d 09 d1                    	orq	%r10, %r9
140a0a39f: 74 30                       	je	0x140a0a3d1 <.text+0xa093d1>
140a0a3a1: 44 8d 42 ff                 	leal	-0x1(%rdx), %r8d
140a0a3a5: 66 41 21 d0                 	andw	%dx, %r8w
140a0a3a9: 44 89 c2                    	movl	%r8d, %edx
140a0a3ac: 75 ab                       	jne	0x140a0a359 <.text+0xa09359>
140a0a3ae: 66 0f 74 cf                 	pcmpeqb	%xmm7, %xmm1
140a0a3b2: 66 0f d7 d1                 	pmovmskb	%xmm1, %edx
140a0a3b6: 85 d2                       	testl	%edx, %edx
140a0a3b8: 0f 85 a4 03 00 00           	jne	0x140a0a762 <.text+0xa09762>
140a0a3be: 48 01 c1                    	addq	%rax, %rcx
140a0a3c1: 48 83 c1 10                 	addq	$0x10, %rcx
140a0a3c5: 48 83 c0 10                 	addq	$0x10, %rax
140a0a3c9: 48 21 f9                    	andq	%rdi, %rcx
140a0a3cc: e9 73 ff ff ff              	jmp	0x140a0a344 <.text+0xa09344>
140a0a3d1: 42 83 7c 06 e0 03           	cmpl	$0x3, -0x20(%rsi,%r8)
140a0a3d7: 0f 85 46 04 00 00           	jne	0x140a0a823 <.text+0xa09823>
140a0a3dd: 49 01 f0                    	addq	%rsi, %r8
140a0a3e0: 49 8b 50 f8                 	movq	-0x8(%r8), %rdx
140a0a3e4: 48 39 95 f8 08 00 00        	cmpq	%rdx, 0x8f8(%rbp)
140a0a3eb: 0f 83 df 05 00 00           	jae	0x140a0a9d0 <.text+0xa099d0>
140a0a3f1: 49 8b 40 f0                 	movq	-0x10(%r8), %rax
140a0a3f5: 48 8b 8d f0 08 00 00        	movq	0x8f0(%rbp), %rcx
140a0a3fc: f2 44 0f 10 04 88           	movsd	(%rax,%rcx,4), %xmm8
140a0a402: f3 44 0f 10 4c 88 08        	movss	0x8(%rax,%rcx,4), %xmm9
140a0a409: 41 b8 0d 00 00 00           	movl	$0xd, %r8d
140a0a40f: 4c 89 f1                    	movq	%r14, %rcx
140a0a412: 48 8d 15 49 c1 0b 02        	leaq	0x20bc149(%rip), %rdx   # 0x142ac6562
140a0a419: e8 52 76 ba ff              	callq	0x1405b1a70 <.text+0x5b0a70>
140a0a41e: 48 89 c1                    	movq	%rax, %rcx
140a0a421: 48 c1 e9 39                 	shrq	$0x39, %rcx
140a0a425: 66 0f 6e c1                 	movd	%ecx, %xmm0
140a0a429: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140a0a42d: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140a0a432: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140a0a437: 31 c9                       	xorl	%ecx, %ecx
140a0a439: 48 21 f8                    	andq	%rdi, %rax
140a0a43c: f3 0f 6f 0c 06              	movdqu	(%rsi,%rax), %xmm1
140a0a441: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
140a0a445: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
140a0a449: 66 0f d7 d2                 	pmovmskb	%xmm2, %edx
140a0a44d: 85 d2                       	testl	%edx, %edx
140a0a44f: 74 55                       	je	0x140a0a4a6 <.text+0xa094a6>
140a0a451: f3 44 0f bc c2              	tzcntl	%edx, %r8d
140a0a456: 49 01 c0                    	addq	%rax, %r8
140a0a459: 49 21 f8                    	andq	%rdi, %r8
140a0a45c: 49 f7 d8                    	negq	%r8
140a0a45f: 4d 6b c0 38                 	imulq	$0x38, %r8, %r8
140a0a463: 4a 83 7c 03 10 0d           	cmpq	$0xd, 0x10(%rbx,%r8)
140a0a469: 75 2e                       	jne	0x140a0a499 <.text+0xa09499>
140a0a46b: 4e 8d 0c 03                 	leaq	(%rbx,%r8), %r9
140a0a46f: 4d 8b 49 08                 	movq	0x8(%r9), %r9
140a0a473: 4d 8b 11                    	movq	(%r9), %r10
140a0a476: 49 bb 56 65 72 74 65 78 5f 4e       	movabsq	$0x4e5f786574726556, %r11 # imm = 0x4E5F786574726556
140a0a480: 4d 31 da                    	xorq	%r11, %r10
140a0a483: 4d 8b 49 05                 	movq	0x5(%r9), %r9
140a0a487: 49 bb 78 5f 4e 6f 72 6d 61 6c       	movabsq	$0x6c616d726f4e5f78, %r11 # imm = 0x6C616D726F4E5F78
140a0a491: 4d 31 d9                    	xorq	%r11, %r9
140a0a494: 4d 09 d1                    	orq	%r10, %r9
140a0a497: 74 2d                       	je	0x140a0a4c6 <.text+0xa094c6>
140a0a499: 44 8d 42 ff                 	leal	-0x1(%rdx), %r8d
140a0a49d: 66 41 21 d0                 	andw	%dx, %r8w
140a0a4a1: 44 89 c2                    	movl	%r8d, %edx
140a0a4a4: 75 ab                       	jne	0x140a0a451 <.text+0xa09451>
140a0a4a6: 66 0f 74 cf                 	pcmpeqb	%xmm7, %xmm1
140a0a4aa: 66 0f d7 d1                 	pmovmskb	%xmm1, %edx
140a0a4ae: 85 d2                       	testl	%edx, %edx
140a0a4b0: 0f 85 c4 02 00 00           	jne	0x140a0a77a <.text+0xa0977a>
140a0a4b6: 48 01 c8                    	addq	%rcx, %rax
140a0a4b9: 48 83 c0 10                 	addq	$0x10, %rax
140a0a4bd: 48 83 c1 10                 	addq	$0x10, %rcx
140a0a4c1: e9 73 ff ff ff              	jmp	0x140a0a439 <.text+0xa09439>
140a0a4c6: 42 83 7c 06 e0 03           	cmpl	$0x3, -0x20(%rsi,%r8)
140a0a4cc: 0f 85 62 03 00 00           	jne	0x140a0a834 <.text+0xa09834>
140a0a4d2: 4c 01 c6                    	addq	%r8, %rsi
140a0a4d5: 48 8b 56 f8                 	movq	-0x8(%rsi), %rdx
140a0a4d9: 48 8b 8d f8 08 00 00        	movq	0x8f8(%rbp), %rcx
140a0a4e0: 48 39 d1                    	cmpq	%rdx, %rcx
140a0a4e3: 0f 83 03 05 00 00           	jae	0x140a0a9ec <.text+0xa099ec>
140a0a4e9: 48 8b 46 f0                 	movq	-0x10(%rsi), %rax
140a0a4ed: 48 8b 8d f0 08 00 00        	movq	0x8f0(%rbp), %rcx
140a0a4f4: f2 44 0f 10 14 88           	movsd	(%rax,%rcx,4), %xmm10
140a0a4fa: f3 44 0f 10 5c 88 08        	movss	0x8(%rax,%rcx,4), %xmm11
140a0a501: 41 b8 e0 01 00 00           	movl	$0x1e0, %r8d            # imm = 0x1E0
140a0a507: 48 8d 8d d0 03 00 00        	leaq	0x3d0(%rbp), %rcx
140a0a50e: 48 8d 55 10                 	leaq	0x10(%rbp), %rdx
140a0a512: e8 e9 96 f1 01              	callq	0x142923c00 <.text+0x2922c00>
140a0a517: 41 b8 e0 01 00 00           	movl	$0x1e0, %r8d            # imm = 0x1E0
140a0a51d: 48 8d 8d b0 05 00 00        	leaq	0x5b0(%rbp), %rcx
140a0a524: 48 8d 95 f0 01 00 00        	leaq	0x1f0(%rbp), %rdx
140a0a52b: e8 d0 96 f1 01              	callq	0x142923c00 <.text+0x2922c00>
140a0a530: 48 8b b5 e8 08 00 00        	movq	0x8e8(%rbp), %rsi
140a0a537: 48 3b b5 d8 08 00 00        	cmpq	0x8d8(%rbp), %rsi
140a0a53e: 4c 8b b5 c8 08 00 00        	movq	0x8c8(%rbp), %r14
140a0a545: 4c 8b bd b8 08 00 00        	movq	0x8b8(%rbp), %r15
140a0a54c: 75 1a                       	jne	0x140a0a568 <.text+0xa09568>
140a0a54e: c6 85 07 09 00 00 01        	movb	$0x1, 0x907(%rbp)
140a0a555: 48 8d 8d d8 08 00 00        	leaq	0x8d8(%rbp), %rcx
140a0a55c: 48 8d 15 65 c7 0b 02        	leaq	0x20bc765(%rip), %rdx   # 0x142ac6cc8
140a0a563: e8 08 56 15 00              	callq	0x140b5fb70 <.text+0xb5eb70>
140a0a568: 48 8b bd e0 08 00 00        	movq	0x8e0(%rbp), %rdi
140a0a56f: 48 69 de e0 03 00 00        	imulq	$0x3e0, %rsi, %rbx      # imm = 0x3E0
140a0a576: 48 8d 0c 1f                 	leaq	(%rdi,%rbx), %rcx
140a0a57a: 41 b8 c0 03 00 00           	movl	$0x3c0, %r8d            # imm = 0x3C0
140a0a580: 48 8d 95 d0 03 00 00        	leaq	0x3d0(%rbp), %rdx
140a0a587: e8 74 96 f1 01              	callq	0x142923c00 <.text+0x2922c00>
140a0a58c: 44 0f 13 84 1f c0 03 00 00  	movlps	%xmm8, 0x3c0(%rdi,%rbx)
140a0a595: f3 44 0f 11 8c 1f c8 03 00 00       	movss	%xmm9, 0x3c8(%rdi,%rbx)
140a0a59f: c7 84 1f cc 03 00 00 00 00 00 00    	movl	$0x0, 0x3cc(%rdi,%rbx)
140a0a5aa: 44 0f 13 94 1f d0 03 00 00  	movlps	%xmm10, 0x3d0(%rdi,%rbx)
140a0a5b3: f3 44 0f 11 9c 1f d8 03 00 00       	movss	%xmm11, 0x3d8(%rdi,%rbx)
140a0a5bd: c7 84 1f dc 03 00 00 00 00 00 00    	movl	$0x0, 0x3dc(%rdi,%rbx)
140a0a5c8: 48 ff c6                    	incq	%rsi
140a0a5cb: 48 89 b5 e8 08 00 00        	movq	%rsi, 0x8e8(%rbp)
140a0a5d2: 4c 8b 8d d0 08 00 00        	movq	0x8d0(%rbp), %r9
140a0a5d9: 48 8b 9d c0 08 00 00        	movq	0x8c0(%rbp), %rbx
140a0a5e0: 48 8d 85 c0 03 00 00        	leaq	0x3c0(%rbp), %rax
140a0a5e7: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140a0a5eb: 4d 39 e9                    	cmpq	%r13, %r9
140a0a5ee: 0f 82 fa f9 ff ff           	jb	0x140a09fee <.text+0xa08fee>
140a0a5f4: 48 8b 33                    	movq	(%rbx), %rsi
140a0a5f7: 48 8b 16                    	movq	(%rsi), %rdx
140a0a5fa: 48 83 c2 10                 	addq	$0x10, %rdx
140a0a5fe: 48 8b 85 00 0a 00 00        	movq	0xa00(%rbp), %rax
140a0a605: 48 8b 08                    	movq	(%rax), %rcx
140a0a608: 4c 8b 85 e0 08 00 00        	movq	0x8e0(%rbp), %r8
140a0a60f: 4c 8b 8d e8 08 00 00        	movq	0x8e8(%rbp), %r9
140a0a616: c6 85 07 09 00 00 01        	movb	$0x1, 0x907(%rbp)
140a0a61d: e8 ce 11 8b 00              	callq	0x1412bb7f0 <.text+0x12ba7f0>
140a0a622: 48 8b 85 80 08 00 00        	movq	0x880(%rbp), %rax
140a0a629: 48 89 85 40 04 00 00        	movq	%rax, 0x440(%rbp)
140a0a630: 0f 28 85 70 08 00 00        	movaps	0x870(%rbp), %xmm0
140a0a637: 0f 29 85 30 04 00 00        	movaps	%xmm0, 0x430(%rbp)
140a0a63e: 0f 28 85 60 08 00 00        	movaps	0x860(%rbp), %xmm0
140a0a645: 0f 29 85 20 04 00 00        	movaps	%xmm0, 0x420(%rbp)
140a0a64c: 0f 28 85 50 08 00 00        	movaps	0x850(%rbp), %xmm0
140a0a653: 0f 29 85 10 04 00 00        	movaps	%xmm0, 0x410(%rbp)
140a0a65a: 66 0f 6f 85 10 08 00 00     	movdqa	0x810(%rbp), %xmm0
140a0a662: 66 0f 6f 8d 20 08 00 00     	movdqa	0x820(%rbp), %xmm1
140a0a66a: 66 0f 6f 95 30 08 00 00     	movdqa	0x830(%rbp), %xmm2
140a0a672: 66 0f 6f 9d 40 08 00 00     	movdqa	0x840(%rbp), %xmm3
140a0a67a: 66 0f 7f 9d 00 04 00 00     	movdqa	%xmm3, 0x400(%rbp)
140a0a682: 66 0f 7f 95 f0 03 00 00     	movdqa	%xmm2, 0x3f0(%rbp)
140a0a68a: 66 0f 7f 8d e0 03 00 00     	movdqa	%xmm1, 0x3e0(%rbp)
140a0a692: 66 0f 7f 85 d0 03 00 00     	movdqa	%xmm0, 0x3d0(%rbp)
140a0a69a: 4d 8b 0c 24                 	movq	(%r12), %r9
140a0a69e: c6 85 07 09 00 00 00        	movb	$0x0, 0x907(%rbp)
140a0a6a5: 4c 89 74 24 20              	movq	%r14, 0x20(%rsp)
140a0a6aa: 48 8d 15 67 c3 0b 02        	leaq	0x20bc367(%rip), %rdx   # 0x142ac6a18
140a0a6b1: 48 8d 8d d0 03 00 00        	leaq	0x3d0(%rbp), %rcx
140a0a6b8: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
140a0a6be: e8 dd 05 11 00              	callq	0x140b1aca0 <.text+0xb19ca0>
140a0a6c3: 48 89 85 f8 08 00 00        	movq	%rax, 0x8f8(%rbp)
140a0a6ca: c6 85 04 09 00 00 00        	movb	$0x0, 0x904(%rbp)
140a0a6d1: 48 8d 8d 88 08 00 00        	leaq	0x888(%rbp), %rcx
140a0a6d8: e8 23 55 e8 ff              	callq	0x14088fc00 <.text+0x88ec00>
140a0a6dd: 48 8b 85 d8 08 00 00        	movq	0x8d8(%rbp), %rax
140a0a6e4: 48 85 c0                    	testq	%rax, %rax
140a0a6e7: 74 19                       	je	0x140a0a702 <.text+0xa09702>
140a0a6e9: 48 8b 8d e0 08 00 00        	movq	0x8e0(%rbp), %rcx
140a0a6f0: 48 69 d0 e0 03 00 00        	imulq	$0x3e0, %rax, %rdx      # imm = 0x3E0
140a0a6f7: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a0a6fd: e8 1e 95 c0 ff              	callq	0x140613c20 <.text+0x612c20>
140a0a702: c6 85 05 09 00 00 00        	movb	$0x0, 0x905(%rbp)
140a0a709: 48 8d 8d 90 07 00 00        	leaq	0x790(%rbp), %rcx
140a0a710: e8 1b e4 e7 ff              	callq	0x140888b30 <.text+0x887b30>
140a0a715: e9 61 ed ff ff              	jmp	0x140a0947b <.text+0xa0847b>
140a0a71a: c6 85 03 09 00 00 01        	movb	$0x1, 0x903(%rbp)
140a0a721: 48 8d 0d b0 c3 0b 02        	leaq	0x20bc3b0(%rip), %rcx   # 0x142ac6ad8
140a0a728: e8 e3 eb ec 01              	callq	0x1428d9310 <.text+0x28d8310>
140a0a72d: e9 cd 02 00 00              	jmp	0x140a0a9ff <.text+0xa099ff>
140a0a732: c6 85 07 09 00 00 01        	movb	$0x1, 0x907(%rbp)
140a0a739: 48 8d 0d e8 c5 0b 02        	leaq	0x20bc5e8(%rip), %rcx   # 0x142ac6d28
140a0a740: e8 cb eb ec 01              	callq	0x1428d9310 <.text+0x28d8310>
140a0a745: e9 b5 02 00 00              	jmp	0x140a0a9ff <.text+0xa099ff>
140a0a74a: c6 85 07 09 00 00 01        	movb	$0x1, 0x907(%rbp)
140a0a751: 48 8d 0d 00 c6 0b 02        	leaq	0x20bc600(%rip), %rcx   # 0x142ac6d58
140a0a758: e8 b3 eb ec 01              	callq	0x1428d9310 <.text+0x28d8310>
140a0a75d: e9 9d 02 00 00              	jmp	0x140a0a9ff <.text+0xa099ff>
140a0a762: c6 85 07 09 00 00 01        	movb	$0x1, 0x907(%rbp)
140a0a769: 48 8d 0d f8 c4 0b 02        	leaq	0x20bc4f8(%rip), %rcx   # 0x142ac6c68
140a0a770: e8 9b eb ec 01              	callq	0x1428d9310 <.text+0x28d8310>
140a0a775: e9 85 02 00 00              	jmp	0x140a0a9ff <.text+0xa099ff>
140a0a77a: c6 85 07 09 00 00 01        	movb	$0x1, 0x907(%rbp)
140a0a781: 48 8d 0d 10 c5 0b 02        	leaq	0x20bc510(%rip), %rcx   # 0x142ac6c98
140a0a788: e8 83 eb ec 01              	callq	0x1428d9310 <.text+0x28d8310>
140a0a78d: e9 6d 02 00 00              	jmp	0x140a0a9ff <.text+0xa099ff>
140a0a792: c6 85 07 09 00 00 01        	movb	$0x1, 0x907(%rbp)
140a0a799: e8 82 db e9 01              	callq	0x1428a8320 <.text+0x28a7320>
140a0a79e: e9 5c 02 00 00              	jmp	0x140a0a9ff <.text+0xa099ff>
140a0a7a3: c6 85 07 09 00 00 01        	movb	$0x1, 0x907(%rbp)
140a0a7aa: e8 91 db e9 01              	callq	0x1428a8340 <.text+0x28a7340>
140a0a7af: e9 4b 02 00 00              	jmp	0x140a0a9ff <.text+0xa099ff>
140a0a7b4: c6 85 06 09 00 00 01        	movb	$0x1, 0x906(%rbp)
140a0a7bb: b9 08 00 00 00              	movl	$0x8, %ecx
140a0a7c0: ba 18 00 00 00              	movl	$0x18, %edx
140a0a7c5: e8 36 e8 ec 01              	callq	0x1428d9000 <.text+0x28d8000>
140a0a7ca: e9 30 02 00 00              	jmp	0x140a0a9ff <.text+0xa099ff>
140a0a7cf: 48 8b 85 d8 03 00 00        	movq	0x3d8(%rbp), %rax
140a0a7d6: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
140a0a7dd: 48 8d 05 4c c4 0b 02        	leaq	0x20bc44c(%rip), %rax   # 0x142ac6c30
140a0a7e4: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a0a7e9: 48 8d 0d 70 bb 0b 02        	leaq	0x20bbb70(%rip), %rcx   # 0x142ac6360
140a0a7f0: 4c 8d 0d 49 bb 0b 02        	leaq	0x20bbb49(%rip), %r9    # 0x142ac6340
140a0a7f7: 4c 8d 85 f0 01 00 00        	leaq	0x1f0(%rbp), %r8
140a0a7fe: ba 2b 00 00 00              	movl	$0x2b, %edx
140a0a803: e8 58 ef ec 01              	callq	0x1428d9760 <.text+0x28d8760>
140a0a808: e9 f2 01 00 00              	jmp	0x140a0a9ff <.text+0xa099ff>
140a0a80d: 4c 8d 05 9c 5f 0c 02        	leaq	0x20c5f9c(%rip), %r8    # 0x142ad07b0
140a0a814: b9 01 00 00 00              	movl	$0x1, %ecx
140a0a819: ba 15 00 00 00              	movl	$0x15, %edx
140a0a81e: e8 c0 e7 ec 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140a0a823: c6 85 07 09 00 00 01        	movb	$0x1, 0x907(%rbp)
140a0a82a: e8 31 db e9 01              	callq	0x1428a8360 <.text+0x28a7360>
140a0a82f: e9 cb 01 00 00              	jmp	0x140a0a9ff <.text+0xa099ff>
140a0a834: c6 85 07 09 00 00 01        	movb	$0x1, 0x907(%rbp)
140a0a83b: e8 40 db e9 01              	callq	0x1428a8380 <.text+0x28a7380>
140a0a840: e9 ba 01 00 00              	jmp	0x140a0a9ff <.text+0xa099ff>
140a0a845: 48 8b 85 d8 03 00 00        	movq	0x3d8(%rbp), %rax
140a0a84c: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
140a0a853: 48 8d 05 d6 c2 0b 02        	leaq	0x20bc2d6(%rip), %rax   # 0x142ac6b30
140a0a85a: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a0a85f: 48 8d 0d fa ba 0b 02        	leaq	0x20bbafa(%rip), %rcx   # 0x142ac6360
140a0a866: 4c 8d 0d d3 ba 0b 02        	leaq	0x20bbad3(%rip), %r9    # 0x142ac6340
140a0a86d: 4c 8d 85 f0 01 00 00        	leaq	0x1f0(%rbp), %r8
140a0a874: ba 2b 00 00 00              	movl	$0x2b, %edx
140a0a879: e8 e2 ee ec 01              	callq	0x1428d9760 <.text+0x28d8760>
140a0a87e: e9 7c 01 00 00              	jmp	0x140a0a9ff <.text+0xa099ff>
140a0a883: 48 8b 85 d8 03 00 00        	movq	0x3d8(%rbp), %rax
140a0a88a: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
140a0a891: 48 8d 05 d8 c2 0b 02        	leaq	0x20bc2d8(%rip), %rax   # 0x142ac6b70
140a0a898: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a0a89d: 48 8d 0d bc ba 0b 02        	leaq	0x20bbabc(%rip), %rcx   # 0x142ac6360
140a0a8a4: 4c 8d 0d 95 ba 0b 02        	leaq	0x20bba95(%rip), %r9    # 0x142ac6340
140a0a8ab: 4c 8d 85 f0 01 00 00        	leaq	0x1f0(%rbp), %r8
140a0a8b2: ba 2b 00 00 00              	movl	$0x2b, %edx
140a0a8b7: e8 a4 ee ec 01              	callq	0x1428d9760 <.text+0x28d8760>
140a0a8bc: e9 3e 01 00 00              	jmp	0x140a0a9ff <.text+0xa099ff>
140a0a8c1: 48 8b 85 d8 03 00 00        	movq	0x3d8(%rbp), %rax
140a0a8c8: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
140a0a8cf: 48 8d 05 da c2 0b 02        	leaq	0x20bc2da(%rip), %rax   # 0x142ac6bb0
140a0a8d6: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a0a8db: 48 8d 0d 7e ba 0b 02        	leaq	0x20bba7e(%rip), %rcx   # 0x142ac6360
140a0a8e2: 4c 8d 0d 57 ba 0b 02        	leaq	0x20bba57(%rip), %r9    # 0x142ac6340
140a0a8e9: 4c 8d 85 f0 01 00 00        	leaq	0x1f0(%rbp), %r8
140a0a8f0: ba 2b 00 00 00              	movl	$0x2b, %edx
140a0a8f5: e8 66 ee ec 01              	callq	0x1428d9760 <.text+0x28d8760>
140a0a8fa: e9 00 01 00 00              	jmp	0x140a0a9ff <.text+0xa099ff>
140a0a8ff: 48 8d 0d e2 c2 0b 02        	leaq	0x20bc2e2(%rip), %rcx   # 0x142ac6be8
140a0a906: e8 05 ea ec 01              	callq	0x1428d9310 <.text+0x28d8310>
140a0a90b: 48 8b 85 d8 03 00 00        	movq	0x3d8(%rbp), %rax
140a0a912: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
140a0a919: 48 8d 05 58 c1 0b 02        	leaq	0x20bc158(%rip), %rax   # 0x142ac6a78
140a0a920: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a0a925: 48 8d 0d 34 ba 0b 02        	leaq	0x20bba34(%rip), %rcx   # 0x142ac6360
140a0a92c: 4c 8d 0d 0d ba 0b 02        	leaq	0x20bba0d(%rip), %r9    # 0x142ac6340
140a0a933: 4c 8d 85 f0 01 00 00        	leaq	0x1f0(%rbp), %r8
140a0a93a: ba 2b 00 00 00              	movl	$0x2b, %edx
140a0a93f: e8 1c ee ec 01              	callq	0x1428d9760 <.text+0x28d8760>
140a0a944: e9 b6 00 00 00              	jmp	0x140a0a9ff <.text+0xa099ff>
140a0a949: 48 8b 85 d8 03 00 00        	movq	0x3d8(%rbp), %rax
140a0a950: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
140a0a957: 48 8d 05 62 c1 0b 02        	leaq	0x20bc162(%rip), %rax   # 0x142ac6ac0
140a0a95e: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a0a963: 48 8d 0d f6 b9 0b 02        	leaq	0x20bb9f6(%rip), %rcx   # 0x142ac6360
140a0a96a: 4c 8d 0d cf b9 0b 02        	leaq	0x20bb9cf(%rip), %r9    # 0x142ac6340
140a0a971: 4c 8d 85 f0 01 00 00        	leaq	0x1f0(%rbp), %r8
140a0a978: ba 2b 00 00 00              	movl	$0x2b, %edx
140a0a97d: e8 de ed ec 01              	callq	0x1428d9760 <.text+0x28d8760>
140a0a982: eb 7b                       	jmp	0x140a0a9ff <.text+0xa099ff>
140a0a984: c6 85 07 09 00 00 01        	movb	$0x1, 0x907(%rbp)
140a0a98b: 4c 8d 05 7e c3 0b 02        	leaq	0x20bc37e(%rip), %r8    # 0x142ac6d10
140a0a992: 4c 89 f9                    	movq	%r15, %rcx
140a0a995: 4c 89 fa                    	movq	%r15, %rdx
140a0a998: e8 7b eb ec 01              	callq	0x1428d9518 <.text+0x28d8518>
140a0a99d: eb 60                       	jmp	0x140a0a9ff <.text+0xa099ff>
140a0a99f: c6 85 07 09 00 00 01        	movb	$0x1, 0x907(%rbp)
140a0a9a6: 4c 8d 05 93 c3 0b 02        	leaq	0x20bc393(%rip), %r8    # 0x142ac6d40
140a0a9ad: e8 66 eb ec 01              	callq	0x1428d9518 <.text+0x28d8518>
140a0a9b2: eb 4b                       	jmp	0x140a0a9ff <.text+0xa099ff>
140a0a9b4: c6 85 07 09 00 00 01        	movb	$0x1, 0x907(%rbp)
140a0a9bb: 4c 8d 05 ae c3 0b 02        	leaq	0x20bc3ae(%rip), %r8    # 0x142ac6d70
140a0a9c2: 48 8b 8d f8 08 00 00        	movq	0x8f8(%rbp), %rcx
140a0a9c9: e8 4a eb ec 01              	callq	0x1428d9518 <.text+0x28d8518>
140a0a9ce: eb 2f                       	jmp	0x140a0a9ff <.text+0xa099ff>
140a0a9d0: c6 85 07 09 00 00 01        	movb	$0x1, 0x907(%rbp)
140a0a9d7: 4c 8d 05 a2 c2 0b 02        	leaq	0x20bc2a2(%rip), %r8    # 0x142ac6c80
140a0a9de: 48 8b 8d f8 08 00 00        	movq	0x8f8(%rbp), %rcx
140a0a9e5: e8 2e eb ec 01              	callq	0x1428d9518 <.text+0x28d8518>
140a0a9ea: eb 13                       	jmp	0x140a0a9ff <.text+0xa099ff>
140a0a9ec: c6 85 07 09 00 00 01        	movb	$0x1, 0x907(%rbp)
140a0a9f3: 4c 8d 05 b6 c2 0b 02        	leaq	0x20bc2b6(%rip), %r8    # 0x142ac6cb0
140a0a9fa: e8 19 eb ec 01              	callq	0x1428d9518 <.text+0x28d8518>
140a0a9ff: 0f 0b                       	ud2
140a0aa01: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
140a0aa10: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a0aa15: 55                          	pushq	%rbp
140a0aa16: 41 57                       	pushq	%r15
140a0aa18: 41 56                       	pushq	%r14
140a0aa1a: 41 55                       	pushq	%r13
140a0aa1c: 41 54                       	pushq	%r12
140a0aa1e: 56                          	pushq	%rsi
140a0aa1f: 57                          	pushq	%rdi
140a0aa20: 53                          	pushq	%rbx
140a0aa21: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140a0aa28: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a0aa2f: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140a0aa35: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140a0aa3b: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140a0aa41: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140a0aa47: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140a0aa4d: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140a0aa56: 66 0f 7f bc 24 90 00 00 00  	movdqa	%xmm7, 0x90(%rsp)
140a0aa5f: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
140a0aa68: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
140a0aa6f: e8 bc 21 34 00              	callq	0x140d4cc30 <.text+0xd4bc30>
140a0aa74: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
140a0aa7c: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140a0aa84: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140a0aa8d: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140a0aa93: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140a0aa99: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140a0aa9f: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140a0aaa5: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140a0aaab: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140a0aab2: 5b                          	popq	%rbx
140a0aab3: 5f                          	popq	%rdi
140a0aab4: 5e                          	popq	%rsi
140a0aab5: 41 5c                       	popq	%r12
140a0aab7: 41 5d                       	popq	%r13
140a0aab9: 41 5e                       	popq	%r14
140a0aabb: 41 5f                       	popq	%r15
140a0aabd: 5d                          	popq	%rbp
140a0aabe: c3                          	retq
140a0aabf: 90                          	nop
140a0aac0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a0aac5: 55                          	pushq	%rbp
140a0aac6: 41 57                       	pushq	%r15
140a0aac8: 41 56                       	pushq	%r14
140a0aaca: 41 55                       	pushq	%r13
140a0aacc: 41 54                       	pushq	%r12
140a0aace: 56                          	pushq	%rsi
140a0aacf: 57                          	pushq	%rdi
140a0aad0: 53                          	pushq	%rbx
140a0aad1: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140a0aad8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a0aadf: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140a0aae5: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140a0aaeb: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140a0aaf1: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140a0aaf7: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140a0aafd: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140a0ab06: 66 0f 7f bc 24 90 00 00 00  	movdqa	%xmm7, 0x90(%rsp)
140a0ab0f: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
140a0ab18: 0f b6 9d 03 09 00 00        	movzbl	0x903(%rbp), %ebx
140a0ab1f: 48 8d 8d 90 07 00 00        	leaq	0x790(%rbp), %rcx
140a0ab26: e8 05 e0 e7 ff              	callq	0x140888b30 <.text+0x887b30>
140a0ab2b: 88 9d 05 09 00 00           	movb	%bl, 0x905(%rbp)
140a0ab31: 66 0f 6f b4 24 a0 00 00 00  	movdqa	0xa0(%rsp), %xmm6
140a0ab3a: 66 0f 6f bc 24 90 00 00 00  	movdqa	0x90(%rsp), %xmm7
140a0ab43: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140a0ab4c: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140a0ab52: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140a0ab58: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140a0ab5e: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140a0ab64: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140a0ab6a: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140a0ab71: 5b                          	popq	%rbx
140a0ab72: 5f                          	popq	%rdi
140a0ab73: 5e                          	popq	%rsi
140a0ab74: 41 5c                       	popq	%r12
140a0ab76: 41 5d                       	popq	%r13
140a0ab78: 41 5e                       	popq	%r14
140a0ab7a: 41 5f                       	popq	%r15
140a0ab7c: 5d                          	popq	%rbp
140a0ab7d: c3                          	retq
140a0ab7e: 66 90                       	nop
140a0ab80: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a0ab85: 55                          	pushq	%rbp
140a0ab86: 41 57                       	pushq	%r15
140a0ab88: 41 56                       	pushq	%r14
140a0ab8a: 41 55                       	pushq	%r13
140a0ab8c: 41 54                       	pushq	%r12
140a0ab8e: 56                          	pushq	%rsi
140a0ab8f: 57                          	pushq	%rdi
140a0ab90: 53                          	pushq	%rbx
140a0ab91: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140a0ab98: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a0ab9f: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140a0aba5: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140a0abab: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140a0abb1: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140a0abb7: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140a0abbd: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140a0abc6: 66 0f 7f bc 24 90 00 00 00  	movdqa	%xmm7, 0x90(%rsp)
140a0abcf: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
140a0abd8: 0f b6 9d 04 09 00 00        	movzbl	0x904(%rbp), %ebx
140a0abdf: 48 8b 85 d8 08 00 00        	movq	0x8d8(%rbp), %rax
140a0abe6: 48 85 c0                    	testq	%rax, %rax
140a0abe9: 74 19                       	je	0x140a0ac04 <.text+0xa09c04>
140a0abeb: 48 8b 8d e0 08 00 00        	movq	0x8e0(%rbp), %rcx
140a0abf2: 48 69 d0 e0 03 00 00        	imulq	$0x3e0, %rax, %rdx      # imm = 0x3E0
140a0abf9: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a0abff: e8 1c 90 c0 ff              	callq	0x140613c20 <.text+0x612c20>
140a0ac04: 80 e3 01                    	andb	$0x1, %bl
140a0ac07: 88 9d 03 09 00 00           	movb	%bl, 0x903(%rbp)
140a0ac0d: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
140a0ac15: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140a0ac1d: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140a0ac26: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140a0ac2c: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140a0ac32: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140a0ac38: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140a0ac3e: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140a0ac44: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140a0ac4b: 5b                          	popq	%rbx
140a0ac4c: 5f                          	popq	%rdi
140a0ac4d: 5e                          	popq	%rsi
140a0ac4e: 41 5c                       	popq	%r12
140a0ac50: 41 5d                       	popq	%r13
140a0ac52: 41 5e                       	popq	%r14
140a0ac54: 41 5f                       	popq	%r15
140a0ac56: 5d                          	popq	%rbp
140a0ac57: c3                          	retq
140a0ac58: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140a0ac60: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a0ac65: 55                          	pushq	%rbp
140a0ac66: 41 57                       	pushq	%r15
140a0ac68: 41 56                       	pushq	%r14
140a0ac6a: 41 55                       	pushq	%r13
140a0ac6c: 41 54                       	pushq	%r12
140a0ac6e: 56                          	pushq	%rsi
140a0ac6f: 57                          	pushq	%rdi
140a0ac70: 53                          	pushq	%rbx
140a0ac71: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140a0ac78: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a0ac7f: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140a0ac85: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140a0ac8b: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140a0ac91: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140a0ac97: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140a0ac9d: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140a0aca6: 66 0f 7f bc 24 90 00 00 00  	movdqa	%xmm7, 0x90(%rsp)
140a0acaf: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
140a0acb8: 0f b6 9d 07 09 00 00        	movzbl	0x907(%rbp), %ebx
140a0acbf: 48 8d 8d 88 08 00 00        	leaq	0x888(%rbp), %rcx
140a0acc6: e8 35 4f e8 ff              	callq	0x14088fc00 <.text+0x88ec00>
140a0accb: 88 9d 04 09 00 00           	movb	%bl, 0x904(%rbp)
140a0acd1: 66 0f 6f b4 24 a0 00 00 00  	movdqa	0xa0(%rsp), %xmm6
140a0acda: 66 0f 6f bc 24 90 00 00 00  	movdqa	0x90(%rsp), %xmm7
140a0ace3: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140a0acec: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140a0acf2: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140a0acf8: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140a0acfe: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140a0ad04: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140a0ad0a: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140a0ad11: 5b                          	popq	%rbx
140a0ad12: 5f                          	popq	%rdi
140a0ad13: 5e                          	popq	%rsi
140a0ad14: 41 5c                       	popq	%r12
140a0ad16: 41 5d                       	popq	%r13
140a0ad18: 41 5e                       	popq	%r14
140a0ad1a: 41 5f                       	popq	%r15
140a0ad1c: 5d                          	popq	%rbp
140a0ad1d: c3                          	retq
140a0ad1e: 66 90                       	nop
140a0ad20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a0ad25: 55                          	pushq	%rbp
140a0ad26: 41 57                       	pushq	%r15
140a0ad28: 41 56                       	pushq	%r14
140a0ad2a: 41 55                       	pushq	%r13
140a0ad2c: 41 54                       	pushq	%r12
140a0ad2e: 56                          	pushq	%rsi
140a0ad2f: 57                          	pushq	%rdi
140a0ad30: 53                          	pushq	%rbx
140a0ad31: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140a0ad38: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a0ad3f: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140a0ad45: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140a0ad4b: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140a0ad51: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140a0ad57: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140a0ad5d: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140a0ad66: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
140a0ad6e: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
140a0ad77: 0f b6 9d 02 09 00 00        	movzbl	0x902(%rbp), %ebx
140a0ad7e: f6 c3 01                    	testb	$0x1, %bl
140a0ad81: 74 5d                       	je	0x140a0ade0 <.text+0xa09de0>
140a0ad83: 48 8d 8d 90 07 00 00        	leaq	0x790(%rbp), %rcx
140a0ad8a: e8 e1 11 e8 ff              	callq	0x14088bf70 <.text+0x88af70>
140a0ad8f: 88 9d 06 09 00 00           	movb	%bl, 0x906(%rbp)
140a0ad95: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
140a0ad9d: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140a0ada5: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140a0adae: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140a0adb4: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140a0adba: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140a0adc0: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140a0adc6: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140a0adcc: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140a0add3: 5b                          	popq	%rbx
140a0add4: 5f                          	popq	%rdi
140a0add5: 5e                          	popq	%rsi
140a0add6: 41 5c                       	popq	%r12
140a0add8: 41 5d                       	popq	%r13
140a0adda: 41 5e                       	popq	%r14
140a0addc: 41 5f                       	popq	%r15
140a0adde: 5d                          	popq	%rbp
140a0addf: c3                          	retq
140a0ade0: 88 9d 06 09 00 00           	movb	%bl, 0x906(%rbp)
140a0ade6: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
140a0adee: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140a0adf6: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140a0adff: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140a0ae05: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140a0ae0b: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140a0ae11: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140a0ae17: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140a0ae1d: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140a0ae24: 5b                          	popq	%rbx
140a0ae25: 5f                          	popq	%rdi
140a0ae26: 5e                          	popq	%rsi
140a0ae27: 41 5c                       	popq	%r12
140a0ae29: 41 5d                       	popq	%r13
140a0ae2b: 41 5e                       	popq	%r14
140a0ae2d: 41 5f                       	popq	%r15
140a0ae2f: 5d                          	popq	%rbp
140a0ae30: c3                          	retq
140a0ae31: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
140a0ae40: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a0ae45: 55                          	pushq	%rbp
140a0ae46: 41 57                       	pushq	%r15
140a0ae48: 41 56                       	pushq	%r14
140a0ae4a: 41 55                       	pushq	%r13
140a0ae4c: 41 54                       	pushq	%r12
140a0ae4e: 56                          	pushq	%rsi
140a0ae4f: 57                          	pushq	%rdi
140a0ae50: 53                          	pushq	%rbx
140a0ae51: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140a0ae58: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a0ae5f: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140a0ae65: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140a0ae6b: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140a0ae71: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140a0ae77: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140a0ae7d: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140a0ae86: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
140a0ae8e: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
140a0ae97: f6 85 06 09 00 00 01        	testb	$0x1, 0x906(%rbp)
140a0ae9e: 75 4b                       	jne	0x140a0aeeb <.text+0xa09eeb>
140a0aea0: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
140a0aea8: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140a0aeb0: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140a0aeb9: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140a0aebf: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140a0aec5: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140a0aecb: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140a0aed1: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140a0aed7: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140a0aede: 5b                          	popq	%rbx
140a0aedf: 5f                          	popq	%rdi
140a0aee0: 5e                          	popq	%rsi
140a0aee1: 41 5c                       	popq	%r12
140a0aee3: 41 5d                       	popq	%r13
140a0aee5: 41 5e                       	popq	%r14
140a0aee7: 41 5f                       	popq	%r15
140a0aee9: 5d                          	popq	%rbp
140a0aeea: c3                          	retq
140a0aeeb: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
140a0aef2: e8 89 f7 73 ff              	callq	0x14014a680 <.text+0x149680>
140a0aef7: eb a7                       	jmp	0x140a0aea0 <.text+0xa09ea0>
140a0aef9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140a0af00: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a0af05: 55                          	pushq	%rbp
140a0af06: 41 57                       	pushq	%r15
140a0af08: 41 56                       	pushq	%r14
140a0af0a: 41 55                       	pushq	%r13
140a0af0c: 41 54                       	pushq	%r12
140a0af0e: 56                          	pushq	%rsi
140a0af0f: 57                          	pushq	%rdi
140a0af10: 53                          	pushq	%rbx
140a0af11: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140a0af18: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a0af1f: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140a0af25: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140a0af2b: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140a0af31: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140a0af37: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140a0af3d: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140a0af46: 66 0f 7f bc 24 90 00 00 00  	movdqa	%xmm7, 0x90(%rsp)
140a0af4f: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
140a0af58: 80 bd 05 09 00 00 00        	cmpb	$0x0, 0x905(%rbp)
140a0af5f: 75 4b                       	jne	0x140a0afac <.text+0xa09fac>
140a0af61: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
140a0af69: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140a0af71: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140a0af7a: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140a0af80: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140a0af86: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140a0af8c: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140a0af92: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140a0af98: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140a0af9f: 5b                          	popq	%rbx
140a0afa0: 5f                          	popq	%rdi
140a0afa1: 5e                          	popq	%rsi
140a0afa2: 41 5c                       	popq	%r12
140a0afa4: 41 5d                       	popq	%r13
140a0afa6: 41 5e                       	popq	%r14
140a0afa8: 41 5f                       	popq	%r15
140a0afaa: 5d                          	popq	%rbp
140a0afab: c3                          	retq
140a0afac: 48 8d 8d 10 08 00 00        	leaq	0x810(%rbp), %rcx
140a0afb3: e8 78 db e7 ff              	callq	0x140888b30 <.text+0x887b30>
140a0afb8: eb a7                       	jmp	0x140a0af61 <.text+0xa09f61>
140a0afba: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140a0afc0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a0afc5: 55                          	pushq	%rbp
140a0afc6: 41 57                       	pushq	%r15
140a0afc8: 41 56                       	pushq	%r14
140a0afca: 41 55                       	pushq	%r13
140a0afcc: 41 54                       	pushq	%r12
140a0afce: 56                          	pushq	%rsi
140a0afcf: 57                          	pushq	%rdi
140a0afd0: 53                          	pushq	%rbx
140a0afd1: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140a0afd8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a0afdf: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140a0afe5: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140a0afeb: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140a0aff1: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140a0aff7: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140a0affd: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140a0b006: 66 0f 7f bc 24 90 00 00 00  	movdqa	%xmm7, 0x90(%rsp)
140a0b00f: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
140a0b018: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
140a0b01f: e8 0c 1c 34 00              	callq	0x140d4cc30 <.text+0xd4bc30>
140a0b024: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
140a0b02c: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140a0b034: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140a0b03d: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140a0b043: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140a0b049: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140a0b04f: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140a0b055: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140a0b05b: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140a0b062: 5b                          	popq	%rbx
140a0b063: 5f                          	popq	%rdi
140a0b064: 5e                          	popq	%rsi
140a0b065: 41 5c                       	popq	%r12
140a0b067: 41 5d                       	popq	%r13
140a0b069: 41 5e                       	popq	%r14
140a0b06b: 41 5f                       	popq	%r15
140a0b06d: 5d                          	popq	%rbp
140a0b06e: c3                          	retq
140a0b06f: 90                          	nop
140a0b070: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a0b075: 55                          	pushq	%rbp
140a0b076: 41 57                       	pushq	%r15
140a0b078: 41 56                       	pushq	%r14
140a0b07a: 41 55                       	pushq	%r13
140a0b07c: 41 54                       	pushq	%r12
140a0b07e: 56                          	pushq	%rsi
140a0b07f: 57                          	pushq	%rdi
140a0b080: 53                          	pushq	%rbx
140a0b081: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140a0b088: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a0b08f: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140a0b095: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140a0b09b: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140a0b0a1: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140a0b0a7: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140a0b0ad: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140a0b0b6: 66 0f 7f bc 24 90 00 00 00  	movdqa	%xmm7, 0x90(%rsp)
140a0b0bf: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
140a0b0c8: c6 85 05 09 00 00 01        	movb	$0x1, 0x905(%rbp)
140a0b0cf: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
140a0b0d6: e8 55 1b 34 00              	callq	0x140d4cc30 <.text+0xd4bc30>
140a0b0db: c6 85 05 09 00 00 01        	movb	$0x1, 0x905(%rbp)
140a0b0e2: 66 0f 6f b4 24 a0 00 00 00  	movdqa	0xa0(%rsp), %xmm6
140a0b0eb: 66 0f 6f bc 24 90 00 00 00  	movdqa	0x90(%rsp), %xmm7
140a0b0f4: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140a0b0fd: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140a0b103: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140a0b109: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140a0b10f: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140a0b115: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140a0b11b: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140a0b122: 5b                          	popq	%rbx
140a0b123: 5f                          	popq	%rdi
140a0b124: 5e                          	popq	%rsi
140a0b125: 41 5c                       	popq	%r12
140a0b127: 41 5d                       	popq	%r13
140a0b129: 41 5e                       	popq	%r14
140a0b12b: 41 5f                       	popq	%r15
140a0b12d: 5d                          	popq	%rbp
140a0b12e: c3                          	retq
140a0b12f: 90                          	nop
140a0b130: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a0b135: 55                          	pushq	%rbp
140a0b136: 41 57                       	pushq	%r15
140a0b138: 41 56                       	pushq	%r14
140a0b13a: 41 55                       	pushq	%r13
140a0b13c: 41 54                       	pushq	%r12
140a0b13e: 56                          	pushq	%rsi
140a0b13f: 57                          	pushq	%rdi
140a0b140: 53                          	pushq	%rbx
140a0b141: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140a0b148: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a0b14f: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140a0b155: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140a0b15b: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140a0b161: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140a0b167: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140a0b16d: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140a0b176: 66 0f 7f bc 24 90 00 00 00  	movdqa	%xmm7, 0x90(%rsp)
140a0b17f: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
140a0b188: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
140a0b18f: e8 9c 1a 34 00              	callq	0x140d4cc30 <.text+0xd4bc30>
140a0b194: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
140a0b19c: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140a0b1a4: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140a0b1ad: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140a0b1b3: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140a0b1b9: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140a0b1bf: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140a0b1c5: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140a0b1cb: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140a0b1d2: 5b                          	popq	%rbx
140a0b1d3: 5f                          	popq	%rdi
140a0b1d4: 5e                          	popq	%rsi
140a0b1d5: 41 5c                       	popq	%r12
140a0b1d7: 41 5d                       	popq	%r13
140a0b1d9: 41 5e                       	popq	%r14
140a0b1db: 41 5f                       	popq	%r15
140a0b1dd: 5d                          	popq	%rbp
140a0b1de: c3                          	retq
140a0b1df: 90                          	nop
140a0b1e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a0b1e5: 55                          	pushq	%rbp
140a0b1e6: 41 57                       	pushq	%r15
140a0b1e8: 41 56                       	pushq	%r14
140a0b1ea: 41 55                       	pushq	%r13
140a0b1ec: 41 54                       	pushq	%r12
140a0b1ee: 56                          	pushq	%rsi
140a0b1ef: 57                          	pushq	%rdi
140a0b1f0: 53                          	pushq	%rbx
140a0b1f1: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140a0b1f8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a0b1ff: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140a0b205: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140a0b20b: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140a0b211: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140a0b217: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140a0b21d: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140a0b226: 66 0f 7f bc 24 90 00 00 00  	movdqa	%xmm7, 0x90(%rsp)
140a0b22f: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
140a0b238: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
140a0b23f: e8 ec 19 34 00              	callq	0x140d4cc30 <.text+0xd4bc30>
140a0b244: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
140a0b24c: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140a0b254: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140a0b25d: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140a0b263: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140a0b269: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140a0b26f: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140a0b275: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140a0b27b: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140a0b282: 5b                          	popq	%rbx
140a0b283: 5f                          	popq	%rdi
140a0b284: 5e                          	popq	%rsi
140a0b285: 41 5c                       	popq	%r12
140a0b287: 41 5d                       	popq	%r13
140a0b289: 41 5e                       	popq	%r14
140a0b28b: 41 5f                       	popq	%r15
140a0b28d: 5d                          	popq	%rbp
140a0b28e: c3                          	retq
140a0b28f: 90                          	nop
140a0b290: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a0b295: 55                          	pushq	%rbp
140a0b296: 41 57                       	pushq	%r15
140a0b298: 41 56                       	pushq	%r14
140a0b29a: 41 55                       	pushq	%r13
140a0b29c: 41 54                       	pushq	%r12
140a0b29e: 56                          	pushq	%rsi
140a0b29f: 57                          	pushq	%rdi
140a0b2a0: 53                          	pushq	%rbx
140a0b2a1: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140a0b2a8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a0b2af: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140a0b2b5: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140a0b2bb: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140a0b2c1: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140a0b2c7: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140a0b2cd: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140a0b2d6: 66 0f 7f bc 24 90 00 00 00  	movdqa	%xmm7, 0x90(%rsp)
140a0b2df: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
140a0b2e8: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
140a0b2ef: e8 3c 19 34 00              	callq	0x140d4cc30 <.text+0xd4bc30>
140a0b2f4: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
140a0b2fc: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140a0b304: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140a0b30d: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140a0b313: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140a0b319: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140a0b31f: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140a0b325: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140a0b32b: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140a0b332: 5b                          	popq	%rbx
140a0b333: 5f                          	popq	%rdi
140a0b334: 5e                          	popq	%rsi
140a0b335: 41 5c                       	popq	%r12
140a0b337: 41 5d                       	popq	%r13
140a0b339: 41 5e                       	popq	%r14
140a0b33b: 41 5f                       	popq	%r15
140a0b33d: 5d                          	popq	%rbp
140a0b33e: c3                          	retq
