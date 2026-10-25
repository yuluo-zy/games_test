
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14082bd80: 41 57                       	pushq	%r15
14082bd82: 41 56                       	pushq	%r14
14082bd84: 41 54                       	pushq	%r12
14082bd86: 56                          	pushq	%rsi
14082bd87: 57                          	pushq	%rdi
14082bd88: 53                          	pushq	%rbx
14082bd89: 48 83 ec 38                 	subq	$0x38, %rsp
14082bd8d: 48 8b 01                    	movq	(%rcx), %rax
14082bd90: 48 8b 51 08                 	movq	0x8(%rcx), %rdx
14082bd94: 8b 49 14                    	movl	0x14(%rcx), %ecx
14082bd97: 4c 8b 80 f8 00 00 00        	movq	0xf8(%rax), %r8
14082bd9e: 4c 8b 88 00 01 00 00        	movq	0x100(%rax), %r9
14082bda5: 4f 8d 0c 88                 	leaq	(%r8,%r9,4), %r9
14082bda9: 80 b8 08 01 00 00 00        	cmpb	$0x0, 0x108(%rax)
14082bdb0: 74 0f                       	je	0x14082bdc1 <.text+0x82adc1>
14082bdb2: 31 f6                       	xorl	%esi, %esi
14082bdb4: 45 31 db                    	xorl	%r11d, %r11d
14082bdb7: 45 31 d2                    	xorl	%r10d, %r10d
14082bdba: 31 db                       	xorl	%ebx, %ebx
14082bdbc: e9 95 00 00 00              	jmp	0x14082be56 <.text+0x82ae56>
14082bdc1: bf 08 00 00 00              	movl	$0x8, %edi
14082bdc6: 31 f6                       	xorl	%esi, %esi
14082bdc8: 45 31 db                    	xorl	%r11d, %r11d
14082bdcb: 45 31 d2                    	xorl	%r10d, %r10d
14082bdce: 45 31 f6                    	xorl	%r14d, %r14d
14082bdd1: e9 87 01 00 00              	jmp	0x14082bf5d <.text+0x82af5d>
14082bdd6: 49 8b 7b 18                 	movq	0x18(%r11), %rdi
14082bdda: 45 31 db                    	xorl	%r11d, %r11d
14082bddd: 48 8b 98 18 01 00 00        	movq	0x118(%rax), %rbx
14082bde4: 49 83 c0 04                 	addq	$0x4, %r8
14082bde8: 48 8b 34 de                 	movq	(%rsi,%rbx,8), %rsi
14082bdec: 48 f7 d6                    	notq	%rsi
14082bdef: 48 8d 1c 76                 	leaq	(%rsi,%rsi,2), %rbx
14082bdf3: 48 c1 e3 04                 	shlq	$0x4, %rbx
14082bdf7: 48 8b 74 1f 10              	movq	0x10(%rdi,%rbx), %rsi
14082bdfc: 48 8b 7c 1f 28              	movq	0x28(%rdi,%rbx), %rdi
14082be01: 31 db                       	xorl	%ebx, %ebx
14082be03: 41 89 de                    	movl	%ebx, %r14d
14082be06: 4d 89 f7                    	movq	%r14, %r15
14082be09: 49 c1 e7 04                 	shlq	$0x4, %r15
14082be0d: 4f 8d 3c 7f                 	leaq	(%r15,%r15,2), %r15
14082be11: 48 ff c3                    	incq	%rbx
14082be14: 43 0f 28 04 3b              	movaps	(%r11,%r15), %xmm0
14082be19: 43 0f 28 4c 3b 10           	movaps	0x10(%r11,%r15), %xmm1
14082be1f: 43 0f 28 54 3b 20           	movaps	0x20(%r11,%r15), %xmm2
14082be25: 0f 29 54 24 20              	movaps	%xmm2, 0x20(%rsp)
14082be2a: 0f 29 4c 24 10              	movaps	%xmm1, 0x10(%rsp)
14082be2f: 0f 29 04 24                 	movaps	%xmm0, (%rsp)
14082be33: 42 89 0c b7                 	movl	%ecx, (%rdi,%r14,4)
14082be37: 0f 28 04 24                 	movaps	(%rsp), %xmm0
14082be3b: 0f 28 4c 24 10              	movaps	0x10(%rsp), %xmm1
14082be40: 0f 28 54 24 20              	movaps	0x20(%rsp), %xmm2
14082be45: 42 0f 29 54 3e 20           	movaps	%xmm2, 0x20(%rsi,%r15)
14082be4b: 42 0f 29 4c 3e 10           	movaps	%xmm1, 0x10(%rsi,%r15)
14082be51: 42 0f 29 04 3e              	movaps	%xmm0, (%rsi,%r15)
14082be56: 4c 39 d3                    	cmpq	%r10, %rbx
14082be59: 75 a8                       	jne	0x14082be03 <.text+0x82ae03>
14082be5b: 4d 39 c8                    	cmpq	%r9, %r8
14082be5e: 0f 84 82 01 00 00           	je	0x14082bfe6 <.text+0x82afe6>
14082be64: 4c 8b 9a a8 01 00 00        	movq	0x1a8(%rdx), %r11
14082be6b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
14082be70: 45 8b 10                    	movl	(%r8), %r10d
14082be73: 4b 8d 34 d2                 	leaq	(%r10,%r10,8), %rsi
14082be77: 4d 8b 54 f3 10              	movq	0x10(%r11,%rsi,8), %r10
14082be7c: 4d 85 d2                    	testq	%r10, %r10
14082be7f: 75 0f                       	jne	0x14082be90 <.text+0x82ae90>
14082be81: 49 83 c0 04                 	addq	$0x4, %r8
14082be85: 4d 39 c8                    	cmpq	%r9, %r8
14082be88: 75 e6                       	jne	0x14082be70 <.text+0x82ae70>
14082be8a: e9 57 01 00 00              	jmp	0x14082bfe6 <.text+0x82afe6>
14082be8f: 90                          	nop
14082be90: 4d 8d 1c f3                 	leaq	(%r11,%rsi,8), %r11
14082be94: 48 8b b8 10 01 00 00        	movq	0x110(%rax), %rdi
14082be9b: 49 8b 73 38                 	movq	0x38(%r11), %rsi
14082be9f: 49 39 7b 40                 	cmpq	%rdi, 0x40(%r11)
14082bea3: 0f 86 2d ff ff ff           	jbe	0x14082bdd6 <.text+0x82add6>
14082bea9: 48 8b 1c fe                 	movq	(%rsi,%rdi,8), %rbx
14082bead: 49 8b 7b 18                 	movq	0x18(%r11), %rdi
14082beb1: 48 85 db                    	testq	%rbx, %rbx
14082beb4: 0f 84 20 ff ff ff           	je	0x14082bdda <.text+0x82adda>
14082beba: 48 f7 d3                    	notq	%rbx
14082bebd: 4c 8d 1c 5b                 	leaq	(%rbx,%rbx,2), %r11
14082bec1: 49 c1 e3 04                 	shlq	$0x4, %r11
14082bec5: 4e 8b 5c 1f 10              	movq	0x10(%rdi,%r11), %r11
14082beca: e9 0e ff ff ff              	jmp	0x14082bddd <.text+0x82addd>
14082becf: 49 8b 5b 18                 	movq	0x18(%r11), %rbx
14082bed3: 45 31 db                    	xorl	%r11d, %r11d
14082bed6: 4c 8b b0 18 01 00 00        	movq	0x118(%rax), %r14
14082bedd: 49 83 c0 04                 	addq	$0x4, %r8
14082bee1: 4a 8b 34 f6                 	movq	(%rsi,%r14,8), %rsi
14082bee5: 48 f7 d6                    	notq	%rsi
14082bee8: 4c 8d 34 76                 	leaq	(%rsi,%rsi,2), %r14
14082beec: 49 c1 e6 04                 	shlq	$0x4, %r14
14082bef0: 4a 8b 74 33 10              	movq	0x10(%rbx,%r14), %rsi
14082bef5: 4a 8b 5c 33 28              	movq	0x28(%rbx,%r14), %rbx
14082befa: 48 8b 7f 50                 	movq	0x50(%rdi), %rdi
14082befe: 45 31 f6                    	xorl	%r14d, %r14d
14082bf01: 4d 89 f7                    	movq	%r14, %r15
14082bf04: 49 c1 e7 04                 	shlq	$0x4, %r15
14082bf08: 46 8b 7c 3f 08              	movl	0x8(%rdi,%r15), %r15d
14082bf0d: 4d 89 fc                    	movq	%r15, %r12
14082bf10: 49 c1 e4 04                 	shlq	$0x4, %r12
14082bf14: 4f 8d 24 64                 	leaq	(%r12,%r12,2), %r12
14082bf18: 49 ff c6                    	incq	%r14
14082bf1b: 43 0f 28 04 23              	movaps	(%r11,%r12), %xmm0
14082bf20: 43 0f 28 4c 23 10           	movaps	0x10(%r11,%r12), %xmm1
14082bf26: 43 0f 28 54 23 20           	movaps	0x20(%r11,%r12), %xmm2
14082bf2c: 0f 29 54 24 20              	movaps	%xmm2, 0x20(%rsp)
14082bf31: 0f 29 4c 24 10              	movaps	%xmm1, 0x10(%rsp)
14082bf36: 0f 29 04 24                 	movaps	%xmm0, (%rsp)
14082bf3a: 42 89 0c bb                 	movl	%ecx, (%rbx,%r15,4)
14082bf3e: 0f 28 04 24                 	movaps	(%rsp), %xmm0
14082bf42: 0f 28 4c 24 10              	movaps	0x10(%rsp), %xmm1
14082bf47: 0f 28 54 24 20              	movaps	0x20(%rsp), %xmm2
14082bf4c: 42 0f 29 54 26 20           	movaps	%xmm2, 0x20(%rsi,%r12)
14082bf52: 42 0f 29 4c 26 10           	movaps	%xmm1, 0x10(%rsi,%r12)
14082bf58: 42 0f 29 04 26              	movaps	%xmm0, (%rsi,%r12)
14082bf5d: 4d 39 d6                    	cmpq	%r10, %r14
14082bf60: 75 9f                       	jne	0x14082bf01 <.text+0x82af01>
14082bf62: 4d 39 c8                    	cmpq	%r9, %r8
14082bf65: 74 7f                       	je	0x14082bfe6 <.text+0x82afe6>
14082bf67: 48 8b ba 00 01 00 00        	movq	0x100(%rdx), %rdi
14082bf6e: 66 90                       	nop
14082bf70: 45 8b 10                    	movl	(%r8), %r10d
14082bf73: 4f 8d 1c 92                 	leaq	(%r10,%r10,4), %r11
14082bf77: 49 c1 e3 05                 	shlq	$0x5, %r11
14082bf7b: 4e 8b 54 1f 58              	movq	0x58(%rdi,%r11), %r10
14082bf80: 4d 85 d2                    	testq	%r10, %r10
14082bf83: 75 0b                       	jne	0x14082bf90 <.text+0x82af90>
14082bf85: 49 83 c0 04                 	addq	$0x4, %r8
14082bf89: 4d 39 c8                    	cmpq	%r9, %r8
14082bf8c: 75 e2                       	jne	0x14082bf70 <.text+0x82af70>
14082bf8e: eb 56                       	jmp	0x14082bfe6 <.text+0x82afe6>
14082bf90: 4c 01 df                    	addq	%r11, %rdi
14082bf93: 44 8b 9f 94 00 00 00        	movl	0x94(%rdi), %r11d
14082bf9a: 4c 8b b2 a8 01 00 00        	movq	0x1a8(%rdx), %r14
14082bfa1: 4f 8d 3c db                 	leaq	(%r11,%r11,8), %r15
14082bfa5: 4f 8d 1c fe                 	leaq	(%r14,%r15,8), %r11
14082bfa9: 48 8b 98 10 01 00 00        	movq	0x110(%rax), %rbx
14082bfb0: 4b 8b 74 fe 38              	movq	0x38(%r14,%r15,8), %rsi
14082bfb5: 4b 39 5c fe 40              	cmpq	%rbx, 0x40(%r14,%r15,8)
14082bfba: 0f 86 0f ff ff ff           	jbe	0x14082becf <.text+0x82aecf>
14082bfc0: 4c 8b 34 de                 	movq	(%rsi,%rbx,8), %r14
14082bfc4: 49 8b 5b 18                 	movq	0x18(%r11), %rbx
14082bfc8: 4d 85 f6                    	testq	%r14, %r14
14082bfcb: 0f 84 02 ff ff ff           	je	0x14082bed3 <.text+0x82aed3>
14082bfd1: 49 f7 d6                    	notq	%r14
14082bfd4: 4f 8d 1c 76                 	leaq	(%r14,%r14,2), %r11
14082bfd8: 49 c1 e3 04                 	shlq	$0x4, %r11
14082bfdc: 4e 8b 5c 1b 10              	movq	0x10(%rbx,%r11), %r11
14082bfe1: e9 f0 fe ff ff              	jmp	0x14082bed6 <.text+0x82aed6>
14082bfe6: 48 83 c4 38                 	addq	$0x38, %rsp
14082bfea: 5b                          	popq	%rbx
14082bfeb: 5f                          	popq	%rdi
14082bfec: 5e                          	popq	%rsi
14082bfed: 41 5c                       	popq	%r12
14082bfef: 41 5e                       	popq	%r14
14082bff1: 41 5f                       	popq	%r15
14082bff3: c3                          	retq
