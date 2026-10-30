
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1407d20c0: 55                          	pushq	%rbp
1407d20c1: 41 57                       	pushq	%r15
1407d20c3: 41 56                       	pushq	%r14
1407d20c5: 41 55                       	pushq	%r13
1407d20c7: 41 54                       	pushq	%r12
1407d20c9: 56                          	pushq	%rsi
1407d20ca: 57                          	pushq	%rdi
1407d20cb: 53                          	pushq	%rbx
1407d20cc: 48 81 ec f8 03 00 00        	subq	$0x3f8, %rsp            # imm = 0x3F8
1407d20d3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1407d20db: 44 0f 29 b5 60 03 00 00     	movaps	%xmm14, 0x360(%rbp)
1407d20e3: 44 0f 29 ad 50 03 00 00     	movaps	%xmm13, 0x350(%rbp)
1407d20eb: 44 0f 29 a5 40 03 00 00     	movaps	%xmm12, 0x340(%rbp)
1407d20f3: 66 44 0f 7f 9d 30 03 00 00  	movdqa	%xmm11, 0x330(%rbp)
1407d20fc: 44 0f 29 95 20 03 00 00     	movaps	%xmm10, 0x320(%rbp)
1407d2104: 44 0f 29 8d 10 03 00 00     	movaps	%xmm9, 0x310(%rbp)
1407d210c: 66 44 0f 29 85 00 03 00 00  	movapd	%xmm8, 0x300(%rbp)
1407d2115: 0f 29 bd f0 02 00 00        	movaps	%xmm7, 0x2f0(%rbp)
1407d211c: 66 0f 29 b5 e0 02 00 00     	movapd	%xmm6, 0x2e0(%rbp)
1407d2124: 48 c7 85 d8 02 00 00 fe ff ff ff    	movq	$-0x2, 0x2d8(%rbp)
1407d212f: 4d 89 cf                    	movq	%r9, %r15
1407d2132: 4c 89 c7                    	movq	%r8, %rdi
1407d2135: 48 89 d6                    	movq	%rdx, %rsi
1407d2138: 48 89 cb                    	movq	%rcx, %rbx
1407d213b: e8 c0 48 44 00              	callq	0x140c16a00 <.text+0xc15a00>
1407d2140: 88 85 d5 02 00 00           	movb	%al, 0x2d5(%rbp)
1407d2146: 84 c0                       	testb	%al, %al
1407d2148: 0f 84 37 02 00 00           	je	0x1407d2385 <.text+0x7d1385>
1407d214e: 4c 8d 25 36 5a 2b 02        	leaq	0x22b5a36(%rip), %r12   # 0x142a87b8b
1407d2155: 41 be 62 00 00 00           	movl	$0x62, %r14d
1407d215b: ba 62 00 00 00              	movl	$0x62, %edx
1407d2160: 4c 89 e1                    	movq	%r12, %rcx
1407d2163: e8 a8 c9 ff ff              	callq	0x1407ceb10 <.text+0x7cdb10>
1407d2168: 48 83 f8 01                 	cmpq	$0x1, %rax
1407d216c: 0f 85 ab 00 00 00           	jne	0x1407d221d <.text+0x7d121d>
1407d2172: 49 89 d1                    	movq	%rdx, %r9
1407d2175: 48 85 d2                    	testq	%rdx, %rdx
1407d2178: 74 36                       	je	0x1407d21b0 <.text+0x7d11b0>
1407d217a: 49 83 f9 62                 	cmpq	$0x62, %r9
1407d217e: 73 2e                       	jae	0x1407d21ae <.text+0x7d11ae>
1407d2180: 48 8d 05 04 5a 2b 02        	leaq	0x22b5a04(%rip), %rax   # 0x142a87b8b
1407d2187: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
1407d218c: 7f 22                       	jg	0x1407d21b0 <.text+0x7d11b0>
1407d218e: 48 8d 05 db 5a 2b 02        	leaq	0x22b5adb(%rip), %rax   # 0x142a87c70
1407d2195: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407d219a: 48 8d 0d ea 59 2b 02        	leaq	0x22b59ea(%rip), %rcx   # 0x142a87b8b
1407d21a1: ba 62 00 00 00              	movl	$0x62, %edx
1407d21a6: 45 31 c0                    	xorl	%r8d, %r8d
1407d21a9: e8 92 77 10 02              	callq	0x1428d9940 <.text+0x28d8940>
1407d21ae: 75 de                       	jne	0x1407d218e <.text+0x7d118e>
1407d21b0: 4c 8d 25 d4 59 2b 02        	leaq	0x22b59d4(%rip), %r12   # 0x142a87b8b
1407d21b7: 4c 89 e1                    	movq	%r12, %rcx
1407d21ba: 4c 89 ca                    	movq	%r9, %rdx
1407d21bd: e8 4e c9 ff ff              	callq	0x1407ceb10 <.text+0x7cdb10>
1407d21c2: 48 83 f8 01                 	cmpq	$0x1, %rax
1407d21c6: 75 55                       	jne	0x1407d221d <.text+0x7d121d>
1407d21c8: 49 89 d0                    	movq	%rdx, %r8
1407d21cb: 49 83 c0 02                 	addq	$0x2, %r8
1407d21cf: 74 39                       	je	0x1407d220a <.text+0x7d120a>
1407d21d1: 49 83 f8 62                 	cmpq	$0x62, %r8
1407d21d5: 73 31                       	jae	0x1407d2208 <.text+0x7d1208>
1407d21d7: 48 8d 05 ad 59 2b 02        	leaq	0x22b59ad(%rip), %rax   # 0x142a87b8b
1407d21de: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
1407d21e3: 7f 25                       	jg	0x1407d220a <.text+0x7d120a>
1407d21e5: 48 8d 05 9c 5a 2b 02        	leaq	0x22b5a9c(%rip), %rax   # 0x142a87c88
1407d21ec: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407d21f1: 48 8d 0d 93 59 2b 02        	leaq	0x22b5993(%rip), %rcx   # 0x142a87b8b
1407d21f8: ba 62 00 00 00              	movl	$0x62, %edx
1407d21fd: 41 b9 62 00 00 00           	movl	$0x62, %r9d
1407d2203: e8 38 77 10 02              	callq	0x1428d9940 <.text+0x28d8940>
1407d2208: 75 db                       	jne	0x1407d21e5 <.text+0x7d11e5>
1407d220a: 41 be 60 00 00 00           	movl	$0x60, %r14d
1407d2210: 49 29 d6                    	subq	%rdx, %r14
1407d2213: 4c 8d 25 71 59 2b 02        	leaq	0x22b5971(%rip), %r12   # 0x142a87b8b
1407d221a: 4d 01 c4                    	addq	%r8, %r12
1407d221d: 4c 8d 05 1c 5d 2b 02        	leaq	0x22b5d1c(%rip), %r8    # 0x142a87f40
1407d2224: 4c 8d 2d d5 5c 2b 02        	leaq	0x22b5cd5(%rip), %r13   # 0x142a87f00
1407d222b: 4c 89 bd c8 02 00 00        	movq	%r15, 0x2c8(%rbp)
1407d2232: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1407d2240: 41 0f be 40 ff              	movsbl	-0x1(%r8), %eax
1407d2245: 85 c0                       	testl	%eax, %eax
1407d2247: 78 17                       	js	0x1407d2260 <.text+0x7d1260>
1407d2249: 49 ff c8                    	decq	%r8
1407d224c: 83 f8 5c                    	cmpl	$0x5c, %eax
1407d224f: 75 63                       	jne	0x1407d22b4 <.text+0x7d12b4>
1407d2251: eb 73                       	jmp	0x1407d22c6 <.text+0x7d12c6>
1407d2253: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1407d2260: 41 0f b6 48 fe              	movzbl	-0x2(%r8), %ecx
1407d2265: 80 f9 c0                    	cmpb	$-0x40, %cl
1407d2268: 7d 23                       	jge	0x1407d228d <.text+0x7d128d>
1407d226a: 41 0f b6 50 fd              	movzbl	-0x3(%r8), %edx
1407d226f: 80 fa c0                    	cmpb	$-0x40, %dl
1407d2272: 7d 22                       	jge	0x1407d2296 <.text+0x7d1296>
1407d2274: 45 0f b6 48 fc              	movzbl	-0x4(%r8), %r9d
1407d2279: 49 83 c0 fc                 	addq	$-0x4, %r8
1407d227d: 41 83 e1 07                 	andl	$0x7, %r9d
1407d2281: 41 c1 e1 06                 	shll	$0x6, %r9d
1407d2285: 83 e2 3f                    	andl	$0x3f, %edx
1407d2288: 44 09 ca                    	orl	%r9d, %edx
1407d228b: eb 10                       	jmp	0x1407d229d <.text+0x7d129d>
1407d228d: 49 83 c0 fe                 	addq	$-0x2, %r8
1407d2291: 83 e1 1f                    	andl	$0x1f, %ecx
1407d2294: eb 0f                       	jmp	0x1407d22a5 <.text+0x7d12a5>
1407d2296: 49 83 c0 fd                 	addq	$-0x3, %r8
1407d229a: 83 e2 0f                    	andl	$0xf, %edx
1407d229d: c1 e2 06                    	shll	$0x6, %edx
1407d22a0: 83 e1 3f                    	andl	$0x3f, %ecx
1407d22a3: 09 d1                       	orl	%edx, %ecx
1407d22a5: c1 e1 06                    	shll	$0x6, %ecx
1407d22a8: 24 3f                       	andb	$0x3f, %al
1407d22aa: 0f b6 c0                    	movzbl	%al, %eax
1407d22ad: 09 c8                       	orl	%ecx, %eax
1407d22af: 83 f8 5c                    	cmpl	$0x5c, %eax
1407d22b2: 74 12                       	je	0x1407d22c6 <.text+0x7d12c6>
1407d22b4: 83 f8 2f                    	cmpl	$0x2f, %eax
1407d22b7: 74 0d                       	je	0x1407d22c6 <.text+0x7d12c6>
1407d22b9: 4d 39 e8                    	cmpq	%r13, %r8
1407d22bc: 75 82                       	jne	0x1407d2240 <.text+0x7d1240>
1407d22be: 41 bf 40 00 00 00           	movl	$0x40, %r15d
1407d22c4: eb 4d                       	jmp	0x1407d2313 <.text+0x7d1313>
1407d22c6: 4c 8d 2d 33 5c 2b 02        	leaq	0x22b5c33(%rip), %r13   # 0x142a87f00
1407d22cd: 4d 29 e8                    	subq	%r13, %r8
1407d22d0: 49 ff c0                    	incq	%r8
1407d22d3: 74 32                       	je	0x1407d2307 <.text+0x7d1307>
1407d22d5: 49 83 f8 40                 	cmpq	$0x40, %r8
1407d22d9: 73 2a                       	jae	0x1407d2305 <.text+0x7d1305>
1407d22db: 43 80 3c 28 bf              	cmpb	$-0x41, (%r8,%r13)
1407d22e0: 7f 25                       	jg	0x1407d2307 <.text+0x7d1307>
1407d22e2: 48 8d 05 67 59 2b 02        	leaq	0x22b5967(%rip), %rax   # 0x142a87c50
1407d22e9: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407d22ee: 48 8d 0d 0b 5c 2b 02        	leaq	0x22b5c0b(%rip), %rcx   # 0x142a87f00
1407d22f5: ba 40 00 00 00              	movl	$0x40, %edx
1407d22fa: 41 b9 40 00 00 00           	movl	$0x40, %r9d
1407d2300: e8 3b 76 10 02              	callq	0x1428d9940 <.text+0x28d8940>
1407d2305: 75 db                       	jne	0x1407d22e2 <.text+0x7d12e2>
1407d2307: 41 bf 40 00 00 00           	movl	$0x40, %r15d
1407d230d: 4d 29 c7                    	subq	%r8, %r15
1407d2310: 4d 01 c5                    	addq	%r8, %r13
1407d2313: e8 18 5e 44 00              	callq	0x140c18130 <.text+0xc17130>
1407d2318: 48 89 c1                    	movq	%rax, %rcx
1407d231b: 48 8b 00                    	movq	(%rax), %rax
1407d231e: 48 83 f8 01                 	cmpq	$0x1, %rax
1407d2322: 0f 85 66 15 00 00           	jne	0x1407d388e <.text+0x7d288e>
1407d2328: 48 83 c1 08                 	addq	$0x8, %rcx
1407d232c: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1407d2330: 0f 85 4f 16 00 00           	jne	0x1407d3985 <.text+0x7d2985>
1407d2336: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1407d233d: 48 89 8d b8 02 00 00        	movq	%rcx, 0x2b8(%rbp)
1407d2344: 48 83 c1 08                 	addq	$0x8, %rcx
1407d2348: 4c 89 7c 24 20              	movq	%r15, 0x20(%rsp)
1407d234d: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
1407d2356: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
1407d235f: 4c 89 e2                    	movq	%r12, %rdx
1407d2362: 4d 89 f0                    	movq	%r14, %r8
1407d2365: 4d 89 e9                    	movq	%r13, %r9
1407d2368: e8 d3 49 44 00              	callq	0x140c16d40 <.text+0xc15d40>
1407d236d: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
1407d2374: 48 8b 85 b8 02 00 00        	movq	0x2b8(%rbp), %rax
1407d237b: 48 ff 00                    	incq	(%rax)
1407d237e: 4c 8b bd c8 02 00 00        	movq	0x2c8(%rbp), %r15
1407d2385: 48 8b 85 e0 03 00 00        	movq	0x3e0(%rbp), %rax
1407d238c: 49 8b 0f                    	movq	(%r15), %rcx
1407d238f: 48 8d 91 98 01 00 00        	leaq	0x198(%rcx), %rdx
1407d2396: 48 89 95 e8 00 00 00        	movq	%rdx, 0xe8(%rbp)
1407d239d: 4c 8b 49 08                 	movq	0x8(%rcx), %r9
1407d23a1: 48 69 49 10 50 01 00 00     	imulq	$0x150, 0x10(%rcx), %rcx # imm = 0x150
1407d23a9: 4c 01 c9                    	addq	%r9, %rcx
1407d23ac: 48 8b 10                    	movq	(%rax), %rdx
1407d23af: 8b 43 1c                    	movl	0x1c(%rbx), %eax
1407d23b2: 89 85 38 02 00 00           	movl	%eax, 0x238(%rbp)
1407d23b8: 48 8b 03                    	movq	(%rbx), %rax
1407d23bb: 48 89 85 d8 00 00 00        	movq	%rax, 0xd8(%rbp)
1407d23c2: 48 8b 43 10                 	movq	0x10(%rbx), %rax
1407d23c6: 48 89 85 c8 00 00 00        	movq	%rax, 0xc8(%rbp)
1407d23cd: 8b 46 1c                    	movl	0x1c(%rsi), %eax
1407d23d0: 89 85 3c 02 00 00           	movl	%eax, 0x23c(%rbp)
1407d23d6: 48 8b 46 10                 	movq	0x10(%rsi), %rax
1407d23da: 48 89 85 d0 00 00 00        	movq	%rax, 0xd0(%rbp)
1407d23e1: b8 90 00 00 00              	movl	$0x90, %eax
1407d23e6: 4c 8b 06                    	movq	(%rsi), %r8
1407d23e9: 49 01 c0                    	addq	%rax, %r8
1407d23ec: 4c 89 85 e0 00 00 00        	movq	%r8, 0xe0(%rbp)
1407d23f3: 8b 47 1c                    	movl	0x1c(%rdi), %eax
1407d23f6: 89 85 b8 02 00 00           	movl	%eax, 0x2b8(%rbp)
1407d23fc: 48 8b 37                    	movq	(%rdi), %rsi
1407d23ff: 48 8b 47 10                 	movq	0x10(%rdi), %rax
1407d2403: 48 89 85 50 02 00 00        	movq	%rax, 0x250(%rbp)
1407d240a: 48 8b 85 e8 03 00 00        	movq	0x3e8(%rbp), %rax
1407d2411: 48 8b 00                    	movq	(%rax), %rax
1407d2414: 48 89 85 18 02 00 00        	movq	%rax, 0x218(%rbp)
1407d241b: f3 0f 10 3d 29 e6 2a 02     	movss	0x22ae629(%rip), %xmm7  # 0x142a80a4c
1407d2423: f3 44 0f 10 05 d8 34 15 02  	movss	0x21534d8(%rip), %xmm8  # 0x142925904
1407d242c: f3 44 0f 10 0d a3 34 15 02  	movss	0x21534a3(%rip), %xmm9  # 0x1429258d8
1407d2435: f2 44 0f 10 15 22 35 15 02  	movsd	0x2153522(%rip), %xmm10 # 0x142925960
1407d243e: 66 45 0f 76 db              	pcmpeqd	%xmm11, %xmm11
1407d2443: 48 89 95 08 01 00 00        	movq	%rdx, 0x108(%rbp)
1407d244a: 48 8d 42 20                 	leaq	0x20(%rdx), %rax
1407d244e: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
1407d2452: 48 8d 86 80 01 00 00        	leaq	0x180(%rsi), %rax
1407d2459: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
1407d245d: 31 c0                       	xorl	%eax, %eax
1407d245f: 49 39 c9                    	cmpq	%rcx, %r9
1407d2462: 0f 84 e5 0c 00 00           	je	0x1407d314d <.text+0x7d214d>
1407d2468: 4c 89 cb                    	movq	%r9, %rbx
1407d246b: eb 13                       	jmp	0x1407d2480 <.text+0x7d1480>
1407d246d: 0f 1f 00                    	nopl	(%rax)
1407d2470: 48 81 c3 50 01 00 00        	addq	$0x150, %rbx            # imm = 0x150
1407d2477: 48 39 cb                    	cmpq	%rcx, %rbx
1407d247a: 0f 84 cd 0c 00 00           	je	0x1407d314d <.text+0x7d214d>
1407d2480: 80 bb 48 01 00 00 01        	cmpb	$0x1, 0x148(%rbx)
1407d2487: 75 e7                       	jne	0x1407d2470 <.text+0x7d1470>
1407d2489: 48 3b 83 98 00 00 00        	cmpq	0x98(%rbx), %rax
1407d2490: 70 de                       	jo	0x1407d2470 <.text+0x7d1470>
1407d2492: 48 89 8d 10 01 00 00        	movq	%rcx, 0x110(%rbp)
1407d2499: 48 8b 43 60                 	movq	0x60(%rbx), %rax
1407d249d: 48 89 85 90 02 00 00        	movq	%rax, 0x290(%rbp)
1407d24a4: 48 8b 8b 18 01 00 00        	movq	0x118(%rbx), %rcx
1407d24ab: 48 c1 e9 03                 	shrq	$0x3, %rcx
1407d24af: 48 89 4d 40                 	movq	%rcx, 0x40(%rbp)
1407d24b3: 48 39 c8                    	cmpq	%rcx, %rax
1407d24b6: 0f 85 e6 14 00 00           	jne	0x1407d39a2 <.text+0x7d29a2>
1407d24bc: 48 8d 93 28 01 00 00        	leaq	0x128(%rbx), %rdx
1407d24c3: 48 8b 83 a0 00 00 00        	movq	0xa0(%rbx), %rax
1407d24ca: 48 89 45 f8                 	movq	%rax, -0x8(%rbp)
1407d24ce: 48 8b 83 a8 00 00 00        	movq	0xa8(%rbx), %rax
1407d24d5: 48 89 85 78 01 00 00        	movq	%rax, 0x178(%rbp)
1407d24dc: 48 8b 85 08 01 00 00        	movq	0x108(%rbp), %rax
1407d24e3: 48 83 78 18 00              	cmpq	$0x0, 0x18(%rax)
1407d24e8: 48 89 95 c8 02 00 00        	movq	%rdx, 0x2c8(%rbp)
1407d24ef: 0f 84 c1 00 00 00           	je	0x1407d25b6 <.text+0x7d15b6>
1407d24f5: 48 8b 4d 10                 	movq	0x10(%rbp), %rcx
1407d24f9: e8 e2 f6 dd ff              	callq	0x1405b1be0 <.text+0x5b0be0>
1407d24fe: 48 8b 95 08 01 00 00        	movq	0x108(%rbp), %rdx
1407d2505: 48 8b 0a                    	movq	(%rdx), %rcx
1407d2508: 48 8b 52 08                 	movq	0x8(%rdx), %rdx
1407d250c: 49 89 d0                    	movq	%rdx, %r8
1407d250f: 49 21 c0                    	andq	%rax, %r8
1407d2512: 48 c1 e8 39                 	shrq	$0x39, %rax
1407d2516: 66 0f 6e c0                 	movd	%eax, %xmm0
1407d251a: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407d251e: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407d2523: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407d2528: 31 c0                       	xorl	%eax, %eax
1407d252a: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
1407d2530: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1407d2534: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1407d2538: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
1407d253d: 45 85 c9                    	testl	%r9d, %r9d
1407d2540: 74 52                       	je	0x1407d2594 <.text+0x7d1594>
1407d2542: 4c 8b 95 c8 02 00 00        	movq	0x2c8(%rbp), %r10
1407d2549: f3 41 0f 7e 12              	movq	(%r10), %xmm2
1407d254e: f3 45 0f bc d1              	tzcntl	%r9d, %r10d
1407d2553: 4d 01 c2                    	addq	%r8, %r10
1407d2556: 49 21 d2                    	andq	%rdx, %r10
1407d2559: 49 c1 e2 03                 	shlq	$0x3, %r10
1407d255d: 49 89 cb                    	movq	%rcx, %r11
1407d2560: 4d 29 d3                    	subq	%r10, %r11
1407d2563: f3 41 0f 7e 5b f8           	movq	-0x8(%r11), %xmm3
1407d2569: 66 0f 76 da                 	pcmpeqd	%xmm2, %xmm3
1407d256d: 66 0f 70 db 50              	pshufd	$0x50, %xmm3, %xmm3     # xmm3 = xmm3[0,0,1,1]
1407d2572: 66 44 0f 50 d3              	movmskpd	%xmm3, %r10d
1407d2577: 41 b3 01                    	movb	$0x1, %r11b
1407d257a: 44 89 9d c4 02 00 00        	movl	%r11d, 0x2c4(%rbp)
1407d2581: 41 83 fa 03                 	cmpl	$0x3, %r10d
1407d2585: 74 39                       	je	0x1407d25c0 <.text+0x7d15c0>
1407d2587: 45 8d 51 ff                 	leal	-0x1(%r9), %r10d
1407d258b: 66 45 21 ca                 	andw	%r9w, %r10w
1407d258f: 45 89 d1                    	movl	%r10d, %r9d
1407d2592: 75 ba                       	jne	0x1407d254e <.text+0x7d154e>
1407d2594: 66 41 0f 74 cb              	pcmpeqb	%xmm11, %xmm1
1407d2599: 66 44 0f d7 c9              	pmovmskb	%xmm1, %r9d
1407d259e: 45 85 c9                    	testl	%r9d, %r9d
1407d25a1: 75 13                       	jne	0x1407d25b6 <.text+0x7d15b6>
1407d25a3: 49 01 c0                    	addq	%rax, %r8
1407d25a6: 49 83 c0 10                 	addq	$0x10, %r8
1407d25aa: 48 83 c0 10                 	addq	$0x10, %rax
1407d25ae: 49 21 d0                    	andq	%rdx, %r8
1407d25b1: e9 74 ff ff ff              	jmp	0x1407d252a <.text+0x7d152a>
1407d25b6: c7 85 c4 02 00 00 00 00 00 00       	movl	$0x0, 0x2c4(%rbp)
1407d25c0: 48 8b 83 40 01 00 00        	movq	0x140(%rbx), %rax
1407d25c7: 48 83 f8 02                 	cmpq	$0x2, %rax
1407d25cb: 72 10                       	jb	0x1407d25dd <.text+0x7d15dd>
1407d25cd: 4c 8b 83 30 01 00 00        	movq	0x130(%rbx), %r8
1407d25d4: 48 8b 83 38 01 00 00        	movq	0x138(%rbx), %rax
1407d25db: eb 07                       	jmp	0x1407d25e4 <.text+0x7d15e4>
1407d25dd: 4c 8d 83 30 01 00 00        	leaq	0x130(%rbx), %r8
1407d25e4: 48 8d 8b d0 00 00 00        	leaq	0xd0(%rbx), %rcx
1407d25eb: 48 89 8d f0 00 00 00        	movq	%rcx, 0xf0(%rbp)
1407d25f2: 48 8d 8b 50 01 00 00        	leaq	0x150(%rbx), %rcx
1407d25f9: 48 89 8d 18 01 00 00        	movq	%rcx, 0x118(%rbp)
1407d2600: 49 8d 04 80                 	leaq	(%r8,%rax,4), %rax
1407d2604: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
1407d2608: 48 8d 83 f0 00 00 00        	leaq	0xf0(%rbx), %rax
1407d260f: 48 89 85 f8 00 00 00        	movq	%rax, 0xf8(%rbp)
1407d2616: 45 31 d2                    	xorl	%r10d, %r10d
1407d2619: 4c 89 c0                    	movq	%r8, %rax
1407d261c: 48 3b 45 f0                 	cmpq	-0x10(%rbp), %rax
1407d2620: 0f 84 5a 0a 00 00           	je	0x1407d3080 <.text+0x7d2080>
1407d2626: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1407d2630: 44 0f b7 68 02              	movzwl	0x2(%rax), %r13d
1407d2635: 45 0f b7 f5                 	movzwl	%r13w, %r14d
1407d2639: 4f 8d 0c 32                 	leaq	(%r10,%r14), %r9
1407d263d: 49 83 f9 ff                 	cmpq	$-0x1, %r9
1407d2641: 0f 84 85 12 00 00           	je	0x1407d38cc <.text+0x7d28cc>
1407d2647: 49 8d 49 01                 	leaq	0x1(%r9), %rcx
1407d264b: 48 89 cf                    	movq	%rcx, %rdi
1407d264e: 4c 29 d7                    	subq	%r10, %rdi
1407d2651: 0f 82 86 12 00 00           	jb	0x1407d38dd <.text+0x7d28dd>
1407d2657: 4c 3b 8d 78 01 00 00        	cmpq	0x178(%rbp), %r9
1407d265e: 0f 83 90 12 00 00           	jae	0x1407d38f4 <.text+0x7d28f4>
1407d2664: 48 8b 53 78                 	movq	0x78(%rbx), %rdx
1407d2668: 49 39 d1                    	cmpq	%rdx, %r9
1407d266b: 0f 83 9b 12 00 00           	jae	0x1407d390c <.text+0x7d290c>
1407d2671: 48 8b 53 60                 	movq	0x60(%rbx), %rdx
1407d2675: 49 39 d1                    	cmpq	%rdx, %r9
1407d2678: 0f 83 9f 12 00 00           	jae	0x1407d391d <.text+0x7d291d>
1407d267e: 4a 8d 0c d5 00 00 00 00     	leaq	(,%r10,8), %rcx
1407d2686: 48 03 4b 70                 	addq	0x70(%rbx), %rcx
1407d268a: 48 89 8d 60 02 00 00        	movq	%rcx, 0x260(%rbp)
1407d2691: 4a 8d 0c 95 00 00 00 00     	leaq	(,%r10,4), %rcx
1407d2699: 48 03 4b 58                 	addq	0x58(%rbx), %rcx
1407d269d: 48 89 8d 68 02 00 00        	movq	%rcx, 0x268(%rbp)
1407d26a4: 48 8b 8b 10 01 00 00        	movq	0x110(%rbx), %rcx
1407d26ab: 48 8b 93 18 01 00 00        	movq	0x118(%rbx), %rdx
1407d26b2: 4c 89 95 90 02 00 00        	movq	%r10, 0x290(%rbp)
1407d26b9: 4c 89 8d 98 02 00 00        	movq	%r9, 0x298(%rbp)
1407d26c0: c6 85 a0 02 00 00 00        	movb	$0x0, 0x2a0(%rbp)
1407d26c7: 49 89 d0                    	movq	%rdx, %r8
1407d26ca: 49 c1 e8 03                 	shrq	$0x3, %r8
1407d26ce: 4c 89 85 30 01 00 00        	movq	%r8, 0x130(%rbp)
1407d26d5: 4d 39 ca                    	cmpq	%r9, %r10
1407d26d8: 0f 87 0a 13 00 00           	ja	0x1407d39e8 <.text+0x7d29e8>
1407d26de: 4d 39 c2                    	cmpq	%r8, %r10
1407d26e1: 0f 83 01 13 00 00           	jae	0x1407d39e8 <.text+0x7d29e8>
1407d26e7: 4d 39 c1                    	cmpq	%r8, %r9
1407d26ea: 0f 83 f8 12 00 00           	jae	0x1407d39e8 <.text+0x7d29e8>
1407d26f0: 4c 89 8d 58 02 00 00        	movq	%r9, 0x258(%rbp)
1407d26f7: 4c 8d 40 04                 	leaq	0x4(%rax), %r8
1407d26fb: 4c 89 85 80 02 00 00        	movq	%r8, 0x280(%rbp)
1407d2702: 4f 8d 04 52                 	leaq	(%r10,%r10,2), %r8
1407d2706: 4c 8b 4d f8                 	movq	-0x8(%rbp), %r9
1407d270a: 4f 8d 0c 81                 	leaq	(%r9,%r8,4), %r9
1407d270e: 41 89 c8                    	movl	%ecx, %r8d
1407d2711: 41 83 e0 07                 	andl	$0x7, %r8d
1407d2715: 83 e2 07                    	andl	$0x7, %edx
1407d2718: 4e 8d 3c c2                 	leaq	(%rdx,%r8,8), %r15
1407d271c: 4c 89 95 70 02 00 00        	movq	%r10, 0x270(%rbp)
1407d2723: 4d 01 d7                    	addq	%r10, %r15
1407d2726: 48 83 e1 f8                 	andq	$-0x8, %rcx
1407d272a: 4c 89 fa                    	movq	%r15, %rdx
1407d272d: 48 c1 ea 06                 	shrq	$0x6, %rdx
1407d2731: 48 8d 0c d1                 	leaq	(%rcx,%rdx,8), %rcx
1407d2735: 45 89 fc                    	movl	%r15d, %r12d
1407d2738: 41 c1 ec 03                 	shrl	$0x3, %r12d
1407d273c: 41 83 e4 07                 	andl	$0x7, %r12d
1407d2740: 41 83 e7 07                 	andl	$0x7, %r15d
1407d2744: 48 89 8d 78 02 00 00        	movq	%rcx, 0x278(%rbp)
1407d274b: 49 09 cc                    	orq	%rcx, %r12
1407d274e: 80 38 03                    	cmpb	$0x3, (%rax)
1407d2751: 0f 85 a9 04 00 00           	jne	0x1407d2c00 <.text+0x7d1c00>
1407d2757: 48 89 7d d0                 	movq	%rdi, -0x30(%rbp)
1407d275b: 48 83 ff 02                 	cmpq	$0x2, %rdi
1407d275f: 0f 85 c9 11 00 00           	jne	0x1407d392e <.text+0x7d292e>
1407d2765: 41 0f 10 31                 	movups	(%r9), %xmm6
1407d2769: f2 45 0f 10 61 10           	movsd	0x10(%r9), %xmm12
1407d276f: 8b bb 28 01 00 00           	movl	0x128(%rbx), %edi
1407d2775: 44 8b b3 2c 01 00 00        	movl	0x12c(%rbx), %r14d
1407d277c: f3 44 0f 16 ee              	movshdup	%xmm6, %xmm13           # xmm13 = xmm6[1,1,3,3]
1407d2781: 45 0f 2e ec                 	ucomiss	%xmm12, %xmm13
1407d2785: 76 29                       	jbe	0x1407d27b0 <.text+0x7d17b0>
1407d2787: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
1407d278e: e8 5d 77 18 00              	callq	0x140959ef0 <.text+0x958ef0>
1407d2793: 45 0f 57 f6                 	xorps	%xmm14, %xmm14
1407d2797: 48 85 c0                    	testq	%rax, %rax
1407d279a: 74 4b                       	je	0x1407d27e7 <.text+0x7d17e7>
1407d279c: 80 bb e8 00 00 00 01        	cmpb	$0x1, 0xe8(%rbx)
1407d27a3: 48 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %rcx
1407d27aa: 74 29                       	je	0x1407d27d5 <.text+0x7d17d5>
1407d27ac: eb 39                       	jmp	0x1407d27e7 <.text+0x7d17e7>
1407d27ae: 66 90                       	nop
1407d27b0: 48 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %rcx
1407d27b7: e8 34 77 18 00              	callq	0x140959ef0 <.text+0x958ef0>
1407d27bc: 45 0f 57 f6                 	xorps	%xmm14, %xmm14
1407d27c0: 48 85 c0                    	testq	%rax, %rax
1407d27c3: 74 22                       	je	0x1407d27e7 <.text+0x7d17e7>
1407d27c5: 80 bb 08 01 00 00 01        	cmpb	$0x1, 0x108(%rbx)
1407d27cc: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
1407d27d3: 75 12                       	jne	0x1407d27e7 <.text+0x7d17e7>
1407d27d5: e8 16 77 18 00              	callq	0x140959ef0 <.text+0x958ef0>
1407d27da: 44 0f 28 f7                 	movaps	%xmm7, %xmm14
1407d27de: 48 85 c0                    	testq	%rax, %rax
1407d27e1: 74 04                       	je	0x1407d27e7 <.text+0x7d17e7>
1407d27e3: 45 0f 57 f6                 	xorps	%xmm14, %xmm14
1407d27e7: 0f 28 c6                    	movaps	%xmm6, %xmm0
1407d27ea: 0f c6 c6 e8                 	shufps	$0xe8, %xmm6, %xmm0     # xmm0 = xmm0[0,2],xmm6[2,3]
1407d27ee: 41 0f 28 cc                 	movaps	%xmm12, %xmm1
1407d27f2: 0f c6 ce f5                 	shufps	$0xf5, %xmm6, %xmm1     # xmm1 = xmm1[1,1],xmm6[3,3]
1407d27f6: 0f c6 c9 e2                 	shufps	$0xe2, %xmm1, %xmm1     # xmm1 = xmm1[2,0,2,3]
1407d27fa: 0f 5c c1                    	subps	%xmm1, %xmm0
1407d27fd: 0f 28 c8                    	movaps	%xmm0, %xmm1
1407d2800: 0f 59 c8                    	mulps	%xmm0, %xmm1
1407d2803: f3 0f 16 d1                 	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
1407d2807: f3 0f 58 d1                 	addss	%xmm1, %xmm2
1407d280b: 0f 57 c9                    	xorps	%xmm1, %xmm1
1407d280e: f3 0f 51 ca                 	sqrtss	%xmm2, %xmm1
1407d2812: 66 41 0f 28 d0              	movapd	%xmm8, %xmm2
1407d2817: f3 0f 5e d1                 	divss	%xmm1, %xmm2
1407d281b: f3 0f 12 ca                 	movsldup	%xmm2, %xmm1            # xmm1 = xmm2[0,0,2,2]
1407d281f: 0f 59 c8                    	mulps	%xmm0, %xmm1
1407d2822: 0f 13 4d 40                 	movlps	%xmm1, 0x40(%rbp)
1407d2826: 48 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %rcx
1407d282d: 48 8d 55 40                 	leaq	0x40(%rbp), %rdx
1407d2831: e8 8a a5 4c 00              	callq	0x140c9cdc0 <.text+0xc9bdc0>
1407d2836: 45 0f 2e e5                 	ucomiss	%xmm13, %xmm12
1407d283a: 76 2f                       	jbe	0x1407d286b <.text+0x7d186b>
1407d283c: 80 bb e8 00 00 00 01        	cmpb	$0x1, 0xe8(%rbx)
1407d2843: 76 26                       	jbe	0x1407d286b <.text+0x7d186b>
1407d2845: c6 44 24 20 01              	movb	$0x1, 0x20(%rsp)
1407d284a: 48 8b 8d 18 02 00 00        	movq	0x218(%rbp), %rcx
1407d2851: 48 8b 95 e8 00 00 00        	movq	0xe8(%rbp), %rdx
1407d2858: 41 89 f8                    	movl	%edi, %r8d
1407d285b: 45 89 f1                    	movl	%r14d, %r9d
1407d285e: e8 dd 69 00 00              	callq	0x1407d9240 <.text+0x7d8240>
1407d2863: 41 0f 28 c1                 	movaps	%xmm9, %xmm0
1407d2867: 84 c0                       	testb	%al, %al
1407d2869: 74 45                       	je	0x1407d28b0 <.text+0x7d18b0>
1407d286b: 45 0f 2e ec                 	ucomiss	%xmm12, %xmm13
1407d286f: 0f 57 c0                    	xorps	%xmm0, %xmm0
1407d2872: 76 3c                       	jbe	0x1407d28b0 <.text+0x7d18b0>
1407d2874: 80 bb 08 01 00 00 01        	cmpb	$0x1, 0x108(%rbx)
1407d287b: 76 33                       	jbe	0x1407d28b0 <.text+0x7d18b0>
1407d287d: c6 44 24 20 00              	movb	$0x0, 0x20(%rsp)
1407d2882: 48 8b 8d 18 02 00 00        	movq	0x218(%rbp), %rcx
1407d2889: 48 8b 95 e8 00 00 00        	movq	0xe8(%rbp), %rdx
1407d2890: 45 89 f0                    	movl	%r14d, %r8d
1407d2893: 41 89 f9                    	movl	%edi, %r9d
1407d2896: e8 a5 69 00 00              	callq	0x1407d9240 <.text+0x7d8240>
1407d289b: 0f 57 c0                    	xorps	%xmm0, %xmm0
1407d289e: 84 c0                       	testb	%al, %al
1407d28a0: 75 0e                       	jne	0x1407d28b0 <.text+0x7d18b0>
1407d28a2: 41 0f 28 c1                 	movaps	%xmm9, %xmm0
1407d28a6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1407d28b0: 45 0f 2e ec                 	ucomiss	%xmm12, %xmm13
1407d28b4: f3 0f 10 8d 80 01 00 00     	movss	0x180(%rbp), %xmm1
1407d28bc: f2 0f 10 95 84 01 00 00     	movsd	0x184(%rbp), %xmm2
1407d28c4: 76 2a                       	jbe	0x1407d28f0 <.text+0x7d18f0>
1407d28c6: 44 0f 14 f0                 	unpcklps	%xmm0, %xmm14           # xmm14 = xmm14[0],xmm0[0],xmm14[1],xmm0[1]
1407d28ca: 45 0f c6 f6 40              	shufps	$0x40, %xmm14, %xmm14   # xmm14 = xmm14[0,0,0,1]
1407d28cf: 0f c6 ca 40                 	shufps	$0x40, %xmm2, %xmm1     # xmm1 = xmm1[0,0],xmm2[0,1]
1407d28d3: 0f c6 c9 78                 	shufps	$0x78, %xmm1, %xmm1     # xmm1 = xmm1[0,2,3,1]
1407d28d7: 41 0f 59 ce                 	mulps	%xmm14, %xmm1
1407d28db: f3 0f 12 c0                 	movsldup	%xmm0, %xmm0            # xmm0 = xmm0[0,0,2,2]
1407d28df: 0f 59 c2                    	mulps	%xmm2, %xmm0
1407d28e2: 0f 5c f1                    	subps	%xmm1, %xmm6
1407d28e5: 44 0f 5c e0                 	subps	%xmm0, %xmm12
1407d28e9: eb 27                       	jmp	0x1407d2912 <.text+0x7d1912>
1407d28eb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1407d28f0: 41 0f 14 c6                 	unpcklps	%xmm14, %xmm0           # xmm0 = xmm0[0],xmm14[0],xmm0[1],xmm14[1]
1407d28f4: 0f c6 c0 40                 	shufps	$0x40, %xmm0, %xmm0     # xmm0 = xmm0[0,0,0,1]
1407d28f8: 0f c6 ca 40                 	shufps	$0x40, %xmm2, %xmm1     # xmm1 = xmm1[0,0],xmm2[0,1]
1407d28fc: 0f c6 c9 78                 	shufps	$0x78, %xmm1, %xmm1     # xmm1 = xmm1[0,2,3,1]
1407d2900: 0f 59 c8                    	mulps	%xmm0, %xmm1
1407d2903: f3 41 0f 12 c6              	movsldup	%xmm14, %xmm0           # xmm0 = xmm14[0,0,2,2]
1407d2908: 0f 59 c2                    	mulps	%xmm2, %xmm0
1407d290b: 0f 58 f1                    	addps	%xmm1, %xmm6
1407d290e: 44 0f 58 e0                 	addps	%xmm0, %xmm12
1407d2912: 0f 29 75 20                 	movaps	%xmm6, 0x20(%rbp)
1407d2916: 44 0f 13 65 30              	movlps	%xmm12, 0x30(%rbp)
1407d291b: 48 8b 85 60 02 00 00        	movq	0x260(%rbp), %rax
1407d2922: 0f 10 00                    	movups	(%rax), %xmm0
1407d2925: 0f 29 85 50 01 00 00        	movaps	%xmm0, 0x150(%rbp)
1407d292c: 48 8b 85 68 02 00 00        	movq	0x268(%rbp), %rax
1407d2933: f3 0f 7e 00                 	movq	(%rax), %xmm0
1407d2937: 66 0f d6 85 c0 00 00 00     	movq	%xmm0, 0xc0(%rbp)
1407d293f: 48 8d 85 c4 00 00 00        	leaq	0xc4(%rbp), %rax
1407d2946: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
1407d294b: 48 8d 85 c0 00 00 00        	leaq	0xc0(%rbp), %rax
1407d2952: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
1407d2957: 48 8d 85 58 01 00 00        	leaq	0x158(%rbp), %rax
1407d295e: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407d2963: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
1407d2967: 48 8d 55 20                 	leaq	0x20(%rbp), %rdx
1407d296b: 4c 8d 45 2c                 	leaq	0x2c(%rbp), %r8
1407d296f: 4c 8d 8d 50 01 00 00        	leaq	0x150(%rbp), %r9
1407d2976: e8 15 c3 00 00              	callq	0x1407dec90 <.text+0x7ddc90>
1407d297b: 41 b6 01                    	movb	$0x1, %r14b
1407d297e: 80 bd c4 02 00 00 00        	cmpb	$0x0, 0x2c4(%rbp)
1407d2985: 74 59                       	je	0x1407d29e0 <.text+0x7d19e0>
1407d2987: 41 83 e4 07                 	andl	$0x7, %r12d
1407d298b: 4b 8d 04 e7                 	leaq	(%r15,%r12,8), %rax
1407d298f: 48 8b 8d 78 02 00 00        	movq	0x278(%rbp), %rcx
1407d2996: 48 8b 09                    	movq	(%rcx), %rcx
1407d2999: 48 0f a3 c1                 	btq	%rax, %rcx
1407d299d: 4c 8d bd 80 01 00 00        	leaq	0x180(%rbp), %r15
1407d29a4: 4c 8b a5 70 02 00 00        	movq	0x270(%rbp), %r12
1407d29ab: 72 41                       	jb	0x1407d29ee <.text+0x7d19ee>
1407d29ad: 48 c7 85 20 02 00 00 01 00 00 00    	movq	$0x1, 0x220(%rbp)
1407d29b8: 66 45 85 ed                 	testw	%r13w, %r13w
1407d29bc: 0f 84 7e 11 00 00           	je	0x1407d3b40 <.text+0x7d2b40>
1407d29c2: 48 ff c0                    	incq	%rax
1407d29c5: 89 c1                       	movl	%eax, %ecx
1407d29c7: c1 e9 06                    	shrl	$0x6, %ecx
1407d29ca: 48 8b 95 78 02 00 00        	movq	0x278(%rbp), %rdx
1407d29d1: 48 8b 0c ca                 	movq	(%rdx,%rcx,8), %rcx
1407d29d5: 48 0f a3 c1                 	btq	%rax, %rcx
1407d29d9: 41 0f 92 c6                 	setb	%r14b
1407d29dd: eb 0f                       	jmp	0x1407d29ee <.text+0x7d19ee>
1407d29df: 90                          	nop
1407d29e0: 4c 8d bd 80 01 00 00        	leaq	0x180(%rbp), %r15
1407d29e7: 4c 8b a5 70 02 00 00        	movq	0x270(%rbp), %r12
1407d29ee: 0f 28 85 b0 00 00 00        	movaps	0xb0(%rbp), %xmm0
1407d29f5: 48 8d 85 90 01 00 00        	leaq	0x190(%rbp), %rax
1407d29fc: 0f 29 40 70                 	movaps	%xmm0, 0x70(%rax)
1407d2a00: 0f 28 85 a0 00 00 00        	movaps	0xa0(%rbp), %xmm0
1407d2a07: 0f 29 40 60                 	movaps	%xmm0, 0x60(%rax)
1407d2a0b: 0f 28 85 90 00 00 00        	movaps	0x90(%rbp), %xmm0
1407d2a12: 0f 29 40 50                 	movaps	%xmm0, 0x50(%rax)
1407d2a16: 0f 28 85 80 00 00 00        	movaps	0x80(%rbp), %xmm0
1407d2a1d: 0f 29 40 40                 	movaps	%xmm0, 0x40(%rax)
1407d2a21: 66 0f 6f 45 40              	movdqa	0x40(%rbp), %xmm0
1407d2a26: 0f 28 4d 50                 	movaps	0x50(%rbp), %xmm1
1407d2a2a: 0f 28 55 60                 	movaps	0x60(%rbp), %xmm2
1407d2a2e: 66 0f 28 5d 70              	movapd	0x70(%rbp), %xmm3
1407d2a33: 66 0f 29 58 30              	movapd	%xmm3, 0x30(%rax)
1407d2a38: 0f 29 50 20                 	movaps	%xmm2, 0x20(%rax)
1407d2a3c: 0f 29 48 10                 	movaps	%xmm1, 0x10(%rax)
1407d2a40: 66 0f 7f 00                 	movdqa	%xmm0, (%rax)
1407d2a44: 48 c7 85 80 01 00 00 01 00 00 00    	movq	$0x1, 0x180(%rbp)
1407d2a4f: 48 c7 85 88 01 00 00 01 00 00 00    	movq	$0x1, 0x188(%rbp)
1407d2a5a: 0f b6 05 a8 21 e6 02        	movzbl	0x2e621a8(%rip), %eax   # 0x143634c09
1407d2a61: b9 90 00 00 00              	movl	$0x90, %ecx
1407d2a66: ba 10 00 00 00              	movl	$0x10, %edx
1407d2a6b: e8 a0 11 e4 ff              	callq	0x140613c10 <.text+0x612c10>
1407d2a70: 48 85 c0                    	testq	%rax, %rax
1407d2a73: 0f 84 ed 0f 00 00           	je	0x1407d3a66 <.text+0x7d2a66>
1407d2a79: 48 89 c7                    	movq	%rax, %rdi
1407d2a7c: 41 b8 90 00 00 00           	movl	$0x90, %r8d
1407d2a82: 48 89 c1                    	movq	%rax, %rcx
1407d2a85: 4c 89 fa                    	movq	%r15, %rdx
1407d2a88: e8 73 11 15 02              	callq	0x142923c00 <.text+0x2922c00>
1407d2a8d: b0 10                       	movb	$0x10, %al
1407d2a8f: 44 28 f0                    	subb	%r14b, %al
1407d2a92: 48 8b 8d c8 02 00 00        	movq	0x2c8(%rbp), %rcx
1407d2a99: 48 8b 09                    	movq	(%rcx), %rcx
1407d2a9c: 48 89 8d 88 01 00 00        	movq	%rcx, 0x188(%rbp)
1407d2aa3: 44 89 a5 84 01 00 00        	movl	%r12d, 0x184(%rbp)
1407d2aaa: 88 85 80 01 00 00           	movb	%al, 0x180(%rbp)
1407d2ab0: c6 85 d6 02 00 00 00        	movb	$0x0, 0x2d6(%rbp)
1407d2ab7: 48 8d 8d 30 01 00 00        	leaq	0x130(%rbp), %rcx
1407d2abe: 48 89 fa                    	movq	%rdi, %rdx
1407d2ac1: 4d 89 f8                    	movq	%r15, %r8
1407d2ac4: e8 47 c0 0f 00              	callq	0x1408ceb10 <.text+0x8cdb10>
1407d2ac9: 48 8d bd 20 02 00 00        	leaq	0x220(%rbp), %rdi
1407d2ad0: 8b 85 38 02 00 00           	movl	0x238(%rbp), %eax
1407d2ad6: 48 8b 8d c8 00 00 00        	movq	0xc8(%rbp), %rcx
1407d2add: 89 01                       	movl	%eax, (%rcx)
1407d2adf: 8b 85 3c 02 00 00           	movl	0x23c(%rbp), %eax
1407d2ae5: 48 8b 8d d0 00 00 00        	movq	0xd0(%rbp), %rcx
1407d2aec: 89 01                       	movl	%eax, (%rcx)
1407d2aee: 48 8b 85 c8 02 00 00        	movq	0x2c8(%rbp), %rax
1407d2af5: 48 8b 00                    	movq	(%rax), %rax
1407d2af8: 48 89 85 68 01 00 00        	movq	%rax, 0x168(%rbp)
1407d2aff: 44 89 a5 70 01 00 00        	movl	%r12d, 0x170(%rbp)
1407d2b06: f3 0f 6f 85 30 01 00 00     	movdqu	0x130(%rbp), %xmm0
1407d2b0e: 0f 10 8d 40 01 00 00        	movups	0x140(%rbp), %xmm1
1407d2b15: 0f 29 8d a0 02 00 00        	movaps	%xmm1, 0x2a0(%rbp)
1407d2b1c: 66 0f 7f 85 90 02 00 00     	movdqa	%xmm0, 0x290(%rbp)
1407d2b24: f2 44 0f 11 95 20 02 00 00  	movsd	%xmm10, 0x220(%rbp)
1407d2b2d: c7 85 28 02 00 00 00 00 00 00       	movl	$0x0, 0x228(%rbp)
1407d2b37: c6 85 d4 02 00 00 01        	movb	$0x1, 0x2d4(%rbp)
1407d2b3e: 4c 89 f9                    	movq	%r15, %rcx
1407d2b41: 48 89 fa                    	movq	%rdi, %rdx
1407d2b44: e8 e7 ef 4b 00              	callq	0x140c91b30 <.text+0xc90b30>
1407d2b49: c6 85 d4 02 00 00 00        	movb	$0x0, 0x2d4(%rbp)
1407d2b50: 4c 89 7c 24 28              	movq	%r15, 0x28(%rsp)
1407d2b55: 48 8d 85 90 02 00 00        	leaq	0x290(%rbp), %rax
1407d2b5c: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407d2b61: 48 89 f9                    	movq	%rdi, %rcx
1407d2b64: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
1407d2b6b: 4c 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %r8
1407d2b72: 4c 8d 8d 68 01 00 00        	leaq	0x168(%rbp), %r9
1407d2b79: e8 22 71 ac 00              	callq	0x141299ca0 <.text+0x1298ca0>
1407d2b7e: 48 8b 85 20 02 00 00        	movq	0x220(%rbp), %rax
1407d2b85: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
1407d2b89: 8b 85 28 02 00 00           	movl	0x228(%rbp), %eax
1407d2b8f: 89 45 c8                    	movl	%eax, -0x38(%rbp)
1407d2b92: 8b 85 b8 02 00 00           	movl	0x2b8(%rbp), %eax
1407d2b98: 48 8b 8d 50 02 00 00        	movq	0x250(%rbp), %rcx
1407d2b9f: 89 01                       	movl	%eax, (%rcx)
1407d2ba1: 48 8b 85 c8 02 00 00        	movq	0x2c8(%rbp), %rax
1407d2ba8: 48 8b 00                    	movq	(%rax), %rax
1407d2bab: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
1407d2bb2: 44 89 a5 88 01 00 00        	movl	%r12d, 0x188(%rbp)
1407d2bb9: 66 0f 57 db                 	xorpd	%xmm3, %xmm3
1407d2bbd: 45 84 f6                    	testb	%r14b, %r14b
1407d2bc0: 75 05                       	jne	0x1407d2bc7 <.text+0x7d1bc7>
1407d2bc2: 66 41 0f 28 d8              	movapd	%xmm8, %xmm3
1407d2bc7: c6 85 d6 02 00 00 00        	movb	$0x0, 0x2d6(%rbp)
1407d2bce: 48 89 f1                    	movq	%rsi, %rcx
1407d2bd1: 4c 89 fa                    	movq	%r15, %rdx
1407d2bd4: 4c 8d 45 c0                 	leaq	-0x40(%rbp), %r8
1407d2bd8: e8 43 71 09 00              	callq	0x140869d20 <.text+0x868d20>
1407d2bdd: 48 8b 85 80 02 00 00        	movq	0x280(%rbp), %rax
1407d2be4: 4c 8b 95 58 02 00 00        	movq	0x258(%rbp), %r10
1407d2beb: 48 3b 45 f0                 	cmpq	-0x10(%rbp), %rax
1407d2bef: 0f 85 3b fa ff ff           	jne	0x1407d2630 <.text+0x7d1630>
1407d2bf5: e9 86 04 00 00              	jmp	0x1407d3080 <.text+0x7d2080>
1407d2bfa: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1407d2c00: 49 8d 46 01                 	leaq	0x1(%r14), %rax
1407d2c04: 48 89 85 28 01 00 00        	movq	%rax, 0x128(%rbp)
1407d2c0b: 41 83 e4 07                 	andl	$0x7, %r12d
1407d2c0f: 4b 8d 04 e7                 	leaq	(%r15,%r12,8), %rax
1407d2c13: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
1407d2c1a: 45 31 c0                    	xorl	%r8d, %r8d
1407d2c1d: 31 c0                       	xorl	%eax, %eax
1407d2c1f: 4c 89 75 00                 	movq	%r14, (%rbp)
1407d2c23: 4c 8d ad 50 01 00 00        	leaq	0x150(%rbp), %r13
1407d2c2a: 4c 89 4d 08                 	movq	%r9, 0x8(%rbp)
1407d2c2e: 49 39 f8                    	cmpq	%rdi, %r8
1407d2c31: 4c 8b 95 58 02 00 00        	movq	0x258(%rbp), %r10
1407d2c38: 0f 83 22 04 00 00           	jae	0x1407d3060 <.text+0x7d2060>
1407d2c3e: 66 90                       	nop
1407d2c40: 49 8d 48 01                 	leaq	0x1(%r8), %rcx
1407d2c44: a8 01                       	testb	$0x1, %al
1407d2c46: 74 28                       	je	0x1407d2c70 <.text+0x7d1c70>
1407d2c48: 4c 8b a5 88 02 00 00        	movq	0x288(%rbp), %r12
1407d2c4f: 4c 89 85 88 02 00 00        	movq	%r8, 0x288(%rbp)
1407d2c56: 49 89 cf                    	movq	%rcx, %r15
1407d2c59: 49 39 fc                    	cmpq	%rdi, %r12
1407d2c5c: 72 32                       	jb	0x1407d2c90 <.text+0x7d1c90>
1407d2c5e: e9 17 0e 00 00              	jmp	0x1407d3a7a <.text+0x7d2a7a>
1407d2c63: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1407d2c70: 48 39 f9                    	cmpq	%rdi, %rcx
1407d2c73: 0f 83 f3 03 00 00           	jae	0x1407d306c <.text+0x7d206c>
1407d2c79: 4d 89 c4                    	movq	%r8, %r12
1407d2c7c: 4d 8d 78 02                 	leaq	0x2(%r8), %r15
1407d2c80: 48 89 8d 88 02 00 00        	movq	%rcx, 0x288(%rbp)
1407d2c87: 49 39 fc                    	cmpq	%rdi, %r12
1407d2c8a: 0f 83 ea 0d 00 00           	jae	0x1407d3a7a <.text+0x7d2a7a>
1407d2c90: 4b 8d 04 64                 	leaq	(%r12,%r12,2), %rax
1407d2c94: 41 8b 4c 81 08              	movl	0x8(%r9,%rax,4), %ecx
1407d2c99: 89 8d 70 01 00 00           	movl	%ecx, 0x170(%rbp)
1407d2c9f: 49 8b 04 81                 	movq	(%r9,%rax,4), %rax
1407d2ca3: 48 89 85 68 01 00 00        	movq	%rax, 0x168(%rbp)
1407d2caa: 48 8b 95 88 02 00 00        	movq	0x288(%rbp), %rdx
1407d2cb1: 48 8d 04 52                 	leaq	(%rdx,%rdx,2), %rax
1407d2cb5: 41 8b 4c 81 08              	movl	0x8(%r9,%rax,4), %ecx
1407d2cba: 89 8d 58 01 00 00           	movl	%ecx, 0x158(%rbp)
1407d2cc0: 49 8b 04 81                 	movq	(%r9,%rax,4), %rax
1407d2cc4: 48 89 85 50 01 00 00        	movq	%rax, 0x150(%rbp)
1407d2ccb: 48 8b 85 60 02 00 00        	movq	0x260(%rbp), %rax
1407d2cd2: 4e 8d 0c e0                 	leaq	(%rax,%r12,8), %r9
1407d2cd6: 48 8d 04 d0                 	leaq	(%rax,%rdx,8), %rax
1407d2cda: 4c 8b 85 68 02 00 00        	movq	0x268(%rbp), %r8
1407d2ce1: 4b 8d 0c a0                 	leaq	(%r8,%r12,4), %rcx
1407d2ce5: 49 8d 14 90                 	leaq	(%r8,%rdx,4), %rdx
1407d2ce9: 48 89 54 24 30              	movq	%rdx, 0x30(%rsp)
1407d2cee: 48 89 4c 24 28              	movq	%rcx, 0x28(%rsp)
1407d2cf3: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407d2cf8: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
1407d2cfc: 48 8d 95 68 01 00 00        	leaq	0x168(%rbp), %rdx
1407d2d03: 4d 89 e8                    	movq	%r13, %r8
1407d2d06: e8 85 bf 00 00              	callq	0x1407dec90 <.text+0x7ddc90>
1407d2d0b: 80 bd c4 02 00 00 00        	cmpb	$0x0, 0x2c4(%rbp)
1407d2d12: 4c 89 7d e8                 	movq	%r15, -0x18(%rbp)
1407d2d16: 74 36                       	je	0x1407d2d4e <.text+0x7d1d4e>
1407d2d18: 4c 89 a5 20 02 00 00        	movq	%r12, 0x220(%rbp)
1407d2d1f: 4d 39 f4                    	cmpq	%r14, %r12
1407d2d22: 0f 87 7d 0d 00 00           	ja	0x1407d3aa5 <.text+0x7d2aa5>
1407d2d28: 48 8b 85 00 01 00 00        	movq	0x100(%rbp), %rax
1407d2d2f: 4c 01 e0                    	addq	%r12, %rax
1407d2d32: 48 89 c1                    	movq	%rax, %rcx
1407d2d35: 48 c1 e9 06                 	shrq	$0x6, %rcx
1407d2d39: 48 8b 95 78 02 00 00        	movq	0x278(%rbp), %rdx
1407d2d40: 48 8b 0c ca                 	movq	(%rdx,%rcx,8), %rcx
1407d2d44: 48 0f a3 c1                 	btq	%rax, %rcx
1407d2d48: 0f 83 14 02 00 00           	jae	0x1407d2f62 <.text+0x7d1f62>
1407d2d4e: 48 c7 85 80 01 00 00 01 00 00 00    	movq	$0x1, 0x180(%rbp)
1407d2d59: 48 c7 85 88 01 00 00 01 00 00 00    	movq	$0x1, 0x188(%rbp)
1407d2d64: 0f 28 85 b0 00 00 00        	movaps	0xb0(%rbp), %xmm0
1407d2d6b: 48 8d 85 90 01 00 00        	leaq	0x190(%rbp), %rax
1407d2d72: 0f 29 40 70                 	movaps	%xmm0, 0x70(%rax)
1407d2d76: 0f 28 85 a0 00 00 00        	movaps	0xa0(%rbp), %xmm0
1407d2d7d: 0f 29 40 60                 	movaps	%xmm0, 0x60(%rax)
1407d2d81: 0f 28 85 90 00 00 00        	movaps	0x90(%rbp), %xmm0
1407d2d88: 0f 29 40 50                 	movaps	%xmm0, 0x50(%rax)
1407d2d8c: 0f 28 85 80 00 00 00        	movaps	0x80(%rbp), %xmm0
1407d2d93: 0f 29 40 40                 	movaps	%xmm0, 0x40(%rax)
1407d2d97: 66 0f 6f 45 40              	movdqa	0x40(%rbp), %xmm0
1407d2d9c: 66 0f 6f 4d 50              	movdqa	0x50(%rbp), %xmm1
1407d2da1: 66 0f 6f 55 60              	movdqa	0x60(%rbp), %xmm2
1407d2da6: 66 0f 28 5d 70              	movapd	0x70(%rbp), %xmm3
1407d2dab: 66 0f 29 58 30              	movapd	%xmm3, 0x30(%rax)
1407d2db0: 66 0f 7f 50 20              	movdqa	%xmm2, 0x20(%rax)
1407d2db5: 66 0f 7f 48 10              	movdqa	%xmm1, 0x10(%rax)
1407d2dba: 66 0f 7f 00                 	movdqa	%xmm0, (%rax)
1407d2dbe: 0f b6 05 44 1e e6 02        	movzbl	0x2e61e44(%rip), %eax   # 0x143634c09
1407d2dc5: b9 90 00 00 00              	movl	$0x90, %ecx
1407d2dca: ba 10 00 00 00              	movl	$0x10, %edx
1407d2dcf: e8 3c 0e e4 ff              	callq	0x140613c10 <.text+0x612c10>
1407d2dd4: 48 85 c0                    	testq	%rax, %rax
1407d2dd7: 0f 84 b4 0c 00 00           	je	0x1407d3a91 <.text+0x7d2a91>
1407d2ddd: 49 89 c7                    	movq	%rax, %r15
1407d2de0: 41 b8 90 00 00 00           	movl	$0x90, %r8d
1407d2de6: 48 89 c1                    	movq	%rax, %rcx
1407d2de9: 4c 8d ad 80 01 00 00        	leaq	0x180(%rbp), %r13
1407d2df0: 4c 89 ea                    	movq	%r13, %rdx
1407d2df3: e8 08 0e 15 02              	callq	0x142923c00 <.text+0x2922c00>
1407d2df8: 66 0f 57 f6                 	xorpd	%xmm6, %xmm6
1407d2dfc: b0 0f                       	movb	$0xf, %al
1407d2dfe: 4c 8b b5 c8 02 00 00        	movq	0x2c8(%rbp), %r14
1407d2e05: 44 03 a5 70 02 00 00        	addl	0x270(%rbp), %r12d
1407d2e0c: 49 8b 0e                    	movq	(%r14), %rcx
1407d2e0f: 48 89 8d 88 01 00 00        	movq	%rcx, 0x188(%rbp)
1407d2e16: 44 89 a5 84 01 00 00        	movl	%r12d, 0x184(%rbp)
1407d2e1d: 88 85 80 01 00 00           	movb	%al, 0x180(%rbp)
1407d2e23: c6 85 d7 02 00 00 00        	movb	$0x0, 0x2d7(%rbp)
1407d2e2a: 48 8d 8d 30 01 00 00        	leaq	0x130(%rbp), %rcx
1407d2e31: 4c 89 fa                    	movq	%r15, %rdx
1407d2e34: 4d 89 e8                    	movq	%r13, %r8
1407d2e37: e8 d4 bc 0f 00              	callq	0x1408ceb10 <.text+0x8cdb10>
1407d2e3c: 8b 85 38 02 00 00           	movl	0x238(%rbp), %eax
1407d2e42: 48 8b 8d c8 00 00 00        	movq	0xc8(%rbp), %rcx
1407d2e49: 89 01                       	movl	%eax, (%rcx)
1407d2e4b: 8b 85 3c 02 00 00           	movl	0x23c(%rbp), %eax
1407d2e51: 48 8b 8d d0 00 00 00        	movq	0xd0(%rbp), %rcx
1407d2e58: 89 01                       	movl	%eax, (%rcx)
1407d2e5a: 49 8b 06                    	movq	(%r14), %rax
1407d2e5d: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
1407d2e61: 44 89 65 28                 	movl	%r12d, 0x28(%rbp)
1407d2e65: f3 0f 6f 85 30 01 00 00     	movdqu	0x130(%rbp), %xmm0
1407d2e6d: f3 0f 6f 8d 40 01 00 00     	movdqu	0x140(%rbp), %xmm1
1407d2e75: 66 0f 7f 8d a0 02 00 00     	movdqa	%xmm1, 0x2a0(%rbp)
1407d2e7d: 66 0f 7f 85 90 02 00 00     	movdqa	%xmm0, 0x290(%rbp)
1407d2e85: f2 44 0f 11 95 20 02 00 00  	movsd	%xmm10, 0x220(%rbp)
1407d2e8e: c7 85 28 02 00 00 00 00 00 00       	movl	$0x0, 0x228(%rbp)
1407d2e98: c6 85 d3 02 00 00 01        	movb	$0x1, 0x2d3(%rbp)
1407d2e9f: 4c 89 e9                    	movq	%r13, %rcx
1407d2ea2: 4c 8d bd 20 02 00 00        	leaq	0x220(%rbp), %r15
1407d2ea9: 4c 89 fa                    	movq	%r15, %rdx
1407d2eac: e8 7f ec 4b 00              	callq	0x140c91b30 <.text+0xc90b30>
1407d2eb1: c6 85 d3 02 00 00 00        	movb	$0x0, 0x2d3(%rbp)
1407d2eb8: 4c 89 6c 24 28              	movq	%r13, 0x28(%rsp)
1407d2ebd: 48 8d 85 90 02 00 00        	leaq	0x290(%rbp), %rax
1407d2ec4: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407d2ec9: 4c 89 f9                    	movq	%r15, %rcx
1407d2ecc: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
1407d2ed3: 4c 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %r8
1407d2eda: 4c 8d 4d 20                 	leaq	0x20(%rbp), %r9
1407d2ede: e8 bd 6d ac 00              	callq	0x141299ca0 <.text+0x1298ca0>
1407d2ee3: 48 8b 85 20 02 00 00        	movq	0x220(%rbp), %rax
1407d2eea: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
1407d2eee: 8b 85 28 02 00 00           	movl	0x228(%rbp), %eax
1407d2ef4: 89 45 e0                    	movl	%eax, -0x20(%rbp)
1407d2ef7: 8b 85 b8 02 00 00           	movl	0x2b8(%rbp), %eax
1407d2efd: 48 8b 8d 50 02 00 00        	movq	0x250(%rbp), %rcx
1407d2f04: 89 01                       	movl	%eax, (%rcx)
1407d2f06: 48 8b 85 c8 02 00 00        	movq	0x2c8(%rbp), %rax
1407d2f0d: 48 8b 00                    	movq	(%rax), %rax
1407d2f10: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
1407d2f17: 44 89 a5 88 01 00 00        	movl	%r12d, 0x188(%rbp)
1407d2f1e: c6 85 d7 02 00 00 00        	movb	$0x0, 0x2d7(%rbp)
1407d2f25: 48 89 f1                    	movq	%rsi, %rcx
1407d2f28: 4c 89 ea                    	movq	%r13, %rdx
1407d2f2b: 4c 8d 45 d8                 	leaq	-0x28(%rbp), %r8
1407d2f2f: 66 0f 28 de                 	movapd	%xmm6, %xmm3
1407d2f33: e8 e8 6d 09 00              	callq	0x140869d20 <.text+0x868d20>
1407d2f38: b0 01                       	movb	$0x1, %al
1407d2f3a: 4c 8d ad 50 01 00 00        	leaq	0x150(%rbp), %r13
1407d2f41: 4c 8b 75 00                 	movq	(%rbp), %r14
1407d2f45: 4c 8b 4d 08                 	movq	0x8(%rbp), %r9
1407d2f49: 4c 8b 45 e8                 	movq	-0x18(%rbp), %r8
1407d2f4d: 49 39 f8                    	cmpq	%rdi, %r8
1407d2f50: 4c 8b 95 58 02 00 00        	movq	0x258(%rbp), %r10
1407d2f57: 0f 82 e3 fc ff ff           	jb	0x1407d2c40 <.text+0x7d1c40>
1407d2f5d: e9 fe 00 00 00              	jmp	0x1407d3060 <.text+0x7d2060>
1407d2f62: 48 8b 85 88 02 00 00        	movq	0x288(%rbp), %rax
1407d2f69: 48 89 85 20 02 00 00        	movq	%rax, 0x220(%rbp)
1407d2f70: 4c 39 f0                    	cmpq	%r14, %rax
1407d2f73: 0f 87 73 0c 00 00           	ja	0x1407d3bec <.text+0x7d2bec>
1407d2f79: 48 8b 8d 00 01 00 00        	movq	0x100(%rbp), %rcx
1407d2f80: 4c 8d 34 08                 	leaq	(%rax,%rcx), %r14
1407d2f84: 4c 89 f0                    	movq	%r14, %rax
1407d2f87: 48 c1 e8 06                 	shrq	$0x6, %rax
1407d2f8b: 48 8b 8d 78 02 00 00        	movq	0x278(%rbp), %rcx
1407d2f92: 4c 8b 2c c1                 	movq	(%rcx,%rax,8), %r13
1407d2f96: 48 c7 85 80 01 00 00 01 00 00 00    	movq	$0x1, 0x180(%rbp)
1407d2fa1: 48 c7 85 88 01 00 00 01 00 00 00    	movq	$0x1, 0x188(%rbp)
1407d2fac: 0f 28 85 b0 00 00 00        	movaps	0xb0(%rbp), %xmm0
1407d2fb3: 48 8d 85 90 01 00 00        	leaq	0x190(%rbp), %rax
1407d2fba: 0f 29 40 70                 	movaps	%xmm0, 0x70(%rax)
1407d2fbe: 0f 28 85 a0 00 00 00        	movaps	0xa0(%rbp), %xmm0
1407d2fc5: 0f 29 40 60                 	movaps	%xmm0, 0x60(%rax)
1407d2fc9: 0f 28 85 90 00 00 00        	movaps	0x90(%rbp), %xmm0
1407d2fd0: 0f 29 40 50                 	movaps	%xmm0, 0x50(%rax)
1407d2fd4: 0f 28 85 80 00 00 00        	movaps	0x80(%rbp), %xmm0
1407d2fdb: 0f 29 40 40                 	movaps	%xmm0, 0x40(%rax)
1407d2fdf: 66 0f 6f 45 40              	movdqa	0x40(%rbp), %xmm0
1407d2fe4: 66 0f 6f 4d 50              	movdqa	0x50(%rbp), %xmm1
1407d2fe9: 66 0f 6f 55 60              	movdqa	0x60(%rbp), %xmm2
1407d2fee: 66 0f 28 5d 70              	movapd	0x70(%rbp), %xmm3
1407d2ff3: 66 0f 29 58 30              	movapd	%xmm3, 0x30(%rax)
1407d2ff8: 66 0f 7f 50 20              	movdqa	%xmm2, 0x20(%rax)
1407d2ffd: 66 0f 7f 48 10              	movdqa	%xmm1, 0x10(%rax)
1407d3002: 66 0f 7f 00                 	movdqa	%xmm0, (%rax)
1407d3006: 0f b6 05 fc 1b e6 02        	movzbl	0x2e61bfc(%rip), %eax   # 0x143634c09
1407d300d: b9 90 00 00 00              	movl	$0x90, %ecx
1407d3012: ba 10 00 00 00              	movl	$0x10, %edx
1407d3017: e8 f4 0b e4 ff              	callq	0x140613c10 <.text+0x612c10>
1407d301c: 48 85 c0                    	testq	%rax, %rax
1407d301f: 0f 84 b3 0b 00 00           	je	0x1407d3bd8 <.text+0x7d2bd8>
1407d3025: 49 89 c7                    	movq	%rax, %r15
1407d3028: 41 b8 90 00 00 00           	movl	$0x90, %r8d
1407d302e: 48 89 c1                    	movq	%rax, %rcx
1407d3031: 48 8d 95 80 01 00 00        	leaq	0x180(%rbp), %rdx
1407d3038: e8 c3 0b 15 02              	callq	0x142923c00 <.text+0x2922c00>
1407d303d: 4d 0f a3 f5                 	btq	%r14, %r13
1407d3041: 73 08                       	jae	0x1407d304b <.text+0x7d204b>
1407d3043: 66 0f 57 f6                 	xorpd	%xmm6, %xmm6
1407d3047: b0 0f                       	movb	$0xf, %al
1407d3049: eb 07                       	jmp	0x1407d3052 <.text+0x7d2052>
1407d304b: b0 10                       	movb	$0x10, %al
1407d304d: 66 41 0f 28 f0              	movapd	%xmm8, %xmm6
1407d3052: 4c 8d ad 80 01 00 00        	leaq	0x180(%rbp), %r13
1407d3059: e9 a0 fd ff ff              	jmp	0x1407d2dfe <.text+0x7d1dfe>
1407d305e: 66 90                       	nop
1407d3060: 4c 8b 85 80 02 00 00        	movq	0x280(%rbp), %r8
1407d3067: e9 ad f5 ff ff              	jmp	0x1407d2619 <.text+0x7d1619>
1407d306c: 4c 8b 85 80 02 00 00        	movq	0x280(%rbp), %r8
1407d3073: e9 a1 f5 ff ff              	jmp	0x1407d2619 <.text+0x7d1619>
1407d3078: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1407d3080: 48 8b 85 78 01 00 00        	movq	0x178(%rbp), %rax
1407d3087: 48 83 e8 01                 	subq	$0x1, %rax
1407d308b: bf 00 00 00 00              	movl	$0x0, %edi
1407d3090: 48 0f 43 f8                 	cmovaeq	%rax, %rdi
1407d3094: 85 ff                       	testl	%edi, %edi
1407d3096: 48 8b 8d 10 01 00 00        	movq	0x110(%rbp), %rcx
1407d309d: b8 00 00 00 00              	movl	$0x0, %eax
1407d30a2: 4c 8b 8d 18 01 00 00        	movq	0x118(%rbp), %r9
1407d30a9: 0f 84 b0 f3 ff ff           	je	0x1407d245f <.text+0x7d145f>
1407d30af: 8b 85 b8 02 00 00           	movl	0x2b8(%rbp), %eax
1407d30b5: 48 8b 8d 50 02 00 00        	movq	0x250(%rbp), %rcx
1407d30bc: 89 01                       	movl	%eax, (%rcx)
1407d30be: 8b 93 28 01 00 00           	movl	0x128(%rbx), %edx
1407d30c4: 44 8b 83 2c 01 00 00        	movl	0x12c(%rbx), %r8d
1407d30cb: 48 8b 4d 18                 	movq	0x18(%rbp), %rcx
1407d30cf: 41 89 f9                    	movl	%edi, %r9d
1407d30d2: e8 c9 39 a9 00              	callq	0x141266aa0 <.text+0x1265aa0>
1407d30d7: 45 31 f6                    	xorl	%r14d, %r14d
1407d30da: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1407d30e0: 41 39 fe                    	cmpl	%edi, %r14d
1407d30e3: 4c 8d 85 80 01 00 00        	leaq	0x180(%rbp), %r8
1407d30ea: 73 4c                       	jae	0x1407d3138 <.text+0x7d2138>
1407d30ec: 8b 85 b8 02 00 00           	movl	0x2b8(%rbp), %eax
1407d30f2: 48 8b 8d 50 02 00 00        	movq	0x250(%rbp), %rcx
1407d30f9: 89 01                       	movl	%eax, (%rcx)
1407d30fb: 8b 83 28 01 00 00           	movl	0x128(%rbx), %eax
1407d3101: 8b 8b 2c 01 00 00           	movl	0x12c(%rbx), %ecx
1407d3107: 89 45 40                    	movl	%eax, 0x40(%rbp)
1407d310a: 89 4d 44                    	movl	%ecx, 0x44(%rbp)
1407d310d: 44 89 75 48                 	movl	%r14d, 0x48(%rbp)
1407d3111: 41 ff c6                    	incl	%r14d
1407d3114: 89 85 80 01 00 00           	movl	%eax, 0x180(%rbp)
1407d311a: 89 8d 84 01 00 00           	movl	%ecx, 0x184(%rbp)
1407d3120: 44 89 b5 88 01 00 00        	movl	%r14d, 0x188(%rbp)
1407d3127: 48 89 f1                    	movq	%rsi, %rcx
1407d312a: 48 8d 55 40                 	leaq	0x40(%rbp), %rdx
1407d312e: 45 31 c9                    	xorl	%r9d, %r9d
1407d3131: e8 ba 6e 09 00              	callq	0x140869ff0 <.text+0x868ff0>
1407d3136: eb a8                       	jmp	0x1407d30e0 <.text+0x7d20e0>
1407d3138: 48 8b 8d 10 01 00 00        	movq	0x110(%rbp), %rcx
1407d313f: 31 c0                       	xorl	%eax, %eax
1407d3141: 4c 8b 8d 18 01 00 00        	movq	0x118(%rbp), %r9
1407d3148: e9 12 f3 ff ff              	jmp	0x1407d245f <.text+0x7d145f>
1407d314d: e8 ae 38 44 00              	callq	0x140c16a00 <.text+0xc15a00>
1407d3152: 41 89 c6                    	movl	%eax, %r14d
1407d3155: 45 84 f6                    	testb	%r14b, %r14b
1407d3158: 0f 84 73 01 00 00           	je	0x1407d32d1 <.text+0x7d22d1>
1407d315e: 4c 8d 05 db 4d 2b 02        	leaq	0x22b4ddb(%rip), %r8    # 0x142a87f40
1407d3165: 48 8d 3d 94 4d 2b 02        	leaq	0x22b4d94(%rip), %rdi   # 0x142a87f00
1407d316c: 0f 1f 40 00                 	nopl	(%rax)
1407d3170: 41 0f be 40 ff              	movsbl	-0x1(%r8), %eax
1407d3175: 85 c0                       	testl	%eax, %eax
1407d3177: 78 17                       	js	0x1407d3190 <.text+0x7d2190>
1407d3179: 49 ff c8                    	decq	%r8
1407d317c: 83 f8 5c                    	cmpl	$0x5c, %eax
1407d317f: 75 63                       	jne	0x1407d31e4 <.text+0x7d21e4>
1407d3181: eb 72                       	jmp	0x1407d31f5 <.text+0x7d21f5>
1407d3183: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1407d3190: 41 0f b6 48 fe              	movzbl	-0x2(%r8), %ecx
1407d3195: 80 f9 c0                    	cmpb	$-0x40, %cl
1407d3198: 7d 23                       	jge	0x1407d31bd <.text+0x7d21bd>
1407d319a: 41 0f b6 50 fd              	movzbl	-0x3(%r8), %edx
1407d319f: 80 fa c0                    	cmpb	$-0x40, %dl
1407d31a2: 7d 22                       	jge	0x1407d31c6 <.text+0x7d21c6>
1407d31a4: 45 0f b6 48 fc              	movzbl	-0x4(%r8), %r9d
1407d31a9: 49 83 c0 fc                 	addq	$-0x4, %r8
1407d31ad: 41 83 e1 07                 	andl	$0x7, %r9d
1407d31b1: 41 c1 e1 06                 	shll	$0x6, %r9d
1407d31b5: 83 e2 3f                    	andl	$0x3f, %edx
1407d31b8: 44 09 ca                    	orl	%r9d, %edx
1407d31bb: eb 10                       	jmp	0x1407d31cd <.text+0x7d21cd>
1407d31bd: 49 83 c0 fe                 	addq	$-0x2, %r8
1407d31c1: 83 e1 1f                    	andl	$0x1f, %ecx
1407d31c4: eb 0f                       	jmp	0x1407d31d5 <.text+0x7d21d5>
1407d31c6: 49 83 c0 fd                 	addq	$-0x3, %r8
1407d31ca: 83 e2 0f                    	andl	$0xf, %edx
1407d31cd: c1 e2 06                    	shll	$0x6, %edx
1407d31d0: 83 e1 3f                    	andl	$0x3f, %ecx
1407d31d3: 09 d1                       	orl	%edx, %ecx
1407d31d5: c1 e1 06                    	shll	$0x6, %ecx
1407d31d8: 24 3f                       	andb	$0x3f, %al
1407d31da: 0f b6 c0                    	movzbl	%al, %eax
1407d31dd: 09 c8                       	orl	%ecx, %eax
1407d31df: 83 f8 5c                    	cmpl	$0x5c, %eax
1407d31e2: 74 11                       	je	0x1407d31f5 <.text+0x7d21f5>
1407d31e4: 83 f8 2f                    	cmpl	$0x2f, %eax
1407d31e7: 74 0c                       	je	0x1407d31f5 <.text+0x7d21f5>
1407d31e9: 49 39 f8                    	cmpq	%rdi, %r8
1407d31ec: 75 82                       	jne	0x1407d3170 <.text+0x7d2170>
1407d31ee: bb 40 00 00 00              	movl	$0x40, %ebx
1407d31f3: eb 51                       	jmp	0x1407d3246 <.text+0x7d2246>
1407d31f5: 48 8d 3d 04 4d 2b 02        	leaq	0x22b4d04(%rip), %rdi   # 0x142a87f00
1407d31fc: 49 29 f8                    	subq	%rdi, %r8
1407d31ff: 49 ff c0                    	incq	%r8
1407d3202: 74 37                       	je	0x1407d323b <.text+0x7d223b>
1407d3204: 49 83 f8 40                 	cmpq	$0x40, %r8
1407d3208: 73 2f                       	jae	0x1407d3239 <.text+0x7d2239>
1407d320a: 41 80 3c 38 bf              	cmpb	$-0x41, (%r8,%rdi)
1407d320f: 7f 2a                       	jg	0x1407d323b <.text+0x7d223b>
1407d3211: 48 8d 05 38 4a 2b 02        	leaq	0x22b4a38(%rip), %rax   # 0x142a87c50
1407d3218: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407d321d: 48 8d 0d dc 4c 2b 02        	leaq	0x22b4cdc(%rip), %rcx   # 0x142a87f00
1407d3224: ba 40 00 00 00              	movl	$0x40, %edx
1407d3229: 41 b9 40 00 00 00           	movl	$0x40, %r9d
1407d322f: e8 0c 67 10 02              	callq	0x1428d9940 <.text+0x28d8940>
1407d3234: e9 65 0a 00 00              	jmp	0x1407d3c9e <.text+0x7d2c9e>
1407d3239: 75 d6                       	jne	0x1407d3211 <.text+0x7d2211>
1407d323b: bb 40 00 00 00              	movl	$0x40, %ebx
1407d3240: 4c 29 c3                    	subq	%r8, %rbx
1407d3243: 4c 01 c7                    	addq	%r8, %rdi
1407d3246: e8 e5 4e 44 00              	callq	0x140c18130 <.text+0xc17130>
1407d324b: 48 89 c1                    	movq	%rax, %rcx
1407d324e: 48 8b 00                    	movq	(%rax), %rax
1407d3251: 48 85 c0                    	testq	%rax, %rax
1407d3254: 74 0f                       	je	0x1407d3265 <.text+0x7d2265>
1407d3256: 83 f8 01                    	cmpl	$0x1, %eax
1407d3259: 0f 85 25 0a 00 00           	jne	0x1407d3c84 <.text+0x7d2c84>
1407d325f: 48 83 c1 08                 	addq	$0x8, %rcx
1407d3263: eb 13                       	jmp	0x1407d3278 <.text+0x7d2278>
1407d3265: 31 d2                       	xorl	%edx, %edx
1407d3267: e8 14 f3 11 02              	callq	0x1428f2580 <.text+0x28f1580>
1407d326c: 48 89 c1                    	movq	%rax, %rcx
1407d326f: 48 85 c0                    	testq	%rax, %rax
1407d3272: 0f 84 0c 0a 00 00           	je	0x1407d3c84 <.text+0x7d2c84>
1407d3278: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1407d327c: 0f 85 0f 07 00 00           	jne	0x1407d3991 <.text+0x7d2991>
1407d3282: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1407d3289: 48 89 8d c8 02 00 00        	movq	%rcx, 0x2c8(%rbp)
1407d3290: 48 83 c1 08                 	addq	$0x8, %rcx
1407d3294: 48 89 5c 24 20              	movq	%rbx, 0x20(%rsp)
1407d3299: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
1407d32a2: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
1407d32ab: 48 8d 15 96 4d 2b 02        	leaq	0x22b4d96(%rip), %rdx   # 0x142a88048
1407d32b2: 41 b8 0d 00 00 00           	movl	$0xd, %r8d
1407d32b8: 49 89 f9                    	movq	%rdi, %r9
1407d32bb: e8 80 3a 44 00              	callq	0x140c16d40 <.text+0xc15d40>
1407d32c0: 48 89 85 88 02 00 00        	movq	%rax, 0x288(%rbp)
1407d32c7: 48 8b 85 c8 02 00 00        	movq	0x2c8(%rbp), %rax
1407d32ce: 48 ff 00                    	incq	(%rax)
1407d32d1: 44 88 b5 80 02 00 00        	movb	%r14b, 0x280(%rbp)
1407d32d8: 48 c7 85 80 01 00 00 00 00 00 00    	movq	$0x0, 0x180(%rbp)
1407d32e3: 48 c7 85 88 01 00 00 04 00 00 00    	movq	$0x4, 0x188(%rbp)
1407d32ee: 48 c7 85 90 01 00 00 00 00 00 00    	movq	$0x0, 0x190(%rbp)
1407d32f9: 48 8b 8d 18 02 00 00        	movq	0x218(%rbp), %rcx
1407d3300: e8 ab b5 12 00              	callq	0x1408fe8b0 <.text+0x8fd8b0>
1407d3305: 48 8b 48 08                 	movq	0x8(%rax), %rcx
1407d3309: 48 8b 40 10                 	movq	0x10(%rax), %rax
1407d330d: 48 8d 04 80                 	leaq	(%rax,%rax,4), %rax
1407d3311: 48 8d 04 c1                 	leaq	(%rcx,%rax,8), %rax
1407d3315: 48 89 85 78 02 00 00        	movq	%rax, 0x278(%rbp)
1407d331c: 48 8d 86 a0 01 00 00        	leaq	0x1a0(%rsi), %rax
1407d3323: 48 89 85 c8 02 00 00        	movq	%rax, 0x2c8(%rbp)
1407d332a: 41 bf 04 00 00 00           	movl	$0x4, %r15d
1407d3330: 66 0f 76 f6                 	pcmpeqd	%xmm6, %xmm6
1407d3334: eb 11                       	jmp	0x1407d3347 <.text+0x7d2347>
1407d3336: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1407d3340: 48 8b 8d 70 02 00 00        	movq	0x270(%rbp), %rcx
1407d3347: 48 3b 8d 78 02 00 00        	cmpq	0x278(%rbp), %rcx
1407d334e: 0f 84 01 04 00 00           	je	0x1407d3755 <.text+0x7d2755>
1407d3354: 48 89 8d 70 02 00 00        	movq	%rcx, 0x270(%rbp)
1407d335b: 8b 79 20                    	movl	0x20(%rcx), %edi
1407d335e: 48 c7 85 90 01 00 00 00 00 00 00    	movq	$0x0, 0x190(%rbp)
1407d3369: 48 8b 8d 18 02 00 00        	movq	0x218(%rbp), %rcx
1407d3370: e8 3b b5 12 00              	callq	0x1408fe8b0 <.text+0x8fd8b0>
1407d3375: 89 7d 40                    	movl	%edi, 0x40(%rbp)
1407d3378: 48 89 c1                    	movq	%rax, %rcx
1407d337b: 48 8d 55 40                 	leaq	0x40(%rbp), %rdx
1407d337f: e8 cc aa a8 00              	callq	0x14125de50 <.text+0x125ce50>
1407d3384: 48 85 c0                    	testq	%rax, %rax
1407d3387: 0f 84 83 00 00 00           	je	0x1407d3410 <.text+0x7d2410>
1407d338d: 48 8b 48 10                 	movq	0x10(%rax), %rcx
1407d3391: 48 85 c9                    	testq	%rcx, %rcx
1407d3394: 74 7a                       	je	0x1407d3410 <.text+0x7d2410>
1407d3396: 48 8b 58 08                 	movq	0x8(%rax), %rbx
1407d339a: 4c 8d 34 cb                 	leaq	(%rbx,%rcx,8), %r14
1407d339e: 45 31 e4                    	xorl	%r12d, %r12d
1407d33a1: 4c 89 f8                    	movq	%r15, %rax
1407d33a4: eb 1d                       	jmp	0x1407d33c3 <.text+0x7d23c3>
1407d33a6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1407d33b0: 46 89 3c a0                 	movl	%r15d, (%rax,%r12,4)
1407d33b4: 49 ff c4                    	incq	%r12
1407d33b7: 4c 89 a5 90 01 00 00        	movq	%r12, 0x190(%rbp)
1407d33be: 4c 39 f3                    	cmpq	%r14, %rbx
1407d33c1: 74 5d                       	je	0x1407d3420 <.text+0x7d2420>
1407d33c3: 44 8b 3b                    	movl	(%rbx), %r15d
1407d33c6: 48 83 c3 08                 	addq	$0x8, %rbx
1407d33ca: 4c 3b a5 80 01 00 00        	cmpq	0x180(%rbp), %r12
1407d33d1: 75 dd                       	jne	0x1407d33b0 <.text+0x7d23b0>
1407d33d3: 4d 89 f0                    	movq	%r14, %r8
1407d33d6: 49 29 d8                    	subq	%rbx, %r8
1407d33d9: 49 c1 e8 03                 	shrq	$0x3, %r8
1407d33dd: 49 ff c0                    	incq	%r8
1407d33e0: 48 c7 44 24 20 04 00 00 00  	movq	$0x4, 0x20(%rsp)
1407d33e9: 41 b9 04 00 00 00           	movl	$0x4, %r9d
1407d33ef: 48 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %rcx
1407d33f6: 4c 89 e2                    	movq	%r12, %rdx
1407d33f9: e8 22 4b 0d 02              	callq	0x1428a7f20 <.text+0x28a6f20>
1407d33fe: 48 8b 85 88 01 00 00        	movq	0x188(%rbp), %rax
1407d3405: eb a9                       	jmp	0x1407d33b0 <.text+0x7d23b0>
1407d3407: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1407d3410: 45 31 e4                    	xorl	%r12d, %r12d
1407d3413: eb 12                       	jmp	0x1407d3427 <.text+0x7d2427>
1407d3415: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1407d3420: 4c 8b bd 88 01 00 00        	movq	0x188(%rbp), %r15
1407d3427: 48 83 85 70 02 00 00 28     	addq	$0x28, 0x270(%rbp)
1407d342f: 4f 8d 34 a7                 	leaq	(%r15,%r12,4), %r14
1407d3433: 4c 89 f8                    	movq	%r15, %rax
1407d3436: 48 c7 85 58 02 00 00 00 00 00 00    	movq	$0x0, 0x258(%rbp)
1407d3441: 4c 89 bd 68 02 00 00        	movq	%r15, 0x268(%rbp)
1407d3448: eb 14                       	jmp	0x1407d345e <.text+0x7d245e>
1407d344a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1407d3450: 4c 8b bd 68 02 00 00        	movq	0x268(%rbp), %r15
1407d3457: 48 8b 85 60 02 00 00        	movq	0x260(%rbp), %rax
1407d345e: 4c 39 f0                    	cmpq	%r14, %rax
1407d3461: 0f 84 d9 fe ff ff           	je	0x1407d3340 <.text+0x7d2340>
1407d3467: 48 8b 8d 58 02 00 00        	movq	0x258(%rbp), %rcx
1407d346e: 48 ff c1                    	incq	%rcx
1407d3471: 44 8b 20                    	movl	(%rax), %r12d
1407d3474: 48 83 c0 04                 	addq	$0x4, %rax
1407d3478: 48 89 85 60 02 00 00        	movq	%rax, 0x260(%rbp)
1407d347f: 48 89 c8                    	movq	%rcx, %rax
1407d3482: 48 89 8d 58 02 00 00        	movq	%rcx, 0x258(%rbp)
1407d3489: e9 92 00 00 00              	jmp	0x1407d3520 <.text+0x7d2520>
1407d348e: 66 90                       	nop
1407d3490: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
1407d3496: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
1407d349a: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
1407d349e: 66 44 0f d7 d3              	pmovmskb	%xmm3, %r10d
1407d34a3: 45 85 d2                    	testl	%r10d, %r10d
1407d34a6: 74 38                       	je	0x1407d34e0 <.text+0x7d24e0>
1407d34a8: f3 45 0f bc da              	tzcntl	%r10d, %r11d
1407d34ad: 4d 01 c3                    	addq	%r8, %r11
1407d34b0: 49 21 d3                    	andq	%rdx, %r11
1407d34b3: 49 f7 db                    	negq	%r11
1407d34b6: 4f 8d 1c 5b                 	leaq	(%r11,%r11,2), %r11
1407d34ba: f3 42 0f 7e 1c 98           	movq	(%rax,%r11,4), %xmm3
1407d34c0: 66 0f 76 d9                 	pcmpeqd	%xmm1, %xmm3
1407d34c4: 66 0f 70 db 50              	pshufd	$0x50, %xmm3, %xmm3     # xmm3 = xmm3[0,0,1,1]
1407d34c9: 66 0f 50 db                 	movmskpd	%xmm3, %ebx
1407d34cd: 83 fb 03                    	cmpl	$0x3, %ebx
1407d34d0: 74 2c                       	je	0x1407d34fe <.text+0x7d24fe>
1407d34d2: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
1407d34d6: 66 45 21 d3                 	andw	%r10w, %r11w
1407d34da: 45 89 da                    	movl	%r11d, %r10d
1407d34dd: 75 c9                       	jne	0x1407d34a8 <.text+0x7d24a8>
1407d34df: 90                          	nop
1407d34e0: 66 0f 74 d6                 	pcmpeqb	%xmm6, %xmm2
1407d34e4: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
1407d34e9: 45 85 d2                    	testl	%r10d, %r10d
1407d34ec: 75 27                       	jne	0x1407d3515 <.text+0x7d2515>
1407d34ee: 4d 01 c8                    	addq	%r9, %r8
1407d34f1: 49 83 c0 10                 	addq	$0x10, %r8
1407d34f5: 49 83 c1 10                 	addq	$0x10, %r9
1407d34f9: 49 21 d0                    	andq	%rdx, %r8
1407d34fc: eb 92                       	jmp	0x1407d3490 <.text+0x7d2490>
1407d34fe: 42 8b 5c 99 fc              	movl	-0x4(%rcx,%r11,4), %ebx
1407d3503: 85 db                       	testl	%ebx, %ebx
1407d3505: 0f 84 58 04 00 00           	je	0x1407d3963 <.text+0x7d2963>
1407d350b: ff cb                       	decl	%ebx
1407d350d: 8b 85 40 02 00 00           	movl	0x240(%rbp), %eax
1407d3513: eb 64                       	jmp	0x1407d3579 <.text+0x7d2579>
1407d3515: 31 c0                       	xorl	%eax, %eax
1407d3517: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1407d3520: 48 85 c0                    	testq	%rax, %rax
1407d3523: 0f 85 10 02 00 00           	jne	0x1407d3739 <.text+0x7d2739>
1407d3529: 4d 39 f7                    	cmpq	%r14, %r15
1407d352c: 0f 84 1e ff ff ff           	je	0x1407d3450 <.text+0x7d2450>
1407d3532: 45 8b 2f                    	movl	(%r15), %r13d
1407d3535: 89 f9                       	movl	%edi, %ecx
1407d3537: 44 89 e2                    	movl	%r12d, %edx
1407d353a: e8 21 dd 12 00              	callq	0x140901260 <.text+0x900260>
1407d353f: 89 85 48 02 00 00           	movl	%eax, 0x248(%rbp)
1407d3545: 89 95 4c 02 00 00           	movl	%edx, 0x24c(%rbp)
1407d354b: 89 f9                       	movl	%edi, %ecx
1407d354d: 44 89 ea                    	movl	%r13d, %edx
1407d3550: e8 0b dd 12 00              	callq	0x140901260 <.text+0x900260>
1407d3555: 49 83 c7 04                 	addq	$0x4, %r15
1407d3559: 89 85 40 02 00 00           	movl	%eax, 0x240(%rbp)
1407d355f: 89 95 44 02 00 00           	movl	%edx, 0x244(%rbp)
1407d3565: 45 31 ed                    	xorl	%r13d, %r13d
1407d3568: bb 00 00 00 00              	movl	$0x0, %ebx
1407d356d: 39 bd 48 02 00 00           	cmpl	%edi, 0x248(%rbp)
1407d3573: 0f 85 f7 00 00 00           	jne	0x1407d3670 <.text+0x7d2670>
1407d3579: 39 f8                       	cmpl	%edi, %eax
1407d357b: 0f 84 62 01 00 00           	je	0x1407d36e3 <.text+0x7d26e3>
1407d3581: 48 83 be 98 01 00 00 00     	cmpq	$0x0, 0x198(%rsi)
1407d3589: b8 00 00 00 00              	movl	$0x0, %eax
1407d358e: 74 90                       	je	0x1407d3520 <.text+0x7d2520>
1407d3590: 48 8b 8d c8 02 00 00        	movq	0x2c8(%rbp), %rcx
1407d3597: 48 8d 95 40 02 00 00        	leaq	0x240(%rbp), %rdx
1407d359e: e8 3d e6 dd ff              	callq	0x1405b1be0 <.text+0x5b0be0>
1407d35a3: 48 8b 8e 80 01 00 00        	movq	0x180(%rsi), %rcx
1407d35aa: 48 8b 96 88 01 00 00        	movq	0x188(%rsi), %rdx
1407d35b1: 49 89 d0                    	movq	%rdx, %r8
1407d35b4: 49 21 c0                    	andq	%rax, %r8
1407d35b7: 48 c1 e8 39                 	shrq	$0x39, %rax
1407d35bb: 66 0f 6e c0                 	movd	%eax, %xmm0
1407d35bf: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407d35c3: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407d35c8: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407d35cd: 48 8d 41 f4                 	leaq	-0xc(%rcx), %rax
1407d35d1: f3 0f 7e 8d 40 02 00 00     	movq	0x240(%rbp), %xmm1
1407d35d9: 45 31 c9                    	xorl	%r9d, %r9d
1407d35dc: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
1407d35e2: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
1407d35e6: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
1407d35ea: 66 44 0f d7 d3              	pmovmskb	%xmm3, %r10d
1407d35ef: 45 85 d2                    	testl	%r10d, %r10d
1407d35f2: 74 4c                       	je	0x1407d3640 <.text+0x7d2640>
1407d35f4: f3 45 0f bc da              	tzcntl	%r10d, %r11d
1407d35f9: 4d 01 c3                    	addq	%r8, %r11
1407d35fc: 49 21 d3                    	andq	%rdx, %r11
1407d35ff: 49 f7 db                    	negq	%r11
1407d3602: 4f 8d 1c 5b                 	leaq	(%r11,%r11,2), %r11
1407d3606: f3 42 0f 7e 1c 98           	movq	(%rax,%r11,4), %xmm3
1407d360c: 66 0f 76 d9                 	pcmpeqd	%xmm1, %xmm3
1407d3610: 66 0f 70 db 50              	pshufd	$0x50, %xmm3, %xmm3     # xmm3 = xmm3[0,0,1,1]
1407d3615: 66 44 0f 50 eb              	movmskpd	%xmm3, %r13d
1407d361a: 41 83 fd 03                 	cmpl	$0x3, %r13d
1407d361e: 0f 84 b0 00 00 00           	je	0x1407d36d4 <.text+0x7d26d4>
1407d3624: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
1407d3628: 66 45 21 d3                 	andw	%r10w, %r11w
1407d362c: 45 89 da                    	movl	%r11d, %r10d
1407d362f: 75 c3                       	jne	0x1407d35f4 <.text+0x7d25f4>
1407d3631: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1407d3640: 66 0f 74 d6                 	pcmpeqb	%xmm6, %xmm2
1407d3644: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
1407d3649: 45 85 d2                    	testl	%r10d, %r10d
1407d364c: 0f 85 e0 00 00 00           	jne	0x1407d3732 <.text+0x7d2732>
1407d3652: 4d 01 c8                    	addq	%r9, %r8
1407d3655: 49 83 c0 10                 	addq	$0x10, %r8
1407d3659: 49 83 c1 10                 	addq	$0x10, %r9
1407d365d: 49 21 d0                    	andq	%rdx, %r8
1407d3660: e9 77 ff ff ff              	jmp	0x1407d35dc <.text+0x7d25dc>
1407d3665: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1407d3670: 48 83 be 98 01 00 00 00     	cmpq	$0x0, 0x198(%rsi)
1407d3678: b8 00 00 00 00              	movl	$0x0, %eax
1407d367d: 0f 84 9d fe ff ff           	je	0x1407d3520 <.text+0x7d2520>
1407d3683: 48 8b 8d c8 02 00 00        	movq	0x2c8(%rbp), %rcx
1407d368a: 48 8d 95 48 02 00 00        	leaq	0x248(%rbp), %rdx
1407d3691: e8 4a e5 dd ff              	callq	0x1405b1be0 <.text+0x5b0be0>
1407d3696: 48 8b 8e 80 01 00 00        	movq	0x180(%rsi), %rcx
1407d369d: 48 8b 96 88 01 00 00        	movq	0x188(%rsi), %rdx
1407d36a4: 49 89 d0                    	movq	%rdx, %r8
1407d36a7: 49 21 c0                    	andq	%rax, %r8
1407d36aa: 48 c1 e8 39                 	shrq	$0x39, %rax
1407d36ae: 66 0f 6e c0                 	movd	%eax, %xmm0
1407d36b2: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407d36b6: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407d36bb: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407d36c0: 48 8d 41 f4                 	leaq	-0xc(%rcx), %rax
1407d36c4: f3 0f 7e 8d 48 02 00 00     	movq	0x248(%rbp), %xmm1
1407d36cc: 45 31 c9                    	xorl	%r9d, %r9d
1407d36cf: e9 bc fd ff ff              	jmp	0x1407d3490 <.text+0x7d2490>
1407d36d4: 46 8b 6c 99 fc              	movl	-0x4(%rcx,%r11,4), %r13d
1407d36d9: 41 83 ed 01                 	subl	$0x1, %r13d
1407d36dd: 0f 82 91 02 00 00           	jb	0x1407d3974 <.text+0x7d2974>
1407d36e3: 8b 85 b8 02 00 00           	movl	0x2b8(%rbp), %eax
1407d36e9: 48 8b 8d 50 02 00 00        	movq	0x250(%rbp), %rcx
1407d36f0: 89 01                       	movl	%eax, (%rcx)
1407d36f2: 48 8b 85 48 02 00 00        	movq	0x248(%rbp), %rax
1407d36f9: 48 89 85 90 02 00 00        	movq	%rax, 0x290(%rbp)
1407d3700: 89 9d 98 02 00 00           	movl	%ebx, 0x298(%rbp)
1407d3706: 48 8b 85 40 02 00 00        	movq	0x240(%rbp), %rax
1407d370d: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
1407d3711: 44 89 6d 48                 	movl	%r13d, 0x48(%rbp)
1407d3715: 48 89 f1                    	movq	%rsi, %rcx
1407d3718: 48 8d 95 90 02 00 00        	leaq	0x290(%rbp), %rdx
1407d371f: 4c 8d 45 40                 	leaq	0x40(%rbp), %r8
1407d3723: 45 31 c9                    	xorl	%r9d, %r9d
1407d3726: e8 c5 68 09 00              	callq	0x140869ff0 <.text+0x868ff0>
1407d372b: 31 c0                       	xorl	%eax, %eax
1407d372d: e9 ee fd ff ff              	jmp	0x1407d3520 <.text+0x7d2520>
1407d3732: 31 c0                       	xorl	%eax, %eax
1407d3734: e9 e7 fd ff ff              	jmp	0x1407d3520 <.text+0x7d2520>
1407d3739: 4c 89 f1                    	movq	%r14, %rcx
1407d373c: 4c 29 f9                    	subq	%r15, %rcx
1407d373f: 48 c1 e9 02                 	shrq	$0x2, %rcx
1407d3743: 4d 8d 3c 87                 	leaq	(%r15,%rax,4), %r15
1407d3747: 48 39 c1                    	cmpq	%rax, %rcx
1407d374a: 0f 87 e2 fd ff ff           	ja	0x1407d3532 <.text+0x7d2532>
1407d3750: e9 fb fc ff ff              	jmp	0x1407d3450 <.text+0x7d2450>
1407d3755: 48 8b 95 80 01 00 00        	movq	0x180(%rbp), %rdx
1407d375c: 48 85 d2                    	testq	%rdx, %rdx
1407d375f: 74 12                       	je	0x1407d3773 <.text+0x7d2773>
1407d3761: 48 c1 e2 02                 	shlq	$0x2, %rdx
1407d3765: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1407d376b: 4c 89 f9                    	movq	%r15, %rcx
1407d376e: e8 ad 04 e4 ff              	callq	0x140613c20 <.text+0x612c20>
1407d3773: 80 bd 80 02 00 00 00        	cmpb	$0x0, 0x280(%rbp)
1407d377a: 74 64                       	je	0x1407d37e0 <.text+0x7d27e0>
1407d377c: e8 af 49 44 00              	callq	0x140c18130 <.text+0xc17130>
1407d3781: 48 89 c1                    	movq	%rax, %rcx
1407d3784: 48 8b 00                    	movq	(%rax), %rax
1407d3787: 48 85 c0                    	testq	%rax, %rax
1407d378a: 74 0f                       	je	0x1407d379b <.text+0x7d279b>
1407d378c: 83 f8 01                    	cmpl	$0x1, %eax
1407d378f: 0f 85 fd 04 00 00           	jne	0x1407d3c92 <.text+0x7d2c92>
1407d3795: 48 83 c1 08                 	addq	$0x8, %rcx
1407d3799: eb 13                       	jmp	0x1407d37ae <.text+0x7d27ae>
1407d379b: 31 d2                       	xorl	%edx, %edx
1407d379d: e8 de ed 11 02              	callq	0x1428f2580 <.text+0x28f1580>
1407d37a2: 48 89 c1                    	movq	%rax, %rcx
1407d37a5: 48 85 c0                    	testq	%rax, %rax
1407d37a8: 0f 84 e4 04 00 00           	je	0x1407d3c92 <.text+0x7d2c92>
1407d37ae: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1407d37b2: 0f 85 1f 02 00 00           	jne	0x1407d39d7 <.text+0x7d29d7>
1407d37b8: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1407d37bf: 48 89 8d c8 02 00 00        	movq	%rcx, 0x2c8(%rbp)
1407d37c6: 48 83 c1 08                 	addq	$0x8, %rcx
1407d37ca: 48 8b 95 88 02 00 00        	movq	0x288(%rbp), %rdx
1407d37d1: e8 1a 39 44 00              	callq	0x140c170f0 <.text+0xc160f0>
1407d37d6: 48 8b 85 c8 02 00 00        	movq	0x2c8(%rbp), %rax
1407d37dd: 48 ff 00                    	incq	(%rax)
1407d37e0: 80 bd d5 02 00 00 00        	cmpb	$0x0, 0x2d5(%rbp)
1407d37e7: 74 4b                       	je	0x1407d3834 <.text+0x7d2834>
1407d37e9: e8 42 49 44 00              	callq	0x140c18130 <.text+0xc17130>
1407d37ee: 48 89 c1                    	movq	%rax, %rcx
1407d37f1: 48 8b 00                    	movq	(%rax), %rax
1407d37f4: 48 83 f8 01                 	cmpq	$0x1, %rax
1407d37f8: 0f 85 aa 00 00 00           	jne	0x1407d38a8 <.text+0x7d28a8>
1407d37fe: 48 83 c1 08                 	addq	$0x8, %rcx
1407d3802: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1407d3806: 0f 85 79 01 00 00           	jne	0x1407d3985 <.text+0x7d2985>
1407d380c: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1407d3813: 48 89 8d c8 02 00 00        	movq	%rcx, 0x2c8(%rbp)
1407d381a: 48 83 c1 08                 	addq	$0x8, %rcx
1407d381e: 48 8b 95 20 01 00 00        	movq	0x120(%rbp), %rdx
1407d3825: e8 c6 38 44 00              	callq	0x140c170f0 <.text+0xc160f0>
1407d382a: 48 8b 85 c8 02 00 00        	movq	0x2c8(%rbp), %rax
1407d3831: 48 ff 00                    	incq	(%rax)
1407d3834: 0f 28 b5 e0 02 00 00        	movaps	0x2e0(%rbp), %xmm6
1407d383b: 0f 28 bd f0 02 00 00        	movaps	0x2f0(%rbp), %xmm7
1407d3842: 44 0f 28 85 00 03 00 00     	movaps	0x300(%rbp), %xmm8
1407d384a: 44 0f 28 8d 10 03 00 00     	movaps	0x310(%rbp), %xmm9
1407d3852: 44 0f 28 95 20 03 00 00     	movaps	0x320(%rbp), %xmm10
1407d385a: 44 0f 28 9d 30 03 00 00     	movaps	0x330(%rbp), %xmm11
1407d3862: 44 0f 28 a5 40 03 00 00     	movaps	0x340(%rbp), %xmm12
1407d386a: 44 0f 28 ad 50 03 00 00     	movaps	0x350(%rbp), %xmm13
1407d3872: 44 0f 28 b5 60 03 00 00     	movaps	0x360(%rbp), %xmm14
1407d387a: 48 81 c4 f8 03 00 00        	addq	$0x3f8, %rsp            # imm = 0x3F8
1407d3881: 5b                          	popq	%rbx
1407d3882: 5f                          	popq	%rdi
1407d3883: 5e                          	popq	%rsi
1407d3884: 41 5c                       	popq	%r12
1407d3886: 41 5d                       	popq	%r13
1407d3888: 41 5e                       	popq	%r14
1407d388a: 41 5f                       	popq	%r15
1407d388c: 5d                          	popq	%rbp
1407d388d: c3                          	retq
1407d388e: 48 85 c0                    	testq	%rax, %rax
1407d3891: 75 2d                       	jne	0x1407d38c0 <.text+0x7d28c0>
1407d3893: 31 d2                       	xorl	%edx, %edx
1407d3895: e8 e6 ec 11 02              	callq	0x1428f2580 <.text+0x28f1580>
1407d389a: 48 89 c1                    	movq	%rax, %rcx
1407d389d: 48 85 c0                    	testq	%rax, %rax
1407d38a0: 0f 85 86 ea ff ff           	jne	0x1407d232c <.text+0x7d132c>
1407d38a6: eb 18                       	jmp	0x1407d38c0 <.text+0x7d28c0>
1407d38a8: 48 85 c0                    	testq	%rax, %rax
1407d38ab: 75 13                       	jne	0x1407d38c0 <.text+0x7d28c0>
1407d38ad: 31 d2                       	xorl	%edx, %edx
1407d38af: e8 cc ec 11 02              	callq	0x1428f2580 <.text+0x28f1580>
1407d38b4: 48 89 c1                    	movq	%rax, %rcx
1407d38b7: 48 85 c0                    	testq	%rax, %rax
1407d38ba: 0f 85 42 ff ff ff           	jne	0x1407d3802 <.text+0x7d2802>
1407d38c0: 48 8d 0d 01 db 44 02        	leaq	0x244db01(%rip), %rcx   # 0x142c213c8
1407d38c7: e8 04 48 10 02              	callq	0x1428d80d0 <.text+0x28d70d0>
1407d38cc: 48 8d 0d cd 47 2b 02        	leaq	0x22b47cd(%rip), %rcx   # 0x142a880a0
1407d38d3: e8 c8 5f 10 02              	callq	0x1428d98a0 <.text+0x28d88a0>
1407d38d8: e9 c1 03 00 00              	jmp	0x1407d3c9e <.text+0x7d2c9e>
1407d38dd: 4c 8d 05 bc 47 2b 02        	leaq	0x22b47bc(%rip), %r8    # 0x142a880a0
1407d38e4: 48 89 ca                    	movq	%rcx, %rdx
1407d38e7: 4c 89 d1                    	movq	%r10, %rcx
1407d38ea: e8 61 5f 10 02              	callq	0x1428d9850 <.text+0x28d8850>
1407d38ef: e9 aa 03 00 00              	jmp	0x1407d3c9e <.text+0x7d2c9e>
1407d38f4: 4c 8d 05 a5 47 2b 02        	leaq	0x22b47a5(%rip), %r8    # 0x142a880a0
1407d38fb: 48 8b 95 78 01 00 00        	movq	0x178(%rbp), %rdx
1407d3902: e8 39 5f 10 02              	callq	0x1428d9840 <.text+0x28d8840>
1407d3907: e9 92 03 00 00              	jmp	0x1407d3c9e <.text+0x7d2c9e>
1407d390c: 4c 8d 05 a5 47 2b 02        	leaq	0x22b47a5(%rip), %r8    # 0x142a880b8
1407d3913: e8 28 5f 10 02              	callq	0x1428d9840 <.text+0x28d8840>
1407d3918: e9 81 03 00 00              	jmp	0x1407d3c9e <.text+0x7d2c9e>
1407d391d: 4c 8d 05 ac 47 2b 02        	leaq	0x22b47ac(%rip), %r8    # 0x142a880d0
1407d3924: e8 17 5f 10 02              	callq	0x1428d9840 <.text+0x28d8840>
1407d3929: e9 70 03 00 00              	jmp	0x1407d3c9e <.text+0x7d2c9e>
1407d392e: 48 c7 85 80 01 00 00 00 00 00 00    	movq	$0x0, 0x180(%rbp)
1407d3939: 48 8d 05 b0 47 2b 02        	leaq	0x22b47b0(%rip), %rax   # 0x142a880f0
1407d3940: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407d3945: 4c 8d 05 9c 47 2b 02        	leaq	0x22b479c(%rip), %r8    # 0x142a880e8
1407d394c: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
1407d3950: 4c 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %r9
1407d3957: 31 c9                       	xorl	%ecx, %ecx
1407d3959: e8 38 5c 10 02              	callq	0x1428d9596 <.text+0x28d8596>
1407d395e: e9 3b 03 00 00              	jmp	0x1407d3c9e <.text+0x7d2c9e>
1407d3963: 48 8d 0d ee 46 2b 02        	leaq	0x22b46ee(%rip), %rcx   # 0x142a88058
1407d396a: e8 a1 59 10 02              	callq	0x1428d9310 <.text+0x28d8310>
1407d396f: e9 2a 03 00 00              	jmp	0x1407d3c9e <.text+0x7d2c9e>
1407d3974: 48 8d 0d f5 46 2b 02        	leaq	0x22b46f5(%rip), %rcx   # 0x142a88070
1407d397b: e8 90 59 10 02              	callq	0x1428d9310 <.text+0x28d8310>
1407d3980: e9 19 03 00 00              	jmp	0x1407d3c9e <.text+0x7d2c9e>
1407d3985: 48 8d 0d ac da 44 02        	leaq	0x244daac(%rip), %rcx   # 0x142c21438
1407d398c: e8 bf 58 10 02              	callq	0x1428d9250 <.text+0x28d8250>
1407d3991: 48 8d 0d a0 da 44 02        	leaq	0x244daa0(%rip), %rcx   # 0x142c21438
1407d3998: e8 b3 58 10 02              	callq	0x1428d9250 <.text+0x28d8250>
1407d399d: e9 fc 02 00 00              	jmp	0x1407d3c9e <.text+0x7d2c9e>
1407d39a2: 48 c7 85 80 01 00 00 00 00 00 00    	movq	$0x0, 0x180(%rbp)
1407d39ad: 48 8d 05 d4 46 2b 02        	leaq	0x22b46d4(%rip), %rax   # 0x142a88088
1407d39b4: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407d39b9: 48 8d 95 90 02 00 00        	leaq	0x290(%rbp), %rdx
1407d39c0: 4c 8d 45 40                 	leaq	0x40(%rbp), %r8
1407d39c4: 4c 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %r9
1407d39cb: 31 c9                       	xorl	%ecx, %ecx
1407d39cd: e8 c4 5b 10 02              	callq	0x1428d9596 <.text+0x28d8596>
1407d39d2: e9 c7 02 00 00              	jmp	0x1407d3c9e <.text+0x7d2c9e>
1407d39d7: 48 8d 0d 5a da 44 02        	leaq	0x244da5a(%rip), %rcx   # 0x142c21438
1407d39de: e8 6d 58 10 02              	callq	0x1428d9250 <.text+0x28d8250>
1407d39e3: e9 b6 02 00 00              	jmp	0x1407d3c9e <.text+0x7d2c9e>
1407d39e8: 48 8d 85 90 02 00 00        	leaq	0x290(%rbp), %rax
1407d39ef: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
1407d39f3: 48 8d 05 06 8c ac 00        	leaq	0xac8c06(%rip), %rax    # 0x14129c600 <.text+0x129b600>
1407d39fa: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
1407d39fe: 48 8d 85 30 01 00 00        	leaq	0x130(%rbp), %rax
1407d3a05: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
1407d3a09: 48 8d 05 b0 9c 5b 00        	leaq	0x5b9cb0(%rip), %rax    # 0x140d8d6c0 <.text+0xd8c6c0>
1407d3a10: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
1407d3a14: 48 8d 05 fd 3d 2b 02        	leaq	0x22b3dfd(%rip), %rax   # 0x142a87818
1407d3a1b: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
1407d3a22: 48 c7 85 88 01 00 00 02 00 00 00    	movq	$0x2, 0x188(%rbp)
1407d3a2d: 48 c7 85 a0 01 00 00 00 00 00 00    	movq	$0x0, 0x1a0(%rbp)
1407d3a38: 48 8d 45 40                 	leaq	0x40(%rbp), %rax
1407d3a3c: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
1407d3a43: 48 c7 85 98 01 00 00 02 00 00 00    	movq	$0x2, 0x198(%rbp)
1407d3a4e: 48 8d 15 43 3e 2b 02        	leaq	0x22b3e43(%rip), %rdx   # 0x142a87898
1407d3a55: 48 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %rcx
1407d3a5c: e8 2f 59 10 02              	callq	0x1428d9390 <.text+0x28d8390>
1407d3a61: e9 38 02 00 00              	jmp	0x1407d3c9e <.text+0x7d2c9e>
1407d3a66: b9 10 00 00 00              	movl	$0x10, %ecx
1407d3a6b: ba 90 00 00 00              	movl	$0x90, %edx
1407d3a70: e8 8b 55 10 02              	callq	0x1428d9000 <.text+0x28d8000>
1407d3a75: e9 24 02 00 00              	jmp	0x1407d3c9e <.text+0x7d2c9e>
1407d3a7a: 4c 8d 05 87 46 2b 02        	leaq	0x22b4687(%rip), %r8    # 0x142a88108
1407d3a81: 4c 89 e1                    	movq	%r12, %rcx
1407d3a84: 48 89 fa                    	movq	%rdi, %rdx
1407d3a87: e8 8c 5a 10 02              	callq	0x1428d9518 <.text+0x28d8518>
1407d3a8c: e9 0d 02 00 00              	jmp	0x1407d3c9e <.text+0x7d2c9e>
1407d3a91: b9 10 00 00 00              	movl	$0x10, %ecx
1407d3a96: ba 90 00 00 00              	movl	$0x90, %edx
1407d3a9b: e8 60 55 10 02              	callq	0x1428d9000 <.text+0x28d8000>
1407d3aa0: e9 f9 01 00 00              	jmp	0x1407d3c9e <.text+0x7d2c9e>
1407d3aa5: 48 8b 85 28 01 00 00        	movq	0x128(%rbp), %rax
1407d3aac: 48 89 85 30 01 00 00        	movq	%rax, 0x130(%rbp)
1407d3ab3: 48 8d 85 20 02 00 00        	leaq	0x220(%rbp), %rax
1407d3aba: 48 89 85 90 02 00 00        	movq	%rax, 0x290(%rbp)
1407d3ac1: 48 8d 05 f8 9b 5b 00        	leaq	0x5b9bf8(%rip), %rax    # 0x140d8d6c0 <.text+0xd8c6c0>
1407d3ac8: 48 89 85 98 02 00 00        	movq	%rax, 0x298(%rbp)
1407d3acf: 48 8d 8d 30 01 00 00        	leaq	0x130(%rbp), %rcx
1407d3ad6: 48 89 8d a0 02 00 00        	movq	%rcx, 0x2a0(%rbp)
1407d3add: 48 89 85 a8 02 00 00        	movq	%rax, 0x2a8(%rbp)
1407d3ae4: 48 8d 05 bd 42 2b 02        	leaq	0x22b42bd(%rip), %rax   # 0x142a87da8
1407d3aeb: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
1407d3af2: 48 c7 85 88 01 00 00 02 00 00 00    	movq	$0x2, 0x188(%rbp)
1407d3afd: 48 c7 85 a0 01 00 00 00 00 00 00    	movq	$0x0, 0x1a0(%rbp)
1407d3b08: 48 8d 85 90 02 00 00        	leaq	0x290(%rbp), %rax
1407d3b0f: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
1407d3b16: 48 c7 85 98 01 00 00 02 00 00 00    	movq	$0x2, 0x198(%rbp)
1407d3b21: c6 85 d7 02 00 00 01        	movb	$0x1, 0x2d7(%rbp)
1407d3b28: 48 8d 15 99 42 2b 02        	leaq	0x22b4299(%rip), %rdx   # 0x142a87dc8
1407d3b2f: 48 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %rcx
1407d3b36: e8 55 58 10 02              	callq	0x1428d9390 <.text+0x28d8390>
1407d3b3b: e9 5e 01 00 00              	jmp	0x1407d3c9e <.text+0x7d2c9e>
1407d3b40: 48 c7 85 30 01 00 00 01 00 00 00    	movq	$0x1, 0x130(%rbp)
1407d3b4b: 48 8d 85 20 02 00 00        	leaq	0x220(%rbp), %rax
1407d3b52: 48 89 85 90 02 00 00        	movq	%rax, 0x290(%rbp)
1407d3b59: 48 8d 05 60 9b 5b 00        	leaq	0x5b9b60(%rip), %rax    # 0x140d8d6c0 <.text+0xd8c6c0>
1407d3b60: 48 89 85 98 02 00 00        	movq	%rax, 0x298(%rbp)
1407d3b67: 48 8d 8d 30 01 00 00        	leaq	0x130(%rbp), %rcx
1407d3b6e: 48 89 8d a0 02 00 00        	movq	%rcx, 0x2a0(%rbp)
1407d3b75: 48 89 85 a8 02 00 00        	movq	%rax, 0x2a8(%rbp)
1407d3b7c: 48 8d 05 25 42 2b 02        	leaq	0x22b4225(%rip), %rax   # 0x142a87da8
1407d3b83: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
1407d3b8a: 48 c7 85 88 01 00 00 02 00 00 00    	movq	$0x2, 0x188(%rbp)
1407d3b95: 48 c7 85 a0 01 00 00 00 00 00 00    	movq	$0x0, 0x1a0(%rbp)
1407d3ba0: 48 8d 85 90 02 00 00        	leaq	0x290(%rbp), %rax
1407d3ba7: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
1407d3bae: 48 c7 85 98 01 00 00 02 00 00 00    	movq	$0x2, 0x198(%rbp)
1407d3bb9: c6 85 d6 02 00 00 01        	movb	$0x1, 0x2d6(%rbp)
1407d3bc0: 48 8d 15 01 42 2b 02        	leaq	0x22b4201(%rip), %rdx   # 0x142a87dc8
1407d3bc7: 48 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %rcx
1407d3bce: e8 bd 57 10 02              	callq	0x1428d9390 <.text+0x28d8390>
1407d3bd3: e9 c6 00 00 00              	jmp	0x1407d3c9e <.text+0x7d2c9e>
1407d3bd8: b9 10 00 00 00              	movl	$0x10, %ecx
1407d3bdd: ba 90 00 00 00              	movl	$0x90, %edx
1407d3be2: e8 19 54 10 02              	callq	0x1428d9000 <.text+0x28d8000>
1407d3be7: e9 b2 00 00 00              	jmp	0x1407d3c9e <.text+0x7d2c9e>
1407d3bec: 48 8b 85 28 01 00 00        	movq	0x128(%rbp), %rax
1407d3bf3: 48 89 85 30 01 00 00        	movq	%rax, 0x130(%rbp)
1407d3bfa: 48 8d 85 20 02 00 00        	leaq	0x220(%rbp), %rax
1407d3c01: 48 89 85 90 02 00 00        	movq	%rax, 0x290(%rbp)
1407d3c08: 48 8d 05 b1 9a 5b 00        	leaq	0x5b9ab1(%rip), %rax    # 0x140d8d6c0 <.text+0xd8c6c0>
1407d3c0f: 48 89 85 98 02 00 00        	movq	%rax, 0x298(%rbp)
1407d3c16: 48 8d 8d 30 01 00 00        	leaq	0x130(%rbp), %rcx
1407d3c1d: 48 89 8d a0 02 00 00        	movq	%rcx, 0x2a0(%rbp)
1407d3c24: 48 89 85 a8 02 00 00        	movq	%rax, 0x2a8(%rbp)
1407d3c2b: 48 8d 05 76 41 2b 02        	leaq	0x22b4176(%rip), %rax   # 0x142a87da8
1407d3c32: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
1407d3c39: 48 c7 85 88 01 00 00 02 00 00 00    	movq	$0x2, 0x188(%rbp)
1407d3c44: 48 c7 85 a0 01 00 00 00 00 00 00    	movq	$0x0, 0x1a0(%rbp)
1407d3c4f: 48 8d 85 90 02 00 00        	leaq	0x290(%rbp), %rax
1407d3c56: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
1407d3c5d: 48 c7 85 98 01 00 00 02 00 00 00    	movq	$0x2, 0x198(%rbp)
1407d3c68: c6 85 d7 02 00 00 01        	movb	$0x1, 0x2d7(%rbp)
1407d3c6f: 48 8d 15 52 41 2b 02        	leaq	0x22b4152(%rip), %rdx   # 0x142a87dc8
1407d3c76: 48 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %rcx
1407d3c7d: e8 0e 57 10 02              	callq	0x1428d9390 <.text+0x28d8390>
1407d3c82: eb 1a                       	jmp	0x1407d3c9e <.text+0x7d2c9e>
1407d3c84: 48 8d 0d 3d d7 44 02        	leaq	0x244d73d(%rip), %rcx   # 0x142c213c8
1407d3c8b: e8 40 44 10 02              	callq	0x1428d80d0 <.text+0x28d70d0>
1407d3c90: eb 0c                       	jmp	0x1407d3c9e <.text+0x7d2c9e>
1407d3c92: 48 8d 0d 2f d7 44 02        	leaq	0x244d72f(%rip), %rcx   # 0x142c213c8
1407d3c99: e8 32 44 10 02              	callq	0x1428d80d0 <.text+0x28d70d0>
1407d3c9e: 0f 0b                       	ud2
1407d3ca0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407d3ca5: 55                          	pushq	%rbp
1407d3ca6: 41 57                       	pushq	%r15
1407d3ca8: 41 56                       	pushq	%r14
1407d3caa: 41 55                       	pushq	%r13
1407d3cac: 41 54                       	pushq	%r12
1407d3cae: 56                          	pushq	%rsi
1407d3caf: 57                          	pushq	%rdi
1407d3cb0: 53                          	pushq	%rbx
1407d3cb1: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1407d3cb8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407d3cbf: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1407d3cc5: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1407d3ccb: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1407d3cd1: 66 44 0f 7f 5c 24 60        	movdqa	%xmm11, 0x60(%rsp)
1407d3cd8: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1407d3cde: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1407d3ce7: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
1407d3cf0: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
1407d3cf8: 66 0f 7f b4 24 b0 00 00 00  	movdqa	%xmm6, 0xb0(%rsp)
1407d3d01: 48 8b 85 c8 02 00 00        	movq	0x2c8(%rbp), %rax
1407d3d08: 48 ff 00                    	incq	(%rax)
1407d3d0b: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
1407d3d13: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1407d3d1b: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
1407d3d24: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1407d3d2d: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1407d3d33: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
1407d3d39: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1407d3d3f: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1407d3d45: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1407d3d4b: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1407d3d52: 5b                          	popq	%rbx
1407d3d53: 5f                          	popq	%rdi
1407d3d54: 5e                          	popq	%rsi
1407d3d55: 41 5c                       	popq	%r12
1407d3d57: 41 5d                       	popq	%r13
1407d3d59: 41 5e                       	popq	%r14
1407d3d5b: 41 5f                       	popq	%r15
1407d3d5d: 5d                          	popq	%rbp
1407d3d5e: c3                          	retq
1407d3d5f: 90                          	nop
1407d3d60: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407d3d65: 55                          	pushq	%rbp
1407d3d66: 41 57                       	pushq	%r15
1407d3d68: 41 56                       	pushq	%r14
1407d3d6a: 41 55                       	pushq	%r13
1407d3d6c: 41 54                       	pushq	%r12
1407d3d6e: 56                          	pushq	%rsi
1407d3d6f: 57                          	pushq	%rdi
1407d3d70: 53                          	pushq	%rbx
1407d3d71: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1407d3d78: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407d3d7f: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1407d3d85: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1407d3d8b: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1407d3d91: 66 44 0f 7f 5c 24 60        	movdqa	%xmm11, 0x60(%rsp)
1407d3d98: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1407d3d9e: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1407d3da7: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
1407d3db0: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
1407d3db8: 66 0f 7f b4 24 b0 00 00 00  	movdqa	%xmm6, 0xb0(%rsp)
1407d3dc1: 48 8b 85 c8 02 00 00        	movq	0x2c8(%rbp), %rax
1407d3dc8: 48 ff 00                    	incq	(%rax)
1407d3dcb: 66 0f 28 b4 24 b0 00 00 00  	movapd	0xb0(%rsp), %xmm6
1407d3dd4: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1407d3ddc: 66 44 0f 28 84 24 90 00 00 00       	movapd	0x90(%rsp), %xmm8
1407d3de6: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1407d3def: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1407d3df5: 66 44 0f 6f 5c 24 60        	movdqa	0x60(%rsp), %xmm11
1407d3dfc: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1407d3e02: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1407d3e08: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1407d3e0e: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1407d3e15: 5b                          	popq	%rbx
1407d3e16: 5f                          	popq	%rdi
1407d3e17: 5e                          	popq	%rsi
1407d3e18: 41 5c                       	popq	%r12
1407d3e1a: 41 5d                       	popq	%r13
1407d3e1c: 41 5e                       	popq	%r14
1407d3e1e: 41 5f                       	popq	%r15
1407d3e20: 5d                          	popq	%rbp
1407d3e21: c3                          	retq
1407d3e22: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1407d3e30: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407d3e35: 55                          	pushq	%rbp
1407d3e36: 41 57                       	pushq	%r15
1407d3e38: 41 56                       	pushq	%r14
1407d3e3a: 41 55                       	pushq	%r13
1407d3e3c: 41 54                       	pushq	%r12
1407d3e3e: 56                          	pushq	%rsi
1407d3e3f: 57                          	pushq	%rdi
1407d3e40: 53                          	pushq	%rbx
1407d3e41: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1407d3e48: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407d3e4f: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1407d3e55: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1407d3e5b: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1407d3e61: 66 44 0f 7f 5c 24 60        	movdqa	%xmm11, 0x60(%rsp)
1407d3e68: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1407d3e6e: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1407d3e77: 66 44 0f 29 84 24 90 00 00 00       	movapd	%xmm8, 0x90(%rsp)
1407d3e81: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
1407d3e89: 66 0f 29 b4 24 b0 00 00 00  	movapd	%xmm6, 0xb0(%rsp)
1407d3e92: 48 8b 85 c8 02 00 00        	movq	0x2c8(%rbp), %rax
1407d3e99: 48 ff 00                    	incq	(%rax)
1407d3e9c: 66 0f 28 b4 24 b0 00 00 00  	movapd	0xb0(%rsp), %xmm6
1407d3ea5: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1407d3ead: 66 44 0f 28 84 24 90 00 00 00       	movapd	0x90(%rsp), %xmm8
1407d3eb7: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1407d3ec0: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1407d3ec6: 66 44 0f 6f 5c 24 60        	movdqa	0x60(%rsp), %xmm11
1407d3ecd: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1407d3ed3: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1407d3ed9: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1407d3edf: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1407d3ee6: 5b                          	popq	%rbx
1407d3ee7: 5f                          	popq	%rdi
1407d3ee8: 5e                          	popq	%rsi
1407d3ee9: 41 5c                       	popq	%r12
1407d3eeb: 41 5d                       	popq	%r13
1407d3eed: 41 5e                       	popq	%r14
1407d3eef: 41 5f                       	popq	%r15
1407d3ef1: 5d                          	popq	%rbp
1407d3ef2: c3                          	retq
1407d3ef3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1407d3f00: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407d3f05: 55                          	pushq	%rbp
1407d3f06: 41 57                       	pushq	%r15
1407d3f08: 41 56                       	pushq	%r14
1407d3f0a: 41 55                       	pushq	%r13
1407d3f0c: 41 54                       	pushq	%r12
1407d3f0e: 56                          	pushq	%rsi
1407d3f0f: 57                          	pushq	%rdi
1407d3f10: 53                          	pushq	%rbx
1407d3f11: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1407d3f18: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407d3f1f: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1407d3f25: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1407d3f2b: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1407d3f31: 66 44 0f 7f 5c 24 60        	movdqa	%xmm11, 0x60(%rsp)
1407d3f38: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1407d3f3e: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1407d3f47: 66 44 0f 29 84 24 90 00 00 00       	movapd	%xmm8, 0x90(%rsp)
1407d3f51: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
1407d3f59: 66 0f 29 b4 24 b0 00 00 00  	movapd	%xmm6, 0xb0(%rsp)
1407d3f62: 48 8b 85 b8 02 00 00        	movq	0x2b8(%rbp), %rax
1407d3f69: 48 ff 00                    	incq	(%rax)
1407d3f6c: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
1407d3f74: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1407d3f7c: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
1407d3f85: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1407d3f8e: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1407d3f94: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
1407d3f9a: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1407d3fa0: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1407d3fa6: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1407d3fac: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1407d3fb3: 5b                          	popq	%rbx
1407d3fb4: 5f                          	popq	%rdi
1407d3fb5: 5e                          	popq	%rsi
1407d3fb6: 41 5c                       	popq	%r12
1407d3fb8: 41 5d                       	popq	%r13
1407d3fba: 41 5e                       	popq	%r14
1407d3fbc: 41 5f                       	popq	%r15
1407d3fbe: 5d                          	popq	%rbp
1407d3fbf: c3                          	retq
1407d3fc0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407d3fc5: 55                          	pushq	%rbp
1407d3fc6: 41 57                       	pushq	%r15
1407d3fc8: 41 56                       	pushq	%r14
1407d3fca: 41 55                       	pushq	%r13
1407d3fcc: 41 54                       	pushq	%r12
1407d3fce: 56                          	pushq	%rsi
1407d3fcf: 57                          	pushq	%rdi
1407d3fd0: 53                          	pushq	%rbx
1407d3fd1: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1407d3fd8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407d3fdf: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1407d3fe5: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1407d3feb: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1407d3ff1: 66 44 0f 7f 5c 24 60        	movdqa	%xmm11, 0x60(%rsp)
1407d3ff8: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1407d3ffe: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1407d4007: 66 44 0f 29 84 24 90 00 00 00       	movapd	%xmm8, 0x90(%rsp)
1407d4011: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
1407d4019: 66 0f 29 b4 24 b0 00 00 00  	movapd	%xmm6, 0xb0(%rsp)
1407d4022: c6 85 d7 02 00 00 00        	movb	$0x0, 0x2d7(%rbp)
1407d4029: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1407d4030: e8 ab db e2 ff              	callq	0x140601be0 <.text+0x600be0>
1407d4035: c6 85 d7 02 00 00 00        	movb	$0x0, 0x2d7(%rbp)
1407d403c: 66 0f 28 b4 24 b0 00 00 00  	movapd	0xb0(%rsp), %xmm6
1407d4045: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1407d404d: 66 44 0f 28 84 24 90 00 00 00       	movapd	0x90(%rsp), %xmm8
1407d4057: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1407d4060: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1407d4066: 66 44 0f 6f 5c 24 60        	movdqa	0x60(%rsp), %xmm11
1407d406d: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1407d4073: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1407d4079: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1407d407f: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1407d4086: 5b                          	popq	%rbx
1407d4087: 5f                          	popq	%rdi
1407d4088: 5e                          	popq	%rsi
1407d4089: 41 5c                       	popq	%r12
1407d408b: 41 5d                       	popq	%r13
1407d408d: 41 5e                       	popq	%r14
1407d408f: 41 5f                       	popq	%r15
1407d4091: 5d                          	popq	%rbp
1407d4092: c3                          	retq
1407d4093: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1407d40a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407d40a5: 55                          	pushq	%rbp
1407d40a6: 41 57                       	pushq	%r15
1407d40a8: 41 56                       	pushq	%r14
1407d40aa: 41 55                       	pushq	%r13
1407d40ac: 41 54                       	pushq	%r12
1407d40ae: 56                          	pushq	%rsi
1407d40af: 57                          	pushq	%rdi
1407d40b0: 53                          	pushq	%rbx
1407d40b1: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1407d40b8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407d40bf: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1407d40c5: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1407d40cb: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1407d40d1: 66 44 0f 7f 5c 24 60        	movdqa	%xmm11, 0x60(%rsp)
1407d40d8: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1407d40de: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1407d40e7: 66 44 0f 29 84 24 90 00 00 00       	movapd	%xmm8, 0x90(%rsp)
1407d40f1: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
1407d40f9: 66 0f 29 b4 24 b0 00 00 00  	movapd	%xmm6, 0xb0(%rsp)
1407d4102: c6 85 d7 02 00 00 00        	movb	$0x0, 0x2d7(%rbp)
1407d4109: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1407d4110: e8 cb da e2 ff              	callq	0x140601be0 <.text+0x600be0>
1407d4115: c6 85 d7 02 00 00 00        	movb	$0x0, 0x2d7(%rbp)
1407d411c: 66 0f 28 b4 24 b0 00 00 00  	movapd	0xb0(%rsp), %xmm6
1407d4125: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1407d412d: 66 44 0f 28 84 24 90 00 00 00       	movapd	0x90(%rsp), %xmm8
1407d4137: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1407d4140: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1407d4146: 66 44 0f 6f 5c 24 60        	movdqa	0x60(%rsp), %xmm11
1407d414d: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1407d4153: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1407d4159: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1407d415f: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1407d4166: 5b                          	popq	%rbx
1407d4167: 5f                          	popq	%rdi
1407d4168: 5e                          	popq	%rsi
1407d4169: 41 5c                       	popq	%r12
1407d416b: 41 5d                       	popq	%r13
1407d416d: 41 5e                       	popq	%r14
1407d416f: 41 5f                       	popq	%r15
1407d4171: 5d                          	popq	%rbp
1407d4172: c3                          	retq
1407d4173: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1407d4180: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407d4185: 55                          	pushq	%rbp
1407d4186: 41 57                       	pushq	%r15
1407d4188: 41 56                       	pushq	%r14
1407d418a: 41 55                       	pushq	%r13
1407d418c: 41 54                       	pushq	%r12
1407d418e: 56                          	pushq	%rsi
1407d418f: 57                          	pushq	%rdi
1407d4190: 53                          	pushq	%rbx
1407d4191: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1407d4198: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407d419f: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1407d41a5: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1407d41ab: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1407d41b1: 66 44 0f 7f 5c 24 60        	movdqa	%xmm11, 0x60(%rsp)
1407d41b8: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1407d41be: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1407d41c7: 66 44 0f 29 84 24 90 00 00 00       	movapd	%xmm8, 0x90(%rsp)
1407d41d1: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
1407d41d9: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
1407d41e1: c6 85 d6 02 00 00 00        	movb	$0x0, 0x2d6(%rbp)
1407d41e8: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1407d41ef: e8 ec d9 e2 ff              	callq	0x140601be0 <.text+0x600be0>
1407d41f4: c6 85 d6 02 00 00 00        	movb	$0x0, 0x2d6(%rbp)
1407d41fb: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
1407d4203: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1407d420b: 66 44 0f 28 84 24 90 00 00 00       	movapd	0x90(%rsp), %xmm8
1407d4215: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1407d421e: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1407d4224: 66 44 0f 6f 5c 24 60        	movdqa	0x60(%rsp), %xmm11
1407d422b: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1407d4231: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1407d4237: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1407d423d: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1407d4244: 5b                          	popq	%rbx
1407d4245: 5f                          	popq	%rdi
1407d4246: 5e                          	popq	%rsi
1407d4247: 41 5c                       	popq	%r12
1407d4249: 41 5d                       	popq	%r13
1407d424b: 41 5e                       	popq	%r14
1407d424d: 41 5f                       	popq	%r15
1407d424f: 5d                          	popq	%rbp
1407d4250: c3                          	retq
1407d4251: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1407d4260: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407d4265: 55                          	pushq	%rbp
1407d4266: 41 57                       	pushq	%r15
1407d4268: 41 56                       	pushq	%r14
1407d426a: 41 55                       	pushq	%r13
1407d426c: 41 54                       	pushq	%r12
1407d426e: 56                          	pushq	%rsi
1407d426f: 57                          	pushq	%rdi
1407d4270: 53                          	pushq	%rbx
1407d4271: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1407d4278: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407d427f: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1407d4285: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1407d428b: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1407d4291: 66 44 0f 7f 5c 24 60        	movdqa	%xmm11, 0x60(%rsp)
1407d4298: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1407d429e: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1407d42a7: 66 44 0f 29 84 24 90 00 00 00       	movapd	%xmm8, 0x90(%rsp)
1407d42b1: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
1407d42b9: 66 0f 29 b4 24 b0 00 00 00  	movapd	%xmm6, 0xb0(%rsp)
1407d42c2: 80 bd d3 02 00 00 00        	cmpb	$0x0, 0x2d3(%rbp)
1407d42c9: 74 77                       	je	0x1407d4342 <.text+0x7d3342>
1407d42cb: 48 8b 85 a8 02 00 00        	movq	0x2a8(%rbp), %rax
1407d42d2: f0                          	lock
1407d42d3: 48 ff 08                    	decq	(%rax)
1407d42d6: 75 0c                       	jne	0x1407d42e4 <.text+0x7d32e4>
1407d42d8: 48 8d 8d a8 02 00 00        	leaq	0x2a8(%rbp), %rcx
1407d42df: e8 3c 83 38 00              	callq	0x140b5c620 <.text+0xb5b620>
1407d42e4: c6 85 d7 02 00 00 00        	movb	$0x0, 0x2d7(%rbp)
1407d42eb: 66 0f 28 b4 24 b0 00 00 00  	movapd	0xb0(%rsp), %xmm6
1407d42f4: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1407d42fc: 66 44 0f 28 84 24 90 00 00 00       	movapd	0x90(%rsp), %xmm8
1407d4306: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1407d430f: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1407d4315: 66 44 0f 6f 5c 24 60        	movdqa	0x60(%rsp), %xmm11
1407d431c: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1407d4322: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1407d4328: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1407d432e: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1407d4335: 5b                          	popq	%rbx
1407d4336: 5f                          	popq	%rdi
1407d4337: 5e                          	popq	%rsi
1407d4338: 41 5c                       	popq	%r12
1407d433a: 41 5d                       	popq	%r13
1407d433c: 41 5e                       	popq	%r14
1407d433e: 41 5f                       	popq	%r15
1407d4340: 5d                          	popq	%rbp
1407d4341: c3                          	retq
1407d4342: c6 85 d7 02 00 00 00        	movb	$0x0, 0x2d7(%rbp)
1407d4349: 66 0f 28 b4 24 b0 00 00 00  	movapd	0xb0(%rsp), %xmm6
1407d4352: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1407d435a: 66 44 0f 28 84 24 90 00 00 00       	movapd	0x90(%rsp), %xmm8
1407d4364: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1407d436d: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1407d4373: 66 44 0f 6f 5c 24 60        	movdqa	0x60(%rsp), %xmm11
1407d437a: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1407d4380: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1407d4386: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1407d438c: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1407d4393: 5b                          	popq	%rbx
1407d4394: 5f                          	popq	%rdi
1407d4395: 5e                          	popq	%rsi
1407d4396: 41 5c                       	popq	%r12
1407d4398: 41 5d                       	popq	%r13
1407d439a: 41 5e                       	popq	%r14
1407d439c: 41 5f                       	popq	%r15
1407d439e: 5d                          	popq	%rbp
1407d439f: c3                          	retq
1407d43a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407d43a5: 55                          	pushq	%rbp
1407d43a6: 41 57                       	pushq	%r15
1407d43a8: 41 56                       	pushq	%r14
1407d43aa: 41 55                       	pushq	%r13
1407d43ac: 41 54                       	pushq	%r12
1407d43ae: 56                          	pushq	%rsi
1407d43af: 57                          	pushq	%rdi
1407d43b0: 53                          	pushq	%rbx
1407d43b1: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1407d43b8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407d43bf: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1407d43c5: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1407d43cb: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1407d43d1: 66 44 0f 7f 5c 24 60        	movdqa	%xmm11, 0x60(%rsp)
1407d43d8: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1407d43de: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1407d43e7: 66 44 0f 29 84 24 90 00 00 00       	movapd	%xmm8, 0x90(%rsp)
1407d43f1: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
1407d43f9: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
1407d4401: 80 bd d4 02 00 00 00        	cmpb	$0x0, 0x2d4(%rbp)
1407d4408: 74 76                       	je	0x1407d4480 <.text+0x7d3480>
1407d440a: 48 8b 85 a8 02 00 00        	movq	0x2a8(%rbp), %rax
1407d4411: f0                          	lock
1407d4412: 48 ff 08                    	decq	(%rax)
1407d4415: 75 0c                       	jne	0x1407d4423 <.text+0x7d3423>
1407d4417: 48 8d 8d a8 02 00 00        	leaq	0x2a8(%rbp), %rcx
1407d441e: e8 fd 81 38 00              	callq	0x140b5c620 <.text+0xb5b620>
1407d4423: c6 85 d6 02 00 00 00        	movb	$0x0, 0x2d6(%rbp)
1407d442a: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
1407d4432: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1407d443a: 66 44 0f 28 84 24 90 00 00 00       	movapd	0x90(%rsp), %xmm8
1407d4444: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1407d444d: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1407d4453: 66 44 0f 6f 5c 24 60        	movdqa	0x60(%rsp), %xmm11
1407d445a: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1407d4460: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1407d4466: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1407d446c: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1407d4473: 5b                          	popq	%rbx
1407d4474: 5f                          	popq	%rdi
1407d4475: 5e                          	popq	%rsi
1407d4476: 41 5c                       	popq	%r12
1407d4478: 41 5d                       	popq	%r13
1407d447a: 41 5e                       	popq	%r14
1407d447c: 41 5f                       	popq	%r15
1407d447e: 5d                          	popq	%rbp
1407d447f: c3                          	retq
1407d4480: c6 85 d6 02 00 00 00        	movb	$0x0, 0x2d6(%rbp)
1407d4487: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
1407d448f: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1407d4497: 66 44 0f 28 84 24 90 00 00 00       	movapd	0x90(%rsp), %xmm8
1407d44a1: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1407d44aa: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1407d44b0: 66 44 0f 6f 5c 24 60        	movdqa	0x60(%rsp), %xmm11
1407d44b7: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1407d44bd: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1407d44c3: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1407d44c9: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1407d44d0: 5b                          	popq	%rbx
1407d44d1: 5f                          	popq	%rdi
1407d44d2: 5e                          	popq	%rsi
1407d44d3: 41 5c                       	popq	%r12
1407d44d5: 41 5d                       	popq	%r13
1407d44d7: 41 5e                       	popq	%r14
1407d44d9: 41 5f                       	popq	%r15
1407d44db: 5d                          	popq	%rbp
1407d44dc: c3                          	retq
1407d44dd: 0f 1f 00                    	nopl	(%rax)
1407d44e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407d44e5: 55                          	pushq	%rbp
1407d44e6: 41 57                       	pushq	%r15
1407d44e8: 41 56                       	pushq	%r14
1407d44ea: 41 55                       	pushq	%r13
1407d44ec: 41 54                       	pushq	%r12
1407d44ee: 56                          	pushq	%rsi
1407d44ef: 57                          	pushq	%rdi
1407d44f0: 53                          	pushq	%rbx
1407d44f1: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1407d44f8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407d44ff: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1407d4505: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1407d450b: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1407d4511: 66 44 0f 7f 5c 24 60        	movdqa	%xmm11, 0x60(%rsp)
1407d4518: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1407d451e: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1407d4527: 66 44 0f 29 84 24 90 00 00 00       	movapd	%xmm8, 0x90(%rsp)
1407d4531: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
1407d4539: 66 0f 29 b4 24 b0 00 00 00  	movapd	%xmm6, 0xb0(%rsp)
1407d4542: 80 bd d7 02 00 00 00        	cmpb	$0x0, 0x2d7(%rbp)
1407d4549: 74 5d                       	je	0x1407d45a8 <.text+0x7d35a8>
1407d454b: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
1407d454f: e8 8c d6 e2 ff              	callq	0x140601be0 <.text+0x600be0>
1407d4554: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
1407d455c: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1407d4564: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
1407d456d: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1407d4576: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1407d457c: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
1407d4582: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1407d4588: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1407d458e: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1407d4594: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1407d459b: 5b                          	popq	%rbx
1407d459c: 5f                          	popq	%rdi
1407d459d: 5e                          	popq	%rsi
1407d459e: 41 5c                       	popq	%r12
1407d45a0: 41 5d                       	popq	%r13
1407d45a2: 41 5e                       	popq	%r14
1407d45a4: 41 5f                       	popq	%r15
1407d45a6: 5d                          	popq	%rbp
1407d45a7: c3                          	retq
1407d45a8: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
1407d45b0: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1407d45b8: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
1407d45c1: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1407d45ca: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1407d45d0: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
1407d45d6: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1407d45dc: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1407d45e2: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1407d45e8: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1407d45ef: 5b                          	popq	%rbx
1407d45f0: 5f                          	popq	%rdi
1407d45f1: 5e                          	popq	%rsi
1407d45f2: 41 5c                       	popq	%r12
1407d45f4: 41 5d                       	popq	%r13
1407d45f6: 41 5e                       	popq	%r14
1407d45f8: 41 5f                       	popq	%r15
1407d45fa: 5d                          	popq	%rbp
1407d45fb: c3                          	retq
1407d45fc: 0f 1f 40 00                 	nopl	(%rax)
1407d4600: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407d4605: 55                          	pushq	%rbp
1407d4606: 41 57                       	pushq	%r15
1407d4608: 41 56                       	pushq	%r14
1407d460a: 41 55                       	pushq	%r13
1407d460c: 41 54                       	pushq	%r12
1407d460e: 56                          	pushq	%rsi
1407d460f: 57                          	pushq	%rdi
1407d4610: 53                          	pushq	%rbx
1407d4611: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1407d4618: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407d461f: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1407d4625: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1407d462b: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1407d4631: 66 44 0f 7f 5c 24 60        	movdqa	%xmm11, 0x60(%rsp)
1407d4638: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1407d463e: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1407d4647: 66 44 0f 29 84 24 90 00 00 00       	movapd	%xmm8, 0x90(%rsp)
1407d4651: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
1407d4659: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
1407d4661: 80 bd d6 02 00 00 00        	cmpb	$0x0, 0x2d6(%rbp)
1407d4668: 74 5d                       	je	0x1407d46c7 <.text+0x7d36c7>
1407d466a: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
1407d466e: e8 6d d5 e2 ff              	callq	0x140601be0 <.text+0x600be0>
1407d4673: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
1407d467b: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1407d4683: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
1407d468c: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1407d4695: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1407d469b: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
1407d46a1: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1407d46a7: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1407d46ad: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1407d46b3: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1407d46ba: 5b                          	popq	%rbx
1407d46bb: 5f                          	popq	%rdi
1407d46bc: 5e                          	popq	%rsi
1407d46bd: 41 5c                       	popq	%r12
1407d46bf: 41 5d                       	popq	%r13
1407d46c1: 41 5e                       	popq	%r14
1407d46c3: 41 5f                       	popq	%r15
1407d46c5: 5d                          	popq	%rbp
1407d46c6: c3                          	retq
1407d46c7: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
1407d46cf: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1407d46d7: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
1407d46e0: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1407d46e9: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1407d46ef: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
1407d46f5: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1407d46fb: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1407d4701: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1407d4707: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1407d470e: 5b                          	popq	%rbx
1407d470f: 5f                          	popq	%rdi
1407d4710: 5e                          	popq	%rsi
1407d4711: 41 5c                       	popq	%r12
1407d4713: 41 5d                       	popq	%r13
1407d4715: 41 5e                       	popq	%r14
1407d4717: 41 5f                       	popq	%r15
1407d4719: 5d                          	popq	%rbp
1407d471a: c3                          	retq
1407d471b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1407d4720: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407d4725: 55                          	pushq	%rbp
1407d4726: 41 57                       	pushq	%r15
1407d4728: 41 56                       	pushq	%r14
1407d472a: 41 55                       	pushq	%r13
1407d472c: 41 54                       	pushq	%r12
1407d472e: 56                          	pushq	%rsi
1407d472f: 57                          	pushq	%rdi
1407d4730: 53                          	pushq	%rbx
1407d4731: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1407d4738: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407d473f: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1407d4745: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1407d474b: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1407d4751: 66 44 0f 7f 5c 24 60        	movdqa	%xmm11, 0x60(%rsp)
1407d4758: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1407d475e: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1407d4767: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
1407d4770: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
1407d4778: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
1407d4780: 48 8b 95 80 01 00 00        	movq	0x180(%rbp), %rdx
1407d4787: 48 85 d2                    	testq	%rdx, %rdx
1407d478a: 74 16                       	je	0x1407d47a2 <.text+0x7d37a2>
1407d478c: 48 8b 8d 88 01 00 00        	movq	0x188(%rbp), %rcx
1407d4793: 48 c1 e2 02                 	shlq	$0x2, %rdx
1407d4797: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1407d479d: e8 7e f4 e3 ff              	callq	0x140613c20 <.text+0x612c20>
1407d47a2: 80 bd 80 02 00 00 00        	cmpb	$0x0, 0x280(%rbp)
1407d47a9: 74 54                       	je	0x1407d47ff <.text+0x7d37ff>
1407d47ab: e8 80 39 44 00              	callq	0x140c18130 <.text+0xc17130>
1407d47b0: 48 8b 08                    	movq	(%rax), %rcx
1407d47b3: 48 85 c9                    	testq	%rcx, %rcx
1407d47b6: 75 1b                       	jne	0x1407d47d3 <.text+0x7d37d3>
1407d47b8: 48 89 c1                    	movq	%rax, %rcx
1407d47bb: 31 d2                       	xorl	%edx, %edx
1407d47bd: e8 be dd 11 02              	callq	0x1428f2580 <.text+0x28f1580>
1407d47c2: 48 85 c0                    	testq	%rax, %rax
1407d47c5: 75 15                       	jne	0x1407d47dc <.text+0x7d37dc>
1407d47c7: 48 8d 0d fa cb 44 02        	leaq	0x244cbfa(%rip), %rcx   # 0x142c213c8
1407d47ce: e8 fd 38 10 02              	callq	0x1428d80d0 <.text+0x28d70d0>
1407d47d3: 83 f9 02                    	cmpl	$0x2, %ecx
1407d47d6: 74 ef                       	je	0x1407d47c7 <.text+0x7d37c7>
1407d47d8: 48 83 c0 08                 	addq	$0x8, %rax
1407d47dc: 48 83 38 00                 	cmpq	$0x0, (%rax)
1407d47e0: 75 71                       	jne	0x1407d4853 <.text+0x7d3853>
1407d47e2: 48 c7 00 ff ff ff ff        	movq	$-0x1, (%rax)
1407d47e9: 48 8d 48 08                 	leaq	0x8(%rax), %rcx
1407d47ed: 48 8b 95 88 02 00 00        	movq	0x288(%rbp), %rdx
1407d47f4: 48 89 c6                    	movq	%rax, %rsi
1407d47f7: e8 f4 28 44 00              	callq	0x140c170f0 <.text+0xc160f0>
1407d47fc: 48 ff 06                    	incq	(%rsi)
1407d47ff: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
1407d4807: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1407d480f: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
1407d4818: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1407d4821: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1407d4827: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
1407d482d: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1407d4833: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1407d4839: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1407d483f: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1407d4846: 5b                          	popq	%rbx
1407d4847: 5f                          	popq	%rdi
1407d4848: 5e                          	popq	%rsi
1407d4849: 41 5c                       	popq	%r12
1407d484b: 41 5d                       	popq	%r13
1407d484d: 41 5e                       	popq	%r14
1407d484f: 41 5f                       	popq	%r15
1407d4851: 5d                          	popq	%rbp
1407d4852: c3                          	retq
1407d4853: 48 8d 0d de cb 44 02        	leaq	0x244cbde(%rip), %rcx   # 0x142c21438
1407d485a: e8 f1 49 10 02              	callq	0x1428d9250 <.text+0x28d8250>
1407d485f: cc                          	int3
1407d4860: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407d4865: 55                          	pushq	%rbp
1407d4866: 41 57                       	pushq	%r15
1407d4868: 41 56                       	pushq	%r14
1407d486a: 41 55                       	pushq	%r13
1407d486c: 41 54                       	pushq	%r12
1407d486e: 56                          	pushq	%rsi
1407d486f: 57                          	pushq	%rdi
1407d4870: 53                          	pushq	%rbx
1407d4871: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1407d4878: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407d487f: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1407d4885: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1407d488b: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1407d4891: 66 44 0f 7f 5c 24 60        	movdqa	%xmm11, 0x60(%rsp)
1407d4898: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1407d489e: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1407d48a7: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
1407d48b0: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
1407d48b8: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
1407d48c0: 80 bd d5 02 00 00 00        	cmpb	$0x0, 0x2d5(%rbp)
1407d48c7: 74 39                       	je	0x1407d4902 <.text+0x7d3902>
1407d48c9: e8 62 38 44 00              	callq	0x140c18130 <.text+0xc17130>
1407d48ce: 48 89 c6                    	movq	%rax, %rsi
1407d48d1: 48 8b 00                    	movq	(%rax), %rax
1407d48d4: 48 83 f8 01                 	cmpq	$0x1, %rax
1407d48d8: 75 7c                       	jne	0x1407d4956 <.text+0x7d3956>
1407d48da: 48 83 c6 08                 	addq	$0x8, %rsi
1407d48de: 48 83 3e 00                 	cmpq	$0x0, (%rsi)
1407d48e2: 0f 85 95 00 00 00           	jne	0x1407d497d <.text+0x7d397d>
1407d48e8: 48 c7 06 ff ff ff ff        	movq	$-0x1, (%rsi)
1407d48ef: 48 8d 4e 08                 	leaq	0x8(%rsi), %rcx
1407d48f3: 48 8b 95 20 01 00 00        	movq	0x120(%rbp), %rdx
1407d48fa: e8 f1 27 44 00              	callq	0x140c170f0 <.text+0xc160f0>
1407d48ff: 48 ff 06                    	incq	(%rsi)
1407d4902: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
1407d490a: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1407d4912: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
1407d491b: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1407d4924: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1407d492a: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
1407d4930: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1407d4936: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1407d493c: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1407d4942: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1407d4949: 5b                          	popq	%rbx
1407d494a: 5f                          	popq	%rdi
1407d494b: 5e                          	popq	%rsi
1407d494c: 41 5c                       	popq	%r12
1407d494e: 41 5d                       	popq	%r13
1407d4950: 41 5e                       	popq	%r14
1407d4952: 41 5f                       	popq	%r15
1407d4954: 5d                          	popq	%rbp
1407d4955: c3                          	retq
1407d4956: 48 85 c0                    	testq	%rax, %rax
1407d4959: 75 16                       	jne	0x1407d4971 <.text+0x7d3971>
1407d495b: 48 89 f1                    	movq	%rsi, %rcx
1407d495e: 31 d2                       	xorl	%edx, %edx
1407d4960: e8 1b dc 11 02              	callq	0x1428f2580 <.text+0x28f1580>
1407d4965: 48 89 c6                    	movq	%rax, %rsi
1407d4968: 48 85 c0                    	testq	%rax, %rax
1407d496b: 0f 85 6d ff ff ff           	jne	0x1407d48de <.text+0x7d38de>
1407d4971: 48 8d 0d 50 ca 44 02        	leaq	0x244ca50(%rip), %rcx   # 0x142c213c8
1407d4978: e8 53 37 10 02              	callq	0x1428d80d0 <.text+0x28d70d0>
1407d497d: 48 8d 0d b4 ca 44 02        	leaq	0x244cab4(%rip), %rcx   # 0x142c21438
1407d4984: e8 c7 48 10 02              	callq	0x1428d9250 <.text+0x28d8250>
1407d4989: cc                          	int3
