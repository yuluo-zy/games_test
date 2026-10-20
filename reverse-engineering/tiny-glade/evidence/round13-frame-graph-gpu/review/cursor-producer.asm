
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1409d1710: 55                          	pushq	%rbp
1409d1711: 41 57                       	pushq	%r15
1409d1713: 41 56                       	pushq	%r14
1409d1715: 41 55                       	pushq	%r13
1409d1717: 41 54                       	pushq	%r12
1409d1719: 56                          	pushq	%rsi
1409d171a: 57                          	pushq	%rdi
1409d171b: 53                          	pushq	%rbx
1409d171c: 48 81 ec 58 09 00 00        	subq	$0x958, %rsp            # imm = 0x958
1409d1723: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1409d172b: 48 c7 85 d0 08 00 00 fe ff ff ff    	movq	$-0x2, 0x8d0(%rbp)
1409d1736: 4d 89 c6                    	movq	%r8, %r14
1409d1739: 48 89 d7                    	movq	%rdx, %rdi
1409d173c: 48 89 cb                    	movq	%rcx, %rbx
1409d173f: e8 bc 52 24 00              	callq	0x140c16a00 <.text+0xc15a00>
1409d1744: 84 c0                       	testb	%al, %al
1409d1746: 88 85 c0 08 00 00           	movb	%al, 0x8c0(%rbp)
1409d174c: 74 5f                       	je	0x1409d17ad <.text+0x9d07ad>
1409d174e: 4c 8d 3d f0 fc 0e 02        	leaq	0x20efcf0(%rip), %r15   # 0x142ac1445
1409d1755: be 57 00 00 00              	movl	$0x57, %esi
1409d175a: ba 57 00 00 00              	movl	$0x57, %edx
1409d175f: 4c 89 f9                    	movq	%r15, %rcx
1409d1762: e8 99 c0 ff ff              	callq	0x1409cd800 <.text+0x9cc800>
1409d1767: 48 83 f8 01                 	cmpq	$0x1, %rax
1409d176b: 0f 85 b1 00 00 00           	jne	0x1409d1822 <.text+0x9d0822>
1409d1771: 49 89 d1                    	movq	%rdx, %r9
1409d1774: 48 85 d2                    	testq	%rdx, %rdx
1409d1777: 74 3d                       	je	0x1409d17b6 <.text+0x9d07b6>
1409d1779: 49 83 f9 57                 	cmpq	$0x57, %r9
1409d177d: 73 35                       	jae	0x1409d17b4 <.text+0x9d07b4>
1409d177f: 48 8d 05 bf fc 0e 02        	leaq	0x20efcbf(%rip), %rax   # 0x142ac1445
1409d1786: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
1409d178b: 7f 29                       	jg	0x1409d17b6 <.text+0x9d07b6>
1409d178d: 48 8d 05 8c fd 0e 02        	leaq	0x20efd8c(%rip), %rax   # 0x142ac1520
1409d1794: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1409d1799: 48 8d 0d a5 fc 0e 02        	leaq	0x20efca5(%rip), %rcx   # 0x142ac1445
1409d17a0: ba 57 00 00 00              	movl	$0x57, %edx
1409d17a5: 45 31 c0                    	xorl	%r8d, %r8d
1409d17a8: e8 93 81 f0 01              	callq	0x1428d9940 <.text+0x28d8940>
1409d17ad: 31 c0                       	xorl	%eax, %eax
1409d17af: e9 d2 01 00 00              	jmp	0x1409d1986 <.text+0x9d0986>
1409d17b4: 75 d7                       	jne	0x1409d178d <.text+0x9d078d>
1409d17b6: 4c 8d 3d 88 fc 0e 02        	leaq	0x20efc88(%rip), %r15   # 0x142ac1445
1409d17bd: 4c 89 f9                    	movq	%r15, %rcx
1409d17c0: 4c 89 ca                    	movq	%r9, %rdx
1409d17c3: e8 38 c0 ff ff              	callq	0x1409cd800 <.text+0x9cc800>
1409d17c8: 48 83 f8 01                 	cmpq	$0x1, %rax
1409d17cc: 75 54                       	jne	0x1409d1822 <.text+0x9d0822>
1409d17ce: 49 89 d0                    	movq	%rdx, %r8
1409d17d1: 49 83 c0 02                 	addq	$0x2, %r8
1409d17d5: 74 39                       	je	0x1409d1810 <.text+0x9d0810>
1409d17d7: 49 83 f8 57                 	cmpq	$0x57, %r8
1409d17db: 73 31                       	jae	0x1409d180e <.text+0x9d080e>
1409d17dd: 48 8d 05 61 fc 0e 02        	leaq	0x20efc61(%rip), %rax   # 0x142ac1445
1409d17e4: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
1409d17e9: 7f 25                       	jg	0x1409d1810 <.text+0x9d0810>
1409d17eb: 48 8d 05 46 fd 0e 02        	leaq	0x20efd46(%rip), %rax   # 0x142ac1538
1409d17f2: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1409d17f7: 48 8d 0d 47 fc 0e 02        	leaq	0x20efc47(%rip), %rcx   # 0x142ac1445
1409d17fe: ba 57 00 00 00              	movl	$0x57, %edx
1409d1803: 41 b9 57 00 00 00           	movl	$0x57, %r9d
1409d1809: e8 32 81 f0 01              	callq	0x1428d9940 <.text+0x28d8940>
1409d180e: 75 db                       	jne	0x1409d17eb <.text+0x9d07eb>
1409d1810: be 55 00 00 00              	movl	$0x55, %esi
1409d1815: 48 29 d6                    	subq	%rdx, %rsi
1409d1818: 4c 8d 3d 26 fc 0e 02        	leaq	0x20efc26(%rip), %r15   # 0x142ac1445
1409d181f: 4d 01 c7                    	addq	%r8, %r15
1409d1822: 48 8d 05 6c ff 0e 02        	leaq	0x20eff6c(%rip), %rax   # 0x142ac1795
1409d1829: 4c 8d 25 18 ff 0e 02        	leaq	0x20eff18(%rip), %r12   # 0x142ac1748
1409d1830: 0f be 48 ff                 	movsbl	-0x1(%rax), %ecx
1409d1834: 85 c9                       	testl	%ecx, %ecx
1409d1836: 78 18                       	js	0x1409d1850 <.text+0x9d0850>
1409d1838: 48 ff c8                    	decq	%rax
1409d183b: 83 f9 5c                    	cmpl	$0x5c, %ecx
1409d183e: 75 69                       	jne	0x1409d18a9 <.text+0x9d08a9>
1409d1840: eb 7d                       	jmp	0x1409d18bf <.text+0x9d08bf>
1409d1842: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1409d1850: 0f b6 50 fe                 	movzbl	-0x2(%rax), %edx
1409d1854: 80 fa c0                    	cmpb	$-0x40, %dl
1409d1857: 7d 25                       	jge	0x1409d187e <.text+0x9d087e>
1409d1859: 44 0f b6 40 fd              	movzbl	-0x3(%rax), %r8d
1409d185e: 41 80 f8 c0                 	cmpb	$-0x40, %r8b
1409d1862: 7d 23                       	jge	0x1409d1887 <.text+0x9d0887>
1409d1864: 44 0f b6 48 fc              	movzbl	-0x4(%rax), %r9d
1409d1869: 48 83 c0 fc                 	addq	$-0x4, %rax
1409d186d: 41 83 e1 07                 	andl	$0x7, %r9d
1409d1871: 41 c1 e1 06                 	shll	$0x6, %r9d
1409d1875: 41 83 e0 3f                 	andl	$0x3f, %r8d
1409d1879: 45 09 c8                    	orl	%r9d, %r8d
1409d187c: eb 11                       	jmp	0x1409d188f <.text+0x9d088f>
1409d187e: 48 83 c0 fe                 	addq	$-0x2, %rax
1409d1882: 83 e2 1f                    	andl	$0x1f, %edx
1409d1885: eb 12                       	jmp	0x1409d1899 <.text+0x9d0899>
1409d1887: 48 83 c0 fd                 	addq	$-0x3, %rax
1409d188b: 41 83 e0 0f                 	andl	$0xf, %r8d
1409d188f: 41 c1 e0 06                 	shll	$0x6, %r8d
1409d1893: 83 e2 3f                    	andl	$0x3f, %edx
1409d1896: 44 09 c2                    	orl	%r8d, %edx
1409d1899: c1 e2 06                    	shll	$0x6, %edx
1409d189c: 80 e1 3f                    	andb	$0x3f, %cl
1409d189f: 0f b6 c9                    	movzbl	%cl, %ecx
1409d18a2: 09 d1                       	orl	%edx, %ecx
1409d18a4: 83 f9 5c                    	cmpl	$0x5c, %ecx
1409d18a7: 74 16                       	je	0x1409d18bf <.text+0x9d08bf>
1409d18a9: 83 f9 2f                    	cmpl	$0x2f, %ecx
1409d18ac: 74 11                       	je	0x1409d18bf <.text+0x9d08bf>
1409d18ae: 4c 39 e0                    	cmpq	%r12, %rax
1409d18b1: 0f 85 79 ff ff ff           	jne	0x1409d1830 <.text+0x9d0830>
1409d18b7: 41 bd 4d 00 00 00           	movl	$0x4d, %r13d
1409d18bd: eb 50                       	jmp	0x1409d190f <.text+0x9d090f>
1409d18bf: 4c 8d 25 82 fe 0e 02        	leaq	0x20efe82(%rip), %r12   # 0x142ac1748
1409d18c6: 4c 29 e0                    	subq	%r12, %rax
1409d18c9: 49 89 c0                    	movq	%rax, %r8
1409d18cc: 49 ff c0                    	incq	%r8
1409d18cf: 74 32                       	je	0x1409d1903 <.text+0x9d0903>
1409d18d1: 49 83 f8 4d                 	cmpq	$0x4d, %r8
1409d18d5: 73 2a                       	jae	0x1409d1901 <.text+0x9d0901>
1409d18d7: 43 80 3c 20 bf              	cmpb	$-0x41, (%r8,%r12)
1409d18dc: 7f 25                       	jg	0x1409d1903 <.text+0x9d0903>
1409d18de: 48 8d 05 1b fc 0e 02        	leaq	0x20efc1b(%rip), %rax   # 0x142ac1500
1409d18e5: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1409d18ea: 48 8d 0d 57 fe 0e 02        	leaq	0x20efe57(%rip), %rcx   # 0x142ac1748
1409d18f1: ba 4d 00 00 00              	movl	$0x4d, %edx
1409d18f6: 41 b9 4d 00 00 00           	movl	$0x4d, %r9d
1409d18fc: e8 3f 80 f0 01              	callq	0x1428d9940 <.text+0x28d8940>
1409d1901: 75 db                       	jne	0x1409d18de <.text+0x9d08de>
1409d1903: 41 bd 4c 00 00 00           	movl	$0x4c, %r13d
1409d1909: 49 29 c5                    	subq	%rax, %r13
1409d190c: 4d 01 c4                    	addq	%r8, %r12
1409d190f: e8 1c 68 24 00              	callq	0x140c18130 <.text+0xc17130>
1409d1914: 48 89 c1                    	movq	%rax, %rcx
1409d1917: 48 8b 00                    	movq	(%rax), %rax
1409d191a: 48 83 f8 01                 	cmpq	$0x1, %rax
1409d191e: 0f 85 01 05 00 00           	jne	0x1409d1e25 <.text+0x9d0e25>
1409d1924: 48 83 c1 08                 	addq	$0x8, %rcx
1409d1928: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1409d192c: 0f 85 2d 05 00 00           	jne	0x1409d1e5f <.text+0x9d0e5f>
1409d1932: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1409d1939: 48 89 8d b8 08 00 00        	movq	%rcx, 0x8b8(%rbp)
1409d1940: 48 83 c1 08                 	addq	$0x8, %rcx
1409d1944: 4c 89 6c 24 20              	movq	%r13, 0x20(%rsp)
1409d1949: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
1409d1952: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
1409d195b: 4c 89 fa                    	movq	%r15, %rdx
1409d195e: 49 89 f0                    	movq	%rsi, %r8
1409d1961: 4d 89 e1                    	movq	%r12, %r9
1409d1964: e8 d7 53 24 00              	callq	0x140c16d40 <.text+0xc15d40>
1409d1969: 48 8b 8d b8 08 00 00        	movq	0x8b8(%rbp), %rcx
1409d1970: 48 ff 01                    	incq	(%rcx)
1409d1973: 48 89 85 b8 08 00 00        	movq	%rax, 0x8b8(%rbp)
1409d197a: 48 89 85 98 08 00 00        	movq	%rax, 0x898(%rbp)
1409d1981: b8 01 00 00 00              	movl	$0x1, %eax
1409d1986: 48 89 85 90 08 00 00        	movq	%rax, 0x890(%rbp)
1409d198d: 4c 8d 67 38                 	leaq	0x38(%rdi), %r12
1409d1991: 48 ba b3 61 b4 27 07 ee 97 85       	movabsq	$-0x7a6811f8d84b9e4d, %rdx # imm = 0x8597EE0727B461B3
1409d199b: 49 b8 5c f1 15 ae c5 7e 29 8b       	movabsq	$-0x74d6813a51ea0ea4, %r8 # imm = 0x8B297EC5AE15F15C
1409d19a5: 4c 89 e1                    	movq	%r12, %rcx
1409d19a8: e8 43 bd 70 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
1409d19ad: 48 85 c0                    	testq	%rax, %rax
1409d19b0: 0f 84 b5 04 00 00           	je	0x1409d1e6b <.text+0x9d0e6b>
1409d19b6: 48 39 97 18 02 00 00        	cmpq	%rdx, 0x218(%rdi)
1409d19bd: 0f 86 a8 04 00 00           	jbe	0x1409d1e6b <.text+0x9d0e6b>
1409d19c3: 48 8b 87 10 02 00 00        	movq	0x210(%rdi), %rax
1409d19ca: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
1409d19ce: 48 85 c9                    	testq	%rcx, %rcx
1409d19d1: 0f 84 94 04 00 00           	je	0x1409d1e6b <.text+0x9d0e6b>
1409d19d7: 48 8b 87 e0 01 00 00        	movq	0x1e0(%rdi), %rax
1409d19de: 48 f7 d1                    	notq	%rcx
1409d19e1: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1409d19e5: 48 c1 e1 05                 	shlq	$0x5, %rcx
1409d19e9: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
1409d19ef: 0f 84 76 04 00 00           	je	0x1409d1e6b <.text+0x9d0e6b>
1409d19f5: 48 01 c8                    	addq	%rcx, %rax
1409d19f8: 48 8b 48 10                 	movq	0x10(%rax), %rcx
1409d19fc: 48 8d 15 ad fd 0e 02        	leaq	0x20efdad(%rip), %rdx   # 0x142ac17b0
1409d1a03: 41 b8 0d 00 00 00           	movl	$0xd, %r8d
1409d1a09: e8 92 60 f6 ff              	callq	0x140937aa0 <.text+0x936aa0>
1409d1a0e: a8 01                       	testb	$0x1, %al
1409d1a10: 0f 84 7f 04 00 00           	je	0x1409d1e95 <.text+0x9d0e95>
1409d1a16: 49 89 d7                    	movq	%rdx, %r15
1409d1a19: 48 ba a6 12 23 8c a9 dc 4d 86       	movabsq	$-0x79b2235673dced5a, %rdx # imm = 0x864DDCA98C2312A6
1409d1a23: 49 b8 4e 48 9a 9e 36 e9 b0 27       	movabsq	$0x27b0e9369e9a484e, %r8 # imm = 0x27B0E9369E9A484E
1409d1a2d: 4c 89 e1                    	movq	%r12, %rcx
1409d1a30: e8 bb bc 70 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
1409d1a35: 48 85 c0                    	testq	%rax, %rax
1409d1a38: 0f 84 3b 04 00 00           	je	0x1409d1e79 <.text+0x9d0e79>
1409d1a3e: 48 39 97 18 02 00 00        	cmpq	%rdx, 0x218(%rdi)
1409d1a45: 0f 86 2e 04 00 00           	jbe	0x1409d1e79 <.text+0x9d0e79>
1409d1a4b: 48 8b 87 10 02 00 00        	movq	0x210(%rdi), %rax
1409d1a52: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
1409d1a56: 48 85 c9                    	testq	%rcx, %rcx
1409d1a59: 0f 84 1a 04 00 00           	je	0x1409d1e79 <.text+0x9d0e79>
1409d1a5f: 48 8b 87 e0 01 00 00        	movq	0x1e0(%rdi), %rax
1409d1a66: 48 f7 d1                    	notq	%rcx
1409d1a69: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1409d1a6d: 48 c1 e1 05                 	shlq	$0x5, %rcx
1409d1a71: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
1409d1a77: 0f 84 fc 03 00 00           	je	0x1409d1e79 <.text+0x9d0e79>
1409d1a7d: 48 01 c8                    	addq	%rcx, %rax
1409d1a80: 48 8b 40 10                 	movq	0x10(%rax), %rax
1409d1a84: 8b 30                       	movl	(%rax), %esi
1409d1a86: 44 8b 68 04                 	movl	0x4(%rax), %r13d
1409d1a8a: 48 ba 40 7c d4 87 e7 56 7b 3d       	movabsq	$0x3d7b56e787d47c40, %rdx # imm = 0x3D7B56E787D47C40
1409d1a94: 49 b8 c7 bf 43 8d 0a b1 b0 67       	movabsq	$0x67b0b10a8d43bfc7, %r8 # imm = 0x67B0B10A8D43BFC7
1409d1a9e: 4c 89 e1                    	movq	%r12, %rcx
1409d1aa1: e8 4a bc 70 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
1409d1aa6: 48 85 c0                    	testq	%rax, %rax
1409d1aa9: 0f 84 d8 03 00 00           	je	0x1409d1e87 <.text+0x9d0e87>
1409d1aaf: 48 39 97 18 02 00 00        	cmpq	%rdx, 0x218(%rdi)
1409d1ab6: 0f 86 cb 03 00 00           	jbe	0x1409d1e87 <.text+0x9d0e87>
1409d1abc: 48 8b 87 10 02 00 00        	movq	0x210(%rdi), %rax
1409d1ac3: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
1409d1ac7: 48 85 c9                    	testq	%rcx, %rcx
1409d1aca: 0f 84 b7 03 00 00           	je	0x1409d1e87 <.text+0x9d0e87>
1409d1ad0: 48 8b 87 e0 01 00 00        	movq	0x1e0(%rdi), %rax
1409d1ad7: 48 f7 d1                    	notq	%rcx
1409d1ada: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1409d1ade: 48 c1 e1 05                 	shlq	$0x5, %rcx
1409d1ae2: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
1409d1ae8: 0f 84 99 03 00 00           	je	0x1409d1e87 <.text+0x9d0e87>
1409d1aee: 48 01 c8                    	addq	%rcx, %rax
1409d1af1: 48 8b 40 10                 	movq	0x10(%rax), %rax
1409d1af5: 80 b8 f4 00 00 00 00        	cmpb	$0x0, 0xf4(%rax)
1409d1afc: 0f 84 c3 02 00 00           	je	0x1409d1dc5 <.text+0x9d0dc5>
1409d1b02: 49 8d 4e 04                 	leaq	0x4(%r14), %rcx
1409d1b06: e8 85 3c 20 00              	callq	0x140bd5790 <.text+0xbd4790>
1409d1b0b: c7 85 b0 03 00 00 00 00 00 00       	movl	$0x0, 0x3b0(%rbp)
1409d1b15: 4c 89 b5 b8 03 00 00        	movq	%r14, 0x3b8(%rbp)
1409d1b1c: c7 85 98 07 00 00 02 00 00 00       	movl	$0x2, 0x798(%rbp)
1409d1b26: 48 89 5c 24 40              	movq	%rbx, 0x40(%rsp)
1409d1b2b: 48 89 7c 24 38              	movq	%rdi, 0x38(%rsp)
1409d1b30: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
1409d1b35: 48 8d 85 98 07 00 00        	leaq	0x798(%rbp), %rax
1409d1b3c: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
1409d1b41: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1409d1b4a: 48 8d 15 b7 fc 0e 02        	leaq	0x20efcb7(%rip), %rdx   # 0x142ac1808
1409d1b51: 48 8d 4d c8                 	leaq	-0x38(%rbp), %rcx
1409d1b55: 4c 8d 8d b0 03 00 00        	leaq	0x3b0(%rbp), %r9
1409d1b5c: 41 b8 0d 00 00 00           	movl	$0xd, %r8d
1409d1b62: e8 09 5e e9 ff              	callq	0x140867970 <.text+0x866970>
1409d1b67: c6 85 cf 08 00 00 01        	movb	$0x1, 0x8cf(%rbp)
1409d1b6e: e8 dd 4b 21 00              	callq	0x140be6750 <.text+0xbe5750>
1409d1b73: 89 c7                       	movl	%eax, %edi
1409d1b75: c6 85 cf 08 00 00 01        	movb	$0x1, 0x8cf(%rbp)
1409d1b7c: 31 c9                       	xorl	%ecx, %ecx
1409d1b7e: e8 9d 7f 70 ff              	callq	0x1400d9b20 <.text+0xd8b20>
1409d1b83: 48 85 c0                    	testq	%rax, %rax
1409d1b86: 0f 84 17 03 00 00           	je	0x1409d1ea3 <.text+0x9d0ea3>
1409d1b8c: 89 bd 40 08 00 00           	movl	%edi, 0x840(%rbp)
1409d1b92: 66 c7 85 48 08 00 00 00 00  	movw	$0x0, 0x848(%rbp)
1409d1b9b: c7 85 44 08 00 00 00 00 80 3f       	movl	$0x3f800000, 0x844(%rbp) # imm = 0x3F800000
1409d1ba5: 48 c7 85 98 07 00 00 00 00 00 00    	movq	$0x0, 0x798(%rbp)
1409d1bb0: 4c 89 bd 38 08 00 00        	movq	%r15, 0x838(%rbp)
1409d1bb7: 48 c7 85 a8 07 00 00 00 00 00 00    	movq	$0x0, 0x7a8(%rbp)
1409d1bc2: 48 c7 85 b0 07 00 00 04 00 00 00    	movq	$0x4, 0x7b0(%rbp)
1409d1bcd: 48 c7 85 b8 07 00 00 00 00 00 00    	movq	$0x0, 0x7b8(%rbp)
1409d1bd8: 0f 10 00                    	movups	(%rax), %xmm0
1409d1bdb: 48 ff 00                    	incq	(%rax)
1409d1bde: 0f 10 0d 4b 4b 0c 02        	movups	0x20c4b4b(%rip), %xmm1  # 0x142a96730
1409d1be5: 0f 11 8d 18 08 00 00        	movups	%xmm1, 0x818(%rbp)
1409d1bec: 0f 10 0d 2d 4b 0c 02        	movups	0x20c4b2d(%rip), %xmm1  # 0x142a96720
1409d1bf3: 0f 11 8d 08 08 00 00        	movups	%xmm1, 0x808(%rbp)
1409d1bfa: 0f 11 85 28 08 00 00        	movups	%xmm0, 0x828(%rbp)
1409d1c01: 48 b8 03 00 00 00 01 00 00 00       	movabsq	$0x100000003, %rax # imm = 0x100000003
1409d1c0b: 48 89 85 d8 07 00 00        	movq	%rax, 0x7d8(%rbp)
1409d1c12: 48 c7 85 c0 07 00 00 00 00 00 00    	movq	$0x0, 0x7c0(%rbp)
1409d1c1d: 48 c7 85 c8 07 00 00 01 00 00 00    	movq	$0x1, 0x7c8(%rbp)
1409d1c28: 48 c7 85 d0 07 00 00 00 00 00 00    	movq	$0x0, 0x7d0(%rbp)
1409d1c33: c6 85 cf 08 00 00 01        	movb	$0x1, 0x8cf(%rbp)
1409d1c3a: 48 8d 8d b0 03 00 00        	leaq	0x3b0(%rbp), %rcx
1409d1c41: 48 8d 95 98 07 00 00        	leaq	0x798(%rbp), %rdx
1409d1c48: 4c 8d 45 c8                 	leaq	-0x38(%rbp), %r8
1409d1c4c: 45 31 c9                    	xorl	%r9d, %r9d
1409d1c4f: e8 8c 3f f3 ff              	callq	0x140905be0 <.text+0x904be0>
1409d1c54: 80 bd 8e 04 00 00 08        	cmpb	$0x8, 0x48e(%rbp)
1409d1c5b: 74 6f                       	je	0x1409d1ccc <.text+0x9d0ccc>
1409d1c5d: 0f 57 c0                    	xorps	%xmm0, %xmm0
1409d1c60: f3 48 0f 2a c6              	cvtsi2ss	%rsi, %xmm0
1409d1c65: 0f 57 c9                    	xorps	%xmm1, %xmm1
1409d1c68: f3 49 0f 2a cd              	cvtsi2ss	%r13, %xmm1
1409d1c6d: f3 48 0f 2c c1              	cvttss2si	%xmm1, %rax
1409d1c72: 31 c9                       	xorl	%ecx, %ecx
1409d1c74: 0f 57 d2                    	xorps	%xmm2, %xmm2
1409d1c77: 0f 2e ca                    	ucomiss	%xmm2, %xmm1
1409d1c7a: 0f 42 c1                    	cmovbl	%ecx, %eax
1409d1c7d: f3 0f 10 1d 4b eb fc 01     	movss	0x1fceb4b(%rip), %xmm3  # 0x1429a07d0
1409d1c85: 0f 2e cb                    	ucomiss	%xmm3, %xmm1
1409d1c88: ba ff ff ff ff              	movl	$0xffffffff, %edx       # imm = 0xFFFFFFFF
1409d1c8d: 0f 47 c2                    	cmoval	%edx, %eax
1409d1c90: 48 c1 e0 20                 	shlq	$0x20, %rax
1409d1c94: f3 4c 0f 2c c8              	cvttss2si	%xmm0, %r9
1409d1c99: 0f 2e c2                    	ucomiss	%xmm2, %xmm0
1409d1c9c: 44 0f 42 c9                 	cmovbl	%ecx, %r9d
1409d1ca0: 0f 2e c3                    	ucomiss	%xmm3, %xmm0
1409d1ca3: 44 0f 47 ca                 	cmoval	%edx, %r9d
1409d1ca7: 49 09 c1                    	orq	%rax, %r9
1409d1caa: 48 8d 8d 68 04 00 00        	leaq	0x468(%rbp), %rcx
1409d1cb1: ba e8 00 00 00              	movl	$0xe8, %edx
1409d1cb6: 48 03 95 98 04 00 00        	addq	0x498(%rbp), %rdx
1409d1cbd: 49 b8 59 05 86 b7 25 06 5a 0a       	movabsq	$0xa5a0625b7860559, %r8 # imm = 0xA5A0625B7860559
1409d1cc7: e8 94 9a f9 ff              	callq	0x14096b760 <.text+0x96a760>
1409d1ccc: 48 8d bd 98 07 00 00        	leaq	0x798(%rbp), %rdi
1409d1cd3: 48 8d 9d b0 03 00 00        	leaq	0x3b0(%rbp), %rbx
1409d1cda: 41 b8 f8 00 00 00           	movl	$0xf8, %r8d
1409d1ce0: 48 89 f9                    	movq	%rdi, %rcx
1409d1ce3: 48 89 da                    	movq	%rbx, %rdx
1409d1ce6: e8 15 1f f5 01              	callq	0x142923c00 <.text+0x2922c00>
1409d1ceb: 48 b8 80 e7 c4 bc 14 4d f1 f9       	movabsq	$-0x60eb2eb433b1880, %rax # imm = 0xF9F14D14BCC4E780
1409d1cf5: 48 89 85 a0 08 00 00        	movq	%rax, 0x8a0(%rbp)
1409d1cfc: c7 85 b0 03 00 00 02 00 00 00       	movl	$0x2, 0x3b0(%rbp)
1409d1d06: 48 89 5c 24 20              	movq	%rbx, 0x20(%rsp)
1409d1d0b: 48 8d 8d a0 08 00 00        	leaq	0x8a0(%rbp), %rcx
1409d1d12: 49 b8 29 a4 db e3 8e 22 0d 9f       	movabsq	$-0x60f2dd711c245bd7, %r8 # imm = 0x9F0D228EE3DBA429
1409d1d1c: 48 89 fa                    	movq	%rdi, %rdx
1409d1d1f: 41 b1 1d                    	movb	$0x1d, %r9b
1409d1d22: e8 e9 68 f3 ff              	callq	0x140908610 <.text+0x907610>
1409d1d27: 48 8d bd b0 03 00 00        	leaq	0x3b0(%rbp), %rdi
1409d1d2e: 48 8d 95 98 07 00 00        	leaq	0x798(%rbp), %rdx
1409d1d35: 41 b8 f8 00 00 00           	movl	$0xf8, %r8d
1409d1d3b: 48 89 f9                    	movq	%rdi, %rcx
1409d1d3e: e8 bd 1e f5 01              	callq	0x142923c00 <.text+0x2922c00>
1409d1d43: c6 85 cf 08 00 00 01        	movb	$0x1, 0x8cf(%rbp)
1409d1d4a: 48 8d 8d 98 07 00 00        	leaq	0x798(%rbp), %rcx
1409d1d51: 49 b8 0d f0 ce 29 8f 7a ce 19       	movabsq	$0x19ce7a8f29cef00d, %r8 # imm = 0x19CE7A8F29CEF00D
1409d1d5b: 48 89 fa                    	movq	%rdi, %rdx
1409d1d5e: e8 2d 8d c5 00              	callq	0x14162aa90 <.text+0x1629a90>
1409d1d63: 80 a5 40 08 00 00 7f        	andb	$0x7f, 0x840(%rbp)
1409d1d6a: 0f b6 85 43 08 00 00        	movzbl	0x843(%rbp), %eax
1409d1d71: 24 9f                       	andb	$-0x61, %al
1409d1d73: 0c 40                       	orb	$0x40, %al
1409d1d75: 88 85 43 08 00 00           	movb	%al, 0x843(%rbp)
1409d1d7b: 48 8d 95 98 07 00 00        	leaq	0x798(%rbp), %rdx
1409d1d82: 41 b8 f8 00 00 00           	movl	$0xf8, %r8d
1409d1d88: 48 89 f9                    	movq	%rdi, %rcx
1409d1d8b: e8 70 1e f5 01              	callq	0x142923c00 <.text+0x2922c00>
1409d1d90: c6 85 cf 08 00 00 01        	movb	$0x1, 0x8cf(%rbp)
1409d1d97: ba 06 00 00 00              	movl	$0x6, %edx
1409d1d9c: 48 89 f9                    	movq	%rdi, %rcx
1409d1d9f: e8 ec 4c f3 ff              	callq	0x140906a90 <.text+0x905a90>
1409d1da4: 48 8d 55 c8                 	leaq	-0x38(%rbp), %rdx
1409d1da8: 41 b8 e8 03 00 00           	movl	$0x3e8, %r8d            # imm = 0x3E8
1409d1dae: 48 89 f9                    	movq	%rdi, %rcx
1409d1db1: e8 4a 1e f5 01              	callq	0x142923c00 <.text+0x2922c00>
1409d1db6: c6 85 cf 08 00 00 00        	movb	$0x0, 0x8cf(%rbp)
1409d1dbd: 48 89 f9                    	movq	%rdi, %rcx
1409d1dc0: e8 ab 65 e9 ff              	callq	0x140868370 <.text+0x867370>
1409d1dc5: 80 bd c0 08 00 00 00        	cmpb	$0x0, 0x8c0(%rbp)
1409d1dcc: 74 43                       	je	0x1409d1e11 <.text+0x9d0e11>
1409d1dce: e8 5d 63 24 00              	callq	0x140c18130 <.text+0xc17130>
1409d1dd3: 48 89 c1                    	movq	%rax, %rcx
1409d1dd6: 48 8b 00                    	movq	(%rax), %rax
1409d1dd9: 48 83 f8 01                 	cmpq	$0x1, %rax
1409d1ddd: 75 60                       	jne	0x1409d1e3f <.text+0x9d0e3f>
1409d1ddf: 48 83 c1 08                 	addq	$0x8, %rcx
1409d1de3: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1409d1de7: 75 76                       	jne	0x1409d1e5f <.text+0x9d0e5f>
1409d1de9: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1409d1df0: 48 89 8d c0 08 00 00        	movq	%rcx, 0x8c0(%rbp)
1409d1df7: 48 83 c1 08                 	addq	$0x8, %rcx
1409d1dfb: 48 8b 95 b8 08 00 00        	movq	0x8b8(%rbp), %rdx
1409d1e02: e8 e9 52 24 00              	callq	0x140c170f0 <.text+0xc160f0>
1409d1e07: 48 8b 85 c0 08 00 00        	movq	0x8c0(%rbp), %rax
1409d1e0e: 48 ff 00                    	incq	(%rax)
1409d1e11: 48 81 c4 58 09 00 00        	addq	$0x958, %rsp            # imm = 0x958
1409d1e18: 5b                          	popq	%rbx
1409d1e19: 5f                          	popq	%rdi
1409d1e1a: 5e                          	popq	%rsi
1409d1e1b: 41 5c                       	popq	%r12
1409d1e1d: 41 5d                       	popq	%r13
1409d1e1f: 41 5e                       	popq	%r14
1409d1e21: 41 5f                       	popq	%r15
1409d1e23: 5d                          	popq	%rbp
1409d1e24: c3                          	retq
1409d1e25: 48 85 c0                    	testq	%rax, %rax
1409d1e28: 75 29                       	jne	0x1409d1e53 <.text+0x9d0e53>
1409d1e2a: 31 d2                       	xorl	%edx, %edx
1409d1e2c: e8 9f 69 f2 01              	callq	0x1428f87d0 <.text+0x28f77d0>
1409d1e31: 48 89 c1                    	movq	%rax, %rcx
1409d1e34: 48 85 c0                    	testq	%rax, %rax
1409d1e37: 0f 85 eb fa ff ff           	jne	0x1409d1928 <.text+0x9d0928>
1409d1e3d: eb 14                       	jmp	0x1409d1e53 <.text+0x9d0e53>
1409d1e3f: 48 85 c0                    	testq	%rax, %rax
1409d1e42: 75 0f                       	jne	0x1409d1e53 <.text+0x9d0e53>
1409d1e44: 31 d2                       	xorl	%edx, %edx
1409d1e46: e8 85 69 f2 01              	callq	0x1428f87d0 <.text+0x28f77d0>
1409d1e4b: 48 89 c1                    	movq	%rax, %rcx
1409d1e4e: 48 85 c0                    	testq	%rax, %rax
1409d1e51: 75 90                       	jne	0x1409d1de3 <.text+0x9d0de3>
1409d1e53: 48 8d 0d 46 8c 26 02        	leaq	0x2268c46(%rip), %rcx   # 0x142c3aaa0
1409d1e5a: e8 71 62 f0 01              	callq	0x1428d80d0 <.text+0x28d70d0>
1409d1e5f: 48 8d 0d 0a 8d 26 02        	leaq	0x2268d0a(%rip), %rcx   # 0x142c3ab70
1409d1e66: e8 e5 73 f0 01              	callq	0x1428d9250 <.text+0x28d8250>
1409d1e6b: 48 8d 0d 26 f9 0e 02        	leaq	0x20ef926(%rip), %rcx   # 0x142ac1798
1409d1e72: e8 99 74 f0 01              	callq	0x1428d9310 <.text+0x28d8310>
1409d1e77: eb 3d                       	jmp	0x1409d1eb6 <.text+0x9d0eb6>
1409d1e79: 48 8d 0d 58 f9 0e 02        	leaq	0x20ef958(%rip), %rcx   # 0x142ac17d8
1409d1e80: e8 8b 74 f0 01              	callq	0x1428d9310 <.text+0x28d8310>
1409d1e85: eb 2f                       	jmp	0x1409d1eb6 <.text+0x9d0eb6>
1409d1e87: 48 8d 0d 62 f9 0e 02        	leaq	0x20ef962(%rip), %rcx   # 0x142ac17f0
1409d1e8e: e8 7d 74 f0 01              	callq	0x1428d9310 <.text+0x28d8310>
1409d1e93: eb 21                       	jmp	0x1409d1eb6 <.text+0x9d0eb6>
1409d1e95: 48 8d 0d 24 f9 0e 02        	leaq	0x20ef924(%rip), %rcx   # 0x142ac17c0
1409d1e9c: e8 6f 74 f0 01              	callq	0x1428d9310 <.text+0x28d8310>
1409d1ea1: eb 13                       	jmp	0x1409d1eb6 <.text+0x9d0eb6>
1409d1ea3: c6 85 cf 08 00 00 01        	movb	$0x1, 0x8cf(%rbp)
1409d1eaa: 48 8d 0d ef 8b 26 02        	leaq	0x2268bef(%rip), %rcx   # 0x142c3aaa0
1409d1eb1: e8 1a 62 f0 01              	callq	0x1428d80d0 <.text+0x28d70d0>
1409d1eb6: 0f 0b                       	ud2
1409d1eb8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1409d1ec0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409d1ec5: 55                          	pushq	%rbp
1409d1ec6: 41 57                       	pushq	%r15
1409d1ec8: 41 56                       	pushq	%r14
1409d1eca: 41 55                       	pushq	%r13
1409d1ecc: 41 54                       	pushq	%r12
1409d1ece: 56                          	pushq	%rsi
1409d1ecf: 57                          	pushq	%rdi
1409d1ed0: 53                          	pushq	%rbx
1409d1ed1: 48 83 ec 48                 	subq	$0x48, %rsp
1409d1ed5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409d1edc: 48 8b 85 b8 08 00 00        	movq	0x8b8(%rbp), %rax
1409d1ee3: 48 ff 00                    	incq	(%rax)
1409d1ee6: 48 83 c4 48                 	addq	$0x48, %rsp
1409d1eea: 5b                          	popq	%rbx
1409d1eeb: 5f                          	popq	%rdi
1409d1eec: 5e                          	popq	%rsi
1409d1eed: 41 5c                       	popq	%r12
1409d1eef: 41 5d                       	popq	%r13
1409d1ef1: 41 5e                       	popq	%r14
1409d1ef3: 41 5f                       	popq	%r15
1409d1ef5: 5d                          	popq	%rbp
1409d1ef6: c3                          	retq
1409d1ef7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1409d1f00: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409d1f05: 55                          	pushq	%rbp
1409d1f06: 41 57                       	pushq	%r15
1409d1f08: 41 56                       	pushq	%r14
1409d1f0a: 41 55                       	pushq	%r13
1409d1f0c: 41 54                       	pushq	%r12
1409d1f0e: 56                          	pushq	%rsi
1409d1f0f: 57                          	pushq	%rdi
1409d1f10: 53                          	pushq	%rbx
1409d1f11: 48 83 ec 48                 	subq	$0x48, %rsp
1409d1f15: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409d1f1c: 80 bd c0 08 00 00 00        	cmpb	$0x0, 0x8c0(%rbp)
1409d1f23: 74 2f                       	je	0x1409d1f54 <.text+0x9d0f54>
1409d1f25: 48 8d 85 98 08 00 00        	leaq	0x898(%rbp), %rax
1409d1f2c: 48 89 85 b0 08 00 00        	movq	%rax, 0x8b0(%rbp)
1409d1f33: 48 8d 85 b0 08 00 00        	leaq	0x8b0(%rbp), %rax
1409d1f3a: 48 89 85 a8 08 00 00        	movq	%rax, 0x8a8(%rbp)
1409d1f41: 48 8d 0d 58 f5 0e 02        	leaq	0x20ef558(%rip), %rcx   # 0x142ac14a0
1409d1f48: 48 8d 95 a8 08 00 00        	leaq	0x8a8(%rbp), %rdx
1409d1f4f: e8 7c d3 95 00              	callq	0x14132f2d0 <.text+0x132e2d0>
1409d1f54: 90                          	nop
1409d1f55: 48 83 c4 48                 	addq	$0x48, %rsp
1409d1f59: 5b                          	popq	%rbx
1409d1f5a: 5f                          	popq	%rdi
1409d1f5b: 5e                          	popq	%rsi
1409d1f5c: 41 5c                       	popq	%r12
1409d1f5e: 41 5d                       	popq	%r13
1409d1f60: 41 5e                       	popq	%r14
1409d1f62: 41 5f                       	popq	%r15
1409d1f64: 5d                          	popq	%rbp
1409d1f65: c3                          	retq
1409d1f66: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1409d1f70: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409d1f75: 55                          	pushq	%rbp
1409d1f76: 41 57                       	pushq	%r15
1409d1f78: 41 56                       	pushq	%r14
1409d1f7a: 41 55                       	pushq	%r13
1409d1f7c: 41 54                       	pushq	%r12
1409d1f7e: 56                          	pushq	%rsi
1409d1f7f: 57                          	pushq	%rdi
1409d1f80: 53                          	pushq	%rbx
1409d1f81: 48 83 ec 48                 	subq	$0x48, %rsp
1409d1f85: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409d1f8c: 48 8b 85 c0 08 00 00        	movq	0x8c0(%rbp), %rax
1409d1f93: 48 ff 00                    	incq	(%rax)
1409d1f96: 48 83 c4 48                 	addq	$0x48, %rsp
1409d1f9a: 5b                          	popq	%rbx
1409d1f9b: 5f                          	popq	%rdi
1409d1f9c: 5e                          	popq	%rsi
1409d1f9d: 41 5c                       	popq	%r12
1409d1f9f: 41 5d                       	popq	%r13
1409d1fa1: 41 5e                       	popq	%r14
1409d1fa3: 41 5f                       	popq	%r15
1409d1fa5: 5d                          	popq	%rbp
1409d1fa6: c3                          	retq
1409d1fa7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1409d1fb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409d1fb5: 55                          	pushq	%rbp
1409d1fb6: 41 57                       	pushq	%r15
1409d1fb8: 41 56                       	pushq	%r14
1409d1fba: 41 55                       	pushq	%r13
1409d1fbc: 41 54                       	pushq	%r12
1409d1fbe: 56                          	pushq	%rsi
1409d1fbf: 57                          	pushq	%rdi
1409d1fc0: 53                          	pushq	%rbx
1409d1fc1: 48 83 ec 48                 	subq	$0x48, %rsp
1409d1fc5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409d1fcc: 80 bd cf 08 00 00 00        	cmpb	$0x0, 0x8cf(%rbp)
1409d1fd3: 74 1b                       	je	0x1409d1ff0 <.text+0x9d0ff0>
1409d1fd5: 48 8d 4d c8                 	leaq	-0x38(%rbp), %rcx
1409d1fd9: e8 e2 03 ec ff              	callq	0x1408923c0 <.text+0x8913c0>
1409d1fde: 90                          	nop
1409d1fdf: 48 83 c4 48                 	addq	$0x48, %rsp
1409d1fe3: 5b                          	popq	%rbx
1409d1fe4: 5f                          	popq	%rdi
1409d1fe5: 5e                          	popq	%rsi
1409d1fe6: 41 5c                       	popq	%r12
1409d1fe8: 41 5d                       	popq	%r13
1409d1fea: 41 5e                       	popq	%r14
1409d1fec: 41 5f                       	popq	%r15
1409d1fee: 5d                          	popq	%rbp
1409d1fef: c3                          	retq
1409d1ff0: 48 83 c4 48                 	addq	$0x48, %rsp
1409d1ff4: 5b                          	popq	%rbx
1409d1ff5: 5f                          	popq	%rdi
1409d1ff6: 5e                          	popq	%rsi
1409d1ff7: 41 5c                       	popq	%r12
1409d1ff9: 41 5d                       	popq	%r13
1409d1ffb: 41 5e                       	popq	%r14
1409d1ffd: 41 5f                       	popq	%r15
1409d1fff: 5d                          	popq	%rbp
1409d2000: c3                          	retq
1409d2001: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1409d2010: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409d2015: 55                          	pushq	%rbp
1409d2016: 41 57                       	pushq	%r15
1409d2018: 41 56                       	pushq	%r14
1409d201a: 41 55                       	pushq	%r13
1409d201c: 41 54                       	pushq	%r12
1409d201e: 56                          	pushq	%rsi
1409d201f: 57                          	pushq	%rdi
1409d2020: 53                          	pushq	%rbx
1409d2021: 48 83 ec 48                 	subq	$0x48, %rsp
1409d2025: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409d202c: c6 85 cf 08 00 00 01        	movb	$0x1, 0x8cf(%rbp)
1409d2033: 48 8d 8d b0 03 00 00        	leaq	0x3b0(%rbp), %rcx
1409d203a: e8 81 15 ec ff              	callq	0x1408935c0 <.text+0x8925c0>
1409d203f: c6 85 cf 08 00 00 01        	movb	$0x1, 0x8cf(%rbp)
1409d2046: 48 83 c4 48                 	addq	$0x48, %rsp
1409d204a: 5b                          	popq	%rbx
1409d204b: 5f                          	popq	%rdi
1409d204c: 5e                          	popq	%rsi
1409d204d: 41 5c                       	popq	%r12
1409d204f: 41 5d                       	popq	%r13
1409d2051: 41 5e                       	popq	%r14
1409d2053: 41 5f                       	popq	%r15
1409d2055: 5d                          	popq	%rbp
1409d2056: c3                          	retq
1409d2057: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1409d2060: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409d2065: 55                          	pushq	%rbp
1409d2066: 41 57                       	pushq	%r15
1409d2068: 41 56                       	pushq	%r14
1409d206a: 41 55                       	pushq	%r13
1409d206c: 41 54                       	pushq	%r12
1409d206e: 56                          	pushq	%rsi
1409d206f: 57                          	pushq	%rdi
1409d2070: 53                          	pushq	%rbx
1409d2071: 48 83 ec 48                 	subq	$0x48, %rsp
1409d2075: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409d207c: c6 85 cf 08 00 00 01        	movb	$0x1, 0x8cf(%rbp)
1409d2083: 48 8d 8d 98 07 00 00        	leaq	0x798(%rbp), %rcx
1409d208a: e8 31 15 ec ff              	callq	0x1408935c0 <.text+0x8925c0>
1409d208f: c6 85 cf 08 00 00 01        	movb	$0x1, 0x8cf(%rbp)
1409d2096: 48 83 c4 48                 	addq	$0x48, %rsp
1409d209a: 5b                          	popq	%rbx
1409d209b: 5f                          	popq	%rdi
1409d209c: 5e                          	popq	%rsi
1409d209d: 41 5c                       	popq	%r12
1409d209f: 41 5d                       	popq	%r13
1409d20a1: 41 5e                       	popq	%r14
1409d20a3: 41 5f                       	popq	%r15
1409d20a5: 5d                          	popq	%rbp
1409d20a6: c3                          	retq
