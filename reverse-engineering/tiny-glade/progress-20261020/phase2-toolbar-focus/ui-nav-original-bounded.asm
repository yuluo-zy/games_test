
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14233eed0: 55                          	pushq	%rbp
14233eed1: 41 57                       	pushq	%r15
14233eed3: 41 56                       	pushq	%r14
14233eed5: 41 55                       	pushq	%r13
14233eed7: 41 54                       	pushq	%r12
14233eed9: 56                          	pushq	%rsi
14233eeda: 57                          	pushq	%rdi
14233eedb: 53                          	pushq	%rbx
14233eedc: 48 81 ec 78 02 00 00        	subq	$0x278, %rsp            # imm = 0x278
14233eee3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14233eeeb: 44 0f 29 85 e0 01 00 00     	movaps	%xmm8, 0x1e0(%rbp)
14233eef3: 0f 29 bd d0 01 00 00        	movaps	%xmm7, 0x1d0(%rbp)
14233eefa: 0f 29 b5 c0 01 00 00        	movaps	%xmm6, 0x1c0(%rbp)
14233ef01: 48 c7 85 b8 01 00 00 fe ff ff ff    	movq	$-0x2, 0x1b8(%rbp)
14233ef0c: 4c 89 cb                    	movq	%r9, %rbx
14233ef0f: 4c 89 c6                    	movq	%r8, %rsi
14233ef12: 48 89 95 80 01 00 00        	movq	%rdx, 0x180(%rbp)
14233ef19: 49 89 cf                    	movq	%rcx, %r15
14233ef1c: 4c 8b ad 60 02 00 00        	movq	0x260(%rbp), %r13
14233ef23: 49 8b 7d 00                 	movq	(%r13), %rdi
14233ef27: 48 8d 4f 10                 	leaq	0x10(%rdi), %rcx
14233ef2b: e8 10 a2 66 fe              	callq	0x1409a9140 <.text+0x9a8140>
14233ef30: 84 c0                       	testb	%al, %al
14233ef32: 74 16                       	je	0x14233ef4a <.text+0x233df4a>
14233ef34: 41 8b 45 1c                 	movl	0x1c(%r13), %eax
14233ef38: 49 8b 4d 10                 	movq	0x10(%r13), %rcx
14233ef3c: 89 01                       	movl	%eax, (%rcx)
14233ef3e: 48 c7 07 00 00 00 00        	movq	$0x0, (%rdi)
14233ef45: e9 a0 04 00 00              	jmp	0x14233f3ea <.text+0x233e3ea>
14233ef4a: 48 8b 36                    	movq	(%rsi), %rsi
14233ef4d: 8b 8e a0 00 00 00           	movl	0xa0(%rsi), %ecx
14233ef53: f3 0f 10 8e a4 00 00 00     	movss	0xa4(%rsi), %xmm1
14233ef5b: e8 30 0b 60 fe              	callq	0x14093fa90 <.text+0x93ea90>
14233ef60: 84 c0                       	testb	%al, %al
14233ef62: 74 1d                       	je	0x14233ef81 <.text+0x233df81>
14233ef64: 8b 8e a8 00 00 00           	movl	0xa8(%rsi), %ecx
14233ef6a: f3 0f 10 8e ac 00 00 00     	movss	0xac(%rsi), %xmm1
14233ef72: e8 19 0b 60 fe              	callq	0x14093fa90 <.text+0x93ea90>
14233ef77: 34 01                       	xorb	$0x1, %al
14233ef79: 89 85 ac 01 00 00           	movl	%eax, 0x1ac(%rbp)
14233ef7f: eb 0a                       	jmp	0x14233ef8b <.text+0x233df8b>
14233ef81: c7 85 ac 01 00 00 00 00 00 00       	movl	$0x0, 0x1ac(%rbp)
14233ef8b: 8b 8e a8 00 00 00           	movl	0xa8(%rsi), %ecx
14233ef91: f3 0f 10 8e ac 00 00 00     	movss	0xac(%rsi), %xmm1
14233ef99: e8 f2 0a 60 fe              	callq	0x14093fa90 <.text+0x93ea90>
14233ef9e: 84 c0                       	testb	%al, %al
14233efa0: 74 1d                       	je	0x14233efbf <.text+0x233dfbf>
14233efa2: 8b 8e a0 00 00 00           	movl	0xa0(%rsi), %ecx
14233efa8: f3 0f 10 8e a4 00 00 00     	movss	0xa4(%rsi), %xmm1
14233efb0: e8 db 0a 60 fe              	callq	0x14093fa90 <.text+0x93ea90>
14233efb5: 34 01                       	xorb	$0x1, %al
14233efb7: 89 85 98 01 00 00           	movl	%eax, 0x198(%rbp)
14233efbd: eb 0a                       	jmp	0x14233efc9 <.text+0x233dfc9>
14233efbf: c7 85 98 01 00 00 00 00 00 00       	movl	$0x0, 0x198(%rbp)
14233efc9: 8b 8e b0 00 00 00           	movl	0xb0(%rsi), %ecx
14233efcf: f3 0f 10 8e b4 00 00 00     	movss	0xb4(%rsi), %xmm1
14233efd7: e8 b4 0a 60 fe              	callq	0x14093fa90 <.text+0x93ea90>
14233efdc: 84 c0                       	testb	%al, %al
14233efde: 74 1d                       	je	0x14233effd <.text+0x233dffd>
14233efe0: 8b 8e b8 00 00 00           	movl	0xb8(%rsi), %ecx
14233efe6: f3 0f 10 8e bc 00 00 00     	movss	0xbc(%rsi), %xmm1
14233efee: e8 9d 0a 60 fe              	callq	0x14093fa90 <.text+0x93ea90>
14233eff3: 34 01                       	xorb	$0x1, %al
14233eff5: 89 85 9c 01 00 00           	movl	%eax, 0x19c(%rbp)
14233effb: eb 0a                       	jmp	0x14233f007 <.text+0x233e007>
14233effd: c7 85 9c 01 00 00 00 00 00 00       	movl	$0x0, 0x19c(%rbp)
14233f007: 8b 8e b8 00 00 00           	movl	0xb8(%rsi), %ecx
14233f00d: f3 0f 10 8e bc 00 00 00     	movss	0xbc(%rsi), %xmm1
14233f015: e8 76 0a 60 fe              	callq	0x14093fa90 <.text+0x93ea90>
14233f01a: 84 c0                       	testb	%al, %al
14233f01c: 48 89 9d 68 01 00 00        	movq	%rbx, 0x168(%rbp)
14233f023: 74 1a                       	je	0x14233f03f <.text+0x233e03f>
14233f025: 8b 8e b0 00 00 00           	movl	0xb0(%rsi), %ecx
14233f02b: f3 0f 10 8e b4 00 00 00     	movss	0xb4(%rsi), %xmm1
14233f033: e8 58 0a 60 fe              	callq	0x14093fa90 <.text+0x93ea90>
14233f038: 89 c1                       	movl	%eax, %ecx
14233f03a: 80 f1 01                    	xorb	$0x1, %cl
14233f03d: eb 02                       	jmp	0x14233f041 <.text+0x233e041>
14233f03f: 31 c9                       	xorl	%ecx, %ecx
14233f041: 8b 85 9c 01 00 00           	movl	0x19c(%rbp), %eax
14233f047: 89 c3                       	movl	%eax, %ebx
14233f049: 89 8d b0 01 00 00           	movl	%ecx, 0x1b0(%rbp)
14233f04f: 08 cb                       	orb	%cl, %bl
14233f051: 8b 85 ac 01 00 00           	movl	0x1ac(%rbp), %eax
14233f057: 41 89 c4                    	movl	%eax, %r12d
14233f05a: 44 0a a5 98 01 00 00        	orb	0x198(%rbp), %r12b
14233f061: 44 08 e3                    	orb	%r12b, %bl
14233f064: 49 8b 4d 00                 	movq	(%r13), %rcx
14233f068: 83 79 10 01                 	cmpl	$0x1, 0x10(%rcx)
14233f06c: 75 45                       	jne	0x14233f0b3 <.text+0x233e0b3>
14233f06e: 48 8b 79 18                 	movq	0x18(%rcx), %rdi
14233f072: 8b 8e 90 00 00 00           	movl	0x90(%rsi), %ecx
14233f078: f3 0f 10 8e 94 00 00 00     	movss	0x94(%rsi), %xmm1
14233f080: e8 3b 0a 60 fe              	callq	0x14093fac0 <.text+0x93eac0>
14233f085: 84 c0                       	testb	%al, %al
14233f087: 74 7b                       	je	0x14233f104 <.text+0x233e104>
14233f089: 49 8b 45 00                 	movq	(%r13), %rax
14233f08d: 0f 10 40 10                 	movups	0x10(%rax), %xmm0
14233f091: 0f 29 85 f0 00 00 00        	movaps	%xmm0, 0xf0(%rbp)
14233f098: 41 8b 4d 1c                 	movl	0x1c(%r13), %ecx
14233f09c: 49 8b 55 10                 	movq	0x10(%r13), %rdx
14233f0a0: 89 0a                       	movl	%ecx, (%rdx)
14233f0a2: 66 0f 6f 85 f0 00 00 00     	movdqa	0xf0(%rbp), %xmm0
14233f0aa: f3 0f 7f 00                 	movdqu	%xmm0, (%rax)
14233f0ae: e9 d1 00 00 00              	jmp	0x14233f184 <.text+0x233e184>
14233f0b3: 84 db                       	testb	%bl, %bl
14233f0b5: 75 1f                       	jne	0x14233f0d6 <.text+0x233e0d6>
14233f0b7: 8b 8e 98 00 00 00           	movl	0x98(%rsi), %ecx
14233f0bd: f3 0f 10 8e 9c 00 00 00     	movss	0x9c(%rsi), %xmm1
14233f0c5: e8 c6 09 60 fe              	callq	0x14093fa90 <.text+0x93ea90>
14233f0ca: 84 c0                       	testb	%al, %al
14233f0cc: 0f 84 18 03 00 00           	je	0x14233f3ea <.text+0x233e3ea>
14233f0d2: 49 8b 4d 00                 	movq	(%r13), %rcx
14233f0d6: e8 75 a0 66 fe              	callq	0x1409a9150 <.text+0x9a8150>
14233f0db: 84 c0                       	testb	%al, %al
14233f0dd: 0f 84 bc 00 00 00           	je	0x14233f19f <.text+0x233e19f>
14233f0e3: 41 8b 45 1c                 	movl	0x1c(%r13), %eax
14233f0e7: 49 8b 4d 10                 	movq	0x10(%r13), %rcx
14233f0eb: 89 01                       	movl	%eax, (%rcx)
14233f0ed: 49 8b 45 00                 	movq	(%r13), %rax
14233f0f1: 0f 10 00                    	movups	(%rax), %xmm0
14233f0f4: 0f 29 45 70                 	movaps	%xmm0, 0x70(%rbp)
14233f0f8: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
14233f0ff: e9 ce 02 00 00              	jmp	0x14233f3d2 <.text+0x233e3d2>
14233f104: 49 8b 57 08                 	movq	0x8(%r15), %rdx
14233f108: 48 89 bd 70 01 00 00        	movq	%rdi, 0x170(%rbp)
14233f10f: 89 f8                       	movl	%edi, %eax
14233f111: 48 3b 42 10                 	cmpq	0x10(%rdx), %rax
14233f115: 73 6d                       	jae	0x14233f184 <.text+0x233e184>
14233f117: 48 8b 4a 08                 	movq	0x8(%rdx), %rcx
14233f11b: 48 8b bd 70 01 00 00        	movq	0x170(%rbp), %rdi
14233f122: 48 c1 ef 20                 	shrq	$0x20, %rdi
14233f126: 48 8d 04 80                 	leaq	(%rax,%rax,4), %rax
14233f12a: 39 3c 81                    	cmpl	%edi, (%rcx,%rax,4)
14233f12d: 75 55                       	jne	0x14233f184 <.text+0x233e184>
14233f12f: 48 8d 0c 81                 	leaq	(%rcx,%rax,4), %rcx
14233f133: 44 8b 41 04                 	movl	0x4(%rcx), %r8d
14233f137: b8 ff ff ff ff              	movl	$0xffffffff, %eax       # imm = 0xFFFFFFFF
14233f13c: 49 39 c0                    	cmpq	%rax, %r8
14233f13f: 74 43                       	je	0x14233f184 <.text+0x233e184>
14233f141: 49 8b 07                    	movq	(%r15), %rax
14233f144: 4c 39 40 38                 	cmpq	%r8, 0x38(%rax)
14233f148: 76 3a                       	jbe	0x14233f184 <.text+0x233e184>
14233f14a: 4c 8b 48 28                 	movq	0x28(%rax), %r9
14233f14e: 45 89 c2                    	movl	%r8d, %r10d
14233f151: 41 c1 ea 06                 	shrl	$0x6, %r10d
14233f155: 4f 8b 0c d1                 	movq	(%r9,%r10,8), %r9
14233f159: 4d 0f a3 c1                 	btq	%r8, %r9
14233f15d: 73 25                       	jae	0x14233f184 <.text+0x233e184>
14233f15f: 44 8b 51 0c                 	movl	0xc(%rcx), %r10d
14233f163: 4c 39 82 08 01 00 00        	cmpq	%r8, 0x108(%rdx)
14233f16a: 0f 86 bf 02 00 00           	jbe	0x14233f42f <.text+0x233e42f>
14233f170: 4f 8d 0c 80                 	leaq	(%r8,%r8,4), %r9
14233f174: 49 c1 e1 05                 	shlq	$0x5, %r9
14233f178: 4c 03 8a 00 01 00 00        	addq	0x100(%rdx), %r9
14233f17f: e9 ae 02 00 00              	jmp	0x14233f432 <.text+0x233e432>
14233f184: 41 8b 45 1c                 	movl	0x1c(%r13), %eax
14233f188: 49 8b 4d 10                 	movq	0x10(%r13), %rcx
14233f18c: 89 01                       	movl	%eax, (%rcx)
14233f18e: 49 8b 45 00                 	movq	(%r13), %rax
14233f192: 48 c7 40 10 00 00 00 00     	movq	$0x0, 0x10(%rax)
14233f19a: e9 4b 02 00 00              	jmp	0x14233f3ea <.text+0x233e3ea>
14233f19f: 49 8b 07                    	movq	(%r15), %rax
14233f1a2: 49 8b 4f 08                 	movq	0x8(%r15), %rcx
14233f1a6: 48 8b 90 f8 00 00 00        	movq	0xf8(%rax), %rdx
14233f1ad: 4c 8b 80 00 01 00 00        	movq	0x100(%rax), %r8
14233f1b4: 4e 8d 04 82                 	leaq	(%rdx,%r8,4), %r8
14233f1b8: 80 b8 08 01 00 00 00        	cmpb	$0x0, 0x108(%rax)
14233f1bf: 0f 84 c8 00 00 00           	je	0x14233f28d <.text+0x233e28d>
14233f1c5: bb 08 00 00 00              	movl	$0x8, %ebx
14233f1ca: 45 31 c9                    	xorl	%r9d, %r9d
14233f1cd: 45 31 d2                    	xorl	%r10d, %r10d
14233f1d0: 31 f6                       	xorl	%esi, %esi
14233f1d2: 45 31 f6                    	xorl	%r14d, %r14d
14233f1d5: 45 31 db                    	xorl	%r11d, %r11d
14233f1d8: 4d 39 de                    	cmpq	%r11, %r14
14233f1db: 75 72                       	jne	0x14233f24f <.text+0x233e24f>
14233f1dd: 0f 1f 00                    	nopl	(%rax)
14233f1e0: 4c 39 c2                    	cmpq	%r8, %rdx
14233f1e3: 0f 84 c5 01 00 00           	je	0x14233f3ae <.text+0x233e3ae>
14233f1e9: 44 8b 1a                    	movl	(%rdx), %r11d
14233f1ec: 48 83 c2 04                 	addq	$0x4, %rdx
14233f1f0: 48 8b b1 a8 01 00 00        	movq	0x1a8(%rcx), %rsi
14233f1f7: 4b 8d 3c db                 	leaq	(%r11,%r11,8), %rdi
14233f1fb: 4c 8b 5c fe 10              	movq	0x10(%rsi,%rdi,8), %r11
14233f200: 4d 85 db                    	testq	%r11, %r11
14233f203: 74 db                       	je	0x14233f1e0 <.text+0x233e1e0>
14233f205: 48 8d 3c fe                 	leaq	(%rsi,%rdi,8), %rdi
14233f209: 48 8b 77 40                 	movq	0x40(%rdi), %rsi
14233f20d: 48 3b b0 10 01 00 00        	cmpq	0x110(%rax), %rsi
14233f214: 48 8b 98 18 01 00 00        	movq	0x118(%rax), %rbx
14233f21b: 48 39 de                    	cmpq	%rbx, %rsi
14233f21e: 76 26                       	jbe	0x14233f246 <.text+0x233e246>
14233f220: 48 8b 77 38                 	movq	0x38(%rdi), %rsi
14233f224: 48 8b 34 de                 	movq	(%rsi,%rbx,8), %rsi
14233f228: 48 85 f6                    	testq	%rsi, %rsi
14233f22b: 74 19                       	je	0x14233f246 <.text+0x233e246>
14233f22d: 4c 8b 57 18                 	movq	0x18(%rdi), %r10
14233f231: 48 f7 d6                    	notq	%rsi
14233f234: 48 8d 34 76                 	leaq	(%rsi,%rsi,2), %rsi
14233f238: 48 c1 e6 04                 	shlq	$0x4, %rsi
14233f23c: 4d 8b 54 32 10              	movq	0x10(%r10,%rsi), %r10
14233f241: 40 b6 01                    	movb	$0x1, %sil
14233f244: eb 02                       	jmp	0x14233f248 <.text+0x233e248>
14233f246: 31 f6                       	xorl	%esi, %esi
14233f248: 48 8b 5f 08                 	movq	0x8(%rdi), %rbx
14233f24c: 45 31 f6                    	xorl	%r14d, %r14d
14233f24f: 4a 8b 3c f3                 	movq	(%rbx,%r14,8), %rdi
14233f253: 45 89 f7                    	movl	%r14d, %r15d
14233f256: 4f 8d 3c 7f                 	leaq	(%r15,%r15,2), %r15
14233f25a: 4d 01 d7                    	addq	%r10, %r15
14233f25d: 40 f6 c6 01                 	testb	$0x1, %sil
14233f261: 4d 0f 44 f9                 	cmoveq	%r9, %r15
14233f265: 49 89 fc                    	movq	%rdi, %r12
14233f268: 49 c1 ec 20                 	shrq	$0x20, %r12
14233f26c: 0f 84 3c 01 00 00           	je	0x14233f3ae <.text+0x233e3ae>
14233f272: 4d 85 ff                    	testq	%r15, %r15
14233f275: 0f 84 99 01 00 00           	je	0x14233f414 <.text+0x233e414>
14233f27b: 49 ff c6                    	incq	%r14
14233f27e: 41 80 3f 00                 	cmpb	$0x0, (%r15)
14233f282: 0f 84 50 ff ff ff           	je	0x14233f1d8 <.text+0x233e1d8>
14233f288: e9 87 01 00 00              	jmp	0x14233f414 <.text+0x233e414>
14233f28d: be 08 00 00 00              	movl	$0x8, %esi
14233f292: 45 31 d2                    	xorl	%r10d, %r10d
14233f295: 45 31 db                    	xorl	%r11d, %r11d
14233f298: 31 db                       	xorl	%ebx, %ebx
14233f29a: 45 31 c9                    	xorl	%r9d, %r9d
14233f29d: 4c 39 cb                    	cmpq	%r9, %rbx
14233f2a0: 0f 85 bf 00 00 00           	jne	0x14233f365 <.text+0x233e365>
14233f2a6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
14233f2b0: 4c 39 c2                    	cmpq	%r8, %rdx
14233f2b3: 0f 84 f5 00 00 00           	je	0x14233f3ae <.text+0x233e3ae>
14233f2b9: 44 8b 0a                    	movl	(%rdx), %r9d
14233f2bc: 48 83 c2 04                 	addq	$0x4, %rdx
14233f2c0: 48 8b b1 00 01 00 00        	movq	0x100(%rcx), %rsi
14233f2c7: 4f 8d 1c 89                 	leaq	(%r9,%r9,4), %r11
14233f2cb: 49 c1 e3 05                 	shlq	$0x5, %r11
14233f2cf: 4e 8b 4c 1e 58              	movq	0x58(%rsi,%r11), %r9
14233f2d4: 4d 85 c9                    	testq	%r9, %r9
14233f2d7: 74 d7                       	je	0x14233f2b0 <.text+0x233e2b0>
14233f2d9: 4c 01 de                    	addq	%r11, %rsi
14233f2dc: 48 8b b8 10 01 00 00        	movq	0x110(%rax), %rdi
14233f2e3: 4c 8b 9e 88 00 00 00        	movq	0x88(%rsi), %r11
14233f2ea: 49 39 fb                    	cmpq	%rdi, %r11
14233f2ed: 76 0c                       	jbe	0x14233f2fb <.text+0x233e2fb>
14233f2ef: 48 8b 9e 80 00 00 00        	movq	0x80(%rsi), %rbx
14233f2f6: 48 83 3c fb 00              	cmpq	$0x0, (%rbx,%rdi,8)
14233f2fb: 48 8b b8 18 01 00 00        	movq	0x118(%rax), %rdi
14233f302: 49 39 fb                    	cmpq	%rdi, %r11
14233f305: 76 50                       	jbe	0x14233f357 <.text+0x233e357>
14233f307: 4c 8b 9e 80 00 00 00        	movq	0x80(%rsi), %r11
14233f30e: 49 83 3c fb 00              	cmpq	$0x0, (%r11,%rdi,8)
14233f313: 74 42                       	je	0x14233f357 <.text+0x233e357>
14233f315: 44 8b 96 94 00 00 00        	movl	0x94(%rsi), %r10d
14233f31c: 4f 8d 14 d2                 	leaq	(%r10,%r10,8), %r10
14233f320: 49 c1 e2 03                 	shlq	$0x3, %r10
14233f324: 4c 03 91 a8 01 00 00        	addq	0x1a8(%rcx), %r10
14233f32b: 41 b3 01                    	movb	$0x1, %r11b
14233f32e: 49 39 7a 40                 	cmpq	%rdi, 0x40(%r10)
14233f332: 76 28                       	jbe	0x14233f35c <.text+0x233e35c>
14233f334: 49 8b 5a 38                 	movq	0x38(%r10), %rbx
14233f338: 48 8b 3c fb                 	movq	(%rbx,%rdi,8), %rdi
14233f33c: 48 85 ff                    	testq	%rdi, %rdi
14233f33f: 74 1b                       	je	0x14233f35c <.text+0x233e35c>
14233f341: 4d 8b 52 18                 	movq	0x18(%r10), %r10
14233f345: 48 f7 d7                    	notq	%rdi
14233f348: 48 8d 3c 7f                 	leaq	(%rdi,%rdi,2), %rdi
14233f34c: 48 c1 e7 04                 	shlq	$0x4, %rdi
14233f350: 4d 8b 54 3a 10              	movq	0x10(%r10,%rdi), %r10
14233f355: eb 08                       	jmp	0x14233f35f <.text+0x233e35f>
14233f357: 45 31 db                    	xorl	%r11d, %r11d
14233f35a: eb 03                       	jmp	0x14233f35f <.text+0x233e35f>
14233f35c: 45 31 d2                    	xorl	%r10d, %r10d
14233f35f: 48 8b 76 50                 	movq	0x50(%rsi), %rsi
14233f363: 31 db                       	xorl	%ebx, %ebx
14233f365: 49 89 de                    	movq	%rbx, %r14
14233f368: 49 c1 e6 04                 	shlq	$0x4, %r14
14233f36c: 4a 8b 3c 36                 	movq	(%rsi,%r14), %rdi
14233f370: 41 f6 c3 01                 	testb	$0x1, %r11b
14233f374: 74 2f                       	je	0x14233f3a5 <.text+0x233e3a5>
14233f376: 49 89 ff                    	movq	%rdi, %r15
14233f379: 49 c1 ef 20                 	shrq	$0x20, %r15
14233f37d: 74 2f                       	je	0x14233f3ae <.text+0x233e3ae>
14233f37f: 4d 85 d2                    	testq	%r10, %r10
14233f382: 0f 84 8c 00 00 00           	je	0x14233f414 <.text+0x233e414>
14233f388: 49 01 f6                    	addq	%rsi, %r14
14233f38b: 45 8b 76 08                 	movl	0x8(%r14), %r14d
14233f38f: 4f 8d 34 76                 	leaq	(%r14,%r14,2), %r14
14233f393: 4d 01 d6                    	addq	%r10, %r14
14233f396: 48 ff c3                    	incq	%rbx
14233f399: 41 80 3e 00                 	cmpb	$0x0, (%r14)
14233f39d: 0f 84 fa fe ff ff           	je	0x14233f29d <.text+0x233e29d>
14233f3a3: eb 6f                       	jmp	0x14233f414 <.text+0x233e414>
14233f3a5: 48 89 f8                    	movq	%rdi, %rax
14233f3a8: 48 c1 e8 20                 	shrq	$0x20, %rax
14233f3ac: 75 66                       	jne	0x14233f414 <.text+0x233e414>
14233f3ae: 48 c7 85 20 01 00 00 03 00 00 00    	movq	$0x3, 0x120(%rbp)
14233f3b9: 48 8d 85 f0 00 00 00        	leaq	0xf0(%rbp), %rax
14233f3c0: 48 c7 85 f0 00 00 00 00 00 00 00    	movq	$0x0, 0xf0(%rbp)
14233f3cb: 0f 10 00                    	movups	(%rax), %xmm0
14233f3ce: 0f 29 45 70                 	movaps	%xmm0, 0x70(%rbp)
14233f3d2: 41 8b 45 1c                 	movl	0x1c(%r13), %eax
14233f3d6: 49 8b 4d 10                 	movq	0x10(%r13), %rcx
14233f3da: 89 01                       	movl	%eax, (%rcx)
14233f3dc: 49 8b 45 00                 	movq	(%r13), %rax
14233f3e0: 66 0f 6f 45 70              	movdqa	0x70(%rbp), %xmm0
14233f3e5: f3 0f 7f 40 10              	movdqu	%xmm0, 0x10(%rax)
14233f3ea: 0f 28 b5 c0 01 00 00        	movaps	0x1c0(%rbp), %xmm6
14233f3f1: 0f 28 bd d0 01 00 00        	movaps	0x1d0(%rbp), %xmm7
14233f3f8: 44 0f 28 85 e0 01 00 00     	movaps	0x1e0(%rbp), %xmm8
14233f400: 48 81 c4 78 02 00 00        	addq	$0x278, %rsp            # imm = 0x278
14233f407: 5b                          	popq	%rbx
14233f408: 5f                          	popq	%rdi
14233f409: 5e                          	popq	%rsi
14233f40a: 41 5c                       	popq	%r12
14233f40c: 41 5d                       	popq	%r13
14233f40e: 41 5e                       	popq	%r14
14233f410: 41 5f                       	popq	%r15
14233f412: 5d                          	popq	%rbp
14233f413: c3                          	retq
14233f414: 48 c7 85 20 01 00 00 01 00 00 00    	movq	$0x1, 0x120(%rbp)
14233f41f: 48 89 bd 28 01 00 00        	movq	%rdi, 0x128(%rbp)
14233f426: 48 8d 85 20 01 00 00        	leaq	0x120(%rbp), %rax
14233f42d: eb 91                       	jmp	0x14233f3c0 <.text+0x233e3c0>
14233f42f: 45 31 c9                    	xorl	%r9d, %r9d
14233f432: 4f 8d 04 d2                 	leaq	(%r10,%r10,8), %r8
14233f436: 49 c1 e0 03                 	shlq	$0x3, %r8
14233f43a: 4c 03 82 a8 01 00 00        	addq	0x1a8(%rdx), %r8
14233f441: 4c 8b 90 10 01 00 00        	movq	0x110(%rax), %r10
14233f448: 4d 8b 99 88 00 00 00        	movq	0x88(%r9), %r11
14233f44f: 4d 39 d3                    	cmpq	%r10, %r11
14233f452: 44 88 a5 aa 01 00 00        	movb	%r12b, 0x1aa(%rbp)
14233f459: 76 39                       	jbe	0x14233f494 <.text+0x233e494>
14233f45b: 49 8b 91 80 00 00 00        	movq	0x80(%r9), %rdx
14233f462: 4a 83 3c d2 00              	cmpq	$0x0, (%rdx,%r10,8)
14233f467: 74 2b                       	je	0x14233f494 <.text+0x233e494>
14233f469: b2 01                       	movb	$0x1, %dl
14233f46b: 4d 39 50 40                 	cmpq	%r10, 0x40(%r8)
14233f46f: 76 25                       	jbe	0x14233f496 <.text+0x233e496>
14233f471: 4d 8b 70 38                 	movq	0x38(%r8), %r14
14233f475: 4f 8b 14 d6                 	movq	(%r14,%r10,8), %r10
14233f479: 4d 85 d2                    	testq	%r10, %r10
14233f47c: 74 18                       	je	0x14233f496 <.text+0x233e496>
14233f47e: 4d 8b 70 18                 	movq	0x18(%r8), %r14
14233f482: 49 f7 d2                    	notq	%r10
14233f485: 4f 8d 14 52                 	leaq	(%r10,%r10,2), %r10
14233f489: 49 c1 e2 04                 	shlq	$0x4, %r10
14233f48d: 4f 8b 54 16 10              	movq	0x10(%r14,%r10), %r10
14233f492: eb 05                       	jmp	0x14233f499 <.text+0x233e499>
14233f494: 31 d2                       	xorl	%edx, %edx
14233f496: 45 31 d2                    	xorl	%r10d, %r10d
14233f499: 44 8b 61 10                 	movl	0x10(%rcx), %r12d
14233f49d: 48 8b 88 18 01 00 00        	movq	0x118(%rax), %rcx
14233f4a4: 49 39 cb                    	cmpq	%rcx, %r11
14233f4a7: 76 0c                       	jbe	0x14233f4b5 <.text+0x233e4b5>
14233f4a9: 4d 8b 89 80 00 00 00        	movq	0x80(%r9), %r9
14233f4b0: 49 83 3c c9 00              	cmpq	$0x0, (%r9,%rcx,8)
14233f4b5: 48 8b 80 20 01 00 00        	movq	0x120(%rax), %rax
14233f4bc: 49 39 40 40                 	cmpq	%rax, 0x40(%r8)
14233f4c0: 76 23                       	jbe	0x14233f4e5 <.text+0x233e4e5>
14233f4c2: 49 8b 48 38                 	movq	0x38(%r8), %rcx
14233f4c6: 48 8b 04 c1                 	movq	(%rcx,%rax,8), %rax
14233f4ca: 48 85 c0                    	testq	%rax, %rax
14233f4cd: 74 16                       	je	0x14233f4e5 <.text+0x233e4e5>
14233f4cf: 49 8b 48 18                 	movq	0x18(%r8), %rcx
14233f4d3: 48 f7 d0                    	notq	%rax
14233f4d6: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14233f4da: 48 c1 e0 04                 	shlq	$0x4, %rax
14233f4de: 4c 8b 74 01 10              	movq	0x10(%rcx,%rax), %r14
14233f4e3: eb 03                       	jmp	0x14233f4e8 <.text+0x233e4e8>
14233f4e5: 45 31 f6                    	xorl	%r14d, %r14d
14233f4e8: 4c 89 e0                    	movq	%r12, %rax
14233f4eb: 48 c1 e0 04                 	shlq	$0x4, %rax
14233f4ef: 49 01 c2                    	addq	%rax, %r10
14233f4f2: 31 c0                       	xorl	%eax, %eax
14233f4f4: 84 d2                       	testb	%dl, %dl
14233f4f6: 49 0f 45 c2                 	cmovneq	%r10, %rax
14233f4fa: 48 89 85 e8 00 00 00        	movq	%rax, 0xe8(%rbp)
14233f501: 8b 8e 98 00 00 00           	movl	0x98(%rsi), %ecx
14233f507: f3 0f 10 8e 9c 00 00 00     	movss	0x9c(%rsi), %xmm1
14233f50f: e8 7c 05 60 fe              	callq	0x14093fa90 <.text+0x93ea90>
14233f514: 84 c0                       	testb	%al, %al
14233f516: 74 1f                       	je	0x14233f537 <.text+0x233e537>
14233f518: 48 8b 8d 68 02 00 00        	movq	0x268(%rbp), %rcx
14233f51f: 4c 8d 05 5a e9 d0 00        	leaq	0xd0e95a(%rip), %r8     # 0x14304de80
14233f526: 48 8b 95 70 01 00 00        	movq	0x170(%rbp), %rdx
14233f52d: e8 8e 05 c9 fe              	callq	0x140fcfac0 <.text+0xfceac0>
14233f532: e9 52 fb ff ff              	jmp	0x14233f089 <.text+0x233e089>
14233f537: 49 c1 e4 06                 	shlq	$0x6, %r12
14233f53b: 4d 01 e6                    	addq	%r12, %r14
14233f53e: b9 08 00 00 00              	movl	$0x8, %ecx
14233f543: 4c 89 f0                    	movq	%r14, %rax
14233f546: 80 bd ac 01 00 00 00        	cmpb	$0x0, 0x1ac(%rbp)
14233f54d: 8b 95 b0 01 00 00           	movl	0x1b0(%rbp), %edx
14233f553: 75 39                       	jne	0x14233f58e <.text+0x233e58e>
14233f555: 80 bd 98 01 00 00 00        	cmpb	$0x0, 0x198(%rbp)
14233f55c: 74 0b                       	je	0x14233f569 <.text+0x233e569>
14233f55e: 49 8d 46 10                 	leaq	0x10(%r14), %rax
14233f562: b9 18 00 00 00              	movl	$0x18, %ecx
14233f567: eb 25                       	jmp	0x14233f58e <.text+0x233e58e>
14233f569: 80 bd 9c 01 00 00 00        	cmpb	$0x0, 0x19c(%rbp)
14233f570: 74 0b                       	je	0x14233f57d <.text+0x233e57d>
14233f572: 49 8d 46 20                 	leaq	0x20(%r14), %rax
14233f576: b9 28 00 00 00              	movl	$0x28, %ecx
14233f57b: eb 11                       	jmp	0x14233f58e <.text+0x233e58e>
14233f57d: 84 d2                       	testb	%dl, %dl
14233f57f: 0f 84 65 fe ff ff           	je	0x14233f3ea <.text+0x233e3ea>
14233f585: 49 8d 46 30                 	leaq	0x30(%r14), %rax
14233f589: b9 38 00 00 00              	movl	$0x38, %ecx
14233f58e: 49 8b 14 0e                 	movq	(%r14,%rcx), %rdx
14233f592: 48 8b 08                    	movq	(%rax), %rcx
14233f595: 48 85 c9                    	testq	%rcx, %rcx
14233f598: 0f 94 c0                    	sete	%al
14233f59b: 84 c3                       	testb	%al, %bl
14233f59d: 0f 84 db 00 00 00           	je	0x14233f67e <.text+0x233e67e>
14233f5a3: 48 83 bd e8 00 00 00 00     	cmpq	$0x0, 0xe8(%rbp)
14233f5ab: 0f 84 b6 0a 00 00           	je	0x142340067 <.text+0x233f067>
14233f5b1: 48 89 8d a8 00 00 00        	movq	%rcx, 0xa8(%rbp)
14233f5b8: 48 89 95 a0 00 00 00        	movq	%rdx, 0xa0(%rbp)
14233f5bf: 4c 89 bd d0 00 00 00        	movq	%r15, 0xd0(%rbp)
14233f5c6: 48 8d 0d 23 e1 d0 00        	leaq	0xd0e123(%rip), %rcx    # 0x14304d6f0
14233f5cd: e8 0e 06 c9 fe              	callq	0x140fcfbe0 <.text+0xfcebe0>
14233f5d2: 0f 10 35 37 e1 d0 00        	movups	0xd0e137(%rip), %xmm6   # 0x14304d710
14233f5d9: 0f 29 75 40                 	movaps	%xmm6, 0x40(%rbp)
14233f5dd: 0f 10 3d 3c e1 d0 00        	movups	0xd0e13c(%rip), %xmm7   # 0x14304d720
14233f5e4: 0f 29 7d 50                 	movaps	%xmm7, 0x50(%rbp)
14233f5e8: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
14233f5ec: 48 89 55 68                 	movq	%rdx, 0x68(%rbp)
14233f5f0: 48 c7 85 20 01 00 00 00 00 00 00    	movq	$0x0, 0x120(%rbp)
14233f5fb: 48 c7 85 28 01 00 00 04 00 00 00    	movq	$0x4, 0x128(%rbp)
14233f606: 48 c7 85 30 01 00 00 00 00 00 00    	movq	$0x0, 0x130(%rbp)
14233f611: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
14233f618: 48 8b 00                    	movq	(%rax), %rax
14233f61b: 4c 8b b0 a0 00 00 00        	movq	0xa0(%rax), %r14
14233f622: 48 8b 80 a8 00 00 00        	movq	0xa8(%rax), %rax
14233f629: 48 8d 04 80                 	leaq	(%rax,%rax,4), %rax
14233f62d: 4d 8d 3c c6                 	leaq	(%r14,%rax,8), %r15
14233f631: 31 db                       	xorl	%ebx, %ebx
14233f633: 48 8d 35 2e e8 d0 00        	leaq	0xd0e82e(%rip), %rsi    # 0x14304de68
14233f63a: 48 8d bd 20 01 00 00        	leaq	0x120(%rbp), %rdi
14233f641: eb 1a                       	jmp	0x14233f65d <.text+0x233e65d>
14233f643: 49 83 c6 28                 	addq	$0x28, %r14
14233f647: 48 8b 85 28 01 00 00        	movq	0x128(%rbp), %rax
14233f64e: 44 0f 13 04 d8              	movlps	%xmm8, (%rax,%rbx,8)
14233f653: 48 ff c3                    	incq	%rbx
14233f656: 48 89 9d 30 01 00 00        	movq	%rbx, 0x130(%rbp)
14233f65d: 4d 39 fe                    	cmpq	%r15, %r14
14233f660: 74 2c                       	je	0x14233f68e <.text+0x233e68e>
14233f662: f2 45 0f 10 46 10           	movsd	0x10(%r14), %xmm8
14233f668: 48 3b 9d 20 01 00 00        	cmpq	0x120(%rbp), %rbx
14233f66f: 75 d2                       	jne	0x14233f643 <.text+0x233e643>
14233f671: 48 89 f9                    	movq	%rdi, %rcx
14233f674: 48 89 f2                    	movq	%rsi, %rdx
14233f677: e8 14 58 40 fe              	callq	0x140744e90 <.text+0x743e90>
14233f67c: eb c5                       	jmp	0x14233f643 <.text+0x233e643>
14233f67e: 48 8d 05 53 e8 d0 00        	leaq	0xd0e853(%rip), %rax    # 0x14304ded8
14233f685: 48 63 0c 88                 	movslq	(%rax,%rcx,4), %rcx
14233f689: 48 01 c1                    	addq	%rax, %rcx
14233f68c: ff e1                       	jmpq	*%rcx
14233f68e: 48 8d bd f0 00 00 00        	leaq	0xf0(%rbp), %rdi
14233f695: 8b b5 b0 01 00 00           	movl	0x1b0(%rbp), %esi
14233f69b: 48 85 db                    	testq	%rbx, %rbx
14233f69e: 75 1b                       	jne	0x14233f6bb <.text+0x233e6bb>
14233f6a0: e9 2b 01 00 00              	jmp	0x14233f7d0 <.text+0x233e7d0>
14233f6a5: 48 8b 9d 30 01 00 00        	movq	0x130(%rbp), %rbx
14233f6ac: 8b b5 b0 01 00 00           	movl	0x1b0(%rbp), %esi
14233f6b2: 48 85 db                    	testq	%rbx, %rbx
14233f6b5: 0f 84 15 01 00 00           	je	0x14233f7d0 <.text+0x233e7d0>
14233f6bb: 48 8d 43 ff                 	leaq	-0x1(%rbx), %rax
14233f6bf: 48 89 85 30 01 00 00        	movq	%rax, 0x130(%rbp)
14233f6c6: 48 8b 85 28 01 00 00        	movq	0x128(%rbp), %rax
14233f6cd: 8b 74 d8 f8                 	movl	-0x8(%rax,%rbx,8), %esi
14233f6d1: 8b 5c d8 fc                 	movl	-0x4(%rax,%rbx,8), %ebx
14233f6d5: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
14233f6dc: 48 8b 10                    	movq	(%rax), %rdx
14233f6df: 48 89 f9                    	movq	%rdi, %rcx
14233f6e2: 41 89 f0                    	movl	%esi, %r8d
14233f6e5: 41 89 d9                    	movl	%ebx, %r9d
14233f6e8: e8 63 82 ca fe              	callq	0x140fe7950 <.text+0xfe6950>
14233f6ed: 83 bd f0 00 00 00 04        	cmpl	$0x4, 0xf0(%rbp)
14233f6f4: 75 af                       	jne	0x14233f6a5 <.text+0x233e6a5>
14233f6f6: 48 8b 85 f8 00 00 00        	movq	0xf8(%rbp), %rax
14233f6fd: 48 89 85 a0 01 00 00        	movq	%rax, 0x1a0(%rbp)
14233f704: 4c 8b ad 00 01 00 00        	movq	0x100(%rbp), %r13
14233f70b: 4c 8b bd 08 01 00 00        	movq	0x108(%rbp), %r15
14233f712: 49 c1 e7 03                 	shlq	$0x3, %r15
14233f716: 4c 89 ad 88 01 00 00        	movq	%r13, 0x188(%rbp)
14233f71d: eb 39                       	jmp	0x14233f758 <.text+0x233e758>
14233f71f: 90                          	nop
14233f720: 49 83 c5 08                 	addq	$0x8, %r13
14233f724: 48 8b 85 28 01 00 00        	movq	0x128(%rbp), %rax
14233f72b: 44 89 34 f8                 	movl	%r14d, (%rax,%rdi,8)
14233f72f: 44 89 64 f8 04              	movl	%r12d, 0x4(%rax,%rdi,8)
14233f734: 48 ff c7                    	incq	%rdi
14233f737: 48 89 bd 30 01 00 00        	movq	%rdi, 0x130(%rbp)
14233f73e: 49 83 c7 f8                 	addq	$-0x8, %r15
14233f742: 89 5c 24 20                 	movl	%ebx, 0x20(%rsp)
14233f746: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
14233f74a: 44 89 f2                    	movl	%r14d, %edx
14233f74d: 45 89 e0                    	movl	%r12d, %r8d
14233f750: 41 89 f1                    	movl	%esi, %r9d
14233f753: e8 b8 2f c7 fe              	callq	0x140fb2710 <.text+0xfb1710>
14233f758: 4d 85 ff                    	testq	%r15, %r15
14233f75b: 74 2d                       	je	0x14233f78a <.text+0x233e78a>
14233f75d: 45 8b 75 00                 	movl	(%r13), %r14d
14233f761: 45 8b 65 04                 	movl	0x4(%r13), %r12d
14233f765: 48 8b bd 30 01 00 00        	movq	0x130(%rbp), %rdi
14233f76c: 48 3b bd 20 01 00 00        	cmpq	0x120(%rbp), %rdi
14233f773: 75 ab                       	jne	0x14233f720 <.text+0x233e720>
14233f775: 48 8d 8d 20 01 00 00        	leaq	0x120(%rbp), %rcx
14233f77c: 48 8d 15 6d e6 d0 00        	leaq	0xd0e66d(%rip), %rdx    # 0x14304ddf0
14233f783: e8 08 57 40 fe              	callq	0x140744e90 <.text+0x743e90>
14233f788: eb 96                       	jmp	0x14233f720 <.text+0x233e720>
14233f78a: 48 8b 95 a0 01 00 00        	movq	0x1a0(%rbp), %rdx
14233f791: 48 85 d2                    	testq	%rdx, %rdx
14233f794: 74 16                       	je	0x14233f7ac <.text+0x233e7ac>
14233f796: 48 c1 e2 03                 	shlq	$0x3, %rdx
14233f79a: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14233f7a0: 48 8b 8d 88 01 00 00        	movq	0x188(%rbp), %rcx
14233f7a7: e8 74 44 2d fe              	callq	0x140613c20 <.text+0x612c20>
14233f7ac: 48 8b 9d 30 01 00 00        	movq	0x130(%rbp), %rbx
14233f7b3: 4c 8b ad 60 02 00 00        	movq	0x260(%rbp), %r13
14233f7ba: 8b b5 b0 01 00 00           	movl	0x1b0(%rbp), %esi
14233f7c0: 48 8d bd f0 00 00 00        	leaq	0xf0(%rbp), %rdi
14233f7c7: 48 85 db                    	testq	%rbx, %rbx
14233f7ca: 0f 85 eb fe ff ff           	jne	0x14233f6bb <.text+0x233e6bb>
14233f7d0: 48 8b 95 20 01 00 00        	movq	0x120(%rbp), %rdx
14233f7d7: 48 85 d2                    	testq	%rdx, %rdx
14233f7da: 74 16                       	je	0x14233f7f2 <.text+0x233e7f2>
14233f7dc: 48 8b 8d 28 01 00 00        	movq	0x128(%rbp), %rcx
14233f7e3: 48 c1 e2 03                 	shlq	$0x3, %rdx
14233f7e7: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14233f7ed: e8 2e 44 2d fe              	callq	0x140613c20 <.text+0x612c20>
14233f7f2: 0f 28 45 40                 	movaps	0x40(%rbp), %xmm0
14233f7f6: 0f 28 4d 50                 	movaps	0x50(%rbp), %xmm1
14233f7fa: 0f 28 55 60                 	movaps	0x60(%rbp), %xmm2
14233f7fe: 0f 29 55 d0                 	movaps	%xmm2, -0x30(%rbp)
14233f802: 0f b6 85 ac 01 00 00        	movzbl	0x1ac(%rbp), %eax
14233f809: 0f 57 d2                    	xorps	%xmm2, %xmm2
14233f80c: f3 0f 2a d0                 	cvtsi2ss	%eax, %xmm2
14233f810: 0f 29 4d c0                 	movaps	%xmm1, -0x40(%rbp)
14233f814: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
14233f818: 0f 57 c0                    	xorps	%xmm0, %xmm0
14233f81b: f3 0f 5c c2                 	subss	%xmm2, %xmm0
14233f81f: 0f b6 85 98 01 00 00        	movzbl	0x198(%rbp), %eax
14233f826: 0f 57 c9                    	xorps	%xmm1, %xmm1
14233f829: f3 0f 2a c8                 	cvtsi2ss	%eax, %xmm1
14233f82d: 0f b6 85 9c 01 00 00        	movzbl	0x19c(%rbp), %eax
14233f834: 0f 57 d2                    	xorps	%xmm2, %xmm2
14233f837: f3 0f 2a d0                 	cvtsi2ss	%eax, %xmm2
14233f83b: 40 0f b6 c6                 	movzbl	%sil, %eax
14233f83f: f3 0f 2a d8                 	cvtsi2ss	%eax, %xmm3
14233f843: f3 0f 58 c8                 	addss	%xmm0, %xmm1
14233f847: f3 0f 5c d3                 	subss	%xmm3, %xmm2
14233f84b: f3 0f 11 8d e0 00 00 00     	movss	%xmm1, 0xe0(%rbp)
14233f853: f3 0f 11 95 e4 00 00 00     	movss	%xmm2, 0xe4(%rbp)
14233f85b: 48 8d 0d 8e de d0 00        	leaq	0xd0de8e(%rip), %rcx    # 0x14304d6f0
14233f862: e8 79 03 c9 fe              	callq	0x140fcfbe0 <.text+0xfcebe0>
14233f867: 0f 29 7d 20                 	movaps	%xmm7, 0x20(%rbp)
14233f86b: 0f 29 75 10                 	movaps	%xmm6, 0x10(%rbp)
14233f86f: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
14233f873: 48 89 55 38                 	movq	%rdx, 0x38(%rbp)
14233f877: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
14233f87e: 48 8b 08                    	movq	(%rax), %rcx
14233f881: 4c 8b 78 08                 	movq	0x8(%rax), %r15
14233f885: 48 8b b1 f8 00 00 00        	movq	0xf8(%rcx), %rsi
14233f88c: 48 8b 81 00 01 00 00        	movq	0x100(%rcx), %rax
14233f893: 48 8d 3c 86                 	leaq	(%rsi,%rax,4), %rdi
14233f897: 48 89 8d a0 01 00 00        	movq	%rcx, 0x1a0(%rbp)
14233f89e: 0f b6 81 08 01 00 00        	movzbl	0x108(%rcx), %eax
14233f8a5: 88 85 80 01 00 00           	movb	%al, 0x180(%rbp)
14233f8ab: b8 08 00 00 00              	movl	$0x8, %eax
14233f8b0: 48 89 85 78 01 00 00        	movq	%rax, 0x178(%rbp)
14233f8b7: 31 db                       	xorl	%ebx, %ebx
14233f8b9: 48 89 85 88 01 00 00        	movq	%rax, 0x188(%rbp)
14233f8c0: 45 31 f6                    	xorl	%r14d, %r14d
14233f8c3: 45 31 e4                    	xorl	%r12d, %r12d
14233f8c6: eb 42                       	jmp	0x14233f90a <.text+0x233e90a>
14233f8c8: 31 db                       	xorl	%ebx, %ebx
14233f8ca: 48 8b 40 08                 	movq	0x8(%rax), %rax
14233f8ce: 48 89 85 88 01 00 00        	movq	%rax, 0x188(%rbp)
14233f8d5: 45 31 e4                    	xorl	%r12d, %r12d
14233f8d8: 48 8b 85 88 01 00 00        	movq	0x188(%rbp), %rax
14233f8df: 4a 8d 04 e0                 	leaq	(%rax,%r12,8), %rax
14233f8e3: 44 89 e1                    	movl	%r12d, %ecx
14233f8e6: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
14233f8ea: 48 8d 0c 8b                 	leaq	(%rbx,%rcx,4), %rcx
14233f8ee: 4c 8b 08                    	movq	(%rax), %r9
14233f8f1: 49 ff c4                    	incq	%r12
14233f8f4: 8b 11                       	movl	(%rcx), %edx
14233f8f6: 44 8b 41 04                 	movl	0x4(%rcx), %r8d
14233f8fa: c6 85 b6 01 00 00 01        	movb	$0x1, 0x1b6(%rbp)
14233f901: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
14233f905: e8 b6 2a c7 fe              	callq	0x140fb23c0 <.text+0xfb13c0>
14233f90a: 80 bd 80 01 00 00 00        	cmpb	$0x0, 0x180(%rbp)
14233f911: 74 6f                       	je	0x14233f982 <.text+0x233e982>
14233f913: 4d 39 f4                    	cmpq	%r14, %r12
14233f916: 75 c0                       	jne	0x14233f8d8 <.text+0x233e8d8>
14233f918: 48 39 fe                    	cmpq	%rdi, %rsi
14233f91b: 0f 84 0e 01 00 00           	je	0x14233fa2f <.text+0x233ea2f>
14233f921: 8b 0e                       	movl	(%rsi), %ecx
14233f923: 48 83 c6 04                 	addq	$0x4, %rsi
14233f927: 49 8b 87 a8 01 00 00        	movq	0x1a8(%r15), %rax
14233f92e: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
14233f932: 4c 8b 74 c8 10              	movq	0x10(%rax,%rcx,8), %r14
14233f937: 4d 85 f6                    	testq	%r14, %r14
14233f93a: 74 dc                       	je	0x14233f918 <.text+0x233e918>
14233f93c: 48 8d 04 c8                 	leaq	(%rax,%rcx,8), %rax
14233f940: 48 8b 8d a0 01 00 00        	movq	0x1a0(%rbp), %rcx
14233f947: 48 8b 89 10 01 00 00        	movq	0x110(%rcx), %rcx
14233f94e: 48 39 48 40                 	cmpq	%rcx, 0x40(%rax)
14233f952: 0f 86 70 ff ff ff           	jbe	0x14233f8c8 <.text+0x233e8c8>
14233f958: 48 8b 50 38                 	movq	0x38(%rax), %rdx
14233f95c: 48 8b 0c ca                 	movq	(%rdx,%rcx,8), %rcx
14233f960: 48 85 c9                    	testq	%rcx, %rcx
14233f963: 0f 84 5f ff ff ff           	je	0x14233f8c8 <.text+0x233e8c8>
14233f969: 48 8b 50 18                 	movq	0x18(%rax), %rdx
14233f96d: 48 f7 d1                    	notq	%rcx
14233f970: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
14233f974: 48 c1 e1 04                 	shlq	$0x4, %rcx
14233f978: 48 8b 5c 0a 10              	movq	0x10(%rdx,%rcx), %rbx
14233f97d: e9 48 ff ff ff              	jmp	0x14233f8ca <.text+0x233e8ca>
14233f982: 4d 39 f4                    	cmpq	%r14, %r12
14233f985: 0f 85 89 00 00 00           	jne	0x14233fa14 <.text+0x233ea14>
14233f98b: 48 39 fe                    	cmpq	%rdi, %rsi
14233f98e: 0f 84 9b 00 00 00           	je	0x14233fa2f <.text+0x233ea2f>
14233f994: 8b 0e                       	movl	(%rsi), %ecx
14233f996: 48 83 c6 04                 	addq	$0x4, %rsi
14233f99a: 49 8b 87 00 01 00 00        	movq	0x100(%r15), %rax
14233f9a1: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
14233f9a5: 48 c1 e1 05                 	shlq	$0x5, %rcx
14233f9a9: 4c 8b 74 08 58              	movq	0x58(%rax,%rcx), %r14
14233f9ae: 4d 85 f6                    	testq	%r14, %r14
14233f9b1: 74 d8                       	je	0x14233f98b <.text+0x233e98b>
14233f9b3: 48 01 c8                    	addq	%rcx, %rax
14233f9b6: 8b 90 94 00 00 00           	movl	0x94(%rax), %edx
14233f9bc: 49 8b 8f a8 01 00 00        	movq	0x1a8(%r15), %rcx
14233f9c3: 48 8d 14 d2                 	leaq	(%rdx,%rdx,8), %rdx
14233f9c7: 4c 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %r8
14233f9ce: 4d 8b 80 10 01 00 00        	movq	0x110(%r8), %r8
14233f9d5: 4c 39 44 d1 40              	cmpq	%r8, 0x40(%rcx,%rdx,8)
14233f9da: 76 28                       	jbe	0x14233fa04 <.text+0x233ea04>
14233f9dc: 4c 8b 4c d1 38              	movq	0x38(%rcx,%rdx,8), %r9
14233f9e1: 4f 8b 04 c1                 	movq	(%r9,%r8,8), %r8
14233f9e5: 4d 85 c0                    	testq	%r8, %r8
14233f9e8: 74 1a                       	je	0x14233fa04 <.text+0x233ea04>
14233f9ea: 48 8d 0c d1                 	leaq	(%rcx,%rdx,8), %rcx
14233f9ee: 48 8b 49 18                 	movq	0x18(%rcx), %rcx
14233f9f2: 49 f7 d0                    	notq	%r8
14233f9f5: 4b 8d 14 40                 	leaq	(%r8,%r8,2), %rdx
14233f9f9: 48 c1 e2 04                 	shlq	$0x4, %rdx
14233f9fd: 48 8b 5c 11 10              	movq	0x10(%rcx,%rdx), %rbx
14233fa02: eb 02                       	jmp	0x14233fa06 <.text+0x233ea06>
14233fa04: 31 db                       	xorl	%ebx, %ebx
14233fa06: 48 8b 40 50                 	movq	0x50(%rax), %rax
14233fa0a: 48 89 85 78 01 00 00        	movq	%rax, 0x178(%rbp)
14233fa11: 45 31 e4                    	xorl	%r12d, %r12d
14233fa14: 4c 89 e1                    	movq	%r12, %rcx
14233fa17: 48 c1 e1 04                 	shlq	$0x4, %rcx
14233fa1b: 48 8b 95 78 01 00 00        	movq	0x178(%rbp), %rdx
14233fa22: 48 8d 04 0a                 	leaq	(%rdx,%rcx), %rax
14233fa26: 8b 4c 0a 08                 	movl	0x8(%rdx,%rcx), %ecx
14233fa2a: e9 b7 fe ff ff              	jmp	0x14233f8e6 <.text+0x233e8e6>
14233fa2f: 66 0f 6f 45 10              	movdqa	0x10(%rbp), %xmm0
14233fa34: 66 0f 28 4d 20              	movapd	0x20(%rbp), %xmm1
14233fa39: 0f 28 55 30                 	movaps	0x30(%rbp), %xmm2
14233fa3d: 0f 29 55 00                 	movaps	%xmm2, (%rbp)
14233fa41: 66 0f 29 4d f0              	movapd	%xmm1, -0x10(%rbp)
14233fa46: 66 0f 7f 45 e0              	movdqa	%xmm0, -0x20(%rbp)
14233fa4b: 48 8b 9d 68 01 00 00        	movq	0x168(%rbp), %rbx
14233fa52: 48 89 5d 70                 	movq	%rbx, 0x70(%rbp)
14233fa56: 48 8d 45 e0                 	leaq	-0x20(%rbp), %rax
14233fa5a: 48 89 45 78                 	movq	%rax, 0x78(%rbp)
14233fa5e: 48 8b 85 d0 00 00 00        	movq	0xd0(%rbp), %rax
14233fa65: 48 89 85 80 00 00 00        	movq	%rax, 0x80(%rbp)
14233fa6c: 48 8b 85 e8 00 00 00        	movq	0xe8(%rbp), %rax
14233fa73: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
14233fa7a: 48 8d bd e0 00 00 00        	leaq	0xe0(%rbp), %rdi
14233fa81: 48 89 bd 90 00 00 00        	movq	%rdi, 0x90(%rbp)
14233fa88: 4c 89 ad 98 00 00 00        	movq	%r13, 0x98(%rbp)
14233fa8f: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
14233fa96: 48 8b 95 a0 01 00 00        	movq	0x1a0(%rbp), %rdx
14233fa9d: 4d 89 f8                    	movq	%r15, %r8
14233faa0: 4c 8b 8d 70 01 00 00        	movq	0x170(%rbp), %r9
14233faa7: e8 b4 8d ff ff              	callq	0x142338860 <.text+0x2337860>
14233faac: 48 8b 85 f0 00 00 00        	movq	0xf0(%rbp), %rax
14233fab3: 48 85 c0                    	testq	%rax, %rax
14233fab6: 0f 84 5e 06 00 00           	je	0x14234011a <.text+0x233f11a>
14233fabc: 48 8b 00                    	movq	(%rax), %rax
14233fabf: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
14233fac6: 48 8d 8d 20 01 00 00        	leaq	0x120(%rbp), %rcx
14233facd: e8 ee 1b c7 fe              	callq	0x140fb16c0 <.text+0xfb06c0>
14233fad2: 8b 85 ac 01 00 00           	movl	0x1ac(%rbp), %eax
14233fad8: 0a 85 b0 01 00 00           	orb	0x1b0(%rbp), %al
14233fade: 88 85 ab 01 00 00           	movb	%al, 0x1ab(%rbp)
14233fae4: 8b 95 90 01 00 00           	movl	0x190(%rbp), %edx
14233faea: 44 8b bd 94 01 00 00        	movl	0x194(%rbp), %r15d
14233faf1: 48 8d 8d 20 01 00 00        	leaq	0x120(%rbp), %rcx
14233faf8: 4c 8d 25 b9 e3 d0 00        	leaq	0xd0e3b9(%rip), %r12    # 0x14304deb8
14233faff: 48 89 ce                    	movq	%rcx, %rsi
14233fb02: 45 89 f8                    	movl	%r15d, %r8d
14233fb05: e8 66 2a c7 fe              	callq	0x140fb2570 <.text+0xfb1570>
14233fb0a: 84 c0                       	testb	%al, %al
14233fb0c: 0f 85 f3 04 00 00           	jne	0x142340005 <.text+0x233f005>
14233fb12: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
14233fb16: 48 8d 95 90 01 00 00        	leaq	0x190(%rbp), %rdx
14233fb1d: e8 ee 84 ff ff              	callq	0x142338010 <.text+0x2337010>
14233fb22: 48 85 c0                    	testq	%rax, %rax
14233fb25: 0f 84 ca 00 00 00           	je	0x14233fbf5 <.text+0x233ebf5>
14233fb2b: 8b 10                       	movl	(%rax), %edx
14233fb2d: 44 8b 78 04                 	movl	0x4(%rax), %r15d
14233fb31: 48 8b 0b                    	movq	(%rbx), %rcx
14233fb34: 41 89 d6                    	movl	%edx, %r14d
14233fb37: 45 89 f8                    	movl	%r15d, %r8d
14233fb3a: e8 11 cb b0 ff              	callq	0x141e4c650 <.text+0x1e4b650>
14233fb3f: 80 b8 4b 01 00 00 00        	cmpb	$0x0, 0x14b(%rax)
14233fb46: 0f 95 c0                    	setne	%al
14233fb49: 32 85 aa 01 00 00           	xorb	0x1aa(%rbp), %al
14233fb4f: 0f 84 29 01 00 00           	je	0x14233fc7e <.text+0x233ec7e>
14233fb55: 48 8b 13                    	movq	(%rbx), %rdx
14233fb58: 48 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %rcx
14233fb5f: 45 89 f0                    	movl	%r14d, %r8d
14233fb62: 45 89 f9                    	movl	%r15d, %r9d
14233fb65: e8 e6 7d ca fe              	callq	0x140fe7950 <.text+0xfe6950>
14233fb6a: 83 bd b0 00 00 00 04        	cmpl	$0x4, 0xb0(%rbp)
14233fb71: 0f 85 e0 05 00 00           	jne	0x142340157 <.text+0x233f157>
14233fb77: 48 8b 85 b8 00 00 00        	movq	0xb8(%rbp), %rax
14233fb7e: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
14233fb85: 48 8b 95 c0 00 00 00        	movq	0xc0(%rbp), %rdx
14233fb8c: 48 8b 85 c8 00 00 00        	movq	0xc8(%rbp), %rax
14233fb93: 48 85 c0                    	testq	%rax, %rax
14233fb96: 48 89 95 a0 01 00 00        	movq	%rdx, 0x1a0(%rbp)
14233fb9d: 0f 84 66 05 00 00           	je	0x142340109 <.text+0x233f109>
14233fba3: f3 0f 7e 85 90 01 00 00     	movq	0x190(%rbp), %xmm0
14233fbab: 4c 8d 14 c5 00 00 00 00     	leaq	(,%rax,8), %r10
14233fbb3: 48 c7 c1 f8 ff ff ff        	movq	$-0x8, %rcx
14233fbba: 45 31 c0                    	xorl	%r8d, %r8d
14233fbbd: f3 42 0f 7e 0c c2           	movq	(%rdx,%r8,8), %xmm1
14233fbc3: 66 0f 76 c8                 	pcmpeqd	%xmm0, %xmm1
14233fbc7: 66 0f 70 c9 50              	pshufd	$0x50, %xmm1, %xmm1     # xmm1 = xmm1[0,0,1,1]
14233fbcc: 66 44 0f 50 c9              	movmskpd	%xmm1, %r9d
14233fbd1: 41 83 f9 03                 	cmpl	$0x3, %r9d
14233fbd5: 0f 84 c9 00 00 00           	je	0x14233fca4 <.text+0x233eca4>
14233fbdb: 49 ff c0                    	incq	%r8
14233fbde: 4d 8d 0c 0a                 	leaq	(%r10,%rcx), %r9
14233fbe2: 49 83 c1 f8                 	addq	$-0x8, %r9
14233fbe6: 48 83 c1 f8                 	addq	$-0x8, %rcx
14233fbea: 49 83 f9 f8                 	cmpq	$-0x8, %r9
14233fbee: 75 cd                       	jne	0x14233fbbd <.text+0x233ebbd>
14233fbf0: e9 14 05 00 00              	jmp	0x142340109 <.text+0x233f109>
14233fbf5: 48 8b 03                    	movq	(%rbx), %rax
14233fbf8: 4c 8b 80 a8 00 00 00        	movq	0xa8(%rax), %r8
14233fbff: 4d 85 c0                    	testq	%r8, %r8
14233fc02: 0f 84 fd 03 00 00           	je	0x142340005 <.text+0x233f005>
14233fc08: 48 8b 80 a0 00 00 00        	movq	0xa0(%rax), %rax
14233fc0f: 8b 8d 90 01 00 00           	movl	0x190(%rbp), %ecx
14233fc15: 8b 95 94 01 00 00           	movl	0x194(%rbp), %edx
14233fc1b: 49 c1 e0 03                 	shlq	$0x3, %r8
14233fc1f: 4f 8d 0c 80                 	leaq	(%r8,%r8,4), %r9
14233fc23: 45 31 c0                    	xorl	%r8d, %r8d
14233fc26: eb 0d                       	jmp	0x14233fc35 <.text+0x233ec35>
14233fc28: 49 83 c0 28                 	addq	$0x28, %r8
14233fc2c: 4d 39 c1                    	cmpq	%r8, %r9
14233fc2f: 0f 84 d0 03 00 00           	je	0x142340005 <.text+0x233f005>
14233fc35: 42 39 54 00 14              	cmpl	%edx, 0x14(%rax,%r8)
14233fc3a: 75 ec                       	jne	0x14233fc28 <.text+0x233ec28>
14233fc3c: 42 39 4c 00 10              	cmpl	%ecx, 0x10(%rax,%r8)
14233fc41: 75 e5                       	jne	0x14233fc28 <.text+0x233ec28>
14233fc43: 46 0f b6 54 00 24           	movzbl	0x24(%rax,%r8), %r10d
14233fc49: 41 83 fa 04                 	cmpl	$0x4, %r10d
14233fc4d: 74 d9                       	je	0x14233fc28 <.text+0x233ec28>
14233fc4f: 46 8b 74 00 18              	movl	0x18(%rax,%r8), %r14d
14233fc54: 46 8b 7c 00 1c              	movl	0x1c(%rax,%r8), %r15d
14233fc59: 44 89 d0                    	movl	%r10d, %eax
14233fc5c: 48 8d 0d 65 e2 d0 00        	leaq	0xd0e265(%rip), %rcx    # 0x14304dec8
14233fc63: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
14233fc67: 48 01 c8                    	addq	%rcx, %rax
14233fc6a: ff e0                       	jmpq	*%rax
14233fc6c: 80 bd 9c 01 00 00 00        	cmpb	$0x0, 0x19c(%rbp)
14233fc73: 0f 85 5c 03 00 00           	jne	0x14233ffd5 <.text+0x233efd5>
14233fc79: e9 6e 03 00 00              	jmp	0x14233ffec <.text+0x233efec>
14233fc7e: 48 c7 85 50 01 00 00 00 00 00 00    	movq	$0x0, 0x150(%rbp)
14233fc89: 48 c7 85 58 01 00 00 04 00 00 00    	movq	$0x4, 0x158(%rbp)
14233fc94: 48 c7 85 60 01 00 00 00 00 00 00    	movq	$0x0, 0x160(%rbp)
14233fc9f: e9 0f 01 00 00              	jmp	0x14233fdb3 <.text+0x233edb3>
14233fca4: 80 bd ab 01 00 00 00        	cmpb	$0x0, 0x1ab(%rbp)
14233fcab: 74 39                       	je	0x14233fce6 <.text+0x233ece6>
14233fcad: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
14233fcb4: e8 d7 88 ff ff              	callq	0x142338590 <.text+0x2337590>
14233fcb9: 48 8b 95 00 01 00 00        	movq	0x100(%rbp), %rdx
14233fcc0: 48 83 fa 02                 	cmpq	$0x2, %rdx
14233fcc4: 0f 82 a9 00 00 00           	jb	0x14233fd73 <.text+0x233ed73>
14233fcca: 48 8b 85 f8 00 00 00        	movq	0xf8(%rbp), %rax
14233fcd1: 49 89 d1                    	movq	%rdx, %r9
14233fcd4: 49 d1 e9                    	shrq	%r9
14233fcd7: 48 8d 0c d0                 	leaq	(%rax,%rdx,8), %rcx
14233fcdb: 49 83 f9 01                 	cmpq	$0x1, %r9
14233fcdf: 75 1f                       	jne	0x14233fd00 <.text+0x233ed00>
14233fce1: 45 31 c0                    	xorl	%r8d, %r8d
14233fce4: eb 64                       	jmp	0x14233fd4a <.text+0x233ed4a>
14233fce6: 49 f7 d0                    	notq	%r8
14233fce9: 49 01 c0                    	addq	%rax, %r8
14233fcec: 48 29 ca                    	subq	%rcx, %rdx
14233fcef: 48 8d 8d 50 01 00 00        	leaq	0x150(%rbp), %rcx
14233fcf6: e8 95 88 ff ff              	callq	0x142338590 <.text+0x2337590>
14233fcfb: e9 91 00 00 00              	jmp	0x14233fd91 <.text+0x233ed91>
14233fd00: 49 83 e1 fe                 	andq	$-0x2, %r9
14233fd04: 4c 8d 52 ff                 	leaq	-0x1(%rdx), %r10
14233fd08: 45 31 c0                    	xorl	%r8d, %r8d
14233fd0b: 48 bf fe ff ff ff ff ff ff 1f       	movabsq	$0x1ffffffffffffffe, %rdi # imm = 0x1FFFFFFFFFFFFFFE
14233fd15: 4e 8b 1c d0                 	movq	(%rax,%r10,8), %r11
14233fd19: 4a 8b 1c c0                 	movq	(%rax,%r8,8), %rbx
14233fd1d: 4e 89 1c c0                 	movq	%r11, (%rax,%r8,8)
14233fd21: 4a 89 1c d0                 	movq	%rbx, (%rax,%r10,8)
14233fd25: 4d 89 c3                    	movq	%r8, %r11
14233fd28: 49 31 fb                    	xorq	%rdi, %r11
14233fd2b: 4a 8b 1c d9                 	movq	(%rcx,%r11,8), %rbx
14233fd2f: 4e 8b 6c c0 08              	movq	0x8(%rax,%r8,8), %r13
14233fd34: 4a 89 5c c0 08              	movq	%rbx, 0x8(%rax,%r8,8)
14233fd39: 4e 89 2c d9                 	movq	%r13, (%rcx,%r11,8)
14233fd3d: 49 83 c0 02                 	addq	$0x2, %r8
14233fd41: 49 83 c2 fe                 	addq	$-0x2, %r10
14233fd45: 4d 39 c1                    	cmpq	%r8, %r9
14233fd48: 75 cb                       	jne	0x14233fd15 <.text+0x233ed15>
14233fd4a: f6 c2 02                    	testb	$0x2, %dl
14233fd4d: 48 8b 9d 68 01 00 00        	movq	0x168(%rbp), %rbx
14233fd54: 48 8d bd e0 00 00 00        	leaq	0xe0(%rbp), %rdi
14233fd5b: 74 16                       	je	0x14233fd73 <.text+0x233ed73>
14233fd5d: 4c 89 c2                    	movq	%r8, %rdx
14233fd60: 48 f7 d2                    	notq	%rdx
14233fd63: 4c 8b 0c d1                 	movq	(%rcx,%rdx,8), %r9
14233fd67: 4e 8b 14 c0                 	movq	(%rax,%r8,8), %r10
14233fd6b: 4e 89 0c c0                 	movq	%r9, (%rax,%r8,8)
14233fd6f: 4c 89 14 d1                 	movq	%r10, (%rcx,%rdx,8)
14233fd73: 48 8b 85 00 01 00 00        	movq	0x100(%rbp), %rax
14233fd7a: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
14233fd81: f3 0f 6f 85 f0 00 00 00     	movdqu	0xf0(%rbp), %xmm0
14233fd89: 66 0f 7f 85 50 01 00 00     	movdqa	%xmm0, 0x150(%rbp)
14233fd91: 48 8b 95 80 01 00 00        	movq	0x180(%rbp), %rdx
14233fd98: 48 85 d2                    	testq	%rdx, %rdx
14233fd9b: 74 16                       	je	0x14233fdb3 <.text+0x233edb3>
14233fd9d: 48 c1 e2 03                 	shlq	$0x3, %rdx
14233fda1: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14233fda7: 48 8b 8d a0 01 00 00        	movq	0x1a0(%rbp), %rcx
14233fdae: e8 6d 3e 2d fe              	callq	0x140613c20 <.text+0x612c20>
14233fdb3: 48 8b 03                    	movq	(%rbx), %rax
14233fdb6: 48 8b 98 a0 00 00 00        	movq	0xa0(%rax), %rbx
14233fdbd: 48 8b 80 a8 00 00 00        	movq	0xa8(%rax), %rax
14233fdc4: 48 8d 04 80                 	leaq	(%rax,%rax,4), %rax
14233fdc8: 48 8d 14 c3                 	leaq	(%rbx,%rax,8), %rdx
14233fdcc: 48 89 95 a0 01 00 00        	movq	%rdx, 0x1a0(%rbp)
14233fdd3: eb 1d                       	jmp	0x14233fdf2 <.text+0x233edf2>
14233fdd5: 48 8b 85 58 01 00 00        	movq	0x158(%rbp), %rax
14233fddc: 42 0f 13 34 e8              	movlps	%xmm6, (%rax,%r13,8)
14233fde1: 49 ff c5                    	incq	%r13
14233fde4: 4c 89 ad 60 01 00 00        	movq	%r13, 0x160(%rbp)
14233fdeb: 48 8b 95 a0 01 00 00        	movq	0x1a0(%rbp), %rdx
14233fdf2: 49 89 d8                    	movq	%rbx, %r8
14233fdf5: 48 39 d3                    	cmpq	%rdx, %rbx
14233fdf8: 0f 84 94 00 00 00           	je	0x14233fe92 <.text+0x233ee92>
14233fdfe: 49 8d 58 28                 	leaq	0x28(%r8), %rbx
14233fe02: 41 0f b6 40 24              	movzbl	0x24(%r8), %eax
14233fe07: 83 f8 04                    	cmpl	$0x4, %eax
14233fe0a: 74 e6                       	je	0x14233fdf2 <.text+0x233edf2>
14233fe0c: 41 8b 48 1c                 	movl	0x1c(%r8), %ecx
14233fe10: 3b 8d 94 01 00 00           	cmpl	0x194(%rbp), %ecx
14233fe16: 75 da                       	jne	0x14233fdf2 <.text+0x233edf2>
14233fe18: 8b 8d 90 01 00 00           	movl	0x190(%rbp), %ecx
14233fe1e: 41 39 48 18                 	cmpl	%ecx, 0x18(%r8)
14233fe22: 75 ce                       	jne	0x14233fdf2 <.text+0x233edf2>
14233fe24: 89 c0                       	movl	%eax, %eax
14233fe26: 49 63 04 84                 	movslq	(%r12,%rax,4), %rax
14233fe2a: 4c 01 e0                    	addq	%r12, %rax
14233fe2d: ff e0                       	jmpq	*%rax
14233fe2f: 80 bd b0 01 00 00 00        	cmpb	$0x0, 0x1b0(%rbp)
14233fe36: 74 ba                       	je	0x14233fdf2 <.text+0x233edf2>
14233fe38: eb 1f                       	jmp	0x14233fe59 <.text+0x233ee59>
14233fe3a: 80 bd ac 01 00 00 00        	cmpb	$0x0, 0x1ac(%rbp)
14233fe41: 74 af                       	je	0x14233fdf2 <.text+0x233edf2>
14233fe43: eb 14                       	jmp	0x14233fe59 <.text+0x233ee59>
14233fe45: 80 bd 98 01 00 00 00        	cmpb	$0x0, 0x198(%rbp)
14233fe4c: 74 a4                       	je	0x14233fdf2 <.text+0x233edf2>
14233fe4e: eb 09                       	jmp	0x14233fe59 <.text+0x233ee59>
14233fe50: 80 bd 9c 01 00 00 00        	cmpb	$0x0, 0x19c(%rbp)
14233fe57: 74 99                       	je	0x14233fdf2 <.text+0x233edf2>
14233fe59: 4c 89 85 80 01 00 00        	movq	%r8, 0x180(%rbp)
14233fe60: f2 41 0f 10 70 10           	movsd	0x10(%r8), %xmm6
14233fe66: 4c 8b ad 60 01 00 00        	movq	0x160(%rbp), %r13
14233fe6d: 4c 3b ad 50 01 00 00        	cmpq	0x150(%rbp), %r13
14233fe74: 0f 85 5b ff ff ff           	jne	0x14233fdd5 <.text+0x233edd5>
14233fe7a: 48 8d 8d 50 01 00 00        	leaq	0x150(%rbp), %rcx
14233fe81: 48 8d 15 c8 df d0 00        	leaq	0xd0dfc8(%rip), %rdx    # 0x14304de50
14233fe88: e8 03 50 40 fe              	callq	0x140744e90 <.text+0x743e90>
14233fe8d: e9 43 ff ff ff              	jmp	0x14233fdd5 <.text+0x233edd5>
14233fe92: 4c 89 85 80 01 00 00        	movq	%r8, 0x180(%rbp)
14233fe99: 48 8b 85 50 01 00 00        	movq	0x150(%rbp), %rax
14233fea0: 48 89 85 78 01 00 00        	movq	%rax, 0x178(%rbp)
14233fea7: 48 8b 85 58 01 00 00        	movq	0x158(%rbp), %rax
14233feae: 48 89 85 88 01 00 00        	movq	%rax, 0x188(%rbp)
14233feb5: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
14233febc: 48 85 c0                    	testq	%rax, %rax
14233febf: 0f 84 a8 00 00 00           	je	0x14233ff6d <.text+0x233ef6d>
14233fec5: 4c 8b ad 88 01 00 00        	movq	0x188(%rbp), %r13
14233fecc: 48 8d 1c c5 00 00 00 00     	leaq	(,%rax,8), %rbx
14233fed4: 4c 01 eb                    	addq	%r13, %rbx
14233fed7: 41 8b 55 00                 	movl	(%r13), %edx
14233fedb: 45 8b 45 04                 	movl	0x4(%r13), %r8d
14233fedf: c7 85 b4 00 00 00 00 00 00 00       	movl	$0x0, 0xb4(%rbp)
14233fee9: c7 85 dc 00 00 00 ff ff 7f 7f       	movl	$0x7f7fffff, 0xdc(%rbp) # imm = 0x7F7FFFFF
14233fef3: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
14233fefa: 48 8b 08                    	movq	(%rax), %rcx
14233fefd: 48 8d 45 e0                 	leaq	-0x20(%rbp), %rax
14233ff01: 48 89 85 f0 00 00 00        	movq	%rax, 0xf0(%rbp)
14233ff08: 48 8b 85 d0 00 00 00        	movq	0xd0(%rbp), %rax
14233ff0f: 48 89 85 f8 00 00 00        	movq	%rax, 0xf8(%rbp)
14233ff16: 48 8b 85 e8 00 00 00        	movq	0xe8(%rbp), %rax
14233ff1d: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
14233ff24: 48 89 bd 08 01 00 00        	movq	%rdi, 0x108(%rbp)
14233ff2b: 48 8d 85 dc 00 00 00        	leaq	0xdc(%rbp), %rax
14233ff32: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
14233ff39: 48 8d 85 b0 00 00 00        	leaq	0xb0(%rbp), %rax
14233ff40: 48 89 85 18 01 00 00        	movq	%rax, 0x118(%rbp)
14233ff47: 4c 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %r9
14233ff4e: e8 0d 8d b6 ff              	callq	0x141ea8c60 <.text+0x1ea7c60>
14233ff53: 83 bd b4 00 00 00 00        	cmpl	$0x0, 0xb4(%rbp)
14233ff5a: 0f 85 2f 01 00 00           	jne	0x14234008f <.text+0x233f08f>
14233ff60: 49 83 c5 08                 	addq	$0x8, %r13
14233ff64: 49 39 dd                    	cmpq	%rbx, %r13
14233ff67: 0f 85 6a ff ff ff           	jne	0x14233fed7 <.text+0x233eed7>
14233ff6d: 48 8b 95 78 01 00 00        	movq	0x178(%rbp), %rdx
14233ff74: 48 85 d2                    	testq	%rdx, %rdx
14233ff77: 74 16                       	je	0x14233ff8f <.text+0x233ef8f>
14233ff79: 48 c1 e2 03                 	shlq	$0x3, %rdx
14233ff7d: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14233ff83: 48 8b 8d 88 01 00 00        	movq	0x188(%rbp), %rcx
14233ff8a: e8 91 3c 2d fe              	callq	0x140613c20 <.text+0x612c20>
14233ff8f: 44 89 f2                    	movl	%r14d, %edx
14233ff92: 44 89 b5 90 01 00 00        	movl	%r14d, 0x190(%rbp)
14233ff99: 44 89 bd 94 01 00 00        	movl	%r15d, 0x194(%rbp)
14233ffa0: 4c 8b ad 60 02 00 00        	movq	0x260(%rbp), %r13
14233ffa7: 48 8b 9d 68 01 00 00        	movq	0x168(%rbp), %rbx
14233ffae: 48 89 f1                    	movq	%rsi, %rcx
14233ffb1: e9 49 fb ff ff              	jmp	0x14233faff <.text+0x233eaff>
14233ffb6: 80 bd 98 01 00 00 00        	cmpb	$0x0, 0x198(%rbp)
14233ffbd: 75 16                       	jne	0x14233ffd5 <.text+0x233efd5>
14233ffbf: eb 2b                       	jmp	0x14233ffec <.text+0x233efec>
14233ffc1: 80 bd ac 01 00 00 00        	cmpb	$0x0, 0x1ac(%rbp)
14233ffc8: 75 0b                       	jne	0x14233ffd5 <.text+0x233efd5>
14233ffca: eb 20                       	jmp	0x14233ffec <.text+0x233efec>
14233ffcc: 80 bd b0 01 00 00 00        	cmpb	$0x0, 0x1b0(%rbp)
14233ffd3: 74 17                       	je	0x14233ffec <.text+0x233efec>
14233ffd5: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
14233ffd9: 44 89 f2                    	movl	%r14d, %edx
14233ffdc: 45 89 f8                    	movl	%r15d, %r8d
14233ffdf: e8 7c 06 00 00              	callq	0x142340660 <.text+0x233f660>
14233ffe4: 84 c0                       	testb	%al, %al
14233ffe6: 0f 85 ec 00 00 00           	jne	0x1423400d8 <.text+0x233f0d8>
14233ffec: 44 89 f2                    	movl	%r14d, %edx
14233ffef: 44 89 b5 90 01 00 00        	movl	%r14d, 0x190(%rbp)
14233fff6: 44 89 bd 94 01 00 00        	movl	%r15d, 0x194(%rbp)
14233fffd: 48 89 f1                    	movq	%rsi, %rcx
142340000: e9 fa fa ff ff              	jmp	0x14233faff <.text+0x233eaff>
142340005: 48 8d 8d 20 01 00 00        	leaq	0x120(%rbp), %rcx
14234000c: e8 3f 9f e0 fd              	callq	0x140149f50 <.text+0x148f50>
142340011: c6 85 b6 01 00 00 00        	movb	$0x0, 0x1b6(%rbp)
142340018: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
14234001c: e8 4f 9c e0 fd              	callq	0x140149c70 <.text+0x148c70>
142340021: c6 85 b7 01 00 00 00        	movb	$0x0, 0x1b7(%rbp)
142340028: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
14234002c: e8 3f 9c e0 fd              	callq	0x140149c70 <.text+0x148c70>
142340031: 48 8d 05 a0 de d0 00        	leaq	0xd0dea0(%rip), %rax    # 0x14304ded8
142340038: 48 8b 8d a8 00 00 00        	movq	0xa8(%rbp), %rcx
14234003f: 48 63 0c 88                 	movslq	(%rax,%rcx,4), %rcx
142340043: 48 01 c1                    	addq	%rax, %rcx
142340046: 48 8b 95 a0 00 00 00        	movq	0xa0(%rbp), %rdx
14234004d: ff e1                       	jmpq	*%rcx
14234004f: 48 89 95 70 01 00 00        	movq	%rdx, 0x170(%rbp)
142340056: 48 8b bd 70 01 00 00        	movq	0x170(%rbp), %rdi
14234005d: 48 c1 ef 20                 	shrq	$0x20, %rdi
142340061: 0f 84 83 f3 ff ff           	je	0x14233f3ea <.text+0x233e3ea>
142340067: 41 8b 45 1c                 	movl	0x1c(%r13), %eax
14234006b: 49 8b 4d 10                 	movq	0x10(%r13), %rcx
14234006f: 89 01                       	movl	%eax, (%rcx)
142340071: 49 8b 45 00                 	movq	(%r13), %rax
142340075: 48 c7 40 10 01 00 00 00     	movq	$0x1, 0x10(%rax)
14234007d: 48 8b 8d 70 01 00 00        	movq	0x170(%rbp), %rcx
142340084: 89 48 18                    	movl	%ecx, 0x18(%rax)
142340087: 89 78 1c                    	movl	%edi, 0x1c(%rax)
14234008a: e9 5b f3 ff ff              	jmp	0x14233f3ea <.text+0x233e3ea>
14234008f: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
142340096: 4c 8b 85 60 02 00 00        	movq	0x260(%rbp), %r8
14234009d: 41 8b 48 1c                 	movl	0x1c(%r8), %ecx
1423400a1: 49 8b 50 10                 	movq	0x10(%r8), %rdx
1423400a5: 89 0a                       	movl	%ecx, (%rdx)
1423400a7: 49 8b 08                    	movq	(%r8), %rcx
1423400aa: 48 c7 41 10 01 00 00 00     	movq	$0x1, 0x10(%rcx)
1423400b2: 48 89 41 18                 	movq	%rax, 0x18(%rcx)
1423400b6: 48 8b 95 78 01 00 00        	movq	0x178(%rbp), %rdx
1423400bd: 48 85 d2                    	testq	%rdx, %rdx
1423400c0: 74 16                       	je	0x1423400d8 <.text+0x233f0d8>
1423400c2: 48 c1 e2 03                 	shlq	$0x3, %rdx
1423400c6: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1423400cc: 48 8b 8d 88 01 00 00        	movq	0x188(%rbp), %rcx
1423400d3: e8 48 3b 2d fe              	callq	0x140613c20 <.text+0x612c20>
1423400d8: 48 8d 8d 20 01 00 00        	leaq	0x120(%rbp), %rcx
1423400df: e8 6c 9e e0 fd              	callq	0x140149f50 <.text+0x148f50>
1423400e4: c6 85 b6 01 00 00 00        	movb	$0x0, 0x1b6(%rbp)
1423400eb: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
1423400ef: e8 7c 9b e0 fd              	callq	0x140149c70 <.text+0x148c70>
1423400f4: c6 85 b7 01 00 00 00        	movb	$0x0, 0x1b7(%rbp)
1423400fb: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
1423400ff: e8 6c 9b e0 fd              	callq	0x140149c70 <.text+0x148c70>
142340104: e9 e1 f2 ff ff              	jmp	0x14233f3ea <.text+0x233e3ea>
142340109: 48 8d 0d 28 dd d0 00        	leaq	0xd0dd28(%rip), %rcx    # 0x14304de38
142340110: e8 fb 91 59 00              	callq	0x1428d9310 <.text+0x28d8310>
142340115: e9 88 00 00 00              	jmp	0x1423401a2 <.text+0x233f1a2>
14234011a: f3 0f 6f 85 f8 00 00 00     	movdqu	0xf8(%rbp), %xmm0
142340122: 66 0f 7f 85 20 01 00 00     	movdqa	%xmm0, 0x120(%rbp)
14234012a: 48 8d 05 d7 dc d0 00        	leaq	0xd0dcd7(%rip), %rax    # 0x14304de08
142340131: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
142340136: 48 8d 0d c3 d6 d0 00        	leaq	0xd0d6c3(%rip), %rcx    # 0x14304d800
14234013d: 4c 8d 0d 9c d6 d0 00        	leaq	0xd0d69c(%rip), %r9     # 0x14304d7e0
142340144: 4c 8d 85 20 01 00 00        	leaq	0x120(%rbp), %r8
14234014b: ba 2b 00 00 00              	movl	$0x2b, %edx
142340150: e8 0b 96 59 00              	callq	0x1428d9760 <.text+0x28d8760>
142340155: eb 4b                       	jmp	0x1423401a2 <.text+0x233f1a2>
142340157: f3 0f 6f 85 b0 00 00 00     	movdqu	0xb0(%rbp), %xmm0
14234015f: 66 0f 10 8d c0 00 00 00     	movupd	0xc0(%rbp), %xmm1
142340167: 66 0f 29 8d 00 01 00 00     	movapd	%xmm1, 0x100(%rbp)
14234016f: 66 0f 7f 85 f0 00 00 00     	movdqa	%xmm0, 0xf0(%rbp)
142340177: 48 8d 05 a2 dc d0 00        	leaq	0xd0dca2(%rip), %rax    # 0x14304de20
14234017e: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
142340183: 48 8d 0d 76 d6 d0 00        	leaq	0xd0d676(%rip), %rcx    # 0x14304d800
14234018a: 4c 8d 0d 9f d6 d0 00        	leaq	0xd0d69f(%rip), %r9     # 0x14304d830
142340191: 4c 8d 85 f0 00 00 00        	leaq	0xf0(%rbp), %r8
142340198: ba 2b 00 00 00              	movl	$0x2b, %edx
14234019d: e8 be 95 59 00              	callq	0x1428d9760 <.text+0x28d8760>
1423401a2: 0f 0b                       	ud2
1423401a4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1423401b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1423401b5: 55                          	pushq	%rbp
1423401b6: 41 57                       	pushq	%r15
1423401b8: 41 56                       	pushq	%r14
1423401ba: 41 55                       	pushq	%r13
1423401bc: 41 54                       	pushq	%r12
1423401be: 56                          	pushq	%rsi
1423401bf: 57                          	pushq	%rdi
1423401c0: 53                          	pushq	%rbx
1423401c1: 48 83 ec 58                 	subq	$0x58, %rsp
1423401c5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1423401cc: 44 0f 29 44 24 20           	movaps	%xmm8, 0x20(%rsp)
1423401d2: 0f 29 7c 24 30              	movaps	%xmm7, 0x30(%rsp)
1423401d7: 0f 29 74 24 40              	movaps	%xmm6, 0x40(%rsp)
1423401dc: 48 83 bd 78 01 00 00 00     	cmpq	$0x0, 0x178(%rbp)
1423401e4: 74 1d                       	je	0x142340203 <.text+0x233f203>
1423401e6: 48 8b 95 78 01 00 00        	movq	0x178(%rbp), %rdx
1423401ed: 48 c1 e2 03                 	shlq	$0x3, %rdx
1423401f1: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1423401f7: 48 8b 8d 88 01 00 00        	movq	0x188(%rbp), %rcx
1423401fe: e8 1d 3a 2d fe              	callq	0x140613c20 <.text+0x612c20>
142340203: 0f 28 74 24 40              	movaps	0x40(%rsp), %xmm6
142340208: 0f 28 7c 24 30              	movaps	0x30(%rsp), %xmm7
14234020d: 44 0f 28 44 24 20           	movaps	0x20(%rsp), %xmm8
142340213: 48 83 c4 58                 	addq	$0x58, %rsp
142340217: 5b                          	popq	%rbx
142340218: 5f                          	popq	%rdi
142340219: 5e                          	popq	%rsi
14234021a: 41 5c                       	popq	%r12
14234021c: 41 5d                       	popq	%r13
14234021e: 41 5e                       	popq	%r14
142340220: 41 5f                       	popq	%r15
142340222: 5d                          	popq	%rbp
142340223: c3                          	retq
142340224: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
142340230: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
142340235: 55                          	pushq	%rbp
142340236: 41 57                       	pushq	%r15
142340238: 41 56                       	pushq	%r14
14234023a: 41 55                       	pushq	%r13
14234023c: 41 54                       	pushq	%r12
14234023e: 56                          	pushq	%rsi
14234023f: 57                          	pushq	%rdi
142340240: 53                          	pushq	%rbx
142340241: 48 83 ec 58                 	subq	$0x58, %rsp
142340245: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14234024c: 44 0f 29 44 24 20           	movaps	%xmm8, 0x20(%rsp)
142340252: 0f 29 7c 24 30              	movaps	%xmm7, 0x30(%rsp)
142340257: 0f 29 74 24 40              	movaps	%xmm6, 0x40(%rsp)
14234025c: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
142340263: 48 3b 85 a0 01 00 00        	cmpq	0x1a0(%rbp), %rax
14234026a: 75 21                       	jne	0x14234028d <.text+0x233f28d>
14234026c: 0f 28 74 24 40              	movaps	0x40(%rsp), %xmm6
142340271: 0f 28 7c 24 30              	movaps	0x30(%rsp), %xmm7
142340276: 44 0f 28 44 24 20           	movaps	0x20(%rsp), %xmm8
14234027c: 48 83 c4 58                 	addq	$0x58, %rsp
142340280: 5b                          	popq	%rbx
142340281: 5f                          	popq	%rdi
142340282: 5e                          	popq	%rsi
142340283: 41 5c                       	popq	%r12
142340285: 41 5d                       	popq	%r13
142340287: 41 5e                       	popq	%r14
142340289: 41 5f                       	popq	%r15
14234028b: 5d                          	popq	%rbp
14234028c: c3                          	retq
14234028d: 48 8b 95 50 01 00 00        	movq	0x150(%rbp), %rdx
142340294: 48 85 d2                    	testq	%rdx, %rdx
142340297: 74 16                       	je	0x1423402af <.text+0x233f2af>
142340299: 48 8b 8d 58 01 00 00        	movq	0x158(%rbp), %rcx
1423402a0: 48 c1 e2 03                 	shlq	$0x3, %rdx
1423402a4: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1423402aa: e8 71 39 2d fe              	callq	0x140613c20 <.text+0x612c20>
1423402af: 0f 28 74 24 40              	movaps	0x40(%rsp), %xmm6
1423402b4: 0f 28 7c 24 30              	movaps	0x30(%rsp), %xmm7
1423402b9: 44 0f 28 44 24 20           	movaps	0x20(%rsp), %xmm8
1423402bf: 48 83 c4 58                 	addq	$0x58, %rsp
1423402c3: 5b                          	popq	%rbx
1423402c4: 5f                          	popq	%rdi
1423402c5: 5e                          	popq	%rsi
1423402c6: 41 5c                       	popq	%r12
1423402c8: 41 5d                       	popq	%r13
1423402ca: 41 5e                       	popq	%r14
1423402cc: 41 5f                       	popq	%r15
1423402ce: 5d                          	popq	%rbp
1423402cf: c3                          	retq
1423402d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1423402d5: 55                          	pushq	%rbp
1423402d6: 41 57                       	pushq	%r15
1423402d8: 41 56                       	pushq	%r14
1423402da: 41 55                       	pushq	%r13
1423402dc: 41 54                       	pushq	%r12
1423402de: 56                          	pushq	%rsi
1423402df: 57                          	pushq	%rdi
1423402e0: 53                          	pushq	%rbx
1423402e1: 48 83 ec 58                 	subq	$0x58, %rsp
1423402e5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1423402ec: 44 0f 29 44 24 20           	movaps	%xmm8, 0x20(%rsp)
1423402f2: 0f 29 7c 24 30              	movaps	%xmm7, 0x30(%rsp)
1423402f7: 0f 29 74 24 40              	movaps	%xmm6, 0x40(%rsp)
1423402fc: 48 83 bd 80 01 00 00 00     	cmpq	$0x0, 0x180(%rbp)
142340304: 74 1d                       	je	0x142340323 <.text+0x233f323>
142340306: 48 8b 95 80 01 00 00        	movq	0x180(%rbp), %rdx
14234030d: 48 c1 e2 03                 	shlq	$0x3, %rdx
142340311: 41 b8 04 00 00 00           	movl	$0x4, %r8d
142340317: 48 8b 8d a0 01 00 00        	movq	0x1a0(%rbp), %rcx
14234031e: e8 fd 38 2d fe              	callq	0x140613c20 <.text+0x612c20>
142340323: 0f 28 74 24 40              	movaps	0x40(%rsp), %xmm6
142340328: 0f 28 7c 24 30              	movaps	0x30(%rsp), %xmm7
14234032d: 44 0f 28 44 24 20           	movaps	0x20(%rsp), %xmm8
142340333: 48 83 c4 58                 	addq	$0x58, %rsp
142340337: 5b                          	popq	%rbx
142340338: 5f                          	popq	%rdi
142340339: 5e                          	popq	%rsi
14234033a: 41 5c                       	popq	%r12
14234033c: 41 5d                       	popq	%r13
14234033e: 41 5e                       	popq	%r14
142340340: 41 5f                       	popq	%r15
142340342: 5d                          	popq	%rbp
142340343: c3                          	retq
142340344: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
142340350: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
142340355: 55                          	pushq	%rbp
142340356: 41 57                       	pushq	%r15
142340358: 41 56                       	pushq	%r14
14234035a: 41 55                       	pushq	%r13
14234035c: 41 54                       	pushq	%r12
14234035e: 56                          	pushq	%rsi
14234035f: 57                          	pushq	%rdi
142340360: 53                          	pushq	%rbx
142340361: 48 83 ec 58                 	subq	$0x58, %rsp
142340365: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14234036c: 44 0f 29 44 24 20           	movaps	%xmm8, 0x20(%rsp)
142340372: 0f 29 7c 24 30              	movaps	%xmm7, 0x30(%rsp)
142340377: 0f 29 74 24 40              	movaps	%xmm6, 0x40(%rsp)
14234037c: 48 8d 8d 20 01 00 00        	leaq	0x120(%rbp), %rcx
142340383: e8 c8 9b e0 fd              	callq	0x140149f50 <.text+0x148f50>
142340388: 0f 28 74 24 40              	movaps	0x40(%rsp), %xmm6
14234038d: 0f 28 7c 24 30              	movaps	0x30(%rsp), %xmm7
142340392: 44 0f 28 44 24 20           	movaps	0x20(%rsp), %xmm8
142340398: 48 83 c4 58                 	addq	$0x58, %rsp
14234039c: 5b                          	popq	%rbx
14234039d: 5f                          	popq	%rdi
14234039e: 5e                          	popq	%rsi
14234039f: 41 5c                       	popq	%r12
1423403a1: 41 5d                       	popq	%r13
1423403a3: 41 5e                       	popq	%r14
1423403a5: 41 5f                       	popq	%r15
1423403a7: 5d                          	popq	%rbp
1423403a8: c3                          	retq
1423403a9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1423403b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1423403b5: 55                          	pushq	%rbp
1423403b6: 41 57                       	pushq	%r15
1423403b8: 41 56                       	pushq	%r14
1423403ba: 41 55                       	pushq	%r13
1423403bc: 41 54                       	pushq	%r12
1423403be: 56                          	pushq	%rsi
1423403bf: 57                          	pushq	%rdi
1423403c0: 53                          	pushq	%rbx
1423403c1: 48 83 ec 58                 	subq	$0x58, %rsp
1423403c5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1423403cc: 44 0f 29 44 24 20           	movaps	%xmm8, 0x20(%rsp)
1423403d2: 0f 29 7c 24 30              	movaps	%xmm7, 0x30(%rsp)
1423403d7: 0f 29 74 24 40              	movaps	%xmm6, 0x40(%rsp)
1423403dc: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
1423403e0: e8 8b 98 e0 fd              	callq	0x140149c70 <.text+0x148c70>
1423403e5: c6 85 b6 01 00 00 00        	movb	$0x0, 0x1b6(%rbp)
1423403ec: 0f 28 74 24 40              	movaps	0x40(%rsp), %xmm6
1423403f1: 0f 28 7c 24 30              	movaps	0x30(%rsp), %xmm7
1423403f6: 44 0f 28 44 24 20           	movaps	0x20(%rsp), %xmm8
1423403fc: 48 83 c4 58                 	addq	$0x58, %rsp
142340400: 5b                          	popq	%rbx
142340401: 5f                          	popq	%rdi
142340402: 5e                          	popq	%rsi
142340403: 41 5c                       	popq	%r12
142340405: 41 5d                       	popq	%r13
142340407: 41 5e                       	popq	%r14
142340409: 41 5f                       	popq	%r15
14234040b: 5d                          	popq	%rbp
14234040c: c3                          	retq
14234040d: 0f 1f 00                    	nopl	(%rax)
142340410: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
142340415: 55                          	pushq	%rbp
142340416: 41 57                       	pushq	%r15
142340418: 41 56                       	pushq	%r14
14234041a: 41 55                       	pushq	%r13
14234041c: 41 54                       	pushq	%r12
14234041e: 56                          	pushq	%rsi
14234041f: 57                          	pushq	%rdi
142340420: 53                          	pushq	%rbx
142340421: 48 83 ec 58                 	subq	$0x58, %rsp
142340425: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14234042c: 44 0f 29 44 24 20           	movaps	%xmm8, 0x20(%rsp)
142340432: 0f 29 7c 24 30              	movaps	%xmm7, 0x30(%rsp)
142340437: 0f 29 74 24 40              	movaps	%xmm6, 0x40(%rsp)
14234043c: 80 bd b6 01 00 00 00        	cmpb	$0x0, 0x1b6(%rbp)
142340443: 74 2a                       	je	0x14234046f <.text+0x233f46f>
142340445: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
142340449: e8 22 98 e0 fd              	callq	0x140149c70 <.text+0x148c70>
14234044e: 0f 28 74 24 40              	movaps	0x40(%rsp), %xmm6
142340453: 0f 28 7c 24 30              	movaps	0x30(%rsp), %xmm7
142340458: 44 0f 28 44 24 20           	movaps	0x20(%rsp), %xmm8
14234045e: 48 83 c4 58                 	addq	$0x58, %rsp
142340462: 5b                          	popq	%rbx
142340463: 5f                          	popq	%rdi
142340464: 5e                          	popq	%rsi
142340465: 41 5c                       	popq	%r12
142340467: 41 5d                       	popq	%r13
142340469: 41 5e                       	popq	%r14
14234046b: 41 5f                       	popq	%r15
14234046d: 5d                          	popq	%rbp
14234046e: c3                          	retq
14234046f: 0f 28 74 24 40              	movaps	0x40(%rsp), %xmm6
142340474: 0f 28 7c 24 30              	movaps	0x30(%rsp), %xmm7
142340479: 44 0f 28 44 24 20           	movaps	0x20(%rsp), %xmm8
14234047f: 48 83 c4 58                 	addq	$0x58, %rsp
142340483: 5b                          	popq	%rbx
142340484: 5f                          	popq	%rdi
142340485: 5e                          	popq	%rsi
142340486: 41 5c                       	popq	%r12
142340488: 41 5d                       	popq	%r13
14234048a: 41 5e                       	popq	%r14
14234048c: 41 5f                       	popq	%r15
14234048e: 5d                          	popq	%rbp
14234048f: c3                          	retq
142340490: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
142340495: 55                          	pushq	%rbp
142340496: 41 57                       	pushq	%r15
142340498: 41 56                       	pushq	%r14
14234049a: 41 55                       	pushq	%r13
14234049c: 41 54                       	pushq	%r12
14234049e: 56                          	pushq	%rsi
14234049f: 57                          	pushq	%rdi
1423404a0: 53                          	pushq	%rbx
1423404a1: 48 83 ec 58                 	subq	$0x58, %rsp
1423404a5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1423404ac: 44 0f 29 44 24 20           	movaps	%xmm8, 0x20(%rsp)
1423404b2: 0f 29 7c 24 30              	movaps	%xmm7, 0x30(%rsp)
1423404b7: 0f 29 74 24 40              	movaps	%xmm6, 0x40(%rsp)
1423404bc: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
1423404c0: e8 ab 97 e0 fd              	callq	0x140149c70 <.text+0x148c70>
1423404c5: c6 85 b7 01 00 00 00        	movb	$0x0, 0x1b7(%rbp)
1423404cc: 0f 28 74 24 40              	movaps	0x40(%rsp), %xmm6
1423404d1: 0f 28 7c 24 30              	movaps	0x30(%rsp), %xmm7
1423404d6: 44 0f 28 44 24 20           	movaps	0x20(%rsp), %xmm8
1423404dc: 48 83 c4 58                 	addq	$0x58, %rsp
1423404e0: 5b                          	popq	%rbx
1423404e1: 5f                          	popq	%rdi
1423404e2: 5e                          	popq	%rsi
1423404e3: 41 5c                       	popq	%r12
1423404e5: 41 5d                       	popq	%r13
1423404e7: 41 5e                       	popq	%r14
1423404e9: 41 5f                       	popq	%r15
1423404eb: 5d                          	popq	%rbp
1423404ec: c3                          	retq
1423404ed: 0f 1f 00                    	nopl	(%rax)
1423404f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1423404f5: 55                          	pushq	%rbp
1423404f6: 41 57                       	pushq	%r15
1423404f8: 41 56                       	pushq	%r14
1423404fa: 41 55                       	pushq	%r13
1423404fc: 41 54                       	pushq	%r12
1423404fe: 56                          	pushq	%rsi
1423404ff: 57                          	pushq	%rdi
142340500: 53                          	pushq	%rbx
142340501: 48 83 ec 58                 	subq	$0x58, %rsp
142340505: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14234050c: 44 0f 29 44 24 20           	movaps	%xmm8, 0x20(%rsp)
142340512: 0f 29 7c 24 30              	movaps	%xmm7, 0x30(%rsp)
142340517: 0f 29 74 24 40              	movaps	%xmm6, 0x40(%rsp)
14234051c: 48 83 bd a0 01 00 00 00     	cmpq	$0x0, 0x1a0(%rbp)
142340524: 74 1d                       	je	0x142340543 <.text+0x233f543>
142340526: 48 8b 95 a0 01 00 00        	movq	0x1a0(%rbp), %rdx
14234052d: 48 c1 e2 03                 	shlq	$0x3, %rdx
142340531: 41 b8 04 00 00 00           	movl	$0x4, %r8d
142340537: 48 8b 8d 88 01 00 00        	movq	0x188(%rbp), %rcx
14234053e: e8 dd 36 2d fe              	callq	0x140613c20 <.text+0x612c20>
142340543: 0f 28 74 24 40              	movaps	0x40(%rsp), %xmm6
142340548: 0f 28 7c 24 30              	movaps	0x30(%rsp), %xmm7
14234054d: 44 0f 28 44 24 20           	movaps	0x20(%rsp), %xmm8
142340553: 48 83 c4 58                 	addq	$0x58, %rsp
142340557: 5b                          	popq	%rbx
142340558: 5f                          	popq	%rdi
142340559: 5e                          	popq	%rsi
14234055a: 41 5c                       	popq	%r12
14234055c: 41 5d                       	popq	%r13
14234055e: 41 5e                       	popq	%r14
142340560: 41 5f                       	popq	%r15
142340562: 5d                          	popq	%rbp
142340563: c3                          	retq
142340564: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
142340570: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
142340575: 55                          	pushq	%rbp
142340576: 41 57                       	pushq	%r15
142340578: 41 56                       	pushq	%r14
14234057a: 41 55                       	pushq	%r13
14234057c: 41 54                       	pushq	%r12
14234057e: 56                          	pushq	%rsi
14234057f: 57                          	pushq	%rdi
142340580: 53                          	pushq	%rbx
142340581: 48 83 ec 58                 	subq	$0x58, %rsp
142340585: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14234058c: 44 0f 29 44 24 20           	movaps	%xmm8, 0x20(%rsp)
142340592: 0f 29 7c 24 30              	movaps	%xmm7, 0x30(%rsp)
142340597: 0f 29 74 24 40              	movaps	%xmm6, 0x40(%rsp)
14234059c: 48 8b 95 20 01 00 00        	movq	0x120(%rbp), %rdx
1423405a3: 48 85 d2                    	testq	%rdx, %rdx
1423405a6: 74 16                       	je	0x1423405be <.text+0x233f5be>
1423405a8: 48 8b 8d 28 01 00 00        	movq	0x128(%rbp), %rcx
1423405af: 48 c1 e2 03                 	shlq	$0x3, %rdx
1423405b3: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1423405b9: e8 62 36 2d fe              	callq	0x140613c20 <.text+0x612c20>
1423405be: c6 85 b7 01 00 00 01        	movb	$0x1, 0x1b7(%rbp)
1423405c5: 0f 28 74 24 40              	movaps	0x40(%rsp), %xmm6
1423405ca: 0f 28 7c 24 30              	movaps	0x30(%rsp), %xmm7
1423405cf: 44 0f 28 44 24 20           	movaps	0x20(%rsp), %xmm8
1423405d5: 48 83 c4 58                 	addq	$0x58, %rsp
1423405d9: 5b                          	popq	%rbx
1423405da: 5f                          	popq	%rdi
1423405db: 5e                          	popq	%rsi
1423405dc: 41 5c                       	popq	%r12
1423405de: 41 5d                       	popq	%r13
1423405e0: 41 5e                       	popq	%r14
1423405e2: 41 5f                       	popq	%r15
1423405e4: 5d                          	popq	%rbp
1423405e5: c3                          	retq
1423405e6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1423405f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1423405f5: 55                          	pushq	%rbp
1423405f6: 41 57                       	pushq	%r15
1423405f8: 41 56                       	pushq	%r14
1423405fa: 41 55                       	pushq	%r13
1423405fc: 41 54                       	pushq	%r12
1423405fe: 56                          	pushq	%rsi
1423405ff: 57                          	pushq	%rdi
142340600: 53                          	pushq	%rbx
142340601: 48 83 ec 58                 	subq	$0x58, %rsp
142340605: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14234060c: 44 0f 29 44 24 20           	movaps	%xmm8, 0x20(%rsp)
142340612: 0f 29 7c 24 30              	movaps	%xmm7, 0x30(%rsp)
142340617: 0f 29 74 24 40              	movaps	%xmm6, 0x40(%rsp)
14234061c: 80 bd b7 01 00 00 00        	cmpb	$0x0, 0x1b7(%rbp)
142340623: 75 21                       	jne	0x142340646 <.text+0x233f646>
142340625: 0f 28 74 24 40              	movaps	0x40(%rsp), %xmm6
14234062a: 0f 28 7c 24 30              	movaps	0x30(%rsp), %xmm7
14234062f: 44 0f 28 44 24 20           	movaps	0x20(%rsp), %xmm8
142340635: 48 83 c4 58                 	addq	$0x58, %rsp
142340639: 5b                          	popq	%rbx
14234063a: 5f                          	popq	%rdi
14234063b: 5e                          	popq	%rsi
14234063c: 41 5c                       	popq	%r12
14234063e: 41 5d                       	popq	%r13
142340640: 41 5e                       	popq	%r14
142340642: 41 5f                       	popq	%r15
142340644: 5d                          	popq	%rbp
142340645: c3                          	retq
142340646: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
14234064a: e8 21 96 e0 fd              	callq	0x140149c70 <.text+0x148c70>
14234064f: eb d4                       	jmp	0x142340625 <.text+0x233f625>
