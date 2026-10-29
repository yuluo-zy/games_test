
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1409566d0: 55                          	pushq	%rbp
1409566d1: 41 57                       	pushq	%r15
1409566d3: 41 56                       	pushq	%r14
1409566d5: 41 55                       	pushq	%r13
1409566d7: 41 54                       	pushq	%r12
1409566d9: 56                          	pushq	%rsi
1409566da: 57                          	pushq	%rdi
1409566db: 53                          	pushq	%rbx
1409566dc: 48 81 ec 68 01 00 00        	subq	$0x168, %rsp            # imm = 0x168
1409566e3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1409566eb: 0f 29 b5 d0 00 00 00        	movaps	%xmm6, 0xd0(%rbp)
1409566f2: 48 c7 85 c8 00 00 00 fe ff ff ff    	movq	$-0x2, 0xc8(%rbp)
1409566fd: 44 89 cb                    	movl	%r9d, %ebx
140956700: 48 89 d6                    	movq	%rdx, %rsi
140956703: 48 89 cf                    	movq	%rcx, %rdi
140956706: 4d 8b a8 e0 00 00 00        	movq	0xe0(%r8), %r13
14095670d: 49 8b 80 e8 00 00 00        	movq	0xe8(%r8), %rax
140956714: 80 79 09 00                 	cmpb	$0x0, 0x9(%rcx)
140956718: 0f 84 91 00 00 00           	je	0x1409567af <.text+0x9557af>
14095671e: 4d 8b b8 90 00 00 00        	movq	0x90(%r8), %r15
140956725: 4d 8b a0 98 00 00 00        	movq	0x98(%r8), %r12
14095672c: 4c 39 e0                    	cmpq	%r12, %rax
14095672f: 4c 0f 42 e0                 	cmovbq	%rax, %r12
140956733: 4a 8d 04 a5 00 00 00 00     	leaq	(,%r12,4), %rax
14095673b: 4c 8d 34 40                 	leaq	(%rax,%rax,2), %r14
14095673f: 48 b8 aa aa aa aa aa aa aa 0a       	movabsq	$0xaaaaaaaaaaaaaaa, %rax # imm = 0xAAAAAAAAAAAAAAA
140956749: 80 bd 50 01 00 00 00        	cmpb	$0x0, 0x150(%rbp)
140956750: 0f 84 d5 00 00 00           	je	0x14095682b <.text+0x95582b>
140956756: 49 39 c4                    	cmpq	%rax, %r12
140956759: 0f 87 f5 04 00 00           	ja	0x140956c54 <.text+0x955c54>
14095675f: 4d 85 e4                    	testq	%r12, %r12
140956762: 0f 84 25 01 00 00           	je	0x14095688d <.text+0x95588d>
140956768: 48 89 b5 b8 00 00 00        	movq	%rsi, 0xb8(%rbp)
14095676f: 4c 89 c6                    	movq	%r8, %rsi
140956772: 0f b6 05 90 e4 cd 02        	movzbl	0x2cde490(%rip), %eax   # 0x143634c09
140956779: b8 04 00 00 00              	movl	$0x4, %eax
14095677e: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
140956785: ba 04 00 00 00              	movl	$0x4, %edx
14095678a: 4c 89 f1                    	movq	%r14, %rcx
14095678d: e8 7e d4 cb ff              	callq	0x140613c10 <.text+0x612c10>
140956792: 48 85 c0                    	testq	%rax, %rax
140956795: 0f 84 c4 04 00 00           	je	0x140956c5f <.text+0x955c5f>
14095679b: 49 83 fc 01                 	cmpq	$0x1, %r12
14095679f: 0f 85 cb 01 00 00           	jne	0x140956970 <.text+0x955970>
1409567a5: 31 c9                       	xorl	%ecx, %ecx
1409567a7: 49 89 f0                    	movq	%rsi, %r8
1409567aa: e9 48 02 00 00              	jmp	0x1409569f7 <.text+0x9559f7>
1409567af: 4d 8b a0 98 00 00 00        	movq	0x98(%r8), %r12
1409567b6: 4c 39 e0                    	cmpq	%r12, %rax
1409567b9: 4c 0f 42 e0                 	cmovbq	%rax, %r12
1409567bd: 4a 8d 04 a5 00 00 00 00     	leaq	(,%r12,4), %rax
1409567c5: 4c 8d 34 40                 	leaq	(%rax,%rax,2), %r14
1409567c9: 48 b8 aa aa aa aa aa aa aa 0a       	movabsq	$0xaaaaaaaaaaaaaaa, %rax # imm = 0xAAAAAAAAAAAAAAA
1409567d3: 49 39 c4                    	cmpq	%rax, %r12
1409567d6: 0f 87 63 04 00 00           	ja	0x140956c3f <.text+0x955c3f>
1409567dc: f3 0f 10 35 8c f0 fc 01     	movss	0x1fcf08c(%rip), %xmm6  # 0x142925870
1409567e4: 4d 85 e4                    	testq	%r12, %r12
1409567e7: 0f 84 93 00 00 00           	je	0x140956880 <.text+0x955880>
1409567ed: 48 89 b5 b8 00 00 00        	movq	%rsi, 0xb8(%rbp)
1409567f4: 49 8b b0 90 00 00 00        	movq	0x90(%r8), %rsi
1409567fb: 0f b6 05 07 e4 cd 02        	movzbl	0x2cde407(%rip), %eax   # 0x143634c09
140956802: 41 bf 04 00 00 00           	movl	$0x4, %r15d
140956808: ba 04 00 00 00              	movl	$0x4, %edx
14095680d: 4c 89 f1                    	movq	%r14, %rcx
140956810: e8 fb d3 cb ff              	callq	0x140613c10 <.text+0x612c10>
140956815: 48 85 c0                    	testq	%rax, %rax
140956818: 0f 84 24 04 00 00           	je	0x140956c42 <.text+0x955c42>
14095681e: 49 83 fc 01                 	cmpq	$0x1, %r12
140956822: 75 76                       	jne	0x14095689a <.text+0x95589a>
140956824: 31 c9                       	xorl	%ecx, %ecx
140956826: e9 fc 00 00 00              	jmp	0x140956927 <.text+0x955927>
14095682b: 49 39 c4                    	cmpq	%rax, %r12
14095682e: 0f 87 20 04 00 00           	ja	0x140956c54 <.text+0x955c54>
140956834: 4d 85 e4                    	testq	%r12, %r12
140956837: 74 54                       	je	0x14095688d <.text+0x95588d>
140956839: 48 89 b5 b8 00 00 00        	movq	%rsi, 0xb8(%rbp)
140956840: 4c 89 c6                    	movq	%r8, %rsi
140956843: 0f b6 05 bf e3 cd 02        	movzbl	0x2cde3bf(%rip), %eax   # 0x143634c09
14095684a: b8 04 00 00 00              	movl	$0x4, %eax
14095684f: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
140956856: ba 04 00 00 00              	movl	$0x4, %edx
14095685b: 4c 89 f1                    	movq	%r14, %rcx
14095685e: e8 ad d3 cb ff              	callq	0x140613c10 <.text+0x612c10>
140956863: 48 85 c0                    	testq	%rax, %rax
140956866: 0f 84 f3 03 00 00           	je	0x140956c5f <.text+0x955c5f>
14095686c: 49 83 fc 01                 	cmpq	$0x1, %r12
140956870: 0f 85 b8 01 00 00           	jne	0x140956a2e <.text+0x955a2e>
140956876: 31 c9                       	xorl	%ecx, %ecx
140956878: 49 89 f0                    	movq	%rsi, %r8
14095687b: e9 37 02 00 00              	jmp	0x140956ab7 <.text+0x955ab7>
140956880: b8 04 00 00 00              	movl	$0x4, %eax
140956885: 45 31 e4                    	xorl	%r12d, %r12d
140956888: e9 84 02 00 00              	jmp	0x140956b11 <.text+0x955b11>
14095688d: b8 04 00 00 00              	movl	$0x4, %eax
140956892: 45 31 e4                    	xorl	%r12d, %r12d
140956895: e9 5e 02 00 00              	jmp	0x140956af8 <.text+0x955af8>
14095689a: 48 ba fe ff ff ff ff ff ff 0f       	movabsq	$0xffffffffffffffe, %rdx # imm = 0xFFFFFFFFFFFFFFE
1409568a4: 4c 21 e2                    	andq	%r12, %rdx
1409568a7: 41 b8 05 00 00 00           	movl	$0x5, %r8d
1409568ad: 31 c9                       	xorl	%ecx, %ecx
1409568af: f3 0f 10 05 51 f0 fc 01     	movss	0x1fcf051(%rip), %xmm0  # 0x142925908
1409568b7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1409568c0: f3 42 0f 10 4c 86 ec        	movss	-0x14(%rsi,%r8,4), %xmm1
1409568c7: f3 42 0f 10 54 86 f4        	movss	-0xc(%rsi,%r8,4), %xmm2
1409568ce: f3 41 0f 10 5c 8d 00        	movss	(%r13,%rcx,4), %xmm3
1409568d5: f3 0f 58 d8                 	addss	%xmm0, %xmm3
1409568d9: f3 42 0f 11 4c 80 ec        	movss	%xmm1, -0x14(%rax,%r8,4)
1409568e0: f3 42 0f 11 5c 80 f0        	movss	%xmm3, -0x10(%rax,%r8,4)
1409568e7: f3 42 0f 11 54 80 f4        	movss	%xmm2, -0xc(%rax,%r8,4)
1409568ee: f3 42 0f 10 4c 86 f8        	movss	-0x8(%rsi,%r8,4), %xmm1
1409568f5: f3 42 0f 10 14 86           	movss	(%rsi,%r8,4), %xmm2
1409568fb: f3 41 0f 10 5c 8d 04        	movss	0x4(%r13,%rcx,4), %xmm3
140956902: f3 0f 58 d8                 	addss	%xmm0, %xmm3
140956906: 48 83 c1 02                 	addq	$0x2, %rcx
14095690a: f3 42 0f 11 4c 80 f8        	movss	%xmm1, -0x8(%rax,%r8,4)
140956911: f3 42 0f 11 5c 80 fc        	movss	%xmm3, -0x4(%rax,%r8,4)
140956918: f3 42 0f 11 14 80           	movss	%xmm2, (%rax,%r8,4)
14095691e: 49 83 c0 06                 	addq	$0x6, %r8
140956922: 48 39 ca                    	cmpq	%rcx, %rdx
140956925: 75 99                       	jne	0x1409568c0 <.text+0x9558c0>
140956927: 41 f6 c4 01                 	testb	$0x1, %r12b
14095692b: 74 37                       	je	0x140956964 <.text+0x955964>
14095692d: 48 8d 14 8d 00 00 00 00     	leaq	(,%rcx,4), %rdx
140956935: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
140956939: f3 0f 10 04 16              	movss	(%rsi,%rdx), %xmm0
14095693e: f3 41 0f 10 4c 8d 00        	movss	(%r13,%rcx,4), %xmm1
140956945: f3 0f 58 0d bb ef fc 01     	addss	0x1fcefbb(%rip), %xmm1  # 0x142925908
14095694d: f3 0f 10 54 16 08           	movss	0x8(%rsi,%rdx), %xmm2
140956953: f3 0f 11 04 10              	movss	%xmm0, (%rax,%rdx)
140956958: f3 0f 11 4c 10 04           	movss	%xmm1, 0x4(%rax,%rdx)
14095695e: f3 0f 11 54 10 08           	movss	%xmm2, 0x8(%rax,%rdx)
140956964: 48 8b b5 b8 00 00 00        	movq	0xb8(%rbp), %rsi
14095696b: e9 a1 01 00 00              	jmp	0x140956b11 <.text+0x955b11>
140956970: 48 ba fe ff ff ff ff ff ff 0f       	movabsq	$0xffffffffffffffe, %rdx # imm = 0xFFFFFFFFFFFFFFE
14095697a: 4c 21 e2                    	andq	%r12, %rdx
14095697d: 41 b9 05 00 00 00           	movl	$0x5, %r9d
140956983: 31 c9                       	xorl	%ecx, %ecx
140956985: f3 0f 10 05 93 df 13 02     	movss	0x213df93(%rip), %xmm0  # 0x142a94920
14095698d: 49 89 f0                    	movq	%rsi, %r8
140956990: f3 43 0f 10 4c 8f ec        	movss	-0x14(%r15,%r9,4), %xmm1
140956997: f3 43 0f 10 54 8f f4        	movss	-0xc(%r15,%r9,4), %xmm2
14095699e: f3 41 0f 10 5c 8d 00        	movss	(%r13,%rcx,4), %xmm3
1409569a5: f3 0f 58 d8                 	addss	%xmm0, %xmm3
1409569a9: f3 42 0f 11 4c 88 ec        	movss	%xmm1, -0x14(%rax,%r9,4)
1409569b0: f3 42 0f 11 5c 88 f0        	movss	%xmm3, -0x10(%rax,%r9,4)
1409569b7: f3 42 0f 11 54 88 f4        	movss	%xmm2, -0xc(%rax,%r9,4)
1409569be: f3 43 0f 10 4c 8f f8        	movss	-0x8(%r15,%r9,4), %xmm1
1409569c5: f3 43 0f 10 14 8f           	movss	(%r15,%r9,4), %xmm2
1409569cb: f3 41 0f 10 5c 8d 04        	movss	0x4(%r13,%rcx,4), %xmm3
1409569d2: f3 0f 58 d8                 	addss	%xmm0, %xmm3
1409569d6: 48 83 c1 02                 	addq	$0x2, %rcx
1409569da: f3 42 0f 11 4c 88 f8        	movss	%xmm1, -0x8(%rax,%r9,4)
1409569e1: f3 42 0f 11 5c 88 fc        	movss	%xmm3, -0x4(%rax,%r9,4)
1409569e8: f3 42 0f 11 14 88           	movss	%xmm2, (%rax,%r9,4)
1409569ee: 49 83 c1 06                 	addq	$0x6, %r9
1409569f2: 48 39 ca                    	cmpq	%rcx, %rdx
1409569f5: 75 99                       	jne	0x140956990 <.text+0x955990>
1409569f7: 41 f6 c4 01                 	testb	$0x1, %r12b
1409569fb: 48 8b b5 b8 00 00 00        	movq	0xb8(%rbp), %rsi
140956a02: 0f 84 f0 00 00 00           	je	0x140956af8 <.text+0x955af8>
140956a08: 48 8d 14 8d 00 00 00 00     	leaq	(,%rcx,4), %rdx
140956a10: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
140956a14: f3 41 0f 10 04 17           	movss	(%r15,%rdx), %xmm0
140956a1a: f3 41 0f 10 4c 8d 00        	movss	(%r13,%rcx,4), %xmm1
140956a21: f3 0f 58 0d f7 de 13 02     	addss	0x213def7(%rip), %xmm1  # 0x142a94920
140956a29: e9 b2 00 00 00              	jmp	0x140956ae0 <.text+0x955ae0>
140956a2e: 48 ba fe ff ff ff ff ff ff 0f       	movabsq	$0xffffffffffffffe, %rdx # imm = 0xFFFFFFFFFFFFFFE
140956a38: 4c 21 e2                    	andq	%r12, %rdx
140956a3b: 41 b9 05 00 00 00           	movl	$0x5, %r9d
140956a41: 31 c9                       	xorl	%ecx, %ecx
140956a43: 0f 57 c0                    	xorps	%xmm0, %xmm0
140956a46: 49 89 f0                    	movq	%rsi, %r8
140956a49: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140956a50: f3 43 0f 10 4c 8f ec        	movss	-0x14(%r15,%r9,4), %xmm1
140956a57: f3 43 0f 10 54 8f f4        	movss	-0xc(%r15,%r9,4), %xmm2
140956a5e: f3 41 0f 10 5c 8d 00        	movss	(%r13,%rcx,4), %xmm3
140956a65: f3 0f 58 d8                 	addss	%xmm0, %xmm3
140956a69: f3 42 0f 11 4c 88 ec        	movss	%xmm1, -0x14(%rax,%r9,4)
140956a70: f3 42 0f 11 5c 88 f0        	movss	%xmm3, -0x10(%rax,%r9,4)
140956a77: f3 42 0f 11 54 88 f4        	movss	%xmm2, -0xc(%rax,%r9,4)
140956a7e: f3 43 0f 10 4c 8f f8        	movss	-0x8(%r15,%r9,4), %xmm1
140956a85: f3 43 0f 10 14 8f           	movss	(%r15,%r9,4), %xmm2
140956a8b: f3 41 0f 10 5c 8d 04        	movss	0x4(%r13,%rcx,4), %xmm3
140956a92: f3 0f 58 d8                 	addss	%xmm0, %xmm3
140956a96: 48 83 c1 02                 	addq	$0x2, %rcx
140956a9a: f3 42 0f 11 4c 88 f8        	movss	%xmm1, -0x8(%rax,%r9,4)
140956aa1: f3 42 0f 11 5c 88 fc        	movss	%xmm3, -0x4(%rax,%r9,4)
140956aa8: f3 42 0f 11 14 88           	movss	%xmm2, (%rax,%r9,4)
140956aae: 49 83 c1 06                 	addq	$0x6, %r9
140956ab2: 48 39 ca                    	cmpq	%rcx, %rdx
140956ab5: 75 99                       	jne	0x140956a50 <.text+0x955a50>
140956ab7: 41 f6 c4 01                 	testb	$0x1, %r12b
140956abb: 48 8b b5 b8 00 00 00        	movq	0xb8(%rbp), %rsi
140956ac2: 74 34                       	je	0x140956af8 <.text+0x955af8>
140956ac4: 48 8d 14 8d 00 00 00 00     	leaq	(,%rcx,4), %rdx
140956acc: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
140956ad0: f3 41 0f 10 04 17           	movss	(%r15,%rdx), %xmm0
140956ad6: 0f 57 c9                    	xorps	%xmm1, %xmm1
140956ad9: f3 41 0f 58 4c 8d 00        	addss	(%r13,%rcx,4), %xmm1
140956ae0: f3 41 0f 10 54 17 08        	movss	0x8(%r15,%rdx), %xmm2
140956ae7: f3 0f 11 04 10              	movss	%xmm0, (%rax,%rdx)
140956aec: f3 0f 11 4c 10 04           	movss	%xmm1, 0x4(%rax,%rdx)
140956af2: f3 0f 11 54 10 08           	movss	%xmm2, 0x8(%rax,%rdx)
140956af8: 31 c9                       	xorl	%ecx, %ecx
140956afa: 41 83 b8 58 02 00 00 02     	cmpl	$0x2, 0x258(%r8)
140956b02: 0f 92 c1                    	setb	%cl
140956b05: 48 8d 15 fc d0 15 02        	leaq	0x215d0fc(%rip), %rdx   # 0x142ab3c08
140956b0c: f3 0f 10 34 8a              	movss	(%rdx,%rcx,4), %xmm6
140956b11: c6 45 70 01                 	movb	$0x1, 0x70(%rbp)
140956b15: f3 0f 11 75 74              	movss	%xmm6, 0x74(%rbp)
140956b1a: 4c 89 65 78                 	movq	%r12, 0x78(%rbp)
140956b1e: 48 89 85 80 00 00 00        	movq	%rax, 0x80(%rbp)
140956b25: 4c 89 a5 88 00 00 00        	movq	%r12, 0x88(%rbp)
140956b2c: 48 89 bd 90 00 00 00        	movq	%rdi, 0x90(%rbp)
140956b33: 48 8d 05 86 e3 1d 00        	leaq	0x1de386(%rip), %rax    # 0x140b34ec0 <.text+0xb33ec0>
140956b3a: 48 89 85 98 00 00 00        	movq	%rax, 0x98(%rbp)
140956b41: 48 8d 05 60 dc 15 02        	leaq	0x215dc60(%rip), %rax   # 0x142ab47a8
140956b48: 48 89 45 a8                 	movq	%rax, -0x58(%rbp)
140956b4c: 48 c7 45 b0 01 00 00 00     	movq	$0x1, -0x50(%rbp)
140956b54: 48 8d 85 90 00 00 00        	leaq	0x90(%rbp), %rax
140956b5b: 48 89 45 b8                 	movq	%rax, -0x48(%rbp)
140956b5f: 48 c7 45 c0 01 00 00 00     	movq	$0x1, -0x40(%rbp)
140956b67: 48 c7 45 c8 00 00 00 00     	movq	$0x0, -0x38(%rbp)
140956b6f: c6 85 c7 00 00 00 01        	movb	$0x1, 0xc7(%rbp)
140956b76: 48 8d 4d 58                 	leaq	0x58(%rbp), %rcx
140956b7a: 48 8d 55 a8                 	leaq	-0x58(%rbp), %rdx
140956b7e: e8 0d 34 42 00              	callq	0x140d79f90 <.text+0xd78f90>
140956b83: 80 f3 01                    	xorb	$0x1, %bl
140956b86: 0f b6 c3                    	movzbl	%bl, %eax
140956b89: 8d 0c 40                    	leal	(%rax,%rax,2), %ecx
140956b8c: 8d 14 00                    	leal	(%rax,%rax), %edx
140956b8f: 0f 10 45 58                 	movups	0x58(%rbp), %xmm0
140956b93: 0f 29 85 90 00 00 00        	movaps	%xmm0, 0x90(%rbp)
140956b9a: 4c 8b 45 68                 	movq	0x68(%rbp), %r8
140956b9e: 4c 89 85 a0 00 00 00        	movq	%r8, 0xa0(%rbp)
140956ba5: 0f 10 45 70                 	movups	0x70(%rbp), %xmm0
140956ba9: 0f 10 8d 80 00 00 00        	movups	0x80(%rbp), %xmm1
140956bb0: 0f 11 4d 40                 	movups	%xmm1, 0x40(%rbp)
140956bb4: 0f 11 45 30                 	movups	%xmm0, 0x30(%rbp)
140956bb8: 44 0f b6 47 08              	movzbl	0x8(%rdi), %r8d
140956bbd: 41 80 f0 01                 	xorb	$0x1, %r8b
140956bc1: 48 c7 45 a8 00 00 00 00     	movq	$0x0, -0x58(%rbp)
140956bc9: 48 c7 45 b0 08 00 00 00     	movq	$0x8, -0x50(%rbp)
140956bd1: 48 c7 45 b8 00 00 00 00     	movq	$0x0, -0x48(%rbp)
140956bd9: 89 45 c0                    	movl	%eax, -0x40(%rbp)
140956bdc: 48 c7 45 c8 01 00 00 00     	movq	$0x1, -0x38(%rbp)
140956be4: 89 55 d8                    	movl	%edx, -0x28(%rbp)
140956be7: 89 4d f0                    	movl	%ecx, -0x10(%rbp)
140956bea: 89 4d 08                    	movl	%ecx, 0x8(%rbp)
140956bed: 66 c7 45 20 02 02           	movw	$0x202, 0x20(%rbp)      # imm = 0x202
140956bf3: 48 c7 45 24 00 00 00 00     	movq	$0x0, 0x24(%rbp)
140956bfb: c7 45 2c 00 00 00 00        	movl	$0x0, 0x2c(%rbp)
140956c02: 44 88 45 50                 	movb	%r8b, 0x50(%rbp)
140956c06: c6 45 51 01                 	movb	$0x1, 0x51(%rbp)
140956c0a: c6 85 c7 00 00 00 00        	movb	$0x0, 0xc7(%rbp)
140956c11: 48 8d 95 90 00 00 00        	leaq	0x90(%rbp), %rdx
140956c18: 4c 8d 45 a8                 	leaq	-0x58(%rbp), %r8
140956c1c: 48 89 f1                    	movq	%rsi, %rcx
140956c1f: e8 cc c5 fc ff              	callq	0x1409231f0 <.text+0x9221f0>
140956c24: 0f 28 b5 d0 00 00 00        	movaps	0xd0(%rbp), %xmm6
140956c2b: 48 81 c4 68 01 00 00        	addq	$0x168, %rsp            # imm = 0x168
140956c32: 5b                          	popq	%rbx
140956c33: 5f                          	popq	%rdi
140956c34: 5e                          	popq	%rsi
140956c35: 41 5c                       	popq	%r12
140956c37: 41 5d                       	popq	%r13
140956c39: 41 5e                       	popq	%r14
140956c3b: 41 5f                       	popq	%r15
140956c3d: 5d                          	popq	%rbp
140956c3e: c3                          	retq
140956c3f: 45 31 ff                    	xorl	%r15d, %r15d
140956c42: 4c 8d 05 cf d0 15 02        	leaq	0x215d0cf(%rip), %r8    # 0x142ab3d18
140956c49: 4c 89 f9                    	movq	%r15, %rcx
140956c4c: 4c 89 f2                    	movq	%r14, %rdx
140956c4f: e8 8f 23 f8 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140956c54: 48 c7 85 b0 00 00 00 00 00 00 00    	movq	$0x0, 0xb0(%rbp)
140956c5f: 4c 8d 05 b2 d0 15 02        	leaq	0x215d0b2(%rip), %r8    # 0x142ab3d18
140956c66: 48 8b 8d b0 00 00 00        	movq	0xb0(%rbp), %rcx
140956c6d: 4c 89 f2                    	movq	%r14, %rdx
140956c70: e8 6e 23 f8 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140956c75: cc                          	int3
140956c76: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140956c80: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140956c85: 55                          	pushq	%rbp
140956c86: 41 57                       	pushq	%r15
140956c88: 41 56                       	pushq	%r14
140956c8a: 41 55                       	pushq	%r13
140956c8c: 41 54                       	pushq	%r12
140956c8e: 56                          	pushq	%rsi
140956c8f: 57                          	pushq	%rdi
140956c90: 53                          	pushq	%rbx
140956c91: 48 83 ec 38                 	subq	$0x38, %rsp
140956c95: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140956c9c: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
140956ca1: 80 bd c7 00 00 00 00        	cmpb	$0x0, 0xc7(%rbp)
140956ca8: 75 16                       	jne	0x140956cc0 <.text+0x955cc0>
140956caa: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
140956caf: 48 83 c4 38                 	addq	$0x38, %rsp
140956cb3: 5b                          	popq	%rbx
140956cb4: 5f                          	popq	%rdi
140956cb5: 5e                          	popq	%rsi
140956cb6: 41 5c                       	popq	%r12
140956cb8: 41 5d                       	popq	%r13
140956cba: 41 5e                       	popq	%r14
140956cbc: 41 5f                       	popq	%r15
140956cbe: 5d                          	popq	%rbp
140956cbf: c3                          	retq
140956cc0: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
140956cc4: e8 07 89 ff ff              	callq	0x14094f5d0 <.text+0x94e5d0>
140956cc9: eb df                       	jmp	0x140956caa <.text+0x955caa>
140956ccb: cc                          	int3
140956ccc: cc                          	int3
140956ccd: cc                          	int3
140956cce: cc                          	int3
140956ccf: cc                          	int3
