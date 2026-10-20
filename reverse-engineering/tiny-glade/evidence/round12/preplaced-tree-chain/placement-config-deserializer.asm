
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141416520: 55                          	pushq	%rbp
141416521: 41 57                       	pushq	%r15
141416523: 41 56                       	pushq	%r14
141416525: 41 55                       	pushq	%r13
141416527: 41 54                       	pushq	%r12
141416529: 56                          	pushq	%rsi
14141652a: 57                          	pushq	%rdi
14141652b: 53                          	pushq	%rbx
14141652c: 48 81 ec 38 01 00 00        	subq	$0x138, %rsp            # imm = 0x138
141416533: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14141653b: 0f 29 b5 a0 00 00 00        	movaps	%xmm6, 0xa0(%rbp)
141416542: 48 c7 85 98 00 00 00 fe ff ff ff    	movq	$-0x2, 0x98(%rbp)
14141654d: 48 89 d7                    	movq	%rdx, %rdi
141416550: 48 89 4d 20                 	movq	%rcx, 0x20(%rbp)
141416554: 4c 8b 52 20                 	movq	0x20(%rdx), %r10
141416558: 48 8b 42 28                 	movq	0x28(%rdx), %rax
14141655c: 4c 39 d0                    	cmpq	%r10, %rax
14141655f: 73 45                       	jae	0x1414165a6 <.text+0x14155a6>
141416561: 48 8d 5f 18                 	leaq	0x18(%rdi), %rbx
141416565: 48 8b 57 18                 	movq	0x18(%rdi), %rdx
141416569: 48 ff c0                    	incq	%rax
14141656c: 4d 89 d0                    	movq	%r10, %r8
14141656f: 49 f7 d8                    	negq	%r8
141416572: 49 bf 00 26 00 00 01 00 00 00       	movabsq	$0x100002600, %r15 # imm = 0x100002600
14141657c: 0f 1f 40 00                 	nopl	(%rax)
141416580: 44 0f b6 4c 02 ff           	movzbl	-0x1(%rdx,%rax), %r9d
141416586: 49 83 f9 20                 	cmpq	$0x20, %r9
14141658a: 77 33                       	ja	0x1414165bf <.text+0x14155bf>
14141658c: 4d 0f a3 cf                 	btq	%r9, %r15
141416590: 73 2d                       	jae	0x1414165bf <.text+0x14155bf>
141416592: 48 89 47 28                 	movq	%rax, 0x28(%rdi)
141416596: 4d 8d 0c 00                 	leaq	(%r8,%rax), %r9
14141659a: 49 ff c1                    	incq	%r9
14141659d: 48 ff c0                    	incq	%rax
1414165a0: 49 83 f9 01                 	cmpq	$0x1, %r9
1414165a4: 75 da                       	jne	0x141416580 <.text+0x1415580>
1414165a6: 48 c7 45 d0 05 00 00 00     	movq	$0x5, -0x30(%rbp)
1414165ae: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
1414165b2: 48 89 f9                    	movq	%rdi, %rcx
1414165b5: e8 66 a3 48 01              	callq	0x1428a0920 <.text+0x289f920>
1414165ba: e9 3e 0e 00 00              	jmp	0x1414173fd <.text+0x14163fd>
1414165bf: 41 83 f9 5b                 	cmpl	$0x5b, %r9d
1414165c3: 0f 84 53 05 00 00           	je	0x141416b1c <.text+0x1415b1c>
1414165c9: 41 83 f9 7b                 	cmpl	$0x7b, %r9d
1414165cd: 0f 85 e7 08 00 00           	jne	0x141416eba <.text+0x1415eba>
1414165d3: fe 4f 30                    	decb	0x30(%rdi)
1414165d6: 0f 84 57 0f 00 00           	je	0x141417533 <.text+0x1416533>
1414165dc: 48 89 47 28                 	movq	%rax, 0x28(%rdi)
1414165e0: 41 b5 01                    	movb	$0x1, %r13b
1414165e3: 31 f6                       	xorl	%esi, %esi
1414165e5: 49 b8 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %r8 # imm = 0x8000000000000001
1414165ef: 49 be 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %r14 # imm = 0x8000000000000000
1414165f9: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
1414165fd: 4c 89 b5 88 00 00 00        	movq	%r14, 0x88(%rbp)
141416604: 4c 89 b5 80 00 00 00        	movq	%r14, 0x80(%rbp)
14141660b: 4c 89 45 70                 	movq	%r8, 0x70(%rbp)
14141660f: 4c 39 d0                    	cmpq	%r10, %rax
141416612: 0f 83 6f 04 00 00           	jae	0x141416a87 <.text+0x1415a87>
141416618: 48 8b 13                    	movq	(%rbx), %rdx
14141661b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
141416620: 44 0f b6 04 02              	movzbl	(%rdx,%rax), %r8d
141416625: 49 83 f8 2c                 	cmpq	$0x2c, %r8
141416629: 77 67                       	ja	0x141416692 <.text+0x1415692>
14141662b: 4d 0f a3 c7                 	btq	%r8, %r15
14141662f: 73 11                       	jae	0x141416642 <.text+0x1415642>
141416631: 48 ff c0                    	incq	%rax
141416634: 48 89 47 28                 	movq	%rax, 0x28(%rdi)
141416638: 49 39 c2                    	cmpq	%rax, %r10
14141663b: 75 e3                       	jne	0x141416620 <.text+0x1415620>
14141663d: e9 45 04 00 00              	jmp	0x141416a87 <.text+0x1415a87>
141416642: 49 83 f8 2c                 	cmpq	$0x2c, %r8
141416646: 75 4a                       	jne	0x141416692 <.text+0x1415692>
141416648: 41 f6 c5 01                 	testb	$0x1, %r13b
14141664c: 0f 85 09 0a 00 00           	jne	0x14141705b <.text+0x141605b>
141416652: 48 ff c0                    	incq	%rax
141416655: 48 89 47 28                 	movq	%rax, 0x28(%rdi)
141416659: 4c 39 d0                    	cmpq	%r10, %rax
14141665c: 0f 83 d4 05 00 00           	jae	0x141416c36 <.text+0x1415c36>
141416662: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
141416670: 44 0f b6 04 02              	movzbl	(%rdx,%rax), %r8d
141416675: 49 83 f8 20                 	cmpq	$0x20, %r8
141416679: 77 2e                       	ja	0x1414166a9 <.text+0x14156a9>
14141667b: 4d 0f a3 c7                 	btq	%r8, %r15
14141667f: 73 28                       	jae	0x1414166a9 <.text+0x14156a9>
141416681: 48 ff c0                    	incq	%rax
141416684: 48 89 47 28                 	movq	%rax, 0x28(%rdi)
141416688: 49 39 c2                    	cmpq	%rax, %r10
14141668b: 75 e3                       	jne	0x141416670 <.text+0x1415670>
14141668d: e9 a4 05 00 00              	jmp	0x141416c36 <.text+0x1415c36>
141416692: 41 83 f8 7d                 	cmpl	$0x7d, %r8d
141416696: 0f 84 41 05 00 00           	je	0x141416bdd <.text+0x1415bdd>
14141669c: 41 f6 c5 01                 	testb	$0x1, %r13b
1414166a0: 0f 84 9a 0e 00 00           	je	0x141417540 <.text+0x1416540>
1414166a6: 45 31 ed                    	xorl	%r13d, %r13d
1414166a9: 41 80 f8 22                 	cmpb	$0x22, %r8b
1414166ad: 0f 85 61 09 00 00           	jne	0x141417014 <.text+0x1416014>
1414166b3: 48 ff c0                    	incq	%rax
1414166b6: 48 89 47 28                 	movq	%rax, 0x28(%rdi)
1414166ba: 48 c7 47 10 00 00 00 00     	movq	$0x0, 0x10(%rdi)
1414166c2: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
1414166c9: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
1414166cd: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
1414166d4: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
1414166d8: 48 8b 45 70                 	movq	0x70(%rbp), %rax
1414166dc: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
1414166e0: c6 85 97 00 00 00 01        	movb	$0x1, 0x97(%rbp)
1414166e7: 48 89 da                    	movq	%rbx, %rdx
1414166ea: 49 89 f8                    	movq	%rdi, %r8
1414166ed: e8 1e 42 77 ff              	callq	0x140b8a910 <.text+0xb89910>
1414166f2: 83 7d d0 02                 	cmpl	$0x2, -0x30(%rbp)
1414166f6: 4c 8b 65 d8                 	movq	-0x28(%rbp), %r12
1414166fa: 0f 84 c3 03 00 00           	je	0x141416ac3 <.text+0x1415ac3>
141416700: 4c 8b 45 e0                 	movq	-0x20(%rbp), %r8
141416704: 48 8d 4d 30                 	leaq	0x30(%rbp), %rcx
141416708: 4c 89 e2                    	movq	%r12, %rdx
14141670b: e8 10 df 5f ff              	callq	0x140a14620 <.text+0xa13620>
141416710: f6 45 30 01                 	testb	$0x1, 0x30(%rbp)
141416714: 0f 85 7f 0a 00 00           	jne	0x141417199 <.text+0x1416199>
14141671a: 0f b6 45 31                 	movzbl	0x31(%rbp), %eax
14141671e: 48 8d 0d 4b 73 84 01        	leaq	0x184734b(%rip), %rcx   # 0x142c5da70
141416725: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
141416729: 48 01 c8                    	addq	%rcx, %rax
14141672c: ff e0                       	jmpq	*%rax
14141672e: 48 8b 8d 88 00 00 00        	movq	0x88(%rbp), %rcx
141416735: 48 89 c8                    	movq	%rcx, %rax
141416738: 48 f7 d8                    	negq	%rax
14141673b: 0f 81 ab 0e 00 00           	jno	0x1414175ec <.text+0x14165ec>
141416741: 4c 8b 4f 20                 	movq	0x20(%rdi), %r9
141416745: 48 8b 47 28                 	movq	0x28(%rdi), %rax
141416749: 4c 39 c8                    	cmpq	%r9, %rax
14141674c: 48 8d 4d 30                 	leaq	0x30(%rbp), %rcx
141416750: 0f 83 da 0c 00 00           	jae	0x141417430 <.text+0x1416430>
141416756: 48 8b 13                    	movq	(%rbx), %rdx
141416759: 48 ff c0                    	incq	%rax
14141675c: 49 f7 d9                    	negq	%r9
14141675f: 44 0f b6 44 02 ff           	movzbl	-0x1(%rdx,%rax), %r8d
141416765: 49 83 f8 3a                 	cmpq	$0x3a, %r8
141416769: 0f 87 ec 0e 00 00           	ja	0x14141765b <.text+0x141665b>
14141676f: 4d 0f a3 c7                 	btq	%r8, %r15
141416773: 0f 83 43 02 00 00           	jae	0x1414169bc <.text+0x14159bc>
141416779: 48 89 47 28                 	movq	%rax, 0x28(%rdi)
14141677d: 4d 8d 04 01                 	leaq	(%r9,%rax), %r8
141416781: 49 ff c0                    	incq	%r8
141416784: 48 ff c0                    	incq	%rax
141416787: 49 83 f8 01                 	cmpq	$0x1, %r8
14141678b: 75 d2                       	jne	0x14141675f <.text+0x141575f>
14141678d: e9 9e 0c 00 00              	jmp	0x141417430 <.text+0x1416430>
141416792: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
141416799: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
14141679d: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
1414167a4: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
1414167a8: 48 8b 45 70                 	movq	0x70(%rbp), %rax
1414167ac: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
1414167b0: c6 85 97 00 00 00 01        	movb	$0x1, 0x97(%rbp)
1414167b7: 48 89 f9                    	movq	%rdi, %rcx
1414167ba: e8 41 29 ff ff              	callq	0x141409100 <.text+0x1408100>
1414167bf: 49 89 c4                    	movq	%rax, %r12
1414167c2: 48 85 c0                    	testq	%rax, %rax
1414167c5: 0f 85 f8 02 00 00           	jne	0x141416ac3 <.text+0x1415ac3>
1414167cb: 4c 8b 65 50                 	movq	0x50(%rbp), %r12
1414167cf: e9 9a 02 00 00              	jmp	0x141416a6e <.text+0x1415a6e>
1414167d4: 48 8b 45 70                 	movq	0x70(%rbp), %rax
1414167d8: 49 b9 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %r9 # imm = 0x8000000000000001
1414167e2: 4c 39 c8                    	cmpq	%r9, %rax
1414167e5: 0f 85 93 0d 00 00           	jne	0x14141757e <.text+0x141657e>
1414167eb: 48 8b 4f 20                 	movq	0x20(%rdi), %rcx
1414167ef: 48 8b 47 28                 	movq	0x28(%rdi), %rax
1414167f3: 48 39 c8                    	cmpq	%rcx, %rax
1414167f6: 0f 83 a6 09 00 00           	jae	0x1414171a2 <.text+0x14161a2>
1414167fc: 48 8b 13                    	movq	(%rbx), %rdx
1414167ff: 48 ff c0                    	incq	%rax
141416802: 48 f7 d9                    	negq	%rcx
141416805: 44 0f b6 44 02 ff           	movzbl	-0x1(%rdx,%rax), %r8d
14141680b: 49 83 f8 3a                 	cmpq	$0x3a, %r8
14141680f: 0f 87 06 0f 00 00           	ja	0x14141771b <.text+0x141671b>
141416815: 4d 0f a3 c7                 	btq	%r8, %r15
141416819: 0f 83 d6 00 00 00           	jae	0x1414168f5 <.text+0x14158f5>
14141681f: 48 89 47 28                 	movq	%rax, 0x28(%rdi)
141416823: 4c 8d 04 01                 	leaq	(%rcx,%rax), %r8
141416827: 49 ff c0                    	incq	%r8
14141682a: 48 ff c0                    	incq	%rax
14141682d: 49 83 f8 01                 	cmpq	$0x1, %r8
141416831: 75 d2                       	jne	0x141416805 <.text+0x1415805>
141416833: e9 6a 09 00 00              	jmp	0x1414171a2 <.text+0x14161a2>
141416838: 85 f6                       	testl	%esi, %esi
14141683a: 0f 85 e0 0d 00 00           	jne	0x141417620 <.text+0x1416620>
141416840: 4c 8b 4f 20                 	movq	0x20(%rdi), %r9
141416844: 48 8b 47 28                 	movq	0x28(%rdi), %rax
141416848: 4c 39 c8                    	cmpq	%r9, %rax
14141684b: 48 8d 4d 30                 	leaq	0x30(%rbp), %rcx
14141684f: 0f 83 80 0c 00 00           	jae	0x1414174d5 <.text+0x14164d5>
141416855: 48 8b 13                    	movq	(%rbx), %rdx
141416858: 48 ff c0                    	incq	%rax
14141685b: 49 f7 d9                    	negq	%r9
14141685e: 44 0f b6 44 02 ff           	movzbl	-0x1(%rdx,%rax), %r8d
141416864: 49 83 f8 3a                 	cmpq	$0x3a, %r8
141416868: 0f 87 6f 0e 00 00           	ja	0x1414176dd <.text+0x14166dd>
14141686e: 4d 0f a3 c7                 	btq	%r8, %r15
141416872: 0f 83 ec 00 00 00           	jae	0x141416964 <.text+0x1415964>
141416878: 48 89 47 28                 	movq	%rax, 0x28(%rdi)
14141687c: 4d 8d 04 01                 	leaq	(%r9,%rax), %r8
141416880: 49 ff c0                    	incq	%r8
141416883: 48 ff c0                    	incq	%rax
141416886: 49 83 f8 01                 	cmpq	$0x1, %r8
14141688a: 75 d2                       	jne	0x14141685e <.text+0x141585e>
14141688c: e9 44 0c 00 00              	jmp	0x1414174d5 <.text+0x14164d5>
141416891: 48 8b 8d 80 00 00 00        	movq	0x80(%rbp), %rcx
141416898: 48 89 c8                    	movq	%rcx, %rax
14141689b: 48 f7 d8                    	negq	%rax
14141689e: 0f 81 14 0d 00 00           	jno	0x1414175b8 <.text+0x14165b8>
1414168a4: 4c 8b 4f 20                 	movq	0x20(%rdi), %r9
1414168a8: 48 8b 47 28                 	movq	0x28(%rdi), %rax
1414168ac: 4c 39 c8                    	cmpq	%r9, %rax
1414168af: 48 8d 4d 30                 	leaq	0x30(%rbp), %rcx
1414168b3: 0f 83 c5 0b 00 00           	jae	0x14141747e <.text+0x141647e>
1414168b9: 48 8b 13                    	movq	(%rbx), %rdx
1414168bc: 48 ff c0                    	incq	%rax
1414168bf: 49 f7 d9                    	negq	%r9
1414168c2: 44 0f b6 44 02 ff           	movzbl	-0x1(%rdx,%rax), %r8d
1414168c8: 49 83 f8 3a                 	cmpq	$0x3a, %r8
1414168cc: 0f 87 ca 0d 00 00           	ja	0x14141769c <.text+0x141669c>
1414168d2: 4d 0f a3 c7                 	btq	%r8, %r15
1414168d6: 0f 83 36 01 00 00           	jae	0x141416a12 <.text+0x1415a12>
1414168dc: 48 89 47 28                 	movq	%rax, 0x28(%rdi)
1414168e0: 4d 8d 04 01                 	leaq	(%r9,%rax), %r8
1414168e4: 49 ff c0                    	incq	%r8
1414168e7: 48 ff c0                    	incq	%rax
1414168ea: 49 83 f8 01                 	cmpq	$0x1, %r8
1414168ee: 75 d2                       	jne	0x1414168c2 <.text+0x14158c2>
1414168f0: e9 89 0b 00 00              	jmp	0x14141747e <.text+0x141647e>
1414168f5: 49 83 f8 3a                 	cmpq	$0x3a, %r8
1414168f9: 0f 85 1c 0e 00 00           	jne	0x14141771b <.text+0x141671b>
1414168ff: 48 89 47 28                 	movq	%rax, 0x28(%rdi)
141416903: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
14141690a: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
14141690e: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
141416915: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
141416919: 4c 89 4d 58                 	movq	%r9, 0x58(%rbp)
14141691d: c6 85 97 00 00 00 01        	movb	$0x1, 0x97(%rbp)
141416924: 48 8d 4d 30                 	leaq	0x30(%rbp), %rcx
141416928: 48 89 fa                    	movq	%rdi, %rdx
14141692b: e8 a0 65 63 ff              	callq	0x140a4ced0 <.text+0xa4bed0>
141416930: 48 8b 4d 30                 	movq	0x30(%rbp), %rcx
141416934: 48 8b 45 38                 	movq	0x38(%rbp), %rax
141416938: 48 89 4d 70                 	movq	%rcx, 0x70(%rbp)
14141693c: 48 ba 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rdx # imm = 0x8000000000000001
141416946: 48 39 d1                    	cmpq	%rdx, %rcx
141416949: 0f 84 92 08 00 00           	je	0x1414171e1 <.text+0x14161e1>
14141694f: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
141416953: 48 89 4d b8                 	movq	%rcx, -0x48(%rbp)
141416957: 4c 8b 65 50                 	movq	0x50(%rbp), %r12
14141695b: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
14141695f: e9 0a 01 00 00              	jmp	0x141416a6e <.text+0x1415a6e>
141416964: 49 83 f8 3a                 	cmpq	$0x3a, %r8
141416968: 0f 85 6f 0d 00 00           	jne	0x1414176dd <.text+0x14166dd>
14141696e: 48 89 47 28                 	movq	%rax, 0x28(%rdi)
141416972: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
141416979: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
14141697d: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
141416984: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
141416988: 48 8b 45 70                 	movq	0x70(%rbp), %rax
14141698c: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
141416990: c6 85 97 00 00 00 01        	movb	$0x1, 0x97(%rbp)
141416997: 48 89 fa                    	movq	%rdi, %rdx
14141699a: e8 31 66 ff ff              	callq	0x14140cfd0 <.text+0x140bfd0>
14141699f: f6 45 30 01                 	testb	$0x1, 0x30(%rbp)
1414169a3: 0f 85 f0 07 00 00           	jne	0x141417199 <.text+0x1416199>
1414169a9: f3 0f 10 75 34              	movss	0x34(%rbp), %xmm6
1414169ae: be 01 00 00 00              	movl	$0x1, %esi
1414169b3: 4c 8b 65 50                 	movq	0x50(%rbp), %r12
1414169b7: e9 b2 00 00 00              	jmp	0x141416a6e <.text+0x1415a6e>
1414169bc: 49 83 f8 3a                 	cmpq	$0x3a, %r8
1414169c0: 0f 85 95 0c 00 00           	jne	0x14141765b <.text+0x141665b>
1414169c6: 48 89 47 28                 	movq	%rax, 0x28(%rdi)
1414169ca: 4c 89 75 68                 	movq	%r14, 0x68(%rbp)
1414169ce: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
1414169d5: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
1414169d9: 48 8b 45 70                 	movq	0x70(%rbp), %rax
1414169dd: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
1414169e1: c6 85 97 00 00 00 01        	movb	$0x1, 0x97(%rbp)
1414169e8: 48 89 fa                    	movq	%rdi, %rdx
1414169eb: e8 30 d1 ff ff              	callq	0x141413b20 <.text+0x1412b20>
1414169f0: 48 8b 45 30                 	movq	0x30(%rbp), %rax
1414169f4: 4c 8b 65 38                 	movq	0x38(%rbp), %r12
1414169f8: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
1414169ff: 48 f7 d8                    	negq	%rax
141416a02: 0f 80 67 0a 00 00           	jo	0x14141746f <.text+0x141646f>
141416a08: 48 8b 45 40                 	movq	0x40(%rbp), %rax
141416a0c: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
141416a10: eb 5c                       	jmp	0x141416a6e <.text+0x1415a6e>
141416a12: 49 83 f8 3a                 	cmpq	$0x3a, %r8
141416a16: 0f 85 80 0c 00 00           	jne	0x14141769c <.text+0x141669c>
141416a1c: 48 89 47 28                 	movq	%rax, 0x28(%rdi)
141416a20: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
141416a27: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
141416a2b: 4c 89 75 60                 	movq	%r14, 0x60(%rbp)
141416a2f: 48 8b 45 70                 	movq	0x70(%rbp), %rax
141416a33: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
141416a37: c6 85 97 00 00 00 01        	movb	$0x1, 0x97(%rbp)
141416a3e: 48 89 fa                    	movq	%rdi, %rdx
141416a41: e8 4a c0 ff ff              	callq	0x141412a90 <.text+0x1411a90>
141416a46: 48 8b 4d 30                 	movq	0x30(%rbp), %rcx
141416a4a: 48 8b 45 38                 	movq	0x38(%rbp), %rax
141416a4e: 48 89 8d 80 00 00 00        	movq	%rcx, 0x80(%rbp)
141416a55: 48 f7 d9                    	negq	%rcx
141416a58: 0f 80 5c 0a 00 00           	jo	0x1414174ba <.text+0x14164ba>
141416a5e: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
141416a62: 48 89 4d 28                 	movq	%rcx, 0x28(%rbp)
141416a66: 4c 8b 65 50                 	movq	0x50(%rbp), %r12
141416a6a: 48 89 45 78                 	movq	%rax, 0x78(%rbp)
141416a6e: 4c 8b 57 20                 	movq	0x20(%rdi), %r10
141416a72: 48 8b 47 28                 	movq	0x28(%rdi), %rax
141416a76: 4c 89 65 50                 	movq	%r12, 0x50(%rbp)
141416a7a: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
141416a7e: 4c 39 d0                    	cmpq	%r10, %rax
141416a81: 0f 82 91 fb ff ff           	jb	0x141416618 <.text+0x1415618>
141416a87: 48 c7 45 d0 03 00 00 00     	movq	$0x3, -0x30(%rbp)
141416a8f: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
141416a96: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
141416a9a: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
141416aa1: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
141416aa5: 48 8b 45 70                 	movq	0x70(%rbp), %rax
141416aa9: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
141416aad: c6 85 97 00 00 00 01        	movb	$0x1, 0x97(%rbp)
141416ab4: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
141416ab8: 48 89 f9                    	movq	%rdi, %rcx
141416abb: e8 60 9e 48 01              	callq	0x1428a0920 <.text+0x289f920>
141416ac0: 49 89 c4                    	movq	%rax, %r12
141416ac3: 31 db                       	xorl	%ebx, %ebx
141416ac5: 4c 8b 6d 78                 	movq	0x78(%rbp), %r13
141416ac9: 4c 8b bd 88 00 00 00        	movq	0x88(%rbp), %r15
141416ad0: 48 b8 02 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffe, %rax # imm = 0x8000000000000002
141416ada: 48 8b 55 70                 	movq	0x70(%rbp), %rdx
141416ade: 48 39 c2                    	cmpq	%rax, %rdx
141416ae1: 0f 8c 66 04 00 00           	jl	0x141416f4d <.text+0x1415f4d>
141416ae7: 48 85 d2                    	testq	%rdx, %rdx
141416aea: 0f 84 5d 04 00 00           	je	0x141416f4d <.text+0x1415f4d>
141416af0: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141416af6: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
141416afa: e8 21 d1 1f ff              	callq	0x140613c20 <.text+0x612c20>
141416aff: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
141416b06: 48 8d 04 45 00 00 00 00     	leaq	(,%rax,2), %rax
141416b0e: 48 85 c0                    	testq	%rax, %rax
141416b11: 0f 84 4e 04 00 00           	je	0x141416f65 <.text+0x1415f65>
141416b17: e9 ed 06 00 00              	jmp	0x141417209 <.text+0x1416209>
141416b1c: fe 4f 30                    	decb	0x30(%rdi)
141416b1f: 0f 84 0e 0a 00 00           	je	0x141417533 <.text+0x1416533>
141416b25: 48 89 47 28                 	movq	%rax, 0x28(%rdi)
141416b29: 48 89 7d 30                 	movq	%rdi, 0x30(%rbp)
141416b2d: c6 45 38 01                 	movb	$0x1, 0x38(%rbp)
141416b31: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
141416b35: 48 8d 55 30                 	leaq	0x30(%rbp), %rdx
141416b39: e8 a2 53 ff ff              	callq	0x14140bee0 <.text+0x140aee0>
141416b3e: 48 8b 45 d0                 	movq	-0x30(%rbp), %rax
141416b42: 4c 8b 65 d8                 	movq	-0x28(%rbp), %r12
141416b46: 48 bb 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rbx # imm = 0x8000000000000001
141416b50: 48 39 d8                    	cmpq	%rbx, %rax
141416b53: 75 0f                       	jne	0x141416b64 <.text+0x1415b64>
141416b55: 49 be 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %r14 # imm = 0x8000000000000000
141416b5f: e9 ca 01 00 00              	jmp	0x141416d2e <.text+0x1415d2e>
141416b64: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
141416b6b: 48 f7 d8                    	negq	%rax
141416b6e: 0f 80 9e 03 00 00           	jo	0x141416f12 <.text+0x1415f12>
141416b74: 4c 89 65 50                 	movq	%r12, 0x50(%rbp)
141416b78: 48 8b 45 e0                 	movq	-0x20(%rbp), %rax
141416b7c: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
141416b80: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
141416b84: 48 8d 55 30                 	leaq	0x30(%rbp), %rdx
141416b88: e8 c3 41 ff ff              	callq	0x14140ad50 <.text+0x1409d50>
141416b8d: 48 8b 45 d0                 	movq	-0x30(%rbp), %rax
141416b91: 4c 8b 6d d8                 	movq	-0x28(%rbp), %r13
141416b95: 48 39 d8                    	cmpq	%rbx, %rax
141416b98: 75 0c                       	jne	0x141416ba6 <.text+0x1415ba6>
141416b9a: 48 8b 95 88 00 00 00        	movq	0x88(%rbp), %rdx
141416ba1: e9 67 01 00 00              	jmp	0x141416d0d <.text+0x1415d0d>
141416ba6: 48 89 85 80 00 00 00        	movq	%rax, 0x80(%rbp)
141416bad: 48 f7 d8                    	negq	%rax
141416bb0: 0f 81 be 00 00 00           	jno	0x141416c74 <.text+0x1415c74>
141416bb6: 48 8d 15 0b 68 84 01        	leaq	0x184680b(%rip), %rdx   # 0x142c5d3c8
141416bbd: 4c 8d 05 2c 53 84 01        	leaq	0x184532c(%rip), %r8    # 0x142c5bef0
141416bc4: b9 01 00 00 00              	movl	$0x1, %ecx
141416bc9: e8 52 31 4e 01              	callq	0x1428f9d20 <.text+0x28f8d20>
141416bce: 49 89 c5                    	movq	%rax, %r13
141416bd1: 48 8b 95 88 00 00 00        	movq	0x88(%rbp), %rdx
141416bd8: e9 30 01 00 00              	jmp	0x141416d0d <.text+0x1415d0d>
141416bdd: 4c 8b b5 88 00 00 00        	movq	0x88(%rbp), %r14
141416be4: 4c 89 f0                    	movq	%r14, %rax
141416be7: 48 f7 d8                    	negq	%rax
141416bea: 0f 81 6c 02 00 00           	jno	0x141416e5c <.text+0x1415e5c>
141416bf0: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141416bfa: 49 89 c7                    	movq	%rax, %r15
141416bfd: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
141416c01: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
141416c08: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
141416c0c: 48 8b 45 70                 	movq	0x70(%rbp), %rax
141416c10: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
141416c14: c6 85 97 00 00 00 01        	movb	$0x1, 0x97(%rbp)
141416c1b: 48 8d 0d b6 67 84 01        	leaq	0x18467b6(%rip), %rcx   # 0x142c5d3d8
141416c22: ba 06 00 00 00              	movl	$0x6, %edx
141416c27: e8 94 2f 4e 01              	callq	0x1428f9bc0 <.text+0x28f8bc0>
141416c2c: 49 89 c4                    	movq	%rax, %r12
141416c2f: 31 db                       	xorl	%ebx, %ebx
141416c31: e9 fc 02 00 00              	jmp	0x141416f32 <.text+0x1415f32>
141416c36: 48 c7 45 d0 05 00 00 00     	movq	$0x5, -0x30(%rbp)
141416c3e: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
141416c45: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
141416c49: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
141416c50: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
141416c54: 48 8b 45 70                 	movq	0x70(%rbp), %rax
141416c58: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
141416c5c: c6 85 97 00 00 00 01        	movb	$0x1, 0x97(%rbp)
141416c63: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
141416c67: 48 89 f9                    	movq	%rdi, %rcx
141416c6a: e8 b1 9c 48 01              	callq	0x1428a0920 <.text+0x289f920>
141416c6f: e9 4c fe ff ff              	jmp	0x141416ac0 <.text+0x1415ac0>
141416c74: 48 8b 4d 30                 	movq	0x30(%rbp), %rcx
141416c78: 4c 8b 41 20                 	movq	0x20(%rcx), %r8
141416c7c: 48 8b 41 28                 	movq	0x28(%rcx), %rax
141416c80: 4c 39 c0                    	cmpq	%r8, %rax
141416c83: 4c 89 6d 78                 	movq	%r13, 0x78(%rbp)
141416c87: 73 45                       	jae	0x141416cce <.text+0x1415cce>
141416c89: 4c 8b 65 e0                 	movq	-0x20(%rbp), %r12
141416c8d: 48 8b 51 18                 	movq	0x18(%rcx), %rdx
141416c91: 48 ff c0                    	incq	%rax
141416c94: 4d 89 c2                    	movq	%r8, %r10
141416c97: 49 f7 da                    	negq	%r10
141416c9a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
141416ca0: 44 0f b6 4c 02 ff           	movzbl	-0x1(%rdx,%rax), %r9d
141416ca6: 49 83 f9 2c                 	cmpq	$0x2c, %r9
141416caa: 0f 87 44 03 00 00           	ja	0x141416ff4 <.text+0x1415ff4>
141416cb0: 4d 0f a3 cf                 	btq	%r9, %r15
141416cb4: 0f 83 cd 02 00 00           	jae	0x141416f87 <.text+0x1415f87>
141416cba: 48 89 41 28                 	movq	%rax, 0x28(%rcx)
141416cbe: 4d 8d 0c 02                 	leaq	(%r10,%rax), %r9
141416cc2: 49 ff c1                    	incq	%r9
141416cc5: 48 ff c0                    	incq	%rax
141416cc8: 49 83 f9 01                 	cmpq	$0x1, %r9
141416ccc: 75 d2                       	jne	0x141416ca0 <.text+0x1415ca0>
141416cce: 48 c7 45 d0 02 00 00 00     	movq	$0x2, -0x30(%rbp)
141416cd6: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
141416cda: e8 41 9c 48 01              	callq	0x1428a0920 <.text+0x289f920>
141416cdf: 49 89 c5                    	movq	%rax, %r13
141416ce2: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
141416ce6: 48 8b 95 88 00 00 00        	movq	0x88(%rbp), %rdx
141416ced: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
141416cf4: 48 85 c0                    	testq	%rax, %rax
141416cf7: 74 14                       	je	0x141416d0d <.text+0x1415d0d>
141416cf9: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141416cff: 48 89 d3                    	movq	%rdx, %rbx
141416d02: 48 89 c2                    	movq	%rax, %rdx
141416d05: e8 16 cf 1f ff              	callq	0x140613c20 <.text+0x612c20>
141416d0a: 48 89 da                    	movq	%rbx, %rdx
141416d0d: 49 be 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %r14 # imm = 0x8000000000000000
141416d17: 48 85 d2                    	testq	%rdx, %rdx
141416d1a: 74 0f                       	je	0x141416d2b <.text+0x1415d2b>
141416d1c: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141416d22: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
141416d26: e8 f5 ce 1f ff              	callq	0x140613c20 <.text+0x612c20>
141416d2b: 4d 89 ec                    	movq	%r13, %r12
141416d2e: fe 47 30                    	incb	0x30(%rdi)
141416d31: 4c 89 75 d0                 	movq	%r14, -0x30(%rbp)
141416d35: 4c 89 65 d8                 	movq	%r12, -0x28(%rbp)
141416d39: 48 89 c3                    	movq	%rax, %rbx
141416d3c: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
141416d40: 4c 89 7d e8                 	movq	%r15, -0x18(%rbp)
141416d44: 4c 89 6d 78                 	movq	%r13, 0x78(%rbp)
141416d48: 4c 89 6d f0                 	movq	%r13, -0x10(%rbp)
141416d4c: 48 89 4d 28                 	movq	%rcx, 0x28(%rbp)
141416d50: 48 89 4d f8                 	movq	%rcx, -0x8(%rbp)
141416d54: 4d 89 c5                    	movq	%r8, %r13
141416d57: 4c 89 45 00                 	movq	%r8, (%rbp)
141416d5b: 48 89 55 48                 	movq	%rdx, 0x48(%rbp)
141416d5f: 48 89 55 08                 	movq	%rdx, 0x8(%rbp)
141416d63: 4c 89 4d 10                 	movq	%r9, 0x10(%rbp)
141416d67: f3 0f 11 45 18              	movss	%xmm0, 0x18(%rbp)
141416d6c: 48 89 f9                    	movq	%rdi, %rcx
141416d6f: e8 7c 17 c4 fe              	callq	0x1400584f0 <.text+0x574f0>
141416d74: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
141416d78: 0f 10 01                    	movups	(%rcx), %xmm0
141416d7b: 0f 29 45 30                 	movaps	%xmm0, 0x30(%rbp)
141416d7f: 4c 89 f1                    	movq	%r14, %rcx
141416d82: 48 f7 d9                    	negq	%rcx
141416d85: 71 31                       	jno	0x141416db8 <.text+0x1415db8>
141416d87: 49 be 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %r14 # imm = 0x8000000000000000
141416d91: 48 85 c0                    	testq	%rax, %rax
141416d94: 0f 84 ec 05 00 00           	je	0x141417386 <.text+0x1416386>
141416d9a: 48 8b 08                    	movq	(%rax), %rcx
141416d9d: 48 83 f9 01                 	cmpq	$0x1, %rcx
141416da1: 48 89 85 80 00 00 00        	movq	%rax, 0x80(%rbp)
141416da8: 74 7b                       	je	0x141416e25 <.text+0x1415e25>
141416daa: 48 85 c9                    	testq	%rcx, %rcx
141416dad: 0f 84 34 05 00 00           	je	0x1414172e7 <.text+0x14162e7>
141416db3: e9 1b 06 00 00              	jmp	0x1414173d3 <.text+0x14163d3>
141416db8: 48 85 c0                    	testq	%rax, %rax
141416dbb: 0f 84 bd 05 00 00           	je	0x14141737e <.text+0x141637e>
141416dc1: 4d 85 f6                    	testq	%r14, %r14
141416dc4: 74 17                       	je	0x141416ddd <.text+0x1415ddd>
141416dc6: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141416dcc: 48 89 c6                    	movq	%rax, %rsi
141416dcf: 4c 89 e1                    	movq	%r12, %rcx
141416dd2: 4c 89 f2                    	movq	%r14, %rdx
141416dd5: e8 46 ce 1f ff              	callq	0x140613c20 <.text+0x612c20>
141416dda: 48 89 f0                    	movq	%rsi, %rax
141416ddd: 4d 85 ff                    	testq	%r15, %r15
141416de0: 74 18                       	je	0x141416dfa <.text+0x1415dfa>
141416de2: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141416de8: 48 89 c6                    	movq	%rax, %rsi
141416deb: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
141416def: 4c 89 fa                    	movq	%r15, %rdx
141416df2: e8 29 ce 1f ff              	callq	0x140613c20 <.text+0x612c20>
141416df7: 48 89 f0                    	movq	%rsi, %rax
141416dfa: 49 be 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %r14 # imm = 0x8000000000000000
141416e04: 4a 8d 0c 6d 00 00 00 00     	leaq	(,%r13,2), %rcx
141416e0c: 48 85 c9                    	testq	%rcx, %rcx
141416e0f: 75 25                       	jne	0x141416e36 <.text+0x1415e36>
141416e11: 49 89 c4                    	movq	%rax, %r12
141416e14: 4c 89 f0                    	movq	%r14, %rax
141416e17: 48 f7 d8                    	negq	%rax
141416e1a: 0f 81 6e 05 00 00           	jno	0x14141738e <.text+0x141638e>
141416e20: e9 cd 05 00 00              	jmp	0x1414173f2 <.text+0x14163f2>
141416e25: 48 89 c1                    	movq	%rax, %rcx
141416e28: 48 83 c1 08                 	addq	$0x8, %rcx
141416e2c: e8 cf 0b 1d ff              	callq	0x1405e7a00 <.text+0x5e6a00>
141416e31: e9 9d 05 00 00              	jmp	0x1414173d3 <.text+0x14163d3>
141416e36: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141416e3c: 49 89 c4                    	movq	%rax, %r12
141416e3f: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
141416e43: 4c 89 ea                    	movq	%r13, %rdx
141416e46: e8 d5 cd 1f ff              	callq	0x140613c20 <.text+0x612c20>
141416e4b: 4c 89 f0                    	movq	%r14, %rax
141416e4e: 48 f7 d8                    	negq	%rax
141416e51: 0f 81 37 05 00 00           	jno	0x14141738e <.text+0x141638e>
141416e57: e9 96 05 00 00              	jmp	0x1414173f2 <.text+0x14163f2>
141416e5c: 4c 8b bd 80 00 00 00        	movq	0x80(%rbp), %r15
141416e63: 4c 89 f8                    	movq	%r15, %rax
141416e66: 48 f7 d8                    	negq	%rax
141416e69: 48 8b 5d c0                 	movq	-0x40(%rbp), %rbx
141416e6d: 71 66                       	jno	0x141416ed5 <.text+0x1415ed5>
141416e6f: 48 8d 0d 68 65 84 01        	leaq	0x1846568(%rip), %rcx   # 0x142c5d3de
141416e76: ba 0a 00 00 00              	movl	$0xa, %edx
141416e7b: e8 40 2d 4e 01              	callq	0x1428f9bc0 <.text+0x28f8bc0>
141416e80: 49 89 c4                    	movq	%rax, %r12
141416e83: b3 01                       	movb	$0x1, %bl
141416e85: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141416e8f: 48 89 85 80 00 00 00        	movq	%rax, 0x80(%rbp)
141416e96: 4c 8b bd 88 00 00 00        	movq	0x88(%rbp), %r15
141416e9d: 4d 85 ff                    	testq	%r15, %r15
141416ea0: 0f 84 89 00 00 00           	je	0x141416f2f <.text+0x1415f2f>
141416ea6: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141416eac: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
141416eb0: 4c 89 fa                    	movq	%r15, %rdx
141416eb3: e8 68 cd 1f ff              	callq	0x140613c20 <.text+0x612c20>
141416eb8: eb 78                       	jmp	0x141416f32 <.text+0x1415f32>
141416eba: 4c 8d 05 0f 55 84 01        	leaq	0x184550f(%rip), %r8    # 0x142c5c3d0
141416ec1: 48 8d 55 cf                 	leaq	-0x31(%rbp), %rdx
141416ec5: 48 89 f9                    	movq	%rdi, %rcx
141416ec8: e8 33 1d 4e 01              	callq	0x1428f8c00 <.text+0x28f7c00>
141416ecd: 49 89 c4                    	movq	%rax, %r12
141416ed0: e9 1d 05 00 00              	jmp	0x1414173f2 <.text+0x14163f2>
141416ed5: 48 b8 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rax # imm = 0x8000000000000001
141416edf: 48 8b 4d 70                 	movq	0x70(%rbp), %rcx
141416ee3: 48 39 c1                    	cmpq	%rax, %rcx
141416ee6: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141416ef0: 48 0f 45 c1                 	cmovneq	%rcx, %rax
141416ef4: 48 89 c1                    	movq	%rax, %rcx
141416ef7: 40 f6 c6 01                 	testb	$0x1, %sil
141416efb: 4c 8b 65 50                 	movq	0x50(%rbp), %r12
141416eff: 75 08                       	jne	0x141416f09 <.text+0x1415f09>
141416f01: f3 0f 10 35 fb e9 50 01     	movss	0x150e9fb(%rip), %xmm6  # 0x142925904
141416f09: 4c 8b 6d 78                 	movq	0x78(%rbp), %r13
141416f0d: e9 48 03 00 00              	jmp	0x14141725a <.text+0x141625a>
141416f12: 48 8d 15 af 64 84 01        	leaq	0x18464af(%rip), %rdx   # 0x142c5d3c8
141416f19: 4c 8d 05 d0 4f 84 01        	leaq	0x1844fd0(%rip), %r8    # 0x142c5bef0
141416f20: 31 c9                       	xorl	%ecx, %ecx
141416f22: e8 f9 2d 4e 01              	callq	0x1428f9d20 <.text+0x28f8d20>
141416f27: 49 89 c4                    	movq	%rax, %r12
141416f2a: e9 26 fc ff ff              	jmp	0x141416b55 <.text+0x1415b55>
141416f2f: 45 31 ff                    	xorl	%r15d, %r15d
141416f32: 4c 8b 6d 78                 	movq	0x78(%rbp), %r13
141416f36: 48 b8 02 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffe, %rax # imm = 0x8000000000000002
141416f40: 48 8b 55 70                 	movq	0x70(%rbp), %rdx
141416f44: 48 39 c2                    	cmpq	%rax, %rdx
141416f47: 0f 8d 9a fb ff ff           	jge	0x141416ae7 <.text+0x1415ae7>
141416f4d: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
141416f54: 48 8d 04 45 00 00 00 00     	leaq	(,%rax,2), %rax
141416f5c: 48 85 c0                    	testq	%rax, %rax
141416f5f: 0f 85 a4 02 00 00           	jne	0x141417209 <.text+0x1416209>
141416f65: 49 be 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %r14 # imm = 0x8000000000000000
141416f6f: 84 db                       	testb	%bl, %bl
141416f71: 0f 84 b9 02 00 00           	je	0x141417230 <.text+0x1416230>
141416f77: 4c 8b bd 80 00 00 00        	movq	0x80(%rbp), %r15
141416f7e: 48 8b 5d c0                 	movq	-0x40(%rbp), %rbx
141416f82: e9 d3 02 00 00              	jmp	0x14141725a <.text+0x141625a>
141416f87: 49 83 f9 2c                 	cmpq	$0x2c, %r9
141416f8b: 75 67                       	jne	0x141416ff4 <.text+0x1415ff4>
141416f8d: 44 0f b6 55 38              	movzbl	0x38(%rbp), %r10d
141416f92: 45 84 d2                    	testb	%r10b, %r10b
141416f95: 0f 85 03 01 00 00           	jne	0x14141709e <.text+0x141609e>
141416f9b: 48 89 41 28                 	movq	%rax, 0x28(%rcx)
141416f9f: 4c 39 c0                    	cmpq	%r8, %rax
141416fa2: 73 3a                       	jae	0x141416fde <.text+0x1415fde>
141416fa4: 48 ff c0                    	incq	%rax
141416fa7: 41 ba 01 00 00 00           	movl	$0x1, %r10d
141416fad: 4d 29 c2                    	subq	%r8, %r10
141416fb0: 44 0f b6 4c 02 ff           	movzbl	-0x1(%rdx,%rax), %r9d
141416fb6: 49 83 f9 20                 	cmpq	$0x20, %r9
141416fba: 0f 87 ec 00 00 00           	ja	0x1414170ac <.text+0x14160ac>
141416fc0: 4d 0f a3 cf                 	btq	%r9, %r15
141416fc4: 0f 83 e2 00 00 00           	jae	0x1414170ac <.text+0x14160ac>
141416fca: 48 89 41 28                 	movq	%rax, 0x28(%rcx)
141416fce: 4d 8d 04 02                 	leaq	(%r10,%rax), %r8
141416fd2: 49 ff c0                    	incq	%r8
141416fd5: 48 ff c0                    	incq	%rax
141416fd8: 49 83 f8 02                 	cmpq	$0x2, %r8
141416fdc: 75 d2                       	jne	0x141416fb0 <.text+0x1415fb0>
141416fde: 48 c7 45 d0 05 00 00 00     	movq	$0x5, -0x30(%rbp)
141416fe6: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
141416fea: e8 31 99 48 01              	callq	0x1428a0920 <.text+0x289f920>
141416fef: e9 eb fc ff ff              	jmp	0x141416cdf <.text+0x1415cdf>
141416ff4: 41 83 f9 5d                 	cmpl	$0x5d, %r9d
141416ff8: 0f 85 9b 00 00 00           	jne	0x141417099 <.text+0x1416099>
141416ffe: 41 b6 01                    	movb	$0x1, %r14b
141417001: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14141700b: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
14141700f: e9 f5 00 00 00              	jmp	0x141417109 <.text+0x1416109>
141417014: 41 0f b6 c0                 	movzbl	%r8b, %eax
141417018: 83 f8 7d                    	cmpl	$0x7d, %eax
14141701b: 75 3e                       	jne	0x14141705b <.text+0x141605b>
14141701d: 48 c7 45 d0 12 00 00 00     	movq	$0x12, -0x30(%rbp)
141417025: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
14141702c: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
141417030: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
141417037: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
14141703b: 48 8b 45 70                 	movq	0x70(%rbp), %rax
14141703f: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
141417043: c6 85 97 00 00 00 01        	movb	$0x1, 0x97(%rbp)
14141704a: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
14141704e: 48 89 f9                    	movq	%rdi, %rcx
141417051: e8 ca 98 48 01              	callq	0x1428a0920 <.text+0x289f920>
141417056: e9 65 fa ff ff              	jmp	0x141416ac0 <.text+0x1415ac0>
14141705b: 48 c7 45 d0 10 00 00 00     	movq	$0x10, -0x30(%rbp)
141417063: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
14141706a: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
14141706e: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
141417075: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
141417079: 48 8b 45 70                 	movq	0x70(%rbp), %rax
14141707d: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
141417081: c6 85 97 00 00 00 01        	movb	$0x1, 0x97(%rbp)
141417088: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
14141708c: 48 89 f9                    	movq	%rdi, %rcx
14141708f: e8 8c 98 48 01              	callq	0x1428a0920 <.text+0x289f920>
141417094: e9 27 fa ff ff              	jmp	0x141416ac0 <.text+0x1415ac0>
141417099: 44 0f b6 55 38              	movzbl	0x38(%rbp), %r10d
14141709e: 41 f6 c2 01                 	testb	$0x1, %r10b
1414170a2: 0f 84 d3 06 00 00           	je	0x14141777b <.text+0x141677b>
1414170a8: c6 45 38 00                 	movb	$0x0, 0x38(%rbp)
1414170ac: 48 89 ca                    	movq	%rcx, %rdx
1414170af: 41 80 f9 5d                 	cmpb	$0x5d, %r9b
1414170b3: 0f 84 a6 06 00 00           	je	0x14141775f <.text+0x141675f>
1414170b9: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
1414170bd: e8 0e 5e 63 ff              	callq	0x140a4ced0 <.text+0xa4bed0>
1414170c2: 48 8b 4d d0                 	movq	-0x30(%rbp), %rcx
1414170c6: 48 8b 45 d8                 	movq	-0x28(%rbp), %rax
1414170ca: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
1414170ce: 48 39 d9                    	cmpq	%rbx, %rcx
1414170d1: 75 24                       	jne	0x1414170f7 <.text+0x14160f7>
1414170d3: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
1414170d7: 48 8b 95 88 00 00 00        	movq	0x88(%rbp), %rdx
1414170de: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
1414170e5: 4c 8b 6d 48                 	movq	0x48(%rbp), %r13
1414170e9: 48 85 c0                    	testq	%rax, %rax
1414170ec: 0f 85 07 fc ff ff           	jne	0x141416cf9 <.text+0x1415cf9>
1414170f2: e9 16 fc ff ff              	jmp	0x141416d0d <.text+0x1415d0d>
1414170f7: 48 89 c8                    	movq	%rcx, %rax
1414170fa: 48 8b 75 e0                 	movq	-0x20(%rbp), %rsi
1414170fe: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
141417102: 48 f7 d8                    	negq	%rax
141417105: 41 0f 90 c6                 	seto	%r14b
141417109: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
14141710d: 48 8d 55 30                 	leaq	0x30(%rbp), %rdx
141417111: e8 3a 4f ff ff              	callq	0x14140c050 <.text+0x140b050>
141417116: f6 45 d0 01                 	testb	$0x1, -0x30(%rbp)
14141711a: 74 54                       	je	0x141417170 <.text+0x1416170>
14141711c: 48 8b 5d d8                 	movq	-0x28(%rbp), %rbx
141417120: 4c 8b 4d 28                 	movq	0x28(%rbp), %r9
141417124: 4d 85 c9                    	testq	%r9, %r9
141417127: 0f 94 c0                    	sete	%al
14141712a: 41 08 c6                    	orb	%al, %r14b
14141712d: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
141417131: 48 8b 95 88 00 00 00        	movq	0x88(%rbp), %rdx
141417138: 75 1e                       	jne	0x141417158 <.text+0x1416158>
14141713a: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141417140: 49 89 ce                    	movq	%rcx, %r14
141417143: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
141417147: 49 89 d7                    	movq	%rdx, %r15
14141714a: 4c 89 ca                    	movq	%r9, %rdx
14141714d: e8 ce ca 1f ff              	callq	0x140613c20 <.text+0x612c20>
141417152: 4c 89 fa                    	movq	%r15, %rdx
141417155: 4c 89 f1                    	movq	%r14, %rcx
141417158: 49 89 dd                    	movq	%rbx, %r13
14141715b: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
141417162: 48 85 c0                    	testq	%rax, %rax
141417165: 0f 85 8e fb ff ff           	jne	0x141416cf9 <.text+0x1415cf9>
14141716b: e9 9d fb ff ff              	jmp	0x141416d0d <.text+0x1415d0d>
141417170: 4c 89 e1                    	movq	%r12, %rcx
141417173: f6 45 d4 01                 	testb	$0x1, -0x2c(%rbp)
141417177: 4c 8b 6d 78                 	movq	0x78(%rbp), %r13
14141717b: 4c 8b b5 88 00 00 00        	movq	0x88(%rbp), %r14
141417182: 4c 8b bd 80 00 00 00        	movq	0x80(%rbp), %r15
141417189: 0f 84 84 03 00 00           	je	0x141417513 <.text+0x1416513>
14141718f: f3 0f 10 45 d8              	movss	-0x28(%rbp), %xmm0
141417194: e9 82 03 00 00              	jmp	0x14141751b <.text+0x141651b>
141417199: 4c 8b 65 38                 	movq	0x38(%rbp), %r12
14141719d: e9 21 f9 ff ff              	jmp	0x141416ac3 <.text+0x1415ac3>
1414171a2: 48 c7 45 d0 03 00 00 00     	movq	$0x3, -0x30(%rbp)
1414171aa: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
1414171b1: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
1414171b5: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
1414171bc: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
1414171c0: 48 b8 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rax # imm = 0x8000000000000001
1414171ca: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
1414171ce: c6 85 97 00 00 00 01        	movb	$0x1, 0x97(%rbp)
1414171d5: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
1414171d9: 48 89 f9                    	movq	%rdi, %rcx
1414171dc: e8 3f 97 48 01              	callq	0x1428a0920 <.text+0x289f920>
1414171e1: 31 db                       	xorl	%ebx, %ebx
1414171e3: 49 89 c4                    	movq	%rax, %r12
1414171e6: 4c 8b 6d 78                 	movq	0x78(%rbp), %r13
1414171ea: 4c 8b bd 88 00 00 00        	movq	0x88(%rbp), %r15
1414171f1: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
1414171f8: 48 8d 04 45 00 00 00 00     	leaq	(,%rax,2), %rax
141417200: 48 85 c0                    	testq	%rax, %rax
141417203: 0f 84 5c fd ff ff           	je	0x141416f65 <.text+0x1415f65>
141417209: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14141720f: 4c 89 e9                    	movq	%r13, %rcx
141417212: 48 8b 95 80 00 00 00        	movq	0x80(%rbp), %rdx
141417219: e8 02 ca 1f ff              	callq	0x140613c20 <.text+0x612c20>
14141721e: 49 be 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %r14 # imm = 0x8000000000000000
141417228: 84 db                       	testb	%bl, %bl
14141722a: 0f 85 47 fd ff ff           	jne	0x141416f77 <.text+0x1415f77>
141417230: 4a 8d 04 7d 00 00 00 00     	leaq	(,%r15,2), %rax
141417238: 48 85 c0                    	testq	%rax, %rax
14141723b: 48 8b 5d c0                 	movq	-0x40(%rbp), %rbx
14141723f: 74 12                       	je	0x141417253 <.text+0x1416253>
141417241: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141417247: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
14141724b: 4c 89 fa                    	movq	%r15, %rdx
14141724e: e8 cd c9 1f ff              	callq	0x140613c20 <.text+0x612c20>
141417253: 4c 8b bd 80 00 00 00        	movq	0x80(%rbp), %r15
14141725a: fe 47 30                    	incb	0x30(%rdi)
14141725d: 4c 89 75 d0                 	movq	%r14, -0x30(%rbp)
141417261: 4c 89 65 d8                 	movq	%r12, -0x28(%rbp)
141417265: 48 89 5d e0                 	movq	%rbx, -0x20(%rbp)
141417269: 4c 89 7d e8                 	movq	%r15, -0x18(%rbp)
14141726d: 4c 89 6d 78                 	movq	%r13, 0x78(%rbp)
141417271: 4c 89 6d f0                 	movq	%r13, -0x10(%rbp)
141417275: 48 8b 45 28                 	movq	0x28(%rbp), %rax
141417279: 48 89 45 f8                 	movq	%rax, -0x8(%rbp)
14141727d: 49 89 cd                    	movq	%rcx, %r13
141417280: 48 89 4d 00                 	movq	%rcx, (%rbp)
141417284: 48 8b 45 48                 	movq	0x48(%rbp), %rax
141417288: 48 89 45 08                 	movq	%rax, 0x8(%rbp)
14141728c: 48 8b 45 b8                 	movq	-0x48(%rbp), %rax
141417290: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
141417294: f3 0f 11 75 18              	movss	%xmm6, 0x18(%rbp)
141417299: 48 89 f9                    	movq	%rdi, %rcx
14141729c: e8 af 11 c4 fe              	callq	0x140058450 <.text+0x57450>
1414172a1: 48 89 c6                    	movq	%rax, %rsi
1414172a4: 48 8d 45 10                 	leaq	0x10(%rbp), %rax
1414172a8: 0f 10 00                    	movups	(%rax), %xmm0
1414172ab: 0f 29 45 30                 	movaps	%xmm0, 0x30(%rbp)
1414172af: 4c 89 f0                    	movq	%r14, %rax
1414172b2: 48 f7 d8                    	negq	%rax
1414172b5: 71 5f                       	jno	0x141417316 <.text+0x1416316>
1414172b7: 49 be 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %r14 # imm = 0x8000000000000000
1414172c1: 48 85 f6                    	testq	%rsi, %rsi
1414172c4: 0f 84 bc 00 00 00           	je	0x141417386 <.text+0x1416386>
1414172ca: 48 8b 06                    	movq	(%rsi), %rax
1414172cd: 48 83 f8 01                 	cmpq	$0x1, %rax
1414172d1: 48 89 b5 80 00 00 00        	movq	%rsi, 0x80(%rbp)
1414172d8: 0f 84 e9 00 00 00           	je	0x1414173c7 <.text+0x14163c7>
1414172de: 48 85 c0                    	testq	%rax, %rax
1414172e1: 0f 85 ec 00 00 00           	jne	0x1414173d3 <.text+0x14163d3>
1414172e7: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
1414172ee: 48 8b 50 10                 	movq	0x10(%rax), %rdx
1414172f2: 48 85 d2                    	testq	%rdx, %rdx
1414172f5: 0f 84 d8 00 00 00           	je	0x1414173d3 <.text+0x14163d3>
1414172fb: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
141417302: 48 8b 48 08                 	movq	0x8(%rax), %rcx
141417306: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14141730c: e8 0f c9 1f ff              	callq	0x140613c20 <.text+0x612c20>
141417311: e9 bd 00 00 00              	jmp	0x1414173d3 <.text+0x14163d3>
141417316: 48 85 f6                    	testq	%rsi, %rsi
141417319: 74 63                       	je	0x14141737e <.text+0x141637e>
14141731b: 4d 85 f6                    	testq	%r14, %r14
14141731e: 74 11                       	je	0x141417331 <.text+0x1416331>
141417320: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141417326: 4c 89 e1                    	movq	%r12, %rcx
141417329: 4c 89 f2                    	movq	%r14, %rdx
14141732c: e8 ef c8 1f ff              	callq	0x140613c20 <.text+0x612c20>
141417331: 4d 85 ff                    	testq	%r15, %r15
141417334: 74 12                       	je	0x141417348 <.text+0x1416348>
141417336: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14141733c: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
141417340: 4c 89 fa                    	movq	%r15, %rdx
141417343: e8 d8 c8 1f ff              	callq	0x140613c20 <.text+0x612c20>
141417348: 49 be 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %r14 # imm = 0x8000000000000000
141417352: 4a 8d 04 6d 00 00 00 00     	leaq	(,%r13,2), %rax
14141735a: 48 85 c0                    	testq	%rax, %rax
14141735d: 74 12                       	je	0x141417371 <.text+0x1416371>
14141735f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141417365: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
141417369: 4c 89 ea                    	movq	%r13, %rdx
14141736c: e8 af c8 1f ff              	callq	0x140613c20 <.text+0x612c20>
141417371: 49 89 f4                    	movq	%rsi, %r12
141417374: 4c 89 f0                    	movq	%r14, %rax
141417377: 48 f7 d8                    	negq	%rax
14141737a: 71 12                       	jno	0x14141738e <.text+0x141638e>
14141737c: eb 74                       	jmp	0x1414173f2 <.text+0x14163f2>
14141737e: 0f 28 45 30                 	movaps	0x30(%rbp), %xmm0
141417382: 0f 29 45 a0                 	movaps	%xmm0, -0x60(%rbp)
141417386: 4c 89 f0                    	movq	%r14, %rax
141417389: 48 f7 d8                    	negq	%rax
14141738c: 70 64                       	jo	0x1414173f2 <.text+0x14163f2>
14141738e: 48 8b 4d 20                 	movq	0x20(%rbp), %rcx
141417392: 48 89 59 10                 	movq	%rbx, 0x10(%rcx)
141417396: 4c 89 79 18                 	movq	%r15, 0x18(%rcx)
14141739a: 48 8b 45 78                 	movq	0x78(%rbp), %rax
14141739e: 48 89 41 20                 	movq	%rax, 0x20(%rcx)
1414173a2: 48 8b 45 28                 	movq	0x28(%rbp), %rax
1414173a6: 48 89 41 28                 	movq	%rax, 0x28(%rcx)
1414173aa: 4c 89 69 30                 	movq	%r13, 0x30(%rcx)
1414173ae: 48 8b 45 48                 	movq	0x48(%rbp), %rax
1414173b2: 48 89 41 38                 	movq	%rax, 0x38(%rcx)
1414173b6: 0f 28 45 a0                 	movaps	-0x60(%rbp), %xmm0
1414173ba: 0f 11 41 40                 	movups	%xmm0, 0x40(%rcx)
1414173be: 4c 89 31                    	movq	%r14, (%rcx)
1414173c1: 4c 89 61 08                 	movq	%r12, 0x8(%rcx)
1414173c5: eb 4b                       	jmp	0x141417412 <.text+0x1416412>
1414173c7: 48 89 f1                    	movq	%rsi, %rcx
1414173ca: 48 83 c1 08                 	addq	$0x8, %rcx
1414173ce: e8 2d 06 1d ff              	callq	0x1405e7a00 <.text+0x5e6a00>
1414173d3: ba 28 00 00 00              	movl	$0x28, %edx
1414173d8: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1414173de: 48 8b 8d 80 00 00 00        	movq	0x80(%rbp), %rcx
1414173e5: e8 36 c8 1f ff              	callq	0x140613c20 <.text+0x612c20>
1414173ea: 4c 89 f0                    	movq	%r14, %rax
1414173ed: 48 f7 d8                    	negq	%rax
1414173f0: 71 9c                       	jno	0x14141738e <.text+0x141638e>
1414173f2: 4c 89 e1                    	movq	%r12, %rcx
1414173f5: 48 89 fa                    	movq	%rdi, %rdx
1414173f8: e8 03 ae 48 01              	callq	0x1428a2200 <.text+0x28a1200>
1414173fd: 48 8b 4d 20                 	movq	0x20(%rbp), %rcx
141417401: 48 89 41 08                 	movq	%rax, 0x8(%rcx)
141417405: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14141740f: 48 89 01                    	movq	%rax, (%rcx)
141417412: 48 89 c8                    	movq	%rcx, %rax
141417415: 0f 28 b5 a0 00 00 00        	movaps	0xa0(%rbp), %xmm6
14141741c: 48 81 c4 38 01 00 00        	addq	$0x138, %rsp            # imm = 0x138
141417423: 5b                          	popq	%rbx
141417424: 5f                          	popq	%rdi
141417425: 5e                          	popq	%rsi
141417426: 41 5c                       	popq	%r12
141417428: 41 5d                       	popq	%r13
14141742a: 41 5e                       	popq	%r14
14141742c: 41 5f                       	popq	%r15
14141742e: 5d                          	popq	%rbp
14141742f: c3                          	retq
141417430: 48 c7 45 d0 03 00 00 00     	movq	$0x3, -0x30(%rbp)
141417438: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141417442: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
141417446: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
14141744d: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
141417451: 48 8b 45 70                 	movq	0x70(%rbp), %rax
141417455: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
141417459: c6 85 97 00 00 00 01        	movb	$0x1, 0x97(%rbp)
141417460: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
141417464: 48 89 f9                    	movq	%rdi, %rcx
141417467: e8 b4 94 48 01              	callq	0x1428a0920 <.text+0x289f920>
14141746c: 49 89 c4                    	movq	%rax, %r12
14141746f: 49 bf 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %r15 # imm = 0x8000000000000000
141417479: e9 b1 f7 ff ff              	jmp	0x141416c2f <.text+0x1415c2f>
14141747e: 48 c7 45 d0 03 00 00 00     	movq	$0x3, -0x30(%rbp)
141417486: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
14141748d: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
141417491: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14141749b: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
14141749f: 48 8b 45 70                 	movq	0x70(%rbp), %rax
1414174a3: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
1414174a7: c6 85 97 00 00 00 01        	movb	$0x1, 0x97(%rbp)
1414174ae: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
1414174b2: 48 89 f9                    	movq	%rdi, %rcx
1414174b5: e8 66 94 48 01              	callq	0x1428a0920 <.text+0x289f920>
1414174ba: 48 b9 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rcx # imm = 0x8000000000000000
1414174c4: 48 89 8d 80 00 00 00        	movq	%rcx, 0x80(%rbp)
1414174cb: 31 db                       	xorl	%ebx, %ebx
1414174cd: 49 89 c4                    	movq	%rax, %r12
1414174d0: e9 f0 f5 ff ff              	jmp	0x141416ac5 <.text+0x1415ac5>
1414174d5: 48 c7 45 d0 03 00 00 00     	movq	$0x3, -0x30(%rbp)
1414174dd: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
1414174e4: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
1414174e8: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
1414174ef: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
1414174f3: 48 8b 45 70                 	movq	0x70(%rbp), %rax
1414174f7: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
1414174fb: c6 85 97 00 00 00 01        	movb	$0x1, 0x97(%rbp)
141417502: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
141417506: 48 89 f9                    	movq	%rdi, %rcx
141417509: e8 12 94 48 01              	callq	0x1428a0920 <.text+0x289f920>
14141750e: e9 ad f5 ff ff              	jmp	0x141416ac0 <.text+0x1415ac0>
141417513: f3 0f 10 05 e9 e3 50 01     	movss	0x150e3e9(%rip), %xmm0  # 0x142925904
14141751b: 4c 8b 65 50                 	movq	0x50(%rbp), %r12
14141751f: 48 8b 45 c0                 	movq	-0x40(%rbp), %rax
141417523: 48 8b 55 48                 	movq	0x48(%rbp), %rdx
141417527: 4c 8b 45 28                 	movq	0x28(%rbp), %r8
14141752b: 49 89 f1                    	movq	%rsi, %r9
14141752e: e9 fb f7 ff ff              	jmp	0x141416d2e <.text+0x1415d2e>
141417533: 48 c7 45 d0 15 00 00 00     	movq	$0x15, -0x30(%rbp)
14141753b: e9 6e f0 ff ff              	jmp	0x1414165ae <.text+0x14155ae>
141417540: 48 c7 45 d0 08 00 00 00     	movq	$0x8, -0x30(%rbp)
141417548: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
14141754f: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
141417553: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
14141755a: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
14141755e: 48 8b 45 70                 	movq	0x70(%rbp), %rax
141417562: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
141417566: c6 85 97 00 00 00 01        	movb	$0x1, 0x97(%rbp)
14141756d: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
141417571: 48 89 f9                    	movq	%rdi, %rcx
141417574: e8 a7 93 48 01              	callq	0x1428a0920 <.text+0x289f920>
141417579: e9 42 f5 ff ff              	jmp	0x141416ac0 <.text+0x1415ac0>
14141757e: 48 89 c1                    	movq	%rax, %rcx
141417581: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
141417588: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
14141758c: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
141417593: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
141417597: 48 89 4d 58                 	movq	%rcx, 0x58(%rbp)
14141759b: c6 85 97 00 00 00 01        	movb	$0x1, 0x97(%rbp)
1414175a2: 48 8d 0d 3f 5e 84 01        	leaq	0x1845e3f(%rip), %rcx   # 0x142c5d3e8
1414175a9: ba 10 00 00 00              	movl	$0x10, %edx
1414175ae: e8 fd 27 4e 01              	callq	0x1428f9db0 <.text+0x28f8db0>
1414175b3: e9 08 f5 ff ff              	jmp	0x141416ac0 <.text+0x1415ac0>
1414175b8: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
1414175bf: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
1414175c3: 48 89 4d 60                 	movq	%rcx, 0x60(%rbp)
1414175c7: 48 8b 45 70                 	movq	0x70(%rbp), %rax
1414175cb: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
1414175cf: c6 85 97 00 00 00 01        	movb	$0x1, 0x97(%rbp)
1414175d6: 48 8d 0d 01 5e 84 01        	leaq	0x1845e01(%rip), %rcx   # 0x142c5d3de
1414175dd: ba 0a 00 00 00              	movl	$0xa, %edx
1414175e2: e8 c9 27 4e 01              	callq	0x1428f9db0 <.text+0x28f8db0>
1414175e7: e9 d4 f4 ff ff              	jmp	0x141416ac0 <.text+0x1415ac0>
1414175ec: 48 89 4d 68                 	movq	%rcx, 0x68(%rbp)
1414175f0: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
1414175f7: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
1414175fb: 48 8b 45 70                 	movq	0x70(%rbp), %rax
1414175ff: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
141417603: c6 85 97 00 00 00 01        	movb	$0x1, 0x97(%rbp)
14141760a: 48 8d 0d c7 5d 84 01        	leaq	0x1845dc7(%rip), %rcx   # 0x142c5d3d8
141417611: ba 06 00 00 00              	movl	$0x6, %edx
141417616: e8 95 27 4e 01              	callq	0x1428f9db0 <.text+0x28f8db0>
14141761b: e9 a0 f4 ff ff              	jmp	0x141416ac0 <.text+0x1415ac0>
141417620: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
141417627: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
14141762b: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
141417632: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
141417636: 48 8b 45 70                 	movq	0x70(%rbp), %rax
14141763a: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
14141763e: c6 85 97 00 00 00 01        	movb	$0x1, 0x97(%rbp)
141417645: 48 8d 0d ac 5d 84 01        	leaq	0x1845dac(%rip), %rcx   # 0x142c5d3f8
14141764c: ba 19 00 00 00              	movl	$0x19, %edx
141417651: e8 5a 27 4e 01              	callq	0x1428f9db0 <.text+0x28f8db0>
141417656: e9 65 f4 ff ff              	jmp	0x141416ac0 <.text+0x1415ac0>
14141765b: 48 c7 45 d0 06 00 00 00     	movq	$0x6, -0x30(%rbp)
141417663: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14141766d: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
141417671: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
141417678: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
14141767c: 48 8b 45 70                 	movq	0x70(%rbp), %rax
141417680: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
141417684: c6 85 97 00 00 00 01        	movb	$0x1, 0x97(%rbp)
14141768b: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
14141768f: 48 89 f9                    	movq	%rdi, %rcx
141417692: e8 89 92 48 01              	callq	0x1428a0920 <.text+0x289f920>
141417697: e9 d0 fd ff ff              	jmp	0x14141746c <.text+0x141646c>
14141769c: 48 c7 45 d0 06 00 00 00     	movq	$0x6, -0x30(%rbp)
1414176a4: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
1414176ab: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
1414176af: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1414176b9: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
1414176bd: 48 8b 45 70                 	movq	0x70(%rbp), %rax
1414176c1: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
1414176c5: c6 85 97 00 00 00 01        	movb	$0x1, 0x97(%rbp)
1414176cc: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
1414176d0: 48 89 f9                    	movq	%rdi, %rcx
1414176d3: e8 48 92 48 01              	callq	0x1428a0920 <.text+0x289f920>
1414176d8: e9 dd fd ff ff              	jmp	0x1414174ba <.text+0x14164ba>
1414176dd: 48 c7 45 d0 06 00 00 00     	movq	$0x6, -0x30(%rbp)
1414176e5: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
1414176ec: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
1414176f0: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
1414176f7: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
1414176fb: 48 8b 45 70                 	movq	0x70(%rbp), %rax
1414176ff: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
141417703: c6 85 97 00 00 00 01        	movb	$0x1, 0x97(%rbp)
14141770a: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
14141770e: 48 89 f9                    	movq	%rdi, %rcx
141417711: e8 0a 92 48 01              	callq	0x1428a0920 <.text+0x289f920>
141417716: e9 a5 f3 ff ff              	jmp	0x141416ac0 <.text+0x1415ac0>
14141771b: 48 c7 45 d0 06 00 00 00     	movq	$0x6, -0x30(%rbp)
141417723: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
14141772a: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
14141772e: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
141417735: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
141417739: 48 b8 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rax # imm = 0x8000000000000001
141417743: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
141417747: c6 85 97 00 00 00 01        	movb	$0x1, 0x97(%rbp)
14141774e: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
141417752: 48 89 f9                    	movq	%rdi, %rcx
141417755: e8 c6 91 48 01              	callq	0x1428a0920 <.text+0x289f920>
14141775a: e9 82 fa ff ff              	jmp	0x1414171e1 <.text+0x14161e1>
14141775f: 48 c7 45 d0 12 00 00 00     	movq	$0x12, -0x30(%rbp)
141417767: 48 8d 45 d0                 	leaq	-0x30(%rbp), %rax
14141776b: 48 89 d1                    	movq	%rdx, %rcx
14141776e: 48 89 c2                    	movq	%rax, %rdx
141417771: e8 aa 91 48 01              	callq	0x1428a0920 <.text+0x289f920>
141417776: e9 64 f5 ff ff              	jmp	0x141416cdf <.text+0x1415cdf>
14141777b: 48 c7 45 d0 07 00 00 00     	movq	$0x7, -0x30(%rbp)
141417783: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
141417787: e8 94 91 48 01              	callq	0x1428a0920 <.text+0x289f920>
14141778c: e9 4e f5 ff ff              	jmp	0x141416cdf <.text+0x1415cdf>
141417791: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1414177a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1414177a5: 55                          	pushq	%rbp
1414177a6: 41 57                       	pushq	%r15
1414177a8: 41 56                       	pushq	%r14
1414177aa: 41 55                       	pushq	%r13
1414177ac: 41 54                       	pushq	%r12
1414177ae: 56                          	pushq	%rsi
1414177af: 57                          	pushq	%rdi
1414177b0: 53                          	pushq	%rbx
1414177b1: 48 83 ec 38                 	subq	$0x38, %rsp
1414177b5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1414177bc: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
1414177c1: 48 8b 45 28                 	movq	0x28(%rbp), %rax
1414177c5: 48 8d 04 45 00 00 00 00     	leaq	(,%rax,2), %rax
1414177cd: 48 85 c0                    	testq	%rax, %rax
1414177d0: 74 13                       	je	0x1414177e5 <.text+0x14167e5>
1414177d2: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1414177d8: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
1414177dc: 48 8b 55 28                 	movq	0x28(%rbp), %rdx
1414177e0: e8 3b c4 1f ff              	callq	0x140613c20 <.text+0x612c20>
1414177e5: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
1414177ea: 48 83 c4 38                 	addq	$0x38, %rsp
1414177ee: 5b                          	popq	%rbx
1414177ef: 5f                          	popq	%rdi
1414177f0: 5e                          	popq	%rsi
1414177f1: 41 5c                       	popq	%r12
1414177f3: 41 5d                       	popq	%r13
1414177f5: 41 5e                       	popq	%r14
1414177f7: 41 5f                       	popq	%r15
1414177f9: 5d                          	popq	%rbp
1414177fa: c3                          	retq
1414177fb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
141417800: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141417805: 55                          	pushq	%rbp
141417806: 41 57                       	pushq	%r15
141417808: 41 56                       	pushq	%r14
14141780a: 41 55                       	pushq	%r13
14141780c: 41 54                       	pushq	%r12
14141780e: 56                          	pushq	%rsi
14141780f: 57                          	pushq	%rdi
141417810: 53                          	pushq	%rbx
141417811: 48 83 ec 38                 	subq	$0x38, %rsp
141417815: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14141781c: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
141417821: 48 83 bd 88 00 00 00 00     	cmpq	$0x0, 0x88(%rbp)
141417829: 74 16                       	je	0x141417841 <.text+0x1416841>
14141782b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141417831: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
141417835: 48 8b 95 88 00 00 00        	movq	0x88(%rbp), %rdx
14141783c: e8 df c3 1f ff              	callq	0x140613c20 <.text+0x612c20>
141417841: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
141417848: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
14141784c: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141417856: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
14141785a: 48 8b 45 70                 	movq	0x70(%rbp), %rax
14141785e: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
141417862: c6 85 97 00 00 00 00        	movb	$0x0, 0x97(%rbp)
141417869: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
14141786e: 48 83 c4 38                 	addq	$0x38, %rsp
141417872: 5b                          	popq	%rbx
141417873: 5f                          	popq	%rdi
141417874: 5e                          	popq	%rsi
141417875: 41 5c                       	popq	%r12
141417877: 41 5d                       	popq	%r13
141417879: 41 5e                       	popq	%r14
14141787b: 41 5f                       	popq	%r15
14141787d: 5d                          	popq	%rbp
14141787e: c3                          	retq
14141787f: 90                          	nop
141417880: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141417885: 55                          	pushq	%rbp
141417886: 41 57                       	pushq	%r15
141417888: 41 56                       	pushq	%r14
14141788a: 41 55                       	pushq	%r13
14141788c: 41 54                       	pushq	%r12
14141788e: 56                          	pushq	%rsi
14141788f: 57                          	pushq	%rdi
141417890: 53                          	pushq	%rbx
141417891: 48 83 ec 38                 	subq	$0x38, %rsp
141417895: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14141789c: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
1414178a1: ba 28 00 00 00              	movl	$0x28, %edx
1414178a6: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1414178ac: 48 8b 8d 80 00 00 00        	movq	0x80(%rbp), %rcx
1414178b3: e8 68 c3 1f ff              	callq	0x140613c20 <.text+0x612c20>
1414178b8: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
1414178bd: 48 83 c4 38                 	addq	$0x38, %rsp
1414178c1: 5b                          	popq	%rbx
1414178c2: 5f                          	popq	%rdi
1414178c3: 5e                          	popq	%rsi
1414178c4: 41 5c                       	popq	%r12
1414178c6: 41 5d                       	popq	%r13
1414178c8: 41 5e                       	popq	%r14
1414178ca: 41 5f                       	popq	%r15
1414178cc: 5d                          	popq	%rbp
1414178cd: c3                          	retq
1414178ce: 66 90                       	nop
1414178d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1414178d5: 55                          	pushq	%rbp
1414178d6: 41 57                       	pushq	%r15
1414178d8: 41 56                       	pushq	%r14
1414178da: 41 55                       	pushq	%r13
1414178dc: 41 54                       	pushq	%r12
1414178de: 56                          	pushq	%rsi
1414178df: 57                          	pushq	%rdi
1414178e0: 53                          	pushq	%rbx
1414178e1: 48 83 ec 38                 	subq	$0x38, %rsp
1414178e5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1414178ec: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
1414178f1: ba 28 00 00 00              	movl	$0x28, %edx
1414178f6: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1414178fc: 48 8b 8d 80 00 00 00        	movq	0x80(%rbp), %rcx
141417903: e8 18 c3 1f ff              	callq	0x140613c20 <.text+0x612c20>
141417908: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
14141790d: 48 83 c4 38                 	addq	$0x38, %rsp
141417911: 5b                          	popq	%rbx
141417912: 5f                          	popq	%rdi
141417913: 5e                          	popq	%rsi
141417914: 41 5c                       	popq	%r12
141417916: 41 5d                       	popq	%r13
141417918: 41 5e                       	popq	%r14
14141791a: 41 5f                       	popq	%r15
14141791c: 5d                          	popq	%rbp
14141791d: c3                          	retq
14141791e: 66 90                       	nop
141417920: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141417925: 55                          	pushq	%rbp
141417926: 41 57                       	pushq	%r15
141417928: 41 56                       	pushq	%r14
14141792a: 41 55                       	pushq	%r13
14141792c: 41 54                       	pushq	%r12
14141792e: 56                          	pushq	%rsi
14141792f: 57                          	pushq	%rdi
141417930: 53                          	pushq	%rbx
141417931: 48 83 ec 38                 	subq	$0x38, %rsp
141417935: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14141793c: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
141417941: 48 83 bd 80 00 00 00 00     	cmpq	$0x0, 0x80(%rbp)
141417949: 74 16                       	je	0x141417961 <.text+0x1416961>
14141794b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141417951: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
141417955: 48 8b 95 80 00 00 00        	movq	0x80(%rbp), %rdx
14141795c: e8 bf c2 1f ff              	callq	0x140613c20 <.text+0x612c20>
141417961: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
141417966: 48 83 c4 38                 	addq	$0x38, %rsp
14141796a: 5b                          	popq	%rbx
14141796b: 5f                          	popq	%rdi
14141796c: 5e                          	popq	%rsi
14141796d: 41 5c                       	popq	%r12
14141796f: 41 5d                       	popq	%r13
141417971: 41 5e                       	popq	%r14
141417973: 41 5f                       	popq	%r15
141417975: 5d                          	popq	%rbp
141417976: c3                          	retq
141417977: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
141417980: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141417985: 55                          	pushq	%rbp
141417986: 41 57                       	pushq	%r15
141417988: 41 56                       	pushq	%r14
14141798a: 41 55                       	pushq	%r13
14141798c: 41 54                       	pushq	%r12
14141798e: 56                          	pushq	%rsi
14141798f: 57                          	pushq	%rdi
141417990: 53                          	pushq	%rbx
141417991: 48 83 ec 38                 	subq	$0x38, %rsp
141417995: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14141799c: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
1414179a1: 48 83 bd 88 00 00 00 00     	cmpq	$0x0, 0x88(%rbp)
1414179a9: 74 16                       	je	0x1414179c1 <.text+0x14169c1>
1414179ab: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1414179b1: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
1414179b5: 48 8b 95 88 00 00 00        	movq	0x88(%rbp), %rdx
1414179bc: e8 5f c2 1f ff              	callq	0x140613c20 <.text+0x612c20>
1414179c1: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
1414179c6: 48 83 c4 38                 	addq	$0x38, %rsp
1414179ca: 5b                          	popq	%rbx
1414179cb: 5f                          	popq	%rdi
1414179cc: 5e                          	popq	%rsi
1414179cd: 41 5c                       	popq	%r12
1414179cf: 41 5d                       	popq	%r13
1414179d1: 41 5e                       	popq	%r14
1414179d3: 41 5f                       	popq	%r15
1414179d5: 5d                          	popq	%rbp
1414179d6: c3                          	retq
1414179d7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1414179e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1414179e5: 55                          	pushq	%rbp
1414179e6: 41 57                       	pushq	%r15
1414179e8: 41 56                       	pushq	%r14
1414179ea: 41 55                       	pushq	%r13
1414179ec: 41 54                       	pushq	%r12
1414179ee: 56                          	pushq	%rsi
1414179ef: 57                          	pushq	%rdi
1414179f0: 53                          	pushq	%rbx
1414179f1: 48 83 ec 38                 	subq	$0x38, %rsp
1414179f5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1414179fc: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
141417a01: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
141417a05: e8 36 0b ff ff              	callq	0x141408540 <.text+0x1407540>
141417a0a: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
141417a0f: 48 83 c4 38                 	addq	$0x38, %rsp
141417a13: 5b                          	popq	%rbx
141417a14: 5f                          	popq	%rdi
141417a15: 5e                          	popq	%rsi
141417a16: 41 5c                       	popq	%r12
141417a18: 41 5d                       	popq	%r13
141417a1a: 41 5e                       	popq	%r14
141417a1c: 41 5f                       	popq	%r15
141417a1e: 5d                          	popq	%rbp
141417a1f: c3                          	retq
141417a20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141417a25: 55                          	pushq	%rbp
141417a26: 41 57                       	pushq	%r15
141417a28: 41 56                       	pushq	%r14
141417a2a: 41 55                       	pushq	%r13
141417a2c: 41 54                       	pushq	%r12
141417a2e: 56                          	pushq	%rsi
141417a2f: 57                          	pushq	%rdi
141417a30: 53                          	pushq	%rbx
141417a31: 48 83 ec 38                 	subq	$0x38, %rsp
141417a35: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141417a3c: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
141417a41: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
141417a45: e8 f6 0a ff ff              	callq	0x141408540 <.text+0x1407540>
141417a4a: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
141417a4f: 48 83 c4 38                 	addq	$0x38, %rsp
141417a53: 5b                          	popq	%rbx
141417a54: 5f                          	popq	%rdi
141417a55: 5e                          	popq	%rsi
141417a56: 41 5c                       	popq	%r12
141417a58: 41 5d                       	popq	%r13
141417a5a: 41 5e                       	popq	%r14
141417a5c: 41 5f                       	popq	%r15
141417a5e: 5d                          	popq	%rbp
141417a5f: c3                          	retq
141417a60: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141417a65: 55                          	pushq	%rbp
141417a66: 41 57                       	pushq	%r15
141417a68: 41 56                       	pushq	%r14
141417a6a: 41 55                       	pushq	%r13
141417a6c: 41 54                       	pushq	%r12
141417a6e: 56                          	pushq	%rsi
141417a6f: 57                          	pushq	%rdi
141417a70: 53                          	pushq	%rbx
141417a71: 48 83 ec 38                 	subq	$0x38, %rsp
141417a75: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141417a7c: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
141417a81: 0f b6 85 97 00 00 00        	movzbl	0x97(%rbp), %eax
141417a88: 88 85 88 00 00 00           	movb	%al, 0x88(%rbp)
141417a8e: 48 8b 55 58                 	movq	0x58(%rbp), %rdx
141417a92: 48 8b 45 60                 	movq	0x60(%rbp), %rax
141417a96: 48 89 85 80 00 00 00        	movq	%rax, 0x80(%rbp)
141417a9d: 48 8b 45 68                 	movq	0x68(%rbp), %rax
141417aa1: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
141417aa5: 48 89 d0                    	movq	%rdx, %rax
141417aa8: 48 f7 d8                    	negq	%rax
141417aab: 70 23                       	jo	0x141417ad0 <.text+0x1416ad0>
141417aad: 48 b8 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rax # imm = 0x8000000000000001
141417ab7: 48 39 c2                    	cmpq	%rax, %rdx
141417aba: 74 2a                       	je	0x141417ae6 <.text+0x1416ae6>
141417abc: 48 85 d2                    	testq	%rdx, %rdx
141417abf: 74 0f                       	je	0x141417ad0 <.text+0x1416ad0>
141417ac1: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141417ac7: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
141417acb: e8 50 c1 1f ff              	callq	0x140613c20 <.text+0x612c20>
141417ad0: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
141417ad5: 48 83 c4 38                 	addq	$0x38, %rsp
141417ad9: 5b                          	popq	%rbx
141417ada: 5f                          	popq	%rdi
141417adb: 5e                          	popq	%rsi
141417adc: 41 5c                       	popq	%r12
141417ade: 41 5d                       	popq	%r13
141417ae0: 41 5e                       	popq	%r14
141417ae2: 41 5f                       	popq	%r15
141417ae4: 5d                          	popq	%rbp
141417ae5: c3                          	retq
141417ae6: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
141417aeb: 48 83 c4 38                 	addq	$0x38, %rsp
141417aef: 5b                          	popq	%rbx
141417af0: 5f                          	popq	%rdi
141417af1: 5e                          	popq	%rsi
141417af2: 41 5c                       	popq	%r12
141417af4: 41 5d                       	popq	%r13
141417af6: 41 5e                       	popq	%r14
141417af8: 41 5f                       	popq	%r15
141417afa: 5d                          	popq	%rbp
141417afb: c3                          	retq
141417afc: 0f 1f 40 00                 	nopl	(%rax)
141417b00: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141417b05: 55                          	pushq	%rbp
141417b06: 41 57                       	pushq	%r15
141417b08: 41 56                       	pushq	%r14
141417b0a: 41 55                       	pushq	%r13
141417b0c: 41 54                       	pushq	%r12
141417b0e: 56                          	pushq	%rsi
141417b0f: 57                          	pushq	%rdi
141417b10: 53                          	pushq	%rbx
141417b11: 48 83 ec 38                 	subq	$0x38, %rsp
141417b15: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141417b1c: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
141417b21: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
141417b28: 48 f7 d8                    	negq	%rax
141417b2b: 70 2e                       	jo	0x141417b5b <.text+0x1416b5b>
141417b2d: 73 16                       	jae	0x141417b45 <.text+0x1416b45>
141417b2f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141417b35: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
141417b39: 48 8b 95 80 00 00 00        	movq	0x80(%rbp), %rdx
141417b40: e8 db c0 1f ff              	callq	0x140613c20 <.text+0x612c20>
141417b45: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
141417b4a: 48 83 c4 38                 	addq	$0x38, %rsp
141417b4e: 5b                          	popq	%rbx
141417b4f: 5f                          	popq	%rdi
141417b50: 5e                          	popq	%rsi
141417b51: 41 5c                       	popq	%r12
141417b53: 41 5d                       	popq	%r13
141417b55: 41 5e                       	popq	%r14
141417b57: 41 5f                       	popq	%r15
141417b59: 5d                          	popq	%rbp
141417b5a: c3                          	retq
141417b5b: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
141417b60: 48 83 c4 38                 	addq	$0x38, %rsp
141417b64: 5b                          	popq	%rbx
141417b65: 5f                          	popq	%rdi
141417b66: 5e                          	popq	%rsi
141417b67: 41 5c                       	popq	%r12
141417b69: 41 5d                       	popq	%r13
141417b6b: 41 5e                       	popq	%r14
141417b6d: 41 5f                       	popq	%r15
141417b6f: 5d                          	popq	%rbp
141417b70: c3                          	retq
141417b71: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
141417b80: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141417b85: 55                          	pushq	%rbp
141417b86: 41 57                       	pushq	%r15
141417b88: 41 56                       	pushq	%r14
141417b8a: 41 55                       	pushq	%r13
141417b8c: 41 54                       	pushq	%r12
141417b8e: 56                          	pushq	%rsi
141417b8f: 57                          	pushq	%rdi
141417b90: 53                          	pushq	%rbx
141417b91: 48 83 ec 38                 	subq	$0x38, %rsp
141417b95: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141417b9c: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
141417ba1: 48 8b 45 20                 	movq	0x20(%rbp), %rax
141417ba5: 48 f7 d8                    	negq	%rax
141417ba8: 70 23                       	jo	0x141417bcd <.text+0x1416bcd>
141417baa: f6 85 88 00 00 00 01        	testb	$0x1, 0x88(%rbp)
141417bb1: 74 1a                       	je	0x141417bcd <.text+0x1416bcd>
141417bb3: 48 83 7d 20 00              	cmpq	$0x0, 0x20(%rbp)
141417bb8: 74 13                       	je	0x141417bcd <.text+0x1416bcd>
141417bba: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141417bc0: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
141417bc4: 48 8b 55 20                 	movq	0x20(%rbp), %rdx
141417bc8: e8 53 c0 1f ff              	callq	0x140613c20 <.text+0x612c20>
141417bcd: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
141417bd2: 48 83 c4 38                 	addq	$0x38, %rsp
141417bd6: 5b                          	popq	%rbx
141417bd7: 5f                          	popq	%rdi
141417bd8: 5e                          	popq	%rsi
141417bd9: 41 5c                       	popq	%r12
141417bdb: 41 5d                       	popq	%r13
141417bdd: 41 5e                       	popq	%r14
141417bdf: 41 5f                       	popq	%r15
141417be1: 5d                          	popq	%rbp
141417be2: c3                          	retq
