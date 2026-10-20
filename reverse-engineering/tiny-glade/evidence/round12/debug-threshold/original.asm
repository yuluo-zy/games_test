
reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a8b0c0: 41 57                       	pushq	%r15
140a8b0c2: 41 56                       	pushq	%r14
140a8b0c4: 41 55                       	pushq	%r13
140a8b0c6: 41 54                       	pushq	%r12
140a8b0c8: 56                          	pushq	%rsi
140a8b0c9: 57                          	pushq	%rdi
140a8b0ca: 55                          	pushq	%rbp
140a8b0cb: 53                          	pushq	%rbx
140a8b0cc: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
140a8b0d3: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
140a8b0db: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
140a8b0e4: 48 8b 01                    	movq	(%rcx), %rax
140a8b0e7: 80 b8 f3 00 00 00 01        	cmpb	$0x1, 0xf3(%rax)
140a8b0ee: 0f 85 ec 03 00 00           	jne	0x140a8b4e0 <.text+0xa8a4e0>
140a8b0f4: 48 8b 1a                    	movq	(%rdx), %rbx
140a8b0f7: 4c 8b 73 18                 	movq	0x18(%rbx), %r14
140a8b0fb: 4d 85 f6                    	testq	%r14, %r14
140a8b0fe: 0f 84 dc 03 00 00           	je	0x140a8b4e0 <.text+0xa8a4e0>
140a8b104: 49 8b 30                    	movq	(%r8), %rsi
140a8b107: 4c 8b 23                    	movq	(%rbx), %r12
140a8b10a: 4d 8d 6c 24 10              	leaq	0x10(%r12), %r13
140a8b10f: 66 41 0f 6f 04 24           	movdqa	(%r12), %xmm0
140a8b115: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
140a8b119: f7 d0                       	notl	%eax
140a8b11b: 4c 8d 7b 20                 	leaq	0x20(%rbx), %r15
140a8b11f: 48 8d 54 24 30              	leaq	0x30(%rsp), %rdx
140a8b124: 66 0f 76 f6                 	pcmpeqd	%xmm6, %xmm6
140a8b128: 0f 57 ff                    	xorps	%xmm7, %xmm7
140a8b12b: 48 89 b4 24 80 00 00 00     	movq	%rsi, 0x80(%rsp)
140a8b133: 66 85 c0                    	testw	%ax, %ax
140a8b136: 89 4c 24 24                 	movl	%ecx, 0x24(%rsp)
140a8b13a: 74 14                       	je	0x140a8b150 <.text+0xa8a150>
140a8b13c: 8d 48 ff                    	leal	-0x1(%rax), %ecx
140a8b13f: 21 c1                       	andl	%eax, %ecx
140a8b141: 89 4c 24 28                 	movl	%ecx, 0x28(%rsp)
140a8b145: eb 39                       	jmp	0x140a8b180 <.text+0xa8a180>
140a8b147: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140a8b150: 66 41 0f 6f 45 00           	movdqa	(%r13), %xmm0
140a8b156: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
140a8b15a: 49 81 c4 80 d5 ff ff        	addq	$-0x2a80, %r12          # imm = 0xD580
140a8b161: 49 83 c5 10                 	addq	$0x10, %r13
140a8b165: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
140a8b16a: 74 e4                       	je	0x140a8b150 <.text+0xa8a150>
140a8b16c: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
140a8b171: 29 c1                       	subl	%eax, %ecx
140a8b173: f7 d0                       	notl	%eax
140a8b175: 41 89 c0                    	movl	%eax, %r8d
140a8b178: 41 21 c8                    	andl	%ecx, %r8d
140a8b17b: 44 89 44 24 28              	movl	%r8d, 0x28(%rsp)
140a8b180: f3 0f bc c0                 	tzcntl	%eax, %eax
140a8b184: 48 f7 d8                    	negq	%rax
140a8b187: 48 69 c0 a8 02 00 00        	imulq	$0x2a8, %rax, %rax      # imm = 0x2A8
140a8b18e: 49 8b bc 04 58 fd ff ff     	movq	-0x2a8(%r12,%rax), %rdi
140a8b196: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
140a8b19b: 48 83 7b 18 00              	cmpq	$0x0, 0x18(%rbx)
140a8b1a0: 0f 84 5e 03 00 00           	je	0x140a8b504 <.text+0xa8a504>
140a8b1a6: 49 ff ce                    	decq	%r14
140a8b1a9: 4c 89 f9                    	movq	%r15, %rcx
140a8b1ac: e8 bf 5b b2 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140a8b1b1: 48 8b 0b                    	movq	(%rbx), %rcx
140a8b1b4: 48 8b 53 08                 	movq	0x8(%rbx), %rdx
140a8b1b8: 49 89 d0                    	movq	%rdx, %r8
140a8b1bb: 49 21 c0                    	andq	%rax, %r8
140a8b1be: 48 c1 e8 39                 	shrq	$0x39, %rax
140a8b1c2: 66 0f 6e c0                 	movd	%eax, %xmm0
140a8b1c6: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140a8b1ca: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140a8b1cf: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140a8b1d4: 48 8d 81 58 fd ff ff        	leaq	-0x2a8(%rcx), %rax
140a8b1db: 45 31 c9                    	xorl	%r9d, %r9d
140a8b1de: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
140a8b1e4: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
140a8b1e8: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
140a8b1ec: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
140a8b1f1: 45 85 d2                    	testl	%r10d, %r10d
140a8b1f4: 74 2a                       	je	0x140a8b220 <.text+0xa8a220>
140a8b1f6: f3 45 0f bc da              	tzcntl	%r10d, %r11d
140a8b1fb: 4d 01 c3                    	addq	%r8, %r11
140a8b1fe: 49 21 d3                    	andq	%rdx, %r11
140a8b201: 49 f7 db                    	negq	%r11
140a8b204: 4d 69 db a8 02 00 00        	imulq	$0x2a8, %r11, %r11      # imm = 0x2A8
140a8b20b: 4a 39 3c 18                 	cmpq	%rdi, (%rax,%r11)
140a8b20f: 74 3f                       	je	0x140a8b250 <.text+0xa8a250>
140a8b211: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
140a8b215: 66 45 21 d3                 	andw	%r10w, %r11w
140a8b219: 45 89 da                    	movl	%r11d, %r10d
140a8b21c: 75 d8                       	jne	0x140a8b1f6 <.text+0xa8a1f6>
140a8b21e: 66 90                       	nop
140a8b220: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
140a8b224: 66 44 0f d7 d1              	pmovmskb	%xmm1, %r10d
140a8b229: 45 85 d2                    	testl	%r10d, %r10d
140a8b22c: 0f 85 d2 02 00 00           	jne	0x140a8b504 <.text+0xa8a504>
140a8b232: 4d 01 c8                    	addq	%r9, %r8
140a8b235: 49 83 c0 10                 	addq	$0x10, %r8
140a8b239: 49 83 c1 10                 	addq	$0x10, %r9
140a8b23d: 49 21 d0                    	andq	%rdx, %r8
140a8b240: eb 9c                       	jmp	0x140a8b1de <.text+0xa8a1de>
140a8b242: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140a8b250: 4a 83 bc 19 70 fd ff ff 02  	cmpq	$0x2, -0x290(%rcx,%r11)
140a8b259: 0f 82 b1 02 00 00           	jb	0x140a8b510 <.text+0xa8a510>
140a8b25f: f3 42 0f 10 84 19 90 fd ff ff       	movss	-0x270(%rcx,%r11), %xmm0
140a8b269: 0f 2e c7                    	ucomiss	%xmm7, %xmm0
140a8b26c: 0f 86 9e 02 00 00           	jbe	0x140a8b510 <.text+0xa8a510>
140a8b272: 4c 01 d9                    	addq	%r11, %rcx
140a8b275: 48 8b b1 f8 fd ff ff        	movq	-0x208(%rcx), %rsi
140a8b27c: 48 8b 81 30 fe ff ff        	movq	-0x1d0(%rcx), %rax
140a8b283: 48 39 f0                    	cmpq	%rsi, %rax
140a8b286: 48 0f 42 f0                 	cmovbq	%rax, %rsi
140a8b28a: 48 8b 81 48 fe ff ff        	movq	-0x1b8(%rcx), %rax
140a8b291: 48 39 f0                    	cmpq	%rsi, %rax
140a8b294: 48 0f 42 f0                 	cmovbq	%rax, %rsi
140a8b298: 48 8b 81 60 fe ff ff        	movq	-0x1a0(%rcx), %rax
140a8b29f: 48 39 f0                    	cmpq	%rsi, %rax
140a8b2a2: 48 0f 42 f0                 	cmovbq	%rax, %rsi
140a8b2a6: 48 85 f6                    	testq	%rsi, %rsi
140a8b2a9: 0f 84 1b 02 00 00           	je	0x140a8b4ca <.text+0xa8a4ca>
140a8b2af: 4c 89 64 24 60              	movq	%r12, 0x60(%rsp)
140a8b2b4: 4c 89 7c 24 68              	movq	%r15, 0x68(%rsp)
140a8b2b9: 4c 89 74 24 70              	movq	%r14, 0x70(%rsp)
140a8b2be: 48 89 5c 24 78              	movq	%rbx, 0x78(%rsp)
140a8b2c3: 48 8b b9 f0 fd ff ff        	movq	-0x210(%rcx), %rdi
140a8b2ca: 48 8b 81 28 fe ff ff        	movq	-0x1d8(%rcx), %rax
140a8b2d1: 48 89 84 24 98 00 00 00     	movq	%rax, 0x98(%rsp)
140a8b2d9: 48 8b 81 40 fe ff ff        	movq	-0x1c0(%rcx), %rax
140a8b2e0: 48 89 84 24 90 00 00 00     	movq	%rax, 0x90(%rsp)
140a8b2e8: 48 8b 81 58 fe ff ff        	movq	-0x1a8(%rcx), %rax
140a8b2ef: 48 89 84 24 88 00 00 00     	movq	%rax, 0x88(%rsp)
140a8b2f7: 81 64 24 24 00 ff ff ff     	andl	$0xffffff00, 0x24(%rsp) # imm = 0xFFFFFF00
140a8b2ff: 81 e5 00 ff ff ff           	andl	$0xffffff00, %ebp       # imm = 0xFFFFFF00
140a8b305: 89 6c 24 2c                 	movl	%ebp, 0x2c(%rsp)
140a8b309: 81 64 24 5c 00 ff ff ff     	andl	$0xffffff00, 0x5c(%rsp) # imm = 0xFFFFFF00
140a8b311: 45 31 ff                    	xorl	%r15d, %r15d
140a8b314: 48 8d 5c 24 44              	leaq	0x44(%rsp), %rbx
140a8b319: 48 8d 6c 24 30              	leaq	0x30(%rsp), %rbp
140a8b31e: 4c 8b a4 24 80 00 00 00     	movq	0x80(%rsp), %r12
140a8b326: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140a8b330: 4d 8d 77 01                 	leaq	0x1(%r15), %r14
140a8b334: 8b 47 08                    	movl	0x8(%rdi), %eax
140a8b337: 89 43 08                    	movl	%eax, 0x8(%rbx)
140a8b33a: 48 8b 07                    	movq	(%rdi), %rax
140a8b33d: 48 89 03                    	movq	%rax, (%rbx)
140a8b340: c7 44 24 30 00 00 00 00     	movl	$0x0, 0x30(%rsp)
140a8b348: c7 44 24 40 cd cc cc 3d     	movl	$0x3dcccccd, 0x40(%rsp) # imm = 0x3DCCCCCD
140a8b350: c7 44 24 50 00 00 80 3f     	movl	$0x3f800000, 0x50(%rsp) # imm = 0x3F800000
140a8b358: c6 44 24 54 00              	movb	$0x0, 0x54(%rsp)
140a8b35d: 48 89 e9                    	movq	%rbp, %rcx
140a8b360: 4c 89 e2                    	movq	%r12, %rdx
140a8b363: e8 b8 71 07 00              	callq	0x140b02520 <.text+0xb01520>
140a8b368: 48 8b 84 24 98 00 00 00     	movq	0x98(%rsp), %rax
140a8b370: f3 42 0f 10 04 b8           	movss	(%rax,%r15,4), %xmm0
140a8b376: 0f 28 c8                    	movaps	%xmm0, %xmm1
140a8b379: f3 0f 59 cf                 	mulss	%xmm7, %xmm1
140a8b37d: f3 0f 10 57 08              	movss	0x8(%rdi), %xmm2
140a8b382: f3 0f 58 d1                 	addss	%xmm1, %xmm2
140a8b386: c7 44 24 30 01 00 00 00     	movl	$0x1, 0x30(%rsp)
140a8b38e: c7 44 24 40 cd cc 4c 3d     	movl	$0x3d4ccccd, 0x40(%rsp) # imm = 0x3D4CCCCD
140a8b396: f2 0f 10 1f                 	movsd	(%rdi), %xmm3
140a8b39a: 0f 14 c8                    	unpcklps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0],xmm1[1],xmm0[1]
140a8b39d: 0f 58 cb                    	addps	%xmm3, %xmm1
140a8b3a0: 0f 13 4c 24 44              	movlps	%xmm1, 0x44(%rsp)
140a8b3a5: f3 0f 11 54 24 4c           	movss	%xmm2, 0x4c(%rsp)
140a8b3ab: c7 44 24 50 00 00 00 40     	movl	$0x40000000, 0x50(%rsp) # imm = 0x40000000
140a8b3b3: 8b 44 24 24                 	movl	0x24(%rsp), %eax
140a8b3b7: 89 44 24 54                 	movl	%eax, 0x54(%rsp)
140a8b3bb: 48 89 e9                    	movq	%rbp, %rcx
140a8b3be: 4c 89 e2                    	movq	%r12, %rdx
140a8b3c1: e8 5a 71 07 00              	callq	0x140b02520 <.text+0xb01520>
140a8b3c6: 48 8b 84 24 88 00 00 00     	movq	0x88(%rsp), %rax
140a8b3ce: f3 42 0f 10 04 b8           	movss	(%rax,%r15,4), %xmm0
140a8b3d4: 0f 28 c8                    	movaps	%xmm0, %xmm1
140a8b3d7: f3 0f 59 cf                 	mulss	%xmm7, %xmm1
140a8b3db: f3 0f 10 57 08              	movss	0x8(%rdi), %xmm2
140a8b3e0: f3 0f 58 d1                 	addss	%xmm1, %xmm2
140a8b3e4: c7 44 24 30 01 00 00 00     	movl	$0x1, 0x30(%rsp)
140a8b3ec: c7 44 24 40 9a 99 19 3e     	movl	$0x3e19999a, 0x40(%rsp) # imm = 0x3E19999A
140a8b3f4: f2 0f 10 1f                 	movsd	(%rdi), %xmm3
140a8b3f8: 0f 14 c8                    	unpcklps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0],xmm1[1],xmm0[1]
140a8b3fb: 0f 58 cb                    	addps	%xmm3, %xmm1
140a8b3fe: 0f 13 4c 24 44              	movlps	%xmm1, 0x44(%rsp)
140a8b403: f3 0f 11 54 24 4c           	movss	%xmm2, 0x4c(%rsp)
140a8b409: c7 44 24 50 00 00 00 3f     	movl	$0x3f000000, 0x50(%rsp) # imm = 0x3F000000
140a8b411: 8b 44 24 2c                 	movl	0x2c(%rsp), %eax
140a8b415: 89 44 24 54                 	movl	%eax, 0x54(%rsp)
140a8b419: 48 89 e9                    	movq	%rbp, %rcx
140a8b41c: 4c 89 e2                    	movq	%r12, %rdx
140a8b41f: e8 fc 70 07 00              	callq	0x140b02520 <.text+0xb01520>
140a8b424: f3 0f 10 47 08              	movss	0x8(%rdi), %xmm0
140a8b429: 48 8b 84 24 90 00 00 00     	movq	0x90(%rsp), %rax
140a8b431: f3 42 0f 10 0c b8           	movss	(%rax,%r15,4), %xmm1
140a8b437: c7 44 24 30 02 00 00 00     	movl	$0x2, 0x30(%rsp)
140a8b43f: c7 44 24 40 9a 99 19 3e     	movl	$0x3e19999a, 0x40(%rsp) # imm = 0x3E19999A
140a8b447: f2 0f 10 17                 	movsd	(%rdi), %xmm2
140a8b44b: f3 0f 16 da                 	movshdup	%xmm2, %xmm3            # xmm3 = xmm2[1,1,3,3]
140a8b44f: f3 0f 5c cb                 	subss	%xmm3, %xmm1
140a8b453: 0f 28 d9                    	movaps	%xmm1, %xmm3
140a8b456: f3 0f 59 df                 	mulss	%xmm7, %xmm3
140a8b45a: f3 0f 58 c3                 	addss	%xmm3, %xmm0
140a8b45e: 0f 14 d9                    	unpcklps	%xmm1, %xmm3            # xmm3 = xmm3[0],xmm1[0],xmm3[1],xmm1[1]
140a8b461: 0f 58 da                    	addps	%xmm2, %xmm3
140a8b464: 0f 13 5c 24 44              	movlps	%xmm3, 0x44(%rsp)
140a8b469: f3 0f 11 44 24 4c           	movss	%xmm0, 0x4c(%rsp)
140a8b46f: c7 44 24 50 00 00 00 3f     	movl	$0x3f000000, 0x50(%rsp) # imm = 0x3F000000
140a8b477: 8b 44 24 5c                 	movl	0x5c(%rsp), %eax
140a8b47b: 89 44 24 54                 	movl	%eax, 0x54(%rsp)
140a8b47f: 48 89 e9                    	movq	%rbp, %rcx
140a8b482: 4c 89 e2                    	movq	%r12, %rdx
140a8b485: e8 96 70 07 00              	callq	0x140b02520 <.text+0xb01520>
140a8b48a: 48 83 c7 0c                 	addq	$0xc, %rdi
140a8b48e: 4d 89 f7                    	movq	%r14, %r15
140a8b491: 4c 39 f6                    	cmpq	%r14, %rsi
140a8b494: 0f 85 96 fe ff ff           	jne	0x140a8b330 <.text+0xa8a330>
140a8b49a: 48 8b 5c 24 78              	movq	0x78(%rsp), %rbx
140a8b49f: 4c 8b 74 24 70              	movq	0x70(%rsp), %r14
140a8b4a4: 4c 8b 7c 24 68              	movq	0x68(%rsp), %r15
140a8b4a9: 8b 6c 24 2c                 	movl	0x2c(%rsp), %ebp
140a8b4ad: 8b 4c 24 24                 	movl	0x24(%rsp), %ecx
140a8b4b1: 4c 8b 64 24 60              	movq	0x60(%rsp), %r12
140a8b4b6: 8b 44 24 28                 	movl	0x28(%rsp), %eax
140a8b4ba: 4d 85 f6                    	testq	%r14, %r14
140a8b4bd: 48 8d 54 24 30              	leaq	0x30(%rsp), %rdx
140a8b4c2: 0f 85 6b fc ff ff           	jne	0x140a8b133 <.text+0xa8a133>
140a8b4c8: eb 16                       	jmp	0x140a8b4e0 <.text+0xa8a4e0>
140a8b4ca: 8b 4c 24 24                 	movl	0x24(%rsp), %ecx
140a8b4ce: 8b 44 24 28                 	movl	0x28(%rsp), %eax
140a8b4d2: 4d 85 f6                    	testq	%r14, %r14
140a8b4d5: 48 8d 54 24 30              	leaq	0x30(%rsp), %rdx
140a8b4da: 0f 85 53 fc ff ff           	jne	0x140a8b133 <.text+0xa8a133>
140a8b4e0: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
140a8b4e8: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
140a8b4f0: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
140a8b4f7: 5b                          	popq	%rbx
140a8b4f8: 5d                          	popq	%rbp
140a8b4f9: 5f                          	popq	%rdi
140a8b4fa: 5e                          	popq	%rsi
140a8b4fb: 41 5c                       	popq	%r12
140a8b4fd: 41 5d                       	popq	%r13
140a8b4ff: 41 5e                       	popq	%r14
140a8b501: 41 5f                       	popq	%r15
140a8b503: c3                          	retq
140a8b504: 48 8d 0d 45 dd 04 02        	leaq	0x204dd45(%rip), %rcx   # 0x142ad9250
140a8b50b: e8 00 de e4 01              	callq	0x1428d9310 <.text+0x28d8310>
140a8b510: 48 8d 0d 7c ec 04 02        	leaq	0x204ec7c(%rip), %rcx   # 0x142ada193
140a8b517: 4c 8d 05 1a dd 04 02        	leaq	0x204dd1a(%rip), %r8    # 0x142ad9238
140a8b51e: ba 32 00 00 00              	movl	$0x32, %edx
140a8b523: e8 08 df e4 01              	callq	0x1428d9430 <.text+0x28d8430>
140a8b528: cc                          	int3
