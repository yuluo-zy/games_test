
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140ddffb0: 55                          	pushq	%rbp
140ddffb1: 41 56                       	pushq	%r14
140ddffb3: 56                          	pushq	%rsi
140ddffb4: 57                          	pushq	%rdi
140ddffb5: 53                          	pushq	%rbx
140ddffb6: 48 81 ec a0 03 00 00        	subq	$0x3a0, %rsp            # imm = 0x3A0
140ddffbd: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140ddffc5: 48 c7 85 18 03 00 00 fe ff ff ff    	movq	$-0x2, 0x318(%rbp)
140ddffd0: 89 d3                       	movl	%edx, %ebx
140ddffd2: 48 89 ce                    	movq	%rcx, %rsi
140ddffd5: 0f b6 05 2d 4c 85 02        	movzbl	0x2854c2d(%rip), %eax   # 0x143634c09
140ddffdc: b9 50 01 00 00              	movl	$0x150, %ecx            # imm = 0x150
140ddffe1: ba 08 00 00 00              	movl	$0x8, %edx
140ddffe6: e8 25 3c 83 ff              	callq	0x140613c10 <.text+0x612c10>
140ddffeb: 48 85 c0                    	testq	%rax, %rax
140ddffee: 0f 84 0c 03 00 00           	je	0x140de0300 <.text+0xddf300>
140ddfff4: 48 89 85 10 03 00 00        	movq	%rax, 0x310(%rbp)
140ddfffb: 0f 57 c0                    	xorps	%xmm0, %xmm0
140ddfffe: 0f 11 45 30                 	movups	%xmm0, 0x30(%rbp)
140de0002: 0f 11 45 48                 	movups	%xmm0, 0x48(%rbp)
140de0006: 0f 11 45 60                 	movups	%xmm0, 0x60(%rbp)
140de000a: 0f 11 45 78                 	movups	%xmm0, 0x78(%rbp)
140de000e: 0f 11 85 90 00 00 00        	movups	%xmm0, 0x90(%rbp)
140de0015: c7 85 a0 00 00 00 00 00 00 00       	movl	$0x0, 0xa0(%rbp)
140de001f: 0f 11 85 c8 00 00 00        	movups	%xmm0, 0xc8(%rbp)
140de0026: 0f 11 85 e0 00 00 00        	movups	%xmm0, 0xe0(%rbp)
140de002d: 0f 11 85 f8 00 00 00        	movups	%xmm0, 0xf8(%rbp)
140de0034: 0f 11 85 10 01 00 00        	movups	%xmm0, 0x110(%rbp)
140de003b: 0f 11 85 28 01 00 00        	movups	%xmm0, 0x128(%rbp)
140de0042: c7 85 38 01 00 00 00 00 00 00       	movl	$0x0, 0x138(%rbp)
140de004c: 48 c7 45 10 00 00 00 00     	movq	$0x0, 0x10(%rbp)
140de0054: 48 c7 45 18 08 00 00 00     	movq	$0x8, 0x18(%rbp)
140de005c: 48 c7 45 20 00 00 00 00     	movq	$0x0, 0x20(%rbp)
140de0064: 48 c7 45 28 10 00 00 00     	movq	$0x10, 0x28(%rbp)
140de006c: 48 c7 45 40 10 00 00 00     	movq	$0x10, 0x40(%rbp)
140de0074: 48 c7 45 58 10 00 00 00     	movq	$0x10, 0x58(%rbp)
140de007c: 48 c7 45 70 10 00 00 00     	movq	$0x10, 0x70(%rbp)
140de0084: 48 c7 85 88 00 00 00 10 00 00 00    	movq	$0x10, 0x88(%rbp)
140de008f: 49 be 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %r14 # imm = 0x8000000000000000
140de0099: 4c 89 b5 a8 00 00 00        	movq	%r14, 0xa8(%rbp)
140de00a0: 48 8d 05 3e 56 d9 01        	leaq	0x1d9563e(%rip), %rax   # 0x142b756e5
140de00a7: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
140de00ae: 48 c7 85 b8 00 00 00 3d 00 00 00    	movq	$0x3d, 0xb8(%rbp)
140de00b9: 48 c7 85 c0 00 00 00 10 00 00 00    	movq	$0x10, 0xc0(%rbp)
140de00c4: 48 c7 85 d8 00 00 00 10 00 00 00    	movq	$0x10, 0xd8(%rbp)
140de00cf: 48 c7 85 f0 00 00 00 10 00 00 00    	movq	$0x10, 0xf0(%rbp)
140de00da: 48 c7 85 08 01 00 00 10 00 00 00    	movq	$0x10, 0x108(%rbp)
140de00e5: 48 c7 85 20 01 00 00 10 00 00 00    	movq	$0x10, 0x120(%rbp)
140de00f0: c7 85 40 01 00 00 00 00 00 00       	movl	$0x0, 0x140(%rbp)
140de00fa: 66 c7 85 44 01 00 00 01 00  	movw	$0x1, 0x144(%rbp)
140de0103: 4c 89 b5 40 02 00 00        	movq	%r14, 0x240(%rbp)
140de010a: c7 85 98 02 00 00 00 00 00 00       	movl	$0x0, 0x298(%rbp)
140de0114: 0f b6 05 ee 4a 85 02        	movzbl	0x2854aee(%rip), %eax   # 0x143634c09
140de011b: b9 90 02 00 00              	movl	$0x290, %ecx            # imm = 0x290
140de0120: ba 08 00 00 00              	movl	$0x8, %edx
140de0125: e8 e6 3a 83 ff              	callq	0x140613c10 <.text+0x612c10>
140de012a: 48 85 c0                    	testq	%rax, %rax
140de012d: 0f 84 dc 01 00 00           	je	0x140de030f <.text+0xddf30f>
140de0133: 48 89 c7                    	movq	%rax, %rdi
140de0136: 48 8d 55 10                 	leaq	0x10(%rbp), %rdx
140de013a: 41 b8 90 02 00 00           	movl	$0x290, %r8d            # imm = 0x290
140de0140: 48 89 c1                    	movq	%rax, %rcx
140de0143: e8 b8 3a b4 01              	callq	0x142923c00 <.text+0x2922c00>
140de0148: 4c 8d 05 c1 54 d9 01        	leaq	0x1d954c1(%rip), %r8    # 0x142b75610
140de014f: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140de0153: 48 89 fa                    	movq	%rdi, %rdx
140de0156: e8 b5 49 f0 ff              	callq	0x140ce4b10 <.text+0xce3b10>
140de015b: 48 8d 8d a0 02 00 00        	leaq	0x2a0(%rbp), %rcx
140de0162: e8 99 77 c5 00              	callq	0x141a37900 <.text+0x1a36900>
140de0167: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140de016b: e8 c0 e6 c0 00              	callq	0x1419ee830 <.text+0x19ed830>
140de0170: 0f 10 45 00                 	movups	(%rbp), %xmm0
140de0174: 48 8b 85 10 03 00 00        	movq	0x310(%rbp), %rax
140de017b: 0f 11 40 60                 	movups	%xmm0, 0x60(%rax)
140de017f: 0f 10 45 f0                 	movups	-0x10(%rbp), %xmm0
140de0183: 0f 11 40 50                 	movups	%xmm0, 0x50(%rax)
140de0187: 0f 10 45 e0                 	movups	-0x20(%rbp), %xmm0
140de018b: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
140de018f: 0f 10 45 a0                 	movups	-0x60(%rbp), %xmm0
140de0193: 0f 10 4d b0                 	movups	-0x50(%rbp), %xmm1
140de0197: 0f 10 55 c0                 	movups	-0x40(%rbp), %xmm2
140de019b: 0f 10 5d d0                 	movups	-0x30(%rbp), %xmm3
140de019f: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
140de01a3: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
140de01a7: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
140de01ab: 0f 11 00                    	movups	%xmm0, (%rax)
140de01ae: 0f 10 85 a0 02 00 00        	movups	0x2a0(%rbp), %xmm0
140de01b5: 0f 10 8d b0 02 00 00        	movups	0x2b0(%rbp), %xmm1
140de01bc: 0f 10 95 c0 02 00 00        	movups	0x2c0(%rbp), %xmm2
140de01c3: 0f 10 9d d0 02 00 00        	movups	0x2d0(%rbp), %xmm3
140de01ca: 0f 11 40 70                 	movups	%xmm0, 0x70(%rax)
140de01ce: 0f 11 88 80 00 00 00        	movups	%xmm1, 0x80(%rax)
140de01d5: 0f 11 90 90 00 00 00        	movups	%xmm2, 0x90(%rax)
140de01dc: 0f 11 98 a0 00 00 00        	movups	%xmm3, 0xa0(%rax)
140de01e3: 0f 10 85 e0 02 00 00        	movups	0x2e0(%rbp), %xmm0
140de01ea: 0f 11 80 b0 00 00 00        	movups	%xmm0, 0xb0(%rax)
140de01f1: 0f 10 85 f0 02 00 00        	movups	0x2f0(%rbp), %xmm0
140de01f8: 0f 11 80 c0 00 00 00        	movups	%xmm0, 0xc0(%rax)
140de01ff: 0f 10 85 00 03 00 00        	movups	0x300(%rbp), %xmm0
140de0206: 0f 11 80 d0 00 00 00        	movups	%xmm0, 0xd0(%rax)
140de020d: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
140de0211: 0f 10 4d 20                 	movups	0x20(%rbp), %xmm1
140de0215: 0f 10 55 30                 	movups	0x30(%rbp), %xmm2
140de0219: 0f 10 5d 40                 	movups	0x40(%rbp), %xmm3
140de021d: 0f 11 80 e0 00 00 00        	movups	%xmm0, 0xe0(%rax)
140de0224: 0f 11 88 f0 00 00 00        	movups	%xmm1, 0xf0(%rax)
140de022b: 0f 11 90 00 01 00 00        	movups	%xmm2, 0x100(%rax)
140de0232: 0f 11 98 10 01 00 00        	movups	%xmm3, 0x110(%rax)
140de0239: 0f 10 45 50                 	movups	0x50(%rbp), %xmm0
140de023d: 0f 11 80 20 01 00 00        	movups	%xmm0, 0x120(%rax)
140de0244: 0f 10 45 60                 	movups	0x60(%rbp), %xmm0
140de0248: 0f 11 80 30 01 00 00        	movups	%xmm0, 0x130(%rax)
140de024f: 0f 10 45 70                 	movups	0x70(%rbp), %xmm0
140de0253: 0f 11 80 40 01 00 00        	movups	%xmm0, 0x140(%rax)
140de025a: 48 c7 45 18 03 00 00 00     	movq	$0x3, 0x18(%rbp)
140de0262: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
140de0266: 48 c7 45 28 03 00 00 00     	movq	$0x3, 0x28(%rbp)
140de026e: 48 c7 45 30 00 00 00 00     	movq	$0x0, 0x30(%rbp)
140de0276: 48 c7 45 38 08 00 00 00     	movq	$0x8, 0x38(%rbp)
140de027e: 0f 57 c0                    	xorps	%xmm0, %xmm0
140de0281: 0f 11 45 40                 	movups	%xmm0, 0x40(%rbp)
140de0285: 4c 89 75 10                 	movq	%r14, 0x10(%rbp)
140de0289: 88 9d a0 02 00 00           	movb	%bl, 0x2a0(%rbp)
140de028f: 48 8b 0d da c0 84 02        	movq	0x284c0da(%rip), %rcx   # 0x14362c370
140de0296: 4c 8d 05 a3 47 d9 01        	leaq	0x1d947a3(%rip), %r8    # 0x142b74a40
140de029d: 48 8d 95 a0 02 00 00        	leaq	0x2a0(%rbp), %rdx
140de02a4: e8 e7 c8 ff ff              	callq	0x140ddcb90 <.text+0xddbb90>
140de02a9: 49 89 d0                    	movq	%rdx, %r8
140de02ac: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140de02b0: 48 89 c2                    	movq	%rax, %rdx
140de02b3: e8 c8 df ff ff              	callq	0x140dde280 <.text+0xddd280>
140de02b8: 0f 10 45 70                 	movups	0x70(%rbp), %xmm0
140de02bc: 0f 11 46 60                 	movups	%xmm0, 0x60(%rsi)
140de02c0: 0f 10 45 60                 	movups	0x60(%rbp), %xmm0
140de02c4: 0f 11 46 50                 	movups	%xmm0, 0x50(%rsi)
140de02c8: 0f 10 45 50                 	movups	0x50(%rbp), %xmm0
140de02cc: 0f 11 46 40                 	movups	%xmm0, 0x40(%rsi)
140de02d0: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
140de02d4: 0f 10 4d 20                 	movups	0x20(%rbp), %xmm1
140de02d8: 0f 10 55 30                 	movups	0x30(%rbp), %xmm2
140de02dc: 0f 10 5d 40                 	movups	0x40(%rbp), %xmm3
140de02e0: 0f 11 5e 30                 	movups	%xmm3, 0x30(%rsi)
140de02e4: 0f 11 56 20                 	movups	%xmm2, 0x20(%rsi)
140de02e8: 0f 11 4e 10                 	movups	%xmm1, 0x10(%rsi)
140de02ec: 0f 11 06                    	movups	%xmm0, (%rsi)
140de02ef: 48 89 f0                    	movq	%rsi, %rax
140de02f2: 48 81 c4 a0 03 00 00        	addq	$0x3a0, %rsp            # imm = 0x3A0
140de02f9: 5b                          	popq	%rbx
140de02fa: 5f                          	popq	%rdi
140de02fb: 5e                          	popq	%rsi
140de02fc: 41 5e                       	popq	%r14
140de02fe: 5d                          	popq	%rbp
140de02ff: c3                          	retq
140de0300: b9 08 00 00 00              	movl	$0x8, %ecx
140de0305: ba 50 01 00 00              	movl	$0x150, %edx            # imm = 0x150
140de030a: e8 f1 8c af 01              	callq	0x1428d9000 <.text+0x28d8000>
140de030f: b9 08 00 00 00              	movl	$0x8, %ecx
140de0314: ba 90 02 00 00              	movl	$0x290, %edx            # imm = 0x290
140de0319: e8 e2 8c af 01              	callq	0x1428d9000 <.text+0x28d8000>
140de031e: 0f 0b                       	ud2
140de0320: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140de0325: 55                          	pushq	%rbp
140de0326: 41 56                       	pushq	%r14
140de0328: 56                          	pushq	%rsi
140de0329: 57                          	pushq	%rdi
140de032a: 53                          	pushq	%rbx
140de032b: 48 83 ec 20                 	subq	$0x20, %rsp
140de032f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140de0336: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140de033a: e8 71 f0 7e ff              	callq	0x1405cf3b0 <.text+0x5ce3b0>
140de033f: 90                          	nop
140de0340: 48 83 c4 20                 	addq	$0x20, %rsp
140de0344: 5b                          	popq	%rbx
140de0345: 5f                          	popq	%rdi
140de0346: 5e                          	popq	%rsi
140de0347: 41 5e                       	popq	%r14
140de0349: 5d                          	popq	%rbp
140de034a: c3                          	retq
140de034b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140de0350: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140de0355: 55                          	pushq	%rbp
140de0356: 41 56                       	pushq	%r14
140de0358: 56                          	pushq	%rsi
140de0359: 57                          	pushq	%rdi
140de035a: 53                          	pushq	%rbx
140de035b: 48 83 ec 20                 	subq	$0x20, %rsp
140de035f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140de0366: ba 50 01 00 00              	movl	$0x150, %edx            # imm = 0x150
140de036b: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140de0371: 48 8b 8d 10 03 00 00        	movq	0x310(%rbp), %rcx
140de0378: e8 a3 38 83 ff              	callq	0x140613c20 <.text+0x612c20>
140de037d: 90                          	nop
140de037e: 48 83 c4 20                 	addq	$0x20, %rsp
140de0382: 5b                          	popq	%rbx
140de0383: 5f                          	popq	%rdi
140de0384: 5e                          	popq	%rsi
140de0385: 41 5e                       	popq	%r14
140de0387: 5d                          	popq	%rbp
140de0388: c3                          	retq
140de0389: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140de0390: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140de0395: 55                          	pushq	%rbp
140de0396: 41 56                       	pushq	%r14
140de0398: 56                          	pushq	%rsi
140de0399: 57                          	pushq	%rdi
140de039a: 53                          	pushq	%rbx
140de039b: 48 83 ec 20                 	subq	$0x20, %rsp
140de039f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140de03a6: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140de03aa: e8 d1 94 24 ff              	callq	0x140029880 <.text+0x28880>
140de03af: 90                          	nop
140de03b0: 48 83 c4 20                 	addq	$0x20, %rsp
140de03b4: 5b                          	popq	%rbx
140de03b5: 5f                          	popq	%rdi
140de03b6: 5e                          	popq	%rsi
140de03b7: 41 5e                       	popq	%r14
140de03b9: 5d                          	popq	%rbp
140de03ba: c3                          	retq
140de03bb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140de03c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140de03c5: 55                          	pushq	%rbp
140de03c6: 41 56                       	pushq	%r14
140de03c8: 56                          	pushq	%rsi
140de03c9: 57                          	pushq	%rdi
140de03ca: 53                          	pushq	%rbx
140de03cb: 48 83 ec 20                 	subq	$0x20, %rsp
140de03cf: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140de03d6: 48 8d 8d a0 02 00 00        	leaq	0x2a0(%rbp), %rcx
140de03dd: e8 9e 94 24 ff              	callq	0x140029880 <.text+0x28880>
140de03e2: 90                          	nop
140de03e3: 48 83 c4 20                 	addq	$0x20, %rsp
140de03e7: 5b                          	popq	%rbx
140de03e8: 5f                          	popq	%rdi
140de03e9: 5e                          	popq	%rsi
140de03ea: 41 5e                       	popq	%r14
140de03ec: 5d                          	popq	%rbp
140de03ed: c3                          	retq
140de03ee: 66 90                       	nop
140de03f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140de03f5: 55                          	pushq	%rbp
140de03f6: 41 56                       	pushq	%r14
140de03f8: 56                          	pushq	%rsi
140de03f9: 57                          	pushq	%rdi
140de03fa: 53                          	pushq	%rbx
140de03fb: 48 83 ec 20                 	subq	$0x20, %rsp
140de03ff: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140de0406: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140de040a: e8 71 94 24 ff              	callq	0x140029880 <.text+0x28880>
140de040f: 90                          	nop
140de0410: 48 83 c4 20                 	addq	$0x20, %rsp
140de0414: 5b                          	popq	%rbx
140de0415: 5f                          	popq	%rdi
140de0416: 5e                          	popq	%rsi
140de0417: 41 5e                       	popq	%r14
140de0419: 5d                          	popq	%rbp
140de041a: c3                          	retq
