140596690: 66 0f 29 83 80 05 00 00     	movapd	%xmm0, 0x580(%rbx)
140596698: 48 8d 8b c0 3a 00 00        	leaq	0x3ac0(%rbx), %rcx
14059669f: 48 8d 93 80 02 00 00        	leaq	0x280(%rbx), %rdx
1405966a6: 4d 89 f8                    	movq	%r15, %r8
1405966a9: 4c 8d 8b 4e 01 00 00        	leaq	0x14e(%rbx), %r9
1405966b0: e8 8b 52 64 00              	callq	0x140bdb940 <.text+0xbda940>
1405966b5: 31 c0                       	xorl	%eax, %eax
1405966b7: 48 3b 83 c0 3a 00 00        	cmpq	0x3ac0(%rbx), %rax
1405966be: 71 70                       	jno	0x140596730 <.text+0x595730>
1405966c0: 48 8d 8b c8 3a 00 00        	leaq	0x3ac8(%rbx), %rcx
1405966c7: 0f 10 41 40                 	movups	0x40(%rcx), %xmm0
1405966cb: 48 8d 83 10 1a 00 00        	leaq	0x1a10(%rbx), %rax
1405966d2: 0f 11 40 18                 	movups	%xmm0, 0x18(%rax)
1405966d6: 0f 10 01                    	movups	(%rcx), %xmm0
1405966d9: f3 0f 6f 49 10              	movdqu	0x10(%rcx), %xmm1
1405966de: f3 0f 6f 51 20              	movdqu	0x20(%rcx), %xmm2
1405966e3: f3 0f 6f 59 30              	movdqu	0x30(%rcx), %xmm3
1405966e8: f3 0f 7f 58 08              	movdqu	%xmm3, 0x8(%rax)
1405966ed: f3 0f 7f 50 f8              	movdqu	%xmm2, -0x8(%rax)
1405966f2: f3 0f 7f 48 e8              	movdqu	%xmm1, -0x18(%rax)
1405966f7: 0f 11 40 d8                 	movups	%xmm0, -0x28(%rax)
1405966fb: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140596705: 48 89 83 e0 19 00 00        	movq	%rax, 0x19e0(%rbx)
14059670c: 48 8b 05 95 5c 09 03        	movq	0x3095c95(%rip), %rax   # 0x14362c3a8
140596713: 48 8b 00                    	movq	(%rax), %rax
140596716: 48 83 f8 04                 	cmpq	$0x4, %rax
14059671a: 0f 83 54 0f 00 00           	jae	0x140597674 <.text+0x596674>
140596720: e9 06 10 00 00              	jmp	0x14059772b <.text+0x59672b>
140596725: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140596730: 41 b8 e0 0a 00 00           	movl	$0xae0, %r8d            # imm = 0xAE0
140596736: 48 8d 8b e0 2f 00 00        	leaq	0x2fe0(%rbx), %rcx
14059673d: 48 8d 93 c0 3a 00 00        	leaq	0x3ac0(%rbx), %rdx
140596744: e8 b7 d4 38 02              	callq	0x142923c00 <.text+0x2922c00>
1405968d7: 0f 2e c7                    	ucomiss	%xmm7, %xmm0
1405968da: 73 34                       	jae	0x140596910 <.text+0x595910>
1405968dc: 48 c7 83 e8 19 00 00 15 00 00 00    	movq	$0x15, 0x19e8(%rbx)
1405968e7: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1405968f1: 48 89 83 e0 19 00 00        	movq	%rax, 0x19e0(%rbx)
1405968f8: 48 8d 8b e0 2f 00 00        	leaq	0x2fe0(%rbx), %rcx
1405968ff: e8 cc be 05 00              	callq	0x1405f27d0 <.text+0x5f17d0>
140596904: 4c 8d bb 80 05 00 00        	leaq	0x580(%rbx), %r15
14059690b: e9 50 0d 00 00              	jmp	0x140597660 <.text+0x596660>
140597660: 48 8b 05 41 4d 09 03        	movq	0x3094d41(%rip), %rax   # 0x14362c3a8
140597667: 48 8b 00                    	movq	(%rax), %rax
14059766a: 48 83 f8 04                 	cmpq	$0x4, %rax
14059766e: 0f 82 b7 00 00 00           	jb	0x14059772b <.text+0x59672b>
140597674: c6 83 ba 00 00 00 01        	movb	$0x1, 0xba(%rbx)
14059767b: 48 8d 0d 3e 89 46 02        	leaq	0x246893e(%rip), %rcx   # 0x1429fffc0
140597682: e8 f9 66 0e 00              	callq	0x14067dd80 <.text+0x67cd80>
140597687: 0f 10 00                    	movups	(%rax), %xmm0
14059768a: 8b 40 10                    	movl	0x10(%rax), %eax
14059768d: 48 c7 83 b0 05 00 00 04 00 00 00    	movq	$0x4, 0x5b0(%rbx)
140597698: 48 8d 0d 51 7d 46 02        	leaq	0x2467d51(%rip), %rcx   # 0x1429ff3f0
14059769f: 48 89 8b b8 05 00 00        	movq	%rcx, 0x5b8(%rbx)
1405976a6: 48 c7 83 c0 05 00 00 14 00 00 00    	movq	$0x14, 0x5c0(%rbx)
1405976b1: 48 8d 15 f8 88 46 02        	leaq	0x24688f8(%rip), %rdx   # 0x1429fffb0
1405976b8: 48 89 93 d0 05 00 00        	movq	%rdx, 0x5d0(%rbx)
1405976bf: 48 c7 83 d8 05 00 00 01 00 00 00    	movq	$0x1, 0x5d8(%rbx)
1405976ca: 48 c7 83 e0 05 00 00 08 00 00 00    	movq	$0x8, 0x5e0(%rbx)
1405976d5: 48 c7 83 80 05 00 00 00 00 00 00    	movq	$0x0, 0x580(%rbx)
1405976e0: 0f 11 76 40                 	movups	%xmm6, 0x40(%rsi)
1405976e4: 48 89 8b 88 05 00 00        	movq	%rcx, 0x588(%rbx)
1405976eb: 48 c7 83 90 05 00 00 14 00 00 00    	movq	$0x14, 0x590(%rbx)
1405976f6: 48 c7 83 98 05 00 00 00 00 00 00    	movq	$0x0, 0x598(%rbx)
140597701: 0f 11 83 a0 05 00 00        	movups	%xmm0, 0x5a0(%rbx)
140597708: c7 83 c8 05 00 00 01 00 00 00       	movl	$0x1, 0x5c8(%rbx)
140597712: 89 83 cc 05 00 00           	movl	%eax, 0x5cc(%rbx)
140597718: c6 83 ba 00 00 00 01        	movb	$0x1, 0xba(%rbx)
14059771f: 48 8d 4b 4c                 	leaq	0x4c(%rbx), %rcx
140597723: 4c 89 fa                    	movq	%r15, %rdx
140597726: e8 05 2c 7b 00              	callq	0x140d4a330 <.text+0xd49330>
14059772b: 4c 8b bb e0 19 00 00        	movq	0x19e0(%rbx), %r15
140597732: 4c 89 f8                    	movq	%r15, %rax
140597735: 48 f7 d8                    	negq	%rax
140597738: 0f 81 6c 08 00 00           	jno	0x140597faa <.text+0x596faa>
14059773e: 48 8d 83 10 1a 00 00        	leaq	0x1a10(%rbx), %rax
140597745: 0f 10 40 18                 	movups	0x18(%rax), %xmm0
140597749: 0f 29 83 20 30 00 00        	movaps	%xmm0, 0x3020(%rbx)
140597750: 0f 10 40 d8                 	movups	-0x28(%rax), %xmm0
140597754: f3 0f 6f 48 e8              	movdqu	-0x18(%rax), %xmm1
140597759: f3 0f 6f 50 f8              	movdqu	-0x8(%rax), %xmm2
14059775e: f3 0f 6f 58 08              	movdqu	0x8(%rax), %xmm3
140597763: 66 0f 7f 9b 10 30 00 00     	movdqa	%xmm3, 0x3010(%rbx)
14059776b: 66 0f 7f 93 00 30 00 00     	movdqa	%xmm2, 0x3000(%rbx)
140597773: 66 0f 7f 8b f0 2f 00 00     	movdqa	%xmm1, 0x2ff0(%rbx)
14059777b: 0f 29 83 e0 2f 00 00        	movaps	%xmm0, 0x2fe0(%rbx)
140597782: 48 8b 05 1f 4c 09 03        	movq	0x3094c1f(%rip), %rax   # 0x14362c3a8
140597789: 48 8b 00                    	movq	(%rax), %rax
14059778c: 48 85 c0                    	testq	%rax, %rax
14059778f: 4c 8d bb 80 05 00 00        	leaq	0x580(%rbx), %r15
140597796: 0f 84 82 01 00 00           	je	0x14059791e <.text+0x59691e>
14059791e: 4c 8b b3 70 01 00 00        	movq	0x170(%rbx), %r14
140597925: 41 c6 86 98 00 00 00 01     	movb	$0x1, 0x98(%r14)
14059792d: 49 8d 56 10                 	leaq	0x10(%r14), %rdx
140597931: c6 83 9a 00 00 00 01        	movb	$0x1, 0x9a(%rbx)
140597938: 48 8d 8b c0 04 00 00        	leaq	0x4c0(%rbx), %rcx
14059793f: e8 4c 44 7e 00              	callq	0x140d7bd90 <.text+0xd7ad90>
140597944: 45 0f 10 06                 	movups	(%r14), %xmm8
140597948: 41 0f b6 be 98 00 00 00     	movzbl	0x98(%r14), %edi
140597950: 31 c0                       	xorl	%eax, %eax
140597952: 49 3b 46 28                 	cmpq	0x28(%r14), %rax
140597feb: 48 8b 83 70 01 00 00        	movq	0x170(%rbx), %rax
140597ff2: c6 80 98 00 00 00 02        	movb	$0x2, 0x98(%rax)
140597ff9: 0f 10 40 78                 	movups	0x78(%rax), %xmm0
140597ffd: 0f 29 83 00 25 00 00        	movaps	%xmm0, 0x2500(%rbx)
140598004: 48 8b 83 d0 00 00 00        	movq	0xd0(%rbx), %rax
14059800b: c6 80 ac 00 00 00 01        	movb	$0x1, 0xac(%rax)
140598012: 66 0f 6f 83 00 25 00 00     	movdqa	0x2500(%rbx), %xmm0
14059801a: f3 0f 7f 80 ad 00 00 00     	movdqu	%xmm0, 0xad(%rax)
