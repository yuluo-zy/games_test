
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1403d0f70: 55                          	pushq	%rbp
1403d0f71: 41 56                       	pushq	%r14
1403d0f73: 56                          	pushq	%rsi
1403d0f74: 57                          	pushq	%rdi
1403d0f75: 53                          	pushq	%rbx
1403d0f76: 48 81 ec d0 04 00 00        	subq	$0x4d0, %rsp            # imm = 0x4D0
1403d0f7d: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1403d0f85: 48 83 e4 80                 	andq	$-0x80, %rsp
1403d0f89: 48 89 e3                    	movq	%rsp, %rbx
1403d0f8c: 48 89 ab c0 04 00 00        	movq	%rbp, 0x4c0(%rbx)
1403d0f93: 48 c7 85 48 04 00 00 fe ff ff ff    	movq	$-0x2, 0x448(%rbp)
1403d0f9e: 48 89 d6                    	movq	%rdx, %rsi
1403d0fa1: 48 c7 83 80 02 00 00 01 00 00 00    	movq	$0x1, 0x280(%rbx)
1403d0fac: 48 c7 83 88 02 00 00 01 00 00 00    	movq	$0x1, 0x288(%rbx)
1403d0fb7: c6 83 90 02 00 00 00        	movb	$0x0, 0x290(%rbx)
1403d0fbe: 0f 57 c0                    	xorps	%xmm0, %xmm0
1403d0fc1: 0f 29 83 00 01 00 00        	movaps	%xmm0, 0x100(%rbx)
1403d0fc8: 0f 29 83 80 01 00 00        	movaps	%xmm0, 0x180(%rbx)
1403d0fcf: 66 c7 83 00 02 00 00 00 00  	movw	$0x0, 0x200(%rbx)
1403d0fd8: 48 c7 83 08 02 00 00 00 00 00 00    	movq	$0x0, 0x208(%rbx)
1403d0fe3: 48 c7 83 10 02 00 00 08 00 00 00    	movq	$0x8, 0x210(%rbx)
1403d0fee: 0f 11 83 18 02 00 00        	movups	%xmm0, 0x218(%rbx)
1403d0ff5: 48 c7 83 28 02 00 00 08 00 00 00    	movq	$0x8, 0x228(%rbx)
1403d1000: 48 c7 83 30 02 00 00 00 00 00 00    	movq	$0x0, 0x230(%rbx)
1403d100b: c6 83 38 02 00 00 01        	movb	$0x1, 0x238(%rbx)
1403d1012: 0f b6 05 f0 3b 26 03        	movzbl	0x3263bf0(%rip), %eax   # 0x143634c09
1403d1019: b9 00 02 00 00              	movl	$0x200, %ecx            # imm = 0x200
1403d101e: ba 80 00 00 00              	movl	$0x80, %edx
1403d1023: e8 e8 2b 24 00              	callq	0x140613c10 <.text+0x612c10>
1403d1028: 48 85 c0                    	testq	%rax, %rax
1403d102b: 0f 84 c1 03 00 00           	je	0x1403d13f2 <.text+0x3d03f2>
1403d1031: 48 89 c7                    	movq	%rax, %rdi
1403d1034: 48 8d 93 00 01 00 00        	leaq	0x100(%rbx), %rdx
1403d103b: 41 b8 00 02 00 00           	movl	$0x200, %r8d            # imm = 0x200
1403d1041: 48 89 c1                    	movq	%rax, %rcx
1403d1044: e8 b7 2b 55 02              	callq	0x142923c00 <.text+0x2922c00>
1403d1049: 48 c7 83 e0 00 00 00 01 00 00 00    	movq	$0x1, 0xe0(%rbx)
1403d1054: 48 89 bb e8 00 00 00        	movq	%rdi, 0xe8(%rbx)
1403d105b: 48 c7 83 f0 00 00 00 01 00 00 00    	movq	$0x1, 0xf0(%rbx)
1403d1066: 48 89 bb f8 00 00 00        	movq	%rdi, 0xf8(%rbx)
1403d106d: c6 83 84 00 00 00 01        	movb	$0x1, 0x84(%rbx)
1403d1074: c6 83 85 00 00 00 01        	movb	$0x1, 0x85(%rbx)
1403d107b: e8 d0 cb 7c 00              	callq	0x140b9dc50 <.text+0xb9cc50>
1403d1080: 49 89 d0                    	movq	%rdx, %r8
1403d1083: c6 83 84 00 00 00 01        	movb	$0x1, 0x84(%rbx)
1403d108a: c6 83 85 00 00 00 01        	movb	$0x1, 0x85(%rbx)
1403d1091: 48 c7 44 24 20 09 00 00 00  	movq	$0x9, 0x20(%rsp)
1403d109a: 4c 8d 0d fd 0b 60 02        	leaq	0x2600bfd(%rip), %r9    # 0x1429d1c9e
1403d10a1: 48 8d 8b 00 01 00 00        	leaq	0x100(%rbx), %rcx
1403d10a8: 48 89 c2                    	movq	%rax, %rdx
1403d10ab: e8 60 17 99 00              	callq	0x140d62810 <.text+0xd61810>
1403d10b0: c6 83 84 00 00 00 00        	movb	$0x0, 0x84(%rbx)
1403d10b7: c6 83 85 00 00 00 01        	movb	$0x1, 0x85(%rbx)
1403d10be: 48 8d 8b 00 04 00 00        	leaq	0x400(%rbx), %rcx
1403d10c5: 48 8d 93 00 01 00 00        	leaq	0x100(%rbx), %rdx
1403d10cc: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403d10d2: 49 89 f9                    	movq	%rdi, %r9
1403d10d5: e8 26 99 17 00              	callq	0x14054aa00 <.text+0x549a00>
1403d10da: 83 bb 00 04 00 00 02        	cmpl	$0x2, 0x400(%rbx)
1403d10e1: 0f 85 2c 01 00 00           	jne	0x1403d1213 <.text+0x3d0213>
1403d10e7: 48 8b 83 08 04 00 00        	movq	0x408(%rbx), %rax
1403d10ee: 48 89 83 88 00 00 00        	movq	%rax, 0x88(%rbx)
1403d10f5: 48 8b 05 ac b2 25 03        	movq	0x325b2ac(%rip), %rax   # 0x14362c3a8
1403d10fc: 48 8b 00                    	movq	(%rax), %rax
1403d10ff: 48 85 c0                    	testq	%rax, %rax
1403d1102: 0f 84 e1 00 00 00           	je	0x1403d11e9 <.text+0x3d01e9>
1403d1108: 48 8d 83 88 00 00 00        	leaq	0x88(%rbx), %rax
1403d110f: 48 89 83 40 03 00 00        	movq	%rax, 0x340(%rbx)
1403d1116: 48 8d 05 03 bb 97 00        	leaq	0x97bb03(%rip), %rax    # 0x140d4cc20 <.text+0xd4bc20>
1403d111d: 48 89 83 48 03 00 00        	movq	%rax, 0x348(%rbx)
1403d1124: 48 8d 0d 15 0c 60 02        	leaq	0x2600c15(%rip), %rcx   # 0x1429d1d40
1403d112b: e8 50 cc 2a 00              	callq	0x14067dd80 <.text+0x67cd80>
1403d1130: 0f 10 00                    	movups	(%rax), %xmm0
1403d1133: 8b 40 10                    	movl	0x10(%rax), %eax
1403d1136: 48 c7 83 30 01 00 00 01 00 00 00    	movq	$0x1, 0x130(%rbx)
1403d1141: 48 8d 0d 10 0c 60 02        	leaq	0x2600c10(%rip), %rcx   # 0x1429d1d58
1403d1148: 48 89 8b 38 01 00 00        	movq	%rcx, 0x138(%rbx)
1403d114f: 48 c7 83 40 01 00 00 20 00 00 00    	movq	$0x20, 0x140(%rbx)
1403d115a: 48 8d 15 cf 0b 60 02        	leaq	0x2600bcf(%rip), %rdx   # 0x1429d1d30
1403d1161: 48 89 93 50 01 00 00        	movq	%rdx, 0x150(%rbx)
1403d1168: 48 c7 83 58 01 00 00 01 00 00 00    	movq	$0x1, 0x158(%rbx)
1403d1173: 48 8d 93 40 03 00 00        	leaq	0x340(%rbx), %rdx
1403d117a: 48 89 93 60 01 00 00        	movq	%rdx, 0x160(%rbx)
1403d1181: 48 c7 83 68 01 00 00 01 00 00 00    	movq	$0x1, 0x168(%rbx)
1403d118c: 48 c7 83 70 01 00 00 00 00 00 00    	movq	$0x0, 0x170(%rbx)
1403d1197: 48 c7 83 00 01 00 00 00 00 00 00    	movq	$0x0, 0x100(%rbx)
1403d11a2: 48 89 8b 08 01 00 00        	movq	%rcx, 0x108(%rbx)
1403d11a9: 48 c7 83 10 01 00 00 20 00 00 00    	movq	$0x20, 0x110(%rbx)
1403d11b4: 48 c7 83 18 01 00 00 00 00 00 00    	movq	$0x0, 0x118(%rbx)
1403d11bf: 0f 11 83 20 01 00 00        	movups	%xmm0, 0x120(%rbx)
1403d11c6: c7 83 48 01 00 00 01 00 00 00       	movl	$0x1, 0x148(%rbx)
1403d11d0: 89 83 4c 01 00 00           	movl	%eax, 0x14c(%rbx)
1403d11d6: 48 8d 8b a0 00 00 00        	leaq	0xa0(%rbx), %rcx
1403d11dd: 48 8d 93 00 01 00 00        	leaq	0x100(%rbx), %rdx
1403d11e4: e8 47 91 97 00              	callq	0x140d4a330 <.text+0xd49330>
1403d11e9: 48 c7 83 a0 00 00 00 00 00 00 00    	movq	$0x0, 0xa0(%rbx)
1403d11f4: c6 83 84 00 00 00 00        	movb	$0x0, 0x84(%rbx)
1403d11fb: c6 83 85 00 00 00 01        	movb	$0x1, 0x85(%rbx)
1403d1202: 48 8d 8b 88 00 00 00        	leaq	0x88(%rbx), %rcx
1403d1209: e8 22 ba 97 00              	callq	0x140d4cc30 <.text+0xd4bc30>
1403d120e: e9 17 01 00 00              	jmp	0x1403d132a <.text+0x3d032a>
1403d1213: 48 8d 8b 40 03 00 00        	leaq	0x340(%rbx), %rcx
1403d121a: 48 8d 93 00 04 00 00        	leaq	0x400(%rbx), %rdx
1403d1221: 41 b8 c0 00 00 00           	movl	$0xc0, %r8d
1403d1227: e8 d4 29 55 02              	callq	0x142923c00 <.text+0x2922c00>
1403d122c: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1403d1236: 48 89 83 10 01 00 00        	movq	%rax, 0x110(%rbx)
1403d123d: 48 c7 83 00 01 00 00 00 00 00 00    	movq	$0x0, 0x100(%rbx)
1403d1248: c6 83 28 01 00 00 00        	movb	$0x0, 0x128(%rbx)
1403d124f: 0f b6 05 b3 39 26 03        	movzbl	0x32639b3(%rip), %eax   # 0x143634c09
1403d1256: b9 0d 00 00 00              	movl	$0xd, %ecx
1403d125b: ba 01 00 00 00              	movl	$0x1, %edx
1403d1260: e8 ab 29 24 00              	callq	0x140613c10 <.text+0x612c10>
1403d1265: 48 85 c0                    	testq	%rax, %rax
1403d1268: 48 89 83 d8 00 00 00        	movq	%rax, 0xd8(%rbx)
1403d126f: 0f 84 8e 01 00 00           	je	0x1403d1403 <.text+0x3d0403>
1403d1275: 48 b9 20 66 65 74 63 68 65 72       	movabsq	$0x7265686374656620, %rcx # imm = 0x7265686374656620
1403d127f: 48 89 48 05                 	movq	%rcx, 0x5(%rax)
1403d1283: 48 b9 74 68 65 6d 65 20 66 65       	movabsq	$0x656620656d656874, %rcx # imm = 0x656620656D656874
1403d128d: 48 89 08                    	movq	%rcx, (%rax)
1403d1290: 48 c7 83 88 00 00 00 0d 00 00 00    	movq	$0xd, 0x88(%rbx)
1403d129b: 48 89 83 90 00 00 00        	movq	%rax, 0x90(%rbx)
1403d12a2: 48 c7 83 98 00 00 00 0d 00 00 00    	movq	$0xd, 0x98(%rbx)
1403d12ad: 48 8d 8b a0 00 00 00        	leaq	0xa0(%rbx), %rcx
1403d12b4: 48 8d 93 00 01 00 00        	leaq	0x100(%rbx), %rdx
1403d12bb: 4c 8d 83 88 00 00 00        	leaq	0x88(%rbx), %r8
1403d12c2: e8 29 46 98 00              	callq	0x140d558f0 <.text+0xd548f0>
1403d12c7: 4c 8d b3 00 01 00 00        	leaq	0x100(%rbx), %r14
1403d12ce: 48 8d 93 40 03 00 00        	leaq	0x340(%rbx), %rdx
1403d12d5: 41 b8 c0 00 00 00           	movl	$0xc0, %r8d
1403d12db: 4c 89 f1                    	movq	%r14, %rcx
1403d12de: e8 1d 29 55 02              	callq	0x142923c00 <.text+0x2922c00>
1403d12e3: c6 83 86 00 00 00 00        	movb	$0x0, 0x86(%rbx)
1403d12ea: 48 8d 8b 88 00 00 00        	leaq	0x88(%rbx), %rcx
1403d12f1: 48 8d 93 a0 00 00 00        	leaq	0xa0(%rbx), %rdx
1403d12f8: 4d 89 f0                    	movq	%r14, %r8
1403d12fb: e8 a0 a2 eb ff              	callq	0x14028b5a0 <.text+0x28a5a0>
1403d1300: 48 83 bb 88 00 00 00 00     	cmpq	$0x0, 0x88(%rbx)
1403d1308: 0f 84 0d 01 00 00           	je	0x1403d141b <.text+0x3d041b>
1403d130e: 48 8b 83 98 00 00 00        	movq	0x98(%rbx), %rax
1403d1315: 48 89 83 b0 00 00 00        	movq	%rax, 0xb0(%rbx)
1403d131c: 0f 10 83 88 00 00 00        	movups	0x88(%rbx), %xmm0
1403d1323: 0f 29 83 a0 00 00 00        	movaps	%xmm0, 0xa0(%rbx)
1403d132a: c6 83 87 00 00 00 01        	movb	$0x1, 0x87(%rbx)
1403d1331: 48 8d 8b 00 01 00 00        	leaq	0x100(%rbx), %rcx
1403d1338: e8 43 b1 46 00              	callq	0x14083c480 <.text+0x83b480>
1403d133d: c6 83 87 00 00 00 01        	movb	$0x1, 0x87(%rbx)
1403d1344: 48 8d 93 00 01 00 00        	leaq	0x100(%rbx), %rdx
1403d134b: 48 89 f1                    	movq	%rsi, %rcx
1403d134e: e8 bd 4e d6 ff              	callq	0x140136210 <.text+0x135210>
1403d1353: c6 83 00 01 00 00 00        	movb	$0x0, 0x100(%rbx)
1403d135a: 48 c7 83 08 01 00 00 01 00 00 00    	movq	$0x1, 0x108(%rbx)
1403d1365: 48 89 bb 10 01 00 00        	movq	%rdi, 0x110(%rbx)
1403d136c: c6 83 87 00 00 00 00        	movb	$0x0, 0x87(%rbx)
1403d1373: 48 8d 93 00 01 00 00        	leaq	0x100(%rbx), %rdx
1403d137a: 48 89 c1                    	movq	%rax, %rcx
1403d137d: e8 8e b0 d6 ff              	callq	0x14013c410 <.text+0x13b410>
1403d1382: c6 83 87 00 00 00 00        	movb	$0x0, 0x87(%rbx)
1403d1389: 48 89 c1                    	movq	%rax, %rcx
1403d138c: e8 7f 5e f5 ff              	callq	0x140327210 <.text+0x326210>
1403d1391: 48 8b b3 a0 00 00 00        	movq	0xa0(%rbx), %rsi
1403d1398: 48 85 f6                    	testq	%rsi, %rsi
1403d139b: 74 46                       	je	0x1403d13e3 <.text+0x3d03e3>
1403d139d: 48 8b 8b b0 00 00 00        	movq	0xb0(%rbx), %rcx
1403d13a4: ff 15 46 10 dc 02           	callq	*0x2dc1046(%rip)        # 0x1431923f0
1403d13aa: f0                          	lock
1403d13ab: 48 ff 0e                    	decq	(%rsi)
1403d13ae: 75 0c                       	jne	0x1403d13bc <.text+0x3d03bc>
1403d13b0: 48 8d 8b a0 00 00 00        	leaq	0xa0(%rbx), %rcx
1403d13b7: e8 a4 27 98 00              	callq	0x140d53b60 <.text+0xd52b60>
1403d13bc: 48 8b 83 a8 00 00 00        	movq	0xa8(%rbx), %rax
1403d13c3: f0                          	lock
1403d13c4: 48 ff 08                    	decq	(%rax)
1403d13c7: 75 1a                       	jne	0x1403d13e3 <.text+0x3d03e3>
1403d13c9: 48 8d 8b a8 00 00 00        	leaq	0xa8(%rbx), %rcx
1403d13d0: c6 83 84 00 00 00 00        	movb	$0x0, 0x84(%rbx)
1403d13d7: c6 83 85 00 00 00 00        	movb	$0x0, 0x85(%rbx)
1403d13de: e8 cd 29 98 00              	callq	0x140d53db0 <.text+0xd52db0>
1403d13e3: 90                          	nop
1403d13e4: 48 8d a5 50 04 00 00        	leaq	0x450(%rbp), %rsp
1403d13eb: 5b                          	popq	%rbx
1403d13ec: 5f                          	popq	%rdi
1403d13ed: 5e                          	popq	%rsi
1403d13ee: 41 5e                       	popq	%r14
1403d13f0: 5d                          	popq	%rbp
1403d13f1: c3                          	retq
1403d13f2: b9 80 00 00 00              	movl	$0x80, %ecx
1403d13f7: ba 00 02 00 00              	movl	$0x200, %edx            # imm = 0x200
1403d13fc: e8 ff 7b 50 02              	callq	0x1428d9000 <.text+0x28d8000>
1403d1401: eb 51                       	jmp	0x1403d1454 <.text+0x3d0454>
1403d1403: 4c 8d 05 2e 06 60 02        	leaq	0x260062e(%rip), %r8    # 0x1429d1a38
1403d140a: b9 01 00 00 00              	movl	$0x1, %ecx
1403d140f: ba 0d 00 00 00              	movl	$0xd, %edx
1403d1414: e8 ca 7b 50 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1403d1419: eb 39                       	jmp	0x1403d1454 <.text+0x3d0454>
1403d141b: 48 8b 83 90 00 00 00        	movq	0x90(%rbx), %rax
1403d1422: 48 89 83 00 01 00 00        	movq	%rax, 0x100(%rbx)
1403d1429: 48 8d 05 c0 08 60 02        	leaq	0x26008c0(%rip), %rax   # 0x1429d1cf0
1403d1430: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1403d1435: 48 8d 0d 98 08 60 02        	leaq	0x2600898(%rip), %rcx   # 0x1429d1cd4
1403d143c: 4c 8d 0d 65 05 60 02        	leaq	0x2600565(%rip), %r9    # 0x1429d19a8
1403d1443: 4c 8d 83 00 01 00 00        	leaq	0x100(%rbx), %r8
1403d144a: ba 1c 00 00 00              	movl	$0x1c, %edx
1403d144f: e8 0c 83 50 02              	callq	0x1428d9760 <.text+0x28d8760>
