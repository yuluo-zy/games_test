
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1400fb890: 55                          	push	rbp
1400fb891: 41 57                       	push	r15
1400fb893: 41 56                       	push	r14
1400fb895: 41 55                       	push	r13
1400fb897: 41 54                       	push	r12
1400fb899: 56                          	push	rsi
1400fb89a: 57                          	push	rdi
1400fb89b: 53                          	push	rbx
1400fb89c: 48 81 ec 38 04 00 00        	sub	rsp, 0x438
1400fb8a3: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
1400fb8ab: 48 c7 85 b0 03 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x3b0], -0x2
1400fb8b6: 0f 28 85 20 04 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x420]
1400fb8bd: 0f 11 85 e0 02 00 00        	movups	xmmword ptr [rbp + 0x2e0], xmm0
1400fb8c4: 41 0f b6 40 1c              	movzx	eax, byte ptr [r8 + 0x1c]
1400fb8c9: 89 85 a8 03 00 00           	mov	dword ptr [rbp + 0x3a8], eax
1400fb8cf: 83 f8 0e                    	cmp	eax, 0xe
1400fb8d2: 0f 83 08 05 00 00           	jae	0x1400fbde0 <.text+0xfade0>
1400fb8d8: 4c 89 c7                    	mov	rdi, r8
1400fb8db: 41 83 78 08 7a              	cmp	dword ptr [r8 + 0x8], 0x7a
1400fb8e0: 0f 85 12 05 00 00           	jne	0x1400fbdf8 <.text+0xfadf8>
1400fb8e6: 4c 89 ce                    	mov	rsi, r9
1400fb8e9: 48 89 d3                    	mov	rbx, rdx
1400fb8ec: 49 89 ce                    	mov	r14, rcx
1400fb8ef: 48 8d 85 e0 02 00 00        	lea	rax, [rbp + 0x2e0]
1400fb8f6: 48 89 45 c0                 	mov	qword ptr [rbp - 0x40], rax
1400fb8fa: 48 8d 05 8f e5 00 00        	lea	rax, [rip + 0xe58f]     # 0x140109e90 <.text+0x108e90>
1400fb901: 48 89 45 c8                 	mov	qword ptr [rbp - 0x38], rax
1400fb905: 48 8d 05 fc 11 85 02        	lea	rax, [rip + 0x28511fc]  # 0x14294cb08
1400fb90c: 48 89 85 50 01 00 00        	mov	qword ptr [rbp + 0x150], rax
1400fb913: 48 c7 85 58 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x158], 0x1
1400fb91e: 48 8d 45 c0                 	lea	rax, [rbp - 0x40]
1400fb922: 48 89 85 60 01 00 00        	mov	qword ptr [rbp + 0x160], rax
1400fb929: 48 c7 85 68 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x168], 0x1
1400fb934: 48 c7 85 70 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x170], 0x0
1400fb93f: 48 8d 8d 60 03 00 00        	lea	rcx, [rbp + 0x360]
1400fb946: 48 8d 95 50 01 00 00        	lea	rdx, [rbp + 0x150]
1400fb94d: e8 3e e6 c7 00              	call	0x140d79f90 <.text+0xd78f90>
1400fb952: 48 8b 8d 68 03 00 00        	mov	rcx, qword ptr [rbp + 0x368]
1400fb959: 48 8b 95 70 03 00 00        	mov	rdx, qword ptr [rbp + 0x370]
1400fb960: 48 89 8d 58 03 00 00        	mov	qword ptr [rbp + 0x358], rcx
1400fb967: e8 84 3a b1 00              	call	0x140c0f3f0 <.text+0xc0e3f0>
1400fb96c: 49 89 c7                    	mov	r15, rax
1400fb96f: 4c 8b 60 f8                 	mov	r12, qword ptr [rax - 0x8]
1400fb973: 4c 8d 6d c0                 	lea	r13, [rbp - 0x40]
1400fb977: ba 18 00 00 00              	mov	edx, 0x18
1400fb97c: 4c 89 e9                    	mov	rcx, r13
1400fb97f: 41 b8 20 00 00 00           	mov	r8d, 0x20
1400fb985: e8 96 94 ad 00              	call	0x140bd4e20 <.text+0xbd3e20>
1400fb98a: 4c 89 6c 24 20              	mov	qword ptr [rsp + 0x20], r13
1400fb98f: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
1400fb996: 4c 89 f2                    	mov	rdx, r14
1400fb999: 4d 89 f8                    	mov	r8, r15
1400fb99c: 4d 89 e1                    	mov	r9, r12
1400fb99f: e8 2c a9 3f 00              	call	0x1404f62d0 <.text+0x4f52d0>
1400fb9a4: 80 bd 68 01 00 00 02        	cmp	byte ptr [rbp + 0x168], 0x2
1400fb9ab: 0f 84 7a 04 00 00           	je	0x1400fbe2b <.text+0xfae2b>
1400fb9b1: 0f 10 85 50 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x150]
1400fb9b8: 0f 10 8d 60 01 00 00        	movups	xmm1, xmmword ptr [rbp + 0x160]
1400fb9bf: 0f 29 4d d0                 	movaps	xmmword ptr [rbp - 0x30], xmm1
1400fb9c3: 0f 29 45 c0                 	movaps	xmmword ptr [rbp - 0x40], xmm0
1400fb9c7: 0f 29 85 f0 02 00 00        	movaps	xmmword ptr [rbp + 0x2f0], xmm0
1400fb9ce: 48 8b 45 d0                 	mov	rax, qword ptr [rbp - 0x30]
1400fb9d2: 48 89 85 00 03 00 00        	mov	qword ptr [rbp + 0x300], rax
1400fb9d9: 48 8b 95 60 03 00 00        	mov	rdx, qword ptr [rbp + 0x360]
1400fb9e0: 48 85 d2                    	test	rdx, rdx
1400fb9e3: 74 12                       	je	0x1400fb9f7 <.text+0xfa9f7>
1400fb9e5: 41 b8 01 00 00 00           	mov	r8d, 0x1
1400fb9eb: 48 8b 8d 58 03 00 00        	mov	rcx, qword ptr [rbp + 0x358]
1400fb9f2: e8 29 82 51 00              	call	0x140613c20 <.text+0x612c20>
1400fb9f7: 48 89 f9                    	mov	rcx, rdi
1400fb9fa: e8 91 9d ad 00              	call	0x140bd5790 <.text+0xbd4790>
1400fb9ff: 48 89 c7                    	mov	rdi, rax
1400fba02: 48 c1 e8 20                 	shr	rax, 0x20
1400fba06: 41 89 ff                    	mov	r15d, edi
1400fba09: 41 c1 ef 06                 	shr	r15d, 0x6
1400fba0d: 89 f9                       	mov	ecx, edi
1400fba0f: 83 e1 3f                    	and	ecx, 0x3f
1400fba12: 83 f9 01                    	cmp	ecx, 0x1
1400fba15: 41 83 df ff                 	sbb	r15d, -0x1
1400fba19: 48 c1 ef 26                 	shr	rdi, 0x26
1400fba1d: 83 e0 3f                    	and	eax, 0x3f
1400fba20: 83 f8 01                    	cmp	eax, 0x1
1400fba23: 83 df ff                    	sbb	edi, -0x1
1400fba26: 44 89 bd 4c 03 00 00        	mov	dword ptr [rbp + 0x34c], r15d
1400fba2d: 89 bd 50 03 00 00           	mov	dword ptr [rbp + 0x350], edi
1400fba33: c7 85 54 03 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x354], 0x1
1400fba3d: 4c 89 74 24 30              	mov	qword ptr [rsp + 0x30], r14
1400fba42: 48 89 5c 24 28              	mov	qword ptr [rsp + 0x28], rbx
1400fba47: 48 c7 44 24 20 16 00 00 00  	mov	qword ptr [rsp + 0x20], 0x16
1400fba50: 48 8d 15 11 11 85 02        	lea	rdx, [rip + 0x2851111]  # 0x14294cb68
1400fba57: 4c 8d 0d 17 11 85 02        	lea	r9, [rip + 0x2851117]   # 0x14294cb75
1400fba5e: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
1400fba65: 41 b8 0d 00 00 00           	mov	r8d, 0xd
1400fba6b: e8 40 c4 48 00              	call	0x140587eb0 <.text+0x586eb0>
1400fba70: c6 85 ad 03 00 00 01        	mov	byte ptr [rbp + 0x3ad], 0x1
1400fba77: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
1400fba7b: e8 20 9d ad 00              	call	0x140bd57a0 <.text+0xbd47a0>
1400fba80: c6 85 ad 03 00 00 01        	mov	byte ptr [rbp + 0x3ad], 0x1
1400fba87: 48 8d 8d 60 03 00 00        	lea	rcx, [rbp + 0x360]
1400fba8e: 48 8d 55 c0                 	lea	rdx, [rbp - 0x40]
1400fba92: 45 31 c0                    	xor	r8d, r8d
1400fba95: e8 46 9e ad 00              	call	0x140bd58e0 <.text+0xbd48e0>
1400fba9a: c6 85 ad 03 00 00 01        	mov	byte ptr [rbp + 0x3ad], 0x1
1400fbaa1: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
1400fbaa5: 48 8d 9d 60 03 00 00        	lea	rbx, [rbp + 0x360]
1400fbaac: 48 89 da                    	mov	rdx, rbx
1400fbaaf: 41 b8 01 00 00 00           	mov	r8d, 0x1
1400fbab5: 41 b9 01 00 00 00           	mov	r9d, 0x1
1400fbabb: e8 60 9e ad 00              	call	0x140bd5920 <.text+0xbd4920>
1400fbac0: 0f 10 45 c0                 	movups	xmm0, xmmword ptr [rbp - 0x40]
1400fbac4: 0f 10 4d d0                 	movups	xmm1, xmmword ptr [rbp - 0x30]
1400fbac8: 0f 10 55 e0                 	movups	xmm2, xmmword ptr [rbp - 0x20]
1400fbacc: 0f 10 5d ec                 	movups	xmm3, xmmword ptr [rbp - 0x14]
1400fbad0: 0f 29 85 60 03 00 00        	movaps	xmmword ptr [rbp + 0x360], xmm0
1400fbad7: 0f 29 8d 70 03 00 00        	movaps	xmmword ptr [rbp + 0x370], xmm1
1400fbade: 0f 29 95 80 03 00 00        	movaps	xmmword ptr [rbp + 0x380], xmm2
1400fbae5: 0f 11 9d 8c 03 00 00        	movups	xmmword ptr [rbp + 0x38c], xmm3
1400fbaec: 48 89 b5 10 03 00 00        	mov	qword ptr [rbp + 0x310], rsi
1400fbaf3: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
1400fbaf8: 48 8d 9d 10 03 00 00        	lea	rbx, [rbp + 0x310]
1400fbaff: 48 8d 95 50 01 00 00        	lea	rdx, [rbp + 0x150]
1400fbb06: 49 b8 92 02 fc c6 0a b3 66 b4       	movabs	r8, -0x4b994cf53903fd6e
1400fbb10: 48 89 d9                    	mov	rcx, rbx
1400fbb13: 41 b1 37                    	mov	r9b, 0x37
1400fbb16: e8 35 3c 1b 00              	call	0x1402af750 <.text+0x2ae750>
1400fbb1b: 44 8b b5 a8 03 00 00        	mov	r14d, dword ptr [rbp + 0x3a8]
1400fbb22: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
1400fbb26: 48 8d 95 50 01 00 00        	lea	rdx, [rbp + 0x150]
1400fbb2d: 41 b8 90 01 00 00           	mov	r8d, 0x190
1400fbb33: e8 c8 80 82 02              	call	0x142923c00 <.text+0x2922c00>
1400fbb38: c6 85 af 03 00 00 01        	mov	byte ptr [rbp + 0x3af], 0x1
1400fbb3f: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
1400fbb46: e8 55 9c ad 00              	call	0x140bd57a0 <.text+0xbd47a0>
1400fbb4b: c6 85 af 03 00 00 01        	mov	byte ptr [rbp + 0x3af], 0x1
1400fbb52: 48 8d 8d 60 03 00 00        	lea	rcx, [rbp + 0x360]
1400fbb59: 48 8d 95 50 01 00 00        	lea	rdx, [rbp + 0x150]
1400fbb60: 41 b0 01                    	mov	r8b, 0x1
1400fbb63: e8 88 9e ad 00              	call	0x140bd59f0 <.text+0xbd49f0>
1400fbb68: c6 85 af 03 00 00 01        	mov	byte ptr [rbp + 0x3af], 0x1
1400fbb6f: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
1400fbb76: 48 8d 95 60 03 00 00        	lea	rdx, [rbp + 0x360]
1400fbb7d: 41 b8 01 00 00 00           	mov	r8d, 0x1
1400fbb83: e8 58 9d ad 00              	call	0x140bd58e0 <.text+0xbd48e0>
1400fbb88: c6 85 af 03 00 00 01        	mov	byte ptr [rbp + 0x3af], 0x1
1400fbb8f: 48 8d 8d 60 03 00 00        	lea	rcx, [rbp + 0x360]
1400fbb96: 48 8d 95 50 01 00 00        	lea	rdx, [rbp + 0x150]
1400fbb9d: 41 b8 05 00 00 00           	mov	r8d, 0x5
1400fbba3: e8 38 9c ad 00              	call	0x140bd57e0 <.text+0xbd47e0>
1400fbba8: 0f 10 85 60 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x360]
1400fbbaf: 0f 10 8d 70 03 00 00        	movups	xmm1, xmmword ptr [rbp + 0x370]
1400fbbb6: 0f 10 95 80 03 00 00        	movups	xmm2, xmmword ptr [rbp + 0x380]
1400fbbbd: 0f 10 9d 8c 03 00 00        	movups	xmm3, xmmword ptr [rbp + 0x38c]
1400fbbc4: 0f 29 85 10 03 00 00        	movaps	xmmword ptr [rbp + 0x310], xmm0
1400fbbcb: 0f 29 8d 20 03 00 00        	movaps	xmmword ptr [rbp + 0x320], xmm1
1400fbbd2: 0f 29 95 30 03 00 00        	movaps	xmmword ptr [rbp + 0x330], xmm2
1400fbbd9: 0f 11 9d 3c 03 00 00        	movups	xmmword ptr [rbp + 0x33c], xmm3
1400fbbe0: 48 89 b5 60 03 00 00        	mov	qword ptr [rbp + 0x360], rsi
1400fbbe7: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
1400fbbec: 48 8d 8d 60 03 00 00        	lea	rcx, [rbp + 0x360]
1400fbbf3: 48 8d 55 c0                 	lea	rdx, [rbp - 0x40]
1400fbbf7: 49 b8 87 f6 83 56 0a 1f 9d ed       	movabs	r8, -0x1262e0f5a97c0979
1400fbc01: 41 b1 37                    	mov	r9b, 0x37
1400fbc04: e8 47 3b 1b 00              	call	0x1402af750 <.text+0x2ae750>
1400fbc09: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
1400fbc10: 48 8d 55 c0                 	lea	rdx, [rbp - 0x40]
1400fbc14: 41 b8 90 01 00 00           	mov	r8d, 0x190
1400fbc1a: e8 e1 7f 82 02              	call	0x142923c00 <.text+0x2922c00>
1400fbc1f: c6 85 ae 03 00 00 01        	mov	byte ptr [rbp + 0x3ae], 0x1
1400fbc26: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
1400fbc2a: e8 71 9b ad 00              	call	0x140bd57a0 <.text+0xbd47a0>
1400fbc2f: c6 85 ae 03 00 00 01        	mov	byte ptr [rbp + 0x3ae], 0x1
1400fbc36: 48 8d 8d 60 03 00 00        	lea	rcx, [rbp + 0x360]
1400fbc3d: 48 8d 55 c0                 	lea	rdx, [rbp - 0x40]
1400fbc41: 41 b8 05 00 00 00           	mov	r8d, 0x5
1400fbc47: e8 94 9b ad 00              	call	0x140bd57e0 <.text+0xbd47e0>
1400fbc4c: 41 ff ce                    	dec	r14d
1400fbc4f: 41 83 fe 06                 	cmp	r14d, 0x6
1400fbc53: 41 b8 06 00 00 00           	mov	r8d, 0x6
1400fbc59: 45 0f 42 c6                 	cmovb	r8d, r14d
1400fbc5d: c6 85 ae 03 00 00 01        	mov	byte ptr [rbp + 0x3ae], 0x1
1400fbc64: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
1400fbc68: 48 8d 95 60 03 00 00        	lea	rdx, [rbp + 0x360]
1400fbc6f: e8 6c 9c ad 00              	call	0x140bd58e0 <.text+0xbd48e0>
1400fbc74: c6 85 ae 03 00 00 01        	mov	byte ptr [rbp + 0x3ae], 0x1
1400fbc7b: 48 8d 8d 60 03 00 00        	lea	rcx, [rbp + 0x360]
1400fbc82: 48 8d 55 c0                 	lea	rdx, [rbp - 0x40]
1400fbc86: 41 b8 01 00 00 00           	mov	r8d, 0x1
1400fbc8c: 41 b9 01 00 00 00           	mov	r9d, 0x1
1400fbc92: e8 89 9c ad 00              	call	0x140bd5920 <.text+0xbd4920>
1400fbc97: 0f 10 85 60 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x360]
1400fbc9e: 0f 10 8d 70 03 00 00        	movups	xmm1, xmmword ptr [rbp + 0x370]
1400fbca5: 0f 10 95 80 03 00 00        	movups	xmm2, xmmword ptr [rbp + 0x380]
1400fbcac: 0f 10 9d 8c 03 00 00        	movups	xmm3, xmmword ptr [rbp + 0x38c]
1400fbcb3: 0f 29 85 10 03 00 00        	movaps	xmmword ptr [rbp + 0x310], xmm0
1400fbcba: 0f 29 8d 20 03 00 00        	movaps	xmmword ptr [rbp + 0x320], xmm1
1400fbcc1: 0f 29 95 30 03 00 00        	movaps	xmmword ptr [rbp + 0x330], xmm2
1400fbcc8: 0f 11 9d 3c 03 00 00        	movups	xmmword ptr [rbp + 0x33c], xmm3
1400fbccf: 48 89 b5 60 03 00 00        	mov	qword ptr [rbp + 0x360], rsi
1400fbcd6: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
1400fbcdb: 48 8d 8d 60 03 00 00        	lea	rcx, [rbp + 0x360]
1400fbce2: 48 8d 95 50 01 00 00        	lea	rdx, [rbp + 0x150]
1400fbce9: 49 b8 c3 2e 3c 02 91 6f f2 45       	movabs	r8, 0x45f26f91023c2ec3
1400fbcf3: 41 b1 37                    	mov	r9b, 0x37
1400fbcf6: e8 55 3a 1b 00              	call	0x1402af750 <.text+0x2ae750>
1400fbcfb: 48 8d 75 c0                 	lea	rsi, [rbp - 0x40]
1400fbcff: 48 8d 95 50 01 00 00        	lea	rdx, [rbp + 0x150]
1400fbd06: 41 b8 90 01 00 00           	mov	r8d, 0x190
1400fbd0c: 48 89 f1                    	mov	rcx, rsi
1400fbd0f: e8 ec 7e 82 02              	call	0x142923c00 <.text+0x2922c00>
1400fbd14: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
1400fbd1b: 49 b8 24 4f de b4 13 44 8d 06       	movabs	r8, 0x68d4413b4de4f24
1400fbd25: 4c 8d 8d f0 02 00 00        	lea	r9, [rbp + 0x2f0]
1400fbd2c: 48 89 f2                    	mov	rdx, rsi
1400fbd2f: e8 2c 20 87 00              	call	0x14096dd60 <.text+0x96cd60>
1400fbd34: 0f b6 05 ce 8e 53 03        	movzx	eax, byte ptr [rip + 0x3538ece] # 0x143634c09
1400fbd3b: b9 10 00 00 00              	mov	ecx, 0x10
1400fbd40: ba 01 00 00 00              	mov	edx, 0x1
1400fbd45: e8 c6 7e 51 00              	call	0x140613c10 <.text+0x612c10>
1400fbd4a: 48 85 c0                    	test	rax, rax
1400fbd4d: 0f 84 0d 01 00 00           	je	0x1400fbe60 <.text+0xfae60>
1400fbd53: 48 89 c6                    	mov	rsi, rax
1400fbd56: 41 0f af ff                 	imul	edi, r15d
1400fbd5a: 44 89 30                    	mov	dword ptr [rax], r14d
1400fbd5d: 89 78 04                    	mov	dword ptr [rax + 0x4], edi
1400fbd60: 48 c7 40 08 00 00 00 00     	mov	qword ptr [rax + 0x8], 0x0
1400fbd68: 48 8b 95 98 01 00 00        	mov	rdx, qword ptr [rbp + 0x198]
1400fbd6f: 48 85 d2                    	test	rdx, rdx
1400fbd72: 74 12                       	je	0x1400fbd86 <.text+0xfad86>
1400fbd74: 48 8b 8d a0 01 00 00        	mov	rcx, qword ptr [rbp + 0x1a0]
1400fbd7b: 41 b8 01 00 00 00           	mov	r8d, 0x1
1400fbd81: e8 9a 7e 51 00              	call	0x140613c20 <.text+0x612c20>
1400fbd86: 48 c7 85 98 01 00 00 10 00 00 00    	mov	qword ptr [rbp + 0x198], 0x10
1400fbd91: 48 89 b5 a0 01 00 00        	mov	qword ptr [rbp + 0x1a0], rsi
1400fbd98: 48 c7 85 a8 01 00 00 10 00 00 00    	mov	qword ptr [rbp + 0x1a8], 0x10
1400fbda3: 48 8d 75 c0                 	lea	rsi, [rbp - 0x40]
1400fbda7: 48 8d 95 50 01 00 00        	lea	rdx, [rbp + 0x150]
1400fbdae: 41 b8 90 01 00 00           	mov	r8d, 0x190
1400fbdb4: 48 89 f1                    	mov	rcx, rsi
1400fbdb7: e8 44 7e 82 02              	call	0x142923c00 <.text+0x2922c00>
1400fbdbc: 48 8d 95 4c 03 00 00        	lea	rdx, [rbp + 0x34c]
1400fbdc3: 48 89 f1                    	mov	rcx, rsi
1400fbdc6: e8 45 30 87 00              	call	0x14096ee10 <.text+0x96de10>
1400fbdcb: 90                          	nop
1400fbdcc: 48 81 c4 38 04 00 00        	add	rsp, 0x438
1400fbdd3: 5b                          	pop	rbx
1400fbdd4: 5f                          	pop	rdi
1400fbdd5: 5e                          	pop	rsi
1400fbdd6: 41 5c                       	pop	r12
1400fbdd8: 41 5d                       	pop	r13
1400fbdda: 41 5e                       	pop	r14
1400fbddc: 41 5f                       	pop	r15
1400fbdde: 5d                          	pop	rbp
1400fbddf: c3                          	ret
1400fbde0: 48 8d 0d 2d 0c 85 02        	lea	rcx, [rip + 0x2850c2d]  # 0x14294ca14
1400fbde7: 4c 8d 05 d2 0c 85 02        	lea	r8, [rip + 0x2850cd2]   # 0x14294cac0
1400fbdee: ba 2f 00 00 00              	mov	edx, 0x2f
1400fbdf3: e8 38 d6 7d 02              	call	0x1428d9430 <.text+0x28d8430>
1400fbdf8: 48 c7 85 50 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x150], 0x0
1400fbe03: 48 83 c7 08                 	add	rdi, 0x8
1400fbe07: 48 8d 05 d2 0c 85 02        	lea	rax, [rip + 0x2850cd2]  # 0x14294cae0
1400fbe0e: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1400fbe13: 4c 8d 05 be 0c 85 02        	lea	r8, [rip + 0x2850cbe]   # 0x14294cad8
1400fbe1a: 4c 8d 8d 50 01 00 00        	lea	r9, [rbp + 0x150]
1400fbe21: 31 c9                       	xor	ecx, ecx
1400fbe23: 48 89 fa                    	mov	rdx, rdi
1400fbe26: e8 e9 da 7a 02              	call	0x1428a9914 <.text+0x28a8914>
1400fbe2b: 48 8b 85 50 01 00 00        	mov	rax, qword ptr [rbp + 0x150]
1400fbe32: 48 89 45 c0                 	mov	qword ptr [rbp - 0x40], rax
1400fbe36: 48 8d 05 13 0d 85 02        	lea	rax, [rip + 0x2850d13]  # 0x14294cb50
1400fbe3d: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1400fbe42: 48 8d 0d cf 0c 85 02        	lea	rcx, [rip + 0x2850ccf]  # 0x14294cb18
1400fbe49: 4c 8d 0d 40 0d 85 02        	lea	r9, [rip + 0x2850d40]   # 0x14294cb90
1400fbe50: 4c 8d 45 c0                 	lea	r8, [rbp - 0x40]
1400fbe54: ba 35 00 00 00              	mov	edx, 0x35
1400fbe59: e8 02 d9 7d 02              	call	0x1428d9760 <.text+0x28d8760>
1400fbe5e: eb 16                       	jmp	0x1400fbe76 <.text+0xfae76>
1400fbe60: 4c 8d 05 21 12 90 02        	lea	r8, [rip + 0x2901221]   # 0x1429fd088
1400fbe67: b9 01 00 00 00              	mov	ecx, 0x1
1400fbe6c: ba 10 00 00 00              	mov	edx, 0x10
1400fbe71: e8 6d d1 7d 02              	call	0x1428d8fe3 <.text+0x28d7fe3>
1400fbe76: 0f 0b                       	ud2
1400fbe78: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
1400fbe80: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1400fbe85: 55                          	push	rbp
1400fbe86: 41 57                       	push	r15
1400fbe88: 41 56                       	push	r14
1400fbe8a: 41 55                       	push	r13
1400fbe8c: 41 54                       	push	r12
1400fbe8e: 56                          	push	rsi
1400fbe8f: 57                          	push	rdi
1400fbe90: 53                          	push	rbx
1400fbe91: 48 83 ec 38                 	sub	rsp, 0x38
1400fbe95: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1400fbe9c: 48 8b 95 60 03 00 00        	mov	rdx, qword ptr [rbp + 0x360]
1400fbea3: 48 85 d2                    	test	rdx, rdx
1400fbea6: 74 12                       	je	0x1400fbeba <.text+0xfaeba>
1400fbea8: 41 b8 01 00 00 00           	mov	r8d, 0x1
1400fbeae: 48 8b 8d 58 03 00 00        	mov	rcx, qword ptr [rbp + 0x358]
1400fbeb5: e8 66 7d 51 00              	call	0x140613c20 <.text+0x612c20>
1400fbeba: 90                          	nop
1400fbebb: 48 83 c4 38                 	add	rsp, 0x38
1400fbebf: 5b                          	pop	rbx
1400fbec0: 5f                          	pop	rdi
1400fbec1: 5e                          	pop	rsi
1400fbec2: 41 5c                       	pop	r12
1400fbec4: 41 5d                       	pop	r13
1400fbec6: 41 5e                       	pop	r14
1400fbec8: 41 5f                       	pop	r15
1400fbeca: 5d                          	pop	rbp
1400fbecb: c3                          	ret
1400fbecc: 0f 1f 40 00                 	nop	dword ptr [rax]
1400fbed0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1400fbed5: 55                          	push	rbp
1400fbed6: 41 57                       	push	r15
1400fbed8: 41 56                       	push	r14
1400fbeda: 41 55                       	push	r13
1400fbedc: 41 54                       	push	r12
1400fbede: 56                          	push	rsi
1400fbedf: 57                          	push	rdi
1400fbee0: 53                          	push	rbx
1400fbee1: 48 83 ec 38                 	sub	rsp, 0x38
1400fbee5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1400fbeec: 80 bd ae 03 00 00 00        	cmp	byte ptr [rbp + 0x3ae], 0x0
1400fbef3: 75 11                       	jne	0x1400fbf06 <.text+0xfaf06>
1400fbef5: 48 83 c4 38                 	add	rsp, 0x38
1400fbef9: 5b                          	pop	rbx
1400fbefa: 5f                          	pop	rdi
1400fbefb: 5e                          	pop	rsi
1400fbefc: 41 5c                       	pop	r12
1400fbefe: 41 5d                       	pop	r13
1400fbf00: 41 5e                       	pop	r14
1400fbf02: 41 5f                       	pop	r15
1400fbf04: 5d                          	pop	rbp
1400fbf05: c3                          	ret
1400fbf06: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
1400fbf0d: e8 0e d6 50 00              	call	0x140609520 <.text+0x608520>
1400fbf12: eb e1                       	jmp	0x1400fbef5 <.text+0xfaef5>
1400fbf14: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
1400fbf20: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1400fbf25: 55                          	push	rbp
1400fbf26: 41 57                       	push	r15
1400fbf28: 41 56                       	push	r14
1400fbf2a: 41 55                       	push	r13
1400fbf2c: 41 54                       	push	r12
1400fbf2e: 56                          	push	rsi
1400fbf2f: 57                          	push	rdi
1400fbf30: 53                          	push	rbx
1400fbf31: 48 83 ec 38                 	sub	rsp, 0x38
1400fbf35: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1400fbf3c: 80 bd af 03 00 00 00        	cmp	byte ptr [rbp + 0x3af], 0x0
1400fbf43: 75 11                       	jne	0x1400fbf56 <.text+0xfaf56>
1400fbf45: 48 83 c4 38                 	add	rsp, 0x38
1400fbf49: 5b                          	pop	rbx
1400fbf4a: 5f                          	pop	rdi
1400fbf4b: 5e                          	pop	rsi
1400fbf4c: 41 5c                       	pop	r12
1400fbf4e: 41 5d                       	pop	r13
1400fbf50: 41 5e                       	pop	r14
1400fbf52: 41 5f                       	pop	r15
1400fbf54: 5d                          	pop	rbp
1400fbf55: c3                          	ret
1400fbf56: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
1400fbf5a: e8 c1 d5 50 00              	call	0x140609520 <.text+0x608520>
1400fbf5f: eb e4                       	jmp	0x1400fbf45 <.text+0xfaf45>
1400fbf61: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
1400fbf70: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1400fbf75: 55                          	push	rbp
1400fbf76: 41 57                       	push	r15
1400fbf78: 41 56                       	push	r14
1400fbf7a: 41 55                       	push	r13
1400fbf7c: 41 54                       	push	r12
1400fbf7e: 56                          	push	rsi
1400fbf7f: 57                          	push	rdi
1400fbf80: 53                          	push	rbx
1400fbf81: 48 83 ec 38                 	sub	rsp, 0x38
1400fbf85: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1400fbf8c: 80 bd ad 03 00 00 00        	cmp	byte ptr [rbp + 0x3ad], 0x0
1400fbf93: 75 11                       	jne	0x1400fbfa6 <.text+0xfafa6>
1400fbf95: 48 83 c4 38                 	add	rsp, 0x38
1400fbf99: 5b                          	pop	rbx
1400fbf9a: 5f                          	pop	rdi
1400fbf9b: 5e                          	pop	rsi
1400fbf9c: 41 5c                       	pop	r12
1400fbf9e: 41 5d                       	pop	r13
1400fbfa0: 41 5e                       	pop	r14
1400fbfa2: 41 5f                       	pop	r15
1400fbfa4: 5d                          	pop	rbp
1400fbfa5: c3                          	ret
1400fbfa6: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
1400fbfad: e8 6e d5 50 00              	call	0x140609520 <.text+0x608520>
1400fbfb2: eb e1                       	jmp	0x1400fbf95 <.text+0xfaf95>
1400fbfb4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
1400fbfc0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1400fbfc5: 55                          	push	rbp
1400fbfc6: 41 57                       	push	r15
1400fbfc8: 41 56                       	push	r14
1400fbfca: 41 55                       	push	r13
1400fbfcc: 41 54                       	push	r12
1400fbfce: 56                          	push	rsi
1400fbfcf: 57                          	push	rdi
1400fbfd0: 53                          	push	rbx
1400fbfd1: 48 83 ec 38                 	sub	rsp, 0x38
1400fbfd5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1400fbfdc: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
1400fbfe0: e8 4b 0c c5 00              	call	0x140d4cc30 <.text+0xd4bc30>
1400fbfe5: 90                          	nop
1400fbfe6: 48 83 c4 38                 	add	rsp, 0x38
1400fbfea: 5b                          	pop	rbx
1400fbfeb: 5f                          	pop	rdi
1400fbfec: 5e                          	pop	rsi
1400fbfed: 41 5c                       	pop	r12
1400fbfef: 41 5d                       	pop	r13
1400fbff1: 41 5e                       	pop	r14
1400fbff3: 41 5f                       	pop	r15
1400fbff5: 5d                          	pop	rbp
1400fbff6: c3                          	ret
1400fbff7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1400fc000: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1400fc005: 55                          	push	rbp
1400fc006: 41 57                       	push	r15
1400fc008: 41 56                       	push	r14
1400fc00a: 41 55                       	push	r13
1400fc00c: 41 54                       	push	r12
1400fc00e: 56                          	push	rsi
1400fc00f: 57                          	push	rdi
1400fc010: 53                          	push	rbx
1400fc011: 48 83 ec 38                 	sub	rsp, 0x38
1400fc015: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1400fc01c: c6 85 ad 03 00 00 00        	mov	byte ptr [rbp + 0x3ad], 0x0
1400fc023: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
1400fc02a: e8 f1 d4 50 00              	call	0x140609520 <.text+0x608520>
1400fc02f: c6 85 ad 03 00 00 00        	mov	byte ptr [rbp + 0x3ad], 0x0
1400fc036: 48 83 c4 38                 	add	rsp, 0x38
1400fc03a: 5b                          	pop	rbx
1400fc03b: 5f                          	pop	rdi
1400fc03c: 5e                          	pop	rsi
1400fc03d: 41 5c                       	pop	r12
1400fc03f: 41 5d                       	pop	r13
1400fc041: 41 5e                       	pop	r14
1400fc043: 41 5f                       	pop	r15
1400fc045: 5d                          	pop	rbp
1400fc046: c3                          	ret
1400fc047: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1400fc050: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1400fc055: 55                          	push	rbp
1400fc056: 41 57                       	push	r15
1400fc058: 41 56                       	push	r14
1400fc05a: 41 55                       	push	r13
1400fc05c: 41 54                       	push	r12
1400fc05e: 56                          	push	rsi
1400fc05f: 57                          	push	rdi
1400fc060: 53                          	push	rbx
1400fc061: 48 83 ec 38                 	sub	rsp, 0x38
1400fc065: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1400fc06c: c6 85 af 03 00 00 00        	mov	byte ptr [rbp + 0x3af], 0x0
1400fc073: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
1400fc077: e8 a4 d4 50 00              	call	0x140609520 <.text+0x608520>
1400fc07c: c6 85 af 03 00 00 00        	mov	byte ptr [rbp + 0x3af], 0x0
1400fc083: 48 83 c4 38                 	add	rsp, 0x38
1400fc087: 5b                          	pop	rbx
1400fc088: 5f                          	pop	rdi
1400fc089: 5e                          	pop	rsi
1400fc08a: 41 5c                       	pop	r12
1400fc08c: 41 5d                       	pop	r13
1400fc08e: 41 5e                       	pop	r14
1400fc090: 41 5f                       	pop	r15
1400fc092: 5d                          	pop	rbp
1400fc093: c3                          	ret
1400fc094: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
1400fc0a0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1400fc0a5: 55                          	push	rbp
1400fc0a6: 41 57                       	push	r15
1400fc0a8: 41 56                       	push	r14
1400fc0aa: 41 55                       	push	r13
1400fc0ac: 41 54                       	push	r12
1400fc0ae: 56                          	push	rsi
1400fc0af: 57                          	push	rdi
1400fc0b0: 53                          	push	rbx
1400fc0b1: 48 83 ec 38                 	sub	rsp, 0x38
1400fc0b5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1400fc0bc: c6 85 ae 03 00 00 00        	mov	byte ptr [rbp + 0x3ae], 0x0
1400fc0c3: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
1400fc0ca: e8 51 d4 50 00              	call	0x140609520 <.text+0x608520>
1400fc0cf: c6 85 ae 03 00 00 00        	mov	byte ptr [rbp + 0x3ae], 0x0
1400fc0d6: 48 83 c4 38                 	add	rsp, 0x38
1400fc0da: 5b                          	pop	rbx
1400fc0db: 5f                          	pop	rdi
1400fc0dc: 5e                          	pop	rsi
1400fc0dd: 41 5c                       	pop	r12
1400fc0df: 41 5d                       	pop	r13
1400fc0e1: 41 5e                       	pop	r14
1400fc0e3: 41 5f                       	pop	r15
1400fc0e5: 5d                          	pop	rbp
1400fc0e6: c3                          	ret
1400fc0e7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1400fc0f0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1400fc0f5: 55                          	push	rbp
1400fc0f6: 41 57                       	push	r15
1400fc0f8: 41 56                       	push	r14
1400fc0fa: 41 55                       	push	r13
1400fc0fc: 41 54                       	push	r12
1400fc0fe: 56                          	push	rsi
1400fc0ff: 57                          	push	rdi
1400fc100: 53                          	push	rbx
1400fc101: 48 83 ec 38                 	sub	rsp, 0x38
1400fc105: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1400fc10c: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
1400fc113: e8 08 d4 50 00              	call	0x140609520 <.text+0x608520>
1400fc118: 90                          	nop
1400fc119: 48 83 c4 38                 	add	rsp, 0x38
1400fc11d: 5b                          	pop	rbx
1400fc11e: 5f                          	pop	rdi
1400fc11f: 5e                          	pop	rsi
1400fc120: 41 5c                       	pop	r12
1400fc122: 41 5d                       	pop	r13
1400fc124: 41 5e                       	pop	r14
1400fc126: 41 5f                       	pop	r15
1400fc128: 5d                          	pop	rbp
1400fc129: c3                          	ret
