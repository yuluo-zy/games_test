
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140342c10: 55                          	pushq	%rbp
140342c11: 41 57                       	pushq	%r15
140342c13: 41 56                       	pushq	%r14
140342c15: 41 54                       	pushq	%r12
140342c17: 56                          	pushq	%rsi
140342c18: 57                          	pushq	%rdi
140342c19: 53                          	pushq	%rbx
140342c1a: 48 81 ec 40 07 00 00        	subq	$0x740, %rsp            # imm = 0x740
140342c21: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140342c29: 44 0f 29 a5 b0 06 00 00     	movaps	%xmm12, 0x6b0(%rbp)
140342c31: 44 0f 29 9d a0 06 00 00     	movaps	%xmm11, 0x6a0(%rbp)
140342c39: 44 0f 29 95 90 06 00 00     	movaps	%xmm10, 0x690(%rbp)
140342c41: 44 0f 29 8d 80 06 00 00     	movaps	%xmm9, 0x680(%rbp)
140342c49: 44 0f 29 85 70 06 00 00     	movaps	%xmm8, 0x670(%rbp)
140342c51: 0f 29 bd 60 06 00 00        	movaps	%xmm7, 0x660(%rbp)
140342c58: 0f 29 b5 50 06 00 00        	movaps	%xmm6, 0x650(%rbp)
140342c5f: 48 c7 85 48 06 00 00 fe ff ff ff    	movq	$-0x2, 0x648(%rbp)
140342c6a: 48 89 d6                    	movq	%rdx, %rsi
140342c6d: 48 8d 7a 38                 	leaq	0x38(%rdx), %rdi
140342c71: 48 ba 52 96 34 95 9e 26 3d 94       	movabsq	$-0x6bc2d9616acb69ae, %rdx # imm = 0x943D269E95349652
140342c7b: 49 b8 91 d7 c2 89 7e 6b e2 30       	movabsq	$0x30e26b7e89c2d791, %r8 # imm = 0x30E26B7E89C2D791
140342c85: 48 89 f9                    	movq	%rdi, %rcx
140342c88: e8 63 aa d9 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
140342c8d: 48 85 c0                    	testq	%rax, %rax
140342c90: 74 33                       	je	0x140342cc5 <.text+0x341cc5>
140342c92: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
140342c99: 76 2a                       	jbe	0x140342cc5 <.text+0x341cc5>
140342c9b: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
140342ca2: 48 8b 04 d0                 	movq	(%rax,%rdx,8), %rax
140342ca6: 48 85 c0                    	testq	%rax, %rax
140342ca9: 74 1a                       	je	0x140342cc5 <.text+0x341cc5>
140342cab: 48 8b 8e e0 01 00 00        	movq	0x1e0(%rsi), %rcx
140342cb2: 48 f7 d0                    	notq	%rax
140342cb5: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
140342cb9: 48 c1 e0 05                 	shlq	$0x5, %rax
140342cbd: 48 83 7c 01 20 00           	cmpq	$0x0, 0x20(%rcx,%rax)
140342cc3: 75 08                       	jne	0x140342ccd <.text+0x341ccd>
140342cc5: 48 89 f1                    	movq	%rsi, %rcx
140342cc8: e8 83 1e 24 00              	callq	0x140584b50 <.text+0x583b50>
140342ccd: 48 ba dd ee 2a 55 40 1b 87 75       	movabsq	$0x75871b40552aeedd, %rdx # imm = 0x75871B40552AEEDD
140342cd7: 49 b8 72 7d 1f c5 d3 ff 54 ea       	movabsq	$-0x15ab002c3ae0828e, %r8 # imm = 0xEA54FFD3C51F7D72
140342ce1: 48 89 f9                    	movq	%rdi, %rcx
140342ce4: e8 07 aa d9 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
140342ce9: 48 85 c0                    	testq	%rax, %rax
140342cec: 74 33                       	je	0x140342d21 <.text+0x341d21>
140342cee: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
140342cf5: 76 2a                       	jbe	0x140342d21 <.text+0x341d21>
140342cf7: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
140342cfe: 48 8b 04 d0                 	movq	(%rax,%rdx,8), %rax
140342d02: 48 85 c0                    	testq	%rax, %rax
140342d05: 74 1a                       	je	0x140342d21 <.text+0x341d21>
140342d07: 48 8b 8e e0 01 00 00        	movq	0x1e0(%rsi), %rcx
140342d0e: 48 f7 d0                    	notq	%rax
140342d11: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
140342d15: 48 c1 e0 05                 	shlq	$0x5, %rax
140342d19: 48 83 7c 01 20 00           	cmpq	$0x0, 0x20(%rcx,%rax)
140342d1f: 75 08                       	jne	0x140342d29 <.text+0x341d29>
140342d21: 48 89 f1                    	movq	%rsi, %rcx
140342d24: e8 07 96 23 00              	callq	0x14057c330 <.text+0x57b330>
140342d29: 48 ba 48 67 20 09 bd 63 bb d1       	movabsq	$-0x2e449c42f6df98b8, %rdx # imm = 0xD1BB63BD09206748
140342d33: 49 b8 5c 33 d6 9e 21 c9 cb 75       	movabsq	$0x75cbc9219ed6335c, %r8 # imm = 0x75CBC9219ED6335C
140342d3d: 48 89 f9                    	movq	%rdi, %rcx
140342d40: e8 ab a9 d9 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
140342d45: 48 85 c0                    	testq	%rax, %rax
140342d48: 74 33                       	je	0x140342d7d <.text+0x341d7d>
140342d4a: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
140342d51: 76 2a                       	jbe	0x140342d7d <.text+0x341d7d>
140342d53: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
140342d5a: 48 8b 04 d0                 	movq	(%rax,%rdx,8), %rax
140342d5e: 48 85 c0                    	testq	%rax, %rax
140342d61: 74 1a                       	je	0x140342d7d <.text+0x341d7d>
140342d63: 48 8b 8e e0 01 00 00        	movq	0x1e0(%rsi), %rcx
140342d6a: 48 f7 d0                    	notq	%rax
140342d6d: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
140342d71: 48 c1 e0 05                 	shlq	$0x5, %rax
140342d75: 48 83 7c 01 20 00           	cmpq	$0x0, 0x20(%rcx,%rax)
140342d7b: 75 08                       	jne	0x140342d85 <.text+0x341d85>
140342d7d: 48 89 f1                    	movq	%rsi, %rcx
140342d80: e8 5b cd 23 00              	callq	0x14057fae0 <.text+0x57eae0>
140342d85: 48 ba ca 08 8a 19 1b f9 23 81       	movabsq	$-0x7edc06e4e675f736, %rdx # imm = 0x8123F91B198A08CA
140342d8f: 49 b8 e4 4a 71 2f 53 31 99 e4       	movabsq	$-0x1b66ceacd08eb51c, %r8 # imm = 0xE49931532F714AE4
140342d99: 48 89 f9                    	movq	%rdi, %rcx
140342d9c: e8 4f a9 d9 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
140342da1: 48 85 c0                    	testq	%rax, %rax
140342da4: 74 33                       	je	0x140342dd9 <.text+0x341dd9>
140342da6: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
140342dad: 76 2a                       	jbe	0x140342dd9 <.text+0x341dd9>
140342daf: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
140342db6: 48 8b 04 d0                 	movq	(%rax,%rdx,8), %rax
140342dba: 48 85 c0                    	testq	%rax, %rax
140342dbd: 74 1a                       	je	0x140342dd9 <.text+0x341dd9>
140342dbf: 48 8b 8e e0 01 00 00        	movq	0x1e0(%rsi), %rcx
140342dc6: 48 f7 d0                    	notq	%rax
140342dc9: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
140342dcd: 48 c1 e0 05                 	shlq	$0x5, %rax
140342dd1: 48 83 7c 01 20 00           	cmpq	$0x0, 0x20(%rcx,%rax)
140342dd7: 75 08                       	jne	0x140342de1 <.text+0x341de1>
140342dd9: 48 89 f1                    	movq	%rsi, %rcx
140342ddc: e8 9f 23 24 00              	callq	0x140585180 <.text+0x584180>
140342de1: 48 ba 5f 8e 7a 7a c4 78 64 48       	movabsq	$0x486478c47a7a8e5f, %rdx # imm = 0x486478C47A7A8E5F
140342deb: 49 b8 3a 7e bc 2d fa d9 94 be       	movabsq	$-0x416b2605d24381c6, %r8 # imm = 0xBE94D9FA2DBC7E3A
140342df5: 48 89 f9                    	movq	%rdi, %rcx
140342df8: e8 f3 a8 d9 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
140342dfd: 48 85 c0                    	testq	%rax, %rax
140342e00: 74 33                       	je	0x140342e35 <.text+0x341e35>
140342e02: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
140342e09: 76 2a                       	jbe	0x140342e35 <.text+0x341e35>
140342e0b: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
140342e12: 48 8b 04 d0                 	movq	(%rax,%rdx,8), %rax
140342e16: 48 85 c0                    	testq	%rax, %rax
140342e19: 74 1a                       	je	0x140342e35 <.text+0x341e35>
140342e1b: 48 8b 8e e0 01 00 00        	movq	0x1e0(%rsi), %rcx
140342e22: 48 f7 d0                    	notq	%rax
140342e25: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
140342e29: 48 c1 e0 05                 	shlq	$0x5, %rax
140342e2d: 48 83 7c 01 20 00           	cmpq	$0x0, 0x20(%rcx,%rax)
140342e33: 75 08                       	jne	0x140342e3d <.text+0x341e3d>
140342e35: 48 89 f1                    	movq	%rsi, %rcx
140342e38: e8 73 c6 23 00              	callq	0x14057f4b0 <.text+0x57e4b0>
140342e3d: 48 ba 96 9d 76 55 a6 1d 7f 93       	movabsq	$-0x6c80e259aa89626a, %rdx # imm = 0x937F1DA655769D96
140342e47: 49 b8 5f 53 7c 98 68 46 83 f0       	movabsq	$-0xf7cb9976783aca1, %r8 # imm = 0xF0834668987C535F
140342e51: 48 89 f9                    	movq	%rdi, %rcx
140342e54: e8 97 a8 d9 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
140342e59: 48 85 c0                    	testq	%rax, %rax
140342e5c: 74 33                       	je	0x140342e91 <.text+0x341e91>
140342e5e: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
140342e65: 76 2a                       	jbe	0x140342e91 <.text+0x341e91>
140342e67: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
140342e6e: 48 8b 04 d0                 	movq	(%rax,%rdx,8), %rax
140342e72: 48 85 c0                    	testq	%rax, %rax
140342e75: 74 1a                       	je	0x140342e91 <.text+0x341e91>
140342e77: 48 8b 8e e0 01 00 00        	movq	0x1e0(%rsi), %rcx
140342e7e: 48 f7 d0                    	notq	%rax
140342e81: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
140342e85: 48 c1 e0 05                 	shlq	$0x5, %rax
140342e89: 48 83 7c 01 20 00           	cmpq	$0x0, 0x20(%rcx,%rax)
140342e8f: 75 08                       	jne	0x140342e99 <.text+0x341e99>
140342e91: 48 89 f1                    	movq	%rsi, %rcx
140342e94: e8 57 84 22 00              	callq	0x14056b2f0 <.text+0x56a2f0>
140342e99: 48 ba 21 29 83 f2 44 70 cc b8       	movabsq	$-0x47338fbb0d7cd6df, %rdx # imm = 0xB8CC7044F2832921
140342ea3: 49 b8 5a fe 8d ff 8e cc 7e 58       	movabsq	$0x587ecc8eff8dfe5a, %r8 # imm = 0x587ECC8EFF8DFE5A
140342ead: 48 89 f9                    	movq	%rdi, %rcx
140342eb0: e8 3b a8 d9 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
140342eb5: 48 85 c0                    	testq	%rax, %rax
140342eb8: 74 33                       	je	0x140342eed <.text+0x341eed>
140342eba: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
140342ec1: 76 2a                       	jbe	0x140342eed <.text+0x341eed>
140342ec3: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
140342eca: 48 8b 04 d0                 	movq	(%rax,%rdx,8), %rax
140342ece: 48 85 c0                    	testq	%rax, %rax
140342ed1: 74 1a                       	je	0x140342eed <.text+0x341eed>
140342ed3: 48 8b 8e e0 01 00 00        	movq	0x1e0(%rsi), %rcx
140342eda: 48 f7 d0                    	notq	%rax
140342edd: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
140342ee1: 48 c1 e0 05                 	shlq	$0x5, %rax
140342ee5: 48 83 7c 01 20 00           	cmpq	$0x0, 0x20(%rcx,%rax)
140342eeb: 75 08                       	jne	0x140342ef5 <.text+0x341ef5>
140342eed: 48 89 f1                    	movq	%rsi, %rcx
140342ef0: e8 9b 77 22 00              	callq	0x14056a690 <.text+0x569690>
140342ef5: 48 ba 67 4c ef cf 7a 56 7f 71       	movabsq	$0x717f567acfef4c67, %rdx # imm = 0x717F567ACFEF4C67
140342eff: 49 b8 ae b0 83 fa 5f a6 3b 19       	movabsq	$0x193ba65ffa83b0ae, %r8 # imm = 0x193BA65FFA83B0AE
140342f09: 48 89 f9                    	movq	%rdi, %rcx
140342f0c: e8 df a7 d9 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
140342f11: 48 85 c0                    	testq	%rax, %rax
140342f14: 74 33                       	je	0x140342f49 <.text+0x341f49>
140342f16: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
140342f1d: 76 2a                       	jbe	0x140342f49 <.text+0x341f49>
140342f1f: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
140342f26: 48 8b 04 d0                 	movq	(%rax,%rdx,8), %rax
140342f2a: 48 85 c0                    	testq	%rax, %rax
140342f2d: 74 1a                       	je	0x140342f49 <.text+0x341f49>
140342f2f: 48 8b 8e e0 01 00 00        	movq	0x1e0(%rsi), %rcx
140342f36: 48 f7 d0                    	notq	%rax
140342f39: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
140342f3d: 48 c1 e0 05                 	shlq	$0x5, %rax
140342f41: 48 83 7c 01 20 00           	cmpq	$0x0, 0x20(%rcx,%rax)
140342f47: 75 08                       	jne	0x140342f51 <.text+0x341f51>
140342f49: 48 89 f1                    	movq	%rsi, %rcx
140342f4c: e8 1f 41 24 00              	callq	0x140587070 <.text+0x586070>
140342f51: 0f 57 c0                    	xorps	%xmm0, %xmm0
140342f54: 0f 29 85 20 03 00 00        	movaps	%xmm0, 0x320(%rbp)
140342f5b: 48 c7 85 30 03 00 00 08 00 00 00    	movq	$0x8, 0x330(%rbp)
140342f66: 48 c7 85 38 03 00 00 00 00 00 00    	movq	$0x0, 0x338(%rbp)
140342f71: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
140342f78: 48 89 f1                    	movq	%rsi, %rcx
140342f7b: e8 c0 12 dd ff              	callq	0x140114240 <.text+0x113240>
140342f80: 48 89 c6                    	movq	%rax, %rsi
140342f83: e8 b8 a7 a0 00              	callq	0x140d4d740 <.text+0xd4c740>
140342f88: 48 89 c7                    	movq	%rax, %rdi
140342f8b: f6 00 01                    	testb	$0x1, (%rax)
140342f8e: 74 11                       	je	0x140342fa1 <.text+0x341fa1>
140342f90: 48 89 f9                    	movq	%rdi, %rcx
140342f93: 48 83 c1 08                 	addq	$0x8, %rcx
140342f97: 48 8b 47 08                 	movq	0x8(%rdi), %rax
140342f9b: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
140342f9f: eb 1b                       	jmp	0x140342fbc <.text+0x341fbc>
140342fa1: e8 5a 1e a2 00              	callq	0x140d64e00 <.text+0xd63e00>
140342fa6: 48 89 f9                    	movq	%rdi, %rcx
140342fa9: 48 83 c1 08                 	addq	$0x8, %rcx
140342fad: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
140342fb4: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
140342fb8: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
140342fbc: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
140342fc0: 4c 89 01                    	movq	%r8, (%rcx)
140342fc3: 0f 10 35 96 fe 67 02        	movups	0x267fe96(%rip), %xmm6  # 0x1429c2e60
140342fca: 0f 29 b5 c0 05 00 00        	movaps	%xmm6, 0x5c0(%rbp)
140342fd1: 0f 10 3d 78 fe 67 02        	movups	0x267fe78(%rip), %xmm7  # 0x1429c2e50
140342fd8: 0f 29 bd b0 05 00 00        	movaps	%xmm7, 0x5b0(%rbp)
140342fdf: 48 89 85 d0 05 00 00        	movq	%rax, 0x5d0(%rbp)
140342fe6: 48 89 95 d8 05 00 00        	movq	%rdx, 0x5d8(%rbp)
140342fed: e8 4e a7 a0 00              	callq	0x140d4d740 <.text+0xd4c740>
140342ff2: 48 89 c7                    	movq	%rax, %rdi
140342ff5: f6 00 01                    	testb	$0x1, (%rax)
140342ff8: 74 11                       	je	0x14034300b <.text+0x34200b>
140342ffa: 48 89 f9                    	movq	%rdi, %rcx
140342ffd: 48 83 c1 08                 	addq	$0x8, %rcx
140343001: 48 8b 47 08                 	movq	0x8(%rdi), %rax
140343005: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
140343009: eb 1b                       	jmp	0x140343026 <.text+0x342026>
14034300b: e8 f0 1d a2 00              	callq	0x140d64e00 <.text+0xd63e00>
140343010: 48 89 f9                    	movq	%rdi, %rcx
140343013: 48 83 c1 08                 	addq	$0x8, %rcx
140343017: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
14034301e: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
140343022: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
140343026: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
14034302a: 4c 89 01                    	movq	%r8, (%rcx)
14034302d: 0f 29 b5 00 01 00 00        	movaps	%xmm6, 0x100(%rbp)
140343034: 0f 29 bd f0 00 00 00        	movaps	%xmm7, 0xf0(%rbp)
14034303b: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
140343042: 48 89 95 18 01 00 00        	movq	%rdx, 0x118(%rbp)
140343049: e8 f2 a6 a0 00              	callq	0x140d4d740 <.text+0xd4c740>
14034304e: 48 89 c7                    	movq	%rax, %rdi
140343051: f6 00 01                    	testb	$0x1, (%rax)
140343054: 74 11                       	je	0x140343067 <.text+0x342067>
140343056: 48 89 f9                    	movq	%rdi, %rcx
140343059: 48 83 c1 08                 	addq	$0x8, %rcx
14034305d: 48 8b 47 08                 	movq	0x8(%rdi), %rax
140343061: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
140343065: eb 1b                       	jmp	0x140343082 <.text+0x342082>
140343067: e8 94 1d a2 00              	callq	0x140d64e00 <.text+0xd63e00>
14034306c: 48 89 f9                    	movq	%rdi, %rcx
14034306f: 48 83 c1 08                 	addq	$0x8, %rcx
140343073: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
14034307a: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
14034307e: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
140343082: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
140343086: 4c 89 01                    	movq	%r8, (%rcx)
140343089: 0f 28 85 b0 05 00 00        	movaps	0x5b0(%rbp), %xmm0
140343090: 0f 28 8d c0 05 00 00        	movaps	0x5c0(%rbp), %xmm1
140343097: 0f 28 95 d0 05 00 00        	movaps	0x5d0(%rbp), %xmm2
14034309e: 0f 29 95 40 03 00 00        	movaps	%xmm2, 0x340(%rbp)
1403430a5: 0f 29 8d 30 03 00 00        	movaps	%xmm1, 0x330(%rbp)
1403430ac: 0f 29 85 20 03 00 00        	movaps	%xmm0, 0x320(%rbp)
1403430b3: 48 c7 85 b0 03 00 00 00 00 00 00    	movq	$0x0, 0x3b0(%rbp)
1403430be: 48 c7 85 40 04 00 00 00 00 00 00    	movq	$0x0, 0x440(%rbp)
1403430c9: 0f 28 85 f0 00 00 00        	movaps	0xf0(%rbp), %xmm0
1403430d0: 0f 28 8d 00 01 00 00        	movaps	0x100(%rbp), %xmm1
1403430d7: 0f 28 95 10 01 00 00        	movaps	0x110(%rbp), %xmm2
1403430de: 0f 29 85 50 03 00 00        	movaps	%xmm0, 0x350(%rbp)
1403430e5: 0f 29 8d 60 03 00 00        	movaps	%xmm1, 0x360(%rbp)
1403430ec: 0f 29 95 70 03 00 00        	movaps	%xmm2, 0x370(%rbp)
1403430f3: 0f 29 bd 80 03 00 00        	movaps	%xmm7, 0x380(%rbp)
1403430fa: 0f 29 b5 90 03 00 00        	movaps	%xmm6, 0x390(%rbp)
140343101: 48 89 85 a0 03 00 00        	movq	%rax, 0x3a0(%rbp)
140343108: 48 89 95 a8 03 00 00        	movq	%rdx, 0x3a8(%rbp)
14034310f: c6 85 d0 04 00 00 00        	movb	$0x0, 0x4d0(%rbp)
140343116: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
14034311d: 48 89 f1                    	movq	%rsi, %rcx
140343120: e8 1b aa df ff              	callq	0x14013db40 <.text+0x13cb40>
140343125: 48 89 c6                    	movq	%rax, %rsi
140343128: 48 8d bd 20 03 00 00        	leaq	0x320(%rbp), %rdi
14034312f: 48 89 f9                    	movq	%rdi, %rcx
140343132: e8 39 e2 ff ff              	callq	0x140341370 <.text+0x340370>
140343137: 48 89 f1                    	movq	%rsi, %rcx
14034313a: 48 89 fa                    	movq	%rdi, %rdx
14034313d: e8 3e 78 dd ff              	callq	0x14011a980 <.text+0x119980>
140343142: 48 89 c6                    	movq	%rax, %rsi
140343145: 31 c9                       	xorl	%ecx, %ecx
140343147: e8 d4 69 d9 ff              	callq	0x1400d9b20 <.text+0xd8b20>
14034314c: 48 85 c0                    	testq	%rax, %rax
14034314f: 0f 84 6d 22 00 00           	je	0x1403453c2 <.text+0x3443c2>
140343155: 48 c7 85 20 03 00 00 00 00 00 00    	movq	$0x0, 0x320(%rbp)
140343160: 48 c7 85 28 03 00 00 08 00 00 00    	movq	$0x8, 0x328(%rbp)
14034316b: 48 c7 85 30 03 00 00 00 00 00 00    	movq	$0x0, 0x330(%rbp)
140343176: 0f 10 00                    	movups	(%rax), %xmm0
140343179: 48 ff 00                    	incq	(%rax)
14034317c: 44 0f 10 05 ac ce 66 02     	movups	0x266ceac(%rip), %xmm8  # 0x1429b0030
140343184: 44 0f 11 85 48 03 00 00     	movups	%xmm8, 0x348(%rbp)
14034318c: 44 0f 10 0d 8c ce 66 02     	movups	0x266ce8c(%rip), %xmm9  # 0x1429b0020
140343194: 44 0f 11 8d 38 03 00 00     	movups	%xmm9, 0x338(%rbp)
14034319c: 0f 11 85 58 03 00 00        	movups	%xmm0, 0x358(%rbp)
1403431a3: c6 85 68 03 00 00 00        	movb	$0x0, 0x368(%rbp)
1403431aa: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
1403431b1: 48 89 f1                    	movq	%rsi, %rcx
1403431b4: e8 b7 19 df ff              	callq	0x140134b70 <.text+0x133b70>
1403431b9: 48 89 c6                    	movq	%rax, %rsi
1403431bc: e8 7f a5 a0 00              	callq	0x140d4d740 <.text+0xd4c740>
1403431c1: 48 89 c7                    	movq	%rax, %rdi
1403431c4: f6 00 01                    	testb	$0x1, (%rax)
1403431c7: 74 11                       	je	0x1403431da <.text+0x3421da>
1403431c9: 48 89 f9                    	movq	%rdi, %rcx
1403431cc: 48 83 c1 08                 	addq	$0x8, %rcx
1403431d0: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1403431d4: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
1403431d8: eb 1b                       	jmp	0x1403431f5 <.text+0x3421f5>
1403431da: e8 21 1c a2 00              	callq	0x140d64e00 <.text+0xd63e00>
1403431df: 48 89 f9                    	movq	%rdi, %rcx
1403431e2: 48 83 c1 08                 	addq	$0x8, %rcx
1403431e6: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
1403431ed: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
1403431f1: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
1403431f5: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
1403431f9: 4c 89 01                    	movq	%r8, (%rcx)
1403431fc: 0f 29 b5 00 01 00 00        	movaps	%xmm6, 0x100(%rbp)
140343203: 0f 29 bd f0 00 00 00        	movaps	%xmm7, 0xf0(%rbp)
14034320a: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
140343211: 48 89 95 18 01 00 00        	movq	%rdx, 0x118(%rbp)
140343218: e8 23 a5 a0 00              	callq	0x140d4d740 <.text+0xd4c740>
14034321d: 48 89 c7                    	movq	%rax, %rdi
140343220: f6 00 01                    	testb	$0x1, (%rax)
140343223: 74 11                       	je	0x140343236 <.text+0x342236>
140343225: 48 89 f9                    	movq	%rdi, %rcx
140343228: 48 83 c1 08                 	addq	$0x8, %rcx
14034322c: 48 8b 47 08                 	movq	0x8(%rdi), %rax
140343230: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
140343234: eb 1b                       	jmp	0x140343251 <.text+0x342251>
140343236: e8 c5 1b a2 00              	callq	0x140d64e00 <.text+0xd63e00>
14034323b: 48 89 f9                    	movq	%rdi, %rcx
14034323e: 48 83 c1 08                 	addq	$0x8, %rcx
140343242: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
140343249: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
14034324d: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
140343251: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
140343255: 4c 89 01                    	movq	%r8, (%rcx)
140343258: 0f 28 85 f0 00 00 00        	movaps	0xf0(%rbp), %xmm0
14034325f: 0f 28 8d 00 01 00 00        	movaps	0x100(%rbp), %xmm1
140343266: 0f 28 95 10 01 00 00        	movaps	0x110(%rbp), %xmm2
14034326d: 0f 11 95 58 03 00 00        	movups	%xmm2, 0x358(%rbp)
140343274: 0f 11 8d 48 03 00 00        	movups	%xmm1, 0x348(%rbp)
14034327b: 0f 11 85 38 03 00 00        	movups	%xmm0, 0x338(%rbp)
140343282: 0f 11 bd 68 03 00 00        	movups	%xmm7, 0x368(%rbp)
140343289: 0f 11 b5 78 03 00 00        	movups	%xmm6, 0x378(%rbp)
140343290: 48 89 85 88 03 00 00        	movq	%rax, 0x388(%rbp)
140343297: 48 89 95 90 03 00 00        	movq	%rdx, 0x390(%rbp)
14034329e: 48 c7 85 20 03 00 00 00 00 00 00    	movq	$0x0, 0x320(%rbp)
1403432a9: 48 c7 85 28 03 00 00 10 00 00 00    	movq	$0x10, 0x328(%rbp)
1403432b4: 48 c7 85 30 03 00 00 00 00 00 00    	movq	$0x0, 0x330(%rbp)
1403432bf: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
1403432c6: 48 89 f1                    	movq	%rsi, %rcx
1403432c9: e8 02 9d df ff              	callq	0x14013cfd0 <.text+0x13bfd0>
1403432ce: 48 89 c6                    	movq	%rax, %rsi
1403432d1: e8 6a a4 a0 00              	callq	0x140d4d740 <.text+0xd4c740>
1403432d6: 48 89 c7                    	movq	%rax, %rdi
1403432d9: f6 00 01                    	testb	$0x1, (%rax)
1403432dc: 74 11                       	je	0x1403432ef <.text+0x3422ef>
1403432de: 48 89 f9                    	movq	%rdi, %rcx
1403432e1: 48 83 c1 08                 	addq	$0x8, %rcx
1403432e5: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1403432e9: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
1403432ed: eb 1b                       	jmp	0x14034330a <.text+0x34230a>
1403432ef: e8 0c 1b a2 00              	callq	0x140d64e00 <.text+0xd63e00>
1403432f4: 48 89 f9                    	movq	%rdi, %rcx
1403432f7: 48 83 c1 08                 	addq	$0x8, %rcx
1403432fb: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
140343302: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
140343306: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
14034330a: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
14034330e: 4c 89 01                    	movq	%r8, (%rcx)
140343311: 0f 29 b5 30 03 00 00        	movaps	%xmm6, 0x330(%rbp)
140343318: 0f 29 bd 20 03 00 00        	movaps	%xmm7, 0x320(%rbp)
14034331f: 48 89 85 40 03 00 00        	movq	%rax, 0x340(%rbp)
140343326: 48 89 95 48 03 00 00        	movq	%rdx, 0x348(%rbp)
14034332d: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
140343334: 48 89 f1                    	movq	%rsi, %rcx
140343337: e8 64 bb de ff              	callq	0x14012eea0 <.text+0x12dea0>
14034333c: c7 85 20 03 00 00 00 00 00 00       	movl	$0x0, 0x320(%rbp)
140343346: c7 85 30 03 00 00 00 00 00 00       	movl	$0x0, 0x330(%rbp)
140343350: 48 c7 85 40 03 00 00 00 00 00 00    	movq	$0x0, 0x340(%rbp)
14034335b: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
140343362: 48 89 c1                    	movq	%rax, %rcx
140343365: e8 96 2e de ff              	callq	0x140126200 <.text+0x125200>
14034336a: 48 89 c6                    	movq	%rax, %rsi
14034336d: e8 fe ef cb ff              	callq	0x140002370 <.text+0x1370>
140343372: 48 89 f1                    	movq	%rsi, %rcx
140343375: 89 c2                       	movl	%eax, %edx
140343377: e8 44 85 df ff              	callq	0x14013b8c0 <.text+0x13a8c0>
14034337c: c7 85 20 03 00 00 00 00 00 00       	movl	$0x0, 0x320(%rbp)
140343386: 48 c7 85 28 03 00 00 00 00 00 00    	movq	$0x0, 0x328(%rbp)
140343391: c7 85 2f 03 00 00 00 00 00 00       	movl	$0x0, 0x32f(%rbp)
14034339b: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
1403433a2: 48 89 c1                    	movq	%rax, %rcx
1403433a5: e8 26 40 df ff              	callq	0x1401373d0 <.text+0x1363d0>
1403433aa: ba 03 00 00 00              	movl	$0x3, %edx
1403433af: 48 89 c1                    	movq	%rax, %rcx
1403433b2: e8 c9 3c dd ff              	callq	0x140117080 <.text+0x116080>
1403433b7: 48 89 c6                    	movq	%rax, %rsi
1403433ba: e8 81 a3 a0 00              	callq	0x140d4d740 <.text+0xd4c740>
1403433bf: 48 89 c7                    	movq	%rax, %rdi
1403433c2: f6 00 01                    	testb	$0x1, (%rax)
1403433c5: 74 11                       	je	0x1403433d8 <.text+0x3423d8>
1403433c7: 48 89 f9                    	movq	%rdi, %rcx
1403433ca: 48 83 c1 08                 	addq	$0x8, %rcx
1403433ce: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1403433d2: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
1403433d6: eb 1b                       	jmp	0x1403433f3 <.text+0x3423f3>
1403433d8: e8 23 1a a2 00              	callq	0x140d64e00 <.text+0xd63e00>
1403433dd: 48 89 f9                    	movq	%rdi, %rcx
1403433e0: 48 83 c1 08                 	addq	$0x8, %rcx
1403433e4: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
1403433eb: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
1403433ef: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
1403433f3: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
1403433f7: 4c 89 01                    	movq	%r8, (%rcx)
1403433fa: 0f 29 b5 c0 02 00 00        	movaps	%xmm6, 0x2c0(%rbp)
140343401: 0f 29 bd b0 02 00 00        	movaps	%xmm7, 0x2b0(%rbp)
140343408: 48 89 85 d0 02 00 00        	movq	%rax, 0x2d0(%rbp)
14034340f: 48 89 95 d8 02 00 00        	movq	%rdx, 0x2d8(%rbp)
140343416: e8 25 a3 a0 00              	callq	0x140d4d740 <.text+0xd4c740>
14034341b: 48 89 c7                    	movq	%rax, %rdi
14034341e: f6 00 01                    	testb	$0x1, (%rax)
140343421: 74 11                       	je	0x140343434 <.text+0x342434>
140343423: 48 89 f9                    	movq	%rdi, %rcx
140343426: 48 83 c1 08                 	addq	$0x8, %rcx
14034342a: 48 8b 47 08                 	movq	0x8(%rdi), %rax
14034342e: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
140343432: eb 1b                       	jmp	0x14034344f <.text+0x34244f>
140343434: e8 c7 19 a2 00              	callq	0x140d64e00 <.text+0xd63e00>
140343439: 48 89 f9                    	movq	%rdi, %rcx
14034343c: 48 83 c1 08                 	addq	$0x8, %rcx
140343440: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
140343447: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
14034344b: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
14034344f: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
140343453: 4c 89 01                    	movq	%r8, (%rcx)
140343456: 0f 29 b5 c0 05 00 00        	movaps	%xmm6, 0x5c0(%rbp)
14034345d: 0f 29 bd b0 05 00 00        	movaps	%xmm7, 0x5b0(%rbp)
140343464: 48 89 85 d0 05 00 00        	movq	%rax, 0x5d0(%rbp)
14034346b: 48 89 95 d8 05 00 00        	movq	%rdx, 0x5d8(%rbp)
140343472: e8 c9 a2 a0 00              	callq	0x140d4d740 <.text+0xd4c740>
140343477: 48 89 c7                    	movq	%rax, %rdi
14034347a: f6 00 01                    	testb	$0x1, (%rax)
14034347d: 74 11                       	je	0x140343490 <.text+0x342490>
14034347f: 48 89 f9                    	movq	%rdi, %rcx
140343482: 48 83 c1 08                 	addq	$0x8, %rcx
140343486: 48 8b 47 08                 	movq	0x8(%rdi), %rax
14034348a: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
14034348e: eb 1b                       	jmp	0x1403434ab <.text+0x3424ab>
140343490: e8 6b 19 a2 00              	callq	0x140d64e00 <.text+0xd63e00>
140343495: 48 89 f9                    	movq	%rdi, %rcx
140343498: 48 83 c1 08                 	addq	$0x8, %rcx
14034349c: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
1403434a3: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
1403434a7: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
1403434ab: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
1403434af: 4c 89 01                    	movq	%r8, (%rcx)
1403434b2: 0f 29 b5 00 01 00 00        	movaps	%xmm6, 0x100(%rbp)
1403434b9: 0f 29 bd f0 00 00 00        	movaps	%xmm7, 0xf0(%rbp)
1403434c0: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
1403434c7: 48 89 95 18 01 00 00        	movq	%rdx, 0x118(%rbp)
1403434ce: e8 6d a2 a0 00              	callq	0x140d4d740 <.text+0xd4c740>
1403434d3: 48 89 c7                    	movq	%rax, %rdi
1403434d6: f6 00 01                    	testb	$0x1, (%rax)
1403434d9: 74 11                       	je	0x1403434ec <.text+0x3424ec>
1403434db: 48 89 f9                    	movq	%rdi, %rcx
1403434de: 48 83 c1 08                 	addq	$0x8, %rcx
1403434e2: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1403434e6: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
1403434ea: eb 1b                       	jmp	0x140343507 <.text+0x342507>
1403434ec: e8 0f 19 a2 00              	callq	0x140d64e00 <.text+0xd63e00>
1403434f1: 48 89 f9                    	movq	%rdi, %rcx
1403434f4: 48 83 c1 08                 	addq	$0x8, %rcx
1403434f8: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
1403434ff: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
140343503: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
140343507: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
14034350b: 4c 89 01                    	movq	%r8, (%rcx)
14034350e: 0f 28 85 b0 02 00 00        	movaps	0x2b0(%rbp), %xmm0
140343515: 0f 28 8d c0 02 00 00        	movaps	0x2c0(%rbp), %xmm1
14034351c: 0f 28 95 d0 02 00 00        	movaps	0x2d0(%rbp), %xmm2
140343523: 0f 29 95 40 03 00 00        	movaps	%xmm2, 0x340(%rbp)
14034352a: 0f 29 8d 30 03 00 00        	movaps	%xmm1, 0x330(%rbp)
140343531: 0f 29 85 20 03 00 00        	movaps	%xmm0, 0x320(%rbp)
140343538: 0f 28 85 b0 05 00 00        	movaps	0x5b0(%rbp), %xmm0
14034353f: 0f 28 8d c0 05 00 00        	movaps	0x5c0(%rbp), %xmm1
140343546: 0f 28 95 d0 05 00 00        	movaps	0x5d0(%rbp), %xmm2
14034354d: 0f 29 85 50 03 00 00        	movaps	%xmm0, 0x350(%rbp)
140343554: 0f 29 8d 60 03 00 00        	movaps	%xmm1, 0x360(%rbp)
14034355b: 0f 29 95 70 03 00 00        	movaps	%xmm2, 0x370(%rbp)
140343562: 0f 28 85 f0 00 00 00        	movaps	0xf0(%rbp), %xmm0
140343569: 0f 28 8d 00 01 00 00        	movaps	0x100(%rbp), %xmm1
140343570: 0f 28 95 10 01 00 00        	movaps	0x110(%rbp), %xmm2
140343577: 0f 29 85 80 03 00 00        	movaps	%xmm0, 0x380(%rbp)
14034357e: 0f 29 8d 90 03 00 00        	movaps	%xmm1, 0x390(%rbp)
140343585: 0f 29 95 a0 03 00 00        	movaps	%xmm2, 0x3a0(%rbp)
14034358c: 0f 29 b5 c0 03 00 00        	movaps	%xmm6, 0x3c0(%rbp)
140343593: 0f 29 bd b0 03 00 00        	movaps	%xmm7, 0x3b0(%rbp)
14034359a: 48 89 85 d0 03 00 00        	movq	%rax, 0x3d0(%rbp)
1403435a1: 48 89 95 d8 03 00 00        	movq	%rdx, 0x3d8(%rbp)
1403435a8: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
1403435af: 48 89 f1                    	movq	%rsi, %rcx
1403435b2: e8 69 b1 dd ff              	callq	0x14011e720 <.text+0x11d720>
1403435b7: 48 89 c6                    	movq	%rax, %rsi
1403435ba: e8 81 a1 a0 00              	callq	0x140d4d740 <.text+0xd4c740>
1403435bf: 48 89 c7                    	movq	%rax, %rdi
1403435c2: f6 00 01                    	testb	$0x1, (%rax)
1403435c5: 74 11                       	je	0x1403435d8 <.text+0x3425d8>
1403435c7: 48 89 f9                    	movq	%rdi, %rcx
1403435ca: 48 83 c1 08                 	addq	$0x8, %rcx
1403435ce: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1403435d2: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
1403435d6: eb 1b                       	jmp	0x1403435f3 <.text+0x3425f3>
1403435d8: e8 23 18 a2 00              	callq	0x140d64e00 <.text+0xd63e00>
1403435dd: 48 89 f9                    	movq	%rdi, %rcx
1403435e0: 48 83 c1 08                 	addq	$0x8, %rcx
1403435e4: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
1403435eb: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
1403435ef: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
1403435f3: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
1403435f7: 4c 89 01                    	movq	%r8, (%rcx)
1403435fa: c6 85 50 03 00 00 00        	movb	$0x0, 0x350(%rbp)
140343601: 0f 29 bd 20 03 00 00        	movaps	%xmm7, 0x320(%rbp)
140343608: 0f 29 b5 30 03 00 00        	movaps	%xmm6, 0x330(%rbp)
14034360f: 48 89 85 40 03 00 00        	movq	%rax, 0x340(%rbp)
140343616: 48 89 95 48 03 00 00        	movq	%rdx, 0x348(%rbp)
14034361d: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
140343624: 48 89 f1                    	movq	%rsi, %rcx
140343627: e8 f4 f2 de ff              	callq	0x140132920 <.text+0x131920>
14034362c: 48 c7 85 28 03 00 00 00 00 00 00    	movq	$0x0, 0x328(%rbp)
140343637: 48 c7 85 20 03 00 00 06 00 00 00    	movq	$0x6, 0x320(%rbp)
140343642: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
140343649: 48 89 c1                    	movq	%rax, %rcx
14034364c: e8 3f 60 df ff              	callq	0x140139690 <.text+0x138690>
140343651: 48 89 c6                    	movq	%rax, %rsi
140343654: e8 e7 a0 a0 00              	callq	0x140d4d740 <.text+0xd4c740>
140343659: 48 89 c7                    	movq	%rax, %rdi
14034365c: f6 00 01                    	testb	$0x1, (%rax)
14034365f: 74 11                       	je	0x140343672 <.text+0x342672>
140343661: 48 89 f9                    	movq	%rdi, %rcx
140343664: 48 83 c1 08                 	addq	$0x8, %rcx
140343668: 48 8b 47 08                 	movq	0x8(%rdi), %rax
14034366c: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
140343670: eb 1b                       	jmp	0x14034368d <.text+0x34268d>
140343672: e8 89 17 a2 00              	callq	0x140d64e00 <.text+0xd63e00>
140343677: 48 89 f9                    	movq	%rdi, %rcx
14034367a: 48 83 c1 08                 	addq	$0x8, %rcx
14034367e: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
140343685: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
140343689: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
14034368d: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
140343691: 4c 89 01                    	movq	%r8, (%rcx)
140343694: 0f 29 b5 30 03 00 00        	movaps	%xmm6, 0x330(%rbp)
14034369b: 0f 29 bd 20 03 00 00        	movaps	%xmm7, 0x320(%rbp)
1403436a2: 48 89 85 40 03 00 00        	movq	%rax, 0x340(%rbp)
1403436a9: 48 89 95 48 03 00 00        	movq	%rdx, 0x348(%rbp)
1403436b0: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
1403436b7: 48 89 f1                    	movq	%rsi, %rcx
1403436ba: e8 b1 e0 de ff              	callq	0x140131770 <.text+0x130770>
1403436bf: 48 89 c6                    	movq	%rax, %rsi
1403436c2: e8 79 a0 a0 00              	callq	0x140d4d740 <.text+0xd4c740>
1403436c7: 48 89 c7                    	movq	%rax, %rdi
1403436ca: f6 00 01                    	testb	$0x1, (%rax)
1403436cd: 74 11                       	je	0x1403436e0 <.text+0x3426e0>
1403436cf: 48 89 f9                    	movq	%rdi, %rcx
1403436d2: 48 83 c1 08                 	addq	$0x8, %rcx
1403436d6: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1403436da: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
1403436de: eb 1b                       	jmp	0x1403436fb <.text+0x3426fb>
1403436e0: e8 1b 17 a2 00              	callq	0x140d64e00 <.text+0xd63e00>
1403436e5: 48 89 f9                    	movq	%rdi, %rcx
1403436e8: 48 83 c1 08                 	addq	$0x8, %rcx
1403436ec: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
1403436f3: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
1403436f7: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
1403436fb: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
1403436ff: 4c 89 01                    	movq	%r8, (%rcx)
140343702: 0f 29 b5 c0 02 00 00        	movaps	%xmm6, 0x2c0(%rbp)
140343709: 0f 29 bd b0 02 00 00        	movaps	%xmm7, 0x2b0(%rbp)
140343710: 48 89 85 d0 02 00 00        	movq	%rax, 0x2d0(%rbp)
140343717: 48 89 95 d8 02 00 00        	movq	%rdx, 0x2d8(%rbp)
14034371e: e8 1d a0 a0 00              	callq	0x140d4d740 <.text+0xd4c740>
140343723: 48 89 c7                    	movq	%rax, %rdi
140343726: f6 00 01                    	testb	$0x1, (%rax)
140343729: 74 11                       	je	0x14034373c <.text+0x34273c>
14034372b: 48 89 f9                    	movq	%rdi, %rcx
14034372e: 48 83 c1 08                 	addq	$0x8, %rcx
140343732: 48 8b 47 08                 	movq	0x8(%rdi), %rax
140343736: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
14034373a: eb 1b                       	jmp	0x140343757 <.text+0x342757>
14034373c: e8 bf 16 a2 00              	callq	0x140d64e00 <.text+0xd63e00>
140343741: 48 89 f9                    	movq	%rdi, %rcx
140343744: 48 83 c1 08                 	addq	$0x8, %rcx
140343748: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
14034374f: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
140343753: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
140343757: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
14034375b: 4c 89 01                    	movq	%r8, (%rcx)
14034375e: 0f 29 b5 c0 05 00 00        	movaps	%xmm6, 0x5c0(%rbp)
140343765: 0f 29 bd b0 05 00 00        	movaps	%xmm7, 0x5b0(%rbp)
14034376c: 48 89 85 d0 05 00 00        	movq	%rax, 0x5d0(%rbp)
140343773: 48 89 95 d8 05 00 00        	movq	%rdx, 0x5d8(%rbp)
14034377a: e8 c1 9f a0 00              	callq	0x140d4d740 <.text+0xd4c740>
14034377f: 48 89 c7                    	movq	%rax, %rdi
140343782: f6 00 01                    	testb	$0x1, (%rax)
140343785: 74 11                       	je	0x140343798 <.text+0x342798>
140343787: 48 89 f9                    	movq	%rdi, %rcx
14034378a: 48 83 c1 08                 	addq	$0x8, %rcx
14034378e: 48 8b 47 08                 	movq	0x8(%rdi), %rax
140343792: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
140343796: eb 1b                       	jmp	0x1403437b3 <.text+0x3427b3>
140343798: e8 63 16 a2 00              	callq	0x140d64e00 <.text+0xd63e00>
14034379d: 48 89 f9                    	movq	%rdi, %rcx
1403437a0: 48 83 c1 08                 	addq	$0x8, %rcx
1403437a4: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
1403437ab: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
1403437af: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
1403437b3: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
1403437b7: 4c 89 01                    	movq	%r8, (%rcx)
1403437ba: 0f 29 b5 00 01 00 00        	movaps	%xmm6, 0x100(%rbp)
1403437c1: 0f 29 bd f0 00 00 00        	movaps	%xmm7, 0xf0(%rbp)
1403437c8: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
1403437cf: 48 89 95 18 01 00 00        	movq	%rdx, 0x118(%rbp)
1403437d6: e8 65 9f a0 00              	callq	0x140d4d740 <.text+0xd4c740>
1403437db: 48 89 c7                    	movq	%rax, %rdi
1403437de: f6 00 01                    	testb	$0x1, (%rax)
1403437e1: 74 11                       	je	0x1403437f4 <.text+0x3427f4>
1403437e3: 48 89 f9                    	movq	%rdi, %rcx
1403437e6: 48 83 c1 08                 	addq	$0x8, %rcx
1403437ea: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1403437ee: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
1403437f2: eb 1b                       	jmp	0x14034380f <.text+0x34280f>
1403437f4: e8 07 16 a2 00              	callq	0x140d64e00 <.text+0xd63e00>
1403437f9: 48 89 f9                    	movq	%rdi, %rcx
1403437fc: 48 83 c1 08                 	addq	$0x8, %rcx
140343800: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
140343807: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
14034380b: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
14034380f: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
140343813: 4c 89 01                    	movq	%r8, (%rcx)
140343816: 0f 28 85 b0 02 00 00        	movaps	0x2b0(%rbp), %xmm0
14034381d: 0f 28 8d c0 02 00 00        	movaps	0x2c0(%rbp), %xmm1
140343824: 0f 28 95 d0 02 00 00        	movaps	0x2d0(%rbp), %xmm2
14034382b: 0f 29 95 40 03 00 00        	movaps	%xmm2, 0x340(%rbp)
140343832: 0f 29 8d 30 03 00 00        	movaps	%xmm1, 0x330(%rbp)
140343839: 0f 29 85 20 03 00 00        	movaps	%xmm0, 0x320(%rbp)
140343840: 0f 28 85 b0 05 00 00        	movaps	0x5b0(%rbp), %xmm0
140343847: 0f 28 8d c0 05 00 00        	movaps	0x5c0(%rbp), %xmm1
14034384e: 0f 28 95 d0 05 00 00        	movaps	0x5d0(%rbp), %xmm2
140343855: 0f 29 85 50 03 00 00        	movaps	%xmm0, 0x350(%rbp)
14034385c: 0f 29 8d 60 03 00 00        	movaps	%xmm1, 0x360(%rbp)
140343863: 0f 29 95 70 03 00 00        	movaps	%xmm2, 0x370(%rbp)
14034386a: 0f 28 85 f0 00 00 00        	movaps	0xf0(%rbp), %xmm0
140343871: 0f 28 8d 00 01 00 00        	movaps	0x100(%rbp), %xmm1
140343878: 0f 28 95 10 01 00 00        	movaps	0x110(%rbp), %xmm2
14034387f: 0f 29 85 80 03 00 00        	movaps	%xmm0, 0x380(%rbp)
140343886: 0f 29 8d 90 03 00 00        	movaps	%xmm1, 0x390(%rbp)
14034388d: 0f 29 95 a0 03 00 00        	movaps	%xmm2, 0x3a0(%rbp)
140343894: 0f 29 b5 c0 03 00 00        	movaps	%xmm6, 0x3c0(%rbp)
14034389b: 0f 29 bd b0 03 00 00        	movaps	%xmm7, 0x3b0(%rbp)
1403438a2: 48 89 85 d0 03 00 00        	movq	%rax, 0x3d0(%rbp)
1403438a9: 48 89 95 d8 03 00 00        	movq	%rdx, 0x3d8(%rbp)
1403438b0: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
1403438b7: 48 89 f1                    	movq	%rsi, %rcx
1403438ba: e8 11 c7 de ff              	callq	0x14012ffd0 <.text+0x12efd0>
1403438bf: 48 89 c6                    	movq	%rax, %rsi
1403438c2: e8 79 9e a0 00              	callq	0x140d4d740 <.text+0xd4c740>
1403438c7: 48 89 c7                    	movq	%rax, %rdi
1403438ca: f6 00 01                    	testb	$0x1, (%rax)
1403438cd: 74 11                       	je	0x1403438e0 <.text+0x3428e0>
1403438cf: 48 89 f9                    	movq	%rdi, %rcx
1403438d2: 48 83 c1 08                 	addq	$0x8, %rcx
1403438d6: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1403438da: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
1403438de: eb 1b                       	jmp	0x1403438fb <.text+0x3428fb>
1403438e0: e8 1b 15 a2 00              	callq	0x140d64e00 <.text+0xd63e00>
1403438e5: 48 89 f9                    	movq	%rdi, %rcx
1403438e8: 48 83 c1 08                 	addq	$0x8, %rcx
1403438ec: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
1403438f3: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
1403438f7: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
1403438fb: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
1403438ff: 4c 89 01                    	movq	%r8, (%rcx)
140343902: 0f 29 b5 30 03 00 00        	movaps	%xmm6, 0x330(%rbp)
140343909: 0f 29 bd 20 03 00 00        	movaps	%xmm7, 0x320(%rbp)
140343910: 48 89 85 40 03 00 00        	movq	%rax, 0x340(%rbp)
140343917: 48 89 95 48 03 00 00        	movq	%rdx, 0x348(%rbp)
14034391e: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
140343925: 48 89 f1                    	movq	%rsi, %rcx
140343928: e8 03 07 df ff              	callq	0x140134030 <.text+0x133030>
14034392d: 48 89 c6                    	movq	%rax, %rsi
140343930: e8 0b 9e a0 00              	callq	0x140d4d740 <.text+0xd4c740>
140343935: 48 89 c7                    	movq	%rax, %rdi
140343938: f6 00 01                    	testb	$0x1, (%rax)
14034393b: 74 11                       	je	0x14034394e <.text+0x34294e>
14034393d: 48 89 f9                    	movq	%rdi, %rcx
140343940: 48 83 c1 08                 	addq	$0x8, %rcx
140343944: 48 8b 47 08                 	movq	0x8(%rdi), %rax
140343948: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
14034394c: eb 1b                       	jmp	0x140343969 <.text+0x342969>
14034394e: e8 ad 14 a2 00              	callq	0x140d64e00 <.text+0xd63e00>
140343953: 48 89 f9                    	movq	%rdi, %rcx
140343956: 48 83 c1 08                 	addq	$0x8, %rcx
14034395a: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
140343961: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
140343965: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
140343969: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
14034396d: 4c 89 01                    	movq	%r8, (%rcx)
140343970: 0f 29 b5 30 03 00 00        	movaps	%xmm6, 0x330(%rbp)
140343977: 0f 29 bd 20 03 00 00        	movaps	%xmm7, 0x320(%rbp)
14034397e: 48 89 85 40 03 00 00        	movq	%rax, 0x340(%rbp)
140343985: 48 89 95 48 03 00 00        	movq	%rdx, 0x348(%rbp)
14034398c: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
140343993: 48 89 f1                    	movq	%rsi, %rcx
140343996: e8 b5 94 de ff              	callq	0x14012ce50 <.text+0x12be50>
14034399b: 48 89 c6                    	movq	%rax, %rsi
14034399e: 31 c9                       	xorl	%ecx, %ecx
1403439a0: e8 7b 61 d9 ff              	callq	0x1400d9b20 <.text+0xd8b20>
1403439a5: 48 85 c0                    	testq	%rax, %rax
1403439a8: 0f 84 14 1a 00 00           	je	0x1403453c2 <.text+0x3443c2>
1403439ae: 0f 10 00                    	movups	(%rax), %xmm0
1403439b1: 48 ff 00                    	incq	(%rax)
1403439b4: 44 0f 10 15 f4 fe 65 02     	movups	0x265fef4(%rip), %xmm10 # 0x1429a38b0
1403439bc: 44 0f 29 95 00 01 00 00     	movaps	%xmm10, 0x100(%rbp)
1403439c4: 44 0f 10 1d d4 fe 65 02     	movups	0x265fed4(%rip), %xmm11 # 0x1429a38a0
1403439cc: 44 0f 29 9d f0 00 00 00     	movaps	%xmm11, 0xf0(%rbp)
1403439d4: 0f 29 85 10 01 00 00        	movaps	%xmm0, 0x110(%rbp)
1403439db: 31 c9                       	xorl	%ecx, %ecx
1403439dd: e8 3e 61 d9 ff              	callq	0x1400d9b20 <.text+0xd8b20>
1403439e2: 48 85 c0                    	testq	%rax, %rax
1403439e5: 0f 84 5a 1a 00 00           	je	0x140345445 <.text+0x344445>
1403439eb: 0f 10 00                    	movups	(%rax), %xmm0
1403439ee: 48 ff 00                    	incq	(%rax)
1403439f1: 0f 28 8d f0 00 00 00        	movaps	0xf0(%rbp), %xmm1
1403439f8: 0f 28 95 00 01 00 00        	movaps	0x100(%rbp), %xmm2
1403439ff: 0f 28 9d 10 01 00 00        	movaps	0x110(%rbp), %xmm3
140343a06: 0f 29 9d 40 03 00 00        	movaps	%xmm3, 0x340(%rbp)
140343a0d: 0f 29 95 30 03 00 00        	movaps	%xmm2, 0x330(%rbp)
140343a14: 0f 29 8d 20 03 00 00        	movaps	%xmm1, 0x320(%rbp)
140343a1b: 44 0f 29 9d 50 03 00 00     	movaps	%xmm11, 0x350(%rbp)
140343a23: 44 0f 29 95 60 03 00 00     	movaps	%xmm10, 0x360(%rbp)
140343a2b: 0f 29 85 70 03 00 00        	movaps	%xmm0, 0x370(%rbp)
140343a32: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
140343a39: 48 89 f1                    	movq	%rsi, %rcx
140343a3c: e8 bf 83 df ff              	callq	0x14013be00 <.text+0x13ae00>
140343a41: 48 89 c6                    	movq	%rax, %rsi
140343a44: e8 f7 9c a0 00              	callq	0x140d4d740 <.text+0xd4c740>
140343a49: 48 89 c7                    	movq	%rax, %rdi
140343a4c: f6 00 01                    	testb	$0x1, (%rax)
140343a4f: 74 11                       	je	0x140343a62 <.text+0x342a62>
140343a51: 48 89 f9                    	movq	%rdi, %rcx
140343a54: 48 83 c1 08                 	addq	$0x8, %rcx
140343a58: 48 8b 47 08                 	movq	0x8(%rdi), %rax
140343a5c: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
140343a60: eb 1b                       	jmp	0x140343a7d <.text+0x342a7d>
140343a62: e8 99 13 a2 00              	callq	0x140d64e00 <.text+0xd63e00>
140343a67: 48 89 f9                    	movq	%rdi, %rcx
140343a6a: 48 83 c1 08                 	addq	$0x8, %rcx
140343a6e: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
140343a75: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
140343a79: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
140343a7d: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
140343a81: 4c 89 01                    	movq	%r8, (%rcx)
140343a84: 0f 29 b5 c0 05 00 00        	movaps	%xmm6, 0x5c0(%rbp)
140343a8b: 0f 29 bd b0 05 00 00        	movaps	%xmm7, 0x5b0(%rbp)
140343a92: 48 89 85 d0 05 00 00        	movq	%rax, 0x5d0(%rbp)
140343a99: 48 89 95 d8 05 00 00        	movq	%rdx, 0x5d8(%rbp)
140343aa0: e8 9b 9c a0 00              	callq	0x140d4d740 <.text+0xd4c740>
140343aa5: 48 89 c7                    	movq	%rax, %rdi
140343aa8: f6 00 01                    	testb	$0x1, (%rax)
140343aab: 74 11                       	je	0x140343abe <.text+0x342abe>
140343aad: 48 89 f9                    	movq	%rdi, %rcx
140343ab0: 48 83 c1 08                 	addq	$0x8, %rcx
140343ab4: 48 8b 47 08                 	movq	0x8(%rdi), %rax
140343ab8: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
140343abc: eb 1b                       	jmp	0x140343ad9 <.text+0x342ad9>
140343abe: e8 3d 13 a2 00              	callq	0x140d64e00 <.text+0xd63e00>
140343ac3: 48 89 f9                    	movq	%rdi, %rcx
140343ac6: 48 83 c1 08                 	addq	$0x8, %rcx
140343aca: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
140343ad1: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
140343ad5: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
140343ad9: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
140343add: 4c 89 01                    	movq	%r8, (%rcx)
140343ae0: 0f 29 b5 30 03 00 00        	movaps	%xmm6, 0x330(%rbp)
140343ae7: 0f 29 bd 20 03 00 00        	movaps	%xmm7, 0x320(%rbp)
140343aee: 48 89 85 40 03 00 00        	movq	%rax, 0x340(%rbp)
140343af5: 48 89 95 48 03 00 00        	movq	%rdx, 0x348(%rbp)
140343afc: e8 3f 9c a0 00              	callq	0x140d4d740 <.text+0xd4c740>
140343b01: 48 89 c7                    	movq	%rax, %rdi
140343b04: f6 00 01                    	testb	$0x1, (%rax)
140343b07: 74 11                       	je	0x140343b1a <.text+0x342b1a>
140343b09: 48 89 f9                    	movq	%rdi, %rcx
140343b0c: 48 83 c1 08                 	addq	$0x8, %rcx
140343b10: 48 8b 47 08                 	movq	0x8(%rdi), %rax
140343b14: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
140343b18: eb 1b                       	jmp	0x140343b35 <.text+0x342b35>
140343b1a: e8 e1 12 a2 00              	callq	0x140d64e00 <.text+0xd63e00>
140343b1f: 48 89 f9                    	movq	%rdi, %rcx
140343b22: 48 83 c1 08                 	addq	$0x8, %rcx
140343b26: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
140343b2d: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
140343b31: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
140343b35: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
140343b39: 4c 89 01                    	movq	%r8, (%rcx)
140343b3c: 0f 28 85 b0 05 00 00        	movaps	0x5b0(%rbp), %xmm0
140343b43: 0f 28 8d c0 05 00 00        	movaps	0x5c0(%rbp), %xmm1
140343b4a: 0f 28 95 d0 05 00 00        	movaps	0x5d0(%rbp), %xmm2
140343b51: 0f 29 95 10 01 00 00        	movaps	%xmm2, 0x110(%rbp)
140343b58: 0f 29 8d 00 01 00 00        	movaps	%xmm1, 0x100(%rbp)
140343b5f: 0f 29 85 f0 00 00 00        	movaps	%xmm0, 0xf0(%rbp)
140343b66: 48 c7 85 80 01 00 00 00 00 00 00    	movq	$0x0, 0x180(%rbp)
140343b71: 48 c7 85 10 02 00 00 00 00 00 00    	movq	$0x0, 0x210(%rbp)
140343b7c: 0f 28 85 20 03 00 00        	movaps	0x320(%rbp), %xmm0
140343b83: 0f 28 8d 30 03 00 00        	movaps	0x330(%rbp), %xmm1
140343b8a: 0f 28 95 40 03 00 00        	movaps	0x340(%rbp), %xmm2
140343b91: 0f 29 85 20 01 00 00        	movaps	%xmm0, 0x120(%rbp)
140343b98: 0f 29 8d 30 01 00 00        	movaps	%xmm1, 0x130(%rbp)
140343b9f: 0f 29 95 40 01 00 00        	movaps	%xmm2, 0x140(%rbp)
140343ba6: 0f 29 bd 50 01 00 00        	movaps	%xmm7, 0x150(%rbp)
140343bad: 0f 29 b5 60 01 00 00        	movaps	%xmm6, 0x160(%rbp)
140343bb4: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
140343bbb: 48 89 95 78 01 00 00        	movq	%rdx, 0x178(%rbp)
140343bc2: c6 85 a0 02 00 00 00        	movb	$0x0, 0x2a0(%rbp)
140343bc9: e8 72 9b a0 00              	callq	0x140d4d740 <.text+0xd4c740>
140343bce: 48 89 c7                    	movq	%rax, %rdi
140343bd1: f6 00 01                    	testb	$0x1, (%rax)
140343bd4: 74 11                       	je	0x140343be7 <.text+0x342be7>
140343bd6: 48 89 f9                    	movq	%rdi, %rcx
140343bd9: 48 83 c1 08                 	addq	$0x8, %rcx
140343bdd: 48 8b 47 08                 	movq	0x8(%rdi), %rax
140343be1: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
140343be5: eb 1b                       	jmp	0x140343c02 <.text+0x342c02>
140343be7: e8 14 12 a2 00              	callq	0x140d64e00 <.text+0xd63e00>
140343bec: 48 89 f9                    	movq	%rdi, %rcx
140343bef: 48 83 c1 08                 	addq	$0x8, %rcx
140343bf3: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
140343bfa: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
140343bfe: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
140343c02: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
140343c06: 4c 89 01                    	movq	%r8, (%rcx)
140343c09: 0f 29 b5 c0 02 00 00        	movaps	%xmm6, 0x2c0(%rbp)
140343c10: 0f 29 bd b0 02 00 00        	movaps	%xmm7, 0x2b0(%rbp)
140343c17: 48 89 85 d0 02 00 00        	movq	%rax, 0x2d0(%rbp)
140343c1e: 48 89 95 d8 02 00 00        	movq	%rdx, 0x2d8(%rbp)
140343c25: e8 16 9b a0 00              	callq	0x140d4d740 <.text+0xd4c740>
140343c2a: 48 89 c7                    	movq	%rax, %rdi
140343c2d: f6 00 01                    	testb	$0x1, (%rax)
140343c30: 74 11                       	je	0x140343c43 <.text+0x342c43>
140343c32: 48 89 f9                    	movq	%rdi, %rcx
140343c35: 48 83 c1 08                 	addq	$0x8, %rcx
140343c39: 48 8b 47 08                 	movq	0x8(%rdi), %rax
140343c3d: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
140343c41: eb 1b                       	jmp	0x140343c5e <.text+0x342c5e>
140343c43: e8 b8 11 a2 00              	callq	0x140d64e00 <.text+0xd63e00>
140343c48: 48 89 f9                    	movq	%rdi, %rcx
140343c4b: 48 83 c1 08                 	addq	$0x8, %rcx
140343c4f: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
140343c56: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
140343c5a: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
140343c5e: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
140343c62: 4c 89 01                    	movq	%r8, (%rcx)
140343c65: 0f 29 b5 c0 05 00 00        	movaps	%xmm6, 0x5c0(%rbp)
140343c6c: 0f 29 bd b0 05 00 00        	movaps	%xmm7, 0x5b0(%rbp)
140343c73: 48 89 85 d0 05 00 00        	movq	%rax, 0x5d0(%rbp)
140343c7a: 48 89 95 d8 05 00 00        	movq	%rdx, 0x5d8(%rbp)
140343c81: e8 ba 9a a0 00              	callq	0x140d4d740 <.text+0xd4c740>
140343c86: 49 89 c6                    	movq	%rax, %r14
140343c89: f6 00 01                    	testb	$0x1, (%rax)
140343c8c: 74 11                       	je	0x140343c9f <.text+0x342c9f>
140343c8e: 4c 89 f0                    	movq	%r14, %rax
140343c91: 48 83 c0 08                 	addq	$0x8, %rax
140343c95: 49 8b 7e 08                 	movq	0x8(%r14), %rdi
140343c99: 49 8b 5e 10                 	movq	0x10(%r14), %rbx
140343c9d: eb 21                       	jmp	0x140343cc0 <.text+0x342cc0>
140343c9f: e8 5c 11 a2 00              	callq	0x140d64e00 <.text+0xd63e00>
140343ca4: 48 89 c7                    	movq	%rax, %rdi
140343ca7: 48 89 d3                    	movq	%rdx, %rbx
140343caa: 4c 89 f0                    	movq	%r14, %rax
140343cad: 48 83 c0 08                 	addq	$0x8, %rax
140343cb1: 49 c7 06 01 00 00 00        	movq	$0x1, (%r14)
140343cb8: 49 89 7e 08                 	movq	%rdi, 0x8(%r14)
140343cbc: 49 89 56 10                 	movq	%rdx, 0x10(%r14)
140343cc0: 48 8d 4f 01                 	leaq	0x1(%rdi), %rcx
140343cc4: 48 89 08                    	movq	%rcx, (%rax)
140343cc7: 0f 57 c0                    	xorps	%xmm0, %xmm0
140343cca: 0f 11 85 80 05 00 00        	movups	%xmm0, 0x580(%rbp)
140343cd1: 48 8d 8d 38 03 00 00        	leaq	0x338(%rbp), %rcx
140343cd8: 48 8d 95 f0 00 00 00        	leaq	0xf0(%rbp), %rdx
140343cdf: 41 b8 b8 01 00 00           	movl	$0x1b8, %r8d            # imm = 0x1B8
140343ce5: e8 16 ff 5d 02              	callq	0x142923c00 <.text+0x2922c00>
140343cea: 0f 28 85 b0 02 00 00        	movaps	0x2b0(%rbp), %xmm0
140343cf1: 0f 28 8d c0 02 00 00        	movaps	0x2c0(%rbp), %xmm1
140343cf8: 0f 28 95 d0 02 00 00        	movaps	0x2d0(%rbp), %xmm2
140343cff: 0f 11 85 f0 04 00 00        	movups	%xmm0, 0x4f0(%rbp)
140343d06: 0f 11 8d 00 05 00 00        	movups	%xmm1, 0x500(%rbp)
140343d0d: 0f 11 95 10 05 00 00        	movups	%xmm2, 0x510(%rbp)
140343d14: 0f 28 85 b0 05 00 00        	movaps	0x5b0(%rbp), %xmm0
140343d1b: 0f 28 8d c0 05 00 00        	movaps	0x5c0(%rbp), %xmm1
140343d22: 0f 28 95 d0 05 00 00        	movaps	0x5d0(%rbp), %xmm2
140343d29: 0f 11 85 20 05 00 00        	movups	%xmm0, 0x520(%rbp)
140343d30: 0f 11 8d 30 05 00 00        	movups	%xmm1, 0x530(%rbp)
140343d37: 0f 11 95 40 05 00 00        	movups	%xmm2, 0x540(%rbp)
140343d3e: 0f 11 bd 50 05 00 00        	movups	%xmm7, 0x550(%rbp)
140343d45: 0f 11 b5 60 05 00 00        	movups	%xmm6, 0x560(%rbp)
140343d4c: 48 89 bd 70 05 00 00        	movq	%rdi, 0x570(%rbp)
140343d53: 48 89 9d 78 05 00 00        	movq	%rbx, 0x578(%rbp)
140343d5a: 48 c7 85 90 05 00 00 08 00 00 00    	movq	$0x8, 0x590(%rbp)
140343d65: 48 c7 85 98 05 00 00 00 00 00 00    	movq	$0x0, 0x598(%rbp)
140343d70: 48 c7 85 20 03 00 00 00 00 00 00    	movq	$0x0, 0x320(%rbp)
140343d7b: 48 c7 85 28 03 00 00 08 00 00 00    	movq	$0x8, 0x328(%rbp)
140343d86: 48 c7 85 30 03 00 00 00 00 00 00    	movq	$0x0, 0x330(%rbp)
140343d91: c6 85 a2 05 00 00 00        	movb	$0x0, 0x5a2(%rbp)
140343d98: 66 c7 85 a0 05 00 00 00 00  	movw	$0x0, 0x5a0(%rbp)
140343da1: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
140343da8: 48 89 f1                    	movq	%rsi, %rcx
140343dab: e8 a0 e5 de ff              	callq	0x140132350 <.text+0x131350>
140343db0: 48 89 c6                    	movq	%rax, %rsi
140343db3: e8 88 99 a0 00              	callq	0x140d4d740 <.text+0xd4c740>
140343db8: 48 89 c7                    	movq	%rax, %rdi
140343dbb: f6 00 01                    	testb	$0x1, (%rax)
140343dbe: 74 11                       	je	0x140343dd1 <.text+0x342dd1>
140343dc0: 48 89 f9                    	movq	%rdi, %rcx
140343dc3: 48 83 c1 08                 	addq	$0x8, %rcx
140343dc7: 48 8b 47 08                 	movq	0x8(%rdi), %rax
140343dcb: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
140343dcf: eb 1b                       	jmp	0x140343dec <.text+0x342dec>
140343dd1: e8 2a 10 a2 00              	callq	0x140d64e00 <.text+0xd63e00>
140343dd6: 48 89 f9                    	movq	%rdi, %rcx
140343dd9: 48 83 c1 08                 	addq	$0x8, %rcx
140343ddd: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
140343de4: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
140343de8: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
140343dec: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
140343df0: 4c 89 01                    	movq	%r8, (%rcx)
140343df3: 0f 29 b5 30 03 00 00        	movaps	%xmm6, 0x330(%rbp)
140343dfa: 0f 29 bd 20 03 00 00        	movaps	%xmm7, 0x320(%rbp)
140343e01: 48 89 85 40 03 00 00        	movq	%rax, 0x340(%rbp)
140343e08: 48 89 95 48 03 00 00        	movq	%rdx, 0x348(%rbp)
140343e0f: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
140343e16: 48 89 f1                    	movq	%rsi, %rcx
140343e19: e8 f2 0b de ff              	callq	0x140124a10 <.text+0x123a10>
140343e1e: 48 89 c6                    	movq	%rax, %rsi
140343e21: 48 8d bd 20 03 00 00        	leaq	0x320(%rbp), %rdi
140343e28: 48 89 f9                    	movq	%rdi, %rcx
140343e2b: e8 30 7c dc ff              	callq	0x14010ba60 <.text+0x10aa60>
140343e30: 48 89 f1                    	movq	%rsi, %rcx
140343e33: 48 89 fa                    	movq	%rdi, %rdx
140343e36: e8 d5 a2 df ff              	callq	0x14013e110 <.text+0x13d110>
140343e3b: 48 89 c6                    	movq	%rax, %rsi
140343e3e: 48 89 f9                    	movq	%rdi, %rcx
140343e41: e8 fa 7a dc ff              	callq	0x14010b940 <.text+0x10a940>
140343e46: 48 89 f1                    	movq	%rsi, %rcx
140343e49: 48 89 fa                    	movq	%rdi, %rdx
140343e4c: e8 2f 52 df ff              	callq	0x140139080 <.text+0x138080>
140343e51: 48 89 c6                    	movq	%rax, %rsi
140343e54: 48 c7 45 10 00 00 00 00     	movq	$0x0, 0x10(%rbp)
140343e5c: 48 c7 45 18 08 00 00 00     	movq	$0x8, 0x18(%rbp)
140343e64: 48 c7 45 20 00 00 00 00     	movq	$0x0, 0x20(%rbp)
140343e6c: e8 cf 98 a0 00              	callq	0x140d4d740 <.text+0xd4c740>
140343e71: 48 89 c7                    	movq	%rax, %rdi
140343e74: f6 00 01                    	testb	$0x1, (%rax)
140343e77: 74 11                       	je	0x140343e8a <.text+0x342e8a>
140343e79: 48 89 f9                    	movq	%rdi, %rcx
140343e7c: 48 83 c1 08                 	addq	$0x8, %rcx
140343e80: 48 8b 47 08                 	movq	0x8(%rdi), %rax
140343e84: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
140343e88: eb 1b                       	jmp	0x140343ea5 <.text+0x342ea5>
140343e8a: e8 71 0f a2 00              	callq	0x140d64e00 <.text+0xd63e00>
140343e8f: 48 89 f9                    	movq	%rdi, %rcx
140343e92: 48 83 c1 08                 	addq	$0x8, %rcx
140343e96: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
140343e9d: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
140343ea1: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
140343ea5: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
140343ea9: 4c 89 01                    	movq	%r8, (%rcx)
140343eac: 0f 29 b5 90 00 00 00        	movaps	%xmm6, 0x90(%rbp)
140343eb3: 0f 29 bd 80 00 00 00        	movaps	%xmm7, 0x80(%rbp)
140343eba: 48 89 85 a0 00 00 00        	movq	%rax, 0xa0(%rbp)
140343ec1: 48 89 95 a8 00 00 00        	movq	%rdx, 0xa8(%rbp)
140343ec8: 48 8d 8d b0 05 00 00        	leaq	0x5b0(%rbp), %rcx
140343ecf: e8 6c 02 f0 ff              	callq	0x140244140 <.text+0x243140>
140343ed4: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
140343edb: e8 60 02 f0 ff              	callq	0x140244140 <.text+0x243140>
140343ee0: e8 5b 98 a0 00              	callq	0x140d4d740 <.text+0xd4c740>
140343ee5: 48 89 c7                    	movq	%rax, %rdi
140343ee8: f6 00 01                    	testb	$0x1, (%rax)
140343eeb: 74 11                       	je	0x140343efe <.text+0x342efe>
140343eed: 48 89 f9                    	movq	%rdi, %rcx
140343ef0: 48 83 c1 08                 	addq	$0x8, %rcx
140343ef4: 48 8b 47 08                 	movq	0x8(%rdi), %rax
140343ef8: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
140343efc: eb 1b                       	jmp	0x140343f19 <.text+0x342f19>
140343efe: e8 fd 0e a2 00              	callq	0x140d64e00 <.text+0xd63e00>
140343f03: 48 89 f9                    	movq	%rdi, %rcx
140343f06: 48 83 c1 08                 	addq	$0x8, %rcx
140343f0a: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
140343f11: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
140343f15: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
140343f19: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
140343f1d: 4c 89 01                    	movq	%r8, (%rcx)
140343f20: 0f 29 b5 c0 02 00 00        	movaps	%xmm6, 0x2c0(%rbp)
140343f27: 0f 29 bd b0 02 00 00        	movaps	%xmm7, 0x2b0(%rbp)
140343f2e: 48 89 85 d0 02 00 00        	movq	%rax, 0x2d0(%rbp)
140343f35: 48 89 95 d8 02 00 00        	movq	%rdx, 0x2d8(%rbp)
140343f3c: e8 ff 97 a0 00              	callq	0x140d4d740 <.text+0xd4c740>
140343f41: 49 89 c6                    	movq	%rax, %r14
140343f44: f6 00 01                    	testb	$0x1, (%rax)
140343f47: 74 11                       	je	0x140343f5a <.text+0x342f5a>
140343f49: 4c 89 f0                    	movq	%r14, %rax
140343f4c: 48 83 c0 08                 	addq	$0x8, %rax
140343f50: 49 8b 7e 08                 	movq	0x8(%r14), %rdi
140343f54: 49 8b 5e 10                 	movq	0x10(%r14), %rbx
140343f58: eb 21                       	jmp	0x140343f7b <.text+0x342f7b>
140343f5a: e8 a1 0e a2 00              	callq	0x140d64e00 <.text+0xd63e00>
140343f5f: 48 89 c7                    	movq	%rax, %rdi
140343f62: 48 89 d3                    	movq	%rdx, %rbx
140343f65: 4c 89 f0                    	movq	%r14, %rax
140343f68: 48 83 c0 08                 	addq	$0x8, %rax
140343f6c: 49 c7 06 01 00 00 00        	movq	$0x1, (%r14)
140343f73: 49 89 7e 08                 	movq	%rdi, 0x8(%r14)
140343f77: 49 89 56 10                 	movq	%rdx, 0x10(%r14)
140343f7b: 48 8d 4f 01                 	leaq	0x1(%rdi), %rcx
140343f7f: 48 89 08                    	movq	%rcx, (%rax)
140343f82: 48 8b 45 20                 	movq	0x20(%rbp), %rax
140343f86: 48 89 85 30 03 00 00        	movq	%rax, 0x330(%rbp)
140343f8d: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
140343f91: 0f 29 85 20 03 00 00        	movaps	%xmm0, 0x320(%rbp)
140343f98: 0f 28 85 80 00 00 00        	movaps	0x80(%rbp), %xmm0
140343f9f: 0f 28 8d 90 00 00 00        	movaps	0x90(%rbp), %xmm1
140343fa6: 0f 28 95 a0 00 00 00        	movaps	0xa0(%rbp), %xmm2
140343fad: 0f 11 85 58 04 00 00        	movups	%xmm0, 0x458(%rbp)
140343fb4: 0f 11 8d 68 04 00 00        	movups	%xmm1, 0x468(%rbp)
140343fbb: 0f 11 95 78 04 00 00        	movups	%xmm2, 0x478(%rbp)
140343fc2: 48 8d 8d 38 03 00 00        	leaq	0x338(%rbp), %rcx
140343fc9: 48 8d 95 b0 05 00 00        	leaq	0x5b0(%rbp), %rdx
140343fd0: 41 b8 90 00 00 00           	movl	$0x90, %r8d
140343fd6: e8 25 fc 5d 02              	callq	0x142923c00 <.text+0x2922c00>
140343fdb: 48 8d 8d c8 03 00 00        	leaq	0x3c8(%rbp), %rcx
140343fe2: 48 8d 95 f0 00 00 00        	leaq	0xf0(%rbp), %rdx
140343fe9: 41 b8 90 00 00 00           	movl	$0x90, %r8d
140343fef: e8 0c fc 5d 02              	callq	0x142923c00 <.text+0x2922c00>
140343ff4: 0f 28 85 b0 02 00 00        	movaps	0x2b0(%rbp), %xmm0
140343ffb: 0f 28 8d c0 02 00 00        	movaps	0x2c0(%rbp), %xmm1
140344002: 0f 28 95 d0 02 00 00        	movaps	0x2d0(%rbp), %xmm2
140344009: 0f 11 85 88 04 00 00        	movups	%xmm0, 0x488(%rbp)
140344010: 0f 11 8d 98 04 00 00        	movups	%xmm1, 0x498(%rbp)
140344017: 0f 11 95 a8 04 00 00        	movups	%xmm2, 0x4a8(%rbp)
14034401e: 0f 11 bd b8 04 00 00        	movups	%xmm7, 0x4b8(%rbp)
140344025: 0f 11 b5 c8 04 00 00        	movups	%xmm6, 0x4c8(%rbp)
14034402c: 48 89 bd d8 04 00 00        	movq	%rdi, 0x4d8(%rbp)
140344033: 48 89 9d e0 04 00 00        	movq	%rbx, 0x4e0(%rbp)
14034403a: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
140344041: 48 89 f1                    	movq	%rsi, %rcx
140344044: e8 77 85 dd ff              	callq	0x14011c5c0 <.text+0x11b5c0>
140344049: 48 c7 85 20 03 00 00 00 00 00 00    	movq	$0x0, 0x320(%rbp)
140344054: 48 c7 85 28 03 00 00 08 00 00 00    	movq	$0x8, 0x328(%rbp)
14034405f: 48 c7 85 30 03 00 00 00 00 00 00    	movq	$0x0, 0x330(%rbp)
14034406a: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
140344071: 48 89 c1                    	movq	%rax, %rcx
140344074: e8 27 2a dd ff              	callq	0x140116aa0 <.text+0x115aa0>
140344079: 48 c7 85 20 03 00 00 00 00 00 00    	movq	$0x0, 0x320(%rbp)
140344084: 48 c7 85 28 03 00 00 08 00 00 00    	movq	$0x8, 0x328(%rbp)
14034408f: 48 c7 85 30 03 00 00 00 00 00 00    	movq	$0x0, 0x330(%rbp)
14034409a: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
1403440a1: 48 89 c1                    	movq	%rax, %rcx
1403440a4: e8 67 15 de ff              	callq	0x140125610 <.text+0x124610>
1403440a9: 48 89 c6                    	movq	%rax, %rsi
1403440ac: e8 8f 96 a0 00              	callq	0x140d4d740 <.text+0xd4c740>
1403440b1: 48 89 c7                    	movq	%rax, %rdi
1403440b4: f6 00 01                    	testb	$0x1, (%rax)
1403440b7: 74 11                       	je	0x1403440ca <.text+0x3430ca>
1403440b9: 48 89 f9                    	movq	%rdi, %rcx
1403440bc: 48 83 c1 08                 	addq	$0x8, %rcx
1403440c0: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1403440c4: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
1403440c8: eb 1b                       	jmp	0x1403440e5 <.text+0x3430e5>
1403440ca: e8 31 0d a2 00              	callq	0x140d64e00 <.text+0xd63e00>
1403440cf: 48 89 f9                    	movq	%rdi, %rcx
1403440d2: 48 83 c1 08                 	addq	$0x8, %rcx
1403440d6: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
1403440dd: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
1403440e1: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
1403440e5: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
1403440e9: 4c 89 01                    	movq	%r8, (%rcx)
1403440ec: 0f 29 b5 00 01 00 00        	movaps	%xmm6, 0x100(%rbp)
1403440f3: 0f 29 bd f0 00 00 00        	movaps	%xmm7, 0xf0(%rbp)
1403440fa: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
140344101: 48 89 95 18 01 00 00        	movq	%rdx, 0x118(%rbp)
140344108: e8 33 96 a0 00              	callq	0x140d4d740 <.text+0xd4c740>
14034410d: 48 89 c7                    	movq	%rax, %rdi
140344110: f6 00 01                    	testb	$0x1, (%rax)
140344113: 74 11                       	je	0x140344126 <.text+0x343126>
140344115: 48 89 f9                    	movq	%rdi, %rcx
140344118: 48 83 c1 08                 	addq	$0x8, %rcx
14034411c: 48 8b 47 08                 	movq	0x8(%rdi), %rax
140344120: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
140344124: eb 1b                       	jmp	0x140344141 <.text+0x343141>
140344126: e8 d5 0c a2 00              	callq	0x140d64e00 <.text+0xd63e00>
14034412b: 48 89 f9                    	movq	%rdi, %rcx
14034412e: 48 83 c1 08                 	addq	$0x8, %rcx
140344132: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
140344139: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
14034413d: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
140344141: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
140344145: 4c 89 01                    	movq	%r8, (%rcx)
140344148: 0f 28 85 f0 00 00 00        	movaps	0xf0(%rbp), %xmm0
14034414f: 0f 28 8d 00 01 00 00        	movaps	0x100(%rbp), %xmm1
140344156: 0f 28 95 10 01 00 00        	movaps	0x110(%rbp), %xmm2
14034415d: 0f 29 95 40 03 00 00        	movaps	%xmm2, 0x340(%rbp)
140344164: 0f 29 8d 30 03 00 00        	movaps	%xmm1, 0x330(%rbp)
14034416b: 0f 29 85 20 03 00 00        	movaps	%xmm0, 0x320(%rbp)
140344172: 0f 29 bd 50 03 00 00        	movaps	%xmm7, 0x350(%rbp)
140344179: 0f 29 b5 60 03 00 00        	movaps	%xmm6, 0x360(%rbp)
140344180: 48 89 85 70 03 00 00        	movq	%rax, 0x370(%rbp)
140344187: 48 89 95 78 03 00 00        	movq	%rdx, 0x378(%rbp)
14034418e: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
140344195: 48 89 f1                    	movq	%rsi, %rcx
140344198: e8 53 1a de ff              	callq	0x140125bf0 <.text+0x124bf0>
14034419d: 48 89 c6                    	movq	%rax, %rsi
1403441a0: e8 9b 95 a0 00              	callq	0x140d4d740 <.text+0xd4c740>
1403441a5: 48 89 c7                    	movq	%rax, %rdi
1403441a8: f6 00 01                    	testb	$0x1, (%rax)
1403441ab: 74 11                       	je	0x1403441be <.text+0x3431be>
1403441ad: 48 89 f9                    	movq	%rdi, %rcx
1403441b0: 48 83 c1 08                 	addq	$0x8, %rcx
1403441b4: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1403441b8: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
1403441bc: eb 1b                       	jmp	0x1403441d9 <.text+0x3431d9>
1403441be: e8 3d 0c a2 00              	callq	0x140d64e00 <.text+0xd63e00>
1403441c3: 48 89 f9                    	movq	%rdi, %rcx
1403441c6: 48 83 c1 08                 	addq	$0x8, %rcx
1403441ca: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
1403441d1: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
1403441d5: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
1403441d9: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
1403441dd: 4c 89 01                    	movq	%r8, (%rcx)
1403441e0: 0f 29 b5 30 03 00 00        	movaps	%xmm6, 0x330(%rbp)
1403441e7: 0f 29 bd 20 03 00 00        	movaps	%xmm7, 0x320(%rbp)
1403441ee: 48 89 85 40 03 00 00        	movq	%rax, 0x340(%rbp)
1403441f5: 48 89 95 48 03 00 00        	movq	%rdx, 0x348(%rbp)
1403441fc: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
140344203: 48 89 f1                    	movq	%rsi, %rcx
140344206: e8 d5 6a df ff              	callq	0x14013ace0 <.text+0x139ce0>
14034420b: 48 89 c7                    	movq	%rax, %rdi
14034420e: 48 8d 8d b0 05 00 00        	leaq	0x5b0(%rbp), %rcx
140344215: e8 26 ff ef ff              	callq	0x140244140 <.text+0x243140>
14034421a: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
140344221: e8 1a ff ef ff              	callq	0x140244140 <.text+0x243140>
140344226: 31 c9                       	xorl	%ecx, %ecx
140344228: e8 f3 58 d9 ff              	callq	0x1400d9b20 <.text+0xd8b20>
14034422d: 48 85 c0                    	testq	%rax, %rax
140344230: 0f 84 1d 12 00 00           	je	0x140345453 <.text+0x344453>
140344236: 48 8d 8d 48 03 00 00        	leaq	0x348(%rbp), %rcx
14034423d: 48 8d 9d d8 03 00 00        	leaq	0x3d8(%rbp), %rbx
140344244: 44 0f 10 20                 	movups	(%rax), %xmm12
140344248: 48 ff 00                    	incq	(%rax)
14034424b: 48 8d b5 b0 05 00 00        	leaq	0x5b0(%rbp), %rsi
140344252: 41 b8 90 00 00 00           	movl	$0x90, %r8d
140344258: 48 89 f2                    	movq	%rsi, %rdx
14034425b: e8 a0 f9 5d 02              	callq	0x142923c00 <.text+0x2922c00>
140344260: 48 8d 95 f0 00 00 00        	leaq	0xf0(%rbp), %rdx
140344267: 41 b8 90 00 00 00           	movl	$0x90, %r8d
14034426d: 48 89 d9                    	movq	%rbx, %rcx
140344270: e8 8b f9 5d 02              	callq	0x142923c00 <.text+0x2922c00>
140344275: c6 85 98 04 00 00 00        	movb	$0x0, 0x498(%rbp)
14034427c: 48 c7 85 20 03 00 00 00 00 00 00    	movq	$0x0, 0x320(%rbp)
140344287: 48 c7 85 30 03 00 00 03 00 00 00    	movq	$0x3, 0x330(%rbp)
140344292: 44 0f 11 8d 68 04 00 00     	movups	%xmm9, 0x468(%rbp)
14034429a: 44 0f 11 85 78 04 00 00     	movups	%xmm8, 0x478(%rbp)
1403442a2: 44 0f 11 a5 88 04 00 00     	movups	%xmm12, 0x488(%rbp)
1403442aa: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
1403442b1: 48 89 f9                    	movq	%rdi, %rcx
1403442b4: e8 97 6c dd ff              	callq	0x14011af50 <.text+0x119f50>
1403442b9: 48 89 c7                    	movq	%rax, %rdi
1403442bc: e8 7f 94 a0 00              	callq	0x140d4d740 <.text+0xd4c740>
1403442c1: 48 89 c3                    	movq	%rax, %rbx
1403442c4: f6 00 01                    	testb	$0x1, (%rax)
1403442c7: 74 11                       	je	0x1403442da <.text+0x3432da>
1403442c9: 48 89 d9                    	movq	%rbx, %rcx
1403442cc: 48 83 c1 08                 	addq	$0x8, %rcx
1403442d0: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1403442d4: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403442d8: eb 1b                       	jmp	0x1403442f5 <.text+0x3432f5>
1403442da: e8 21 0b a2 00              	callq	0x140d64e00 <.text+0xd63e00>
1403442df: 48 89 d9                    	movq	%rbx, %rcx
1403442e2: 48 83 c1 08                 	addq	$0x8, %rcx
1403442e6: 48 c7 03 01 00 00 00        	movq	$0x1, (%rbx)
1403442ed: 48 89 43 08                 	movq	%rax, 0x8(%rbx)
1403442f1: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1403442f5: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
1403442f9: 4c 89 01                    	movq	%r8, (%rcx)
1403442fc: 0f 29 b5 30 03 00 00        	movaps	%xmm6, 0x330(%rbp)
140344303: 0f 29 bd 20 03 00 00        	movaps	%xmm7, 0x320(%rbp)
14034430a: 48 89 85 40 03 00 00        	movq	%rax, 0x340(%rbp)
140344311: 48 89 95 48 03 00 00        	movq	%rdx, 0x348(%rbp)
140344318: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
14034431f: 48 89 f9                    	movq	%rdi, %rcx
140344322: e8 a9 6f df ff              	callq	0x14013b2d0 <.text+0x13a2d0>
140344327: 48 89 c7                    	movq	%rax, %rdi
14034432a: e8 11 94 a0 00              	callq	0x140d4d740 <.text+0xd4c740>
14034432f: 48 89 c3                    	movq	%rax, %rbx
140344332: f6 00 01                    	testb	$0x1, (%rax)
140344335: 74 11                       	je	0x140344348 <.text+0x343348>
140344337: 48 89 d9                    	movq	%rbx, %rcx
14034433a: 48 83 c1 08                 	addq	$0x8, %rcx
14034433e: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140344342: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140344346: eb 1b                       	jmp	0x140344363 <.text+0x343363>
140344348: e8 b3 0a a2 00              	callq	0x140d64e00 <.text+0xd63e00>
14034434d: 48 89 d9                    	movq	%rbx, %rcx
140344350: 48 83 c1 08                 	addq	$0x8, %rcx
140344354: 48 c7 03 01 00 00 00        	movq	$0x1, (%rbx)
14034435b: 48 89 43 08                 	movq	%rax, 0x8(%rbx)
14034435f: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140344363: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
140344367: 4c 89 01                    	movq	%r8, (%rcx)
14034436a: 0f 29 b5 30 03 00 00        	movaps	%xmm6, 0x330(%rbp)
140344371: 0f 29 bd 20 03 00 00        	movaps	%xmm7, 0x320(%rbp)
140344378: 48 89 85 40 03 00 00        	movq	%rax, 0x340(%rbp)
14034437f: 48 89 95 48 03 00 00        	movq	%rdx, 0x348(%rbp)
140344386: c6 85 50 03 00 00 00        	movb	$0x0, 0x350(%rbp)
14034438d: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
140344394: 48 89 f9                    	movq	%rdi, %rcx
140344397: e8 54 cb dd ff              	callq	0x140120ef0 <.text+0x11fef0>
14034439c: 48 89 c7                    	movq	%rax, %rdi
14034439f: 31 c9                       	xorl	%ecx, %ecx
1403443a1: e8 7a 57 d9 ff              	callq	0x1400d9b20 <.text+0xd8b20>
1403443a6: 48 85 c0                    	testq	%rax, %rax
1403443a9: 0f 84 13 10 00 00           	je	0x1403453c2 <.text+0x3443c2>
1403443af: 0f 10 00                    	movups	(%rax), %xmm0
1403443b2: 48 ff 00                    	incq	(%rax)
1403443b5: 44 0f 29 95 c0 02 00 00     	movaps	%xmm10, 0x2c0(%rbp)
1403443bd: 44 0f 29 9d b0 02 00 00     	movaps	%xmm11, 0x2b0(%rbp)
1403443c5: 0f 29 85 d0 02 00 00        	movaps	%xmm0, 0x2d0(%rbp)
1403443cc: 31 c9                       	xorl	%ecx, %ecx
1403443ce: e8 4d 57 d9 ff              	callq	0x1400d9b20 <.text+0xd8b20>
1403443d3: 48 85 c0                    	testq	%rax, %rax
1403443d6: 0f 84 85 10 00 00           	je	0x140345461 <.text+0x344461>
1403443dc: 0f 10 00                    	movups	(%rax), %xmm0
1403443df: 48 ff 00                    	incq	(%rax)
1403443e2: 44 0f 29 95 c0 05 00 00     	movaps	%xmm10, 0x5c0(%rbp)
1403443ea: 44 0f 29 9d b0 05 00 00     	movaps	%xmm11, 0x5b0(%rbp)
1403443f2: 0f 29 85 d0 05 00 00        	movaps	%xmm0, 0x5d0(%rbp)
1403443f9: 31 c9                       	xorl	%ecx, %ecx
1403443fb: e8 20 57 d9 ff              	callq	0x1400d9b20 <.text+0xd8b20>
140344400: 48 85 c0                    	testq	%rax, %rax
140344403: 0f 84 66 10 00 00           	je	0x14034546f <.text+0x34446f>
140344409: 0f 10 00                    	movups	(%rax), %xmm0
14034440c: 48 ff 00                    	incq	(%rax)
14034440f: 44 0f 29 95 00 01 00 00     	movaps	%xmm10, 0x100(%rbp)
140344417: 44 0f 29 9d f0 00 00 00     	movaps	%xmm11, 0xf0(%rbp)
14034441f: 0f 29 85 10 01 00 00        	movaps	%xmm0, 0x110(%rbp)
140344426: 31 c9                       	xorl	%ecx, %ecx
140344428: e8 f3 56 d9 ff              	callq	0x1400d9b20 <.text+0xd8b20>
14034442d: 48 85 c0                    	testq	%rax, %rax
140344430: 0f 84 47 10 00 00           	je	0x14034547d <.text+0x34447d>
140344436: 0f 10 00                    	movups	(%rax), %xmm0
140344439: 48 ff 00                    	incq	(%rax)
14034443c: 0f 28 8d b0 02 00 00        	movaps	0x2b0(%rbp), %xmm1
140344443: 0f 28 95 c0 02 00 00        	movaps	0x2c0(%rbp), %xmm2
14034444a: 0f 28 9d d0 02 00 00        	movaps	0x2d0(%rbp), %xmm3
140344451: 0f 29 9d 40 03 00 00        	movaps	%xmm3, 0x340(%rbp)
140344458: 0f 29 95 30 03 00 00        	movaps	%xmm2, 0x330(%rbp)
14034445f: 0f 29 8d 20 03 00 00        	movaps	%xmm1, 0x320(%rbp)
140344466: 0f 28 8d b0 05 00 00        	movaps	0x5b0(%rbp), %xmm1
14034446d: 0f 28 95 c0 05 00 00        	movaps	0x5c0(%rbp), %xmm2
140344474: 0f 28 9d d0 05 00 00        	movaps	0x5d0(%rbp), %xmm3
14034447b: 0f 29 8d 50 03 00 00        	movaps	%xmm1, 0x350(%rbp)
140344482: 0f 29 95 60 03 00 00        	movaps	%xmm2, 0x360(%rbp)
140344489: 0f 29 9d 70 03 00 00        	movaps	%xmm3, 0x370(%rbp)
140344490: 0f 28 8d f0 00 00 00        	movaps	0xf0(%rbp), %xmm1
140344497: 0f 28 95 00 01 00 00        	movaps	0x100(%rbp), %xmm2
14034449e: 0f 28 9d 10 01 00 00        	movaps	0x110(%rbp), %xmm3
1403444a5: 0f 29 8d 80 03 00 00        	movaps	%xmm1, 0x380(%rbp)
1403444ac: 0f 29 95 90 03 00 00        	movaps	%xmm2, 0x390(%rbp)
1403444b3: 0f 29 9d a0 03 00 00        	movaps	%xmm3, 0x3a0(%rbp)
1403444ba: 44 0f 29 95 c0 03 00 00     	movaps	%xmm10, 0x3c0(%rbp)
1403444c2: 44 0f 29 9d b0 03 00 00     	movaps	%xmm11, 0x3b0(%rbp)
1403444ca: 0f 29 85 d0 03 00 00        	movaps	%xmm0, 0x3d0(%rbp)
1403444d1: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
1403444d8: 48 89 f9                    	movq	%rdi, %rcx
1403444db: e8 30 5c de ff              	callq	0x14012a110 <.text+0x129110>
1403444e0: 48 89 c7                    	movq	%rax, %rdi
1403444e3: 31 c9                       	xorl	%ecx, %ecx
1403444e5: e8 36 56 d9 ff              	callq	0x1400d9b20 <.text+0xd8b20>
1403444ea: 48 85 c0                    	testq	%rax, %rax
1403444ed: 0f 84 cf 0e 00 00           	je	0x1403453c2 <.text+0x3443c2>
1403444f3: 0f 10 00                    	movups	(%rax), %xmm0
1403444f6: 48 ff 00                    	incq	(%rax)
1403444f9: 0f 29 85 40 03 00 00        	movaps	%xmm0, 0x340(%rbp)
140344500: 44 0f 29 8d 20 03 00 00     	movaps	%xmm9, 0x320(%rbp)
140344508: 44 0f 29 85 30 03 00 00     	movaps	%xmm8, 0x330(%rbp)
140344510: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
140344517: 48 89 f9                    	movq	%rdi, %rcx
14034451a: e8 e1 ad df ff              	callq	0x14013f300 <.text+0x13e300>
14034451f: 48 89 c7                    	movq	%rax, %rdi
140344522: e8 19 92 a0 00              	callq	0x140d4d740 <.text+0xd4c740>
140344527: 48 89 c3                    	movq	%rax, %rbx
14034452a: f6 00 01                    	testb	$0x1, (%rax)
14034452d: 74 11                       	je	0x140344540 <.text+0x343540>
14034452f: 48 89 d9                    	movq	%rbx, %rcx
140344532: 48 83 c1 08                 	addq	$0x8, %rcx
140344536: 48 8b 43 08                 	movq	0x8(%rbx), %rax
14034453a: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14034453e: eb 1b                       	jmp	0x14034455b <.text+0x34355b>
140344540: e8 bb 08 a2 00              	callq	0x140d64e00 <.text+0xd63e00>
140344545: 48 89 d9                    	movq	%rbx, %rcx
140344548: 48 83 c1 08                 	addq	$0x8, %rcx
14034454c: 48 c7 03 01 00 00 00        	movq	$0x1, (%rbx)
140344553: 48 89 43 08                 	movq	%rax, 0x8(%rbx)
140344557: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
14034455b: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
14034455f: 4c 89 01                    	movq	%r8, (%rcx)
140344562: 0f 29 75 20                 	movaps	%xmm6, 0x20(%rbp)
140344566: 0f 29 7d 10                 	movaps	%xmm7, 0x10(%rbp)
14034456a: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
14034456e: 48 89 55 38                 	movq	%rdx, 0x38(%rbp)
140344572: e8 c9 91 a0 00              	callq	0x140d4d740 <.text+0xd4c740>
140344577: 48 89 c3                    	movq	%rax, %rbx
14034457a: f6 00 01                    	testb	$0x1, (%rax)
14034457d: 74 11                       	je	0x140344590 <.text+0x343590>
14034457f: 48 89 d9                    	movq	%rbx, %rcx
140344582: 48 83 c1 08                 	addq	$0x8, %rcx
140344586: 48 8b 43 08                 	movq	0x8(%rbx), %rax
14034458a: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14034458e: eb 1b                       	jmp	0x1403445ab <.text+0x3435ab>
140344590: e8 6b 08 a2 00              	callq	0x140d64e00 <.text+0xd63e00>
140344595: 48 89 d9                    	movq	%rbx, %rcx
140344598: 48 83 c1 08                 	addq	$0x8, %rcx
14034459c: 48 c7 03 01 00 00 00        	movq	$0x1, (%rbx)
1403445a3: 48 89 43 08                 	movq	%rax, 0x8(%rbx)
1403445a7: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1403445ab: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
1403445af: 4c 89 01                    	movq	%r8, (%rcx)
1403445b2: 0f 29 b5 90 00 00 00        	movaps	%xmm6, 0x90(%rbp)
1403445b9: 0f 29 bd 80 00 00 00        	movaps	%xmm7, 0x80(%rbp)
1403445c0: 48 89 85 a0 00 00 00        	movq	%rax, 0xa0(%rbp)
1403445c7: 48 89 95 a8 00 00 00        	movq	%rdx, 0xa8(%rbp)
1403445ce: e8 6d 91 a0 00              	callq	0x140d4d740 <.text+0xd4c740>
1403445d3: 48 89 c3                    	movq	%rax, %rbx
1403445d6: f6 00 01                    	testb	$0x1, (%rax)
1403445d9: 74 11                       	je	0x1403445ec <.text+0x3435ec>
1403445db: 48 89 d9                    	movq	%rbx, %rcx
1403445de: 48 83 c1 08                 	addq	$0x8, %rcx
1403445e2: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1403445e6: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403445ea: eb 1b                       	jmp	0x140344607 <.text+0x343607>
1403445ec: e8 0f 08 a2 00              	callq	0x140d64e00 <.text+0xd63e00>
1403445f1: 48 89 d9                    	movq	%rbx, %rcx
1403445f4: 48 83 c1 08                 	addq	$0x8, %rcx
1403445f8: 48 c7 03 01 00 00 00        	movq	$0x1, (%rbx)
1403445ff: 48 89 43 08                 	movq	%rax, 0x8(%rbx)
140344603: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140344607: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
14034460b: 4c 89 01                    	movq	%r8, (%rcx)
14034460e: 0f 29 b5 c0 02 00 00        	movaps	%xmm6, 0x2c0(%rbp)
140344615: 0f 29 bd b0 02 00 00        	movaps	%xmm7, 0x2b0(%rbp)
14034461c: 48 89 85 d0 02 00 00        	movq	%rax, 0x2d0(%rbp)
140344623: 48 89 95 d8 02 00 00        	movq	%rdx, 0x2d8(%rbp)
14034462a: e8 11 91 a0 00              	callq	0x140d4d740 <.text+0xd4c740>
14034462f: 48 89 c3                    	movq	%rax, %rbx
140344632: f6 00 01                    	testb	$0x1, (%rax)
140344635: 74 11                       	je	0x140344648 <.text+0x343648>
140344637: 48 89 d9                    	movq	%rbx, %rcx
14034463a: 48 83 c1 08                 	addq	$0x8, %rcx
14034463e: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140344642: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140344646: eb 1b                       	jmp	0x140344663 <.text+0x343663>
140344648: e8 b3 07 a2 00              	callq	0x140d64e00 <.text+0xd63e00>
14034464d: 48 89 d9                    	movq	%rbx, %rcx
140344650: 48 83 c1 08                 	addq	$0x8, %rcx
140344654: 48 c7 03 01 00 00 00        	movq	$0x1, (%rbx)
14034465b: 48 89 43 08                 	movq	%rax, 0x8(%rbx)
14034465f: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140344663: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
140344667: 4c 89 01                    	movq	%r8, (%rcx)
14034466a: 0f 29 b5 c0 05 00 00        	movaps	%xmm6, 0x5c0(%rbp)
140344671: 0f 29 bd b0 05 00 00        	movaps	%xmm7, 0x5b0(%rbp)
140344678: 48 89 85 d0 05 00 00        	movq	%rax, 0x5d0(%rbp)
14034467f: 48 89 95 d8 05 00 00        	movq	%rdx, 0x5d8(%rbp)
140344686: 0f 57 c0                    	xorps	%xmm0, %xmm0
140344689: 0f 11 85 f8 00 00 00        	movups	%xmm0, 0xf8(%rbp)
140344690: 0f 11 85 08 01 00 00        	movups	%xmm0, 0x108(%rbp)
140344697: 48 c7 85 18 01 00 00 00 00 00 00    	movq	$0x0, 0x118(%rbp)
1403446a2: 0f 11 45 d0                 	movups	%xmm0, -0x30(%rbp)
1403446a6: 0f 11 45 bc                 	movups	%xmm0, -0x44(%rbp)
1403446aa: 0f 11 45 a8                 	movups	%xmm0, -0x58(%rbp)
1403446ae: 0f 11 45 e4                 	movups	%xmm0, -0x1c(%rbp)
1403446b2: e8 89 90 a0 00              	callq	0x140d4d740 <.text+0xd4c740>
1403446b7: 48 89 c3                    	movq	%rax, %rbx
1403446ba: f6 00 01                    	testb	$0x1, (%rax)
1403446bd: 74 11                       	je	0x1403446d0 <.text+0x3436d0>
1403446bf: 48 89 d9                    	movq	%rbx, %rcx
1403446c2: 48 83 c1 08                 	addq	$0x8, %rcx
1403446c6: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1403446ca: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403446ce: eb 1b                       	jmp	0x1403446eb <.text+0x3436eb>
1403446d0: e8 2b 07 a2 00              	callq	0x140d64e00 <.text+0xd63e00>
1403446d5: 48 89 d9                    	movq	%rbx, %rcx
1403446d8: 48 83 c1 08                 	addq	$0x8, %rcx
1403446dc: 48 c7 03 01 00 00 00        	movq	$0x1, (%rbx)
1403446e3: 48 89 43 08                 	movq	%rax, 0x8(%rbx)
1403446e7: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1403446eb: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
1403446ef: 4c 89 01                    	movq	%r8, (%rcx)
1403446f2: 0f 28 45 10                 	movaps	0x10(%rbp), %xmm0
1403446f6: 0f 28 4d 20                 	movaps	0x20(%rbp), %xmm1
1403446fa: 0f 28 55 30                 	movaps	0x30(%rbp), %xmm2
1403446fe: 0f 11 95 e0 03 00 00        	movups	%xmm2, 0x3e0(%rbp)
140344705: 0f 11 8d d0 03 00 00        	movups	%xmm1, 0x3d0(%rbp)
14034470c: 0f 11 85 c0 03 00 00        	movups	%xmm0, 0x3c0(%rbp)
140344713: 0f 28 85 80 00 00 00        	movaps	0x80(%rbp), %xmm0
14034471a: 0f 28 8d 90 00 00 00        	movaps	0x90(%rbp), %xmm1
140344721: 0f 28 95 a0 00 00 00        	movaps	0xa0(%rbp), %xmm2
140344728: 0f 11 85 f0 03 00 00        	movups	%xmm0, 0x3f0(%rbp)
14034472f: 0f 11 8d 00 04 00 00        	movups	%xmm1, 0x400(%rbp)
140344736: 0f 11 95 10 04 00 00        	movups	%xmm2, 0x410(%rbp)
14034473d: 0f 28 85 b0 02 00 00        	movaps	0x2b0(%rbp), %xmm0
140344744: 0f 28 8d c0 02 00 00        	movaps	0x2c0(%rbp), %xmm1
14034474b: 0f 28 95 d0 02 00 00        	movaps	0x2d0(%rbp), %xmm2
140344752: 0f 11 85 20 04 00 00        	movups	%xmm0, 0x420(%rbp)
140344759: 0f 11 8d 30 04 00 00        	movups	%xmm1, 0x430(%rbp)
140344760: 0f 11 95 40 04 00 00        	movups	%xmm2, 0x440(%rbp)
140344767: 0f 28 85 b0 05 00 00        	movaps	0x5b0(%rbp), %xmm0
14034476e: 0f 28 8d c0 05 00 00        	movaps	0x5c0(%rbp), %xmm1
140344775: 0f 28 95 d0 05 00 00        	movaps	0x5d0(%rbp), %xmm2
14034477c: 0f 11 95 70 04 00 00        	movups	%xmm2, 0x470(%rbp)
140344783: 0f 11 8d 60 04 00 00        	movups	%xmm1, 0x460(%rbp)
14034478a: 0f 11 85 50 04 00 00        	movups	%xmm0, 0x450(%rbp)
140344791: c7 85 20 03 00 00 00 00 00 00       	movl	$0x0, 0x320(%rbp)
14034479b: 0f 10 85 f0 00 00 00        	movups	0xf0(%rbp), %xmm0
1403447a2: 0f 10 8d 00 01 00 00        	movups	0x100(%rbp), %xmm1
1403447a9: 0f 10 95 10 01 00 00        	movups	0x110(%rbp), %xmm2
1403447b0: 0f 11 85 24 03 00 00        	movups	%xmm0, 0x324(%rbp)
1403447b7: 0f 11 8d 34 03 00 00        	movups	%xmm1, 0x334(%rbp)
1403447be: 0f 11 95 44 03 00 00        	movups	%xmm2, 0x344(%rbp)
1403447c5: c7 85 54 03 00 00 00 00 00 00       	movl	$0x0, 0x354(%rbp)
1403447cf: 48 c7 85 5c 03 00 00 00 00 00 00    	movq	$0x0, 0x35c(%rbp)
1403447da: 0f 10 45 cc                 	movups	-0x34(%rbp), %xmm0
1403447de: 0f 11 85 64 03 00 00        	movups	%xmm0, 0x364(%rbp)
1403447e5: 8b 4d dc                    	movl	-0x24(%rbp), %ecx
1403447e8: 89 8d 74 03 00 00           	movl	%ecx, 0x374(%rbp)
1403447ee: c7 85 78 03 00 00 00 00 00 00       	movl	$0x0, 0x378(%rbp)
1403447f8: 8b 4d c8                    	movl	-0x38(%rbp), %ecx
1403447fb: 89 8d 8c 03 00 00           	movl	%ecx, 0x38c(%rbp)
140344801: 0f 10 45 b8                 	movups	-0x48(%rbp), %xmm0
140344805: 0f 11 85 7c 03 00 00        	movups	%xmm0, 0x37c(%rbp)
14034480c: c7 85 90 03 00 00 00 00 00 00       	movl	$0x0, 0x390(%rbp)
140344816: 8b 4d b4                    	movl	-0x4c(%rbp), %ecx
140344819: 89 8d a4 03 00 00           	movl	%ecx, 0x3a4(%rbp)
14034481f: 0f 10 45 a4                 	movups	-0x5c(%rbp), %xmm0
140344823: 0f 11 85 94 03 00 00        	movups	%xmm0, 0x394(%rbp)
14034482a: c7 85 a8 03 00 00 00 00 00 00       	movl	$0x0, 0x3a8(%rbp)
140344834: 8b 4d f0                    	movl	-0x10(%rbp), %ecx
140344837: 89 8d bc 03 00 00           	movl	%ecx, 0x3bc(%rbp)
14034483d: 0f 10 45 e0                 	movups	-0x20(%rbp), %xmm0
140344841: 0f 11 85 ac 03 00 00        	movups	%xmm0, 0x3ac(%rbp)
140344848: 0f 11 b5 90 04 00 00        	movups	%xmm6, 0x490(%rbp)
14034484f: 0f 11 bd 80 04 00 00        	movups	%xmm7, 0x480(%rbp)
140344856: 48 89 85 a0 04 00 00        	movq	%rax, 0x4a0(%rbp)
14034485d: 48 89 95 a8 04 00 00        	movq	%rdx, 0x4a8(%rbp)
140344864: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
14034486b: 48 89 f9                    	movq	%rdi, %rcx
14034486e: e8 dd d7 dd ff              	callq	0x140122050 <.text+0x121050>
140344873: 48 89 c7                    	movq	%rax, %rdi
140344876: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
14034487d: e8 4e fa ef ff              	callq	0x1402442d0 <.text+0x2432d0>
140344882: e8 b9 8e a0 00              	callq	0x140d4d740 <.text+0xd4c740>
140344887: 48 89 c3                    	movq	%rax, %rbx
14034488a: f6 00 01                    	testb	$0x1, (%rax)
14034488d: 74 11                       	je	0x1403448a0 <.text+0x3438a0>
14034488f: 48 89 d9                    	movq	%rbx, %rcx
140344892: 48 83 c1 08                 	addq	$0x8, %rcx
140344896: 48 8b 43 08                 	movq	0x8(%rbx), %rax
14034489a: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14034489e: eb 1b                       	jmp	0x1403448bb <.text+0x3438bb>
1403448a0: e8 5b 05 a2 00              	callq	0x140d64e00 <.text+0xd63e00>
1403448a5: 48 89 d9                    	movq	%rbx, %rcx
1403448a8: 48 83 c1 08                 	addq	$0x8, %rcx
1403448ac: 48 c7 03 01 00 00 00        	movq	$0x1, (%rbx)
1403448b3: 48 89 43 08                 	movq	%rax, 0x8(%rbx)
1403448b7: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1403448bb: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
1403448bf: 4c 89 01                    	movq	%r8, (%rcx)
1403448c2: 0f 29 75 f0                 	movaps	%xmm6, -0x10(%rbp)
1403448c6: 0f 29 7d e0                 	movaps	%xmm7, -0x20(%rbp)
1403448ca: 48 89 45 00                 	movq	%rax, (%rbp)
1403448ce: 48 89 55 08                 	movq	%rdx, 0x8(%rbp)
1403448d2: e8 69 8e a0 00              	callq	0x140d4d740 <.text+0xd4c740>
1403448d7: 48 89 c3                    	movq	%rax, %rbx
1403448da: f6 00 01                    	testb	$0x1, (%rax)
1403448dd: 74 11                       	je	0x1403448f0 <.text+0x3438f0>
1403448df: 48 89 d9                    	movq	%rbx, %rcx
1403448e2: 48 83 c1 08                 	addq	$0x8, %rcx
1403448e6: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1403448ea: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403448ee: eb 1b                       	jmp	0x14034490b <.text+0x34390b>
1403448f0: e8 0b 05 a2 00              	callq	0x140d64e00 <.text+0xd63e00>
1403448f5: 48 89 d9                    	movq	%rbx, %rcx
1403448f8: 48 83 c1 08                 	addq	$0x8, %rcx
1403448fc: 48 c7 03 01 00 00 00        	movq	$0x1, (%rbx)
140344903: 48 89 43 08                 	movq	%rax, 0x8(%rbx)
140344907: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
14034490b: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
14034490f: 4c 89 01                    	movq	%r8, (%rcx)
140344912: 0f 29 75 20                 	movaps	%xmm6, 0x20(%rbp)
140344916: 0f 29 7d 10                 	movaps	%xmm7, 0x10(%rbp)
14034491a: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
14034491e: 48 89 55 38                 	movq	%rdx, 0x38(%rbp)
140344922: e8 19 8e a0 00              	callq	0x140d4d740 <.text+0xd4c740>
140344927: 48 89 c3                    	movq	%rax, %rbx
14034492a: f6 00 01                    	testb	$0x1, (%rax)
14034492d: 74 11                       	je	0x140344940 <.text+0x343940>
14034492f: 48 89 d9                    	movq	%rbx, %rcx
140344932: 48 83 c1 08                 	addq	$0x8, %rcx
140344936: 48 8b 43 08                 	movq	0x8(%rbx), %rax
14034493a: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14034493e: eb 1b                       	jmp	0x14034495b <.text+0x34395b>
140344940: e8 bb 04 a2 00              	callq	0x140d64e00 <.text+0xd63e00>
140344945: 48 89 d9                    	movq	%rbx, %rcx
140344948: 48 83 c1 08                 	addq	$0x8, %rcx
14034494c: 48 c7 03 01 00 00 00        	movq	$0x1, (%rbx)
140344953: 48 89 43 08                 	movq	%rax, 0x8(%rbx)
140344957: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
14034495b: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
14034495f: 4c 89 01                    	movq	%r8, (%rcx)
140344962: 0f 29 b5 90 00 00 00        	movaps	%xmm6, 0x90(%rbp)
140344969: 0f 29 bd 80 00 00 00        	movaps	%xmm7, 0x80(%rbp)
140344970: 48 89 85 a0 00 00 00        	movq	%rax, 0xa0(%rbp)
140344977: 48 89 95 a8 00 00 00        	movq	%rdx, 0xa8(%rbp)
14034497e: e8 bd 8d a0 00              	callq	0x140d4d740 <.text+0xd4c740>
140344983: 48 89 c3                    	movq	%rax, %rbx
140344986: f6 00 01                    	testb	$0x1, (%rax)
140344989: 74 11                       	je	0x14034499c <.text+0x34399c>
14034498b: 48 89 d9                    	movq	%rbx, %rcx
14034498e: 48 83 c1 08                 	addq	$0x8, %rcx
140344992: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140344996: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14034499a: eb 1b                       	jmp	0x1403449b7 <.text+0x3439b7>
14034499c: e8 5f 04 a2 00              	callq	0x140d64e00 <.text+0xd63e00>
1403449a1: 48 89 d9                    	movq	%rbx, %rcx
1403449a4: 48 83 c1 08                 	addq	$0x8, %rcx
1403449a8: 48 c7 03 01 00 00 00        	movq	$0x1, (%rbx)
1403449af: 48 89 43 08                 	movq	%rax, 0x8(%rbx)
1403449b3: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1403449b7: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
1403449bb: 4c 89 01                    	movq	%r8, (%rcx)
1403449be: 0f 29 b5 c0 02 00 00        	movaps	%xmm6, 0x2c0(%rbp)
1403449c5: 0f 29 bd b0 02 00 00        	movaps	%xmm7, 0x2b0(%rbp)
1403449cc: 48 89 85 d0 02 00 00        	movq	%rax, 0x2d0(%rbp)
1403449d3: 48 89 95 d8 02 00 00        	movq	%rdx, 0x2d8(%rbp)
1403449da: 31 c9                       	xorl	%ecx, %ecx
1403449dc: e8 3f 51 d9 ff              	callq	0x1400d9b20 <.text+0xd8b20>
1403449e1: 48 85 c0                    	testq	%rax, %rax
1403449e4: 0f 84 a1 0a 00 00           	je	0x14034548b <.text+0x34448b>
1403449ea: 0f 10 00                    	movups	(%rax), %xmm0
1403449ed: 48 ff 00                    	incq	(%rax)
1403449f0: 0f 29 85 d0 05 00 00        	movaps	%xmm0, 0x5d0(%rbp)
1403449f7: 44 0f 29 8d b0 05 00 00     	movaps	%xmm9, 0x5b0(%rbp)
1403449ff: 44 0f 29 85 c0 05 00 00     	movaps	%xmm8, 0x5c0(%rbp)
140344a07: e8 34 8d a0 00              	callq	0x140d4d740 <.text+0xd4c740>
140344a0c: 48 89 c3                    	movq	%rax, %rbx
140344a0f: f6 00 01                    	testb	$0x1, (%rax)
140344a12: 74 11                       	je	0x140344a25 <.text+0x343a25>
140344a14: 48 89 d8                    	movq	%rbx, %rax
140344a17: 48 83 c0 08                 	addq	$0x8, %rax
140344a1b: 4c 8b 73 08                 	movq	0x8(%rbx), %r14
140344a1f: 4c 8b 7b 10                 	movq	0x10(%rbx), %r15
140344a23: eb 21                       	jmp	0x140344a46 <.text+0x343a46>
140344a25: e8 d6 03 a2 00              	callq	0x140d64e00 <.text+0xd63e00>
140344a2a: 49 89 c6                    	movq	%rax, %r14
140344a2d: 49 89 d7                    	movq	%rdx, %r15
140344a30: 48 89 d8                    	movq	%rbx, %rax
140344a33: 48 83 c0 08                 	addq	$0x8, %rax
140344a37: 48 c7 03 01 00 00 00        	movq	$0x1, (%rbx)
140344a3e: 4c 89 73 08                 	movq	%r14, 0x8(%rbx)
140344a42: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140344a46: 49 8d 4e 01                 	leaq	0x1(%r14), %rcx
140344a4a: 48 89 08                    	movq	%rcx, (%rax)
140344a4d: 4c 8d a5 20 03 00 00        	leaq	0x320(%rbp), %r12
140344a54: 48 8d 9d f0 00 00 00        	leaq	0xf0(%rbp), %rbx
140344a5b: 41 b8 90 00 00 00           	movl	$0x90, %r8d
140344a61: 4c 89 e1                    	movq	%r12, %rcx
140344a64: 48 89 da                    	movq	%rbx, %rdx
140344a67: e8 94 f1 5d 02              	callq	0x142923c00 <.text+0x2922c00>
140344a6c: 0f 28 45 e0                 	movaps	-0x20(%rbp), %xmm0
140344a70: 0f 28 4d f0                 	movaps	-0x10(%rbp), %xmm1
140344a74: 0f 28 55 00                 	movaps	(%rbp), %xmm2
140344a78: 0f 11 85 b0 03 00 00        	movups	%xmm0, 0x3b0(%rbp)
140344a7f: 0f 11 8d c0 03 00 00        	movups	%xmm1, 0x3c0(%rbp)
140344a86: 0f 11 95 d0 03 00 00        	movups	%xmm2, 0x3d0(%rbp)
140344a8d: 0f 28 45 10                 	movaps	0x10(%rbp), %xmm0
140344a91: 0f 28 4d 20                 	movaps	0x20(%rbp), %xmm1
140344a95: 0f 28 55 30                 	movaps	0x30(%rbp), %xmm2
140344a99: 0f 11 85 e0 03 00 00        	movups	%xmm0, 0x3e0(%rbp)
140344aa0: 0f 11 8d f0 03 00 00        	movups	%xmm1, 0x3f0(%rbp)
140344aa7: 0f 11 95 00 04 00 00        	movups	%xmm2, 0x400(%rbp)
140344aae: 0f 28 85 80 00 00 00        	movaps	0x80(%rbp), %xmm0
140344ab5: 0f 28 8d 90 00 00 00        	movaps	0x90(%rbp), %xmm1
140344abc: 0f 28 95 a0 00 00 00        	movaps	0xa0(%rbp), %xmm2
140344ac3: 0f 11 85 10 04 00 00        	movups	%xmm0, 0x410(%rbp)
140344aca: 0f 11 8d 20 04 00 00        	movups	%xmm1, 0x420(%rbp)
140344ad1: 0f 11 95 30 04 00 00        	movups	%xmm2, 0x430(%rbp)
140344ad8: 0f 28 85 b0 02 00 00        	movaps	0x2b0(%rbp), %xmm0
140344adf: 0f 28 8d c0 02 00 00        	movaps	0x2c0(%rbp), %xmm1
140344ae6: 0f 28 95 d0 02 00 00        	movaps	0x2d0(%rbp), %xmm2
140344aed: 0f 11 95 60 04 00 00        	movups	%xmm2, 0x460(%rbp)
140344af4: 0f 11 8d 50 04 00 00        	movups	%xmm1, 0x450(%rbp)
140344afb: 0f 11 85 40 04 00 00        	movups	%xmm0, 0x440(%rbp)
140344b02: 0f 28 85 b0 05 00 00        	movaps	0x5b0(%rbp), %xmm0
140344b09: 0f 28 8d c0 05 00 00        	movaps	0x5c0(%rbp), %xmm1
140344b10: 0f 28 95 d0 05 00 00        	movaps	0x5d0(%rbp), %xmm2
140344b17: 0f 11 85 70 04 00 00        	movups	%xmm0, 0x470(%rbp)
140344b1e: 0f 11 8d 80 04 00 00        	movups	%xmm1, 0x480(%rbp)
140344b25: 0f 11 95 90 04 00 00        	movups	%xmm2, 0x490(%rbp)
140344b2c: 0f 11 bd a0 04 00 00        	movups	%xmm7, 0x4a0(%rbp)
140344b33: 0f 11 b5 b0 04 00 00        	movups	%xmm6, 0x4b0(%rbp)
140344b3a: 4c 89 b5 c0 04 00 00        	movq	%r14, 0x4c0(%rbp)
140344b41: 4c 89 bd c8 04 00 00        	movq	%r15, 0x4c8(%rbp)
140344b48: 48 89 f9                    	movq	%rdi, %rcx
140344b4b: 4c 89 e2                    	movq	%r12, %rdx
140344b4e: e8 bd 5b df ff              	callq	0x14013a710 <.text+0x139710>
140344b53: 48 89 c7                    	movq	%rax, %rdi
140344b56: e8 e5 8b a0 00              	callq	0x140d4d740 <.text+0xd4c740>
140344b5b: 49 89 c6                    	movq	%rax, %r14
140344b5e: f6 00 01                    	testb	$0x1, (%rax)
140344b61: 74 11                       	je	0x140344b74 <.text+0x343b74>
140344b63: 4c 89 f1                    	movq	%r14, %rcx
140344b66: 48 83 c1 08                 	addq	$0x8, %rcx
140344b6a: 49 8b 46 08                 	movq	0x8(%r14), %rax
140344b6e: 49 8b 56 10                 	movq	0x10(%r14), %rdx
140344b72: eb 1b                       	jmp	0x140344b8f <.text+0x343b8f>
140344b74: e8 87 02 a2 00              	callq	0x140d64e00 <.text+0xd63e00>
140344b79: 4c 89 f1                    	movq	%r14, %rcx
140344b7c: 48 83 c1 08                 	addq	$0x8, %rcx
140344b80: 49 c7 06 01 00 00 00        	movq	$0x1, (%r14)
140344b87: 49 89 46 08                 	movq	%rax, 0x8(%r14)
140344b8b: 49 89 56 10                 	movq	%rdx, 0x10(%r14)
140344b8f: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
140344b93: 4c 89 01                    	movq	%r8, (%rcx)
140344b96: 0f 29 b5 00 01 00 00        	movaps	%xmm6, 0x100(%rbp)
140344b9d: 0f 29 bd f0 00 00 00        	movaps	%xmm7, 0xf0(%rbp)
140344ba4: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
140344bab: 48 89 95 18 01 00 00        	movq	%rdx, 0x118(%rbp)
140344bb2: e8 89 8b a0 00              	callq	0x140d4d740 <.text+0xd4c740>
140344bb7: 49 89 c6                    	movq	%rax, %r14
140344bba: f6 00 01                    	testb	$0x1, (%rax)
140344bbd: 74 11                       	je	0x140344bd0 <.text+0x343bd0>
140344bbf: 4c 89 f1                    	movq	%r14, %rcx
140344bc2: 48 83 c1 08                 	addq	$0x8, %rcx
140344bc6: 49 8b 46 08                 	movq	0x8(%r14), %rax
140344bca: 49 8b 56 10                 	movq	0x10(%r14), %rdx
140344bce: eb 1b                       	jmp	0x140344beb <.text+0x343beb>
140344bd0: e8 2b 02 a2 00              	callq	0x140d64e00 <.text+0xd63e00>
140344bd5: 4c 89 f1                    	movq	%r14, %rcx
140344bd8: 48 83 c1 08                 	addq	$0x8, %rcx
140344bdc: 49 c7 06 01 00 00 00        	movq	$0x1, (%r14)
140344be3: 49 89 46 08                 	movq	%rax, 0x8(%r14)
140344be7: 49 89 56 10                 	movq	%rdx, 0x10(%r14)
140344beb: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
140344bef: 4c 89 01                    	movq	%r8, (%rcx)
140344bf2: 0f 28 85 f0 00 00 00        	movaps	0xf0(%rbp), %xmm0
140344bf9: 0f 28 8d 00 01 00 00        	movaps	0x100(%rbp), %xmm1
140344c00: 0f 28 95 10 01 00 00        	movaps	0x110(%rbp), %xmm2
140344c07: 0f 29 95 40 03 00 00        	movaps	%xmm2, 0x340(%rbp)
140344c0e: 0f 29 8d 30 03 00 00        	movaps	%xmm1, 0x330(%rbp)
140344c15: 0f 29 85 20 03 00 00        	movaps	%xmm0, 0x320(%rbp)
140344c1c: 0f 29 bd 50 03 00 00        	movaps	%xmm7, 0x350(%rbp)
140344c23: 0f 29 b5 60 03 00 00        	movaps	%xmm6, 0x360(%rbp)
140344c2a: 48 89 85 70 03 00 00        	movq	%rax, 0x370(%rbp)
140344c31: 48 89 95 78 03 00 00        	movq	%rdx, 0x378(%rbp)
140344c38: 0f 57 c0                    	xorps	%xmm0, %xmm0
140344c3b: 0f 29 85 80 03 00 00        	movaps	%xmm0, 0x380(%rbp)
140344c42: 0f 29 85 90 03 00 00        	movaps	%xmm0, 0x390(%rbp)
140344c49: 48 c7 85 a0 03 00 00 00 00 00 00    	movq	$0x0, 0x3a0(%rbp)
140344c54: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
140344c5b: 48 89 f9                    	movq	%rdi, %rcx
140344c5e: e8 dd fb dc ff              	callq	0x140114840 <.text+0x113840>
140344c63: 48 89 c7                    	movq	%rax, %rdi
140344c66: e8 d5 8a a0 00              	callq	0x140d4d740 <.text+0xd4c740>
140344c6b: 49 89 c6                    	movq	%rax, %r14
140344c6e: f6 00 01                    	testb	$0x1, (%rax)
140344c71: 74 11                       	je	0x140344c84 <.text+0x343c84>
140344c73: 4c 89 f1                    	movq	%r14, %rcx
140344c76: 48 83 c1 08                 	addq	$0x8, %rcx
140344c7a: 49 8b 46 08                 	movq	0x8(%r14), %rax
140344c7e: 49 8b 56 10                 	movq	0x10(%r14), %rdx
140344c82: eb 1b                       	jmp	0x140344c9f <.text+0x343c9f>
140344c84: e8 77 01 a2 00              	callq	0x140d64e00 <.text+0xd63e00>
140344c89: 4c 89 f1                    	movq	%r14, %rcx
140344c8c: 48 83 c1 08                 	addq	$0x8, %rcx
140344c90: 49 c7 06 01 00 00 00        	movq	$0x1, (%r14)
140344c97: 49 89 46 08                 	movq	%rax, 0x8(%r14)
140344c9b: 49 89 56 10                 	movq	%rdx, 0x10(%r14)
140344c9f: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
140344ca3: 4c 89 01                    	movq	%r8, (%rcx)
140344ca6: 0f 11 b5 38 03 00 00        	movups	%xmm6, 0x338(%rbp)
140344cad: 0f 11 bd 28 03 00 00        	movups	%xmm7, 0x328(%rbp)
140344cb4: 48 89 85 48 03 00 00        	movq	%rax, 0x348(%rbp)
140344cbb: 48 89 95 50 03 00 00        	movq	%rdx, 0x350(%rbp)
140344cc2: c7 85 20 03 00 00 00 00 00 00       	movl	$0x0, 0x320(%rbp)
140344ccc: 0f 57 c0                    	xorps	%xmm0, %xmm0
140344ccf: 0f 11 85 58 03 00 00        	movups	%xmm0, 0x358(%rbp)
140344cd6: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
140344cdd: 48 89 f9                    	movq	%rdi, %rcx
140344ce0: e8 fb 1a df ff              	callq	0x1401367e0 <.text+0x1357e0>
140344ce5: 48 89 c7                    	movq	%rax, %rdi
140344ce8: e8 53 8a a0 00              	callq	0x140d4d740 <.text+0xd4c740>
140344ced: 49 89 c6                    	movq	%rax, %r14
140344cf0: f6 00 01                    	testb	$0x1, (%rax)
140344cf3: 74 11                       	je	0x140344d06 <.text+0x343d06>
140344cf5: 4c 89 f1                    	movq	%r14, %rcx
140344cf8: 48 83 c1 08                 	addq	$0x8, %rcx
140344cfc: 49 8b 46 08                 	movq	0x8(%r14), %rax
140344d00: 49 8b 56 10                 	movq	0x10(%r14), %rdx
140344d04: eb 1b                       	jmp	0x140344d21 <.text+0x343d21>
140344d06: e8 f5 00 a2 00              	callq	0x140d64e00 <.text+0xd63e00>
140344d0b: 4c 89 f1                    	movq	%r14, %rcx
140344d0e: 48 83 c1 08                 	addq	$0x8, %rcx
140344d12: 49 c7 06 01 00 00 00        	movq	$0x1, (%r14)
140344d19: 49 89 46 08                 	movq	%rax, 0x8(%r14)
140344d1d: 49 89 56 10                 	movq	%rdx, 0x10(%r14)
140344d21: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
140344d25: 4c 89 01                    	movq	%r8, (%rcx)
140344d28: 0f 11 b5 38 03 00 00        	movups	%xmm6, 0x338(%rbp)
140344d2f: 0f 11 bd 28 03 00 00        	movups	%xmm7, 0x328(%rbp)
140344d36: 48 89 85 48 03 00 00        	movq	%rax, 0x348(%rbp)
140344d3d: 48 89 95 50 03 00 00        	movq	%rdx, 0x350(%rbp)
140344d44: c7 85 20 03 00 00 00 00 00 00       	movl	$0x0, 0x320(%rbp)
140344d4e: 0f 57 c0                    	xorps	%xmm0, %xmm0
140344d51: 0f 11 85 58 03 00 00        	movups	%xmm0, 0x358(%rbp)
140344d58: 0f 11 85 64 03 00 00        	movups	%xmm0, 0x364(%rbp)
140344d5f: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
140344d66: 48 89 f9                    	movq	%rdi, %rcx
140344d69: e8 42 09 df ff              	callq	0x1401356b0 <.text+0x1346b0>
140344d6e: 48 89 c7                    	movq	%rax, %rdi
140344d71: 48 8d 48 38                 	leaq	0x38(%rax), %rcx
140344d75: 48 ba 84 a7 2b 96 51 d2 1b 8e       	movabsq	$-0x71e42dae69d4587c, %rdx # imm = 0x8E1BD251962BA784
140344d7f: 49 b8 9b 52 e6 c0 74 23 78 6a       	movabsq	$0x6a782374c0e6529b, %r8 # imm = 0x6A782374C0E6529B
140344d89: e8 62 89 d9 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
140344d8e: 48 85 c0                    	testq	%rax, %rax
140344d91: 0f 84 37 06 00 00           	je	0x1403453ce <.text+0x3443ce>
140344d97: 48 39 97 18 02 00 00        	cmpq	%rdx, 0x218(%rdi)
140344d9e: 0f 86 2a 06 00 00           	jbe	0x1403453ce <.text+0x3443ce>
140344da4: 48 8b 87 10 02 00 00        	movq	0x210(%rdi), %rax
140344dab: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
140344daf: 48 85 c9                    	testq	%rcx, %rcx
140344db2: 0f 84 16 06 00 00           	je	0x1403453ce <.text+0x3443ce>
140344db8: 48 8b 87 e0 01 00 00        	movq	0x1e0(%rdi), %rax
140344dbf: 48 f7 d1                    	notq	%rcx
140344dc2: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140344dc6: 48 c1 e1 05                 	shlq	$0x5, %rcx
140344dca: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
140344dd0: 0f 84 f8 05 00 00           	je	0x1403453ce <.text+0x3443ce>
140344dd6: 48 01 c8                    	addq	%rcx, %rax
140344dd9: 48 8b 48 10                 	movq	0x10(%rax), %rcx
140344ddd: 8b 97 b0 05 00 00           	movl	0x5b0(%rdi), %edx
140344de3: 89 50 5c                    	movl	%edx, 0x5c(%rax)
140344de6: e8 c5 c1 e2 ff              	callq	0x140170fb0 <.text+0x16ffb0>
140344deb: 48 89 f9                    	movq	%rdi, %rcx
140344dee: e8 5d 25 fe ff              	callq	0x140327350 <.text+0x326350>
140344df3: 48 8d b5 20 03 00 00        	leaq	0x320(%rbp), %rsi
140344dfa: 48 89 f1                    	movq	%rsi, %rcx
140344dfd: e8 0e d2 d6 ff              	callq	0x1400b2010 <.text+0xb1010>
140344e02: 4c 8d 0d 97 e0 67 02        	leaq	0x267e097(%rip), %r9    # 0x1429c2ea0
140344e09: 48 89 d9                    	movq	%rbx, %rcx
140344e0c: 48 89 f2                    	movq	%rsi, %rdx
140344e0f: 41 b0 0a                    	movb	$0xa, %r8b
140344e12: e8 a9 09 f5 ff              	callq	0x1402957c0 <.text+0x2947c0>
140344e17: 48 89 f9                    	movq	%rdi, %rcx
140344e1a: 48 89 da                    	movq	%rbx, %rdx
140344e1d: e8 ce 28 fe ff              	callq	0x1403276f0 <.text+0x3266f0>
140344e22: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140344e26: e8 35 05 dc ff              	callq	0x140105360 <.text+0x104360>
140344e2b: 48 8d 8d b0 05 00 00        	leaq	0x5b0(%rbp), %rcx
140344e32: e8 d9 f9 f8 ff              	callq	0x1402d4810 <.text+0x2d3810>
140344e37: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
140344e3e: e8 ed e1 20 00              	callq	0x140553030 <.text+0x552030>
140344e43: 0f 10 85 10 06 00 00        	movups	0x610(%rbp), %xmm0
140344e4a: 0f 29 85 80 03 00 00        	movaps	%xmm0, 0x380(%rbp)
140344e51: 0f 10 85 00 06 00 00        	movups	0x600(%rbp), %xmm0
140344e58: 0f 29 85 70 03 00 00        	movaps	%xmm0, 0x370(%rbp)
140344e5f: 0f 10 85 f0 05 00 00        	movups	0x5f0(%rbp), %xmm0
140344e66: 0f 29 85 60 03 00 00        	movaps	%xmm0, 0x360(%rbp)
140344e6d: 0f 10 85 b0 05 00 00        	movups	0x5b0(%rbp), %xmm0
140344e74: 0f 10 8d c0 05 00 00        	movups	0x5c0(%rbp), %xmm1
140344e7b: 0f 10 95 d0 05 00 00        	movups	0x5d0(%rbp), %xmm2
140344e82: 0f 10 9d e0 05 00 00        	movups	0x5e0(%rbp), %xmm3
140344e89: 0f 29 9d 50 03 00 00        	movaps	%xmm3, 0x350(%rbp)
140344e90: 0f 29 95 40 03 00 00        	movaps	%xmm2, 0x340(%rbp)
140344e97: 0f 29 8d 30 03 00 00        	movaps	%xmm1, 0x330(%rbp)
140344e9e: 0f 29 85 20 03 00 00        	movaps	%xmm0, 0x320(%rbp)
140344ea5: 0f 10 85 f0 00 00 00        	movups	0xf0(%rbp), %xmm0
140344eac: 0f 10 8d 00 01 00 00        	movups	0x100(%rbp), %xmm1
140344eb3: 0f 10 95 10 01 00 00        	movups	0x110(%rbp), %xmm2
140344eba: 0f 10 9d 20 01 00 00        	movups	0x120(%rbp), %xmm3
140344ec1: 0f 29 85 90 03 00 00        	movaps	%xmm0, 0x390(%rbp)
140344ec8: 0f 29 8d a0 03 00 00        	movaps	%xmm1, 0x3a0(%rbp)
140344ecf: 0f 29 95 b0 03 00 00        	movaps	%xmm2, 0x3b0(%rbp)
140344ed6: 0f 29 9d c0 03 00 00        	movaps	%xmm3, 0x3c0(%rbp)
140344edd: 0f 10 85 30 01 00 00        	movups	0x130(%rbp), %xmm0
140344ee4: 0f 29 85 d0 03 00 00        	movaps	%xmm0, 0x3d0(%rbp)
140344eeb: 0f 10 85 40 01 00 00        	movups	0x140(%rbp), %xmm0
140344ef2: 0f 29 85 e0 03 00 00        	movaps	%xmm0, 0x3e0(%rbp)
140344ef9: 0f 10 85 50 01 00 00        	movups	0x150(%rbp), %xmm0
140344f00: 0f 29 85 f0 03 00 00        	movaps	%xmm0, 0x3f0(%rbp)
140344f07: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
140344f0e: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
140344f15: e8 76 78 f2 ff              	callq	0x14026c790 <.text+0x26b790>
140344f1a: 48 8d 8d b0 02 00 00        	leaq	0x2b0(%rbp), %rcx
140344f21: e8 ea 24 f4 ff              	callq	0x140287410 <.text+0x286410>
140344f26: 48 8d 8d b0 05 00 00        	leaq	0x5b0(%rbp), %rcx
140344f2d: e8 de 16 f4 ff              	callq	0x140286610 <.text+0x285610>
140344f32: 48 8d 8d 20 03 00 00        	leaq	0x320(%rbp), %rcx
140344f39: e8 42 de 14 00              	callq	0x140492d80 <.text+0x491d80>
140344f3e: 0f 10 85 80 03 00 00        	movups	0x380(%rbp), %xmm0
140344f45: 0f 29 85 50 01 00 00        	movaps	%xmm0, 0x150(%rbp)
140344f4c: 0f 10 85 70 03 00 00        	movups	0x370(%rbp), %xmm0
140344f53: 0f 29 85 40 01 00 00        	movaps	%xmm0, 0x140(%rbp)
140344f5a: 0f 10 85 60 03 00 00        	movups	0x360(%rbp), %xmm0
140344f61: 0f 29 85 30 01 00 00        	movaps	%xmm0, 0x130(%rbp)
140344f68: 0f 10 85 20 03 00 00        	movups	0x320(%rbp), %xmm0
140344f6f: 0f 10 8d 30 03 00 00        	movups	0x330(%rbp), %xmm1
140344f76: 0f 10 95 40 03 00 00        	movups	0x340(%rbp), %xmm2
140344f7d: 0f 10 9d 50 03 00 00        	movups	0x350(%rbp), %xmm3
140344f84: 0f 29 9d 20 01 00 00        	movaps	%xmm3, 0x120(%rbp)
140344f8b: 0f 29 95 10 01 00 00        	movaps	%xmm2, 0x110(%rbp)
140344f92: 0f 29 8d 00 01 00 00        	movaps	%xmm1, 0x100(%rbp)
140344f99: 0f 29 85 f0 00 00 00        	movaps	%xmm0, 0xf0(%rbp)
140344fa0: 48 8d 8d 20 03 00 00        	leaq	0x320(%rbp), %rcx
140344fa7: 48 8d 95 f0 00 00 00        	leaq	0xf0(%rbp), %rdx
140344fae: e8 2d 1d 09 00              	callq	0x1403d6ce0 <.text+0x3d5ce0>
140344fb3: 0f 10 85 b0 05 00 00        	movups	0x5b0(%rbp), %xmm0
140344fba: 0f 10 8d c0 05 00 00        	movups	0x5c0(%rbp), %xmm1
140344fc1: 0f 10 95 d0 05 00 00        	movups	0x5d0(%rbp), %xmm2
140344fc8: 0f 10 9d e0 05 00 00        	movups	0x5e0(%rbp), %xmm3
140344fcf: 0f 29 85 f0 00 00 00        	movaps	%xmm0, 0xf0(%rbp)
140344fd6: 0f 29 8d 00 01 00 00        	movaps	%xmm1, 0x100(%rbp)
140344fdd: 0f 29 95 10 01 00 00        	movaps	%xmm2, 0x110(%rbp)
140344fe4: 0f 29 9d 20 01 00 00        	movaps	%xmm3, 0x120(%rbp)
140344feb: 0f 10 85 f0 05 00 00        	movups	0x5f0(%rbp), %xmm0
140344ff2: 0f 29 85 30 01 00 00        	movaps	%xmm0, 0x130(%rbp)
140344ff9: 0f 10 85 00 06 00 00        	movups	0x600(%rbp), %xmm0
140345000: 0f 29 85 40 01 00 00        	movaps	%xmm0, 0x140(%rbp)
140345007: 0f 10 85 10 06 00 00        	movups	0x610(%rbp), %xmm0
14034500e: 0f 29 85 50 01 00 00        	movaps	%xmm0, 0x150(%rbp)
140345015: 0f 10 85 20 03 00 00        	movups	0x320(%rbp), %xmm0
14034501c: 0f 10 8d 30 03 00 00        	movups	0x330(%rbp), %xmm1
140345023: 0f 10 95 40 03 00 00        	movups	0x340(%rbp), %xmm2
14034502a: 0f 10 9d 50 03 00 00        	movups	0x350(%rbp), %xmm3
140345031: 0f 29 85 60 01 00 00        	movaps	%xmm0, 0x160(%rbp)
140345038: 0f 29 8d 70 01 00 00        	movaps	%xmm1, 0x170(%rbp)
14034503f: 0f 29 95 80 01 00 00        	movaps	%xmm2, 0x180(%rbp)
140345046: 0f 29 9d 90 01 00 00        	movaps	%xmm3, 0x190(%rbp)
14034504d: 0f 10 85 60 03 00 00        	movups	0x360(%rbp), %xmm0
140345054: 0f 29 85 a0 01 00 00        	movaps	%xmm0, 0x1a0(%rbp)
14034505b: 0f 10 85 70 03 00 00        	movups	0x370(%rbp), %xmm0
140345062: 0f 29 85 b0 01 00 00        	movaps	%xmm0, 0x1b0(%rbp)
140345069: 0f 10 85 80 03 00 00        	movups	0x380(%rbp), %xmm0
140345070: 0f 29 85 c0 01 00 00        	movaps	%xmm0, 0x1c0(%rbp)
140345077: 0f 10 45 70                 	movups	0x70(%rbp), %xmm0
14034507b: 0f 29 85 80 03 00 00        	movaps	%xmm0, 0x380(%rbp)
140345082: 0f 10 45 60                 	movups	0x60(%rbp), %xmm0
140345086: 0f 29 85 70 03 00 00        	movaps	%xmm0, 0x370(%rbp)
14034508d: 0f 10 45 50                 	movups	0x50(%rbp), %xmm0
140345091: 0f 29 85 60 03 00 00        	movaps	%xmm0, 0x360(%rbp)
140345098: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
14034509c: 0f 10 4d 20                 	movups	0x20(%rbp), %xmm1
1403450a0: 0f 10 55 30                 	movups	0x30(%rbp), %xmm2
1403450a4: 0f 10 5d 40                 	movups	0x40(%rbp), %xmm3
1403450a8: 0f 29 9d 50 03 00 00        	movaps	%xmm3, 0x350(%rbp)
1403450af: 0f 29 95 40 03 00 00        	movaps	%xmm2, 0x340(%rbp)
1403450b6: 0f 29 8d 30 03 00 00        	movaps	%xmm1, 0x330(%rbp)
1403450bd: 0f 29 85 20 03 00 00        	movaps	%xmm0, 0x320(%rbp)
1403450c4: 0f 10 85 e0 00 00 00        	movups	0xe0(%rbp), %xmm0
1403450cb: 0f 29 85 f0 03 00 00        	movaps	%xmm0, 0x3f0(%rbp)
1403450d2: 0f 10 85 d0 00 00 00        	movups	0xd0(%rbp), %xmm0
1403450d9: 0f 29 85 e0 03 00 00        	movaps	%xmm0, 0x3e0(%rbp)
1403450e0: 0f 10 85 c0 00 00 00        	movups	0xc0(%rbp), %xmm0
1403450e7: 0f 29 85 d0 03 00 00        	movaps	%xmm0, 0x3d0(%rbp)
1403450ee: 0f 10 85 80 00 00 00        	movups	0x80(%rbp), %xmm0
1403450f5: 0f 10 8d 90 00 00 00        	movups	0x90(%rbp), %xmm1
1403450fc: 0f 10 95 a0 00 00 00        	movups	0xa0(%rbp), %xmm2
140345103: 0f 10 9d b0 00 00 00        	movups	0xb0(%rbp), %xmm3
14034510a: 0f 29 9d c0 03 00 00        	movaps	%xmm3, 0x3c0(%rbp)
140345111: 0f 29 95 b0 03 00 00        	movaps	%xmm2, 0x3b0(%rbp)
140345118: 0f 29 8d a0 03 00 00        	movaps	%xmm1, 0x3a0(%rbp)
14034511f: 0f 29 85 90 03 00 00        	movaps	%xmm0, 0x390(%rbp)
140345126: 0f 10 85 10 03 00 00        	movups	0x310(%rbp), %xmm0
14034512d: 0f 29 85 60 04 00 00        	movaps	%xmm0, 0x460(%rbp)
140345134: 0f 10 85 00 03 00 00        	movups	0x300(%rbp), %xmm0
14034513b: 0f 29 85 50 04 00 00        	movaps	%xmm0, 0x450(%rbp)
140345142: 0f 10 85 f0 02 00 00        	movups	0x2f0(%rbp), %xmm0
140345149: 0f 29 85 40 04 00 00        	movaps	%xmm0, 0x440(%rbp)
140345150: 0f 10 85 b0 02 00 00        	movups	0x2b0(%rbp), %xmm0
140345157: 0f 10 8d c0 02 00 00        	movups	0x2c0(%rbp), %xmm1
14034515e: 0f 10 95 d0 02 00 00        	movups	0x2d0(%rbp), %xmm2
140345165: 0f 10 9d e0 02 00 00        	movups	0x2e0(%rbp), %xmm3
14034516c: 0f 29 9d 30 04 00 00        	movaps	%xmm3, 0x430(%rbp)
140345173: 0f 29 95 20 04 00 00        	movaps	%xmm2, 0x420(%rbp)
14034517a: 0f 29 8d 10 04 00 00        	movaps	%xmm1, 0x410(%rbp)
140345181: 0f 29 85 00 04 00 00        	movaps	%xmm0, 0x400(%rbp)
140345188: 48 8d 8d 70 04 00 00        	leaq	0x470(%rbp), %rcx
14034518f: 48 8d 95 f0 00 00 00        	leaq	0xf0(%rbp), %rdx
140345196: 41 b8 e0 00 00 00           	movl	$0xe0, %r8d
14034519c: e8 5f ea 5d 02              	callq	0x142923c00 <.text+0x2922c00>
1403451a1: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
1403451a8: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
1403451af: e8 3c 01 d9 ff              	callq	0x1400d52f0 <.text+0xd42f0>
1403451b4: c6 85 b0 05 00 00 19        	movb	$0x19, 0x5b0(%rbp)
1403451bb: 48 8b 0d ae 71 2e 03        	movq	0x32e71ae(%rip), %rcx   # 0x14362c370
1403451c2: 4c 8d 05 cf 5d 5e 02        	leaq	0x25e5dcf(%rip), %r8    # 0x14292af98
1403451c9: 48 8d 95 b0 05 00 00        	leaq	0x5b0(%rbp), %rdx
1403451d0: e8 4b fb 01 00              	callq	0x140364d20 <.text+0x363d20>
1403451d5: 49 89 d0                    	movq	%rdx, %r8
1403451d8: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
1403451df: 48 89 c2                    	movq	%rax, %rdx
1403451e2: e8 d9 16 d9 ff              	callq	0x1400d68c0 <.text+0xd58c0>
1403451e7: 0f 10 85 50 01 00 00        	movups	0x150(%rbp), %xmm0
1403451ee: 0f 29 85 80 03 00 00        	movaps	%xmm0, 0x380(%rbp)
1403451f5: 0f 10 85 40 01 00 00        	movups	0x140(%rbp), %xmm0
1403451fc: 0f 29 85 70 03 00 00        	movaps	%xmm0, 0x370(%rbp)
140345203: 0f 10 85 30 01 00 00        	movups	0x130(%rbp), %xmm0
14034520a: 0f 29 85 60 03 00 00        	movaps	%xmm0, 0x360(%rbp)
140345211: 0f 10 85 f0 00 00 00        	movups	0xf0(%rbp), %xmm0
140345218: 0f 10 8d 00 01 00 00        	movups	0x100(%rbp), %xmm1
14034521f: 0f 10 95 10 01 00 00        	movups	0x110(%rbp), %xmm2
140345226: 0f 10 9d 20 01 00 00        	movups	0x120(%rbp), %xmm3
14034522d: 0f 29 9d 50 03 00 00        	movaps	%xmm3, 0x350(%rbp)
140345234: 0f 29 95 40 03 00 00        	movaps	%xmm2, 0x340(%rbp)
14034523b: 0f 29 8d 30 03 00 00        	movaps	%xmm1, 0x330(%rbp)
140345242: 0f 29 85 20 03 00 00        	movaps	%xmm0, 0x320(%rbp)
140345249: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
140345250: 48 89 f9                    	movq	%rdi, %rcx
140345253: e8 98 24 fe ff              	callq	0x1403276f0 <.text+0x3266f0>
140345258: 4c 8d 05 59 dc 67 02        	leaq	0x267dc59(%rip), %r8    # 0x1429c2eb8
14034525f: 48 8d 8d b0 05 00 00        	leaq	0x5b0(%rbp), %rcx
140345266: b2 28                       	movb	$0x28, %dl
140345268: e8 53 dd fd ff              	callq	0x140322fc0 <.text+0x321fc0>
14034526d: 4c 8d 05 5c dc 67 02        	leaq	0x267dc5c(%rip), %r8    # 0x1429c2ed0
140345274: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
14034527b: b2 22                       	movb	$0x22, %dl
14034527d: e8 de 2d f1 ff              	callq	0x140258060 <.text+0x257060>
140345282: 0f 10 85 10 06 00 00        	movups	0x610(%rbp), %xmm0
140345289: 0f 29 85 80 03 00 00        	movaps	%xmm0, 0x380(%rbp)
140345290: 0f 10 85 00 06 00 00        	movups	0x600(%rbp), %xmm0
140345297: 0f 29 85 70 03 00 00        	movaps	%xmm0, 0x370(%rbp)
14034529e: 0f 10 85 f0 05 00 00        	movups	0x5f0(%rbp), %xmm0
1403452a5: 0f 29 85 60 03 00 00        	movaps	%xmm0, 0x360(%rbp)
1403452ac: 0f 10 85 b0 05 00 00        	movups	0x5b0(%rbp), %xmm0
1403452b3: 0f 10 8d c0 05 00 00        	movups	0x5c0(%rbp), %xmm1
1403452ba: 0f 10 95 d0 05 00 00        	movups	0x5d0(%rbp), %xmm2
1403452c1: 0f 10 9d e0 05 00 00        	movups	0x5e0(%rbp), %xmm3
1403452c8: 0f 29 9d 50 03 00 00        	movaps	%xmm3, 0x350(%rbp)
1403452cf: 0f 29 95 40 03 00 00        	movaps	%xmm2, 0x340(%rbp)
1403452d6: 0f 29 8d 30 03 00 00        	movaps	%xmm1, 0x330(%rbp)
1403452dd: 0f 29 85 20 03 00 00        	movaps	%xmm0, 0x320(%rbp)
1403452e4: 0f 10 85 f0 00 00 00        	movups	0xf0(%rbp), %xmm0
1403452eb: 0f 10 8d 00 01 00 00        	movups	0x100(%rbp), %xmm1
1403452f2: 0f 10 95 10 01 00 00        	movups	0x110(%rbp), %xmm2
1403452f9: 0f 10 9d 20 01 00 00        	movups	0x120(%rbp), %xmm3
140345300: 0f 29 85 90 03 00 00        	movaps	%xmm0, 0x390(%rbp)
140345307: 0f 29 8d a0 03 00 00        	movaps	%xmm1, 0x3a0(%rbp)
14034530e: 0f 29 95 b0 03 00 00        	movaps	%xmm2, 0x3b0(%rbp)
140345315: 0f 29 9d c0 03 00 00        	movaps	%xmm3, 0x3c0(%rbp)
14034531c: 0f 10 85 30 01 00 00        	movups	0x130(%rbp), %xmm0
140345323: 0f 29 85 d0 03 00 00        	movaps	%xmm0, 0x3d0(%rbp)
14034532a: 0f 10 85 40 01 00 00        	movups	0x140(%rbp), %xmm0
140345331: 0f 29 85 e0 03 00 00        	movaps	%xmm0, 0x3e0(%rbp)
140345338: 0f 10 85 50 01 00 00        	movups	0x150(%rbp), %xmm0
14034533f: 0f 29 85 f0 03 00 00        	movaps	%xmm0, 0x3f0(%rbp)
140345346: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
14034534d: 48 89 f9                    	movq	%rdi, %rcx
140345350: e8 fb 25 fe ff              	callq	0x140327950 <.text+0x326950>
140345355: 4c 8d 05 8c db 67 02        	leaq	0x267db8c(%rip), %r8    # 0x1429c2ee8
14034535c: 48 89 f1                    	movq	%rsi, %rcx
14034535f: b2 24                       	movb	$0x24, %dl
140345361: e8 ca 3d d8 ff              	callq	0x1400c9130 <.text+0xc8130>
140345366: 48 89 f9                    	movq	%rdi, %rcx
140345369: 48 89 f2                    	movq	%rsi, %rdx
14034536c: e8 7f 23 fe ff              	callq	0x1403276f0 <.text+0x3266f0>
140345371: 48 89 f9                    	movq	%rdi, %rcx
140345374: b2 01                       	movb	$0x1, %dl
140345376: 0f 28 b5 50 06 00 00        	movaps	0x650(%rbp), %xmm6
14034537d: 0f 28 bd 60 06 00 00        	movaps	0x660(%rbp), %xmm7
140345384: 44 0f 28 85 70 06 00 00     	movaps	0x670(%rbp), %xmm8
14034538c: 44 0f 28 8d 80 06 00 00     	movaps	0x680(%rbp), %xmm9
140345394: 44 0f 28 95 90 06 00 00     	movaps	0x690(%rbp), %xmm10
14034539c: 44 0f 28 9d a0 06 00 00     	movaps	0x6a0(%rbp), %xmm11
1403453a4: 44 0f 28 a5 b0 06 00 00     	movaps	0x6b0(%rbp), %xmm12
1403453ac: 48 81 c4 40 07 00 00        	addq	$0x740, %rsp            # imm = 0x740
1403453b3: 5b                          	popq	%rbx
1403453b4: 5f                          	popq	%rdi
1403453b5: 5e                          	popq	%rsi
1403453b6: 41 5c                       	popq	%r12
1403453b8: 41 5e                       	popq	%r14
1403453ba: 41 5f                       	popq	%r15
1403453bc: 5d                          	popq	%rbp
1403453bd: e9 be 1b fe ff              	jmp	0x140326f80 <.text+0x325f80>
1403453c2: 48 8d 0d 47 d6 5e 02        	leaq	0x25ed647(%rip), %rcx   # 0x142932a10
1403453c9: e8 02 2d 59 02              	callq	0x1428d80d0 <.text+0x28d70d0>
1403453ce: 48 8d 05 63 8e 67 02        	leaq	0x2678e63(%rip), %rax   # 0x1429be238
1403453d5: 48 89 85 f0 00 00 00        	movq	%rax, 0xf0(%rbp)
1403453dc: 48 c7 85 f8 00 00 00 27 00 00 00    	movq	$0x27, 0xf8(%rbp)
1403453e7: 48 89 9d b0 05 00 00        	movq	%rbx, 0x5b0(%rbp)
1403453ee: 48 8d 05 9b 4a dc ff        	leaq	-0x23b565(%rip), %rax   # 0x140109e90 <.text+0x108e90>
1403453f5: 48 89 85 b8 05 00 00        	movq	%rax, 0x5b8(%rbp)
1403453fc: 48 8d 05 65 8f 67 02        	leaq	0x2678f65(%rip), %rax   # 0x1429be368
140345403: 48 89 85 20 03 00 00        	movq	%rax, 0x320(%rbp)
14034540a: 48 c7 85 28 03 00 00 02 00 00 00    	movq	$0x2, 0x328(%rbp)
140345415: 48 c7 85 40 03 00 00 00 00 00 00    	movq	$0x0, 0x340(%rbp)
140345420: 48 89 b5 30 03 00 00        	movq	%rsi, 0x330(%rbp)
140345427: 48 c7 85 38 03 00 00 01 00 00 00    	movq	$0x1, 0x338(%rbp)
140345432: 48 8d 15 b7 8d 67 02        	leaq	0x2678db7(%rip), %rdx   # 0x1429be1f0
140345439: 48 8d 8d 20 03 00 00        	leaq	0x320(%rbp), %rcx
140345440: e8 4b 3f 59 02              	callq	0x1428d9390 <.text+0x28d8390>
140345445: 48 8d 0d c4 d5 5e 02        	leaq	0x25ed5c4(%rip), %rcx   # 0x142932a10
14034544c: e8 7f 2c 59 02              	callq	0x1428d80d0 <.text+0x28d70d0>
140345451: eb 44                       	jmp	0x140345497 <.text+0x344497>
140345453: 48 8d 0d b6 d5 5e 02        	leaq	0x25ed5b6(%rip), %rcx   # 0x142932a10
14034545a: e8 71 2c 59 02              	callq	0x1428d80d0 <.text+0x28d70d0>
14034545f: eb 36                       	jmp	0x140345497 <.text+0x344497>
140345461: 48 8d 0d a8 d5 5e 02        	leaq	0x25ed5a8(%rip), %rcx   # 0x142932a10
140345468: e8 63 2c 59 02              	callq	0x1428d80d0 <.text+0x28d70d0>
14034546d: eb 28                       	jmp	0x140345497 <.text+0x344497>
14034546f: 48 8d 0d 9a d5 5e 02        	leaq	0x25ed59a(%rip), %rcx   # 0x142932a10
140345476: e8 55 2c 59 02              	callq	0x1428d80d0 <.text+0x28d70d0>
14034547b: eb 1a                       	jmp	0x140345497 <.text+0x344497>
14034547d: 48 8d 0d 8c d5 5e 02        	leaq	0x25ed58c(%rip), %rcx   # 0x142932a10
140345484: e8 47 2c 59 02              	callq	0x1428d80d0 <.text+0x28d70d0>
140345489: eb 0c                       	jmp	0x140345497 <.text+0x344497>
14034548b: 48 8d 0d 7e d5 5e 02        	leaq	0x25ed57e(%rip), %rcx   # 0x142932a10
140345492: e8 39 2c 59 02              	callq	0x1428d80d0 <.text+0x28d70d0>
140345497: 0f 0b                       	ud2
140345499: 0f 1f 80 00 00 00 00        	nopl	(%rax)
