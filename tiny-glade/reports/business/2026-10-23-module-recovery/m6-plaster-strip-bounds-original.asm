
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141228815: 48 8b 85 30 03 00 00        	movq	0x330(%rbp), %rax
14122881c: 49 c1 e1 05                 	shlq	$0x5, %r9
141228820: 49 01 c1                    	addq	%rax, %r9
141228823: 48 c1 e6 05                 	shlq	$0x5, %rsi
141228827: 48 01 c6                    	addq	%rax, %rsi
14122882a: eb 20                       	jmp	0x14122884c <.text+0x122784c>
14122882c: 0f 1f 40 00                 	nopl	(%rax)
141228830: 41 0f 13 08                 	movlps	%xmm1, (%r8)
141228834: f3 41 0f 11 40 08           	movss	%xmm0, 0x8(%r8)
14122883a: 0f 57 c0                    	xorps	%xmm0, %xmm0
14122883d: f3 0f 51 c3                 	sqrtss	%xmm3, %xmm0
141228841: f3 41 0f 11 40 0c           	movss	%xmm0, 0xc(%r8)
141228847: 4c 39 ce                    	cmpq	%r9, %rsi
14122884a: 74 ab                       	je	0x1412287f7 <.text+0x12277f7>
14122884c: 49 89 f0                    	movq	%rsi, %r8
14122884f: 48 83 c6 20                 	addq	$0x20, %rsi
141228853: 49 8b 40 10                 	movq	0x10(%r8), %rax
141228857: 48 2b 85 d0 02 00 00        	subq	0x2d0(%rbp), %rax
14122885e: 48 b9 ab aa aa aa aa aa aa aa       	movabsq	$-0x5555555555555555, %rcx # imm = 0xAAAAAAAAAAAAAAAB
141228868: 48 f7 e1                    	mulq	%rcx
14122886b: 48 c1 ea 05                 	shrq	$0x5, %rdx
14122886f: 45 8b 50 18                 	movl	0x18(%r8), %r10d
141228873: 0f 57 db                    	xorps	%xmm3, %xmm3
141228876: 45 31 f6                    	xorl	%r14d, %r14d
141228879: 0f 57 c9                    	xorps	%xmm1, %xmm1
14122887c: 0f 57 d2                    	xorps	%xmm2, %xmm2
14122887f: 31 ff                       	xorl	%edi, %edi
141228881: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
141228890: 45 39 d6                    	cmpl	%r10d, %r14d
141228893: 83 d7 00                    	adcl	$0x0, %edi
141228896: 41 8d 04 16                 	leal	(%r14,%rdx), %eax
14122889a: 48 63 c8                    	movslq	%eax, %rcx
14122889d: 49 39 cc                    	cmpq	%rcx, %r12
1412288a0: 0f 86 2f 08 00 00           	jbe	0x1412290d5 <.text+0x12280d5>
1412288a6: 45 8b 58 1c                 	movl	0x1c(%r8), %r11d
1412288aa: 44 01 d8                    	addl	%r11d, %eax
1412288ad: 48 98                       	cltq
1412288af: 49 39 c4                    	cmpq	%rax, %r12
1412288b2: 0f 86 ff 07 00 00           	jbe	0x1412290b7 <.text+0x12280b7>
1412288b8: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1412288bc: 48 c1 e1 04                 	shlq	$0x4, %rcx
1412288c0: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
1412288c4: 48 c1 e0 04                 	shlq	$0x4, %rax
1412288c8: f2 0f 10 04 0b              	movsd	(%rbx,%rcx), %xmm0
1412288cd: 0f 28 e3                    	movaps	%xmm3, %xmm4
1412288d0: 0f 58 e0                    	addps	%xmm0, %xmm4
1412288d3: f2 0f 10 1c 03              	movsd	(%rbx,%rax), %xmm3
1412288d8: 0f 58 dc                    	addps	%xmm4, %xmm3
1412288db: f3 0f 58 54 0b 08           	addss	0x8(%rbx,%rcx), %xmm2
1412288e1: f3 0f 58 54 03 08           	addss	0x8(%rbx,%rax), %xmm2
1412288e7: f3 41 0f 58 ca              	addss	%xmm10, %xmm1
1412288ec: 45 39 d6                    	cmpl	%r10d, %r14d
1412288ef: 73 08                       	jae	0x1412288f9 <.text+0x12278f9>
1412288f1: 41 89 fe                    	movl	%edi, %r14d
1412288f4: 44 39 d7                    	cmpl	%r10d, %edi
1412288f7: 76 97                       	jbe	0x141228890 <.text+0x1227890>
1412288f9: 41 0f 28 c4                 	movaps	%xmm12, %xmm0
1412288fd: f3 0f 5e c1                 	divss	%xmm1, %xmm0
141228901: f3 0f 12 c8                 	movsldup	%xmm0, %xmm1            # xmm1 = xmm0[0,0,2,2]
141228905: 0f 59 cb                    	mulps	%xmm3, %xmm1
141228908: f3 0f 59 c2                 	mulss	%xmm2, %xmm0
14122890c: 0f 28 d0                    	movaps	%xmm0, %xmm2
14122890f: 0f c6 d1 d4                 	shufps	$0xd4, %xmm1, %xmm2     # xmm2 = xmm2[0,1],xmm1[1,3]
141228913: 0f c6 d1 f2                 	shufps	$0xf2, %xmm1, %xmm2     # xmm2 = xmm2[2,0],xmm1[3,3]
141228917: 45 31 f6                    	xorl	%r14d, %r14d
14122891a: 0f 57 db                    	xorps	%xmm3, %xmm3
14122891d: 31 ff                       	xorl	%edi, %edi
14122891f: 90                          	nop
141228920: 45 39 d6                    	cmpl	%r10d, %r14d
141228923: 83 d7 00                    	adcl	$0x0, %edi
141228926: 41 8d 04 16                 	leal	(%r14,%rdx), %eax
14122892a: 48 63 c8                    	movslq	%eax, %rcx
14122892d: 49 39 cc                    	cmpq	%rcx, %r12
141228930: 0f 86 ba 07 00 00           	jbe	0x1412290f0 <.text+0x12280f0>
141228936: 44 01 d8                    	addl	%r11d, %eax
141228939: 48 98                       	cltq
14122893b: 49 39 c4                    	cmpq	%rax, %r12
14122893e: 0f 86 c7 07 00 00           	jbe	0x14122910b <.text+0x122810b>
141228944: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
141228948: 48 c1 e1 04                 	shlq	$0x4, %rcx
14122894c: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
141228950: 48 c1 e0 04                 	shlq	$0x4, %rax
141228954: 0f 28 e1                    	movaps	%xmm1, %xmm4
141228957: f3 0f 5c 24 0b              	subss	(%rbx,%rcx), %xmm4
14122895c: f3 0f 59 e4                 	mulss	%xmm4, %xmm4
141228960: f2 0f 10 6c 0b 04           	movsd	0x4(%rbx,%rcx), %xmm5
141228966: 0f 28 f2                    	movaps	%xmm2, %xmm6
141228969: 0f 5c f5                    	subps	%xmm5, %xmm6
14122896c: 0f 59 f6                    	mulps	%xmm6, %xmm6
14122896f: f3 0f 58 e6                 	addss	%xmm6, %xmm4
141228973: f3 0f 16 ee                 	movshdup	%xmm6, %xmm5            # xmm5 = xmm6[1,1,3,3]
141228977: f3 0f 58 ec                 	addss	%xmm4, %xmm5
14122897b: 0f 28 e3                    	movaps	%xmm3, %xmm4
14122897e: f3 0f c2 e3 03              	cmpunordss	%xmm3, %xmm4
141228983: 0f 28 f4                    	movaps	%xmm4, %xmm6
141228986: 0f 54 f5                    	andps	%xmm5, %xmm6
141228989: f3 0f 5f eb                 	maxss	%xmm3, %xmm5
14122898d: 0f 55 e5                    	andnps	%xmm5, %xmm4
141228990: 0f 56 e6                    	orps	%xmm6, %xmm4
141228993: 0f 28 e8                    	movaps	%xmm0, %xmm5
141228996: f3 0f 5c 6c 03 08           	subss	0x8(%rbx,%rax), %xmm5
14122899c: f2 0f 10 1c 03              	movsd	(%rbx,%rax), %xmm3
1412289a1: 0f 28 f1                    	movaps	%xmm1, %xmm6
1412289a4: 0f 5c f3                    	subps	%xmm3, %xmm6
1412289a7: 0f 59 f6                    	mulps	%xmm6, %xmm6
1412289aa: f3 0f 16 de                 	movshdup	%xmm6, %xmm3            # xmm3 = xmm6[1,1,3,3]
1412289ae: f3 0f 58 de                 	addss	%xmm6, %xmm3
1412289b2: f3 0f 59 ed                 	mulss	%xmm5, %xmm5
1412289b6: f3 0f 58 eb                 	addss	%xmm3, %xmm5
1412289ba: 0f 28 dc                    	movaps	%xmm4, %xmm3
1412289bd: f3 0f c2 dc 03              	cmpunordss	%xmm4, %xmm3
1412289c2: 0f 28 f3                    	movaps	%xmm3, %xmm6
1412289c5: 0f 54 f5                    	andps	%xmm5, %xmm6
1412289c8: f3 0f 5f ec                 	maxss	%xmm4, %xmm5
1412289cc: 0f 55 dd                    	andnps	%xmm5, %xmm3
1412289cf: 0f 56 de                    	orps	%xmm6, %xmm3
1412289d2: 45 39 d6                    	cmpl	%r10d, %r14d
1412289d5: 0f 83 55 fe ff ff           	jae	0x141228830 <.text+0x1227830>
1412289db: 41 89 fe                    	movl	%edi, %r14d
1412289de: 44 39 d7                    	cmpl	%r10d, %edi
1412289e1: 0f 86 39 ff ff ff           	jbe	0x141228920 <.text+0x1227920>
1412289e7: e9 44 fe ff ff              	jmp	0x141228830 <.text+0x1227830>
1412289ec: 48 c7 44 24 20 30 00 00 00  	movq	$0x30, 0x20(%rsp)
1412289f5: 41 b9 04 00 00 00           	movl	$0x4, %r9d
1412289fb: 48 8d 8d 10 03 00 00        	leaq	0x310(%rbp), %rcx
141228a02: 31 d2                       	xorl	%edx, %edx
141228a04: e8 d7 f3 67 01              	callq	0x1428a7de0 <.text+0x28a6de0>
141228a09: 48 8b bd 10 03 00 00        	movq	0x310(%rbp), %rdi
