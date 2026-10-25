
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
142316760: 55                          	pushq	%rbp
142316761: 56                          	pushq	%rsi
142316762: 57                          	pushq	%rdi
142316763: 48 81 ec 00 01 00 00        	subq	$0x100, %rsp            # imm = 0x100
14231676a: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
142316772: 48 c7 45 78 fe ff ff ff     	movq	$-0x2, 0x78(%rbp)
14231677a: 48 89 d6                    	movq	%rdx, %rsi
14231677d: 48 8b 01                    	movq	(%rcx), %rax
142316780: 48 8b 49 08                 	movq	0x8(%rcx), %rcx
142316784: 48 8d 91 a0 01 00 00        	leaq	0x1a0(%rcx), %rdx
14231678b: 4c 8d 81 f8 00 00 00        	leaq	0xf8(%rcx), %r8
142316792: 4c 8b 88 f8 00 00 00        	movq	0xf8(%rax), %r9
142316799: 4c 8b 90 00 01 00 00        	movq	0x100(%rax), %r10
1423167a0: 4f 8d 14 91                 	leaq	(%r9,%r10,4), %r10
1423167a4: 44 0f b6 98 08 01 00 00     	movzbl	0x108(%rax), %r11d
1423167ac: 44 88 5d 48                 	movb	%r11b, 0x48(%rbp)
1423167b0: 4c 89 4d f8                 	movq	%r9, -0x8(%rbp)
1423167b4: 4c 89 55 00                 	movq	%r10, (%rbp)
1423167b8: 48 c7 45 08 08 00 00 00     	movq	$0x8, 0x8(%rbp)
1423167c0: 48 c7 45 10 00 00 00 00     	movq	$0x0, 0x10(%rbp)
1423167c8: 48 c7 45 18 08 00 00 00     	movq	$0x8, 0x18(%rbp)
1423167d0: 0f 57 c0                    	xorps	%xmm0, %xmm0
1423167d3: 0f 11 45 20                 	movups	%xmm0, 0x20(%rbp)
1423167d7: 0f 11 45 30                 	movups	%xmm0, 0x30(%rbp)
1423167db: 48 c7 45 40 00 00 00 00     	movq	$0x0, 0x40(%rbp)
1423167e3: 48 89 4d d8                 	movq	%rcx, -0x28(%rbp)
1423167e7: 48 89 55 e0                 	movq	%rdx, -0x20(%rbp)
1423167eb: 4c 89 45 e8                 	movq	%r8, -0x18(%rbp)
1423167ef: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
1423167f3: 4c 8d 05 9e 45 d3 00        	leaq	0xd3459e(%rip), %r8     # 0x14304ad98
1423167fa: 48 8d 4d 60                 	leaq	0x60(%rbp), %rcx
1423167fe: 48 8d 55 d8                 	leaq	-0x28(%rbp), %rdx
142316802: e8 59 85 aa ff              	callq	0x141dbed60 <.text+0x1dbdd60>
142316807: 48 8b 45 70                 	movq	0x70(%rbp), %rax
14231680b: 48 85 c0                    	testq	%rax, %rax
14231680e: 74 7c                       	je	0x14231688c <.text+0x231588c>
142316810: 48 83 f8 60                 	cmpq	$0x60, %rax
142316814: 0f 85 9a 00 00 00           	jne	0x1423168b4 <.text+0x23158b4>
14231681a: 48 8b 0e                    	movq	(%rsi), %rcx
14231681d: 48 8d 15 2c 3a d3 00        	leaq	0xd33a2c(%rip), %rdx    # 0x14304a250
142316824: e8 57 7f ff ff              	callq	0x14230e780 <.text+0x230d780>
142316829: 48 89 c6                    	movq	%rax, %rsi
14231682c: 48 85 c0                    	testq	%rax, %rax
14231682f: 74 5b                       	je	0x14231688c <.text+0x231588c>
142316831: 48 8d 05 1d 3c d3 00        	leaq	0xd33c1d(%rip), %rax    # 0x14304a455
142316838: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
14231683c: 48 c7 45 58 44 00 00 00     	movq	$0x44, 0x58(%rbp)
142316844: 48 89 f7                    	movq	%rsi, %rdi
142316847: 48 83 c7 10                 	addq	$0x10, %rdi
14231684b: 48 83 7e 18 44              	cmpq	$0x44, 0x18(%rsi)
142316850: 75 7c                       	jne	0x1423168ce <.text+0x23158ce>
142316852: 48 8b 17                    	movq	(%rdi), %rdx
142316855: 48 8d 0d f9 3b d3 00        	leaq	0xd33bf9(%rip), %rcx    # 0x14304a455
14231685c: 41 b8 44 00 00 00           	movl	$0x44, %r8d
142316862: e8 89 d3 60 00              	callq	0x142923bf0 <.text+0x2922bf0>
142316867: 85 c0                       	testl	%eax, %eax
142316869: 75 63                       	jne	0x1423168ce <.text+0x23158ce>
14231686b: 48 8b 0e                    	movq	(%rsi), %rcx
14231686e: 48 8b 46 08                 	movq	0x8(%rsi), %rax
142316872: ff 50 18                    	callq	*0x18(%rax)
142316875: 48 85 c0                    	testq	%rax, %rax
142316878: 74 12                       	je	0x14231688c <.text+0x231588c>
14231687a: 48 8b 55 68                 	movq	0x68(%rbp), %rdx
14231687e: 41 b8 60 00 00 00           	movl	$0x60, %r8d
142316884: 48 89 c1                    	movq	%rax, %rcx
142316887: e8 54 89 ff ff              	callq	0x14230f1e0 <.text+0x230e1e0>
14231688c: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
142316890: 48 85 d2                    	testq	%rdx, %rdx
142316893: 74 13                       	je	0x1423168a8 <.text+0x23158a8>
142316895: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
142316899: 48 c1 e2 04                 	shlq	$0x4, %rdx
14231689d: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1423168a3: e8 78 d3 2f fe              	callq	0x140613c20 <.text+0x612c20>
1423168a8: 90                          	nop
1423168a9: 48 81 c4 00 01 00 00        	addq	$0x100, %rsp            # imm = 0x100
1423168b0: 5f                          	popq	%rdi
1423168b1: 5e                          	popq	%rsi
1423168b2: 5d                          	popq	%rbp
1423168b3: c3                          	retq
1423168b4: 48 8d 0d f5 44 d3 00        	leaq	0xd344f5(%rip), %rcx    # 0x14304adb0
1423168bb: 4c 8d 05 4e 45 d3 00        	leaq	0xd3454e(%rip), %r8     # 0x14304ae10
1423168c2: ba 37 00 00 00              	movl	$0x37, %edx
1423168c7: e8 64 2b 5c 00              	callq	0x1428d9430 <.text+0x28d8430>
1423168cc: eb 26                       	jmp	0x1423168f4 <.text+0x23158f4>
1423168ce: 48 c7 45 a8 00 00 00 00     	movq	$0x0, -0x58(%rbp)
1423168d6: 48 8d 05 e3 39 d3 00        	leaq	0xd339e3(%rip), %rax    # 0x14304a2c0
1423168dd: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1423168e2: 48 8d 55 50                 	leaq	0x50(%rbp), %rdx
1423168e6: 4c 8d 4d a8                 	leaq	-0x58(%rbp), %r9
1423168ea: 31 c9                       	xorl	%ecx, %ecx
1423168ec: 49 89 f8                    	movq	%rdi, %r8
1423168ef: e8 4d 42 5b 00              	callq	0x1428cab41 <.text+0x28c9b41>
1423168f4: 0f 0b                       	ud2
1423168f6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
142316900: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
142316905: 55                          	pushq	%rbp
142316906: 56                          	pushq	%rsi
142316907: 57                          	pushq	%rdi
142316908: 48 83 ec 30                 	subq	$0x30, %rsp
14231690c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
142316913: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
142316917: 48 85 d2                    	testq	%rdx, %rdx
14231691a: 74 13                       	je	0x14231692f <.text+0x231592f>
14231691c: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
142316920: 48 c1 e2 04                 	shlq	$0x4, %rdx
142316924: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14231692a: e8 f1 d2 2f fe              	callq	0x140613c20 <.text+0x612c20>
14231692f: 90                          	nop
142316930: 48 83 c4 30                 	addq	$0x30, %rsp
142316934: 5f                          	popq	%rdi
142316935: 5e                          	popq	%rsi
142316936: 5d                          	popq	%rbp
142316937: c3                          	retq
