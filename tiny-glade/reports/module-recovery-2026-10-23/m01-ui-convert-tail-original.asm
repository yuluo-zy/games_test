
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14099dc60: 0a 00                       	orb	(%rax), %al
14099dc62: 00 83 7b 28 02 0f           	addb	%al, 0xf02287b(%rbx)
14099dc68: 84 cd                       	testb	%cl, %ch
14099dc6a: 0a 00                       	orb	(%rax), %al
14099dc6c: 00 48 8d                    	addb	%cl, -0x73(%rax)
14099dc6f: 04 76                       	addb	$0x76, %al
14099dc71: 48 89 85 88 01 00 00        	movq	%rax, 0x188(%rbp)
14099dc78: 48 c1 e0 05                 	shlq	$0x5, %rax
14099dc7c: 48 01 c7                    	addq	%rax, %rdi
14099dc7f: 48 89 bd 90 02 00 00        	movq	%rdi, 0x290(%rbp)
14099dc86: 48 8b 07                    	movq	(%rdi), %rax
14099dc89: 48 8d 48 fd                 	leaq	-0x3(%rax), %rcx
14099dc8d: 4c 8d 40 fe                 	leaq	-0x2(%rax), %r8
14099dc91: 45 31 c9                    	xorl	%r9d, %r9d
14099dc94: 48 83 f9 03                 	cmpq	$0x3, %rcx
14099dc98: 4d 0f 42 c8                 	cmovbq	%r8, %r9
14099dc9c: 48 8d 0d 39 e0 11 02        	leaq	0x211e039(%rip), %rcx   # 0x142abbcdc
14099dca3: 4e 63 04 89                 	movslq	(%rcx,%r9,4), %r8
14099dca7: 49 01 c8                    	addq	%rcx, %r8
14099dcaa: 48 89 95 80 02 00 00        	movq	%rdx, 0x280(%rbp)
14099dcb1: 41 ff e0                    	jmpq	*%r8
14099dcb4: 48 8b 8d 78 03 00 00        	movq	0x378(%rbp), %rcx
14099dcbb: 48 8b 09                    	movq	(%rcx), %rcx
14099dcbe: 48 8b 95 90 02 00 00        	movq	0x290(%rbp), %rdx
14099dcc5: 4c 8b 42 08                 	movq	0x8(%rdx), %r8
14099dcc9: 44 8b 4a 10                 	movl	0x10(%rdx), %r9d
14099dccd: 48 89 c2                    	movq	%rax, %rdx
14099dcd0: e8 1b e9 07 00              	callq	0x140a1c5f0 <.text+0xa1b5f0>
14099dcd5: 48 85 c0                    	testq	%rax, %rax
14099dcd8: 0f 84 eb 03 00 00           	je	0x14099e0c9 <.text+0x99d0c9>
14099dcde: 49 89 c6                    	movq	%rax, %r14
14099dce1: 48 8b 85 80 03 00 00        	movq	0x380(%rbp), %rax
14099dce8: 48 8b 10                    	movq	(%rax), %rdx
14099dceb: 48 8b b5 90 02 00 00        	movq	0x290(%rbp), %rsi
14099dcf2: 44 8b 46 10                 	movl	0x10(%rsi), %r8d
14099dcf6: 4c 8d 0d 4b de 11 02        	leaq	0x211de4b(%rip), %r9    # 0x142abbb48
14099dcfd: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
14099dd04: e8 17 cd 07 00              	callq	0x140a1aa20 <.text+0xa19a20>
14099dd09: 48 8b 85 88 03 00 00        	movq	0x388(%rbp), %rax
14099dd10: 48 8b 08                    	movq	(%rax), %rcx
14099dd13: 48 8b 78 20                 	movq	0x20(%rax), %rdi
14099dd17: 8b 56 10                    	movl	0x10(%rsi), %edx
14099dd1a: e8 41 7e 18 00              	callq	0x140b25b60 <.text+0xb24b60>
14099dd1f: 41 0f b6 56 44              	movzbl	0x44(%r14), %edx
14099dd24: 48 89 f9                    	movq	%rdi, %rcx
14099dd27: 41 89 c0                    	movl	%eax, %r8d
14099dd2a: e8 61 89 18 00              	callq	0x140b26690 <.text+0xb25690>
14099dd2f: 49 89 c7                    	movq	%rax, %r15
14099dd32: 48 8d bd a0 01 00 00        	leaq	0x1a0(%rbp), %rdi
14099dd39: 48 89 f9                    	movq	%rdi, %rcx
14099dd3c: 48 8d 95 f0 00 00 00        	leaq	0xf0(%rbp), %rdx
14099dd43: e8 68 68 e0 ff              	callq	0x1407a45b0 <.text+0x7a35b0>
14099dd48: 48 8b 85 98 01 00 00        	movq	0x198(%rbp), %rax
14099dd4f: 4c 8b 00                    	movq	(%rax), %r8
14099dd52: 44 8b 4e 10                 	movl	0x10(%rsi), %r9d
14099dd56: 41 0f b6 46 44              	movzbl	0x44(%r14), %eax
14099dd5b: 41 80 7e 22 02              	cmpb	$0x2, 0x22(%r14)
14099dd60: 0f 92 44 24 28              	setb	0x28(%rsp)
14099dd65: 88 44 24 20                 	movb	%al, 0x20(%rsp)
14099dd69: c6 44 24 30 01              	movb	$0x1, 0x30(%rsp)
14099dd6e: 4c 89 f9                    	movq	%r15, %rcx
14099dd71: 48 89 fa                    	movq	%rdi, %rdx
14099dd74: e8 87 5d 19 00              	callq	0x140b33b00 <.text+0xb32b00>
14099dd79: e9 4b 03 00 00              	jmp	0x14099e0c9 <.text+0x99d0c9>
14099dd7e: 48 8d 0d eb cd 29 02        	leaq	0x229cdeb(%rip), %rcx   # 0x142c3ab70
14099dd85: e8 c6 b4 f3 01              	callq	0x1428d9250 <.text+0x28d8250>
14099dd8a: 48 8b 85 98 03 00 00        	movq	0x398(%rbp), %rax
14099dd91: 48 8b 08                    	movq	(%rax), %rcx
14099dd94: 4c 8b 40 08                 	movq	0x8(%rax), %r8
14099dd98: 4c 8b b1 f8 00 00 00        	movq	0xf8(%rcx), %r14
14099dd9f: 48 8b 81 00 01 00 00        	movq	0x100(%rcx), %rax
14099dda6: 49 8d 3c 86                 	leaq	(%r14,%rax,4), %rdi
14099ddaa: 48 89 8d 90 01 00 00        	movq	%rcx, 0x190(%rbp)
14099ddb1: 44 0f b6 89 08 01 00 00     	movzbl	0x108(%rcx), %r9d
14099ddb9: 48 8b 85 98 01 00 00        	movq	0x198(%rbp), %rax
14099ddc0: 48 8b 00                    	movq	(%rax), %rax
14099ddc3: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
14099ddca: 41 ba 08 00 00 00           	movl	$0x8, %r10d
14099ddd0: 45 31 db                    	xorl	%r11d, %r11d
14099ddd3: 31 f6                       	xorl	%esi, %esi
14099ddd5: 45 31 ff                    	xorl	%r15d, %r15d
14099ddd8: 4c 89 85 88 02 00 00        	movq	%r8, 0x288(%rbp)
14099dddf: 45 84 c9                    	testb	%r9b, %r9b
14099dde2: 74 74                       	je	0x14099de58 <.text+0x99ce58>
14099dde4: 49 39 f7                    	cmpq	%rsi, %r15
14099dde7: 0f 85 fa 00 00 00           	jne	0x14099dee7 <.text+0x99cee7>
14099dded: 49 39 fe                    	cmpq	%rdi, %r14
14099ddf0: 0f 84 d3 02 00 00           	je	0x14099e0c9 <.text+0x99d0c9>
14099ddf6: 41 8b 0e                    	movl	(%r14), %ecx
14099ddf9: 49 83 c6 04                 	addq	$0x4, %r14
14099ddfd: 49 8b 80 a8 01 00 00        	movq	0x1a8(%r8), %rax
14099de04: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
14099de08: 48 8b 74 c8 10              	movq	0x10(%rax,%rcx,8), %rsi
14099de0d: 48 85 f6                    	testq	%rsi, %rsi
14099de10: 74 db                       	je	0x14099dded <.text+0x99cded>
14099de12: 48 8d 04 c8                 	leaq	(%rax,%rcx,8), %rax
14099de16: 48 8b 8d 90 01 00 00        	movq	0x190(%rbp), %rcx
14099de1d: 48 8b 89 10 01 00 00        	movq	0x110(%rcx), %rcx
14099de24: 48 39 48 40                 	cmpq	%rcx, 0x40(%rax)
14099de28: 0f 86 ac 00 00 00           	jbe	0x14099deda <.text+0x99ceda>
14099de2e: 48 8b 50 38                 	movq	0x38(%rax), %rdx
14099de32: 48 8b 0c ca                 	movq	(%rdx,%rcx,8), %rcx
14099de36: 48 85 c9                    	testq	%rcx, %rcx
14099de39: 0f 84 9b 00 00 00           	je	0x14099deda <.text+0x99ceda>
14099de3f: 48 8b 40 18                 	movq	0x18(%rax), %rax
14099de43: 48 f7 d1                    	notq	%rcx
14099de46: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
14099de4a: 48 c1 e1 04                 	shlq	$0x4, %rcx
14099de4e: 4c 8b 5c 08 10              	movq	0x10(%rax,%rcx), %r11
14099de53: e9 85 00 00 00              	jmp	0x14099dedd <.text+0x99cedd>
14099de58: 49 39 f7                    	cmpq	%rsi, %r15
14099de5b: 0f 85 9c 00 00 00           	jne	0x14099defd <.text+0x99cefd>
14099de61: 49 39 fe                    	cmpq	%rdi, %r14
14099de64: 0f 84 5f 02 00 00           	je	0x14099e0c9 <.text+0x99d0c9>
14099de6a: 41 8b 0e                    	movl	(%r14), %ecx
14099de6d: 49 83 c6 04                 	addq	$0x4, %r14
14099de71: 49 8b 80 00 01 00 00        	movq	0x100(%r8), %rax
14099de78: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
14099de7c: 48 c1 e1 05                 	shlq	$0x5, %rcx
14099de80: 48 8b 74 08 58              	movq	0x58(%rax,%rcx), %rsi
14099de85: 48 85 f6                    	testq	%rsi, %rsi
14099de88: 74 d7                       	je	0x14099de61 <.text+0x99ce61>
14099de8a: 48 01 c8                    	addq	%rcx, %rax
14099de8d: 8b 88 94 00 00 00           	movl	0x94(%rax), %ecx
14099de93: 49 8b 90 a8 01 00 00        	movq	0x1a8(%r8), %rdx
14099de9a: 4c 8d 04 c9                 	leaq	(%rcx,%rcx,8), %r8
14099de9e: 48 8b 8d 90 01 00 00        	movq	0x190(%rbp), %rcx
14099dea5: 48 8b 89 10 01 00 00        	movq	0x110(%rcx), %rcx
14099deac: 4a 39 4c c2 40              	cmpq	%rcx, 0x40(%rdx,%r8,8)
14099deb1: 76 39                       	jbe	0x14099deec <.text+0x99ceec>
14099deb3: 4a 8d 14 c2                 	leaq	(%rdx,%r8,8), %rdx
14099deb7: 4c 8b 42 38                 	movq	0x38(%rdx), %r8
14099debb: 49 8b 0c c8                 	movq	(%r8,%rcx,8), %rcx
14099debf: 48 85 c9                    	testq	%rcx, %rcx
14099dec2: 74 28                       	je	0x14099deec <.text+0x99ceec>
14099dec4: 48 8b 52 18                 	movq	0x18(%rdx), %rdx
14099dec8: 48 f7 d1                    	notq	%rcx
14099decb: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
14099decf: 48 c1 e1 04                 	shlq	$0x4, %rcx
14099ded3: 4c 8b 5c 0a 10              	movq	0x10(%rdx,%rcx), %r11
14099ded8: eb 15                       	jmp	0x14099deef <.text+0x99ceef>
14099deda: 45 31 db                    	xorl	%r11d, %r11d
14099dedd: 45 31 ff                    	xorl	%r15d, %r15d
14099dee0: 4c 8b 85 88 02 00 00        	movq	0x288(%rbp), %r8
14099dee7: 44 89 f8                    	movl	%r15d, %eax
14099deea: eb 1d                       	jmp	0x14099df09 <.text+0x99cf09>
14099deec: 45 31 db                    	xorl	%r11d, %r11d
14099deef: 4c 8b 50 50                 	movq	0x50(%rax), %r10
14099def3: 45 31 ff                    	xorl	%r15d, %r15d
14099def6: 4c 8b 85 88 02 00 00        	movq	0x288(%rbp), %r8
14099defd: 4c 89 f8                    	movq	%r15, %rax
14099df00: 48 c1 e0 04                 	shlq	$0x4, %rax
14099df04: 41 8b 44 02 08              	movl	0x8(%r10,%rax), %eax
14099df09: 48 6b d0 38                 	imulq	$0x38, %rax, %rdx
14099df0d: 4c 01 da                    	addq	%r11, %rdx
14099df10: 49 ff c7                    	incq	%r15
14099df13: 48 8b 42 18                 	movq	0x18(%rdx), %rax
14099df17: 48 8b 8d 90 02 00 00        	movq	0x290(%rbp), %rcx
14099df1e: 48 3b 41 08                 	cmpq	0x8(%rcx), %rax
14099df22: 0f 85 b7 fe ff ff           	jne	0x14099dddf <.text+0x99cddf>
14099df28: 8b 42 30                    	movl	0x30(%rdx), %eax
14099df2b: 48 8b 8d 90 02 00 00        	movq	0x290(%rbp), %rcx
14099df32: 3b 41 10                    	cmpl	0x10(%rcx), %eax
14099df35: 0f 85 a4 fe ff ff           	jne	0x14099dddf <.text+0x99cddf>
14099df3b: 4c 89 bd e0 00 00 00        	movq	%r15, 0xe0(%rbp)
14099df42: 4c 89 9d d8 00 00 00        	movq	%r11, 0xd8(%rbp)
14099df49: 4c 89 95 d0 00 00 00        	movq	%r10, 0xd0(%rbp)
14099df50: 44 88 8d c8 00 00 00        	movb	%r9b, 0xc8(%rbp)
14099df57: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
14099df5e: 4c 8d 05 fb db 11 02        	leaq	0x211dbfb(%rip), %r8    # 0x142abbb60
14099df65: e8 86 9f 95 00              	callq	0x1412f7ef0 <.text+0x12f6ef0>
14099df6a: 4c 8b bd 00 01 00 00        	movq	0x100(%rbp), %r15
14099df71: 4d 85 ff                    	testq	%r15, %r15
14099df74: 0f 84 4e 08 00 00           	je	0x14099e7c8 <.text+0x99d7c8>
14099df7a: 48 8b 85 f8 00 00 00        	movq	0xf8(%rbp), %rax
14099df81: 48 8b 08                    	movq	(%rax), %rcx
14099df84: 48 89 8d a0 01 00 00        	movq	%rcx, 0x1a0(%rbp)
14099df8b: 8b 48 08                    	movl	0x8(%rax), %ecx
14099df8e: 89 8d a8 01 00 00           	movl	%ecx, 0x1a8(%rbp)
14099df94: 4c 3b bd f0 00 00 00        	cmpq	0xf0(%rbp), %r15
14099df9b: 75 21                       	jne	0x14099dfbe <.text+0x99cfbe>
14099df9d: c6 85 b7 02 00 00 01        	movb	$0x1, 0x2b7(%rbp)
14099dfa4: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
14099dfab: 48 8d 15 de db 11 02        	leaq	0x211dbde(%rip), %rdx   # 0x142abbb90
14099dfb2: e8 c9 11 20 00              	callq	0x140b9f180 <.text+0xb9e180>
14099dfb7: 48 8b 85 f8 00 00 00        	movq	0xf8(%rbp), %rax
14099dfbe: 4b 8d 0c 7f                 	leaq	(%r15,%r15,2), %rcx
14099dfc2: 8b 95 a8 01 00 00           	movl	0x1a8(%rbp), %edx
14099dfc8: 89 54 88 08                 	movl	%edx, 0x8(%rax,%rcx,4)
14099dfcc: 48 8b 95 a0 01 00 00        	movq	0x1a0(%rbp), %rdx
14099dfd3: 48 89 14 88                 	movq	%rdx, (%rax,%rcx,4)
14099dfd7: 49 ff c7                    	incq	%r15
14099dfda: 4c 89 bd 00 01 00 00        	movq	%r15, 0x100(%rbp)
14099dfe1: 4c 89 bd b0 01 00 00        	movq	%r15, 0x1b0(%rbp)
14099dfe8: 0f 10 85 f0 00 00 00        	movups	0xf0(%rbp), %xmm0
14099dfef: 0f 29 85 a0 01 00 00        	movaps	%xmm0, 0x1a0(%rbp)
14099dff6: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14099e000: 48 89 85 b8 01 00 00        	movq	%rax, 0x1b8(%rbp)
14099e007: c7 85 d0 01 00 00 00 00 00 00       	movl	$0x0, 0x1d0(%rbp)
14099e011: c7 85 e0 01 00 00 00 00 00 40       	movl	$0x40000000, 0x1e0(%rbp) # imm = 0x40000000
14099e01b: 66 c7 85 e4 01 00 00 01 01  	movw	$0x101, 0x1e4(%rbp)     # imm = 0x101
14099e024: c6 85 b7 02 00 00 00        	movb	$0x0, 0x2b7(%rbp)
14099e02b: 48 8d 8d a0 01 00 00        	leaq	0x1a0(%rbp), %rcx
14099e032: 48 8b 95 c0 00 00 00        	movq	0xc0(%rbp), %rdx
14099e039: e8 32 6f 16 00              	callq	0x140b04f70 <.text+0xb03f70>
14099e03e: 4c 8b 85 88 02 00 00        	movq	0x288(%rbp), %r8
14099e045: 44 0f b6 8d c8 00 00 00     	movzbl	0xc8(%rbp), %r9d
14099e04d: 4c 8b 95 d0 00 00 00        	movq	0xd0(%rbp), %r10
14099e054: 4c 8b 9d d8 00 00 00        	movq	0xd8(%rbp), %r11
14099e05b: 4c 8b bd e0 00 00 00        	movq	0xe0(%rbp), %r15
14099e062: e9 78 fd ff ff              	jmp	0x14099dddf <.text+0x99cddf>
14099e067: 48 8b 85 90 02 00 00        	movq	0x290(%rbp), %rax
14099e06e: 48 8b 50 08                 	movq	0x8(%rax), %rdx
14099e072: 4c 8b 40 10                 	movq	0x10(%rax), %r8
14099e076: 48 83 c2 10                 	addq	$0x10, %rdx
14099e07a: 4c 8d b5 a0 01 00 00        	leaq	0x1a0(%rbp), %r14
14099e081: 4c 89 f1                    	movq	%r14, %rcx
14099e084: e8 f7 02 ec ff              	callq	0x14085e380 <.text+0x85d380>
14099e089: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14099e093: 48 89 85 b8 01 00 00        	movq	%rax, 0x1b8(%rbp)
14099e09a: c7 85 d0 01 00 00 00 00 00 00       	movl	$0x0, 0x1d0(%rbp)
14099e0a4: c7 85 e0 01 00 00 00 00 00 40       	movl	$0x40000000, 0x1e0(%rbp) # imm = 0x40000000
14099e0ae: 66 c7 85 e4 01 00 00 01 01  	movw	$0x101, 0x1e4(%rbp)     # imm = 0x101
14099e0b7: 48 8b 85 98 01 00 00        	movq	0x198(%rbp), %rax
14099e0be: 48 8b 10                    	movq	(%rax), %rdx
14099e0c1: 4c 89 f1                    	movq	%r14, %rcx
14099e0c4: e8 a7 6e 16 00              	callq	0x140b04f70 <.text+0xb03f70>
14099e0c9: 48 8b 85 88 01 00 00        	movq	0x188(%rbp), %rax
14099e0d0: 48 c1 e0 04                 	shlq	$0x4, %rax
14099e0d4: 48 8b b5 a8 02 00 00        	movq	0x2a8(%rbp), %rsi
14099e0db: 48 01 c6                    	addq	%rax, %rsi
14099e0de: 48 8b 85 90 02 00 00        	movq	0x290(%rbp), %rax
14099e0e5: 48 8b 48 30                 	movq	0x30(%rax), %rcx
14099e0e9: 48 8b 50 38                 	movq	0x38(%rax), %rdx
14099e0ed: e8 de a2 f1 ff              	callq	0x1408b83d0 <.text+0x8b73d0>
14099e0f2: 48 8d 8d a0 01 00 00        	leaq	0x1a0(%rbp), %rcx
14099e0f9: 89 c2                       	movl	%eax, %edx
14099e0fb: 4c 8b 85 80 00 00 00        	movq	0x80(%rbp), %r8
14099e102: e8 89 ad f1 ff              	callq	0x1408b8e90 <.text+0x8b7e90>
14099e107: 48 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %rax
14099e10e: f3 0f 10 70 10              	movss	0x10(%rax), %xmm6
14099e113: f3 0f 58 70 14              	addss	0x14(%rax), %xmm6
14099e118: f3 44 0f 10 05 4f 77 f8 01  	movss	0x1f8774f(%rip), %xmm8  # 0x142925870
14099e121: f3 41 0f 59 f0              	mulss	%xmm8, %xmm6
14099e126: f3 41 0f 59 f0              	mulss	%xmm8, %xmm6
14099e12b: f3 0f 10 3d 51 78 f8 01     	movss	0x1f87851(%rip), %xmm7  # 0x142925984
14099e133: f3 0f 58 f7                 	addss	%xmm7, %xmm6
14099e137: f3 0f 10 4e 10              	movss	0x10(%rsi), %xmm1
14099e13c: f3 0f 10 56 18              	movss	0x18(%rsi), %xmm2
14099e141: 4c 8d b5 48 01 00 00        	leaq	0x148(%rbp), %r14
14099e148: 4c 89 f1                    	movq	%r14, %rcx
14099e14b: 0f 28 de                    	movaps	%xmm6, %xmm3
14099e14e: e8 ed 93 2f 00              	callq	0x140c97540 <.text+0xc96540>
14099e153: f3 0f 10 5e 14              	movss	0x14(%rsi), %xmm3
14099e158: 48 8d bd f0 00 00 00        	leaq	0xf0(%rbp), %rdi
14099e15f: 48 89 f9                    	movq	%rdi, %rcx
14099e162: 4c 89 f2                    	movq	%r14, %rdx
14099e165: 0f 28 d7                    	movaps	%xmm7, %xmm2
14099e168: e8 e3 9a 2f 00              	callq	0x140c97c50 <.text+0xc96c50>
14099e16d: 4c 8d 05 8c d7 11 02        	leaq	0x211d78c(%rip), %r8    # 0x142abb900
14099e174: 4c 8d bd a0 01 00 00        	leaq	0x1a0(%rbp), %r15
14099e17b: 4c 89 f9                    	movq	%r15, %rcx
14099e17e: 48 89 fa                    	movq	%rdi, %rdx
14099e181: e8 3a e9 95 00              	callq	0x1412fcac0 <.text+0x12fbac0>
14099e186: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14099e190: 48 89 85 b8 01 00 00        	movq	%rax, 0x1b8(%rbp)
14099e197: c7 85 d0 01 00 00 00 00 00 00       	movl	$0x0, 0x1d0(%rbp)
14099e1a1: c7 85 e0 01 00 00 00 00 00 40       	movl	$0x40000000, 0x1e0(%rbp) # imm = 0x40000000
14099e1ab: 66 c7 85 e4 01 00 00 01 00  	movw	$0x1, 0x1e4(%rbp)
14099e1b4: 48 8b 85 98 01 00 00        	movq	0x198(%rbp), %rax
14099e1bb: 4c 8b 30                    	movq	(%rax), %r14
14099e1be: 4c 89 f9                    	movq	%r15, %rcx
14099e1c1: 4c 89 f2                    	movq	%r14, %rdx
14099e1c4: e8 a7 6d 16 00              	callq	0x140b04f70 <.text+0xb03f70>
14099e1c9: f3 0f 58 35 17 fb 03 02     	addss	0x203fb17(%rip), %xmm6  # 0x1429ddce8
14099e1d1: f3 41 0f 5d f0              	minss	%xmm8, %xmm6
14099e1d6: f3 0f 11 b5 80 01 00 00     	movss	%xmm6, 0x180(%rbp)
14099e1de: 8b 46 18                    	movl	0x18(%rsi), %eax
14099e1e1: 89 85 78 01 00 00           	movl	%eax, 0x178(%rbp)
14099e1e7: 48 89 b5 a8 02 00 00        	movq	%rsi, 0x2a8(%rbp)
14099e1ee: 48 8b 46 10                 	movq	0x10(%rsi), %rax
14099e1f2: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
14099e1f9: 48 8d 45 78                 	leaq	0x78(%rbp), %rax
14099e1fd: 48 89 85 a0 01 00 00        	movq	%rax, 0x1a0(%rbp)
14099e204: 48 8d 05 f5 c9 34 00        	leaq	0x34c9f5(%rip), %rax    # 0x140ceac00 <.text+0xce9c00>
14099e20b: 48 89 85 a8 01 00 00        	movq	%rax, 0x1a8(%rbp)
14099e212: 48 8d 85 70 01 00 00        	leaq	0x170(%rbp), %rax
14099e219: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
14099e220: 48 8d 05 a9 d5 3a 00        	leaq	0x3ad5a9(%rip), %rax    # 0x140d4b7d0 <.text+0xd4a7d0>
14099e227: 48 89 85 b8 01 00 00        	movq	%rax, 0x1b8(%rbp)
14099e22e: 48 8d 85 80 01 00 00        	leaq	0x180(%rbp), %rax
14099e235: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
14099e23c: 48 8d 05 cd e6 3e 00        	leaq	0x3ee6cd(%rip), %rax    # 0x140d8c910 <.text+0xd8b910>
14099e243: 48 89 85 c8 01 00 00        	movq	%rax, 0x1c8(%rbp)
14099e24a: 48 8d 05 6f d9 11 02        	leaq	0x211d96f(%rip), %rax   # 0x142abbbc0
14099e251: 48 89 85 f0 00 00 00        	movq	%rax, 0xf0(%rbp)
14099e258: 48 c7 85 f8 00 00 00 03 00 00 00    	movq	$0x3, 0xf8(%rbp)
14099e263: 48 c7 85 10 01 00 00 00 00 00 00    	movq	$0x0, 0x110(%rbp)
14099e26e: 4c 89 bd 00 01 00 00        	movq	%r15, 0x100(%rbp)
14099e275: 48 c7 85 08 01 00 00 03 00 00 00    	movq	$0x3, 0x108(%rbp)
14099e280: 48 8d 8d 48 01 00 00        	leaq	0x148(%rbp), %rcx
14099e287: 48 8d 95 f0 00 00 00        	leaq	0xf0(%rbp), %rdx
14099e28e: e8 6d e6 ff ff              	callq	0x14099c900 <.text+0x99b900>
14099e293: 48 8b 85 50 01 00 00        	movq	0x150(%rbp), %rax
14099e29a: 48 89 85 88 02 00 00        	movq	%rax, 0x288(%rbp)
14099e2a1: 4c 8b bd 58 01 00 00        	movq	0x158(%rbp), %r15
14099e2a8: 8b 85 78 01 00 00           	movl	0x178(%rbp), %eax
14099e2ae: 89 85 a8 01 00 00           	movl	%eax, 0x1a8(%rbp)
14099e2b4: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
14099e2bb: 48 89 85 a0 01 00 00        	movq	%rax, 0x1a0(%rbp)
14099e2c2: f3 0f 10 85 80 01 00 00     	movss	0x180(%rbp), %xmm0
14099e2ca: f3 0f 11 85 ac 01 00 00     	movss	%xmm0, 0x1ac(%rbp)
14099e2d2: 48 8d 8d a0 01 00 00        	leaq	0x1a0(%rbp), %rcx
14099e2d9: e8 02 e7 ff ff              	callq	0x14099c9e0 <.text+0x99b9e0>
14099e2de: 48 8d 0d 0b d9 11 02        	leaq	0x211d90b(%rip), %rcx   # 0x142abbbf0
14099e2e5: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
14099e2ea: 48 8d 8d a0 01 00 00        	leaq	0x1a0(%rbp), %rcx
14099e2f1: 48 8b 95 88 02 00 00        	movq	0x288(%rbp), %rdx
14099e2f8: 4d 89 f8                    	movq	%r15, %r8
14099e2fb: 49 89 c1                    	movq	%rax, %r9
14099e2fe: e8 ad f5 ef ff              	callq	0x14089d8b0 <.text+0x89c8b0>
14099e303: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
14099e30a: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
14099e311: 0f 10 85 a0 01 00 00        	movups	0x1a0(%rbp), %xmm0
14099e318: 0f 10 8d b0 01 00 00        	movups	0x1b0(%rbp), %xmm1
14099e31f: 0f 29 8d 00 01 00 00        	movaps	%xmm1, 0x100(%rbp)
14099e326: 0f 29 85 f0 00 00 00        	movaps	%xmm0, 0xf0(%rbp)
14099e32d: 48 8d 95 f0 00 00 00        	leaq	0xf0(%rbp), %rdx
14099e334: 4c 89 f1                    	movq	%r14, %rcx
14099e337: e8 44 e5 ac 00              	callq	0x14146c880 <.text+0x146b880>
14099e33c: 48 8b 95 48 01 00 00        	movq	0x148(%rbp), %rdx
14099e343: 48 85 d2                    	testq	%rdx, %rdx
14099e346: 74 12                       	je	0x14099e35a <.text+0x99d35a>
14099e348: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14099e34e: 48 8b 8d 88 02 00 00        	movq	0x288(%rbp), %rcx
14099e355: e8 c6 58 c7 ff              	callq	0x140613c20 <.text+0x612c20>
14099e35a: 80 7b 18 02                 	cmpb	$0x2, 0x18(%rbx)
14099e35e: 48 8b 85 80 02 00 00        	movq	0x280(%rbp), %rax
14099e365: 48 8b 8d 90 02 00 00        	movq	0x290(%rbp), %rcx
14099e36c: 0f 82 c8 03 00 00           	jb	0x14099e73a <.text+0x99d73a>
14099e372: 48 39 43 38                 	cmpq	%rax, 0x38(%rbx)
14099e376: 0f 85 be 03 00 00           	jne	0x14099e73a <.text+0x99d73a>
14099e37c: 80 7b 19 03                 	cmpb	$0x3, 0x19(%rbx)
14099e380: 0f 85 b4 03 00 00           	jne	0x14099e73a <.text+0x99d73a>
14099e386: 48 89 cb                    	movq	%rcx, %rbx
14099e389: 48 8b 85 98 02 00 00        	movq	0x298(%rbp), %rax
14099e390: 48 8b 8d 88 01 00 00        	movq	0x188(%rbp), %rcx
14099e397: 48 8d 3c c8                 	leaq	(%rax,%rcx,8), %rdi
14099e39b: 48 8b 85 a8 02 00 00        	movq	0x2a8(%rbp), %rax
14099e3a2: 4c 8d 78 10                 	leaq	0x10(%rax), %r15
14099e3a6: 48 8b 85 90 00 00 00        	movq	0x90(%rbp), %rax
14099e3ad: 8b 8d 64 02 00 00           	movl	0x264(%rbp), %ecx
14099e3b3: 89 08                       	movl	%ecx, (%rax)
14099e3b5: 48 8b 17                    	movq	(%rdi), %rdx
14099e3b8: 48 8b 8d 88 00 00 00        	movq	0x88(%rbp), %rcx
14099e3bf: e8 2c bf 92 ff              	callq	0x1402ca2f0 <.text+0x2c92f0>
14099e3c4: 48 8b 17                    	movq	(%rdi), %rdx
14099e3c7: 4c 8d 05 52 d8 11 02        	leaq	0x211d852(%rip), %r8    # 0x142abbc20
14099e3ce: 48 8d 4d f0                 	leaq	-0x10(%rbp), %rcx
14099e3d2: e8 e9 93 98 00              	callq	0x1413277c0 <.text+0x13267c0>
14099e3d7: 48 8b 4b 30                 	movq	0x30(%rbx), %rcx
14099e3db: 48 8b 53 38                 	movq	0x38(%rbx), %rdx
14099e3df: e8 ec 9f f1 ff              	callq	0x1408b83d0 <.text+0x8b73d0>
14099e3e4: 41 83 3c 24 09              	cmpl	$0x9, (%r12)
14099e3e9: 75 4f                       	jne	0x14099e43a <.text+0x99d43a>
14099e3eb: 41 80 7c 24 04 03           	cmpb	$0x3, 0x4(%r12)
14099e3f1: 75 47                       	jne	0x14099e43a <.text+0x99d43a>
14099e3f3: 41 8b 4f 08                 	movl	0x8(%r15), %ecx
14099e3f7: 89 8d b8 01 00 00           	movl	%ecx, 0x1b8(%rbp)
14099e3fd: 49 8b 0f                    	movq	(%r15), %rcx
14099e400: 48 89 8d b0 01 00 00        	movq	%rcx, 0x1b0(%rbp)
14099e407: c6 85 a0 01 00 00 20        	movb	$0x20, 0x1a0(%rbp)
14099e40e: 88 85 a1 01 00 00           	movb	%al, 0x1a1(%rbp)
14099e414: c6 85 bc 01 00 00 00        	movb	$0x0, 0x1bc(%rbp)
14099e41b: 4c 8d 05 5e d8 11 02        	leaq	0x211d85e(%rip), %r8    # 0x142abbc80
14099e422: 48 8d 95 a0 01 00 00        	leaq	0x1a0(%rbp), %rdx
14099e429: 48 8b 8d a0 03 00 00        	movq	0x3a0(%rbp), %rcx
14099e430: e8 5b 8e 98 00              	callq	0x141327290 <.text+0x1326290>
14099e435: e9 00 03 00 00              	jmp	0x14099e73a <.text+0x99d73a>
14099e43a: 41 8b 47 08                 	movl	0x8(%r15), %eax
14099e43e: 89 85 b8 01 00 00           	movl	%eax, 0x1b8(%rbp)
14099e444: 49 8b 07                    	movq	(%r15), %rax
14099e447: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
14099e44e: c6 85 a0 01 00 00 25        	movb	$0x25, 0x1a0(%rbp)
14099e455: c6 85 bc 01 00 00 00        	movb	$0x0, 0x1bc(%rbp)
14099e45c: 4c 8d 05 d5 d7 11 02        	leaq	0x211d7d5(%rip), %r8    # 0x142abbc38
14099e463: 48 8d 95 a0 01 00 00        	leaq	0x1a0(%rbp), %rdx
14099e46a: 48 8b 8d a0 03 00 00        	movq	0x3a0(%rbp), %rcx
14099e471: e8 1a 8e 98 00              	callq	0x141327290 <.text+0x1326290>
14099e476: 48 8b 85 a8 02 00 00        	movq	0x2a8(%rbp), %rax
14099e47d: 0f 28 30                    	movaps	(%rax), %xmm6
14099e480: 48 8b 85 90 02 00 00        	movq	0x290(%rbp), %rax
14099e487: 8b 40 18                    	movl	0x18(%rax), %eax
14099e48a: 85 c0                       	testl	%eax, %eax
14099e48c: 74 44                       	je	0x14099e4d2 <.text+0x99d4d2>
14099e48e: 83 f8 01                    	cmpl	$0x1, %eax
14099e491: 75 66                       	jne	0x14099e4f9 <.text+0x99d4f9>
14099e493: 41 8b 47 08                 	movl	0x8(%r15), %eax
14099e497: 48 8b 8d 90 02 00 00        	movq	0x290(%rbp), %rcx
14099e49e: f3 44 0f 7e 41 20           	movq	0x20(%rcx), %xmm8
14099e4a4: 89 85 60 02 00 00           	movl	%eax, 0x260(%rbp)
14099e4aa: 49 8b 07                    	movq	(%r15), %rax
14099e4ad: 48 89 85 58 02 00 00        	movq	%rax, 0x258(%rbp)
14099e4b4: 0f 29 75 50                 	movaps	%xmm6, 0x50(%rbp)
14099e4b8: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
14099e4bf: 48 8d 55 50                 	leaq	0x50(%rbp), %rdx
14099e4c3: e8 58 df ff ff              	callq	0x14099c420 <.text+0x99b420>
14099e4c8: 31 f6                       	xorl	%esi, %esi
14099e4ca: 45 31 ff                    	xorl	%r15d, %r15d
14099e4cd: e9 25 01 00 00              	jmp	0x14099e5f7 <.text+0x99d5f7>
14099e4d2: 48 8b 85 90 02 00 00        	movq	0x290(%rbp), %rax
14099e4d9: 48 8b 78 28                 	movq	0x28(%rax), %rdi
14099e4dd: 83 78 1c 01                 	cmpl	$0x1, 0x1c(%rax)
14099e4e1: 75 4f                       	jne	0x14099e532 <.text+0x99d532>
14099e4e3: 48 8b 85 90 02 00 00        	movq	0x290(%rbp), %rax
14099e4ea: f2 0f 10 78 20              	movsd	0x20(%rax), %xmm7
14099e4ef: 66 4c 0f 6e c7              	movq	%rdi, %xmm8
14099e4f4: e9 c1 00 00 00              	jmp	0x14099e5ba <.text+0x99d5ba>
14099e4f9: 41 8b 47 08                 	movl	0x8(%r15), %eax
14099e4fd: 89 85 60 02 00 00           	movl	%eax, 0x260(%rbp)
14099e503: 49 8b 07                    	movq	(%r15), %rax
14099e506: 48 89 85 58 02 00 00        	movq	%rax, 0x258(%rbp)
14099e50d: 48 b8 00 00 00 00 00 00 80 3f       	movabsq	$0x3f80000000000000, %rax # imm = 0x3F80000000000000
14099e517: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
14099e51e: c7 85 68 01 00 00 00 00 00 00       	movl	$0x0, 0x168(%rbp)
14099e528: 41 b7 0b                    	movb	$0xb, %r15b
14099e52b: 31 f6                       	xorl	%esi, %esi
14099e52d: e9 c5 00 00 00              	jmp	0x14099e5f7 <.text+0x99d5f7>
14099e532: 0f 57 c0                    	xorps	%xmm0, %xmm0
14099e535: 48 8b 85 a8 02 00 00        	movq	0x2a8(%rbp), %rax
14099e53c: f3 0f 58 40 18              	addss	0x18(%rax), %xmm0
14099e541: f2 0f 10 48 10              	movsd	0x10(%rax), %xmm1
14099e546: 0f 58 0d b3 46 0e 02        	addps	0x20e46b3(%rip), %xmm1  # 0x142a82c00
14099e54d: 0f 13 8d f0 00 00 00        	movlps	%xmm1, 0xf0(%rbp)
14099e554: f3 0f 11 85 f8 00 00 00     	movss	%xmm0, 0xf8(%rbp)
14099e55c: f2 0f 10 05 9c f7 03 02     	movsd	0x203f79c(%rip), %xmm0  # 0x1429ddd00
14099e564: f2 0f 11 85 fc 00 00 00     	movsd	%xmm0, 0xfc(%rbp)
14099e56c: c7 85 04 01 00 00 00 00 00 80       	movl	$0x80000000, 0x104(%rbp) # imm = 0x80000000
14099e576: c7 44 24 30 02 00 00 00     	movl	$0x2, 0x30(%rsp)
14099e57e: c7 44 24 28 ff ff 7f 7f     	movl	$0x7f7fffff, 0x28(%rsp) # imm = 0x7F7FFFFF
14099e586: 48 8d 8d a0 01 00 00        	leaq	0x1a0(%rbp), %rcx
14099e58d: 4c 8d 85 f0 00 00 00        	leaq	0xf0(%rbp), %r8
14099e594: 4c 89 ea                    	movq	%r13, %rdx
14099e597: 45 31 c9                    	xorl	%r9d, %r9d
14099e59a: e8 c1 f6 f2 ff              	callq	0x1408cdc60 <.text+0x8ccc60>
14099e59f: 8b 85 d4 01 00 00           	movl	0x1d4(%rbp), %eax
14099e5a5: 83 f8 02                    	cmpl	$0x2, %eax
14099e5a8: 0f 85 b6 01 00 00           	jne	0x14099e764 <.text+0x99d764>
14099e5ae: 66 4c 0f 6e c7              	movq	%rdi, %xmm8
14099e5b3: 0f 28 3d 26 73 f8 01        	movaps	0x1f87326(%rip), %xmm7  # 0x1429258e0
14099e5ba: 40 b7 01                    	movb	$0x1, %dil
14099e5bd: 45 31 f6                    	xorl	%r14d, %r14d
14099e5c0: 41 8b 47 08                 	movl	0x8(%r15), %eax
14099e5c4: 89 85 60 02 00 00           	movl	%eax, 0x260(%rbp)
14099e5ca: 49 8b 07                    	movq	(%r15), %rax
14099e5cd: 48 89 85 58 02 00 00        	movq	%rax, 0x258(%rbp)
14099e5d4: 48 b8 00 00 00 00 00 00 80 3f       	movabsq	$0x3f80000000000000, %rax # imm = 0x3F80000000000000
14099e5de: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
14099e5e5: c7 85 68 01 00 00 00 00 00 00       	movl	$0x0, 0x168(%rbp)
14099e5ef: 41 b7 02                    	movb	$0x2, %r15b
14099e5f2: be 01 00 00 00              	movl	$0x1, %esi
14099e5f7: 48 8b 85 90 02 00 00        	movq	0x290(%rbp), %rax
14099e5fe: 48 8b 48 30                 	movq	0x30(%rax), %rcx
14099e602: 48 8b 50 38                 	movq	0x38(%rax), %rdx
14099e606: e8 c5 9d f1 ff              	callq	0x1408b83d0 <.text+0x8b73d0>
14099e60b: 89 c3                       	movl	%eax, %ebx
14099e60d: 48 8d 8d a0 01 00 00        	leaq	0x1a0(%rbp), %rcx
14099e614: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
14099e618: e8 53 88 2f 00              	callq	0x140c96e70 <.text+0xc95e70>
14099e61d: 0f 28 85 a0 01 00 00        	movaps	0x1a0(%rbp), %xmm0
14099e624: 0f 57 05 d5 ea 02 02        	xorps	0x202ead5(%rip), %xmm0  # 0x1429cd100
14099e62b: 0f 28 ce                    	movaps	%xmm6, %xmm1
14099e62e: 0f c6 ce 00                 	shufps	$0x0, %xmm6, %xmm1      # xmm1 = xmm1[0,0],xmm6[0,0]
14099e632: 0f 28 d6                    	movaps	%xmm6, %xmm2
14099e635: 0f c6 d6 55                 	shufps	$0x55, %xmm6, %xmm2     # xmm2 = xmm2[1,1],xmm6[1,1]
14099e639: 0f 28 de                    	movaps	%xmm6, %xmm3
14099e63c: 0f c6 de aa                 	shufps	$0xaa, %xmm6, %xmm3     # xmm3 = xmm3[2,2],xmm6[2,2]
14099e640: 0f c6 f6 ff                 	shufps	$0xff, %xmm6, %xmm6     # xmm6 = xmm6[3,3,3,3]
14099e644: 0f 59 f0                    	mulps	%xmm0, %xmm6
14099e647: 66 0f 70 e0 1b              	pshufd	$0x1b, %xmm0, %xmm4     # xmm4 = xmm0[3,2,1,0]
14099e64c: 0f 59 e1                    	mulps	%xmm1, %xmm4
14099e64f: 66 0f 70 c8 4e              	pshufd	$0x4e, %xmm0, %xmm1     # xmm1 = xmm0[2,3,0,1]
14099e654: 0f 59 ca                    	mulps	%xmm2, %xmm1
14099e657: 0f 59 25 12 eb 02 02        	mulps	0x202eb12(%rip), %xmm4  # 0x1429cd170
14099e65e: 66 0f 70 c0 b1              	pshufd	$0xb1, %xmm0, %xmm0     # xmm0 = xmm0[1,0,3,2]
14099e663: 0f 59 c3                    	mulps	%xmm3, %xmm0
14099e666: 0f 59 0d 13 eb 02 02        	mulps	0x202eb13(%rip), %xmm1  # 0x1429cd180
14099e66d: 0f 59 05 1c eb 02 02        	mulps	0x202eb1c(%rip), %xmm0  # 0x1429cd190
14099e674: 0f 58 e6                    	addps	%xmm6, %xmm4
14099e677: 0f 58 c1                    	addps	%xmm1, %xmm0
14099e67a: 0f 58 c4                    	addps	%xmm4, %xmm0
14099e67d: 48 8b 85 80 02 00 00        	movq	0x280(%rbp), %rax
14099e684: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
14099e68b: 44 88 bd b8 01 00 00        	movb	%r15b, 0x1b8(%rbp)
14099e692: 40 88 bd b9 01 00 00        	movb	%dil, 0x1b9(%rbp)
14099e699: 44 88 b5 ba 01 00 00        	movb	%r14b, 0x1ba(%rbp)
14099e6a0: 8b 85 70 02 00 00           	movl	0x270(%rbp), %eax
14099e6a6: 89 85 bb 01 00 00           	movl	%eax, 0x1bb(%rbp)
14099e6ac: 0f b6 85 74 02 00 00        	movzbl	0x274(%rbp), %eax
14099e6b3: 88 85 bf 01 00 00           	movb	%al, 0x1bf(%rbp)
14099e6b9: 66 44 0f 7f 85 c0 01 00 00  	movdqa	%xmm8, 0x1c0(%rbp)
14099e6c2: 48 8b 85 58 02 00 00        	movq	0x258(%rbp), %rax
14099e6c9: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
14099e6d0: 8b 85 60 02 00 00           	movl	0x260(%rbp), %eax
14099e6d6: 89 85 d8 01 00 00           	movl	%eax, 0x1d8(%rbp)
14099e6dc: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
14099e6e3: 48 89 85 dc 01 00 00        	movq	%rax, 0x1dc(%rbp)
14099e6ea: 8b 85 68 01 00 00           	movl	0x168(%rbp), %eax
14099e6f0: 89 85 e4 01 00 00           	movl	%eax, 0x1e4(%rbp)
14099e6f6: 89 b5 e8 01 00 00           	movl	%esi, 0x1e8(%rbp)
14099e6fc: 0f 13 bd ec 01 00 00        	movlps	%xmm7, 0x1ec(%rbp)
14099e703: 88 9d f8 01 00 00           	movb	%bl, 0x1f8(%rbp)
14099e709: 0f 29 85 a0 01 00 00        	movaps	%xmm0, 0x1a0(%rbp)
14099e710: 4c 8d 05 39 d5 11 02        	leaq	0x211d539(%rip), %r8    # 0x142abbc50
14099e717: 48 8d 95 a0 01 00 00        	leaq	0x1a0(%rbp), %rdx
14099e71e: 48 8b 8d 68 03 00 00        	movq	0x368(%rbp), %rcx
14099e725: e8 46 8e 98 00              	callq	0x141327570 <.text+0x1326570>
14099e72a: 48 8d 15 37 d5 11 02        	leaq	0x211d537(%rip), %rdx   # 0x142abbc68
14099e731: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
14099e735: e8 76 92 98 00              	callq	0x1413279b0 <.text+0x13269b0>
14099e73a: 0f 28 b5 c0 02 00 00        	movaps	0x2c0(%rbp), %xmm6
14099e741: 0f 28 bd d0 02 00 00        	movaps	0x2d0(%rbp), %xmm7
14099e748: 44 0f 28 85 e0 02 00 00     	movaps	0x2e0(%rbp), %xmm8
14099e750: 48 81 c4 78 03 00 00        	addq	$0x378, %rsp            # imm = 0x378
14099e757: 5b                          	popq	%rbx
14099e758: 5f                          	popq	%rdi
14099e759: 5e                          	popq	%rsi
14099e75a: 41 5c                       	popq	%r12
14099e75c: 41 5d                       	popq	%r13
14099e75e: 41 5e                       	popq	%r14
14099e760: 41 5f                       	popq	%r15
14099e762: 5d                          	popq	%rbp
14099e763: c3                          	retq
14099e764: a8 01                       	testb	$0x1, %al
14099e766: 0f 84 42 fe ff ff           	je	0x14099e5ae <.text+0x99d5ae>
14099e76c: 80 bd a0 01 00 00 02        	cmpb	$0x2, 0x1a0(%rbp)
14099e773: 0f 85 35 fe ff ff           	jne	0x14099e5ae <.text+0x99d5ae>
14099e779: f2 0f 10 bd d8 01 00 00     	movsd	0x1d8(%rbp), %xmm7
14099e781: 0f b6 bd a1 01 00 00        	movzbl	0x1a1(%rbp), %edi
14099e788: 44 0f b6 b5 a2 01 00 00     	movzbl	0x1a2(%rbp), %r14d
14099e790: 8b 85 a3 01 00 00           	movl	0x1a3(%rbp), %eax
14099e796: 89 85 70 02 00 00           	movl	%eax, 0x270(%rbp)
14099e79c: 0f b6 85 a7 01 00 00        	movzbl	0x1a7(%rbp), %eax
14099e7a3: 88 85 74 02 00 00           	movb	%al, 0x274(%rbp)
14099e7a9: f3 44 0f 6f 85 a8 01 00 00  	movdqu	0x1a8(%rbp), %xmm8
14099e7b2: e9 09 fe ff ff              	jmp	0x14099e5c0 <.text+0x99d5c0>
14099e7b7: b9 10 00 00 00              	movl	$0x10, %ecx
14099e7bc: ba 60 00 00 00              	movl	$0x60, %edx
14099e7c1: e8 3a a8 f3 01              	callq	0x1428d9000 <.text+0x28d8000>
14099e7c6: eb 17                       	jmp	0x14099e7df <.text+0x99d7df>
14099e7c8: c6 85 b7 02 00 00 01        	movb	$0x1, 0x2b7(%rbp)
14099e7cf: 4c 8d 05 a2 d3 11 02        	leaq	0x211d3a2(%rip), %r8    # 0x142abbb78
14099e7d6: 31 c9                       	xorl	%ecx, %ecx
14099e7d8: 31 d2                       	xorl	%edx, %edx
14099e7da: e8 39 ad f3 01              	callq	0x1428d9518 <.text+0x28d8518>
14099e7df: 0f 0b                       	ud2
14099e7e1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
14099e7f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14099e7f5: 55                          	pushq	%rbp
14099e7f6: 41 57                       	pushq	%r15
14099e7f8: 41 56                       	pushq	%r14
14099e7fa: 41 55                       	pushq	%r13
14099e7fc: 41 54                       	pushq	%r12
14099e7fe: 56                          	pushq	%rsi
14099e7ff: 57                          	pushq	%rdi
14099e800: 53                          	pushq	%rbx
14099e801: 48 83 ec 78                 	subq	$0x78, %rsp
14099e805: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14099e80c: 44 0f 29 44 24 40           	movaps	%xmm8, 0x40(%rsp)
14099e812: 0f 29 7c 24 50              	movaps	%xmm7, 0x50(%rsp)
14099e817: 0f 29 74 24 60              	movaps	%xmm6, 0x60(%rsp)
14099e81c: 80 bd b7 02 00 00 00        	cmpb	$0x0, 0x2b7(%rbp)
14099e823: 74 26                       	je	0x14099e84b <.text+0x99d84b>
14099e825: 48 8b 85 f0 00 00 00        	movq	0xf0(%rbp), %rax
14099e82c: 48 85 c0                    	testq	%rax, %rax
14099e82f: 74 1a                       	je	0x14099e84b <.text+0x99d84b>
14099e831: 48 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %rcx
14099e838: 48 c1 e0 02                 	shlq	$0x2, %rax
14099e83c: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
14099e840: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14099e846: e8 d5 53 c7 ff              	callq	0x140613c20 <.text+0x612c20>
14099e84b: 0f 28 74 24 60              	movaps	0x60(%rsp), %xmm6
14099e850: 0f 28 7c 24 50              	movaps	0x50(%rsp), %xmm7
14099e855: 44 0f 28 44 24 40           	movaps	0x40(%rsp), %xmm8
14099e85b: 48 83 c4 78                 	addq	$0x78, %rsp
14099e85f: 5b                          	popq	%rbx
14099e860: 5f                          	popq	%rdi
14099e861: 5e                          	popq	%rsi
14099e862: 41 5c                       	popq	%r12
14099e864: 41 5d                       	popq	%r13
14099e866: 41 5e                       	popq	%r14
14099e868: 41 5f                       	popq	%r15
14099e86a: 5d                          	popq	%rbp
14099e86b: c3                          	retq
14099e86c: 0f 1f 40 00                 	nopl	(%rax)
14099e870: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14099e875: 55                          	pushq	%rbp
14099e876: 41 57                       	pushq	%r15
14099e878: 41 56                       	pushq	%r14
14099e87a: 41 55                       	pushq	%r13
14099e87c: 41 54                       	pushq	%r12
14099e87e: 56                          	pushq	%rsi
14099e87f: 57                          	pushq	%rdi
14099e880: 53                          	pushq	%rbx
14099e881: 48 83 ec 78                 	subq	$0x78, %rsp
14099e885: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14099e88c: 44 0f 29 44 24 40           	movaps	%xmm8, 0x40(%rsp)
14099e892: 0f 29 7c 24 50              	movaps	%xmm7, 0x50(%rsp)
14099e897: 0f 29 74 24 60              	movaps	%xmm6, 0x60(%rsp)
14099e89c: 48 8b 95 48 01 00 00        	movq	0x148(%rbp), %rdx
14099e8a3: 48 85 d2                    	testq	%rdx, %rdx
14099e8a6: 74 12                       	je	0x14099e8ba <.text+0x99d8ba>
14099e8a8: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14099e8ae: 48 8b 8d 88 02 00 00        	movq	0x288(%rbp), %rcx
14099e8b5: e8 66 53 c7 ff              	callq	0x140613c20 <.text+0x612c20>
14099e8ba: 0f 28 74 24 60              	movaps	0x60(%rsp), %xmm6
14099e8bf: 0f 28 7c 24 50              	movaps	0x50(%rsp), %xmm7
14099e8c4: 44 0f 28 44 24 40           	movaps	0x40(%rsp), %xmm8
14099e8ca: 48 83 c4 78                 	addq	$0x78, %rsp
14099e8ce: 5b                          	popq	%rbx
14099e8cf: 5f                          	popq	%rdi
14099e8d0: 5e                          	popq	%rsi
14099e8d1: 41 5c                       	popq	%r12
14099e8d3: 41 5d                       	popq	%r13
14099e8d5: 41 5e                       	popq	%r14
14099e8d7: 41 5f                       	popq	%r15
14099e8d9: 5d                          	popq	%rbp
14099e8da: c3                          	retq
14099e8db: 0f 1f 44 00 00              	nopl	(%rax,%rax)
14099e8e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14099e8e5: 55                          	pushq	%rbp
14099e8e6: 41 57                       	pushq	%r15
14099e8e8: 41 56                       	pushq	%r14
14099e8ea: 41 55                       	pushq	%r13
14099e8ec: 41 54                       	pushq	%r12
14099e8ee: 56                          	pushq	%rsi
14099e8ef: 57                          	pushq	%rdi
14099e8f0: 53                          	pushq	%rbx
14099e8f1: 48 83 ec 78                 	subq	$0x78, %rsp
14099e8f5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14099e8fc: 44 0f 29 44 24 40           	movaps	%xmm8, 0x40(%rsp)
14099e902: 0f 29 7c 24 50              	movaps	%xmm7, 0x50(%rsp)
14099e907: 66 0f 7f 74 24 60           	movdqa	%xmm6, 0x60(%rsp)
14099e90d: 80 bd 98 02 00 00 00        	cmpb	$0x0, 0x298(%rbp)
14099e914: 74 2f                       	je	0x14099e945 <.text+0x99d945>
14099e916: 48 8d 85 50 01 00 00        	leaq	0x150(%rbp), %rax
14099e91d: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
14099e924: 48 8d 85 80 01 00 00        	leaq	0x180(%rbp), %rax
14099e92b: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
14099e932: 48 8d 0d a7 d0 11 02        	leaq	0x211d0a7(%rip), %rcx   # 0x142abb9e0
14099e939: 48 8d 95 70 01 00 00        	leaq	0x170(%rbp), %rdx
14099e940: e8 8b 09 99 00              	callq	0x14132f2d0 <.text+0x132e2d0>
14099e945: 0f 28 74 24 60              	movaps	0x60(%rsp), %xmm6
14099e94a: 0f 28 7c 24 50              	movaps	0x50(%rsp), %xmm7
14099e94f: 44 0f 28 44 24 40           	movaps	0x40(%rsp), %xmm8
14099e955: 48 83 c4 78                 	addq	$0x78, %rsp
14099e959: 5b                          	popq	%rbx
14099e95a: 5f                          	popq	%rdi
14099e95b: 5e                          	popq	%rsi
14099e95c: 41 5c                       	popq	%r12
14099e95e: 41 5d                       	popq	%r13
14099e960: 41 5e                       	popq	%r14
14099e962: 41 5f                       	popq	%r15
14099e964: 5d                          	popq	%rbp
14099e965: c3                          	retq
14099e966: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
14099e970: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14099e975: 55                          	pushq	%rbp
14099e976: 41 57                       	pushq	%r15
14099e978: 41 56                       	pushq	%r14
14099e97a: 41 55                       	pushq	%r13
14099e97c: 41 54                       	pushq	%r12
14099e97e: 56                          	pushq	%rsi
14099e97f: 57                          	pushq	%rdi
14099e980: 53                          	pushq	%rbx
14099e981: 48 83 ec 78                 	subq	$0x78, %rsp
14099e985: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14099e98c: 44 0f 29 44 24 40           	movaps	%xmm8, 0x40(%rsp)
14099e992: 0f 29 7c 24 50              	movaps	%xmm7, 0x50(%rsp)
14099e997: 66 0f 7f 74 24 60           	movdqa	%xmm6, 0x60(%rsp)
14099e99d: 48 8b 85 98 02 00 00        	movq	0x298(%rbp), %rax
14099e9a4: 48 ff 00                    	incq	(%rax)
14099e9a7: 0f 28 74 24 60              	movaps	0x60(%rsp), %xmm6
14099e9ac: 0f 28 7c 24 50              	movaps	0x50(%rsp), %xmm7
14099e9b1: 44 0f 28 44 24 40           	movaps	0x40(%rsp), %xmm8
14099e9b7: 48 83 c4 78                 	addq	$0x78, %rsp
14099e9bb: 5b                          	popq	%rbx
14099e9bc: 5f                          	popq	%rdi
14099e9bd: 5e                          	popq	%rsi
14099e9be: 41 5c                       	popq	%r12
14099e9c0: 41 5d                       	popq	%r13
14099e9c2: 41 5e                       	popq	%r14
14099e9c4: 41 5f                       	popq	%r15
14099e9c6: 5d                          	popq	%rbp
14099e9c7: c3                          	retq
14099e9c8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
14099e9d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14099e9d5: 55                          	pushq	%rbp
14099e9d6: 41 57                       	pushq	%r15
14099e9d8: 41 56                       	pushq	%r14
14099e9da: 41 55                       	pushq	%r13
14099e9dc: 41 54                       	pushq	%r12
14099e9de: 56                          	pushq	%rsi
14099e9df: 57                          	pushq	%rdi
14099e9e0: 53                          	pushq	%rbx
14099e9e1: 48 83 ec 78                 	subq	$0x78, %rsp
14099e9e5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14099e9ec: 44 0f 29 44 24 40           	movaps	%xmm8, 0x40(%rsp)
14099e9f2: 0f 29 7c 24 50              	movaps	%xmm7, 0x50(%rsp)
14099e9f7: 66 0f 7f 74 24 60           	movdqa	%xmm6, 0x60(%rsp)
14099e9fd: 48 8b 85 80 02 00 00        	movq	0x280(%rbp), %rax
14099ea04: 48 ff 00                    	incq	(%rax)
14099ea07: 0f 28 74 24 60              	movaps	0x60(%rsp), %xmm6
14099ea0c: 0f 28 7c 24 50              	movaps	0x50(%rsp), %xmm7
14099ea11: 44 0f 28 44 24 40           	movaps	0x40(%rsp), %xmm8
14099ea17: 48 83 c4 78                 	addq	$0x78, %rsp
14099ea1b: 5b                          	popq	%rbx
14099ea1c: 5f                          	popq	%rdi
14099ea1d: 5e                          	popq	%rsi
14099ea1e: 41 5c                       	popq	%r12
14099ea20: 41 5d                       	popq	%r13
14099ea22: 41 5e                       	popq	%r14
14099ea24: 41 5f                       	popq	%r15
14099ea26: 5d                          	popq	%rbp
14099ea27: c3                          	retq
14099ea28: cc                          	int3
14099ea29: cc                          	int3
14099ea2a: cc                          	int3
14099ea2b: cc                          	int3
14099ea2c: cc                          	int3
14099ea2d: cc                          	int3
14099ea2e: cc                          	int3
14099ea2f: cc                          	int3
