
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1416feeb0: 48 8b 09                    	movq	(%rcx), %rcx
1416feeb3: 41 8b 40 1c                 	movl	0x1c(%r8), %eax
1416feeb7: 4d 8b 48 10                 	movq	0x10(%r8), %r9
1416feebb: 41 89 01                    	movl	%eax, (%r9)
1416feebe: 49 8b 00                    	movq	(%r8), %rax
1416feec1: 48 8b 12                    	movq	(%rdx), %rdx
1416feec4: f3 0f 10 42 58              	movss	0x58(%rdx), %xmm0
1416feec9: f2 0f 10 49 58              	movsd	0x58(%rcx), %xmm1
1416feece: f2 0f 11 88 40 01 00 00     	movsd	%xmm1, 0x140(%rax)
1416feed6: f2 0f 10 49 60              	movsd	0x60(%rcx), %xmm1
1416feedb: f2 0f 11 88 48 01 00 00     	movsd	%xmm1, 0x148(%rax)
1416feee3: f2 0f 10 49 68              	movsd	0x68(%rcx), %xmm1
1416feee8: f2 0f 11 88 50 01 00 00     	movsd	%xmm1, 0x150(%rax)
1416feef0: f2 0f 10 49 78              	movsd	0x78(%rcx), %xmm1
1416feef5: f2 0f 11 88 60 01 00 00     	movsd	%xmm1, 0x160(%rax)
1416feefd: f2 0f 10 49 70              	movsd	0x70(%rcx), %xmm1
1416fef02: f2 0f 11 88 58 01 00 00     	movsd	%xmm1, 0x158(%rax)
1416fef0a: f2 0f 10 89 80 00 00 00     	movsd	0x80(%rcx), %xmm1
1416fef12: f2 0f 11 88 68 01 00 00     	movsd	%xmm1, 0x168(%rax)
1416fef1a: 0f b6 91 88 00 00 00        	movzbl	0x88(%rcx), %edx
1416fef21: 80 79 30 00                 	cmpb	$0x0, 0x30(%rcx)
1416fef25: 74 26                       	je	0x1416fef4d <.text+0x16fdf4d>
1416fef27: 44 8b 00                    	movl	(%rax), %r8d
1416fef2a: f3 0f 10 48 04              	movss	0x4(%rax), %xmm1
1416fef2f: 84 d2                       	testb	%dl, %dl
1416fef31: 74 25                       	je	0x1416fef58 <.text+0x16fdf58>
1416fef33: 41 83 f8 02                 	cmpl	$0x2, %r8d
1416fef37: 74 2f                       	je	0x1416fef68 <.text+0x16fdf68>
1416fef39: ba 01 00 00 00              	movl	$0x1, %edx
1416fef3e: 0f 57 d2                    	xorps	%xmm2, %xmm2
1416fef41: 41 83 f8 01                 	cmpl	$0x1, %r8d
1416fef45: 75 45                       	jne	0x1416fef8c <.text+0x16fdf8c>
1416fef47: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1416fef4b: eb 3c                       	jmp	0x1416fef89 <.text+0x16fdf89>
1416fef4d: 80 f2 01                    	xorb	$0x1, %dl
1416fef50: 0f b6 d2                    	movzbl	%dl, %edx
1416fef53: c1 e2 02                    	shll	$0x2, %edx
1416fef56: eb 34                       	jmp	0x1416fef8c <.text+0x16fdf8c>
1416fef58: 41 83 f8 01                 	cmpl	$0x1, %r8d
1416fef5c: 74 03                       	je	0x1416fef61 <.text+0x16fdf61>
1416fef5e: 0f 57 c9                    	xorps	%xmm1, %xmm1
1416fef61: ba 03 00 00 00              	movl	$0x3, %edx
1416fef66: eb 21                       	jmp	0x1416fef89 <.text+0x16fdf89>
1416fef68: 0f 57 d2                    	xorps	%xmm2, %xmm2
1416fef6b: 0f 2e d1                    	ucomiss	%xmm1, %xmm2
1416fef6e: 72 10                       	jb	0x1416fef80 <.text+0x16fdf80>
1416fef70: f3 0f 58 0d 0c 6a 22 01     	addss	0x1226a0c(%rip), %xmm1  # 0x142925984
1416fef78: f3 0f 5f 0d f0 1d 23 01     	maxss	0x1231df0(%rip), %xmm1  # 0x142930d70
1416fef80: f3 0f 5c c8                 	subss	%xmm0, %xmm1
1416fef84: ba 02 00 00 00              	movl	$0x2, %edx
1416fef89: 0f 28 d1                    	movaps	%xmm1, %xmm2
1416fef8c: 89 10                       	movl	%edx, (%rax)
1416fef8e: f3 0f 11 50 04              	movss	%xmm2, 0x4(%rax)
1416fef93: 0f b6 91 89 00 00 00        	movzbl	0x89(%rcx), %edx
1416fef9a: 80 79 31 00                 	cmpb	$0x0, 0x31(%rcx)
1416fef9e: 74 27                       	je	0x1416fefc7 <.text+0x16fdfc7>
1416fefa0: 44 8b 40 08                 	movl	0x8(%rax), %r8d
1416fefa4: f3 0f 10 48 0c              	movss	0xc(%rax), %xmm1
1416fefa9: 84 d2                       	testb	%dl, %dl
1416fefab: 74 25                       	je	0x1416fefd2 <.text+0x16fdfd2>
1416fefad: 41 83 f8 02                 	cmpl	$0x2, %r8d
1416fefb1: 74 2f                       	je	0x1416fefe2 <.text+0x16fdfe2>
1416fefb3: ba 01 00 00 00              	movl	$0x1, %edx
1416fefb8: 0f 57 d2                    	xorps	%xmm2, %xmm2
1416fefbb: 41 83 f8 01                 	cmpl	$0x1, %r8d
1416fefbf: 75 45                       	jne	0x1416ff006 <.text+0x16fe006>
1416fefc1: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1416fefc5: eb 3c                       	jmp	0x1416ff003 <.text+0x16fe003>
1416fefc7: 80 f2 01                    	xorb	$0x1, %dl
1416fefca: 0f b6 d2                    	movzbl	%dl, %edx
1416fefcd: c1 e2 02                    	shll	$0x2, %edx
1416fefd0: eb 34                       	jmp	0x1416ff006 <.text+0x16fe006>
1416fefd2: 41 83 f8 01                 	cmpl	$0x1, %r8d
1416fefd6: 74 03                       	je	0x1416fefdb <.text+0x16fdfdb>
1416fefd8: 0f 57 c9                    	xorps	%xmm1, %xmm1
1416fefdb: ba 03 00 00 00              	movl	$0x3, %edx
1416fefe0: eb 21                       	jmp	0x1416ff003 <.text+0x16fe003>
1416fefe2: 0f 57 d2                    	xorps	%xmm2, %xmm2
1416fefe5: 0f 2e d1                    	ucomiss	%xmm1, %xmm2
1416fefe8: 72 10                       	jb	0x1416feffa <.text+0x16fdffa>
1416fefea: f3 0f 58 0d 92 69 22 01     	addss	0x1226992(%rip), %xmm1  # 0x142925984
1416feff2: f3 0f 5f 0d 76 1d 23 01     	maxss	0x1231d76(%rip), %xmm1  # 0x142930d70
1416feffa: f3 0f 5c c8                 	subss	%xmm0, %xmm1
1416feffe: ba 02 00 00 00              	movl	$0x2, %edx
1416ff003: 0f 28 d1                    	movaps	%xmm1, %xmm2
1416ff006: 89 50 08                    	movl	%edx, 0x8(%rax)
1416ff009: f3 0f 11 50 0c              	movss	%xmm2, 0xc(%rax)
1416ff00e: 0f b6 91 8a 00 00 00        	movzbl	0x8a(%rcx), %edx
1416ff015: 80 79 32 00                 	cmpb	$0x0, 0x32(%rcx)
1416ff019: 74 27                       	je	0x1416ff042 <.text+0x16fe042>
1416ff01b: 44 8b 40 10                 	movl	0x10(%rax), %r8d
1416ff01f: f3 0f 10 48 14              	movss	0x14(%rax), %xmm1
1416ff024: 84 d2                       	testb	%dl, %dl
1416ff026: 74 25                       	je	0x1416ff04d <.text+0x16fe04d>
1416ff028: 41 83 f8 02                 	cmpl	$0x2, %r8d
1416ff02c: 74 2f                       	je	0x1416ff05d <.text+0x16fe05d>
1416ff02e: ba 01 00 00 00              	movl	$0x1, %edx
1416ff033: 0f 57 d2                    	xorps	%xmm2, %xmm2
1416ff036: 41 83 f8 01                 	cmpl	$0x1, %r8d
1416ff03a: 75 45                       	jne	0x1416ff081 <.text+0x16fe081>
1416ff03c: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1416ff040: eb 3c                       	jmp	0x1416ff07e <.text+0x16fe07e>
1416ff042: 80 f2 01                    	xorb	$0x1, %dl
1416ff045: 0f b6 d2                    	movzbl	%dl, %edx
1416ff048: c1 e2 02                    	shll	$0x2, %edx
1416ff04b: eb 34                       	jmp	0x1416ff081 <.text+0x16fe081>
1416ff04d: 41 83 f8 01                 	cmpl	$0x1, %r8d
1416ff051: 74 03                       	je	0x1416ff056 <.text+0x16fe056>
1416ff053: 0f 57 c9                    	xorps	%xmm1, %xmm1
1416ff056: ba 03 00 00 00              	movl	$0x3, %edx
1416ff05b: eb 21                       	jmp	0x1416ff07e <.text+0x16fe07e>
1416ff05d: 0f 57 d2                    	xorps	%xmm2, %xmm2
1416ff060: 0f 2e d1                    	ucomiss	%xmm1, %xmm2
1416ff063: 72 10                       	jb	0x1416ff075 <.text+0x16fe075>
1416ff065: f3 0f 58 0d 17 69 22 01     	addss	0x1226917(%rip), %xmm1  # 0x142925984
1416ff06d: f3 0f 5f 0d fb 1c 23 01     	maxss	0x1231cfb(%rip), %xmm1  # 0x142930d70
1416ff075: f3 0f 5c c8                 	subss	%xmm0, %xmm1
1416ff079: ba 02 00 00 00              	movl	$0x2, %edx
1416ff07e: 0f 28 d1                    	movaps	%xmm1, %xmm2
1416ff081: 89 50 10                    	movl	%edx, 0x10(%rax)
1416ff084: f3 0f 11 50 14              	movss	%xmm2, 0x14(%rax)
1416ff089: 0f b6 91 8b 00 00 00        	movzbl	0x8b(%rcx), %edx
1416ff090: 80 79 33 00                 	cmpb	$0x0, 0x33(%rcx)
1416ff094: 74 27                       	je	0x1416ff0bd <.text+0x16fe0bd>
1416ff096: 44 8b 40 18                 	movl	0x18(%rax), %r8d
1416ff09a: f3 0f 10 48 1c              	movss	0x1c(%rax), %xmm1
1416ff09f: 84 d2                       	testb	%dl, %dl
1416ff0a1: 74 25                       	je	0x1416ff0c8 <.text+0x16fe0c8>
1416ff0a3: 41 83 f8 02                 	cmpl	$0x2, %r8d
1416ff0a7: 74 2f                       	je	0x1416ff0d8 <.text+0x16fe0d8>
1416ff0a9: ba 01 00 00 00              	movl	$0x1, %edx
1416ff0ae: 0f 57 d2                    	xorps	%xmm2, %xmm2
1416ff0b1: 41 83 f8 01                 	cmpl	$0x1, %r8d
1416ff0b5: 75 45                       	jne	0x1416ff0fc <.text+0x16fe0fc>
1416ff0b7: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1416ff0bb: eb 3c                       	jmp	0x1416ff0f9 <.text+0x16fe0f9>
1416ff0bd: 80 f2 01                    	xorb	$0x1, %dl
