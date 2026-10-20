
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140314dc0: 41 56                       	pushq	%r14
140314dc2: 56                          	pushq	%rsi
140314dc3: 57                          	pushq	%rdi
140314dc4: 53                          	pushq	%rbx
140314dc5: 48 83 ec 48                 	subq	$0x48, %rsp
140314dc9: 48 89 ce                    	movq	%rcx, %rsi
140314dcc: 48 8b 01                    	movq	(%rcx), %rax
140314dcf: 48 b9 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rcx # imm = 0x8000000000000001
140314dd9: 48 01 c1                    	addq	%rax, %rcx
140314ddc: 45 31 c0                    	xorl	%r8d, %r8d
140314ddf: 49 b9 10 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffff0, %r9 # imm = 0x8000000000000010
140314de9: 4c 39 c8                    	cmpq	%r9, %rax
140314dec: 4c 0f 4c c1                 	cmovlq	%rcx, %r8
140314df0: 48 89 d7                    	movq	%rdx, %rdi
140314df3: 48 8d 05 02 65 6a 02        	leaq	0x26a6502(%rip), %rax   # 0x1429bb2fc
140314dfa: 4a 63 0c 80                 	movslq	(%rax,%r8,4), %rcx
140314dfe: 48 01 c1                    	addq	%rax, %rcx
140314e01: ff e1                       	jmpq	*%rcx
140314e03: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
140314e07: 48 8b 1f                    	movq	(%rdi), %rbx
140314e0a: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140314e0e: 48 39 03                    	cmpq	%rax, (%rbx)
140314e11: 0f 84 70 11 00 00           	je	0x140315f87 <.text+0x314f87>
140314e17: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140314e1b: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140314e1f: 48 ff c0                    	incq	%rax
140314e22: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140314e26: 4c 8d 05 a2 62 6a 02        	leaq	0x26a62a2(%rip), %r8    # 0x1429bb0cf
140314e2d: 41 b9 0b 00 00 00           	movl	$0xb, %r9d
140314e33: 48 89 f9                    	movq	%rdi, %rcx
140314e36: e8 c5 12 1a 00              	callq	0x1404b6100 <.text+0x4b5100>
140314e3b: 48 85 c0                    	testq	%rax, %rax
140314e3e: 0f 85 0e 0f 00 00           	jne	0x140315d52 <.text+0x314d52>
140314e44: 48 8b 1f                    	movq	(%rdi), %rbx
140314e47: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140314e4b: 48 39 13                    	cmpq	%rdx, (%rbx)
140314e4e: 0f 84 dc 14 00 00           	je	0x140316330 <.text+0x315330>
140314e54: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140314e58: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
140314e5c: 48 ff c2                    	incq	%rdx
140314e5f: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140314e63: 48 8b 1f                    	movq	(%rdi), %rbx
140314e66: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140314e6a: 48 39 13                    	cmpq	%rdx, (%rbx)
140314e6d: 0f 84 e3 14 00 00           	je	0x140316356 <.text+0x315356>
140314e73: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140314e77: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
140314e7b: 48 ff c2                    	incq	%rdx
140314e7e: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140314e82: 4c 8d 8e e0 00 00 00        	leaq	0xe0(%rsi), %r9
140314e89: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
140314e8e: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
140314e93: 48 8d 15 40 62 6a 02        	leaq	0x26a6240(%rip), %rdx   # 0x1429bb0da
140314e9a: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140314e9f: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
140314ea5: e8 f6 c2 1a 00              	callq	0x1404c11a0 <.text+0x4c01a0>
140314eaa: 48 85 c0                    	testq	%rax, %rax
140314ead: 0f 85 a7 0e 00 00           	jne	0x140315d5a <.text+0x314d5a>
140314eb3: 4c 8d 8e e8 00 00 00        	leaq	0xe8(%rsi), %r9
140314eba: 48 8d 15 23 62 6a 02        	leaq	0x26a6223(%rip), %rdx   # 0x1429bb0e4
140314ec1: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140314ec6: 41 b8 0c 00 00 00           	movl	$0xc, %r8d
140314ecc: e8 6f 6a 1a 00              	callq	0x1404bb940 <.text+0x4ba940>
140314ed1: 48 85 c0                    	testq	%rax, %rax
140314ed4: 0f 85 80 0e 00 00           	jne	0x140315d5a <.text+0x314d5a>
140314eda: 48 8d 15 0f 62 6a 02        	leaq	0x26a620f(%rip), %rdx   # 0x1429bb0f0
140314ee1: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140314ee6: 41 b8 06 00 00 00           	movl	$0x6, %r8d
140314eec: 49 89 f1                    	movq	%rsi, %r9
140314eef: e8 fc 3a 1a 00              	callq	0x1404b89f0 <.text+0x4b79f0>
140314ef4: 48 85 c0                    	testq	%rax, %rax
140314ef7: 0f 85 5d 0e 00 00           	jne	0x140315d5a <.text+0x314d5a>
140314efd: 48 83 c6 70                 	addq	$0x70, %rsi
140314f01: 48 8d 15 ee 61 6a 02        	leaq	0x26a61ee(%rip), %rdx   # 0x1429bb0f6
140314f08: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140314f0d: 41 b8 05 00 00 00           	movl	$0x5, %r8d
140314f13: 49 89 f1                    	movq	%rsi, %r9
140314f16: e8 d5 3a 1a 00              	callq	0x1404b89f0 <.text+0x4b79f0>
140314f1b: e9 17 0e 00 00              	jmp	0x140315d37 <.text+0x314d37>
140314f20: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
140314f24: 48 8b 1f                    	movq	(%rdi), %rbx
140314f27: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140314f2b: 48 39 03                    	cmpq	%rax, (%rbx)
140314f2e: 0f 84 7d 0e 00 00           	je	0x140315db1 <.text+0x314db1>
140314f34: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140314f38: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140314f3c: 48 ff c0                    	incq	%rax
140314f3f: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140314f43: 4c 8d 05 08 63 6a 02        	leaq	0x26a6308(%rip), %r8    # 0x1429bb252
140314f4a: 41 b9 0b 00 00 00           	movl	$0xb, %r9d
140314f50: 48 89 f9                    	movq	%rdi, %rcx
140314f53: e8 a8 11 1a 00              	callq	0x1404b6100 <.text+0x4b5100>
140314f58: 48 85 c0                    	testq	%rax, %rax
140314f5b: 0f 85 f1 0d 00 00           	jne	0x140315d52 <.text+0x314d52>
140314f61: 48 8b 1f                    	movq	(%rdi), %rbx
140314f64: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140314f68: 48 39 13                    	cmpq	%rdx, (%rbx)
140314f6b: 0f 84 5f 11 00 00           	je	0x1403160d0 <.text+0x3150d0>
140314f71: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140314f75: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
140314f79: 48 ff c2                    	incq	%rdx
140314f7c: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140314f80: 48 8b 1f                    	movq	(%rdi), %rbx
140314f83: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140314f87: 48 39 13                    	cmpq	%rdx, (%rbx)
140314f8a: 0f 84 66 11 00 00           	je	0x1403160f6 <.text+0x3150f6>
140314f90: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140314f94: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
140314f98: 48 ff c2                    	incq	%rdx
140314f9b: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140314f9f: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
140314fa3: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
140314fa8: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
140314fad: 48 8d 15 26 61 6a 02        	leaq	0x26a6126(%rip), %rdx   # 0x1429bb0da
140314fb4: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140314fb9: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
140314fbf: e8 dc c1 1a 00              	callq	0x1404c11a0 <.text+0x4c01a0>
140314fc4: 48 85 c0                    	testq	%rax, %rax
140314fc7: 0f 85 8d 0d 00 00           	jne	0x140315d5a <.text+0x314d5a>
140314fcd: 4c 8d 4e 10                 	leaq	0x10(%rsi), %r9
140314fd1: 48 8d 15 18 61 6a 02        	leaq	0x26a6118(%rip), %rdx   # 0x1429bb0f0
140314fd8: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140314fdd: 41 b8 06 00 00 00           	movl	$0x6, %r8d
140314fe3: e8 a8 73 1a 00              	callq	0x1404bc390 <.text+0x4bb390>
140314fe8: 48 85 c0                    	testq	%rax, %rax
140314feb: 0f 85 69 0d 00 00           	jne	0x140315d5a <.text+0x314d5a>
140314ff1: 48 83 c6 11                 	addq	$0x11, %rsi
140314ff5: 48 8d 15 fa 60 6a 02        	leaq	0x26a60fa(%rip), %rdx   # 0x1429bb0f6
140314ffc: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315001: 41 b8 05 00 00 00           	movl	$0x5, %r8d
140315007: e9 b8 06 00 00              	jmp	0x1403156c4 <.text+0x3146c4>
14031500c: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
140315010: 48 8b 1f                    	movq	(%rdi), %rbx
140315013: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315017: 48 39 03                    	cmpq	%rax, (%rbx)
14031501a: 0f 84 c0 0d 00 00           	je	0x140315de0 <.text+0x314de0>
140315020: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140315024: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140315028: 48 ff c0                    	incq	%rax
14031502b: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
14031502f: 4c 8d 05 da 61 6a 02        	leaq	0x26a61da(%rip), %r8    # 0x1429bb210
140315036: 41 b9 0f 00 00 00           	movl	$0xf, %r9d
14031503c: 48 89 f9                    	movq	%rdi, %rcx
14031503f: e8 bc 10 1a 00              	callq	0x1404b6100 <.text+0x4b5100>
140315044: 48 85 c0                    	testq	%rax, %rax
140315047: 0f 85 05 0d 00 00           	jne	0x140315d52 <.text+0x314d52>
14031504d: 48 8b 1f                    	movq	(%rdi), %rbx
140315050: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315054: 48 39 13                    	cmpq	%rdx, (%rbx)
140315057: 0f 84 bf 10 00 00           	je	0x14031611c <.text+0x31511c>
14031505d: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140315061: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
140315065: 48 ff c2                    	incq	%rdx
140315068: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
14031506c: 48 8b 1f                    	movq	(%rdi), %rbx
14031506f: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315073: 48 39 13                    	cmpq	%rdx, (%rbx)
140315076: 0f 84 c6 10 00 00           	je	0x140316142 <.text+0x315142>
14031507c: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140315080: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
140315084: 48 ff c2                    	incq	%rdx
140315087: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
14031508b: 4c 8d 4e 78                 	leaq	0x78(%rsi), %r9
14031508f: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
140315094: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
140315099: 48 8d 15 3a 60 6a 02        	leaq	0x26a603a(%rip), %rdx   # 0x1429bb0da
1403150a0: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1403150a5: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
1403150ab: e8 f0 c0 1a 00              	callq	0x1404c11a0 <.text+0x4c01a0>
1403150b0: 48 85 c0                    	testq	%rax, %rax
1403150b3: 0f 85 a1 0c 00 00           	jne	0x140315d5a <.text+0x314d5a>
1403150b9: 48 8d 46 08                 	leaq	0x8(%rsi), %rax
1403150bd: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
1403150c2: 48 8d 15 56 61 6a 02        	leaq	0x26a6156(%rip), %rdx   # 0x1429bb21f
1403150c9: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1403150ce: 4c 8d 4c 24 40              	leaq	0x40(%rsp), %r9
1403150d3: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1403150d9: e8 f2 2b 1a 00              	callq	0x1404b7cd0 <.text+0x4b6cd0>
1403150de: 48 85 c0                    	testq	%rax, %rax
1403150e1: 0f 85 73 0c 00 00           	jne	0x140315d5a <.text+0x314d5a>
1403150e7: 48 83 c6 40                 	addq	$0x40, %rsi
1403150eb: 48 89 74 24 40              	movq	%rsi, 0x40(%rsp)
1403150f0: 48 8d 15 30 61 6a 02        	leaq	0x26a6130(%rip), %rdx   # 0x1429bb227
1403150f7: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1403150fc: 4c 8d 4c 24 40              	leaq	0x40(%rsp), %r9
140315101: 41 b8 06 00 00 00           	movl	$0x6, %r8d
140315107: e8 c4 2b 1a 00              	callq	0x1404b7cd0 <.text+0x4b6cd0>
14031510c: e9 26 0c 00 00              	jmp	0x140315d37 <.text+0x314d37>
140315111: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
140315115: 48 8b 1f                    	movq	(%rdi), %rbx
140315118: 48 8b 43 10                 	movq	0x10(%rbx), %rax
14031511c: 48 39 03                    	cmpq	%rax, (%rbx)
14031511f: 0f 84 ea 0c 00 00           	je	0x140315e0f <.text+0x314e0f>
140315125: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140315129: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
14031512d: 48 ff c0                    	incq	%rax
140315130: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140315134: 4c 8d 05 71 60 6a 02        	leaq	0x26a6071(%rip), %r8    # 0x1429bb1ac
14031513b: 41 b9 18 00 00 00           	movl	$0x18, %r9d
140315141: 48 89 f9                    	movq	%rdi, %rcx
140315144: e8 b7 0f 1a 00              	callq	0x1404b6100 <.text+0x4b5100>
140315149: 48 85 c0                    	testq	%rax, %rax
14031514c: 0f 85 00 0c 00 00           	jne	0x140315d52 <.text+0x314d52>
140315152: 48 8b 1f                    	movq	(%rdi), %rbx
140315155: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315159: 48 39 13                    	cmpq	%rdx, (%rbx)
14031515c: 0f 84 06 10 00 00           	je	0x140316168 <.text+0x315168>
140315162: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140315166: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
14031516a: 48 ff c2                    	incq	%rdx
14031516d: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140315171: 48 8b 1f                    	movq	(%rdi), %rbx
140315174: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315178: 48 39 13                    	cmpq	%rdx, (%rbx)
14031517b: 0f 84 0d 10 00 00           	je	0x14031618e <.text+0x31518e>
140315181: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140315185: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
140315189: 48 ff c2                    	incq	%rdx
14031518c: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140315190: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
140315194: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
140315199: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
14031519e: 48 8d 15 35 5f 6a 02        	leaq	0x26a5f35(%rip), %rdx   # 0x1429bb0da
1403151a5: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1403151aa: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
1403151b0: e8 eb bf 1a 00              	callq	0x1404c11a0 <.text+0x4c01a0>
1403151b5: 48 85 c0                    	testq	%rax, %rax
1403151b8: 0f 85 9c 0b 00 00           	jne	0x140315d5a <.text+0x314d5a>
1403151be: 4c 8d 4e 10                 	leaq	0x10(%rsi), %r9
1403151c2: 48 8d 15 27 5f 6a 02        	leaq	0x26a5f27(%rip), %rdx   # 0x1429bb0f0
1403151c9: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1403151ce: 41 b8 06 00 00 00           	movl	$0x6, %r8d
1403151d4: e8 47 4a 1a 00              	callq	0x1404b9c20 <.text+0x4b8c20>
1403151d9: 48 85 c0                    	testq	%rax, %rax
1403151dc: 0f 85 78 0b 00 00           	jne	0x140315d5a <.text+0x314d5a>
1403151e2: 48 83 c6 11                 	addq	$0x11, %rsi
1403151e6: 48 8d 15 09 5f 6a 02        	leaq	0x26a5f09(%rip), %rdx   # 0x1429bb0f6
1403151ed: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1403151f2: 41 b8 05 00 00 00           	movl	$0x5, %r8d
1403151f8: 49 89 f1                    	movq	%rsi, %r9
1403151fb: e8 20 4a 1a 00              	callq	0x1404b9c20 <.text+0x4b8c20>
140315200: e9 32 0b 00 00              	jmp	0x140315d37 <.text+0x314d37>
140315205: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
140315209: 48 8b 1f                    	movq	(%rdi), %rbx
14031520c: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315210: 48 39 03                    	cmpq	%rax, (%rbx)
140315213: 0f 84 25 0c 00 00           	je	0x140315e3e <.text+0x314e3e>
140315219: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
14031521d: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140315221: 48 ff c0                    	incq	%rax
140315224: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140315228: 4c 8d 05 08 5f 6a 02        	leaq	0x26a5f08(%rip), %r8    # 0x1429bb137
14031522f: 41 b9 15 00 00 00           	movl	$0x15, %r9d
140315235: 48 89 f9                    	movq	%rdi, %rcx
140315238: e8 c3 0e 1a 00              	callq	0x1404b6100 <.text+0x4b5100>
14031523d: 48 85 c0                    	testq	%rax, %rax
140315240: 0f 85 0c 0b 00 00           	jne	0x140315d52 <.text+0x314d52>
140315246: 48 8b 1f                    	movq	(%rdi), %rbx
140315249: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14031524d: 48 39 13                    	cmpq	%rdx, (%rbx)
140315250: 0f 84 5e 0f 00 00           	je	0x1403161b4 <.text+0x3151b4>
140315256: 48 8b 43 08                 	movq	0x8(%rbx), %rax
14031525a: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
14031525e: 48 ff c2                    	incq	%rdx
140315261: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140315265: 48 8b 1f                    	movq	(%rdi), %rbx
140315268: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14031526c: 48 39 13                    	cmpq	%rdx, (%rbx)
14031526f: 0f 84 65 0f 00 00           	je	0x1403161da <.text+0x3151da>
140315275: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140315279: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
14031527d: 48 ff c2                    	incq	%rdx
140315280: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140315284: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
140315288: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
14031528d: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
140315292: 48 8d 15 41 5e 6a 02        	leaq	0x26a5e41(%rip), %rdx   # 0x1429bb0da
140315299: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
14031529e: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
1403152a4: e8 f7 be 1a 00              	callq	0x1404c11a0 <.text+0x4c01a0>
1403152a9: 48 85 c0                    	testq	%rax, %rax
1403152ac: 0f 85 a8 0a 00 00           	jne	0x140315d5a <.text+0x314d5a>
1403152b2: 4c 8d 4e 10                 	leaq	0x10(%rsi), %r9
1403152b6: 48 8d 15 33 5e 6a 02        	leaq	0x26a5e33(%rip), %rdx   # 0x1429bb0f0
1403152bd: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1403152c2: 41 b8 06 00 00 00           	movl	$0x6, %r8d
1403152c8: e8 53 78 1a 00              	callq	0x1404bcb20 <.text+0x4bbb20>
1403152cd: 48 85 c0                    	testq	%rax, %rax
1403152d0: 0f 85 84 0a 00 00           	jne	0x140315d5a <.text+0x314d5a>
1403152d6: 48 83 c6 2c                 	addq	$0x2c, %rsi
1403152da: 48 8d 15 15 5e 6a 02        	leaq	0x26a5e15(%rip), %rdx   # 0x1429bb0f6
1403152e1: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1403152e6: 41 b8 05 00 00 00           	movl	$0x5, %r8d
1403152ec: 49 89 f1                    	movq	%rsi, %r9
1403152ef: e8 2c 78 1a 00              	callq	0x1404bcb20 <.text+0x4bbb20>
1403152f4: e9 3e 0a 00 00              	jmp	0x140315d37 <.text+0x314d37>
1403152f9: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
1403152fd: 48 8b 1f                    	movq	(%rdi), %rbx
140315300: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315304: 48 39 03                    	cmpq	%rax, (%rbx)
140315307: 0f 84 60 0b 00 00           	je	0x140315e6d <.text+0x314e6d>
14031530d: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140315311: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140315315: 48 ff c0                    	incq	%rax
140315318: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
14031531c: 4c 8d 05 a1 5e 6a 02        	leaq	0x26a5ea1(%rip), %r8    # 0x1429bb1c4
140315323: 41 b9 0e 00 00 00           	movl	$0xe, %r9d
140315329: 48 89 f9                    	movq	%rdi, %rcx
14031532c: e8 cf 0d 1a 00              	callq	0x1404b6100 <.text+0x4b5100>
140315331: 48 85 c0                    	testq	%rax, %rax
140315334: 0f 85 18 0a 00 00           	jne	0x140315d52 <.text+0x314d52>
14031533a: 48 8b 1f                    	movq	(%rdi), %rbx
14031533d: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315341: 48 39 13                    	cmpq	%rdx, (%rbx)
140315344: 0f 84 b6 0e 00 00           	je	0x140316200 <.text+0x315200>
14031534a: 48 8b 43 08                 	movq	0x8(%rbx), %rax
14031534e: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
140315352: 48 ff c2                    	incq	%rdx
140315355: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140315359: 48 8b 1f                    	movq	(%rdi), %rbx
14031535c: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315360: 48 39 13                    	cmpq	%rdx, (%rbx)
140315363: 0f 84 bd 0e 00 00           	je	0x140316226 <.text+0x315226>
140315369: 48 8b 43 08                 	movq	0x8(%rbx), %rax
14031536d: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
140315371: 48 ff c2                    	incq	%rdx
140315374: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140315378: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
14031537c: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
140315381: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
140315386: 48 8d 15 4d 5d 6a 02        	leaq	0x26a5d4d(%rip), %rdx   # 0x1429bb0da
14031538d: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315392: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
140315398: e8 03 be 1a 00              	callq	0x1404c11a0 <.text+0x4c01a0>
14031539d: 48 85 c0                    	testq	%rax, %rax
1403153a0: 0f 85 b4 09 00 00           	jne	0x140315d5a <.text+0x314d5a>
1403153a6: 4c 8d 4e 10                 	leaq	0x10(%rsi), %r9
1403153aa: 48 8d 15 21 5e 6a 02        	leaq	0x26a5e21(%rip), %rdx   # 0x1429bb1d2
1403153b1: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1403153b6: 41 b8 17 00 00 00           	movl	$0x17, %r8d
1403153bc: e8 cf 6f 1a 00              	callq	0x1404bc390 <.text+0x4bb390>
1403153c1: 48 85 c0                    	testq	%rax, %rax
1403153c4: 0f 85 90 09 00 00           	jne	0x140315d5a <.text+0x314d5a>
1403153ca: 48 83 c6 11                 	addq	$0x11, %rsi
1403153ce: 48 8d 15 14 5e 6a 02        	leaq	0x26a5e14(%rip), %rdx   # 0x1429bb1e9
1403153d5: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1403153da: 41 b8 16 00 00 00           	movl	$0x16, %r8d
1403153e0: e9 df 02 00 00              	jmp	0x1403156c4 <.text+0x3146c4>
1403153e5: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
1403153e9: 48 8b 1f                    	movq	(%rdi), %rbx
1403153ec: 48 8b 43 10                 	movq	0x10(%rbx), %rax
1403153f0: 48 39 03                    	cmpq	%rax, (%rbx)
1403153f3: 0f 84 a3 0a 00 00           	je	0x140315e9c <.text+0x314e9c>
1403153f9: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
1403153fd: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140315401: 48 ff c0                    	incq	%rax
140315404: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140315408: 4c 8d 05 7b 5d 6a 02        	leaq	0x26a5d7b(%rip), %r8    # 0x1429bb18a
14031540f: 41 b9 0b 00 00 00           	movl	$0xb, %r9d
140315415: 48 89 f9                    	movq	%rdi, %rcx
140315418: e8 e3 0c 1a 00              	callq	0x1404b6100 <.text+0x4b5100>
14031541d: 48 85 c0                    	testq	%rax, %rax
140315420: 0f 85 2c 09 00 00           	jne	0x140315d52 <.text+0x314d52>
140315426: 48 8b 1f                    	movq	(%rdi), %rbx
140315429: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14031542d: 48 39 13                    	cmpq	%rdx, (%rbx)
140315430: 0f 84 16 0e 00 00           	je	0x14031624c <.text+0x31524c>
140315436: 48 8b 43 08                 	movq	0x8(%rbx), %rax
14031543a: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
14031543e: 48 ff c2                    	incq	%rdx
140315441: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140315445: 48 8b 1f                    	movq	(%rdi), %rbx
140315448: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14031544c: 48 39 13                    	cmpq	%rdx, (%rbx)
14031544f: 0f 84 1d 0e 00 00           	je	0x140316272 <.text+0x315272>
140315455: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140315459: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
14031545d: 48 ff c2                    	incq	%rdx
140315460: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140315464: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
140315468: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
14031546d: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
140315472: 48 8d 15 61 5c 6a 02        	leaq	0x26a5c61(%rip), %rdx   # 0x1429bb0da
140315479: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
14031547e: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
140315484: e8 17 bd 1a 00              	callq	0x1404c11a0 <.text+0x4c01a0>
140315489: 48 85 c0                    	testq	%rax, %rax
14031548c: 0f 85 c8 08 00 00           	jne	0x140315d5a <.text+0x314d5a>
140315492: 4c 8d 4e 10                 	leaq	0x10(%rsi), %r9
140315496: 48 8d 15 53 5c 6a 02        	leaq	0x26a5c53(%rip), %rdx   # 0x1429bb0f0
14031549d: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1403154a2: 41 b8 06 00 00 00           	movl	$0x6, %r8d
1403154a8: e8 53 5f 1a 00              	callq	0x1404bb400 <.text+0x4ba400>
1403154ad: 48 85 c0                    	testq	%rax, %rax
1403154b0: 0f 85 a4 08 00 00           	jne	0x140315d5a <.text+0x314d5a>
1403154b6: 48 83 c6 11                 	addq	$0x11, %rsi
1403154ba: 48 8d 15 35 5c 6a 02        	leaq	0x26a5c35(%rip), %rdx   # 0x1429bb0f6
1403154c1: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1403154c6: 41 b8 05 00 00 00           	movl	$0x5, %r8d
1403154cc: 49 89 f1                    	movq	%rsi, %r9
1403154cf: e8 2c 5f 1a 00              	callq	0x1404bb400 <.text+0x4ba400>
1403154d4: e9 5e 08 00 00              	jmp	0x140315d37 <.text+0x314d37>
1403154d9: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
1403154dd: 48 8b 1f                    	movq	(%rdi), %rbx
1403154e0: 48 8b 43 10                 	movq	0x10(%rbx), %rax
1403154e4: 48 39 03                    	cmpq	%rax, (%rbx)
1403154e7: 0f 84 de 09 00 00           	je	0x140315ecb <.text+0x314ecb>
1403154ed: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
1403154f1: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
1403154f5: 48 ff c0                    	incq	%rax
1403154f8: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
1403154fc: 4c 8d 05 1e 5c 6a 02        	leaq	0x26a5c1e(%rip), %r8    # 0x1429bb121
140315503: 41 b9 0a 00 00 00           	movl	$0xa, %r9d
140315509: 48 89 f9                    	movq	%rdi, %rcx
14031550c: e8 ef 0b 1a 00              	callq	0x1404b6100 <.text+0x4b5100>
140315511: 48 85 c0                    	testq	%rax, %rax
140315514: 0f 85 38 08 00 00           	jne	0x140315d52 <.text+0x314d52>
14031551a: 48 8b 1f                    	movq	(%rdi), %rbx
14031551d: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315521: 48 39 13                    	cmpq	%rdx, (%rbx)
140315524: 0f 84 6e 0d 00 00           	je	0x140316298 <.text+0x315298>
14031552a: 48 8b 43 08                 	movq	0x8(%rbx), %rax
14031552e: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
140315532: 48 ff c2                    	incq	%rdx
140315535: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140315539: 48 8b 1f                    	movq	(%rdi), %rbx
14031553c: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315540: 48 39 13                    	cmpq	%rdx, (%rbx)
140315543: 0f 84 75 0d 00 00           	je	0x1403162be <.text+0x3152be>
140315549: 48 8b 43 08                 	movq	0x8(%rbx), %rax
14031554d: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
140315551: 48 ff c2                    	incq	%rdx
140315554: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140315558: 4c 8d 4e 78                 	leaq	0x78(%rsi), %r9
14031555c: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
140315561: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
140315566: 48 8d 15 6d 5b 6a 02        	leaq	0x26a5b6d(%rip), %rdx   # 0x1429bb0da
14031556d: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315572: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
140315578: e8 23 bc 1a 00              	callq	0x1404c11a0 <.text+0x4c01a0>
14031557d: 48 85 c0                    	testq	%rax, %rax
140315580: 0f 85 d4 07 00 00           	jne	0x140315d5a <.text+0x314d5a>
140315586: 4c 8d 8e 80 00 00 00        	leaq	0x80(%rsi), %r9
14031558d: 48 8d 15 71 5b 6a 02        	leaq	0x26a5b71(%rip), %rdx   # 0x1429bb105
140315594: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315599: 41 b8 0b 00 00 00           	movl	$0xb, %r8d
14031559f: e8 ac 7c 1a 00              	callq	0x1404bd250 <.text+0x4bc250>
1403155a4: 48 85 c0                    	testq	%rax, %rax
1403155a7: 0f 85 ad 07 00 00           	jne	0x140315d5a <.text+0x314d5a>
1403155ad: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
1403155b1: 48 8d 15 38 5b 6a 02        	leaq	0x26a5b38(%rip), %rdx   # 0x1429bb0f0
1403155b8: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1403155bd: 41 b8 06 00 00 00           	movl	$0x6, %r8d
1403155c3: e8 e8 a1 1a 00              	callq	0x1404bf7b0 <.text+0x4be7b0>
1403155c8: 48 85 c0                    	testq	%rax, %rax
1403155cb: 0f 85 89 07 00 00           	jne	0x140315d5a <.text+0x314d5a>
1403155d1: 48 81 c6 84 00 00 00        	addq	$0x84, %rsi
1403155d8: e9 84 02 00 00              	jmp	0x140315861 <.text+0x314861>
1403155dd: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
1403155e1: 48 8b 1f                    	movq	(%rdi), %rbx
1403155e4: 48 8b 43 10                 	movq	0x10(%rbx), %rax
1403155e8: 48 39 03                    	cmpq	%rax, (%rbx)
1403155eb: 0f 84 09 09 00 00           	je	0x140315efa <.text+0x314efa>
1403155f1: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
1403155f5: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
1403155f9: 48 ff c0                    	incq	%rax
1403155fc: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140315600: 4c 8d 05 26 5c 6a 02        	leaq	0x26a5c26(%rip), %r8    # 0x1429bb22d
140315607: 41 b9 0d 00 00 00           	movl	$0xd, %r9d
14031560d: 48 89 f9                    	movq	%rdi, %rcx
140315610: e8 eb 0a 1a 00              	callq	0x1404b6100 <.text+0x4b5100>
140315615: 48 85 c0                    	testq	%rax, %rax
140315618: 0f 85 34 07 00 00           	jne	0x140315d52 <.text+0x314d52>
14031561e: 48 8b 1f                    	movq	(%rdi), %rbx
140315621: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315625: 48 39 13                    	cmpq	%rdx, (%rbx)
140315628: 0f 84 b6 0c 00 00           	je	0x1403162e4 <.text+0x3152e4>
14031562e: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140315632: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
140315636: 48 ff c2                    	incq	%rdx
140315639: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
14031563d: 48 8b 1f                    	movq	(%rdi), %rbx
140315640: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315644: 48 39 13                    	cmpq	%rdx, (%rbx)
140315647: 0f 84 bd 0c 00 00           	je	0x14031630a <.text+0x31530a>
14031564d: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140315651: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
140315655: 48 ff c2                    	incq	%rdx
140315658: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
14031565c: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
140315660: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
140315665: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
14031566a: 48 8d 15 69 5a 6a 02        	leaq	0x26a5a69(%rip), %rdx   # 0x1429bb0da
140315671: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315676: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
14031567c: e8 1f bb 1a 00              	callq	0x1404c11a0 <.text+0x4c01a0>
140315681: 48 85 c0                    	testq	%rax, %rax
140315684: 0f 85 d0 06 00 00           	jne	0x140315d5a <.text+0x314d5a>
14031568a: 4c 8d 4e 10                 	leaq	0x10(%rsi), %r9
14031568e: 48 8d 15 a5 5b 6a 02        	leaq	0x26a5ba5(%rip), %rdx   # 0x1429bb23a
140315695: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
14031569a: 41 b8 10 00 00 00           	movl	$0x10, %r8d
1403156a0: e8 4b 3a 1a 00              	callq	0x1404b90f0 <.text+0x4b80f0>
1403156a5: 48 85 c0                    	testq	%rax, %rax
1403156a8: 0f 85 ac 06 00 00           	jne	0x140315d5a <.text+0x314d5a>
1403156ae: 48 83 c6 2c                 	addq	$0x2c, %rsi
1403156b2: 48 8d 15 91 5b 6a 02        	leaq	0x26a5b91(%rip), %rdx   # 0x1429bb24a
1403156b9: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1403156be: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1403156c4: 49 89 f1                    	movq	%rsi, %r9
1403156c7: e8 c4 6c 1a 00              	callq	0x1404bc390 <.text+0x4bb390>
1403156cc: e9 66 06 00 00              	jmp	0x140315d37 <.text+0x314d37>
1403156d1: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
1403156d5: 48 8b 1f                    	movq	(%rdi), %rbx
1403156d8: 48 8b 43 10                 	movq	0x10(%rbx), %rax
1403156dc: 48 39 03                    	cmpq	%rax, (%rbx)
1403156df: 0f 84 44 08 00 00           	je	0x140315f29 <.text+0x314f29>
1403156e5: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
1403156e9: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
1403156ed: 48 ff c0                    	incq	%rax
1403156f0: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
1403156f4: 4c 8d 05 30 5a 6a 02        	leaq	0x26a5a30(%rip), %r8    # 0x1429bb12b
1403156fb: 41 b9 0c 00 00 00           	movl	$0xc, %r9d
140315701: e9 92 03 00 00              	jmp	0x140315a98 <.text+0x314a98>
140315706: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
14031570a: 48 8b 1f                    	movq	(%rdi), %rbx
14031570d: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315711: 48 39 03                    	cmpq	%rax, (%rbx)
140315714: 0f 84 3e 08 00 00           	je	0x140315f58 <.text+0x314f58>
14031571a: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
14031571e: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140315722: 48 ff c0                    	incq	%rax
140315725: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140315729: 4c 8d 05 3c 5a 6a 02        	leaq	0x26a5a3c(%rip), %r8    # 0x1429bb16c
140315730: 41 b9 10 00 00 00           	movl	$0x10, %r9d
140315736: e9 5d 03 00 00              	jmp	0x140315a98 <.text+0x314a98>
14031573b: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
14031573f: 48 8b 1f                    	movq	(%rdi), %rbx
140315742: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315746: 48 39 03                    	cmpq	%rax, (%rbx)
140315749: 0f 84 67 08 00 00           	je	0x140315fb6 <.text+0x314fb6>
14031574f: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140315753: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140315757: 48 ff c0                    	incq	%rax
14031575a: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
14031575e: 4c 8d 05 96 59 6a 02        	leaq	0x26a5996(%rip), %r8    # 0x1429bb0fb
140315765: 41 b9 0a 00 00 00           	movl	$0xa, %r9d
14031576b: 48 89 f9                    	movq	%rdi, %rcx
14031576e: e8 8d 09 1a 00              	callq	0x1404b6100 <.text+0x4b5100>
140315773: 48 85 c0                    	testq	%rax, %rax
140315776: 0f 85 d6 05 00 00           	jne	0x140315d52 <.text+0x314d52>
14031577c: 48 8b 1f                    	movq	(%rdi), %rbx
14031577f: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315783: 48 39 13                    	cmpq	%rdx, (%rbx)
140315786: 0f 84 f0 0b 00 00           	je	0x14031637c <.text+0x31537c>
14031578c: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140315790: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
140315794: 48 ff c2                    	incq	%rdx
140315797: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
14031579b: 48 8b 1f                    	movq	(%rdi), %rbx
14031579e: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403157a2: 48 39 13                    	cmpq	%rdx, (%rbx)
1403157a5: 0f 84 f7 0b 00 00           	je	0x1403163a2 <.text+0x3153a2>
1403157ab: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1403157af: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
1403157b3: 48 ff c2                    	incq	%rdx
1403157b6: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1403157ba: 4c 8d 4e 78                 	leaq	0x78(%rsi), %r9
1403157be: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
1403157c3: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
1403157c8: 48 8d 15 0b 59 6a 02        	leaq	0x26a590b(%rip), %rdx   # 0x1429bb0da
1403157cf: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1403157d4: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
1403157da: e8 c1 b9 1a 00              	callq	0x1404c11a0 <.text+0x4c01a0>
1403157df: 48 85 c0                    	testq	%rax, %rax
1403157e2: 0f 85 72 05 00 00           	jne	0x140315d5a <.text+0x314d5a>
1403157e8: 4c 8d 8e 80 00 00 00        	leaq	0x80(%rsi), %r9
1403157ef: 48 8d 15 0f 59 6a 02        	leaq	0x26a590f(%rip), %rdx   # 0x1429bb105
1403157f6: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1403157fb: 41 b8 0b 00 00 00           	movl	$0xb, %r8d
140315801: e8 4a 7a 1a 00              	callq	0x1404bd250 <.text+0x4bc250>
140315806: 48 85 c0                    	testq	%rax, %rax
140315809: 0f 85 4b 05 00 00           	jne	0x140315d5a <.text+0x314d5a>
14031580f: 4c 8d 8e 84 00 00 00        	leaq	0x84(%rsi), %r9
140315816: 48 8d 15 f3 58 6a 02        	leaq	0x26a58f3(%rip), %rdx   # 0x1429bb110
14031581d: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315822: 41 b8 09 00 00 00           	movl	$0x9, %r8d
140315828: e8 53 84 1a 00              	callq	0x1404bdc80 <.text+0x4bcc80>
14031582d: 48 85 c0                    	testq	%rax, %rax
140315830: 0f 85 24 05 00 00           	jne	0x140315d5a <.text+0x314d5a>
140315836: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
14031583a: 48 8d 15 b5 58 6a 02        	leaq	0x26a58b5(%rip), %rdx   # 0x1429bb0f6
140315841: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315846: 41 b8 05 00 00 00           	movl	$0x5, %r8d
14031584c: e8 5f 9f 1a 00              	callq	0x1404bf7b0 <.text+0x4be7b0>
140315851: 48 85 c0                    	testq	%rax, %rax
140315854: 0f 85 00 05 00 00           	jne	0x140315d5a <.text+0x314d5a>
14031585a: 48 81 c6 88 00 00 00        	addq	$0x88, %rsi
140315861: 48 8d 15 b1 58 6a 02        	leaq	0x26a58b1(%rip), %rdx   # 0x1429bb119
140315868: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
14031586d: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140315873: 49 89 f1                    	movq	%rsi, %r9
140315876: e8 35 52 1a 00              	callq	0x1404baab0 <.text+0x4b9ab0>
14031587b: e9 b7 04 00 00              	jmp	0x140315d37 <.text+0x314d37>
140315880: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
140315884: 48 8b 1f                    	movq	(%rdi), %rbx
140315887: 48 8b 43 10                 	movq	0x10(%rbx), %rax
14031588b: 48 39 03                    	cmpq	%rax, (%rbx)
14031588e: 0f 84 51 07 00 00           	je	0x140315fe5 <.text+0x314fe5>
140315894: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140315898: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
14031589c: 48 ff c0                    	incq	%rax
14031589f: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
1403158a3: 4c 8d 05 a2 58 6a 02        	leaq	0x26a58a2(%rip), %r8    # 0x1429bb14c
1403158aa: 41 b9 11 00 00 00           	movl	$0x11, %r9d
1403158b0: 48 89 f9                    	movq	%rdi, %rcx
1403158b3: e8 48 08 1a 00              	callq	0x1404b6100 <.text+0x4b5100>
1403158b8: 48 85 c0                    	testq	%rax, %rax
1403158bb: 0f 85 91 04 00 00           	jne	0x140315d52 <.text+0x314d52>
1403158c1: 48 8b 1f                    	movq	(%rdi), %rbx
1403158c4: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403158c8: 48 39 13                    	cmpq	%rdx, (%rbx)
1403158cb: 0f 84 f7 0a 00 00           	je	0x1403163c8 <.text+0x3153c8>
1403158d1: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1403158d5: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
1403158d9: 48 ff c2                    	incq	%rdx
1403158dc: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1403158e0: 48 8b 1f                    	movq	(%rdi), %rbx
1403158e3: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403158e7: 48 39 13                    	cmpq	%rdx, (%rbx)
1403158ea: 0f 84 fe 0a 00 00           	je	0x1403163ee <.text+0x3153ee>
1403158f0: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1403158f4: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
1403158f8: 48 ff c2                    	incq	%rdx
1403158fb: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1403158ff: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
140315903: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
140315908: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
14031590d: 48 8d 15 c6 57 6a 02        	leaq	0x26a57c6(%rip), %rdx   # 0x1429bb0da
140315914: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315919: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
14031591f: e8 7c b8 1a 00              	callq	0x1404c11a0 <.text+0x4c01a0>
140315924: 48 85 c0                    	testq	%rax, %rax
140315927: 0f 85 2d 04 00 00           	jne	0x140315d5a <.text+0x314d5a>
14031592d: 4c 8d 4e 10                 	leaq	0x10(%rsi), %r9
140315931: 48 8d 15 b8 57 6a 02        	leaq	0x26a57b8(%rip), %rdx   # 0x1429bb0f0
140315938: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
14031593d: 41 b8 06 00 00 00           	movl	$0x6, %r8d
140315943: e8 a8 37 1a 00              	callq	0x1404b90f0 <.text+0x4b80f0>
140315948: 48 85 c0                    	testq	%rax, %rax
14031594b: 0f 85 09 04 00 00           	jne	0x140315d5a <.text+0x314d5a>
140315951: 48 83 c6 2c                 	addq	$0x2c, %rsi
140315955: 48 8d 15 9a 57 6a 02        	leaq	0x26a579a(%rip), %rdx   # 0x1429bb0f6
14031595c: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315961: 41 b8 05 00 00 00           	movl	$0x5, %r8d
140315967: 49 89 f1                    	movq	%rsi, %r9
14031596a: e8 81 37 1a 00              	callq	0x1404b90f0 <.text+0x4b80f0>
14031596f: e9 c3 03 00 00              	jmp	0x140315d37 <.text+0x314d37>
140315974: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
140315978: 48 8b 1f                    	movq	(%rdi), %rbx
14031597b: 48 8b 43 10                 	movq	0x10(%rbx), %rax
14031597f: 48 39 03                    	cmpq	%rax, (%rbx)
140315982: 0f 84 8c 06 00 00           	je	0x140316014 <.text+0x315014>
140315988: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
14031598c: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140315990: 48 ff c0                    	incq	%rax
140315993: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140315997: 4c 8d 05 61 58 6a 02        	leaq	0x26a5861(%rip), %r8    # 0x1429bb1ff
14031599e: 41 b9 11 00 00 00           	movl	$0x11, %r9d
1403159a4: 48 89 f9                    	movq	%rdi, %rcx
1403159a7: e8 54 07 1a 00              	callq	0x1404b6100 <.text+0x4b5100>
1403159ac: 48 85 c0                    	testq	%rax, %rax
1403159af: 0f 85 9d 03 00 00           	jne	0x140315d52 <.text+0x314d52>
1403159b5: 48 8b 1f                    	movq	(%rdi), %rbx
1403159b8: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403159bc: 48 39 13                    	cmpq	%rdx, (%rbx)
1403159bf: 0f 84 4f 0a 00 00           	je	0x140316414 <.text+0x315414>
1403159c5: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1403159c9: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
1403159cd: 48 ff c2                    	incq	%rdx
1403159d0: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1403159d4: 48 8b 1f                    	movq	(%rdi), %rbx
1403159d7: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403159db: 48 39 13                    	cmpq	%rdx, (%rbx)
1403159de: 0f 84 56 0a 00 00           	je	0x14031643a <.text+0x31543a>
1403159e4: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1403159e8: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
1403159ec: 48 ff c2                    	incq	%rdx
1403159ef: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1403159f3: 4c 8d 4e 78                 	leaq	0x78(%rsi), %r9
1403159f7: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
1403159fc: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
140315a01: 48 8d 15 d2 56 6a 02        	leaq	0x26a56d2(%rip), %rdx   # 0x1429bb0da
140315a08: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315a0d: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
140315a13: e8 88 b7 1a 00              	callq	0x1404c11a0 <.text+0x4c01a0>
140315a18: 48 85 c0                    	testq	%rax, %rax
140315a1b: 0f 85 39 03 00 00           	jne	0x140315d5a <.text+0x314d5a>
140315a21: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
140315a25: 48 8d 15 c4 56 6a 02        	leaq	0x26a56c4(%rip), %rdx   # 0x1429bb0f0
140315a2c: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315a31: 41 b8 06 00 00 00           	movl	$0x6, %r8d
140315a37: e8 94 9c 1a 00              	callq	0x1404bf6d0 <.text+0x4be6d0>
140315a3c: 48 85 c0                    	testq	%rax, %rax
140315a3f: 0f 85 15 03 00 00           	jne	0x140315d5a <.text+0x314d5a>
140315a45: 48 83 c6 40                 	addq	$0x40, %rsi
140315a49: 48 8d 15 a6 56 6a 02        	leaq	0x26a56a6(%rip), %rdx   # 0x1429bb0f6
140315a50: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315a55: 41 b8 05 00 00 00           	movl	$0x5, %r8d
140315a5b: 49 89 f1                    	movq	%rsi, %r9
140315a5e: e8 6d 9c 1a 00              	callq	0x1404bf6d0 <.text+0x4be6d0>
140315a63: e9 cf 02 00 00              	jmp	0x140315d37 <.text+0x314d37>
140315a68: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
140315a6c: 48 8b 1f                    	movq	(%rdi), %rbx
140315a6f: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315a73: 48 39 03                    	cmpq	%rax, (%rbx)
140315a76: 0f 84 c7 05 00 00           	je	0x140316043 <.text+0x315043>
140315a7c: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140315a80: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140315a84: 48 ff c0                    	incq	%rax
140315a87: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140315a8b: 4c 8d 05 cb 56 6a 02        	leaq	0x26a56cb(%rip), %r8    # 0x1429bb15d
140315a92: 41 b9 0f 00 00 00           	movl	$0xf, %r9d
140315a98: 48 89 f9                    	movq	%rdi, %rcx
140315a9b: e8 60 06 1a 00              	callq	0x1404b6100 <.text+0x4b5100>
140315aa0: 48 85 c0                    	testq	%rax, %rax
140315aa3: 0f 85 a9 02 00 00           	jne	0x140315d52 <.text+0x314d52>
140315aa9: 48 8b 1f                    	movq	(%rdi), %rbx
140315aac: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315ab0: 48 39 13                    	cmpq	%rdx, (%rbx)
140315ab3: 0f 84 ac 02 00 00           	je	0x140315d65 <.text+0x314d65>
140315ab9: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140315abd: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
140315ac1: 48 ff c2                    	incq	%rdx
140315ac4: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140315ac8: 48 8b 1f                    	movq	(%rdi), %rbx
140315acb: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315acf: 48 39 13                    	cmpq	%rdx, (%rbx)
140315ad2: 0f 84 b3 02 00 00           	je	0x140315d8b <.text+0x314d8b>
140315ad8: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140315adc: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
140315ae0: 48 ff c2                    	incq	%rdx
140315ae3: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140315ae7: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
140315aeb: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
140315af0: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
140315af5: 48 8d 15 de 55 6a 02        	leaq	0x26a55de(%rip), %rdx   # 0x1429bb0da
140315afc: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315b01: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
140315b07: e8 94 b6 1a 00              	callq	0x1404c11a0 <.text+0x4c01a0>
140315b0c: 48 85 c0                    	testq	%rax, %rax
140315b0f: 0f 85 45 02 00 00           	jne	0x140315d5a <.text+0x314d5a>
140315b15: 4c 8d 4e 10                 	leaq	0x10(%rsi), %r9
140315b19: 48 8d 15 d0 55 6a 02        	leaq	0x26a55d0(%rip), %rdx   # 0x1429bb0f0
140315b20: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315b25: 41 b8 06 00 00 00           	movl	$0x6, %r8d
140315b2b: e8 50 81 1a 00              	callq	0x1404bdc80 <.text+0x4bcc80>
140315b30: 48 85 c0                    	testq	%rax, %rax
140315b33: 0f 85 21 02 00 00           	jne	0x140315d5a <.text+0x314d5a>
140315b39: 48 83 c6 14                 	addq	$0x14, %rsi
140315b3d: 48 8d 15 b2 55 6a 02        	leaq	0x26a55b2(%rip), %rdx   # 0x1429bb0f6
140315b44: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315b49: 41 b8 05 00 00 00           	movl	$0x5, %r8d
140315b4f: 49 89 f1                    	movq	%rsi, %r9
140315b52: e8 29 81 1a 00              	callq	0x1404bdc80 <.text+0x4bcc80>
140315b57: e9 db 01 00 00              	jmp	0x140315d37 <.text+0x314d37>
140315b5c: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
140315b60: 48 8b 1f                    	movq	(%rdi), %rbx
140315b63: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315b67: 48 39 03                    	cmpq	%rax, (%rbx)
140315b6a: 0f 84 02 05 00 00           	je	0x140316072 <.text+0x315072>
140315b70: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140315b74: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140315b78: 48 ff c0                    	incq	%rax
140315b7b: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140315b7f: 4c 8d 05 0f 56 6a 02        	leaq	0x26a560f(%rip), %r8    # 0x1429bb195
140315b86: 41 b9 17 00 00 00           	movl	$0x17, %r9d
140315b8c: 48 89 f9                    	movq	%rdi, %rcx
140315b8f: e8 6c 05 1a 00              	callq	0x1404b6100 <.text+0x4b5100>
140315b94: 48 85 c0                    	testq	%rax, %rax
140315b97: 0f 85 b5 01 00 00           	jne	0x140315d52 <.text+0x314d52>
140315b9d: 48 8b 1f                    	movq	(%rdi), %rbx
140315ba0: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315ba4: 48 39 13                    	cmpq	%rdx, (%rbx)
140315ba7: 0f 84 b3 08 00 00           	je	0x140316460 <.text+0x315460>
140315bad: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140315bb1: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
140315bb5: 48 ff c2                    	incq	%rdx
140315bb8: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140315bbc: 48 8b 1f                    	movq	(%rdi), %rbx
140315bbf: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315bc3: 48 39 13                    	cmpq	%rdx, (%rbx)
140315bc6: 0f 84 ba 08 00 00           	je	0x140316486 <.text+0x315486>
140315bcc: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140315bd0: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
140315bd4: 48 ff c2                    	incq	%rdx
140315bd7: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140315bdb: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
140315bdf: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
140315be4: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
140315be9: 48 8d 15 ea 54 6a 02        	leaq	0x26a54ea(%rip), %rdx   # 0x1429bb0da
140315bf0: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315bf5: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
140315bfb: e8 a0 b5 1a 00              	callq	0x1404c11a0 <.text+0x4c01a0>
140315c00: 48 85 c0                    	testq	%rax, %rax
140315c03: 0f 85 51 01 00 00           	jne	0x140315d5a <.text+0x314d5a>
140315c09: 4c 8d 4e 10                 	leaq	0x10(%rsi), %r9
140315c0d: 48 8d 15 dc 54 6a 02        	leaq	0x26a54dc(%rip), %rdx   # 0x1429bb0f0
140315c14: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315c19: 41 b8 06 00 00 00           	movl	$0x6, %r8d
140315c1f: e8 2c 76 1a 00              	callq	0x1404bd250 <.text+0x4bc250>
140315c24: 48 85 c0                    	testq	%rax, %rax
140315c27: 0f 85 2d 01 00 00           	jne	0x140315d5a <.text+0x314d5a>
140315c2d: 48 83 c6 14                 	addq	$0x14, %rsi
140315c31: 48 8d 15 be 54 6a 02        	leaq	0x26a54be(%rip), %rdx   # 0x1429bb0f6
140315c38: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315c3d: 41 b8 05 00 00 00           	movl	$0x5, %r8d
140315c43: 49 89 f1                    	movq	%rsi, %r9
140315c46: e8 05 76 1a 00              	callq	0x1404bd250 <.text+0x4bc250>
140315c4b: e9 e7 00 00 00              	jmp	0x140315d37 <.text+0x314d37>
140315c50: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
140315c54: 48 8b 1f                    	movq	(%rdi), %rbx
140315c57: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315c5b: 48 39 03                    	cmpq	%rax, (%rbx)
140315c5e: 0f 84 3d 04 00 00           	je	0x1403160a1 <.text+0x3150a1>
140315c64: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140315c68: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140315c6c: 48 ff c0                    	incq	%rax
140315c6f: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140315c73: 4c 8d 05 02 55 6a 02        	leaq	0x26a5502(%rip), %r8    # 0x1429bb17c
140315c7a: 41 b9 0e 00 00 00           	movl	$0xe, %r9d
140315c80: 48 89 f9                    	movq	%rdi, %rcx
140315c83: e8 78 04 1a 00              	callq	0x1404b6100 <.text+0x4b5100>
140315c88: 48 85 c0                    	testq	%rax, %rax
140315c8b: 0f 85 c1 00 00 00           	jne	0x140315d52 <.text+0x314d52>
140315c91: 48 8b 1f                    	movq	(%rdi), %rbx
140315c94: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315c98: 48 39 13                    	cmpq	%rdx, (%rbx)
140315c9b: 0f 84 0b 08 00 00           	je	0x1403164ac <.text+0x3154ac>
140315ca1: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140315ca5: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
140315ca9: 48 ff c2                    	incq	%rdx
140315cac: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140315cb0: 48 8b 1f                    	movq	(%rdi), %rbx
140315cb3: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315cb7: 48 39 13                    	cmpq	%rdx, (%rbx)
140315cba: 0f 84 12 08 00 00           	je	0x1403164d2 <.text+0x3154d2>
140315cc0: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140315cc4: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
140315cc8: 48 ff c2                    	incq	%rdx
140315ccb: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140315ccf: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
140315cd3: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
140315cd8: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
140315cdd: 48 8d 15 f6 53 6a 02        	leaq	0x26a53f6(%rip), %rdx   # 0x1429bb0da
140315ce4: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315ce9: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
140315cef: e8 ac b4 1a 00              	callq	0x1404c11a0 <.text+0x4c01a0>
140315cf4: 48 85 c0                    	testq	%rax, %rax
140315cf7: 75 61                       	jne	0x140315d5a <.text+0x314d5a>
140315cf9: 4c 8d 4e 10                 	leaq	0x10(%rsi), %r9
140315cfd: 48 8d 15 ec 53 6a 02        	leaq	0x26a53ec(%rip), %rdx   # 0x1429bb0f0
140315d04: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315d09: 41 b8 06 00 00 00           	movl	$0x6, %r8d
140315d0f: e8 0c 3f 1a 00              	callq	0x1404b9c20 <.text+0x4b8c20>
140315d14: 48 85 c0                    	testq	%rax, %rax
140315d17: 75 41                       	jne	0x140315d5a <.text+0x314d5a>
140315d19: 48 83 c6 11                 	addq	$0x11, %rsi
140315d1d: 48 8d 15 d2 53 6a 02        	leaq	0x26a53d2(%rip), %rdx   # 0x1429bb0f6
140315d24: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140315d29: 41 b8 05 00 00 00           	movl	$0x5, %r8d
140315d2f: 49 89 f1                    	movq	%rsi, %r9
140315d32: e8 e9 3e 1a 00              	callq	0x1404b9c20 <.text+0x4b8c20>
140315d37: 48 85 c0                    	testq	%rax, %rax
140315d3a: 75 1e                       	jne	0x140315d5a <.text+0x314d5a>
140315d3c: 48 8b 44 24 30              	movq	0x30(%rsp), %rax
140315d41: 0f b6 54 24 38              	movzbl	0x38(%rsp), %edx
140315d46: 48 8b 08                    	movq	(%rax), %rcx
140315d49: e8 52 83 d2 ff              	callq	0x14003e0a0 <.text+0x3d0a0>
140315d4e: 31 c0                       	xorl	%eax, %eax
140315d50: eb 08                       	jmp	0x140315d5a <.text+0x314d5a>
140315d52: 48 89 c1                    	movq	%rax, %rcx
140315d55: e8 46 3b 5b 02              	callq	0x1428c98a0 <.text+0x28c88a0>
140315d5a: 90                          	nop
140315d5b: 48 83 c4 48                 	addq	$0x48, %rsp
140315d5f: 5b                          	popq	%rbx
140315d60: 5f                          	popq	%rdi
140315d61: 5e                          	popq	%rsi
140315d62: 41 5e                       	popq	%r14
140315d64: c3                          	retq
140315d65: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140315d6e: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140315d74: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140315d7a: 48 89 d9                    	movq	%rbx, %rcx
140315d7d: e8 5e cc 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140315d82: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315d86: e9 2e fd ff ff              	jmp	0x140315ab9 <.text+0x314ab9>
140315d8b: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140315d94: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140315d9a: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140315da0: 48 89 d9                    	movq	%rbx, %rcx
140315da3: e8 38 cc 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140315da8: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140315dac: e9 27 fd ff ff              	jmp	0x140315ad8 <.text+0x314ad8>
140315db1: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140315dba: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140315dc0: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140315dc6: 48 89 d9                    	movq	%rbx, %rcx
140315dc9: 49 89 d6                    	movq	%rdx, %r14
140315dcc: 48 89 c2                    	movq	%rax, %rdx
140315dcf: e8 0c cc 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140315dd4: 4c 89 f2                    	movq	%r14, %rdx
140315dd7: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315ddb: e9 54 f1 ff ff              	jmp	0x140314f34 <.text+0x313f34>
140315de0: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140315de9: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140315def: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140315df5: 48 89 d9                    	movq	%rbx, %rcx
140315df8: 49 89 d6                    	movq	%rdx, %r14
140315dfb: 48 89 c2                    	movq	%rax, %rdx
140315dfe: e8 dd cb 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140315e03: 4c 89 f2                    	movq	%r14, %rdx
140315e06: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315e0a: e9 11 f2 ff ff              	jmp	0x140315020 <.text+0x314020>
140315e0f: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140315e18: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140315e1e: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140315e24: 48 89 d9                    	movq	%rbx, %rcx
140315e27: 49 89 d6                    	movq	%rdx, %r14
140315e2a: 48 89 c2                    	movq	%rax, %rdx
140315e2d: e8 ae cb 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140315e32: 4c 89 f2                    	movq	%r14, %rdx
140315e35: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315e39: e9 e7 f2 ff ff              	jmp	0x140315125 <.text+0x314125>
140315e3e: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140315e47: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140315e4d: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140315e53: 48 89 d9                    	movq	%rbx, %rcx
140315e56: 49 89 d6                    	movq	%rdx, %r14
140315e59: 48 89 c2                    	movq	%rax, %rdx
140315e5c: e8 7f cb 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140315e61: 4c 89 f2                    	movq	%r14, %rdx
140315e64: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315e68: e9 ac f3 ff ff              	jmp	0x140315219 <.text+0x314219>
140315e6d: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140315e76: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140315e7c: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140315e82: 48 89 d9                    	movq	%rbx, %rcx
140315e85: 49 89 d6                    	movq	%rdx, %r14
140315e88: 48 89 c2                    	movq	%rax, %rdx
140315e8b: e8 50 cb 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140315e90: 4c 89 f2                    	movq	%r14, %rdx
140315e93: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315e97: e9 71 f4 ff ff              	jmp	0x14031530d <.text+0x31430d>
140315e9c: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140315ea5: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140315eab: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140315eb1: 48 89 d9                    	movq	%rbx, %rcx
140315eb4: 49 89 d6                    	movq	%rdx, %r14
140315eb7: 48 89 c2                    	movq	%rax, %rdx
140315eba: e8 21 cb 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140315ebf: 4c 89 f2                    	movq	%r14, %rdx
140315ec2: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315ec6: e9 2e f5 ff ff              	jmp	0x1403153f9 <.text+0x3143f9>
140315ecb: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140315ed4: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140315eda: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140315ee0: 48 89 d9                    	movq	%rbx, %rcx
140315ee3: 49 89 d6                    	movq	%rdx, %r14
140315ee6: 48 89 c2                    	movq	%rax, %rdx
140315ee9: e8 f2 ca 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140315eee: 4c 89 f2                    	movq	%r14, %rdx
140315ef1: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315ef5: e9 f3 f5 ff ff              	jmp	0x1403154ed <.text+0x3144ed>
140315efa: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140315f03: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140315f09: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140315f0f: 48 89 d9                    	movq	%rbx, %rcx
140315f12: 49 89 d6                    	movq	%rdx, %r14
140315f15: 48 89 c2                    	movq	%rax, %rdx
140315f18: e8 c3 ca 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140315f1d: 4c 89 f2                    	movq	%r14, %rdx
140315f20: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315f24: e9 c8 f6 ff ff              	jmp	0x1403155f1 <.text+0x3145f1>
140315f29: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140315f32: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140315f38: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140315f3e: 48 89 d9                    	movq	%rbx, %rcx
140315f41: 49 89 d6                    	movq	%rdx, %r14
140315f44: 48 89 c2                    	movq	%rax, %rdx
140315f47: e8 94 ca 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140315f4c: 4c 89 f2                    	movq	%r14, %rdx
140315f4f: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315f53: e9 8d f7 ff ff              	jmp	0x1403156e5 <.text+0x3146e5>
140315f58: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140315f61: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140315f67: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140315f6d: 48 89 d9                    	movq	%rbx, %rcx
140315f70: 49 89 d6                    	movq	%rdx, %r14
140315f73: 48 89 c2                    	movq	%rax, %rdx
140315f76: e8 65 ca 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140315f7b: 4c 89 f2                    	movq	%r14, %rdx
140315f7e: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315f82: e9 93 f7 ff ff              	jmp	0x14031571a <.text+0x31471a>
140315f87: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140315f90: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140315f96: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140315f9c: 48 89 d9                    	movq	%rbx, %rcx
140315f9f: 49 89 d6                    	movq	%rdx, %r14
140315fa2: 48 89 c2                    	movq	%rax, %rdx
140315fa5: e8 36 ca 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140315faa: 4c 89 f2                    	movq	%r14, %rdx
140315fad: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315fb1: e9 61 ee ff ff              	jmp	0x140314e17 <.text+0x313e17>
140315fb6: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140315fbf: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140315fc5: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140315fcb: 48 89 d9                    	movq	%rbx, %rcx
140315fce: 49 89 d6                    	movq	%rdx, %r14
140315fd1: 48 89 c2                    	movq	%rax, %rdx
140315fd4: e8 07 ca 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140315fd9: 4c 89 f2                    	movq	%r14, %rdx
140315fdc: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140315fe0: e9 6a f7 ff ff              	jmp	0x14031574f <.text+0x31474f>
140315fe5: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140315fee: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140315ff4: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140315ffa: 48 89 d9                    	movq	%rbx, %rcx
140315ffd: 49 89 d6                    	movq	%rdx, %r14
140316000: 48 89 c2                    	movq	%rax, %rdx
140316003: e8 d8 c9 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316008: 4c 89 f2                    	movq	%r14, %rdx
14031600b: 48 8b 43 10                 	movq	0x10(%rbx), %rax
14031600f: e9 80 f8 ff ff              	jmp	0x140315894 <.text+0x314894>
140316014: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14031601d: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316023: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316029: 48 89 d9                    	movq	%rbx, %rcx
14031602c: 49 89 d6                    	movq	%rdx, %r14
14031602f: 48 89 c2                    	movq	%rax, %rdx
140316032: e8 a9 c9 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316037: 4c 89 f2                    	movq	%r14, %rdx
14031603a: 48 8b 43 10                 	movq	0x10(%rbx), %rax
14031603e: e9 45 f9 ff ff              	jmp	0x140315988 <.text+0x314988>
140316043: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14031604c: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316052: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316058: 48 89 d9                    	movq	%rbx, %rcx
14031605b: 49 89 d6                    	movq	%rdx, %r14
14031605e: 48 89 c2                    	movq	%rax, %rdx
140316061: e8 7a c9 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316066: 4c 89 f2                    	movq	%r14, %rdx
140316069: 48 8b 43 10                 	movq	0x10(%rbx), %rax
14031606d: e9 0a fa ff ff              	jmp	0x140315a7c <.text+0x314a7c>
140316072: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14031607b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316081: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316087: 48 89 d9                    	movq	%rbx, %rcx
14031608a: 49 89 d6                    	movq	%rdx, %r14
14031608d: 48 89 c2                    	movq	%rax, %rdx
140316090: e8 4b c9 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316095: 4c 89 f2                    	movq	%r14, %rdx
140316098: 48 8b 43 10                 	movq	0x10(%rbx), %rax
14031609c: e9 cf fa ff ff              	jmp	0x140315b70 <.text+0x314b70>
1403160a1: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1403160aa: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403160b0: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1403160b6: 48 89 d9                    	movq	%rbx, %rcx
1403160b9: 49 89 d6                    	movq	%rdx, %r14
1403160bc: 48 89 c2                    	movq	%rax, %rdx
1403160bf: e8 1c c9 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
1403160c4: 4c 89 f2                    	movq	%r14, %rdx
1403160c7: 48 8b 43 10                 	movq	0x10(%rbx), %rax
1403160cb: e9 94 fb ff ff              	jmp	0x140315c64 <.text+0x314c64>
1403160d0: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1403160d9: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403160df: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1403160e5: 48 89 d9                    	movq	%rbx, %rcx
1403160e8: e8 f3 c8 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
1403160ed: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403160f1: e9 7b ee ff ff              	jmp	0x140314f71 <.text+0x313f71>
1403160f6: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1403160ff: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316105: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14031610b: 48 89 d9                    	movq	%rbx, %rcx
14031610e: e8 cd c8 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316113: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316117: e9 74 ee ff ff              	jmp	0x140314f90 <.text+0x313f90>
14031611c: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316125: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14031612b: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316131: 48 89 d9                    	movq	%rbx, %rcx
140316134: e8 a7 c8 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316139: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14031613d: e9 1b ef ff ff              	jmp	0x14031505d <.text+0x31405d>
140316142: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14031614b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316151: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316157: 48 89 d9                    	movq	%rbx, %rcx
14031615a: e8 81 c8 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
14031615f: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316163: e9 14 ef ff ff              	jmp	0x14031507c <.text+0x31407c>
140316168: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316171: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316177: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14031617d: 48 89 d9                    	movq	%rbx, %rcx
140316180: e8 5b c8 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316185: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316189: e9 d4 ef ff ff              	jmp	0x140315162 <.text+0x314162>
14031618e: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316197: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14031619d: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1403161a3: 48 89 d9                    	movq	%rbx, %rcx
1403161a6: e8 35 c8 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
1403161ab: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403161af: e9 cd ef ff ff              	jmp	0x140315181 <.text+0x314181>
1403161b4: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1403161bd: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403161c3: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1403161c9: 48 89 d9                    	movq	%rbx, %rcx
1403161cc: e8 0f c8 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
1403161d1: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403161d5: e9 7c f0 ff ff              	jmp	0x140315256 <.text+0x314256>
1403161da: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1403161e3: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403161e9: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1403161ef: 48 89 d9                    	movq	%rbx, %rcx
1403161f2: e8 e9 c7 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
1403161f7: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403161fb: e9 75 f0 ff ff              	jmp	0x140315275 <.text+0x314275>
140316200: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316209: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14031620f: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316215: 48 89 d9                    	movq	%rbx, %rcx
140316218: e8 c3 c7 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
14031621d: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316221: e9 24 f1 ff ff              	jmp	0x14031534a <.text+0x31434a>
140316226: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14031622f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316235: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14031623b: 48 89 d9                    	movq	%rbx, %rcx
14031623e: e8 9d c7 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316243: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316247: e9 1d f1 ff ff              	jmp	0x140315369 <.text+0x314369>
14031624c: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316255: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14031625b: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316261: 48 89 d9                    	movq	%rbx, %rcx
140316264: e8 77 c7 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316269: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14031626d: e9 c4 f1 ff ff              	jmp	0x140315436 <.text+0x314436>
140316272: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14031627b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316281: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316287: 48 89 d9                    	movq	%rbx, %rcx
14031628a: e8 51 c7 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
14031628f: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316293: e9 bd f1 ff ff              	jmp	0x140315455 <.text+0x314455>
140316298: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1403162a1: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403162a7: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1403162ad: 48 89 d9                    	movq	%rbx, %rcx
1403162b0: e8 2b c7 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
1403162b5: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403162b9: e9 6c f2 ff ff              	jmp	0x14031552a <.text+0x31452a>
1403162be: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1403162c7: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403162cd: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1403162d3: 48 89 d9                    	movq	%rbx, %rcx
1403162d6: e8 05 c7 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
1403162db: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403162df: e9 65 f2 ff ff              	jmp	0x140315549 <.text+0x314549>
1403162e4: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1403162ed: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403162f3: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1403162f9: 48 89 d9                    	movq	%rbx, %rcx
1403162fc: e8 df c6 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316301: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316305: e9 24 f3 ff ff              	jmp	0x14031562e <.text+0x31462e>
14031630a: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316313: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316319: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14031631f: 48 89 d9                    	movq	%rbx, %rcx
140316322: e8 b9 c6 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316327: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14031632b: e9 1d f3 ff ff              	jmp	0x14031564d <.text+0x31464d>
140316330: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316339: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14031633f: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316345: 48 89 d9                    	movq	%rbx, %rcx
140316348: e8 93 c6 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
14031634d: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316351: e9 fe ea ff ff              	jmp	0x140314e54 <.text+0x313e54>
140316356: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14031635f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316365: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14031636b: 48 89 d9                    	movq	%rbx, %rcx
14031636e: e8 6d c6 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316373: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316377: e9 f7 ea ff ff              	jmp	0x140314e73 <.text+0x313e73>
14031637c: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316385: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14031638b: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316391: 48 89 d9                    	movq	%rbx, %rcx
140316394: e8 47 c6 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316399: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14031639d: e9 ea f3 ff ff              	jmp	0x14031578c <.text+0x31478c>
1403163a2: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1403163ab: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403163b1: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1403163b7: 48 89 d9                    	movq	%rbx, %rcx
1403163ba: e8 21 c6 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
1403163bf: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403163c3: e9 e3 f3 ff ff              	jmp	0x1403157ab <.text+0x3147ab>
1403163c8: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1403163d1: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403163d7: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1403163dd: 48 89 d9                    	movq	%rbx, %rcx
1403163e0: e8 fb c5 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
1403163e5: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403163e9: e9 e3 f4 ff ff              	jmp	0x1403158d1 <.text+0x3148d1>
1403163ee: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1403163f7: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403163fd: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316403: 48 89 d9                    	movq	%rbx, %rcx
140316406: e8 d5 c5 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
14031640b: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14031640f: e9 dc f4 ff ff              	jmp	0x1403158f0 <.text+0x3148f0>
140316414: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14031641d: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316423: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316429: 48 89 d9                    	movq	%rbx, %rcx
14031642c: e8 af c5 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316431: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316435: e9 8b f5 ff ff              	jmp	0x1403159c5 <.text+0x3149c5>
14031643a: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316443: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316449: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14031644f: 48 89 d9                    	movq	%rbx, %rcx
140316452: e8 89 c5 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316457: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14031645b: e9 84 f5 ff ff              	jmp	0x1403159e4 <.text+0x3149e4>
140316460: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316469: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14031646f: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316475: 48 89 d9                    	movq	%rbx, %rcx
140316478: e8 63 c5 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
14031647d: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316481: e9 27 f7 ff ff              	jmp	0x140315bad <.text+0x314bad>
140316486: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14031648f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316495: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14031649b: 48 89 d9                    	movq	%rbx, %rcx
14031649e: e8 3d c5 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
1403164a3: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403164a7: e9 20 f7 ff ff              	jmp	0x140315bcc <.text+0x314bcc>
1403164ac: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1403164b5: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403164bb: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1403164c1: 48 89 d9                    	movq	%rbx, %rcx
1403164c4: e8 17 c5 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
1403164c9: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403164cd: e9 cf f7 ff ff              	jmp	0x140315ca1 <.text+0x314ca1>
1403164d2: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1403164db: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403164e1: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1403164e7: 48 89 d9                    	movq	%rbx, %rcx
1403164ea: e8 f1 c4 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
1403164ef: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403164f3: e9 c8 f7 ff ff              	jmp	0x140315cc0 <.text+0x314cc0>
