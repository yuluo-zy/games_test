
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b68e40: 55                          	pushq	%rbp
140b68e41: 56                          	pushq	%rsi
140b68e42: 57                          	pushq	%rdi
140b68e43: 53                          	pushq	%rbx
140b68e44: 48 81 ec f8 03 00 00        	subq	$0x3f8, %rsp            # imm = 0x3F8
140b68e4b: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140b68e53: 48 c7 85 70 03 00 00 fe ff ff ff    	movq	$-0x2, 0x370(%rbp)
140b68e5e: 48 89 d6                    	movq	%rdx, %rsi
140b68e61: 0f 57 c0                    	xorps	%xmm0, %xmm0
140b68e64: 0f 11 45 c8                 	movups	%xmm0, -0x38(%rbp)
140b68e68: 0f 11 45 e0                 	movups	%xmm0, -0x20(%rbp)
140b68e6c: 48 c7 45 a0 01 00 00 00     	movq	$0x1, -0x60(%rbp)
140b68e74: 48 c7 45 a8 01 00 00 00     	movq	$0x1, -0x58(%rbp)
140b68e7c: c6 45 b0 00                 	movb	$0x0, -0x50(%rbp)
140b68e80: 48 c7 45 b8 00 00 00 00     	movq	$0x0, -0x48(%rbp)
140b68e88: 48 c7 45 c0 08 00 00 00     	movq	$0x8, -0x40(%rbp)
140b68e90: 48 c7 45 d8 08 00 00 00     	movq	$0x8, -0x28(%rbp)
140b68e98: 0f b6 05 6a bd ac 02        	movzbl	0x2acbd6a(%rip), %eax   # 0x143634c09
140b68e9f: b9 50 00 00 00              	movl	$0x50, %ecx
140b68ea4: ba 08 00 00 00              	movl	$0x8, %edx
140b68ea9: e8 62 ad aa ff              	callq	0x140613c10 <.text+0x612c10>
140b68eae: 48 85 c0                    	testq	%rax, %rax
140b68eb1: 0f 84 9e 05 00 00           	je	0x140b69455 <.text+0xb68455>
140b68eb7: 0f 10 45 e0                 	movups	-0x20(%rbp), %xmm0
140b68ebb: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
140b68ebf: 0f 10 45 a0                 	movups	-0x60(%rbp), %xmm0
140b68ec3: 0f 10 4d b0                 	movups	-0x50(%rbp), %xmm1
140b68ec7: 0f 10 55 c0                 	movups	-0x40(%rbp), %xmm2
140b68ecb: 0f 10 5d d0                 	movups	-0x30(%rbp), %xmm3
140b68ecf: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
140b68ed3: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
140b68ed7: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
140b68edb: 0f 11 00                    	movups	%xmm0, (%rax)
140b68ede: 48 89 85 c0 02 00 00        	movq	%rax, 0x2c0(%rbp)
140b68ee5: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140b68eef: 48 89 85 b8 02 00 00        	movq	%rax, 0x2b8(%rbp)
140b68ef6: c6 85 6f 03 00 00 01        	movb	$0x1, 0x36f(%rbp)
140b68efd: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140b68f01: e8 aa 1c fe ff              	callq	0x140b4abb0 <.text+0xb49bb0>
140b68f06: c6 85 f0 02 00 00 10        	movb	$0x10, 0x2f0(%rbp)
140b68f0d: 48 8b 0d 5c 34 ac 02        	movq	0x2ac345c(%rip), %rcx   # 0x14362c370
140b68f14: 4c 8d 05 45 6e 0f 02        	leaq	0x20f6e45(%rip), %r8    # 0x142c5fd60
140b68f1b: 48 8d 95 f0 02 00 00        	leaq	0x2f0(%rbp), %rdx
140b68f22: e8 79 89 fb ff              	callq	0x140b218a0 <.text+0xb208a0>
140b68f27: 49 89 d0                    	movq	%rdx, %r8
140b68f2a: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140b68f2e: 48 89 c2                    	movq	%rax, %rdx
140b68f31: e8 ca a0 8e 00              	callq	0x141453000 <.text+0x1452000>
140b68f36: 0f 10 45 00                 	movups	(%rbp), %xmm0
140b68f3a: 0f 29 85 a0 02 00 00        	movaps	%xmm0, 0x2a0(%rbp)
140b68f41: 0f 10 45 f0                 	movups	-0x10(%rbp), %xmm0
140b68f45: 0f 29 85 90 02 00 00        	movaps	%xmm0, 0x290(%rbp)
140b68f4c: 0f 10 45 e0                 	movups	-0x20(%rbp), %xmm0
140b68f50: 0f 29 85 80 02 00 00        	movaps	%xmm0, 0x280(%rbp)
140b68f57: 0f 10 45 a0                 	movups	-0x60(%rbp), %xmm0
140b68f5b: 0f 10 4d b0                 	movups	-0x50(%rbp), %xmm1
140b68f5f: 0f 10 55 c0                 	movups	-0x40(%rbp), %xmm2
140b68f63: 0f 10 5d d0                 	movups	-0x30(%rbp), %xmm3
140b68f67: 0f 29 9d 70 02 00 00        	movaps	%xmm3, 0x270(%rbp)
140b68f6e: 0f 29 95 60 02 00 00        	movaps	%xmm2, 0x260(%rbp)
140b68f75: 0f 29 8d 50 02 00 00        	movaps	%xmm1, 0x250(%rbp)
140b68f7c: 0f 29 85 40 02 00 00        	movaps	%xmm0, 0x240(%rbp)
140b68f83: 4c 8d 05 4e a0 f8 01        	leaq	0x1f8a04e(%rip), %r8    # 0x142af2fd8
140b68f8a: 48 8d 8d f0 02 00 00        	leaq	0x2f0(%rbp), %rcx
140b68f91: b2 22                       	movb	$0x22, %dl
140b68f93: e8 a8 4c e6 ff              	callq	0x1409cdc40 <.text+0x9ccc40>
140b68f98: 0f 28 85 a0 02 00 00        	movaps	0x2a0(%rbp), %xmm0
140b68f9f: 0f 29 45 00                 	movaps	%xmm0, (%rbp)
140b68fa3: 0f 28 85 90 02 00 00        	movaps	0x290(%rbp), %xmm0
140b68faa: 0f 29 45 f0                 	movaps	%xmm0, -0x10(%rbp)
140b68fae: 0f 28 85 80 02 00 00        	movaps	0x280(%rbp), %xmm0
140b68fb5: 0f 29 45 e0                 	movaps	%xmm0, -0x20(%rbp)
140b68fb9: 0f 28 85 40 02 00 00        	movaps	0x240(%rbp), %xmm0
140b68fc0: 0f 28 8d 50 02 00 00        	movaps	0x250(%rbp), %xmm1
140b68fc7: 0f 28 95 60 02 00 00        	movaps	0x260(%rbp), %xmm2
140b68fce: 0f 28 9d 70 02 00 00        	movaps	0x270(%rbp), %xmm3
140b68fd5: 0f 29 5d d0                 	movaps	%xmm3, -0x30(%rbp)
140b68fd9: 0f 29 55 c0                 	movaps	%xmm2, -0x40(%rbp)
140b68fdd: 0f 29 4d b0                 	movaps	%xmm1, -0x50(%rbp)
140b68fe1: 0f 29 45 a0                 	movaps	%xmm0, -0x60(%rbp)
140b68fe5: 0f 10 85 f0 02 00 00        	movups	0x2f0(%rbp), %xmm0
140b68fec: 0f 10 8d 00 03 00 00        	movups	0x300(%rbp), %xmm1
140b68ff3: 0f 10 95 10 03 00 00        	movups	0x310(%rbp), %xmm2
140b68ffa: 0f 10 9d 20 03 00 00        	movups	0x320(%rbp), %xmm3
140b69001: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
140b69005: 0f 29 4d 20                 	movaps	%xmm1, 0x20(%rbp)
140b69009: 0f 29 55 30                 	movaps	%xmm2, 0x30(%rbp)
140b6900d: 0f 29 5d 40                 	movaps	%xmm3, 0x40(%rbp)
140b69011: 0f 10 85 30 03 00 00        	movups	0x330(%rbp), %xmm0
140b69018: 0f 29 45 50                 	movaps	%xmm0, 0x50(%rbp)
140b6901c: 0f 10 85 40 03 00 00        	movups	0x340(%rbp), %xmm0
140b69023: 0f 29 45 60                 	movaps	%xmm0, 0x60(%rbp)
140b69027: 0f 10 85 50 03 00 00        	movups	0x350(%rbp), %xmm0
140b6902e: 0f 29 45 70                 	movaps	%xmm0, 0x70(%rbp)
140b69032: c6 85 6f 03 00 00 01        	movb	$0x1, 0x36f(%rbp)
140b69039: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
140b6903d: 48 89 f1                    	movq	%rsi, %rcx
140b69040: e8 7b e9 ff ff              	callq	0x140b679c0 <.text+0xb669c0>
140b69045: 48 b8 02 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffe, %rax # imm = 0x8000000000000002
140b6904f: 48 39 85 b8 02 00 00        	cmpq	%rax, 0x2b8(%rbp)
140b69056: 0f 8d 0a 04 00 00           	jge	0x140b69466 <.text+0xb68466>
140b6905c: 48 8b bd c0 02 00 00        	movq	0x2c0(%rbp), %rdi
140b69063: f0                          	lock
140b69064: 48 ff 07                    	incq	(%rdi)
140b69067: 0f 8e 32 04 00 00           	jle	0x140b6949f <.text+0xb6849f>
140b6906d: 48 89 bd f8 02 00 00        	movq	%rdi, 0x2f8(%rbp)
140b69074: 48 bb 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rbx # imm = 0x8000000000000001
140b6907e: 48 89 9d f0 02 00 00        	movq	%rbx, 0x2f0(%rbp)
140b69085: c6 85 6f 03 00 00 01        	movb	$0x1, 0x36f(%rbp)
140b6908c: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140b69090: 48 8d 95 f0 02 00 00        	leaq	0x2f0(%rbp), %rdx
140b69097: e8 94 c4 ea ff              	callq	0x140a15530 <.text+0xa14530>
140b6909c: c6 85 6f 03 00 00 01        	movb	$0x1, 0x36f(%rbp)
140b690a3: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
140b690a7: 48 89 f1                    	movq	%rsi, %rcx
140b690aa: e8 41 61 9d 00              	callq	0x14153f1f0 <.text+0x153e1f0>
140b690af: f0                          	lock
140b690b0: 48 ff 07                    	incq	(%rdi)
140b690b3: 0f 8e e6 03 00 00           	jle	0x140b6949f <.text+0xb6849f>
140b690b9: 48 89 c6                    	movq	%rax, %rsi
140b690bc: 48 89 bd f8 02 00 00        	movq	%rdi, 0x2f8(%rbp)
140b690c3: 48 89 9d f0 02 00 00        	movq	%rbx, 0x2f0(%rbp)
140b690ca: c6 85 6f 03 00 00 01        	movb	$0x1, 0x36f(%rbp)
140b690d1: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140b690d5: 48 8d 95 f0 02 00 00        	leaq	0x2f0(%rbp), %rdx
140b690dc: e8 4f c4 ea ff              	callq	0x140a15530 <.text+0xa14530>
140b690e1: c6 85 6f 03 00 00 01        	movb	$0x1, 0x36f(%rbp)
140b690e8: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
140b690ec: 48 89 f1                    	movq	%rsi, %rcx
140b690ef: e8 7c d4 9c 00              	callq	0x141536570 <.text+0x1535570>
140b690f4: f0                          	lock
140b690f5: 48 ff 07                    	incq	(%rdi)
140b690f8: 0f 8e a1 03 00 00           	jle	0x140b6949f <.text+0xb6849f>
140b690fe: 48 89 c6                    	movq	%rax, %rsi
140b69101: 48 89 bd f8 02 00 00        	movq	%rdi, 0x2f8(%rbp)
140b69108: 48 89 9d f0 02 00 00        	movq	%rbx, 0x2f0(%rbp)
140b6910f: c6 85 6f 03 00 00 01        	movb	$0x1, 0x36f(%rbp)
140b69116: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140b6911a: 48 8d 95 f0 02 00 00        	leaq	0x2f0(%rbp), %rdx
140b69121: e8 0a c4 ea ff              	callq	0x140a15530 <.text+0xa14530>
140b69126: c6 85 6f 03 00 00 01        	movb	$0x1, 0x36f(%rbp)
140b6912d: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
140b69131: 48 89 f1                    	movq	%rsi, %rcx
140b69134: e8 17 9c 9c 00              	callq	0x141532d50 <.text+0x1531d50>
140b69139: 48 89 c6                    	movq	%rax, %rsi
140b6913c: 48 8b 85 e8 02 00 00        	movq	0x2e8(%rbp), %rax
140b69143: 48 89 85 20 03 00 00        	movq	%rax, 0x320(%rbp)
140b6914a: 0f 10 85 b8 02 00 00        	movups	0x2b8(%rbp), %xmm0
140b69151: 0f 10 8d c8 02 00 00        	movups	0x2c8(%rbp), %xmm1
140b69158: 0f 10 95 d8 02 00 00        	movups	0x2d8(%rbp), %xmm2
140b6915f: 0f 29 95 10 03 00 00        	movaps	%xmm2, 0x310(%rbp)
140b69166: 0f 29 8d 00 03 00 00        	movaps	%xmm1, 0x300(%rbp)
140b6916d: 0f 29 85 f0 02 00 00        	movaps	%xmm0, 0x2f0(%rbp)
140b69174: c6 85 6f 03 00 00 00        	movb	$0x0, 0x36f(%rbp)
140b6917b: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140b6917f: 48 8d 95 f0 02 00 00        	leaq	0x2f0(%rbp), %rdx
140b69186: e8 a5 c3 ea ff              	callq	0x140a15530 <.text+0xa14530>
140b6918b: c6 85 6f 03 00 00 00        	movb	$0x0, 0x36f(%rbp)
140b69192: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
140b69196: 48 89 f1                    	movq	%rsi, %rcx
140b69199: e8 12 f0 9c 00              	callq	0x1415381b0 <.text+0x15371b0>
140b6919e: f2 0f 10 05 2a c5 f5 01     	movsd	0x1f5c52a(%rip), %xmm0  # 0x142ac56d0
140b691a6: f2 0f 11 45 a0              	movsd	%xmm0, -0x60(%rbp)
140b691ab: c7 45 a8 00 00 40 40        	movl	$0x40400000, -0x58(%rbp) # imm = 0x40400000
140b691b2: c6 85 6f 03 00 00 00        	movb	$0x0, 0x36f(%rbp)
140b691b9: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
140b691bd: 48 89 c1                    	movq	%rax, %rcx
140b691c0: e8 4b 6e 9c 00              	callq	0x141530010 <.text+0x152f010>
140b691c5: 48 89 c6                    	movq	%rax, %rsi
140b691c8: 48 8d 78 38                 	leaq	0x38(%rax), %rdi
140b691cc: c6 85 6f 03 00 00 00        	movb	$0x0, 0x36f(%rbp)
140b691d3: 48 ba af e5 5a c3 13 84 ce 7a       	movabsq	$0x7ace8413c35ae5af, %rdx # imm = 0x7ACE8413C35AE5AF
140b691dd: 49 b8 de c7 d9 70 4e a2 2e 6c       	movabsq	$0x6c2ea24e70d9c7de, %r8 # imm = 0x6C2EA24E70D9C7DE
140b691e7: 48 89 f9                    	movq	%rdi, %rcx
140b691ea: e8 01 45 57 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
140b691ef: 48 85 c0                    	testq	%rax, %rax
140b691f2: 74 33                       	je	0x140b69227 <.text+0xb68227>
140b691f4: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
140b691fb: 76 2a                       	jbe	0x140b69227 <.text+0xb68227>
140b691fd: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
140b69204: 48 8b 04 d0                 	movq	(%rax,%rdx,8), %rax
140b69208: 48 85 c0                    	testq	%rax, %rax
140b6920b: 74 1a                       	je	0x140b69227 <.text+0xb68227>
140b6920d: 48 8b 8e e0 01 00 00        	movq	0x1e0(%rsi), %rcx
140b69214: 48 f7 d0                    	notq	%rax
140b69217: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
140b6921b: 48 c1 e0 05                 	shlq	$0x5, %rax
140b6921f: 48 83 7c 01 20 00           	cmpq	$0x0, 0x20(%rcx,%rax)
140b69225: 75 0f                       	jne	0x140b69236 <.text+0xb68236>
140b69227: c6 85 6f 03 00 00 00        	movb	$0x0, 0x36f(%rbp)
140b6922e: 48 89 f1                    	movq	%rsi, %rcx
140b69231: e8 1a fe 9a 00              	callq	0x141519050 <.text+0x1518050>
140b69236: c6 85 6f 03 00 00 00        	movb	$0x0, 0x36f(%rbp)
140b6923d: 48 ba 33 4b 8d 0a df cb 41 ec       	movabsq	$-0x13be3420f572b4cd, %rdx # imm = 0xEC41CBDF0A8D4B33
140b69247: 49 b8 74 93 ed 65 cc 66 c0 3a       	movabsq	$0x3ac066cc65ed9374, %r8 # imm = 0x3AC066CC65ED9374
140b69251: 48 89 f9                    	movq	%rdi, %rcx
140b69254: e8 97 44 57 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
140b69259: 48 85 c0                    	testq	%rax, %rax
140b6925c: 74 33                       	je	0x140b69291 <.text+0xb68291>
140b6925e: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
140b69265: 76 2a                       	jbe	0x140b69291 <.text+0xb68291>
140b69267: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
140b6926e: 48 8b 04 d0                 	movq	(%rax,%rdx,8), %rax
140b69272: 48 85 c0                    	testq	%rax, %rax
140b69275: 74 1a                       	je	0x140b69291 <.text+0xb68291>
140b69277: 48 8b 8e e0 01 00 00        	movq	0x1e0(%rsi), %rcx
140b6927e: 48 f7 d0                    	notq	%rax
140b69281: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
140b69285: 48 c1 e0 05                 	shlq	$0x5, %rax
140b69289: 48 83 7c 01 20 00           	cmpq	$0x0, 0x20(%rcx,%rax)
140b6928f: 75 0f                       	jne	0x140b692a0 <.text+0xb682a0>
140b69291: c6 85 6f 03 00 00 00        	movb	$0x0, 0x36f(%rbp)
140b69298: 48 89 f1                    	movq	%rsi, %rcx
140b6929b: e8 a0 1c 9b 00              	callq	0x14151af40 <.text+0x1519f40>
140b692a0: c6 85 6f 03 00 00 00        	movb	$0x0, 0x36f(%rbp)
140b692a7: 48 ba 09 35 a3 06 31 10 b0 5c       	movabsq	$0x5cb0103106a33509, %rdx # imm = 0x5CB0103106A33509
140b692b1: 49 b8 33 a1 41 9b b2 b3 a1 ac       	movabsq	$-0x535e4c4d64be5ecd, %r8 # imm = 0xACA1B3B29B41A133
140b692bb: 48 89 f9                    	movq	%rdi, %rcx
140b692be: e8 2d 44 57 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
140b692c3: 48 85 c0                    	testq	%rax, %rax
140b692c6: 74 33                       	je	0x140b692fb <.text+0xb682fb>
140b692c8: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
140b692cf: 76 2a                       	jbe	0x140b692fb <.text+0xb682fb>
140b692d1: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
140b692d8: 48 8b 04 d0                 	movq	(%rax,%rdx,8), %rax
140b692dc: 48 85 c0                    	testq	%rax, %rax
140b692df: 74 1a                       	je	0x140b692fb <.text+0xb682fb>
140b692e1: 48 8b 8e e0 01 00 00        	movq	0x1e0(%rsi), %rcx
140b692e8: 48 f7 d0                    	notq	%rax
140b692eb: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
140b692ef: 48 c1 e0 05                 	shlq	$0x5, %rax
