
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140987dd0: 55                          	push	rbp
140987dd1: 41 57                       	push	r15
140987dd3: 41 56                       	push	r14
140987dd5: 41 55                       	push	r13
140987dd7: 41 54                       	push	r12
140987dd9: 56                          	push	rsi
140987dda: 57                          	push	rdi
140987ddb: 53                          	push	rbx
140987ddc: 48 81 ec 68 06 00 00        	sub	rsp, 0x668
140987de3: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
140987deb: 0f 29 b5 d0 05 00 00        	movaps	xmmword ptr [rbp + 0x5d0], xmm6
140987df2: 48 c7 85 c8 05 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x5c8], -0x2
140987dfd: 48 89 d3                    	mov	rbx, rdx
140987e00: 49 89 cf                    	mov	r15, rcx
140987e03: 4c 89 85 80 05 00 00        	mov	qword ptr [rbp + 0x580], r8
140987e0a: 4d 85 c0                    	test	r8, r8
140987e0d: 40 0f 95 c6                 	setne	sil
140987e11: 4c 8d 62 38                 	lea	r12, [rdx + 0x38]
140987e15: 48 ba ef a5 99 86 45 78 29 66       	movabs	rdx, 0x662978458699a5ef
140987e1f: 49 b8 46 9b 41 62 c1 57 c0 3e       	movabs	r8, 0x3ec057c162419b46
140987e29: 4c 89 e1                    	mov	rcx, r12
140987e2c: e8 bf 58 75 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
140987e31: 48 85 c0                    	test	rax, rax
140987e34: 0f 84 15 18 00 00           	je	0x14098964f <.text+0x98864f>
140987e3a: 48 39 93 18 02 00 00        	cmp	qword ptr [rbx + 0x218], rdx
140987e41: 0f 86 08 18 00 00           	jbe	0x14098964f <.text+0x98864f>
140987e47: 48 8b 83 10 02 00 00        	mov	rax, qword ptr [rbx + 0x210]
140987e4e: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
140987e52: 48 85 c9                    	test	rcx, rcx
140987e55: 0f 84 f4 17 00 00           	je	0x14098964f <.text+0x98864f>
140987e5b: 48 8b 83 e0 01 00 00        	mov	rax, qword ptr [rbx + 0x1e0]
140987e62: 48 f7 d1                    	not	rcx
140987e65: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
140987e69: 48 c1 e1 05                 	shl	rcx, 0x5
140987e6d: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
140987e73: 0f 84 d6 17 00 00           	je	0x14098964f <.text+0x98864f>
140987e79: 48 01 c8                    	add	rax, rcx
140987e7c: 48 8b 78 10                 	mov	rdi, qword ptr [rax + 0x10]
140987e80: 8b 8b b0 05 00 00           	mov	ecx, dword ptr [rbx + 0x5b0]
140987e86: 89 48 5c                    	mov	dword ptr [rax + 0x5c], ecx
140987e89: 48 8d 4d 30                 	lea	rcx, [rbp + 0x30]
140987e8d: ba 00 c0 06 00              	mov	edx, 0x6c000
140987e92: 45 31 c0                    	xor	r8d, r8d
140987e95: e8 86 cf 24 00              	call	0x140bd4e20 <.text+0xbd3e20>
140987e9a: 0f 10 45 30                 	movups	xmm0, xmmword ptr [rbp + 0x30]
140987e9e: 0f 29 85 20 02 00 00        	movaps	xmmword ptr [rbp + 0x220], xmm0
140987ea5: 48 8d 8d c0 03 00 00        	lea	rcx, [rbp + 0x3c0]
140987eac: 48 8d 95 20 02 00 00        	lea	rdx, [rbp + 0x220]
140987eb3: e8 48 ee 24 00              	call	0x140bd6d00 <.text+0xbd5d00>
140987eb8: 48 8d 05 19 0d 13 02        	lea	rax, [rip + 0x2130d19]  # 0x142ab8bd8
140987ebf: 48 89 85 b0 03 00 00        	mov	qword ptr [rbp + 0x3b0], rax
140987ec6: 48 c7 85 b8 03 00 00 1b 00 00 00    	mov	qword ptr [rbp + 0x3b8], 0x1b
140987ed1: 48 8d 95 b0 03 00 00        	lea	rdx, [rbp + 0x3b0]
140987ed8: 4c 89 f9                    	mov	rcx, r15
140987edb: e8 f0 15 26 00              	call	0x140be94d0 <.text+0xbe84d0>
140987ee0: 0f 10 45 30                 	movups	xmm0, xmmword ptr [rbp + 0x30]
140987ee4: 0f 29 85 f0 01 00 00        	movaps	xmmword ptr [rbp + 0x1f0], xmm0
140987eeb: 89 85 00 02 00 00           	mov	dword ptr [rbp + 0x200], eax
140987ef1: 48 89 bd a0 05 00 00        	mov	qword ptr [rbp + 0x5a0], rdi
140987ef8: 48 8b 87 50 14 00 00        	mov	rax, qword ptr [rdi + 0x1450]
140987eff: 8b 50 60                    	mov	edx, dword ptr [rax + 0x60]
140987f02: 48 c1 e2 02                 	shl	rdx, 0x2
140987f06: 48 8d 4d 30                 	lea	rcx, [rbp + 0x30]
140987f0a: 45 31 c0                    	xor	r8d, r8d
140987f0d: e8 0e cf 24 00              	call	0x140bd4e20 <.text+0xbd3e20>
140987f12: 0f 10 45 30                 	movups	xmm0, xmmword ptr [rbp + 0x30]
140987f16: 0f 29 85 20 02 00 00        	movaps	xmmword ptr [rbp + 0x220], xmm0
140987f1d: 48 8d 8d c0 03 00 00        	lea	rcx, [rbp + 0x3c0]
140987f24: 48 8d 95 20 02 00 00        	lea	rdx, [rbp + 0x220]
140987f2b: e8 d0 ed 24 00              	call	0x140bd6d00 <.text+0xbd5d00>
140987f30: 48 8d 05 bc 0c 13 02        	lea	rax, [rip + 0x2130cbc]  # 0x142ab8bf3
140987f37: 48 89 85 b0 03 00 00        	mov	qword ptr [rbp + 0x3b0], rax
140987f3e: 48 c7 85 b8 03 00 00 1b 00 00 00    	mov	qword ptr [rbp + 0x3b8], 0x1b
140987f49: 48 8d 95 b0 03 00 00        	lea	rdx, [rbp + 0x3b0]
140987f50: 4c 89 f9                    	mov	rcx, r15
140987f53: e8 78 15 26 00              	call	0x140be94d0 <.text+0xbe84d0>
140987f58: 0f 10 45 30                 	movups	xmm0, xmmword ptr [rbp + 0x30]
140987f5c: 0f 29 85 d0 01 00 00        	movaps	xmmword ptr [rbp + 0x1d0], xmm0
140987f63: 89 85 e0 01 00 00           	mov	dword ptr [rbp + 0x1e0], eax
140987f69: 4c 89 7c 24 30              	mov	qword ptr [rsp + 0x30], r15
140987f6e: 48 89 5c 24 28              	mov	qword ptr [rsp + 0x28], rbx
140987f73: 48 c7 44 24 20 1e 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1e
140987f7c: 48 8d 15 c4 0c 13 02        	lea	rdx, [rip + 0x2130cc4]  # 0x142ab8c47
140987f83: 4c 8d 0d d2 0c 13 02        	lea	r9, [rip + 0x2130cd2]   # 0x142ab8c5c
140987f8a: 4c 8d ad b0 03 00 00        	lea	r13, [rbp + 0x3b0]
140987f91: 41 b8 15 00 00 00           	mov	r8d, 0x15
140987f97: 4c 89 e9                    	mov	rcx, r13
140987f9a: e8 01 b3 c7 00              	call	0x1416032a0 <.text+0x16022a0>
140987f9f: 4c 8d b5 20 02 00 00        	lea	r14, [rbp + 0x220]
140987fa6: 49 b8 e3 c5 90 19 c5 9d fe 86       	movabs	r8, -0x7901623ae66f3a1d
140987fb0: 4c 89 f1                    	mov	rcx, r14
140987fb3: 4c 89 ea                    	mov	rdx, r13
140987fb6: e8 55 51 c8 00              	call	0x14160d110 <.text+0x160c110>
140987fbb: 49 b8 74 44 1c 32 11 a8 61 86       	movabs	r8, -0x799e57eecde3bb8c
140987fc5: 4c 89 e9                    	mov	rcx, r13
140987fc8: 4c 89 f2                    	mov	rdx, r14
140987fcb: e8 30 e9 c7 00              	call	0x141606900 <.text+0x1605900>
140987fd0: 49 b8 ba 2c 07 ed b8 e4 8e 2d       	movabs	r8, 0x2d8ee4b8ed072cba
140987fda: 4c 89 f1                    	mov	rcx, r14
140987fdd: 4c 89 ea                    	mov	rdx, r13
140987fe0: e8 4b ca c7 00              	call	0x141604a30 <.text+0x1603a30>
140987fe5: 48 8d 8d b0 03 00 00        	lea	rcx, [rbp + 0x3b0]
140987fec: 49 b8 4d f3 05 c9 b1 38 6c 68       	movabs	r8, 0x686c38b1c905f34d
140987ff6: 4c 89 f2                    	mov	rdx, r14
140987ff9: e8 32 e7 c7 00              	call	0x141606730 <.text+0x1605730>
140987ffe: 0f 10 85 50 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x450]
140988005: 0f 29 45 70                 	movaps	xmmword ptr [rbp + 0x70], xmm0
140988009: 0f 10 85 40 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x440]
140988010: 0f 29 45 60                 	movaps	xmmword ptr [rbp + 0x60], xmm0
140988014: 0f 10 85 30 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x430]
14098801b: 0f 29 45 50                 	movaps	xmmword ptr [rbp + 0x50], xmm0
14098801f: 0f 10 85 20 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x420]
140988026: 0f 29 45 40                 	movaps	xmmword ptr [rbp + 0x40], xmm0
14098802a: 0f 10 85 10 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x410]
140988031: 0f 29 45 30                 	movaps	xmmword ptr [rbp + 0x30], xmm0
140988035: 48 8b 85 e8 04 00 00        	mov	rax, qword ptr [rbp + 0x4e8]
14098803c: 48 8b 8d 30 05 00 00        	mov	rcx, qword ptr [rbp + 0x530]
140988043: 48 8b 95 c0 03 00 00        	mov	rdx, qword ptr [rbp + 0x3c0]
14098804a: 48 89 95 30 02 00 00        	mov	qword ptr [rbp + 0x230], rdx
140988051: 0f 10 85 b0 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x3b0]
140988058: 0f 29 85 20 02 00 00        	movaps	xmmword ptr [rbp + 0x220], xmm0
14098805f: 0f 10 85 c8 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x3c8]
140988066: 0f 29 85 50 02 00 00        	movaps	xmmword ptr [rbp + 0x250], xmm0
14098806d: 48 8b 95 d8 03 00 00        	mov	rdx, qword ptr [rbp + 0x3d8]
140988074: 48 89 95 60 02 00 00        	mov	qword ptr [rbp + 0x260], rdx
14098807b: 0f 10 85 e0 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x3e0]
140988082: 0f 11 85 38 02 00 00        	movups	xmmword ptr [rbp + 0x238], xmm0
140988089: 48 8b 95 f0 03 00 00        	mov	rdx, qword ptr [rbp + 0x3f0]
140988090: 48 89 95 48 02 00 00        	mov	qword ptr [rbp + 0x248], rdx
140988097: 0f 10 85 60 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x460]
14098809e: 0f 10 8d 70 04 00 00        	movups	xmm1, xmmword ptr [rbp + 0x470]
1409880a5: 0f 10 95 80 04 00 00        	movups	xmm2, xmmword ptr [rbp + 0x480]
1409880ac: 0f 10 9d 90 04 00 00        	movups	xmm3, xmmword ptr [rbp + 0x490]
1409880b3: 0f 11 85 98 02 00 00        	movups	xmmword ptr [rbp + 0x298], xmm0
1409880ba: 0f 11 8d a8 02 00 00        	movups	xmmword ptr [rbp + 0x2a8], xmm1
1409880c1: 0f 11 95 b8 02 00 00        	movups	xmmword ptr [rbp + 0x2b8], xmm2
1409880c8: 0f 11 9d c8 02 00 00        	movups	xmmword ptr [rbp + 0x2c8], xmm3
1409880cf: 0f 10 85 a0 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x4a0]
1409880d6: 0f 11 85 d8 02 00 00        	movups	xmmword ptr [rbp + 0x2d8], xmm0
1409880dd: 0f 10 85 b0 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x4b0]
1409880e4: 0f 11 85 e8 02 00 00        	movups	xmmword ptr [rbp + 0x2e8], xmm0
1409880eb: 48 8b 95 08 04 00 00        	mov	rdx, qword ptr [rbp + 0x408]
1409880f2: 48 89 95 78 02 00 00        	mov	qword ptr [rbp + 0x278], rdx
1409880f9: 0f 10 85 f8 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x3f8]
140988100: 0f 11 85 68 02 00 00        	movups	xmmword ptr [rbp + 0x268], xmm0
140988107: 48 89 8d 00 03 00 00        	mov	qword ptr [rbp + 0x300], rcx
14098810e: 48 89 85 f8 02 00 00        	mov	qword ptr [rbp + 0x2f8], rax
140988115: 0f 28 35 b4 02 13 02        	movaps	xmm6, xmmword ptr [rip + 0x21302b4] # 0x142ab83d0
14098811c: 0f 29 b5 80 02 00 00        	movaps	xmmword ptr [rbp + 0x280], xmm6
140988123: 48 8d 4d 30                 	lea	rcx, [rbp + 0x30]
140988127: 48 8d 95 20 02 00 00        	lea	rdx, [rbp + 0x220]
14098812e: e8 9d 53 05 00              	call	0x1409dd4d0 <.text+0x9dc4d0>
140988133: 80 bd e6 04 00 00 08        	cmp	byte ptr [rbp + 0x4e6], 0x8
14098813a: 74 0c                       	je	0x140988148 <.text+0x987148>
14098813c: 48 8d 8d d0 04 00 00        	lea	rcx, [rbp + 0x4d0]
140988143: e8 68 a3 25 00              	call	0x140be24b0 <.text+0xbe14b0>
140988148: 4c 89 7c 24 30              	mov	qword ptr [rsp + 0x30], r15
14098814d: 48 89 5c 24 28              	mov	qword ptr [rsp + 0x28], rbx
140988152: 48 c7 44 24 20 1d 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1d
14098815b: 48 8d 15 26 0b 13 02        	lea	rdx, [rip + 0x2130b26]  # 0x142ab8c88
140988162: 4c 8d 0d 34 0b 13 02        	lea	r9, [rip + 0x2130b34]   # 0x142ab8c9d
140988169: 4c 8d b5 b0 03 00 00        	lea	r14, [rbp + 0x3b0]
140988170: 41 b8 15 00 00 00           	mov	r8d, 0x15
140988176: 4c 89 f1                    	mov	rcx, r14
140988179: e8 22 b1 c7 00              	call	0x1416032a0 <.text+0x16022a0>
14098817e: 48 8d 8d 20 02 00 00        	lea	rcx, [rbp + 0x220]
140988185: 49 b8 01 f1 2c d7 c9 c4 e3 fe       	movabs	r8, -0x11c3b3628d30eff
14098818f: 4c 8d 8d f0 01 00 00        	lea	r9, [rbp + 0x1f0]
140988196: 4c 89 f2                    	mov	rdx, r14
140988199: e8 c2 5b fe ff              	call	0x14096dd60 <.text+0x96cd60>
14098819e: 0f 10 85 c0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2c0]
1409881a5: 0f 29 45 70                 	movaps	xmmword ptr [rbp + 0x70], xmm0
1409881a9: 0f 10 85 b0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2b0]
1409881b0: 0f 29 45 60                 	movaps	xmmword ptr [rbp + 0x60], xmm0
1409881b4: 0f 10 85 a0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2a0]
1409881bb: 0f 29 45 50                 	movaps	xmmword ptr [rbp + 0x50], xmm0
1409881bf: 0f 10 85 90 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x290]
1409881c6: 0f 29 45 40                 	movaps	xmmword ptr [rbp + 0x40], xmm0
1409881ca: 0f 10 85 80 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x280]
1409881d1: 0f 29 45 30                 	movaps	xmmword ptr [rbp + 0x30], xmm0
1409881d5: 48 8b 85 58 03 00 00        	mov	rax, qword ptr [rbp + 0x358]
1409881dc: 48 8b 8d a0 03 00 00        	mov	rcx, qword ptr [rbp + 0x3a0]
1409881e3: 48 8b 95 30 02 00 00        	mov	rdx, qword ptr [rbp + 0x230]
1409881ea: 48 89 95 c0 03 00 00        	mov	qword ptr [rbp + 0x3c0], rdx
1409881f1: 0f 10 85 20 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x220]
1409881f8: 0f 29 85 b0 03 00 00        	movaps	xmmword ptr [rbp + 0x3b0], xmm0
1409881ff: 0f 10 85 38 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x238]
140988206: 0f 29 85 e0 03 00 00        	movaps	xmmword ptr [rbp + 0x3e0], xmm0
14098820d: 48 8b 95 48 02 00 00        	mov	rdx, qword ptr [rbp + 0x248]
140988214: 48 89 95 f0 03 00 00        	mov	qword ptr [rbp + 0x3f0], rdx
14098821b: 0f 10 85 50 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x250]
140988222: 0f 11 85 c8 03 00 00        	movups	xmmword ptr [rbp + 0x3c8], xmm0
140988229: 48 8b 95 60 02 00 00        	mov	rdx, qword ptr [rbp + 0x260]
140988230: 48 89 95 d8 03 00 00        	mov	qword ptr [rbp + 0x3d8], rdx
140988237: 0f 10 85 d0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2d0]
14098823e: 0f 10 8d e0 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x2e0]
140988245: 0f 10 95 f0 02 00 00        	movups	xmm2, xmmword ptr [rbp + 0x2f0]
14098824c: 0f 10 9d 00 03 00 00        	movups	xmm3, xmmword ptr [rbp + 0x300]
140988253: 0f 11 85 28 04 00 00        	movups	xmmword ptr [rbp + 0x428], xmm0
14098825a: 0f 11 8d 38 04 00 00        	movups	xmmword ptr [rbp + 0x438], xmm1
140988261: 0f 11 95 48 04 00 00        	movups	xmmword ptr [rbp + 0x448], xmm2
140988268: 0f 11 9d 58 04 00 00        	movups	xmmword ptr [rbp + 0x458], xmm3
14098826f: 0f 10 85 10 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x310]
140988276: 0f 11 85 68 04 00 00        	movups	xmmword ptr [rbp + 0x468], xmm0
14098827d: 0f 10 85 20 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x320]
140988284: 0f 11 85 78 04 00 00        	movups	xmmword ptr [rbp + 0x478], xmm0
14098828b: 48 8b 95 78 02 00 00        	mov	rdx, qword ptr [rbp + 0x278]
140988292: 48 89 95 08 04 00 00        	mov	qword ptr [rbp + 0x408], rdx
140988299: 0f 10 85 68 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x268]
1409882a0: 0f 11 85 f8 03 00 00        	movups	xmmword ptr [rbp + 0x3f8], xmm0
1409882a7: 48 89 8d 90 04 00 00        	mov	qword ptr [rbp + 0x490], rcx
1409882ae: 48 89 85 88 04 00 00        	mov	qword ptr [rbp + 0x488], rax
1409882b5: 0f 28 05 24 01 13 02        	movaps	xmm0, xmmword ptr [rip + 0x2130124] # 0x142ab83e0
1409882bc: 0f 29 85 10 04 00 00        	movaps	xmmword ptr [rbp + 0x410], xmm0
1409882c3: 48 8d 4d 30                 	lea	rcx, [rbp + 0x30]
1409882c7: 48 8d 95 b0 03 00 00        	lea	rdx, [rbp + 0x3b0]
1409882ce: e8 fd 51 05 00              	call	0x1409dd4d0 <.text+0x9dc4d0>
1409882d3: 80 bd 56 03 00 00 08        	cmp	byte ptr [rbp + 0x356], 0x8
1409882da: 74 0c                       	je	0x1409882e8 <.text+0x9872e8>
1409882dc: 48 8d 8d 40 03 00 00        	lea	rcx, [rbp + 0x340]
1409882e3: e8 c8 a1 25 00              	call	0x140be24b0 <.text+0xbe14b0>
1409882e8: 48 ba 55 93 67 6e 78 cc 26 82       	movabs	rdx, -0x7dd9338791986cab
1409882f2: 49 b8 ac 41 a8 55 6a da cf 65       	movabs	r8, 0x65cfda6a55a841ac
1409882fc: 4c 89 e1                    	mov	rcx, r12
1409882ff: e8 ec 53 75 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
140988304: 48 83 bd 80 05 00 00 00     	cmp	qword ptr [rbp + 0x580], 0x0
14098830c: 40 88 b5 c0 05 00 00        	mov	byte ptr [rbp + 0x5c0], sil
140988313: 74 6d                       	je	0x140988382 <.text+0x987382>
140988315: 48 85 c0                    	test	rax, rax
140988318: 0f 84 3d 13 00 00           	je	0x14098965b <.text+0x98865b>
14098831e: 48 39 93 18 02 00 00        	cmp	qword ptr [rbx + 0x218], rdx
140988325: 0f 86 30 13 00 00           	jbe	0x14098965b <.text+0x98865b>
14098832b: 48 8b 83 10 02 00 00        	mov	rax, qword ptr [rbx + 0x210]
140988332: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
140988336: 48 85 c9                    	test	rcx, rcx
140988339: 0f 84 1c 13 00 00           	je	0x14098965b <.text+0x98865b>
14098833f: 48 8b 83 e0 01 00 00        	mov	rax, qword ptr [rbx + 0x1e0]
140988346: 48 f7 d1                    	not	rcx
140988349: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
14098834d: 48 c1 e1 05                 	shl	rcx, 0x5
140988351: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
140988357: 0f 84 fe 12 00 00           	je	0x14098965b <.text+0x98865b>
14098835d: 48 01 c8                    	add	rax, rcx
140988360: 48 8b 50 10                 	mov	rdx, qword ptr [rax + 0x10]
140988364: 4c 89 f1                    	mov	rcx, r14
140988367: 4d 89 f8                    	mov	r8, r15
14098836a: e8 a1 64 94 00              	call	0x1412ce810 <.text+0x12cd810>
14098836f: 4c 89 f9                    	mov	rcx, r15
140988372: 48 89 da                    	mov	rdx, rbx
140988375: 4d 89 f0                    	mov	r8, r14
140988378: e8 33 58 05 00              	call	0x1409ddbb0 <.text+0x9dcbb0>
14098837d: 40 b6 01                    	mov	sil, 0x1
140988380: eb 6b                       	jmp	0x1409883ed <.text+0x9873ed>
140988382: 48 85 c0                    	test	rax, rax
140988385: 0f 84 d0 12 00 00           	je	0x14098965b <.text+0x98865b>
14098838b: 48 39 93 18 02 00 00        	cmp	qword ptr [rbx + 0x218], rdx
140988392: 0f 86 c3 12 00 00           	jbe	0x14098965b <.text+0x98865b>
140988398: 48 8b 83 10 02 00 00        	mov	rax, qword ptr [rbx + 0x210]
14098839f: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
1409883a3: 48 85 c9                    	test	rcx, rcx
1409883a6: 0f 84 af 12 00 00           	je	0x14098965b <.text+0x98865b>
1409883ac: 48 8b 83 e0 01 00 00        	mov	rax, qword ptr [rbx + 0x1e0]
1409883b3: 48 f7 d1                    	not	rcx
1409883b6: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1409883ba: 48 c1 e1 05                 	shl	rcx, 0x5
1409883be: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
1409883c4: 0f 84 91 12 00 00           	je	0x14098965b <.text+0x98865b>
1409883ca: 48 01 c8                    	add	rax, rcx
1409883cd: 48 8b 50 10                 	mov	rdx, qword ptr [rax + 0x10]
1409883d1: 4c 89 f1                    	mov	rcx, r14
1409883d4: 4d 89 f8                    	mov	r8, r15
1409883d7: e8 a4 8d 94 00              	call	0x1412d1180 <.text+0x12d0180>
1409883dc: 4c 89 f9                    	mov	rcx, r15
1409883df: 48 89 da                    	mov	rdx, rbx
1409883e2: 4d 89 f0                    	mov	r8, r14
1409883e5: e8 c6 57 05 00              	call	0x1409ddbb0 <.text+0x9dcbb0>
1409883ea: 40 b6 04                    	mov	sil, 0x4
1409883ed: 4c 8b a5 a0 05 00 00        	mov	r12, qword ptr [rbp + 0x5a0]
1409883f4: 49 8b bc 24 50 14 00 00     	mov	rdi, qword ptr [r12 + 0x1450]
1409883fc: 4c 89 7c 24 30              	mov	qword ptr [rsp + 0x30], r15
140988401: 48 89 9d 98 05 00 00        	mov	qword ptr [rbp + 0x598], rbx
140988408: 48 89 5c 24 28              	mov	qword ptr [rsp + 0x28], rbx
14098840d: 48 c7 44 24 20 29 00 00 00  	mov	qword ptr [rsp + 0x20], 0x29
140988416: 48 8d 15 4d 27 14 02        	lea	rdx, [rip + 0x214274d]  # 0x142acab6a
14098841d: 4c 8d 0d 66 27 14 02        	lea	r9, [rip + 0x2142766]   # 0x142acab8a
140988424: 48 8d 9d b0 03 00 00        	lea	rbx, [rbp + 0x3b0]
14098842b: 41 b8 20 00 00 00           	mov	r8d, 0x20
140988431: 48 89 d9                    	mov	rcx, rbx
140988434: e8 67 ae c7 00              	call	0x1416032a0 <.text+0x16022a0>
140988439: 4d 8d b4 24 58 14 00 00     	lea	r14, [r12 + 0x1458]
140988441: 4c 8d ad 20 02 00 00        	lea	r13, [rbp + 0x220]
140988448: 49 b8 bd cc bd d5 b2 ae 75 2a       	movabs	r8, 0x2a75aeb2d5bdccbd
140988452: 4c 89 e9                    	mov	rcx, r13
140988455: 48 89 da                    	mov	rdx, rbx
140988458: 4d 89 f1                    	mov	r9, r14
14098845b: e8 40 5e fe ff              	call	0x14096e2a0 <.text+0x96d2a0>
140988460: 49 b8 df 8b be ec 2e 03 24 d0       	movabs	r8, -0x2fdbfcd113417421
14098846a: 48 89 d9                    	mov	rcx, rbx
14098846d: 4c 89 ea                    	mov	rdx, r13
140988470: e8 1b fc c7 00              	call	0x141608090 <.text+0x1607090>
140988475: c6 85 28 02 00 00 00        	mov	byte ptr [rbp + 0x228], 0x0
14098847c: 40 88 b5 af 05 00 00        	mov	byte ptr [rbp + 0x5af], sil
140988483: 40 88 b5 29 02 00 00        	mov	byte ptr [rbp + 0x229], sil
14098848a: 48 89 bd 20 02 00 00        	mov	qword ptr [rbp + 0x220], rdi
140988491: 48 8d 95 20 02 00 00        	lea	rdx, [rbp + 0x220]
140988498: 48 89 d9                    	mov	rcx, rbx
14098849b: e8 40 17 c8 00              	call	0x141609be0 <.text+0x1608be0>
1409884a0: 48 8d 8d 08 02 00 00        	lea	rcx, [rbp + 0x208]
1409884a7: 4c 89 f2                    	mov	rdx, r14
1409884aa: e8 51 e6 24 00              	call	0x140bd6b00 <.text+0xbd5b00>
1409884af: 49 8d 94 24 80 14 00 00     	lea	rdx, [r12 + 0x1480]
1409884b7: 48 8d 8d 40 05 00 00        	lea	rcx, [rbp + 0x540]
1409884be: e8 3d e6 24 00              	call	0x140bd6b00 <.text+0xbd5b00>
1409884c3: 4c 89 bd b0 05 00 00        	mov	qword ptr [rbp + 0x5b0], r15
1409884ca: 49 8b 87 90 01 00 00        	mov	rax, qword ptr [r15 + 0x190]
1409884d1: c6 85 30 02 00 00 00        	mov	byte ptr [rbp + 0x230], 0x0
1409884d8: 48 8b 8d a0 05 00 00        	mov	rcx, qword ptr [rbp + 0x5a0]
1409884df: 48 89 8d 20 02 00 00        	mov	qword ptr [rbp + 0x220], rcx
1409884e6: 48 89 85 28 02 00 00        	mov	qword ptr [rbp + 0x228], rax
1409884ed: 48 8d 8d b0 03 00 00        	lea	rcx, [rbp + 0x3b0]
1409884f4: 48 8d 95 20 02 00 00        	lea	rdx, [rbp + 0x220]
1409884fb: e8 a0 32 1a 00              	call	0x140b2b7a0 <.text+0xb2a7a0>
140988500: 0f b6 85 c0 05 00 00        	movzx	eax, byte ptr [rbp + 0x5c0]
140988507: 31 d2                       	xor	edx, edx
140988509: 88 c2                       	mov	dl, al
14098850b: 48 8b 85 b0 03 00 00        	mov	rax, qword ptr [rbp + 0x3b0]
140988512: 48 89 85 c0 05 00 00        	mov	qword ptr [rbp + 0x5c0], rax
140988519: 48 8b 85 b8 03 00 00        	mov	rax, qword ptr [rbp + 0x3b8]
140988520: 48 89 85 90 05 00 00        	mov	qword ptr [rbp + 0x590], rax
140988527: 48 8b b5 c0 03 00 00        	mov	rsi, qword ptr [rbp + 0x3c0]
14098852e: 48 8b 85 c8 03 00 00        	mov	rax, qword ptr [rbp + 0x3c8]
140988535: 48 89 85 88 05 00 00        	mov	qword ptr [rbp + 0x588], rax
14098853c: 48 8b 85 d0 03 00 00        	mov	rax, qword ptr [rbp + 0x3d0]
140988543: 48 89 85 c8 01 00 00        	mov	qword ptr [rbp + 0x1c8], rax
14098854a: 48 8b bd d8 03 00 00        	mov	rdi, qword ptr [rbp + 0x3d8]
140988551: c6 85 bf 05 00 00 01        	mov	byte ptr [rbp + 0x5bf], 0x1
140988558: 48 8b 85 80 05 00 00        	mov	rax, qword ptr [rbp + 0x580]
14098855f: 48 89 44 24 48              	mov	qword ptr [rsp + 0x48], rax
140988564: 89 54 24 40                 	mov	dword ptr [rsp + 0x40], edx
140988568: 48 8d 85 40 05 00 00        	lea	rax, [rbp + 0x540]
14098856f: 48 89 44 24 38              	mov	qword ptr [rsp + 0x38], rax
140988574: 48 8d 85 08 02 00 00        	lea	rax, [rbp + 0x208]
14098857b: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
140988580: 48 8d 85 f0 01 00 00        	lea	rax, [rbp + 0x1f0]
140988587: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
14098858c: 48 8d 85 d0 01 00 00        	lea	rax, [rbp + 0x1d0]
140988593: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
140988598: 48 8d 8d b0 03 00 00        	lea	rcx, [rbp + 0x3b0]
14098859f: 89 95 a8 05 00 00           	mov	dword ptr [rbp + 0x5a8], edx
1409885a5: 4c 8b 85 98 05 00 00        	mov	r8, qword ptr [rbp + 0x598]
1409885ac: 4c 8b 8d b0 05 00 00        	mov	r9, qword ptr [rbp + 0x5b0]
1409885b3: e8 98 03 0a 00              	call	0x140a28950 <.text+0xa27950>
1409885b8: 48 8b 85 c0 05 00 00        	mov	rax, qword ptr [rbp + 0x5c0]
1409885bf: 48 89 85 20 02 00 00        	mov	qword ptr [rbp + 0x220], rax
1409885c6: 48 8b 85 90 05 00 00        	mov	rax, qword ptr [rbp + 0x590]
1409885cd: 48 89 85 28 02 00 00        	mov	qword ptr [rbp + 0x228], rax
1409885d4: 48 89 b5 30 02 00 00        	mov	qword ptr [rbp + 0x230], rsi
1409885db: 48 8b 85 88 05 00 00        	mov	rax, qword ptr [rbp + 0x588]
1409885e2: 48 89 85 38 02 00 00        	mov	qword ptr [rbp + 0x238], rax
1409885e9: 48 8b 85 c8 01 00 00        	mov	rax, qword ptr [rbp + 0x1c8]
1409885f0: 48 89 85 40 02 00 00        	mov	qword ptr [rbp + 0x240], rax
1409885f7: 48 89 bd 48 02 00 00        	mov	qword ptr [rbp + 0x248], rdi
1409885fe: c6 85 bf 05 00 00 00        	mov	byte ptr [rbp + 0x5bf], 0x0
140988605: 48 8d 8d b0 03 00 00        	lea	rcx, [rbp + 0x3b0]
14098860c: 48 8d 95 20 02 00 00        	lea	rdx, [rbp + 0x220]
140988613: e8 c8 1c c8 00              	call	0x14160a2e0 <.text+0x16092e0>
140988618: 4c 8d bd 90 00 00 00        	lea	r15, [rbp + 0x90]
14098861f: c7 85 c0 05 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x5c0], 0x0
140988629: eb 65                       	jmp	0x140988690 <.text+0x987690>
14098862b: 8b 8d a8 05 00 00           	mov	ecx, dword ptr [rbp + 0x5a8]
140988631: 89 4c 24 48                 	mov	dword ptr [rsp + 0x48], ecx
140988635: 48 8b 85 80 05 00 00        	mov	rax, qword ptr [rbp + 0x580]
14098863c: 48 89 44 24 38              	mov	qword ptr [rsp + 0x38], rax
140988641: 89 4c 24 30                 	mov	dword ptr [rsp + 0x30], ecx
140988645: 48 8d 85 40 05 00 00        	lea	rax, [rbp + 0x540]
14098864c: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
140988651: 48 8d 85 08 02 00 00        	lea	rax, [rbp + 0x208]
140988658: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
14098865d: c6 44 24 40 0d              	mov	byte ptr [rsp + 0x40], 0xd
140988662: 4c 8d 85 f0 01 00 00        	lea	r8, [rbp + 0x1f0]
140988669: 4c 8d 8d d0 01 00 00        	lea	r9, [rbp + 0x1d0]
140988670: 48 8b 8d b0 05 00 00        	mov	rcx, qword ptr [rbp + 0x5b0]
140988677: 48 8b 95 98 05 00 00        	mov	rdx, qword ptr [rbp + 0x598]
14098867e: e8 8d f4 09 00              	call	0x140a27b10 <.text+0xa26b10>
140988683: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140988690: 44 8b 85 c0 05 00 00        	mov	r8d, dword ptr [rbp + 0x5c0]
140988697: 41 0f b6 c0                 	movzx	eax, r8b
14098869b: 48 8d 0d fe 0c 13 02        	lea	rcx, [rip + 0x2130cfe]  # 0x142ab93a0
1409886a2: 48 63 14 81                 	movsxd	rdx, dword ptr [rcx + 4*rax]
1409886a6: 48 01 ca                    	add	rdx, rcx
1409886a9: 44 89 c0                    	mov	eax, r8d
1409886ac: 44 89 c1                    	mov	ecx, r8d
1409886af: 41 b0 02                    	mov	r8b, 0x2
1409886b2: 44 89 85 c0 05 00 00        	mov	dword ptr [rbp + 0x5c0], r8d
1409886b9: 4c 8b 85 a0 05 00 00        	mov	r8, qword ptr [rbp + 0x5a0]
1409886c0: ff e2                       	jmp	rdx
1409886c2: b1 01                       	mov	cl, 0x1
1409886c4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
1409886d0: 89 8d c0 05 00 00           	mov	dword ptr [rbp + 0x5c0], ecx
1409886d6: 89 c1                       	mov	ecx, eax
1409886d8: 0f b6 f9                    	movzx	edi, cl
1409886db: 48 8d 04 7f                 	lea	rax, [rdi + 2*rdi]
1409886df: c1 e0 06                    	shl	eax, 0x6
1409886e2: 49 8d 1c 00                 	lea	rbx, [r8 + rax]
1409886e6: 48 83 c3 08                 	add	rbx, 0x8
1409886ea: 49 8b 44 00 08              	mov	rax, qword ptr [r8 + rax + 0x8]
1409886ef: 48 ba 01 00 00 00 00 00 00 80       	movabs	rdx, -0x7fffffffffffffff
1409886f9: 48 39 d0                    	cmp	rax, rdx
1409886fc: 74 92                       	je	0x140988690 <.text+0x987690>
1409886fe: c0 e1 02                    	shl	cl, 0x2
140988701: 44 0f b6 e1                 	movzx	r12d, cl
140988705: 48 f7 d8                    	neg	rax
140988708: 0f 81 22 04 00 00           	jno	0x140988b30 <.text+0x987b30>
14098870e: 48 8d 05 67 0d 13 02        	lea	rax, [rip + 0x2130d67]  # 0x142ab947c
140988715: 48 63 0c b8                 	movsxd	rcx, dword ptr [rax + 4*rdi]
140988719: 48 01 c1                    	add	rcx, rax
14098871c: ff e1                       	jmp	rcx
14098871e: 48 8d 4d 30                 	lea	rcx, [rbp + 0x30]
140988722: ba 10 00 00 00              	mov	edx, 0x10
140988727: 45 31 c0                    	xor	r8d, r8d
14098872a: e8 f1 c6 24 00              	call	0x140bd4e20 <.text+0xbd3e20>
14098872f: 0f 10 45 30                 	movups	xmm0, xmmword ptr [rbp + 0x30]
140988733: 0f 29 85 20 02 00 00        	movaps	xmmword ptr [rbp + 0x220], xmm0
14098873a: 48 8d 95 20 02 00 00        	lea	rdx, [rbp + 0x220]
140988741: 48 8d 8d c0 03 00 00        	lea	rcx, [rbp + 0x3c0]
140988748: e8 b3 e5 24 00              	call	0x140bd6d00 <.text+0xbd5d00>
14098874d: 48 8d 05 c4 05 13 02        	lea	rax, [rip + 0x21305c4]  # 0x142ab8d18
140988754: 48 89 85 b0 03 00 00        	mov	qword ptr [rbp + 0x3b0], rax
14098875b: 48 c7 85 b8 03 00 00 0d 00 00 00    	mov	qword ptr [rbp + 0x3b8], 0xd
140988766: 48 8d 95 b0 03 00 00        	lea	rdx, [rbp + 0x3b0]
14098876d: 48 8b 8d b0 05 00 00        	mov	rcx, qword ptr [rbp + 0x5b0]
140988774: e8 57 0d 26 00              	call	0x140be94d0 <.text+0xbe84d0>
140988779: 0f 10 45 30                 	movups	xmm0, xmmword ptr [rbp + 0x30]
14098877d: 0f 29 85 60 05 00 00        	movaps	xmmword ptr [rbp + 0x560], xmm0
140988784: 89 85 70 05 00 00           	mov	dword ptr [rbp + 0x570], eax
14098878a: 48 8b 85 b0 05 00 00        	mov	rax, qword ptr [rbp + 0x5b0]
140988791: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
140988796: 48 8b 85 98 05 00 00        	mov	rax, qword ptr [rbp + 0x598]
14098879d: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
1409887a2: 48 c7 44 24 20 2f 00 00 00  	mov	qword ptr [rsp + 0x20], 0x2f
1409887ab: 48 8d 15 73 05 13 02        	lea	rdx, [rip + 0x2130573]  # 0x142ab8d25
1409887b2: 4c 8d 0d 81 05 13 02        	lea	r9, [rip + 0x2130581]   # 0x142ab8d3a
1409887b9: 48 8d 8d b0 03 00 00        	lea	rcx, [rbp + 0x3b0]
1409887c0: 41 b8 15 00 00 00           	mov	r8d, 0x15
1409887c6: e8 d5 aa c7 00              	call	0x1416032a0 <.text+0x16022a0>
1409887cb: 48 8d 8d 20 02 00 00        	lea	rcx, [rbp + 0x220]
1409887d2: 48 8d 95 b0 03 00 00        	lea	rdx, [rbp + 0x3b0]
1409887d9: 49 b8 7e fc 48 4b 83 ca 89 b6       	movabs	r8, -0x4976357cb4b70382
1409887e3: e8 c8 4c c8 00              	call	0x14160d4b0 <.text+0x160c4b0>
1409887e8: 48 8d 8d b0 03 00 00        	lea	rcx, [rbp + 0x3b0]
1409887ef: 48 8d 95 20 02 00 00        	lea	rdx, [rbp + 0x220]
1409887f6: 49 b8 ef 60 20 6a f8 d5 b6 d8       	movabs	r8, -0x27492a0795df9f11
140988800: 4c 8d 8d 60 05 00 00        	lea	r9, [rbp + 0x560]
140988807: e8 54 55 fe ff              	call	0x14096dd60 <.text+0x96cd60>
14098880c: 0f b6 05 f6 c3 ca 02        	movzx	eax, byte ptr [rip + 0x2cac3f6] # 0x143634c09
140988813: b9 08 00 00 00              	mov	ecx, 0x8
140988818: ba 01 00 00 00              	mov	edx, 0x1
14098881d: e8 ee b3 c8 ff              	call	0x140613c10 <.text+0x612c10>
140988822: 48 85 c0                    	test	rax, rax
140988825: 0f 84 c2 0d 00 00           	je	0x1409895ed <.text+0x9885ed>
14098882b: 49 89 c6                    	mov	r14, rax
14098882e: 48 b8 0b 00 00 00 40 00 00 00       	movabs	rax, 0x400000000b
140988838: 49 89 06                    	mov	qword ptr [r14], rax
14098883b: 48 8b 95 f8 03 00 00        	mov	rdx, qword ptr [rbp + 0x3f8]
140988842: 48 85 d2                    	test	rdx, rdx
140988845: 74 12                       	je	0x140988859 <.text+0x987859>
140988847: 48 8b 8d 00 04 00 00        	mov	rcx, qword ptr [rbp + 0x400]
14098884e: 41 b8 01 00 00 00           	mov	r8d, 0x1
140988854: e8 c7 b3 c8 ff              	call	0x140613c20 <.text+0x612c20>
140988859: 48 c7 85 f8 03 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x3f8], 0x8
140988864: 4c 89 b5 00 04 00 00        	mov	qword ptr [rbp + 0x400], r14
14098886b: 48 c7 85 08 04 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x408], 0x8
140988876: 48 8d 4d 30                 	lea	rcx, [rbp + 0x30]
14098887a: 48 8d 95 b0 03 00 00        	lea	rdx, [rbp + 0x3b0]
140988881: 41 b8 90 01 00 00           	mov	r8d, 0x190
140988887: e8 74 b3 f9 01              	call	0x142923c00 <.text+0x2922c00>
14098888c: 41 0f 10 47 40              	movups	xmm0, xmmword ptr [r15 + 0x40]
140988891: 0f 29 85 60 02 00 00        	movaps	xmmword ptr [rbp + 0x260], xmm0
140988898: 41 0f 10 07                 	movups	xmm0, xmmword ptr [r15]
14098889c: 41 0f 10 4f 10              	movups	xmm1, xmmword ptr [r15 + 0x10]
1409888a1: 41 0f 10 57 20              	movups	xmm2, xmmword ptr [r15 + 0x20]
1409888a6: 41 0f 10 5f 30              	movups	xmm3, xmmword ptr [r15 + 0x30]
1409888ab: 0f 29 9d 50 02 00 00        	movaps	xmmword ptr [rbp + 0x250], xmm3
1409888b2: 0f 29 95 40 02 00 00        	movaps	xmmword ptr [rbp + 0x240], xmm2
1409888b9: 0f 29 8d 30 02 00 00        	movaps	xmmword ptr [rbp + 0x230], xmm1
1409888c0: 0f 29 85 20 02 00 00        	movaps	xmmword ptr [rbp + 0x220], xmm0
1409888c7: 48 8b 85 68 01 00 00        	mov	rax, qword ptr [rbp + 0x168]
1409888ce: 48 8b 8d b0 01 00 00        	mov	rcx, qword ptr [rbp + 0x1b0]
1409888d5: 48 8b 55 40                 	mov	rdx, qword ptr [rbp + 0x40]
1409888d9: 48 89 95 c0 03 00 00        	mov	qword ptr [rbp + 0x3c0], rdx
1409888e0: 0f 10 45 30                 	movups	xmm0, xmmword ptr [rbp + 0x30]
1409888e4: 0f 29 85 b0 03 00 00        	movaps	xmmword ptr [rbp + 0x3b0], xmm0
1409888eb: 49 8b 57 c8                 	mov	rdx, qword ptr [r15 - 0x38]
1409888ef: 4c 8d 85 e0 03 00 00        	lea	r8, [rbp + 0x3e0]
1409888f6: 49 89 50 10                 	mov	qword ptr [r8 + 0x10], rdx
1409888fa: 41 0f 10 47 b8              	movups	xmm0, xmmword ptr [r15 - 0x48]
1409888ff: 41 0f 11 00                 	movups	xmmword ptr [r8], xmm0
140988903: 49 8b 57 e0                 	mov	rdx, qword ptr [r15 - 0x20]
140988907: 49 89 50 f8                 	mov	qword ptr [r8 - 0x8], rdx
14098890b: 41 0f 10 47 d0              	movups	xmm0, xmmword ptr [r15 - 0x30]
140988910: 41 0f 11 40 e8              	movups	xmmword ptr [r8 - 0x18], xmm0
140988915: 41 0f 10 87 a0 00 00 00     	movups	xmm0, xmmword ptr [r15 + 0xa0]
14098891d: 41 0f 11 80 98 00 00 00     	movups	xmmword ptr [r8 + 0x98], xmm0
140988925: 41 0f 10 87 90 00 00 00     	movups	xmm0, xmmword ptr [r15 + 0x90]
14098892d: 41 0f 11 80 88 00 00 00     	movups	xmmword ptr [r8 + 0x88], xmm0
140988935: 41 0f 10 47 50              	movups	xmm0, xmmword ptr [r15 + 0x50]
14098893a: 41 0f 10 4f 60              	movups	xmm1, xmmword ptr [r15 + 0x60]
14098893f: 41 0f 10 57 70              	movups	xmm2, xmmword ptr [r15 + 0x70]
140988944: 41 0f 10 9f 80 00 00 00     	movups	xmm3, xmmword ptr [r15 + 0x80]
14098894c: 41 0f 11 58 78              	movups	xmmword ptr [r8 + 0x78], xmm3
140988951: 41 0f 11 50 68              	movups	xmmword ptr [r8 + 0x68], xmm2
140988956: 41 0f 11 48 58              	movups	xmmword ptr [r8 + 0x58], xmm1
14098895b: 41 0f 11 40 48              	movups	xmmword ptr [r8 + 0x48], xmm0
140988960: 49 8b 57 f8                 	mov	rdx, qword ptr [r15 - 0x8]
140988964: 49 89 50 28                 	mov	qword ptr [r8 + 0x28], rdx
140988968: 41 0f 10 47 e8              	movups	xmm0, xmmword ptr [r15 - 0x18]
14098896d: 41 0f 11 40 18              	movups	xmmword ptr [r8 + 0x18], xmm0
140988972: 48 89 8d 90 04 00 00        	mov	qword ptr [rbp + 0x490], rcx
140988979: 48 89 85 88 04 00 00        	mov	qword ptr [rbp + 0x488], rax
140988980: 0f 28 05 59 06 fd 01        	movaps	xmm0, xmmword ptr [rip + 0x1fd0659] # 0x142958fe0
140988987: 0f 29 85 10 04 00 00        	movaps	xmmword ptr [rbp + 0x410], xmm0
14098898e: 48 8d 8d 20 02 00 00        	lea	rcx, [rbp + 0x220]
140988995: 48 8d 95 b0 03 00 00        	lea	rdx, [rbp + 0x3b0]
14098899c: e8 2f 4b 05 00              	call	0x1409dd4d0 <.text+0x9dc4d0>
1409889a1: 80 bd 66 01 00 00 08        	cmp	byte ptr [rbp + 0x166], 0x8
1409889a8: 74 0c                       	je	0x1409889b6 <.text+0x9879b6>
1409889aa: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
1409889b1: e8 fa 9a 25 00              	call	0x140be24b0 <.text+0xbe14b0>
1409889b6: 0f 28 85 60 05 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x560]
1409889bd: 0f 29 45 30                 	movaps	xmmword ptr [rbp + 0x30], xmm0
1409889c1: 48 8b 85 70 05 00 00        	mov	rax, qword ptr [rbp + 0x570]
1409889c8: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
1409889cc: 48 8b 85 80 05 00 00        	mov	rax, qword ptr [rbp + 0x580]
1409889d3: 48 89 44 24 48              	mov	qword ptr [rsp + 0x48], rax
1409889d8: 8b 95 a8 05 00 00           	mov	edx, dword ptr [rbp + 0x5a8]
1409889de: 89 54 24 40                 	mov	dword ptr [rsp + 0x40], edx
1409889e2: 48 8d 85 40 05 00 00        	lea	rax, [rbp + 0x540]
1409889e9: 48 89 44 24 38              	mov	qword ptr [rsp + 0x38], rax
1409889ee: 48 8d 85 08 02 00 00        	lea	rax, [rbp + 0x208]
1409889f5: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
1409889fa: 48 8d 85 f0 01 00 00        	lea	rax, [rbp + 0x1f0]
140988a01: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
140988a06: 48 8d 85 d0 01 00 00        	lea	rax, [rbp + 0x1d0]
140988a0d: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
140988a12: 48 8d 8d b0 03 00 00        	lea	rcx, [rbp + 0x3b0]
140988a19: 4c 8b 85 98 05 00 00        	mov	r8, qword ptr [rbp + 0x598]
140988a20: 4c 8b 8d b0 05 00 00        	mov	r9, qword ptr [rbp + 0x5b0]
140988a27: e8 44 05 0a 00              	call	0x140a28f70 <.text+0xa27f70>
140988a2c: 0f b6 05 d6 c1 ca 02        	movzx	eax, byte ptr [rip + 0x2cac1d6] # 0x143634c09
140988a33: b9 08 00 00 00              	mov	ecx, 0x8
140988a38: ba 01 00 00 00              	mov	edx, 0x1
140988a3d: e8 ce b1 c8 ff              	call	0x140613c10 <.text+0x612c10>
140988a42: 48 85 c0                    	test	rax, rax
140988a45: 0f 84 ba 0b 00 00           	je	0x140989605 <.text+0x988605>
140988a4b: 49 89 c6                    	mov	r14, rax
140988a4e: 49 c1 e4 20                 	shl	r12, 0x20
140988a52: 49 83 cc 0b                 	or	r12, 0xb
140988a56: 4c 89 20                    	mov	qword ptr [rax], r12
140988a59: 48 8b 95 f8 03 00 00        	mov	rdx, qword ptr [rbp + 0x3f8]
140988a60: 48 85 d2                    	test	rdx, rdx
140988a63: 74 12                       	je	0x140988a77 <.text+0x987a77>
140988a65: 48 8b 8d 00 04 00 00        	mov	rcx, qword ptr [rbp + 0x400]
140988a6c: 41 b8 01 00 00 00           	mov	r8d, 0x1
140988a72: e8 a9 b1 c8 ff              	call	0x140613c20 <.text+0x612c20>
140988a77: 48 c7 85 f8 03 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x3f8], 0x8
140988a82: 4c 89 b5 00 04 00 00        	mov	qword ptr [rbp + 0x400], r14
140988a89: 48 c7 85 08 04 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x408], 0x8
140988a94: 4c 8d b5 20 02 00 00        	lea	r14, [rbp + 0x220]
140988a9b: 48 8d 95 b0 03 00 00        	lea	rdx, [rbp + 0x3b0]
140988aa2: 41 b8 90 01 00 00           	mov	r8d, 0x190
140988aa8: 4c 89 f1                    	mov	rcx, r14
140988aab: e8 50 b1 f9 01              	call	0x142923c00 <.text+0x2922c00>
140988ab0: 48 8d 8d b0 03 00 00        	lea	rcx, [rbp + 0x3b0]
140988ab7: 49 b8 b4 e4 61 20 b6 4c 0a 55       	movabs	r8, 0x550a4cb62061e4b4
140988ac1: 4c 8d 4d 30                 	lea	r9, [rbp + 0x30]
140988ac5: 4c 89 f2                    	mov	rdx, r14
140988ac8: e8 f3 4f fe ff              	call	0x14096dac0 <.text+0x96cac0>
140988acd: 48 8d 8d 20 02 00 00        	lea	rcx, [rbp + 0x220]
140988ad4: 48 8d 95 b0 03 00 00        	lea	rdx, [rbp + 0x3b0]
140988adb: 49 b8 ad 87 78 ac d4 56 92 6d       	movabs	r8, 0x6d9256d4ac7887ad
140988ae5: e8 d6 35 c8 00              	call	0x14160c0c0 <.text+0x160b0c0>
140988aea: 48 8d 8d 20 02 00 00        	lea	rcx, [rbp + 0x220]
140988af1: 48 8d 55 30                 	lea	rdx, [rbp + 0x30]
140988af5: 45 31 c0                    	xor	r8d, r8d
140988af8: e8 83 5d fe ff              	call	0x14096e880 <.text+0x96d880>
140988afd: 44 8b 85 c0 05 00 00        	mov	r8d, dword ptr [rbp + 0x5c0]
140988b04: 41 0f b6 c0                 	movzx	eax, r8b
140988b08: 48 8d 0d 91 08 13 02        	lea	rcx, [rip + 0x2130891]  # 0x142ab93a0
140988b0f: 48 63 14 81                 	movsxd	rdx, dword ptr [rcx + 4*rax]
140988b13: 48 01 ca                    	add	rdx, rcx
140988b16: 44 89 c0                    	mov	eax, r8d
140988b19: 44 89 c1                    	mov	ecx, r8d
140988b1c: 41 b0 02                    	mov	r8b, 0x2
140988b1f: 44 89 85 c0 05 00 00        	mov	dword ptr [rbp + 0x5c0], r8d
140988b26: 4c 8b 85 a0 05 00 00        	mov	r8, qword ptr [rbp + 0x5a0]
140988b2d: ff e2                       	jmp	rdx
140988b2f: 90                          	nop
140988b30: 4c 8d 2d 39 2e be 00        	lea	r13, [rip + 0xbe2e39]   # 0x14156b970 <.text+0x156a970>
140988b37: 48 8d 05 d2 08 13 02        	lea	rax, [rip + 0x21308d2]  # 0x142ab9410
140988b3e: 48 63 0c b8                 	movsxd	rcx, dword ptr [rax + 4*rdi]
140988b42: 48 01 c1                    	add	rcx, rax
140988b45: ff e1                       	jmp	rcx
140988b47: 4c 8d 2d a2 23 be 00        	lea	r13, [rip + 0xbe23a2]   # 0x14156aef0 <.text+0x1569ef0>
140988b4e: 8b b3 a0 00 00 00           	mov	esi, dword ptr [rbx + 0xa0]
140988b54: 85 f6                       	test	esi, esi
140988b56: 0f 85 2b 04 00 00           	jne	0x140988f87 <.text+0x987f87>
140988b5c: e9 2f fb ff ff              	jmp	0x140988690 <.text+0x987690>
140988b61: 48 8d 4d 30                 	lea	rcx, [rbp + 0x30]
140988b65: ba 10 00 00 00              	mov	edx, 0x10
140988b6a: 45 31 c0                    	xor	r8d, r8d
140988b6d: e8 ae c2 24 00              	call	0x140bd4e20 <.text+0xbd3e20>
140988b72: 0f 10 45 30                 	movups	xmm0, xmmword ptr [rbp + 0x30]
140988b76: 0f 29 85 20 02 00 00        	movaps	xmmword ptr [rbp + 0x220], xmm0
140988b7d: 48 8d 95 20 02 00 00        	lea	rdx, [rbp + 0x220]
140988b84: 48 8d 8d c0 03 00 00        	lea	rcx, [rbp + 0x3c0]
140988b8b: e8 70 e1 24 00              	call	0x140bd6d00 <.text+0xbd5d00>
140988b90: 48 8d 05 81 01 13 02        	lea	rax, [rip + 0x2130181]  # 0x142ab8d18
140988b97: 48 89 85 b0 03 00 00        	mov	qword ptr [rbp + 0x3b0], rax
140988b9e: 48 c7 85 b8 03 00 00 0d 00 00 00    	mov	qword ptr [rbp + 0x3b8], 0xd
140988ba9: 48 8d 95 b0 03 00 00        	lea	rdx, [rbp + 0x3b0]
140988bb0: 48 8b 8d b0 05 00 00        	mov	rcx, qword ptr [rbp + 0x5b0]
140988bb7: e8 14 09 26 00              	call	0x140be94d0 <.text+0xbe84d0>
140988bbc: 0f 10 45 30                 	movups	xmm0, xmmword ptr [rbp + 0x30]
140988bc0: 0f 29 85 60 05 00 00        	movaps	xmmword ptr [rbp + 0x560], xmm0
140988bc7: 89 85 70 05 00 00           	mov	dword ptr [rbp + 0x570], eax
140988bcd: 48 8b 85 b0 05 00 00        	mov	rax, qword ptr [rbp + 0x5b0]
140988bd4: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
140988bd9: 48 8b 85 98 05 00 00        	mov	rax, qword ptr [rbp + 0x598]
140988be0: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
140988be5: 48 c7 44 24 20 2f 00 00 00  	mov	qword ptr [rsp + 0x20], 0x2f
140988bee: 48 8d 15 30 01 13 02        	lea	rdx, [rip + 0x2130130]  # 0x142ab8d25
140988bf5: 4c 8d 0d 3e 01 13 02        	lea	r9, [rip + 0x213013e]   # 0x142ab8d3a
140988bfc: 48 8d 8d b0 03 00 00        	lea	rcx, [rbp + 0x3b0]
140988c03: 41 b8 15 00 00 00           	mov	r8d, 0x15
140988c09: e8 92 a6 c7 00              	call	0x1416032a0 <.text+0x16022a0>
140988c0e: 48 8d 8d 20 02 00 00        	lea	rcx, [rbp + 0x220]
140988c15: 48 8d 95 b0 03 00 00        	lea	rdx, [rbp + 0x3b0]
140988c1c: 49 b8 7e fc 48 4b 83 ca 89 b6       	movabs	r8, -0x4976357cb4b70382
140988c26: e8 85 48 c8 00              	call	0x14160d4b0 <.text+0x160c4b0>
140988c2b: 48 8d 8d b0 03 00 00        	lea	rcx, [rbp + 0x3b0]
140988c32: 48 8d 95 20 02 00 00        	lea	rdx, [rbp + 0x220]
140988c39: 49 b8 ef 60 20 6a f8 d5 b6 d8       	movabs	r8, -0x27492a0795df9f11
140988c43: 4c 8d 8d 60 05 00 00        	lea	r9, [rbp + 0x560]
140988c4a: e8 11 51 fe ff              	call	0x14096dd60 <.text+0x96cd60>
140988c4f: 0f b6 05 b3 bf ca 02        	movzx	eax, byte ptr [rip + 0x2cabfb3] # 0x143634c09
140988c56: b9 08 00 00 00              	mov	ecx, 0x8
140988c5b: ba 01 00 00 00              	mov	edx, 0x1
140988c60: e8 ab af c8 ff              	call	0x140613c10 <.text+0x612c10>
140988c65: 48 85 c0                    	test	rax, rax
140988c68: 0f 84 67 09 00 00           	je	0x1409895d5 <.text+0x9885d5>
140988c6e: 49 89 c6                    	mov	r14, rax
140988c71: 48 b8 0a 00 00 00 40 00 00 00       	movabs	rax, 0x400000000a
140988c7b: 49 89 06                    	mov	qword ptr [r14], rax
140988c7e: 48 8b 95 f8 03 00 00        	mov	rdx, qword ptr [rbp + 0x3f8]
140988c85: 48 85 d2                    	test	rdx, rdx
140988c88: 74 12                       	je	0x140988c9c <.text+0x987c9c>
140988c8a: 48 8b 8d 00 04 00 00        	mov	rcx, qword ptr [rbp + 0x400]
140988c91: 41 b8 01 00 00 00           	mov	r8d, 0x1
140988c97: e8 84 af c8 ff              	call	0x140613c20 <.text+0x612c20>
140988c9c: 48 c7 85 f8 03 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x3f8], 0x8
140988ca7: 4c 89 b5 00 04 00 00        	mov	qword ptr [rbp + 0x400], r14
140988cae: 48 c7 85 08 04 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x408], 0x8
140988cb9: 48 8d 4d 30                 	lea	rcx, [rbp + 0x30]
140988cbd: 48 8d 95 b0 03 00 00        	lea	rdx, [rbp + 0x3b0]
140988cc4: 41 b8 90 01 00 00           	mov	r8d, 0x190
140988cca: e8 31 af f9 01              	call	0x142923c00 <.text+0x2922c00>
140988ccf: 41 0f 10 47 40              	movups	xmm0, xmmword ptr [r15 + 0x40]
140988cd4: 0f 29 85 60 02 00 00        	movaps	xmmword ptr [rbp + 0x260], xmm0
140988cdb: 41 0f 10 07                 	movups	xmm0, xmmword ptr [r15]
140988cdf: 41 0f 10 4f 10              	movups	xmm1, xmmword ptr [r15 + 0x10]
140988ce4: 41 0f 10 57 20              	movups	xmm2, xmmword ptr [r15 + 0x20]
140988ce9: 41 0f 10 5f 30              	movups	xmm3, xmmword ptr [r15 + 0x30]
140988cee: 0f 29 9d 50 02 00 00        	movaps	xmmword ptr [rbp + 0x250], xmm3
140988cf5: 0f 29 95 40 02 00 00        	movaps	xmmword ptr [rbp + 0x240], xmm2
140988cfc: 0f 29 8d 30 02 00 00        	movaps	xmmword ptr [rbp + 0x230], xmm1
140988d03: 0f 29 85 20 02 00 00        	movaps	xmmword ptr [rbp + 0x220], xmm0
140988d0a: 48 8b 85 68 01 00 00        	mov	rax, qword ptr [rbp + 0x168]
140988d11: 48 8b 8d b0 01 00 00        	mov	rcx, qword ptr [rbp + 0x1b0]
140988d18: 48 8b 55 40                 	mov	rdx, qword ptr [rbp + 0x40]
140988d1c: 48 89 95 c0 03 00 00        	mov	qword ptr [rbp + 0x3c0], rdx
140988d23: 0f 10 45 30                 	movups	xmm0, xmmword ptr [rbp + 0x30]
140988d27: 0f 29 85 b0 03 00 00        	movaps	xmmword ptr [rbp + 0x3b0], xmm0
140988d2e: 49 8b 57 c8                 	mov	rdx, qword ptr [r15 - 0x38]
140988d32: 4c 8d 85 e0 03 00 00        	lea	r8, [rbp + 0x3e0]
140988d39: 49 89 50 10                 	mov	qword ptr [r8 + 0x10], rdx
140988d3d: 41 0f 10 47 b8              	movups	xmm0, xmmword ptr [r15 - 0x48]
140988d42: 41 0f 11 00                 	movups	xmmword ptr [r8], xmm0
140988d46: 49 8b 57 e0                 	mov	rdx, qword ptr [r15 - 0x20]
140988d4a: 49 89 50 f8                 	mov	qword ptr [r8 - 0x8], rdx
140988d4e: 41 0f 10 47 d0              	movups	xmm0, xmmword ptr [r15 - 0x30]
140988d53: 41 0f 11 40 e8              	movups	xmmword ptr [r8 - 0x18], xmm0
140988d58: 41 0f 10 87 a0 00 00 00     	movups	xmm0, xmmword ptr [r15 + 0xa0]
140988d60: 41 0f 11 80 98 00 00 00     	movups	xmmword ptr [r8 + 0x98], xmm0
140988d68: 41 0f 10 87 90 00 00 00     	movups	xmm0, xmmword ptr [r15 + 0x90]
140988d70: 41 0f 11 80 88 00 00 00     	movups	xmmword ptr [r8 + 0x88], xmm0
140988d78: 41 0f 10 47 50              	movups	xmm0, xmmword ptr [r15 + 0x50]
140988d7d: 41 0f 10 4f 60              	movups	xmm1, xmmword ptr [r15 + 0x60]
140988d82: 41 0f 10 57 70              	movups	xmm2, xmmword ptr [r15 + 0x70]
140988d87: 41 0f 10 9f 80 00 00 00     	movups	xmm3, xmmword ptr [r15 + 0x80]
140988d8f: 41 0f 11 58 78              	movups	xmmword ptr [r8 + 0x78], xmm3
140988d94: 41 0f 11 50 68              	movups	xmmword ptr [r8 + 0x68], xmm2
140988d99: 41 0f 11 48 58              	movups	xmmword ptr [r8 + 0x58], xmm1
140988d9e: 41 0f 11 40 48              	movups	xmmword ptr [r8 + 0x48], xmm0
140988da3: 49 8b 57 f8                 	mov	rdx, qword ptr [r15 - 0x8]
140988da7: 49 89 50 28                 	mov	qword ptr [r8 + 0x28], rdx
140988dab: 41 0f 10 47 e8              	movups	xmm0, xmmword ptr [r15 - 0x18]
140988db0: 41 0f 11 40 18              	movups	xmmword ptr [r8 + 0x18], xmm0
140988db5: 48 89 8d 90 04 00 00        	mov	qword ptr [rbp + 0x490], rcx
140988dbc: 48 89 85 88 04 00 00        	mov	qword ptr [rbp + 0x488], rax
140988dc3: 0f 28 05 16 02 fd 01        	movaps	xmm0, xmmword ptr [rip + 0x1fd0216] # 0x142958fe0
140988dca: 0f 29 85 10 04 00 00        	movaps	xmmword ptr [rbp + 0x410], xmm0
140988dd1: 48 8d 8d 20 02 00 00        	lea	rcx, [rbp + 0x220]
140988dd8: 48 8d 95 b0 03 00 00        	lea	rdx, [rbp + 0x3b0]
140988ddf: e8 ec 46 05 00              	call	0x1409dd4d0 <.text+0x9dc4d0>
140988de4: 80 bd 66 01 00 00 08        	cmp	byte ptr [rbp + 0x166], 0x8
140988deb: 74 0c                       	je	0x140988df9 <.text+0x987df9>
140988ded: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
140988df4: e8 b7 96 25 00              	call	0x140be24b0 <.text+0xbe14b0>
140988df9: 0f 28 85 60 05 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x560]
140988e00: 0f 29 45 30                 	movaps	xmmword ptr [rbp + 0x30], xmm0
140988e04: 48 8b 85 70 05 00 00        	mov	rax, qword ptr [rbp + 0x570]
140988e0b: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
140988e0f: 48 8b 85 80 05 00 00        	mov	rax, qword ptr [rbp + 0x580]
140988e16: 48 89 44 24 48              	mov	qword ptr [rsp + 0x48], rax
140988e1b: 8b 95 a8 05 00 00           	mov	edx, dword ptr [rbp + 0x5a8]
140988e21: 89 54 24 40                 	mov	dword ptr [rsp + 0x40], edx
140988e25: 48 8d 85 40 05 00 00        	lea	rax, [rbp + 0x540]
140988e2c: 48 89 44 24 38              	mov	qword ptr [rsp + 0x38], rax
140988e31: 48 8d 85 08 02 00 00        	lea	rax, [rbp + 0x208]
140988e38: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
140988e3d: 48 8d 85 f0 01 00 00        	lea	rax, [rbp + 0x1f0]
140988e44: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
140988e49: 48 8d 85 d0 01 00 00        	lea	rax, [rbp + 0x1d0]
140988e50: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
140988e55: 48 8d 8d b0 03 00 00        	lea	rcx, [rbp + 0x3b0]
140988e5c: 4c 8b 85 98 05 00 00        	mov	r8, qword ptr [rbp + 0x598]
140988e63: 4c 8b 8d b0 05 00 00        	mov	r9, qword ptr [rbp + 0x5b0]
140988e6a: e8 f1 fd 09 00              	call	0x140a28c60 <.text+0xa27c60>
140988e6f: 0f b6 05 93 bd ca 02        	movzx	eax, byte ptr [rip + 0x2cabd93] # 0x143634c09
140988e76: b9 08 00 00 00              	mov	ecx, 0x8
140988e7b: ba 01 00 00 00              	mov	edx, 0x1
140988e80: e8 8b ad c8 ff              	call	0x140613c10 <.text+0x612c10>
140988e85: 48 85 c0                    	test	rax, rax
140988e88: 0f 84 2c 07 00 00           	je	0x1409895ba <.text+0x9885ba>
140988e8e: 49 89 c6                    	mov	r14, rax
140988e91: 49 c1 e4 20                 	shl	r12, 0x20
140988e95: 49 83 cc 0c                 	or	r12, 0xc
140988e99: 4c 89 20                    	mov	qword ptr [rax], r12
140988e9c: 48 8b 95 f8 03 00 00        	mov	rdx, qword ptr [rbp + 0x3f8]
140988ea3: 48 85 d2                    	test	rdx, rdx
140988ea6: 74 12                       	je	0x140988eba <.text+0x987eba>
140988ea8: 48 8b 8d 00 04 00 00        	mov	rcx, qword ptr [rbp + 0x400]
140988eaf: 41 b8 01 00 00 00           	mov	r8d, 0x1
140988eb5: e8 66 ad c8 ff              	call	0x140613c20 <.text+0x612c20>
140988eba: 48 c7 85 f8 03 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x3f8], 0x8
140988ec5: 4c 89 b5 00 04 00 00        	mov	qword ptr [rbp + 0x400], r14
140988ecc: 48 c7 85 08 04 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x408], 0x8
140988ed7: 4c 8d b5 20 02 00 00        	lea	r14, [rbp + 0x220]
140988ede: 48 8d 95 b0 03 00 00        	lea	rdx, [rbp + 0x3b0]
140988ee5: 41 b8 90 01 00 00           	mov	r8d, 0x190
140988eeb: 4c 89 f1                    	mov	rcx, r14
140988eee: e8 0d ad f9 01              	call	0x142923c00 <.text+0x2922c00>
140988ef3: 48 8d 8d b0 03 00 00        	lea	rcx, [rbp + 0x3b0]
140988efa: 49 b8 b4 e4 61 20 b6 4c 0a 55       	movabs	r8, 0x550a4cb62061e4b4
140988f04: 4c 8d 4d 30                 	lea	r9, [rbp + 0x30]
140988f08: 4c 89 f2                    	mov	rdx, r14
140988f0b: e8 b0 4b fe ff              	call	0x14096dac0 <.text+0x96cac0>
140988f10: 48 8d 8d 20 02 00 00        	lea	rcx, [rbp + 0x220]
140988f17: 48 8d 95 b0 03 00 00        	lea	rdx, [rbp + 0x3b0]
140988f1e: 49 b8 ad 87 78 ac d4 56 92 6d       	movabs	r8, 0x6d9256d4ac7887ad
140988f28: e8 f3 4a c8 00              	call	0x14160da20 <.text+0x160ca20>
140988f2d: 48 8d 8d 20 02 00 00        	lea	rcx, [rbp + 0x220]
140988f34: 48 8d 55 30                 	lea	rdx, [rbp + 0x30]
140988f38: 45 31 c0                    	xor	r8d, r8d
140988f3b: e8 40 59 fe ff              	call	0x14096e880 <.text+0x96d880>
140988f40: 44 8b 85 c0 05 00 00        	mov	r8d, dword ptr [rbp + 0x5c0]
140988f47: 41 0f b6 c0                 	movzx	eax, r8b
140988f4b: 48 8d 0d 4e 04 13 02        	lea	rcx, [rip + 0x213044e]  # 0x142ab93a0
140988f52: 48 63 14 81                 	movsxd	rdx, dword ptr [rcx + 4*rax]
140988f56: 48 01 ca                    	add	rdx, rcx
140988f59: 44 89 c0                    	mov	eax, r8d
140988f5c: 44 89 c1                    	mov	ecx, r8d
140988f5f: 41 b0 02                    	mov	r8b, 0x2
140988f62: 44 89 85 c0 05 00 00        	mov	dword ptr [rbp + 0x5c0], r8d
140988f69: 4c 8b 85 a0 05 00 00        	mov	r8, qword ptr [rbp + 0x5a0]
140988f70: ff e2                       	jmp	rdx
140988f72: 4c 8d 2d e7 fe bd 00        	lea	r13, [rip + 0xbdfee7]   # 0x141568e60 <.text+0x1567e60>
140988f79: 8b b3 a0 00 00 00           	mov	esi, dword ptr [rbx + 0xa0]
140988f7f: 85 f6                       	test	esi, esi
140988f81: 0f 84 09 f7 ff ff           	je	0x140988690 <.text+0x987690>
140988f87: 48 8d 4b 68                 	lea	rcx, [rbx + 0x68]
140988f8b: 48 8b 85 b0 05 00 00        	mov	rax, qword ptr [rbp + 0x5b0]
140988f92: 48 8b 90 90 01 00 00        	mov	rdx, qword ptr [rax + 0x190]
140988f99: e8 62 bc 24 00              	call	0x140bd4c00 <.text+0xbd3c00>
140988f9e: 49 89 c6                    	mov	r14, rax
140988fa1: 8b 8d a8 05 00 00           	mov	ecx, dword ptr [rbp + 0x5a8]
140988fa7: 41 ff d5                    	call	r13
140988faa: 49 89 c0                    	mov	r8, rax
140988fad: 49 89 d1                    	mov	r9, rdx
140988fb0: 48 8b 85 80 05 00 00        	mov	rax, qword ptr [rbp + 0x580]
140988fb7: 48 89 44 24 50              	mov	qword ptr [rsp + 0x50], rax
140988fbc: 8b 85 a8 05 00 00           	mov	eax, dword ptr [rbp + 0x5a8]
140988fc2: 89 44 24 48                 	mov	dword ptr [rsp + 0x48], eax
140988fc6: 48 8d 85 40 05 00 00        	lea	rax, [rbp + 0x540]
140988fcd: 48 89 44 24 40              	mov	qword ptr [rsp + 0x40], rax
140988fd2: 48 8d 85 08 02 00 00        	lea	rax, [rbp + 0x208]
140988fd9: 48 89 44 24 38              	mov	qword ptr [rsp + 0x38], rax
140988fde: 48 8d 85 f0 01 00 00        	lea	rax, [rbp + 0x1f0]
140988fe5: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
140988fea: 48 8d 85 d0 01 00 00        	lea	rax, [rbp + 0x1d0]
140988ff1: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
140988ff6: 48 8b 85 b0 05 00 00        	mov	rax, qword ptr [rbp + 0x5b0]
140988ffd: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
140989002: 48 8d 8d b0 03 00 00        	lea	rcx, [rbp + 0x3b0]
140989009: 4c 89 85 88 05 00 00        	mov	qword ptr [rbp + 0x588], r8
140989010: 4c 89 c2                    	mov	rdx, r8
140989013: 4c 89 8d 90 05 00 00        	mov	qword ptr [rbp + 0x590], r9
14098901a: 4d 89 c8                    	mov	r8, r9
14098901d: 4c 8b 8d 98 05 00 00        	mov	r9, qword ptr [rbp + 0x598]
140989024: e8 07 f6 09 00              	call	0x140a28630 <.text+0xa27630>
140989029: 0f b6 05 d9 bb ca 02        	movzx	eax, byte ptr [rip + 0x2cabbd9] # 0x143634c09
140989030: b9 14 00 00 00              	mov	ecx, 0x14
140989035: ba 01 00 00 00              	mov	edx, 0x1
14098903a: e8 d1 ab c8 ff              	call	0x140613c10 <.text+0x612c10>
14098903f: 48 85 c0                    	test	rax, rax
140989042: 0f 84 ef 05 00 00           	je	0x140989637 <.text+0x988637>
140989048: 49 89 c5                    	mov	r13, rax
14098904b: 4c 89 30                    	mov	qword ptr [rax], r14
14098904e: 89 78 08                    	mov	dword ptr [rax + 0x8], edi
140989051: 44 89 60 0c                 	mov	dword ptr [rax + 0xc], r12d
140989055: 89 70 10                    	mov	dword ptr [rax + 0x10], esi
140989058: 48 8b 95 f8 03 00 00        	mov	rdx, qword ptr [rbp + 0x3f8]
14098905f: 48 85 d2                    	test	rdx, rdx
140989062: 74 12                       	je	0x140989076 <.text+0x988076>
140989064: 48 8b 8d 00 04 00 00        	mov	rcx, qword ptr [rbp + 0x400]
14098906b: 41 b8 01 00 00 00           	mov	r8d, 0x1
140989071: e8 aa ab c8 ff              	call	0x140613c20 <.text+0x612c20>
140989076: 48 c7 85 f8 03 00 00 14 00 00 00    	mov	qword ptr [rbp + 0x3f8], 0x14
140989081: 4c 89 ad 00 04 00 00        	mov	qword ptr [rbp + 0x400], r13
140989088: 48 c7 85 08 04 00 00 14 00 00 00    	mov	qword ptr [rbp + 0x408], 0x14
140989093: 48 8d 8d 20 02 00 00        	lea	rcx, [rbp + 0x220]
14098909a: 48 8d 95 b0 03 00 00        	lea	rdx, [rbp + 0x3b0]
1409890a1: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409890a7: e8 54 ab f9 01              	call	0x142923c00 <.text+0x2922c00>
1409890ac: 48 8b 8b 90 00 00 00        	mov	rcx, qword ptr [rbx + 0x90]
1409890b3: 48 8b 83 98 00 00 00        	mov	rax, qword ptr [rbx + 0x98]
1409890ba: 48 8b 40 38                 	mov	rax, qword ptr [rax + 0x38]
1409890be: c6 85 be 05 00 00 01        	mov	byte ptr [rbp + 0x5be], 0x1
1409890c5: ff d0                       	call	rax
1409890c7: c6 85 be 05 00 00 00        	mov	byte ptr [rbp + 0x5be], 0x0
1409890ce: 48 8d 4d 30                 	lea	rcx, [rbp + 0x30]
1409890d2: 48 8d 95 20 02 00 00        	lea	rdx, [rbp + 0x220]
1409890d9: 49 b8 ad 87 78 ac d4 56 92 6d       	movabs	r8, 0x6d9256d4ac7887ad
1409890e3: 49 89 c1                    	mov	r9, rax
1409890e6: e8 d5 49 fe ff              	call	0x14096dac0 <.text+0x96cac0>
1409890eb: 41 0f 10 47 40              	movups	xmm0, xmmword ptr [r15 + 0x40]
1409890f0: 0f 29 85 60 02 00 00        	movaps	xmmword ptr [rbp + 0x260], xmm0
1409890f7: 41 0f 10 07                 	movups	xmm0, xmmword ptr [r15]
1409890fb: 41 0f 10 4f 10              	movups	xmm1, xmmword ptr [r15 + 0x10]
140989100: 41 0f 10 57 20              	movups	xmm2, xmmword ptr [r15 + 0x20]
140989105: 41 0f 10 5f 30              	movups	xmm3, xmmword ptr [r15 + 0x30]
14098910a: 0f 29 9d 50 02 00 00        	movaps	xmmword ptr [rbp + 0x250], xmm3
140989111: 0f 29 95 40 02 00 00        	movaps	xmmword ptr [rbp + 0x240], xmm2
140989118: 0f 29 8d 30 02 00 00        	movaps	xmmword ptr [rbp + 0x230], xmm1
14098911f: 0f 29 85 20 02 00 00        	movaps	xmmword ptr [rbp + 0x220], xmm0
140989126: 48 8b 85 68 01 00 00        	mov	rax, qword ptr [rbp + 0x168]
14098912d: 48 8b 8d b0 01 00 00        	mov	rcx, qword ptr [rbp + 0x1b0]
140989134: 48 8b 55 40                 	mov	rdx, qword ptr [rbp + 0x40]
140989138: 48 89 95 c0 03 00 00        	mov	qword ptr [rbp + 0x3c0], rdx
14098913f: 0f 10 45 30                 	movups	xmm0, xmmword ptr [rbp + 0x30]
140989143: 0f 29 85 b0 03 00 00        	movaps	xmmword ptr [rbp + 0x3b0], xmm0
14098914a: 49 8b 57 c8                 	mov	rdx, qword ptr [r15 - 0x38]
14098914e: 4c 8d 85 e0 03 00 00        	lea	r8, [rbp + 0x3e0]
140989155: 49 89 50 10                 	mov	qword ptr [r8 + 0x10], rdx
140989159: 41 0f 10 47 b8              	movups	xmm0, xmmword ptr [r15 - 0x48]
14098915e: 41 0f 11 00                 	movups	xmmword ptr [r8], xmm0
140989162: 49 8b 57 e0                 	mov	rdx, qword ptr [r15 - 0x20]
140989166: 49 89 50 f8                 	mov	qword ptr [r8 - 0x8], rdx
14098916a: 41 0f 10 47 d0              	movups	xmm0, xmmword ptr [r15 - 0x30]
14098916f: 41 0f 11 40 e8              	movups	xmmword ptr [r8 - 0x18], xmm0
140989174: 41 0f 10 87 a0 00 00 00     	movups	xmm0, xmmword ptr [r15 + 0xa0]
14098917c: 41 0f 11 80 98 00 00 00     	movups	xmmword ptr [r8 + 0x98], xmm0
140989184: 41 0f 10 87 90 00 00 00     	movups	xmm0, xmmword ptr [r15 + 0x90]
14098918c: 41 0f 11 80 88 00 00 00     	movups	xmmword ptr [r8 + 0x88], xmm0
140989194: 41 0f 10 47 50              	movups	xmm0, xmmword ptr [r15 + 0x50]
140989199: 41 0f 10 4f 60              	movups	xmm1, xmmword ptr [r15 + 0x60]
14098919e: 41 0f 10 57 70              	movups	xmm2, xmmword ptr [r15 + 0x70]
1409891a3: 41 0f 10 9f 80 00 00 00     	movups	xmm3, xmmword ptr [r15 + 0x80]
1409891ab: 41 0f 11 58 78              	movups	xmmword ptr [r8 + 0x78], xmm3
1409891b0: 41 0f 11 50 68              	movups	xmmword ptr [r8 + 0x68], xmm2
1409891b5: 41 0f 11 48 58              	movups	xmmword ptr [r8 + 0x58], xmm1
1409891ba: 41 0f 11 40 48              	movups	xmmword ptr [r8 + 0x48], xmm0
1409891bf: 49 8b 57 f8                 	mov	rdx, qword ptr [r15 - 0x8]
1409891c3: 49 89 50 28                 	mov	qword ptr [r8 + 0x28], rdx
1409891c7: 41 0f 10 47 e8              	movups	xmm0, xmmword ptr [r15 - 0x18]
1409891cc: 41 0f 11 40 18              	movups	xmmword ptr [r8 + 0x18], xmm0
1409891d1: 48 89 8d 90 04 00 00        	mov	qword ptr [rbp + 0x490], rcx
1409891d8: 48 89 85 88 04 00 00        	mov	qword ptr [rbp + 0x488], rax
1409891df: c7 85 10 04 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x410], 0x1
1409891e9: 89 b5 14 04 00 00           	mov	dword ptr [rbp + 0x414], esi
1409891ef: 48 b8 01 00 00 00 01 00 00 00       	movabs	rax, 0x100000001
1409891f9: 48 89 85 18 04 00 00        	mov	qword ptr [rbp + 0x418], rax
140989200: 48 8d 8d 20 02 00 00        	lea	rcx, [rbp + 0x220]
140989207: 48 8d 95 b0 03 00 00        	lea	rdx, [rbp + 0x3b0]
14098920e: e8 bd 42 05 00              	call	0x1409dd4d0 <.text+0x9dc4d0>
140989213: 80 bd 66 01 00 00 08        	cmp	byte ptr [rbp + 0x166], 0x8
14098921a: 74 0c                       	je	0x140989228 <.text+0x988228>
14098921c: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
140989223: e8 88 92 25 00              	call	0x140be24b0 <.text+0xbe14b0>
140989228: 48 8b 85 90 05 00 00        	mov	rax, qword ptr [rbp + 0x590]
14098922f: 48 8b 00                    	mov	rax, qword ptr [rax]
140989232: 48 85 c0                    	test	rax, rax
140989235: 74 09                       	je	0x140989240 <.text+0x988240>
140989237: 48 8b 8d 88 05 00 00        	mov	rcx, qword ptr [rbp + 0x588]
14098923e: ff d0                       	call	rax
140989240: 48 8b 85 90 05 00 00        	mov	rax, qword ptr [rbp + 0x590]
140989247: 48 8b 50 08                 	mov	rdx, qword ptr [rax + 0x8]
14098924b: 48 85 d2                    	test	rdx, rdx
14098924e: 0f 84 3c f4 ff ff           	je	0x140988690 <.text+0x987690>
140989254: 4c 8b 40 10                 	mov	r8, qword ptr [rax + 0x10]
140989258: 48 8b 8d 88 05 00 00        	mov	rcx, qword ptr [rbp + 0x588]
14098925f: e8 bc a9 c8 ff              	call	0x140613c20 <.text+0x612c20>
140989264: 0f b6 85 c0 05 00 00        	movzx	eax, byte ptr [rbp + 0x5c0]
14098926b: 48 8d 0d 2e 01 13 02        	lea	rcx, [rip + 0x213012e]  # 0x142ab93a0
140989272: 48 63 14 81                 	movsxd	rdx, dword ptr [rcx + 4*rax]
140989276: 48 01 ca                    	add	rdx, rcx
140989279: 89 c1                       	mov	ecx, eax
14098927b: 41 b0 02                    	mov	r8b, 0x2
14098927e: 44 89 85 c0 05 00 00        	mov	dword ptr [rbp + 0x5c0], r8d
140989285: 4c 8b 85 a0 05 00 00        	mov	r8, qword ptr [rbp + 0x5a0]
14098928c: ff e2                       	jmp	rdx
14098928e: 4c 8d 2d 3b f6 bd 00        	lea	r13, [rip + 0xbdf63b]   # 0x1415688d0 <.text+0x15678d0>
140989295: 8b b3 a0 00 00 00           	mov	esi, dword ptr [rbx + 0xa0]
14098929b: 85 f6                       	test	esi, esi
14098929d: 0f 85 e4 fc ff ff           	jne	0x140988f87 <.text+0x987f87>
1409892a3: e9 e8 f3 ff ff              	jmp	0x140988690 <.text+0x987690>
1409892a8: 8b 8d a8 05 00 00           	mov	ecx, dword ptr [rbp + 0x5a8]
1409892ae: 89 4c 24 48                 	mov	dword ptr [rsp + 0x48], ecx
1409892b2: 48 8b 85 80 05 00 00        	mov	rax, qword ptr [rbp + 0x580]
1409892b9: 48 89 44 24 38              	mov	qword ptr [rsp + 0x38], rax
1409892be: 89 4c 24 30                 	mov	dword ptr [rsp + 0x30], ecx
1409892c2: 48 8d 85 40 05 00 00        	lea	rax, [rbp + 0x540]
1409892c9: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
1409892ce: 48 8d 85 08 02 00 00        	lea	rax, [rbp + 0x208]
1409892d5: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1409892da: c6 44 24 40 08              	mov	byte ptr [rsp + 0x40], 0x8
1409892df: 4c 8d 85 f0 01 00 00        	lea	r8, [rbp + 0x1f0]
1409892e6: 4c 8d 8d d0 01 00 00        	lea	r9, [rbp + 0x1d0]
1409892ed: 48 8b 8d b0 05 00 00        	mov	rcx, qword ptr [rbp + 0x5b0]
1409892f4: 48 8b 95 98 05 00 00        	mov	rdx, qword ptr [rbp + 0x598]
1409892fb: e8 a0 ed 09 00              	call	0x140a280a0 <.text+0xa270a0>
140989300: e9 8b f3 ff ff              	jmp	0x140988690 <.text+0x987690>
140989305: b1 13                       	mov	cl, 0x13
140989307: e9 c4 f3 ff ff              	jmp	0x1409886d0 <.text+0x9876d0>
14098930c: b1 11                       	mov	cl, 0x11
14098930e: e9 bd f3 ff ff              	jmp	0x1409886d0 <.text+0x9876d0>
140989313: b1 0e                       	mov	cl, 0xe
140989315: e9 b6 f3 ff ff              	jmp	0x1409886d0 <.text+0x9876d0>
14098931a: b1 07                       	mov	cl, 0x7
14098931c: e9 af f3 ff ff              	jmp	0x1409886d0 <.text+0x9876d0>
140989321: b1 0f                       	mov	cl, 0xf
140989323: e9 a8 f3 ff ff              	jmp	0x1409886d0 <.text+0x9876d0>
140989328: b1 0c                       	mov	cl, 0xc
14098932a: e9 a1 f3 ff ff              	jmp	0x1409886d0 <.text+0x9876d0>
14098932f: b1 1b                       	mov	cl, 0x1b
140989331: e9 9a f3 ff ff              	jmp	0x1409886d0 <.text+0x9876d0>
140989336: b1 05                       	mov	cl, 0x5
140989338: e9 93 f3 ff ff              	jmp	0x1409886d0 <.text+0x9876d0>
14098933d: b1 12                       	mov	cl, 0x12
14098933f: e9 8c f3 ff ff              	jmp	0x1409886d0 <.text+0x9876d0>
140989344: b1 06                       	mov	cl, 0x6
140989346: e9 85 f3 ff ff              	jmp	0x1409886d0 <.text+0x9876d0>
14098934b: b1 0a                       	mov	cl, 0xa
14098934d: e9 7e f3 ff ff              	jmp	0x1409886d0 <.text+0x9876d0>
140989352: b1 03                       	mov	cl, 0x3
140989354: e9 77 f3 ff ff              	jmp	0x1409886d0 <.text+0x9876d0>
140989359: b1 04                       	mov	cl, 0x4
14098935b: e9 70 f3 ff ff              	jmp	0x1409886d0 <.text+0x9876d0>
140989360: b1 14                       	mov	cl, 0x14
140989362: e9 69 f3 ff ff              	jmp	0x1409886d0 <.text+0x9876d0>
140989367: b1 17                       	mov	cl, 0x17
140989369: e9 62 f3 ff ff              	jmp	0x1409886d0 <.text+0x9876d0>
14098936e: b1 08                       	mov	cl, 0x8
140989370: e9 5b f3 ff ff              	jmp	0x1409886d0 <.text+0x9876d0>
140989375: b1 18                       	mov	cl, 0x18
140989377: e9 54 f3 ff ff              	jmp	0x1409886d0 <.text+0x9876d0>
14098937c: b1 10                       	mov	cl, 0x10
14098937e: e9 4d f3 ff ff              	jmp	0x1409886d0 <.text+0x9876d0>
140989383: b1 09                       	mov	cl, 0x9
140989385: e9 46 f3 ff ff              	jmp	0x1409886d0 <.text+0x9876d0>
14098938a: b1 15                       	mov	cl, 0x15
14098938c: e9 3f f3 ff ff              	jmp	0x1409886d0 <.text+0x9876d0>
140989391: b1 0d                       	mov	cl, 0xd
140989393: e9 38 f3 ff ff              	jmp	0x1409886d0 <.text+0x9876d0>
140989398: b1 0b                       	mov	cl, 0xb
14098939a: e9 31 f3 ff ff              	jmp	0x1409886d0 <.text+0x9876d0>
14098939f: b1 19                       	mov	cl, 0x19
1409893a1: e9 2a f3 ff ff              	jmp	0x1409886d0 <.text+0x9876d0>
1409893a6: b1 16                       	mov	cl, 0x16
1409893a8: e9 23 f3 ff ff              	jmp	0x1409886d0 <.text+0x9876d0>
1409893ad: b1 1a                       	mov	cl, 0x1a
1409893af: e9 1c f3 ff ff              	jmp	0x1409886d0 <.text+0x9876d0>
1409893b4: 48 8d 8d 40 05 00 00        	lea	rcx, [rbp + 0x540]
1409893bb: e8 f0 90 25 00              	call	0x140be24b0 <.text+0xbe14b0>
1409893c0: 48 8b b5 b0 05 00 00        	mov	rsi, qword ptr [rbp + 0x5b0]
1409893c7: 48 8d 8d 08 02 00 00        	lea	rcx, [rbp + 0x208]
1409893ce: e8 dd 90 25 00              	call	0x140be24b0 <.text+0xbe14b0>
1409893d3: 48 89 74 24 30              	mov	qword ptr [rsp + 0x30], rsi
1409893d8: 4c 8b ad 98 05 00 00        	mov	r13, qword ptr [rbp + 0x598]
1409893df: 4c 89 6c 24 28              	mov	qword ptr [rsp + 0x28], r13
1409893e4: 48 c7 44 24 20 22 00 00 00  	mov	qword ptr [rsp + 0x20], 0x22
1409893ed: 48 8d 15 75 f9 12 02        	lea	rdx, [rip + 0x212f975]  # 0x142ab8d69
1409893f4: 4c 8d 0d 83 f9 12 02        	lea	r9, [rip + 0x212f983]   # 0x142ab8d7e
1409893fb: 4c 8d bd b0 03 00 00        	lea	r15, [rbp + 0x3b0]
140989402: 41 b8 15 00 00 00           	mov	r8d, 0x15
140989408: 4c 89 f9                    	mov	rcx, r15
14098940b: e8 90 9e c7 00              	call	0x1416032a0 <.text+0x16022a0>
140989410: 48 8d 8d 20 02 00 00        	lea	rcx, [rbp + 0x220]
140989417: 49 b8 48 72 ad 18 d3 61 b8 aa       	movabs	r8, -0x55479e2ce7528db8
140989421: 4c 8d b5 f0 01 00 00        	lea	r14, [rbp + 0x1f0]
140989428: 4c 89 fa                    	mov	rdx, r15
14098942b: 4d 89 f1                    	mov	r9, r14
14098942e: e8 2d 49 fe ff              	call	0x14096dd60 <.text+0x96cd60>
140989433: 0f 10 85 c0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2c0]
14098943a: 0f 29 45 70                 	movaps	xmmword ptr [rbp + 0x70], xmm0
14098943e: 0f 10 85 b0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2b0]
140989445: 0f 29 45 60                 	movaps	xmmword ptr [rbp + 0x60], xmm0
140989449: 0f 10 85 a0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2a0]
140989450: 0f 29 45 50                 	movaps	xmmword ptr [rbp + 0x50], xmm0
140989454: 0f 10 85 90 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x290]
14098945b: 0f 29 45 40                 	movaps	xmmword ptr [rbp + 0x40], xmm0
14098945f: 0f 10 85 80 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x280]
140989466: 0f 29 45 30                 	movaps	xmmword ptr [rbp + 0x30], xmm0
14098946a: 48 8b 85 58 03 00 00        	mov	rax, qword ptr [rbp + 0x358]
140989471: 48 8b 8d a0 03 00 00        	mov	rcx, qword ptr [rbp + 0x3a0]
140989478: 48 8b 95 30 02 00 00        	mov	rdx, qword ptr [rbp + 0x230]
14098947f: 48 89 95 c0 03 00 00        	mov	qword ptr [rbp + 0x3c0], rdx
140989486: 0f 10 85 20 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x220]
14098948d: 0f 29 85 b0 03 00 00        	movaps	xmmword ptr [rbp + 0x3b0], xmm0
140989494: 0f 10 85 38 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x238]
14098949b: 0f 29 85 e0 03 00 00        	movaps	xmmword ptr [rbp + 0x3e0], xmm0
1409894a2: 48 8b 95 48 02 00 00        	mov	rdx, qword ptr [rbp + 0x248]
1409894a9: 48 89 95 f0 03 00 00        	mov	qword ptr [rbp + 0x3f0], rdx
1409894b0: 0f 10 85 50 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x250]
1409894b7: 0f 11 85 c8 03 00 00        	movups	xmmword ptr [rbp + 0x3c8], xmm0
1409894be: 48 8b 95 60 02 00 00        	mov	rdx, qword ptr [rbp + 0x260]
1409894c5: 48 89 95 d8 03 00 00        	mov	qword ptr [rbp + 0x3d8], rdx
1409894cc: 0f 10 85 d0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2d0]
1409894d3: 0f 10 8d e0 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x2e0]
1409894da: 0f 10 95 f0 02 00 00        	movups	xmm2, xmmword ptr [rbp + 0x2f0]
1409894e1: 0f 10 9d 00 03 00 00        	movups	xmm3, xmmword ptr [rbp + 0x300]
1409894e8: 0f 11 85 28 04 00 00        	movups	xmmword ptr [rbp + 0x428], xmm0
1409894ef: 0f 11 8d 38 04 00 00        	movups	xmmword ptr [rbp + 0x438], xmm1
1409894f6: 0f 11 95 48 04 00 00        	movups	xmmword ptr [rbp + 0x448], xmm2
1409894fd: 0f 11 9d 58 04 00 00        	movups	xmmword ptr [rbp + 0x458], xmm3
140989504: 0f 10 85 10 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x310]
14098950b: 0f 11 85 68 04 00 00        	movups	xmmword ptr [rbp + 0x468], xmm0
140989512: 0f 10 85 20 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x320]
140989519: 0f 11 85 78 04 00 00        	movups	xmmword ptr [rbp + 0x478], xmm0
140989520: 48 8b 95 78 02 00 00        	mov	rdx, qword ptr [rbp + 0x278]
140989527: 48 89 95 08 04 00 00        	mov	qword ptr [rbp + 0x408], rdx
14098952e: 0f 10 85 68 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x268]
140989535: 0f 11 85 f8 03 00 00        	movups	xmmword ptr [rbp + 0x3f8], xmm0
14098953c: 48 89 8d 90 04 00 00        	mov	qword ptr [rbp + 0x490], rcx
140989543: 48 89 85 88 04 00 00        	mov	qword ptr [rbp + 0x488], rax
14098954a: 0f 28 05 9f ee 12 02        	movaps	xmm0, xmmword ptr [rip + 0x212ee9f] # 0x142ab83f0
140989551: 0f 29 85 10 04 00 00        	movaps	xmmword ptr [rbp + 0x410], xmm0
140989558: 48 8d 4d 30                 	lea	rcx, [rbp + 0x30]
14098955c: 48 8d 95 b0 03 00 00        	lea	rdx, [rbp + 0x3b0]
140989563: e8 68 3f 05 00              	call	0x1409dd4d0 <.text+0x9dc4d0>
140989568: 80 bd 56 03 00 00 08        	cmp	byte ptr [rbp + 0x356], 0x8
14098956f: 74 0c                       	je	0x14098957d <.text+0x98857d>
140989571: 48 8d 8d 40 03 00 00        	lea	rcx, [rbp + 0x340]
140989578: e8 33 8f 25 00              	call	0x140be24b0 <.text+0xbe14b0>
14098957d: 31 d2                       	xor	edx, edx
14098957f: b1 02                       	mov	cl, 0x2
140989581: 44 0f b6 c2                 	movzx	r8d, dl
140989585: 48 8d 05 5c ff 12 02        	lea	rax, [rip + 0x212ff5c]  # 0x142ab94e8
14098958c: 4e 63 04 80                 	movsxd	r8, dword ptr [rax + 4*r8]
140989590: 49 01 c0                    	add	r8, rax
140989593: 48 8b b5 b0 05 00 00        	mov	rsi, qword ptr [rbp + 0x5b0]
14098959a: 41 ff e0                    	jmp	r8
14098959d: 48 8d 0d 16 f7 12 02        	lea	rcx, [rip + 0x212f716]  # 0x142ab8cba
1409895a4: 4c 8d 05 55 f7 12 02        	lea	r8, [rip + 0x212f755]   # 0x142ab8d00
1409895ab: ba 28 00 00 00              	mov	edx, 0x28
1409895b0: e8 7b fe f4 01              	call	0x1428d9430 <.text+0x28d8430>
1409895b5: e9 ad 00 00 00              	jmp	0x140989667 <.text+0x988667>
1409895ba: 4c 8d 05 0f 43 2f 02        	lea	r8, [rip + 0x22f430f]   # 0x142c7d8d0
1409895c1: b9 01 00 00 00              	mov	ecx, 0x1
1409895c6: ba 08 00 00 00              	mov	edx, 0x8
1409895cb: e8 13 fa f4 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
1409895d0: e9 92 00 00 00              	jmp	0x140989667 <.text+0x988667>
1409895d5: 4c 8d 05 f4 42 2f 02        	lea	r8, [rip + 0x22f42f4]   # 0x142c7d8d0
1409895dc: b9 01 00 00 00              	mov	ecx, 0x1
1409895e1: ba 08 00 00 00              	mov	edx, 0x8
1409895e6: e8 f8 f9 f4 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
1409895eb: eb 7a                       	jmp	0x140989667 <.text+0x988667>
1409895ed: 4c 8d 05 dc 42 2f 02        	lea	r8, [rip + 0x22f42dc]   # 0x142c7d8d0
1409895f4: b9 01 00 00 00              	mov	ecx, 0x1
1409895f9: ba 08 00 00 00              	mov	edx, 0x8
1409895fe: e8 e0 f9 f4 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
140989603: eb 62                       	jmp	0x140989667 <.text+0x988667>
140989605: 4c 8d 05 c4 42 2f 02        	lea	r8, [rip + 0x22f42c4]   # 0x142c7d8d0
14098960c: b9 01 00 00 00              	mov	ecx, 0x1
140989611: ba 08 00 00 00              	mov	edx, 0x8
140989616: e8 c8 f9 f4 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
14098961b: eb 4a                       	jmp	0x140989667 <.text+0x988667>
14098961d: 48 8d 0d 96 f6 12 02        	lea	rcx, [rip + 0x212f696]  # 0x142ab8cba
140989624: 4c 8d 05 bd f6 12 02        	lea	r8, [rip + 0x212f6bd]   # 0x142ab8ce8
14098962b: ba 28 00 00 00              	mov	edx, 0x28
140989630: e8 fb fd f4 01              	call	0x1428d9430 <.text+0x28d8430>
140989635: eb 30                       	jmp	0x140989667 <.text+0x988667>
140989637: 4c 8d 05 92 42 2f 02        	lea	r8, [rip + 0x22f4292]   # 0x142c7d8d0
14098963e: b9 01 00 00 00              	mov	ecx, 0x1
140989643: ba 14 00 00 00              	mov	edx, 0x14
140989648: e8 96 f9 f4 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
14098964d: eb 18                       	jmp	0x140989667 <.text+0x988667>
14098964f: 48 8d 0d 6a f5 12 02        	lea	rcx, [rip + 0x212f56a]  # 0x142ab8bc0
140989656: e8 b5 fc f4 01              	call	0x1428d9310 <.text+0x28d8310>
14098965b: 48 8d 0d 66 1b 12 02        	lea	rcx, [rip + 0x2121b66]  # 0x142aab1c8
140989662: e8 a9 fc f4 01              	call	0x1428d9310 <.text+0x28d8310>
140989667: 0f 0b                       	ud2
140989669: b1 03                       	mov	cl, 0x3
14098966b: eb 13                       	jmp	0x140989680 <.text+0x988680>
14098966d: b1 04                       	mov	cl, 0x4
14098966f: eb 0f                       	jmp	0x140989680 <.text+0x988680>
140989671: b1 01                       	mov	cl, 0x1
140989673: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140989680: 48 81 c2 c0 06 00 00        	add	rdx, 0x6c0
140989687: 0f b6 c9                    	movzx	ecx, cl
14098968a: 4c 63 04 88                 	movsxd	r8, dword ptr [rax + 4*rcx]
14098968e: 49 01 c0                    	add	r8, rax
140989691: b1 02                       	mov	cl, 0x2
140989693: 41 ff e0                    	jmp	r8
140989696: 48 8d 4d 30                 	lea	rcx, [rbp + 0x30]
14098969a: 45 31 c0                    	xor	r8d, r8d
14098969d: e8 7e b7 24 00              	call	0x140bd4e20 <.text+0xbd3e20>
1409896a2: 0f 10 45 30                 	movups	xmm0, xmmword ptr [rbp + 0x30]
1409896a6: 0f 29 85 20 02 00 00        	movaps	xmmword ptr [rbp + 0x220], xmm0
1409896ad: 48 8d 8d c0 03 00 00        	lea	rcx, [rbp + 0x3c0]
1409896b4: 48 8d 95 20 02 00 00        	lea	rdx, [rbp + 0x220]
1409896bb: e8 40 d6 24 00              	call	0x140bd6d00 <.text+0xbd5d00>
1409896c0: 48 8d 05 d9 f6 12 02        	lea	rax, [rip + 0x212f6d9]  # 0x142ab8da0
1409896c7: 48 89 85 b0 03 00 00        	mov	qword ptr [rbp + 0x3b0], rax
1409896ce: 48 c7 85 b8 03 00 00 12 00 00 00    	mov	qword ptr [rbp + 0x3b8], 0x12
1409896d9: 48 8d 95 b0 03 00 00        	lea	rdx, [rbp + 0x3b0]
1409896e0: 48 89 f1                    	mov	rcx, rsi
1409896e3: e8 e8 fd 25 00              	call	0x140be94d0 <.text+0xbe84d0>
1409896e8: 0f 10 45 30                 	movups	xmm0, xmmword ptr [rbp + 0x30]
1409896ec: 0f 29 85 40 05 00 00        	movaps	xmmword ptr [rbp + 0x540], xmm0
1409896f3: 89 85 50 05 00 00           	mov	dword ptr [rbp + 0x550], eax
1409896f9: 48 8d 4d 30                 	lea	rcx, [rbp + 0x30]
1409896fd: ba b0 01 00 00              	mov	edx, 0x1b0
140989702: 45 31 c0                    	xor	r8d, r8d
140989705: e8 16 b7 24 00              	call	0x140bd4e20 <.text+0xbd3e20>
14098970a: 0f 10 45 30                 	movups	xmm0, xmmword ptr [rbp + 0x30]
14098970e: 0f 29 85 20 02 00 00        	movaps	xmmword ptr [rbp + 0x220], xmm0
140989715: 48 8d 8d c0 03 00 00        	lea	rcx, [rbp + 0x3c0]
14098971c: 48 8d 95 20 02 00 00        	lea	rdx, [rbp + 0x220]
140989723: e8 d8 d5 24 00              	call	0x140bd6d00 <.text+0xbd5d00>
140989728: 48 8d 05 83 f6 12 02        	lea	rax, [rip + 0x212f683]  # 0x142ab8db2
14098972f: 48 89 85 b0 03 00 00        	mov	qword ptr [rbp + 0x3b0], rax
140989736: 48 c7 85 b8 03 00 00 1a 00 00 00    	mov	qword ptr [rbp + 0x3b8], 0x1a
140989741: 48 8d 95 b0 03 00 00        	lea	rdx, [rbp + 0x3b0]
140989748: 48 89 f1                    	mov	rcx, rsi
14098974b: e8 80 fd 25 00              	call	0x140be94d0 <.text+0xbe84d0>
140989750: 0f 10 45 30                 	movups	xmm0, xmmword ptr [rbp + 0x30]
140989754: 0f 29 85 60 05 00 00        	movaps	xmmword ptr [rbp + 0x560], xmm0
14098975b: 89 85 70 05 00 00           	mov	dword ptr [rbp + 0x570], eax
140989761: 48 89 74 24 30              	mov	qword ptr [rsp + 0x30], rsi
140989766: 4c 89 6c 24 28              	mov	qword ptr [rsp + 0x28], r13
14098976b: 48 c7 44 24 20 24 00 00 00  	mov	qword ptr [rsp + 0x20], 0x24
140989774: 48 8d 15 51 f6 12 02        	lea	rdx, [rip + 0x212f651]  # 0x142ab8dcc
14098977b: 4c 8d 0d 65 f6 12 02        	lea	r9, [rip + 0x212f665]   # 0x142ab8de7
140989782: 4c 8d bd b0 03 00 00        	lea	r15, [rbp + 0x3b0]
140989789: 41 b8 1b 00 00 00           	mov	r8d, 0x1b
14098978f: 4c 89 f9                    	mov	rcx, r15
140989792: e8 09 9b c7 00              	call	0x1416032a0 <.text+0x16022a0>
140989797: 4c 8d a5 20 02 00 00        	lea	r12, [rbp + 0x220]
14098979e: 49 b8 e3 c5 90 19 c5 9d fe 86       	movabs	r8, -0x7901623ae66f3a1d
1409897a8: 4c 89 e1                    	mov	rcx, r12
1409897ab: 4c 89 fa                    	mov	rdx, r15
1409897ae: e8 5d 39 c8 00              	call	0x14160d110 <.text+0x160c110>
1409897b3: 49 b8 4d f3 05 c9 b1 38 6c 68       	movabs	r8, 0x686c38b1c905f34d
1409897bd: 4c 89 f9                    	mov	rcx, r15
1409897c0: 4c 89 e2                    	mov	rdx, r12
1409897c3: e8 18 3b c8 00              	call	0x14160d2e0 <.text+0x160c2e0>
1409897c8: 49 b8 b4 e4 61 20 b6 4c 0a 55       	movabs	r8, 0x550a4cb62061e4b4
1409897d2: 4c 8d 8d 40 05 00 00        	lea	r9, [rbp + 0x540]
1409897d9: 4c 89 e1                    	mov	rcx, r12
1409897dc: 4c 89 fa                    	mov	rdx, r15
1409897df: e8 7c 45 fe ff              	call	0x14096dd60 <.text+0x96cd60>
1409897e4: 48 8d 8d b0 03 00 00        	lea	rcx, [rbp + 0x3b0]
1409897eb: 49 b8 84 ac 8a 23 de bb 37 53       	movabs	r8, 0x5337bbde238aac84
1409897f5: 4c 8d 8d 60 05 00 00        	lea	r9, [rbp + 0x560]
1409897fc: 4c 89 e2                    	mov	rdx, r12
1409897ff: e8 5c 45 fe ff              	call	0x14096dd60 <.text+0x96cd60>
140989804: 0f 10 85 50 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x450]
14098980b: 0f 29 45 70                 	movaps	xmmword ptr [rbp + 0x70], xmm0
14098980f: 0f 10 85 40 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x440]
140989816: 0f 29 45 60                 	movaps	xmmword ptr [rbp + 0x60], xmm0
14098981a: 0f 10 85 30 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x430]
140989821: 0f 29 45 50                 	movaps	xmmword ptr [rbp + 0x50], xmm0
140989825: 0f 10 85 20 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x420]
14098982c: 0f 29 45 40                 	movaps	xmmword ptr [rbp + 0x40], xmm0
140989830: 0f 10 85 10 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x410]
140989837: 0f 29 45 30                 	movaps	xmmword ptr [rbp + 0x30], xmm0
14098983b: 48 8b 85 e8 04 00 00        	mov	rax, qword ptr [rbp + 0x4e8]
140989842: 48 8b 8d 30 05 00 00        	mov	rcx, qword ptr [rbp + 0x530]
140989849: 48 8b 95 c0 03 00 00        	mov	rdx, qword ptr [rbp + 0x3c0]
140989850: 48 89 95 30 02 00 00        	mov	qword ptr [rbp + 0x230], rdx
140989857: 0f 10 85 b0 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x3b0]
14098985e: 0f 29 85 20 02 00 00        	movaps	xmmword ptr [rbp + 0x220], xmm0
140989865: 0f 10 85 c8 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x3c8]
14098986c: 0f 29 85 50 02 00 00        	movaps	xmmword ptr [rbp + 0x250], xmm0
140989873: 48 8b 95 d8 03 00 00        	mov	rdx, qword ptr [rbp + 0x3d8]
14098987a: 48 89 95 60 02 00 00        	mov	qword ptr [rbp + 0x260], rdx
140989881: 0f 10 85 e0 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x3e0]
140989888: 0f 11 85 38 02 00 00        	movups	xmmword ptr [rbp + 0x238], xmm0
14098988f: 48 8b 95 f0 03 00 00        	mov	rdx, qword ptr [rbp + 0x3f0]
140989896: 48 89 95 48 02 00 00        	mov	qword ptr [rbp + 0x248], rdx
14098989d: 0f 10 85 60 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x460]
1409898a4: 0f 10 8d 70 04 00 00        	movups	xmm1, xmmword ptr [rbp + 0x470]
1409898ab: 0f 10 95 80 04 00 00        	movups	xmm2, xmmword ptr [rbp + 0x480]
1409898b2: 0f 10 9d 90 04 00 00        	movups	xmm3, xmmword ptr [rbp + 0x490]
1409898b9: 0f 11 85 98 02 00 00        	movups	xmmword ptr [rbp + 0x298], xmm0
1409898c0: 0f 11 8d a8 02 00 00        	movups	xmmword ptr [rbp + 0x2a8], xmm1
1409898c7: 0f 11 95 b8 02 00 00        	movups	xmmword ptr [rbp + 0x2b8], xmm2
1409898ce: 0f 11 9d c8 02 00 00        	movups	xmmword ptr [rbp + 0x2c8], xmm3
1409898d5: 0f 10 85 a0 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x4a0]
1409898dc: 0f 11 85 d8 02 00 00        	movups	xmmword ptr [rbp + 0x2d8], xmm0
1409898e3: 0f 10 85 b0 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x4b0]
1409898ea: 0f 11 85 e8 02 00 00        	movups	xmmword ptr [rbp + 0x2e8], xmm0
1409898f1: 48 8b 95 08 04 00 00        	mov	rdx, qword ptr [rbp + 0x408]
1409898f8: 48 89 95 78 02 00 00        	mov	qword ptr [rbp + 0x278], rdx
1409898ff: 0f 10 85 f8 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x3f8]
140989906: 0f 11 85 68 02 00 00        	movups	xmmword ptr [rbp + 0x268], xmm0
14098990d: 48 89 8d 00 03 00 00        	mov	qword ptr [rbp + 0x300], rcx
140989914: 48 89 85 f8 02 00 00        	mov	qword ptr [rbp + 0x2f8], rax
14098991b: 0f 29 b5 80 02 00 00        	movaps	xmmword ptr [rbp + 0x280], xmm6
140989922: 48 8d 4d 30                 	lea	rcx, [rbp + 0x30]
140989926: 48 8d 95 20 02 00 00        	lea	rdx, [rbp + 0x220]
14098992d: e8 9e 3b 05 00              	call	0x1409dd4d0 <.text+0x9dc4d0>
140989932: 80 bd e6 04 00 00 08        	cmp	byte ptr [rbp + 0x4e6], 0x8
140989939: 0f b6 9d af 05 00 00        	movzx	ebx, byte ptr [rbp + 0x5af]
140989940: 74 0c                       	je	0x14098994e <.text+0x98894e>
140989942: 48 8d 8d d0 04 00 00        	lea	rcx, [rbp + 0x4d0]
140989949: e8 62 8b 25 00              	call	0x140be24b0 <.text+0xbe14b0>
14098994e: 0f 28 85 40 05 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x540]
140989955: 0f 29 85 b0 03 00 00        	movaps	xmmword ptr [rbp + 0x3b0], xmm0
14098995c: 48 8b 85 50 05 00 00        	mov	rax, qword ptr [rbp + 0x550]
140989963: 48 89 85 c0 03 00 00        	mov	qword ptr [rbp + 0x3c0], rax
14098996a: 0f 28 8d 60 05 00 00        	movaps	xmm1, xmmword ptr [rbp + 0x560]
140989971: 0f 11 8d c8 03 00 00        	movups	xmmword ptr [rbp + 0x3c8], xmm1
140989978: 48 8b 8d 70 05 00 00        	mov	rcx, qword ptr [rbp + 0x570]
14098997f: 48 89 8d d8 03 00 00        	mov	qword ptr [rbp + 0x3d8], rcx
140989986: 48 89 45 00                 	mov	qword ptr [rbp], rax
14098998a: 0f 29 45 f0                 	movaps	xmmword ptr [rbp - 0x10], xmm0
14098998e: 48 89 4d 20                 	mov	qword ptr [rbp + 0x20], rcx
140989992: 0f 29 4d 10                 	movaps	xmmword ptr [rbp + 0x10], xmm1
140989996: 48 8d 85 d0 01 00 00        	lea	rax, [rbp + 0x1d0]
14098999d: 48 89 44 24 48              	mov	qword ptr [rsp + 0x48], rax
1409899a2: 4c 89 74 24 40              	mov	qword ptr [rsp + 0x40], r14
1409899a7: 48 8d 45 10                 	lea	rax, [rbp + 0x10]
1409899ab: 48 89 44 24 38              	mov	qword ptr [rsp + 0x38], rax
1409899b0: 48 8d 45 f0                 	lea	rax, [rbp - 0x10]
1409899b4: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
1409899b9: 88 5c 24 28                 	mov	byte ptr [rsp + 0x28], bl
1409899bd: c6 44 24 20 00              	mov	byte ptr [rsp + 0x20], 0x0
1409899c2: 4c 8d a5 08 02 00 00        	lea	r12, [rbp + 0x208]
1409899c9: 4c 89 e1                    	mov	rcx, r12
1409899cc: 48 8b b5 b0 05 00 00        	mov	rsi, qword ptr [rbp + 0x5b0]
1409899d3: 48 89 f2                    	mov	rdx, rsi
1409899d6: 4d 89 e8                    	mov	r8, r13
1409899d9: 4c 8b 8d a0 05 00 00        	mov	r9, qword ptr [rbp + 0x5a0]
1409899e0: e8 9b f8 09 00              	call	0x140a29280 <.text+0xa28280>
1409899e5: 48 89 74 24 30              	mov	qword ptr [rsp + 0x30], rsi
1409899ea: 4c 89 6c 24 28              	mov	qword ptr [rsp + 0x28], r13
1409899ef: 48 c7 44 24 20 1e 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1e
1409899f8: 48 8d 15 48 f2 12 02        	lea	rdx, [rip + 0x212f248]  # 0x142ab8c47
1409899ff: 4c 8d 0d 56 f2 12 02        	lea	r9, [rip + 0x212f256]   # 0x142ab8c5c
140989a06: 4c 8d b5 b0 03 00 00        	lea	r14, [rbp + 0x3b0]
140989a0d: 41 b8 15 00 00 00           	mov	r8d, 0x15
140989a13: 4c 89 f1                    	mov	rcx, r14
140989a16: e8 85 98 c7 00              	call	0x1416032a0 <.text+0x16022a0>
140989a1b: 4c 8d bd 20 02 00 00        	lea	r15, [rbp + 0x220]
140989a22: 49 b8 e3 c5 90 19 c5 9d fe 86       	movabs	r8, -0x7901623ae66f3a1d
140989a2c: 4c 89 f9                    	mov	rcx, r15
140989a2f: 4c 89 f2                    	mov	rdx, r14
140989a32: e8 d9 36 c8 00              	call	0x14160d110 <.text+0x160c110>
140989a37: 49 b8 74 44 1c 32 11 a8 61 86       	movabs	r8, -0x799e57eecde3bb8c
140989a41: 4c 89 f1                    	mov	rcx, r14
140989a44: 4c 89 fa                    	mov	rdx, r15
140989a47: e8 b4 ce c7 00              	call	0x141606900 <.text+0x1605900>
140989a4c: 49 b8 ba 2c 07 ed b8 e4 8e 2d       	movabs	r8, 0x2d8ee4b8ed072cba
140989a56: 4c 89 f9                    	mov	rcx, r15
140989a59: 4c 89 f2                    	mov	rdx, r14
140989a5c: e8 cf af c7 00              	call	0x141604a30 <.text+0x1603a30>
140989a61: 48 8d 8d b0 03 00 00        	lea	rcx, [rbp + 0x3b0]
140989a68: 49 b8 4d f3 05 c9 b1 38 6c 68       	movabs	r8, 0x686c38b1c905f34d
140989a72: 4c 89 fa                    	mov	rdx, r15
140989a75: e8 b6 cc c7 00              	call	0x141606730 <.text+0x1605730>
140989a7a: 0f 10 85 50 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x450]
140989a81: 0f 29 45 70                 	movaps	xmmword ptr [rbp + 0x70], xmm0
140989a85: 0f 10 85 40 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x440]
140989a8c: 0f 29 45 60                 	movaps	xmmword ptr [rbp + 0x60], xmm0
140989a90: 0f 10 85 30 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x430]
140989a97: 0f 29 45 50                 	movaps	xmmword ptr [rbp + 0x50], xmm0
140989a9b: 0f 10 85 20 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x420]
140989aa2: 0f 29 45 40                 	movaps	xmmword ptr [rbp + 0x40], xmm0
140989aa6: 0f 10 85 10 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x410]
140989aad: 0f 29 45 30                 	movaps	xmmword ptr [rbp + 0x30], xmm0
140989ab1: 48 8b 85 e8 04 00 00        	mov	rax, qword ptr [rbp + 0x4e8]
140989ab8: 48 8b 8d 30 05 00 00        	mov	rcx, qword ptr [rbp + 0x530]
140989abf: 48 8b 95 c0 03 00 00        	mov	rdx, qword ptr [rbp + 0x3c0]
140989ac6: 48 89 95 30 02 00 00        	mov	qword ptr [rbp + 0x230], rdx
140989acd: 0f 10 85 b0 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x3b0]
140989ad4: 0f 29 85 20 02 00 00        	movaps	xmmword ptr [rbp + 0x220], xmm0
140989adb: 0f 10 85 c8 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x3c8]
140989ae2: 0f 29 85 50 02 00 00        	movaps	xmmword ptr [rbp + 0x250], xmm0
140989ae9: 48 8b 95 d8 03 00 00        	mov	rdx, qword ptr [rbp + 0x3d8]
140989af0: 48 89 95 60 02 00 00        	mov	qword ptr [rbp + 0x260], rdx
140989af7: 0f 10 85 e0 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x3e0]
140989afe: 0f 11 85 38 02 00 00        	movups	xmmword ptr [rbp + 0x238], xmm0
140989b05: 48 8b 95 f0 03 00 00        	mov	rdx, qword ptr [rbp + 0x3f0]
140989b0c: 48 89 95 48 02 00 00        	mov	qword ptr [rbp + 0x248], rdx
140989b13: 0f 10 85 60 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x460]
140989b1a: 0f 10 8d 70 04 00 00        	movups	xmm1, xmmword ptr [rbp + 0x470]
140989b21: 0f 10 95 80 04 00 00        	movups	xmm2, xmmword ptr [rbp + 0x480]
140989b28: 0f 10 9d 90 04 00 00        	movups	xmm3, xmmword ptr [rbp + 0x490]
140989b2f: 0f 11 85 98 02 00 00        	movups	xmmword ptr [rbp + 0x298], xmm0
140989b36: 0f 11 8d a8 02 00 00        	movups	xmmword ptr [rbp + 0x2a8], xmm1
140989b3d: 0f 11 95 b8 02 00 00        	movups	xmmword ptr [rbp + 0x2b8], xmm2
140989b44: 0f 11 9d c8 02 00 00        	movups	xmmword ptr [rbp + 0x2c8], xmm3
140989b4b: 0f 10 85 a0 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x4a0]
140989b52: 0f 11 85 d8 02 00 00        	movups	xmmword ptr [rbp + 0x2d8], xmm0
140989b59: 0f 10 85 b0 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x4b0]
140989b60: 0f 11 85 e8 02 00 00        	movups	xmmword ptr [rbp + 0x2e8], xmm0
140989b67: 48 8b 95 08 04 00 00        	mov	rdx, qword ptr [rbp + 0x408]
140989b6e: 48 89 95 78 02 00 00        	mov	qword ptr [rbp + 0x278], rdx
140989b75: 0f 10 85 f8 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x3f8]
140989b7c: 0f 11 85 68 02 00 00        	movups	xmmword ptr [rbp + 0x268], xmm0
140989b83: 48 89 8d 00 03 00 00        	mov	qword ptr [rbp + 0x300], rcx
140989b8a: 48 89 85 f8 02 00 00        	mov	qword ptr [rbp + 0x2f8], rax
140989b91: 0f 29 b5 80 02 00 00        	movaps	xmmword ptr [rbp + 0x280], xmm6
140989b98: 48 8d 4d 30                 	lea	rcx, [rbp + 0x30]
140989b9c: 48 8d 95 20 02 00 00        	lea	rdx, [rbp + 0x220]
140989ba3: e8 28 39 05 00              	call	0x1409dd4d0 <.text+0x9dc4d0>
140989ba8: 80 bd e6 04 00 00 08        	cmp	byte ptr [rbp + 0x4e6], 0x8
140989baf: 74 0c                       	je	0x140989bbd <.text+0x988bbd>
140989bb1: 48 8d 8d d0 04 00 00        	lea	rcx, [rbp + 0x4d0]
140989bb8: e8 f3 88 25 00              	call	0x140be24b0 <.text+0xbe14b0>
140989bbd: 48 8b b5 a0 05 00 00        	mov	rsi, qword ptr [rbp + 0x5a0]
140989bc4: 48 8b 86 50 14 00 00        	mov	rax, qword ptr [rsi + 0x1450]
140989bcb: 8b 40 68                    	mov	eax, dword ptr [rax + 0x68]
140989bce: 83 f8 01                    	cmp	eax, 0x1
140989bd1: 83 d0 00                    	adc	eax, 0x0
140989bd4: 89 c1                       	mov	ecx, eax
140989bd6: 83 e1 3f                    	and	ecx, 0x3f
140989bd9: ba 40 00 00 00              	mov	edx, 0x40
140989bde: 29 ca                       	sub	edx, ecx
140989be0: 85 c9                       	test	ecx, ecx
140989be2: 0f 44 d1                    	cmove	edx, ecx
140989be5: 48 01 c2                    	add	rdx, rax
140989be8: 48 c1 e2 02                 	shl	rdx, 0x2
140989bec: 48 8d 4d 30                 	lea	rcx, [rbp + 0x30]
140989bf0: 41 b8 20 00 00 00           	mov	r8d, 0x20
140989bf6: e8 25 b2 24 00              	call	0x140bd4e20 <.text+0xbd3e20>
140989bfb: 0f 10 45 30                 	movups	xmm0, xmmword ptr [rbp + 0x30]
140989bff: 0f 29 85 20 02 00 00        	movaps	xmmword ptr [rbp + 0x220], xmm0
140989c06: 48 8d 8d c0 03 00 00        	lea	rcx, [rbp + 0x3c0]
140989c0d: 48 8d 95 20 02 00 00        	lea	rdx, [rbp + 0x220]
140989c14: e8 e7 d0 24 00              	call	0x140bd6d00 <.text+0xbd5d00>
140989c19: 48 8d 05 ee ef 12 02        	lea	rax, [rip + 0x212efee]  # 0x142ab8c0e
140989c20: 48 89 85 b0 03 00 00        	mov	qword ptr [rbp + 0x3b0], rax
140989c27: 48 c7 85 b8 03 00 00 20 00 00 00    	mov	qword ptr [rbp + 0x3b8], 0x20
140989c32: 48 8d 95 b0 03 00 00        	lea	rdx, [rbp + 0x3b0]
140989c39: 4c 8b bd b0 05 00 00        	mov	r15, qword ptr [rbp + 0x5b0]
140989c40: 4c 89 f9                    	mov	rcx, r15
140989c43: e8 88 f8 25 00              	call	0x140be94d0 <.text+0xbe84d0>
140989c48: 0f 10 45 30                 	movups	xmm0, xmmword ptr [rbp + 0x30]
140989c4c: 0f 29 85 40 05 00 00        	movaps	xmmword ptr [rbp + 0x540], xmm0
140989c53: 89 85 50 05 00 00           	mov	dword ptr [rbp + 0x550], eax
140989c59: 48 8b 86 50 14 00 00        	mov	rax, qword ptr [rsi + 0x1450]
140989c60: 8b 50 6c                    	mov	edx, dword ptr [rax + 0x6c]
140989c63: 83 fa 01                    	cmp	edx, 0x1
140989c66: 83 d2 00                    	adc	edx, 0x0
140989c69: 48 c1 e2 02                 	shl	rdx, 0x2
140989c6d: 48 8d 4d 30                 	lea	rcx, [rbp + 0x30]
140989c71: 41 b8 20 00 00 00           	mov	r8d, 0x20
140989c77: e8 a4 b1 24 00              	call	0x140bd4e20 <.text+0xbd3e20>
140989c7c: 0f 10 45 30                 	movups	xmm0, xmmword ptr [rbp + 0x30]
140989c80: 0f 29 85 20 02 00 00        	movaps	xmmword ptr [rbp + 0x220], xmm0
140989c87: 48 8d 8d c0 03 00 00        	lea	rcx, [rbp + 0x3c0]
140989c8e: 48 8d 95 20 02 00 00        	lea	rdx, [rbp + 0x220]
140989c95: e8 66 d0 24 00              	call	0x140bd6d00 <.text+0xbd5d00>
140989c9a: 48 8d 05 8d ef 12 02        	lea	rax, [rip + 0x212ef8d]  # 0x142ab8c2e
140989ca1: 48 89 85 b0 03 00 00        	mov	qword ptr [rbp + 0x3b0], rax
140989ca8: 48 c7 85 b8 03 00 00 19 00 00 00    	mov	qword ptr [rbp + 0x3b8], 0x19
140989cb3: 48 8d 95 b0 03 00 00        	lea	rdx, [rbp + 0x3b0]
140989cba: 4c 89 f9                    	mov	rcx, r15
140989cbd: e8 0e f8 25 00              	call	0x140be94d0 <.text+0xbe84d0>
140989cc2: 0f 10 45 30                 	movups	xmm0, xmmword ptr [rbp + 0x30]
140989cc6: 0f 29 85 60 05 00 00        	movaps	xmmword ptr [rbp + 0x560], xmm0
140989ccd: 89 85 70 05 00 00           	mov	dword ptr [rbp + 0x570], eax
140989cd3: 48 8b b5 40 05 00 00        	mov	rsi, qword ptr [rbp + 0x540]
140989cda: 48 c1 ee 02                 	shr	rsi, 0x2
140989cde: 48 89 f0                    	mov	rax, rsi
140989ce1: 48 83 e0 3f                 	and	rax, 0x3f
140989ce5: 48 89 85 20 02 00 00        	mov	qword ptr [rbp + 0x220], rax
140989cec: 0f 85 33 06 00 00           	jne	0x14098a325 <.text+0x989325>
140989cf2: 4c 89 7c 24 30              	mov	qword ptr [rsp + 0x30], r15
140989cf7: 4c 89 6c 24 28              	mov	qword ptr [rsp + 0x28], r13
140989cfc: 48 c7 44 24 20 1d 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1d
140989d05: 48 8d 15 1c f1 12 02        	lea	rdx, [rip + 0x212f11c]  # 0x142ab8e28
140989d0c: 4c 8d 0d 8a ef 12 02        	lea	r9, [rip + 0x212ef8a]   # 0x142ab8c9d
140989d13: 41 b8 1d 00 00 00           	mov	r8d, 0x1d
140989d19: 4c 89 f1                    	mov	rcx, r14
140989d1c: e8 7f 95 c7 00              	call	0x1416032a0 <.text+0x16022a0>
140989d21: 48 8d 8d 20 02 00 00        	lea	rcx, [rbp + 0x220]
140989d28: 49 b8 01 f1 2c d7 c9 c4 e3 fe       	movabs	r8, -0x11c3b3628d30eff
140989d32: 4c 8d 8d 40 05 00 00        	lea	r9, [rbp + 0x540]
140989d39: 4c 89 f2                    	mov	rdx, r14
140989d3c: e8 1f 40 fe ff              	call	0x14096dd60 <.text+0x96cd60>
140989d41: 0f 10 85 c0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2c0]
140989d48: 0f 29 45 70                 	movaps	xmmword ptr [rbp + 0x70], xmm0
140989d4c: 0f 10 85 b0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2b0]
140989d53: 0f 29 45 60                 	movaps	xmmword ptr [rbp + 0x60], xmm0
140989d57: 0f 10 85 a0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2a0]
140989d5e: 0f 29 45 50                 	movaps	xmmword ptr [rbp + 0x50], xmm0
140989d62: 0f 10 85 90 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x290]
140989d69: 0f 29 45 40                 	movaps	xmmword ptr [rbp + 0x40], xmm0
140989d6d: 0f 10 85 80 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x280]
140989d74: 0f 29 45 30                 	movaps	xmmword ptr [rbp + 0x30], xmm0
140989d78: 48 8b 85 58 03 00 00        	mov	rax, qword ptr [rbp + 0x358]
140989d7f: 48 8b 8d a0 03 00 00        	mov	rcx, qword ptr [rbp + 0x3a0]
140989d86: 48 8b 95 30 02 00 00        	mov	rdx, qword ptr [rbp + 0x230]
140989d8d: 48 89 95 c0 03 00 00        	mov	qword ptr [rbp + 0x3c0], rdx
140989d94: 0f 10 85 20 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x220]
140989d9b: 0f 29 85 b0 03 00 00        	movaps	xmmword ptr [rbp + 0x3b0], xmm0
140989da2: 0f 10 85 38 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x238]
140989da9: 0f 29 85 e0 03 00 00        	movaps	xmmword ptr [rbp + 0x3e0], xmm0
140989db0: 48 8b 95 48 02 00 00        	mov	rdx, qword ptr [rbp + 0x248]
140989db7: 48 89 95 f0 03 00 00        	mov	qword ptr [rbp + 0x3f0], rdx
140989dbe: 0f 10 85 50 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x250]
140989dc5: 0f 11 85 c8 03 00 00        	movups	xmmword ptr [rbp + 0x3c8], xmm0
140989dcc: 48 8b 95 60 02 00 00        	mov	rdx, qword ptr [rbp + 0x260]
140989dd3: 48 89 95 d8 03 00 00        	mov	qword ptr [rbp + 0x3d8], rdx
140989dda: 0f 10 85 d0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2d0]
140989de1: 0f 10 8d e0 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x2e0]
140989de8: 0f 10 95 f0 02 00 00        	movups	xmm2, xmmword ptr [rbp + 0x2f0]
140989def: 0f 10 9d 00 03 00 00        	movups	xmm3, xmmword ptr [rbp + 0x300]
140989df6: 0f 11 85 28 04 00 00        	movups	xmmword ptr [rbp + 0x428], xmm0
140989dfd: 0f 11 8d 38 04 00 00        	movups	xmmword ptr [rbp + 0x438], xmm1
140989e04: 0f 11 95 48 04 00 00        	movups	xmmword ptr [rbp + 0x448], xmm2
140989e0b: 0f 11 9d 58 04 00 00        	movups	xmmword ptr [rbp + 0x458], xmm3
140989e12: 0f 10 85 10 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x310]
140989e19: 0f 11 85 68 04 00 00        	movups	xmmword ptr [rbp + 0x468], xmm0
140989e20: 0f 10 85 20 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x320]
140989e27: 0f 11 85 78 04 00 00        	movups	xmmword ptr [rbp + 0x478], xmm0
140989e2e: 48 8b 95 78 02 00 00        	mov	rdx, qword ptr [rbp + 0x278]
140989e35: 48 89 95 08 04 00 00        	mov	qword ptr [rbp + 0x408], rdx
140989e3c: 0f 10 85 68 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x268]
140989e43: 0f 11 85 f8 03 00 00        	movups	xmmword ptr [rbp + 0x3f8], xmm0
140989e4a: 48 89 8d 90 04 00 00        	mov	qword ptr [rbp + 0x490], rcx
140989e51: 48 89 85 88 04 00 00        	mov	qword ptr [rbp + 0x488], rax
140989e58: c7 85 10 04 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x410], 0x1
140989e62: 89 b5 14 04 00 00           	mov	dword ptr [rbp + 0x414], esi
140989e68: 48 be 01 00 00 00 01 00 00 00       	movabs	rsi, 0x100000001
140989e72: 48 89 b5 18 04 00 00        	mov	qword ptr [rbp + 0x418], rsi
140989e79: 48 8d 4d 30                 	lea	rcx, [rbp + 0x30]
140989e7d: 48 8d 95 b0 03 00 00        	lea	rdx, [rbp + 0x3b0]
140989e84: e8 47 36 05 00              	call	0x1409dd4d0 <.text+0x9dc4d0>
140989e89: 80 bd 56 03 00 00 08        	cmp	byte ptr [rbp + 0x356], 0x8
140989e90: 74 0c                       	je	0x140989e9e <.text+0x988e9e>
140989e92: 48 8d 8d 40 03 00 00        	lea	rcx, [rbp + 0x340]
140989e99: e8 12 86 25 00              	call	0x140be24b0 <.text+0xbe14b0>
140989e9e: 48 8b bd 60 05 00 00        	mov	rdi, qword ptr [rbp + 0x560]
140989ea5: 4c 89 7c 24 30              	mov	qword ptr [rsp + 0x30], r15
140989eaa: 4c 89 6c 24 28              	mov	qword ptr [rsp + 0x28], r13
140989eaf: 48 c7 44 24 20 1d 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1d
140989eb8: 48 8d 15 86 ef 12 02        	lea	rdx, [rip + 0x212ef86]  # 0x142ab8e45
140989ebf: 4c 8d 0d 9c ef 12 02        	lea	r9, [rip + 0x212ef9c]   # 0x142ab8e62
140989ec6: 41 b8 1d 00 00 00           	mov	r8d, 0x1d
140989ecc: 4c 89 f1                    	mov	rcx, r14
140989ecf: e8 cc 93 c7 00              	call	0x1416032a0 <.text+0x16022a0>
140989ed4: 48 8d 8d 20 02 00 00        	lea	rcx, [rbp + 0x220]
140989edb: 49 b8 7a d4 97 2e d4 30 42 c6       	movabs	r8, -0x39bdcf2bd1682b86
140989ee5: 4c 8d 8d 60 05 00 00        	lea	r9, [rbp + 0x560]
140989eec: 4c 89 f2                    	mov	rdx, r14
140989eef: e8 6c 3e fe ff              	call	0x14096dd60 <.text+0x96cd60>
140989ef4: 0f b6 05 0e ad ca 02        	movzx	eax, byte ptr [rip + 0x2caad0e] # 0x143634c09
140989efb: b9 04 00 00 00              	mov	ecx, 0x4
140989f00: ba 01 00 00 00              	mov	edx, 0x1
140989f05: e8 06 9d c8 ff              	call	0x140613c10 <.text+0x612c10>
140989f0a: 48 85 c0                    	test	rax, rax
140989f0d: 0f 84 45 04 00 00           	je	0x14098a358 <.text+0x989358>
140989f13: 49 89 c6                    	mov	r14, rax
140989f16: 48 c1 ef 02                 	shr	rdi, 0x2
140989f1a: 89 38                       	mov	dword ptr [rax], edi
140989f1c: 48 8b 95 68 02 00 00        	mov	rdx, qword ptr [rbp + 0x268]
140989f23: 48 85 d2                    	test	rdx, rdx
140989f26: 74 12                       	je	0x140989f3a <.text+0x988f3a>
140989f28: 48 8b 8d 70 02 00 00        	mov	rcx, qword ptr [rbp + 0x270]
140989f2f: 41 b8 01 00 00 00           	mov	r8d, 0x1
140989f35: e8 e6 9c c8 ff              	call	0x140613c20 <.text+0x612c20>
140989f3a: 48 c7 85 68 02 00 00 04 00 00 00    	mov	qword ptr [rbp + 0x268], 0x4
140989f45: 4c 89 b5 70 02 00 00        	mov	qword ptr [rbp + 0x270], r14
140989f4c: 48 c7 85 78 02 00 00 04 00 00 00    	mov	qword ptr [rbp + 0x278], 0x4
140989f57: 48 8d 8d b0 03 00 00        	lea	rcx, [rbp + 0x3b0]
140989f5e: 48 8d 95 20 02 00 00        	lea	rdx, [rbp + 0x220]
140989f65: 41 b8 90 01 00 00           	mov	r8d, 0x190
140989f6b: e8 90 9c f9 01              	call	0x142923c00 <.text+0x2922c00>
140989f70: 0f 10 85 50 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x450]
140989f77: 0f 29 45 70                 	movaps	xmmword ptr [rbp + 0x70], xmm0
140989f7b: 0f 10 85 40 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x440]
140989f82: 0f 29 45 60                 	movaps	xmmword ptr [rbp + 0x60], xmm0
140989f86: 0f 10 85 30 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x430]
140989f8d: 0f 29 45 50                 	movaps	xmmword ptr [rbp + 0x50], xmm0
140989f91: 0f 10 85 20 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x420]
140989f98: 0f 29 45 40                 	movaps	xmmword ptr [rbp + 0x40], xmm0
140989f9c: 0f 10 85 10 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x410]
140989fa3: 0f 29 45 30                 	movaps	xmmword ptr [rbp + 0x30], xmm0
140989fa7: 48 8b 85 e8 04 00 00        	mov	rax, qword ptr [rbp + 0x4e8]
140989fae: 48 8b 8d 30 05 00 00        	mov	rcx, qword ptr [rbp + 0x530]
140989fb5: 48 8b 95 c0 03 00 00        	mov	rdx, qword ptr [rbp + 0x3c0]
140989fbc: 48 89 95 30 02 00 00        	mov	qword ptr [rbp + 0x230], rdx
140989fc3: 0f 10 85 b0 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x3b0]
140989fca: 0f 29 85 20 02 00 00        	movaps	xmmword ptr [rbp + 0x220], xmm0
140989fd1: 0f 10 85 c8 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x3c8]
140989fd8: 0f 29 85 50 02 00 00        	movaps	xmmword ptr [rbp + 0x250], xmm0
140989fdf: 48 8b 95 d8 03 00 00        	mov	rdx, qword ptr [rbp + 0x3d8]
140989fe6: 48 89 95 60 02 00 00        	mov	qword ptr [rbp + 0x260], rdx
140989fed: 0f 10 85 e0 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x3e0]
140989ff4: 0f 11 85 38 02 00 00        	movups	xmmword ptr [rbp + 0x238], xmm0
140989ffb: 48 8b 95 f0 03 00 00        	mov	rdx, qword ptr [rbp + 0x3f0]
14098a002: 48 89 95 48 02 00 00        	mov	qword ptr [rbp + 0x248], rdx
14098a009: 0f 10 85 60 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x460]
14098a010: 0f 10 8d 70 04 00 00        	movups	xmm1, xmmword ptr [rbp + 0x470]
14098a017: 0f 10 95 80 04 00 00        	movups	xmm2, xmmword ptr [rbp + 0x480]
14098a01e: 0f 10 9d 90 04 00 00        	movups	xmm3, xmmword ptr [rbp + 0x490]
14098a025: 0f 11 85 98 02 00 00        	movups	xmmword ptr [rbp + 0x298], xmm0
14098a02c: 0f 11 8d a8 02 00 00        	movups	xmmword ptr [rbp + 0x2a8], xmm1
14098a033: 0f 11 95 b8 02 00 00        	movups	xmmword ptr [rbp + 0x2b8], xmm2
14098a03a: 0f 11 9d c8 02 00 00        	movups	xmmword ptr [rbp + 0x2c8], xmm3
14098a041: 0f 10 85 a0 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x4a0]
14098a048: 0f 11 85 d8 02 00 00        	movups	xmmword ptr [rbp + 0x2d8], xmm0
14098a04f: 0f 10 85 b0 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x4b0]
14098a056: 0f 11 85 e8 02 00 00        	movups	xmmword ptr [rbp + 0x2e8], xmm0
14098a05d: 48 8b 95 08 04 00 00        	mov	rdx, qword ptr [rbp + 0x408]
14098a064: 48 89 95 78 02 00 00        	mov	qword ptr [rbp + 0x278], rdx
14098a06b: 0f 10 85 f8 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x3f8]
14098a072: 0f 11 85 68 02 00 00        	movups	xmmword ptr [rbp + 0x268], xmm0
14098a079: 48 89 8d 00 03 00 00        	mov	qword ptr [rbp + 0x300], rcx
14098a080: 48 89 85 f8 02 00 00        	mov	qword ptr [rbp + 0x2f8], rax
14098a087: c7 85 80 02 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x280], 0x1
14098a091: 89 bd 84 02 00 00           	mov	dword ptr [rbp + 0x284], edi
14098a097: 48 89 b5 88 02 00 00        	mov	qword ptr [rbp + 0x288], rsi
14098a09e: 48 8d 4d 30                 	lea	rcx, [rbp + 0x30]
14098a0a2: 48 8d 95 20 02 00 00        	lea	rdx, [rbp + 0x220]
14098a0a9: e8 22 34 05 00              	call	0x1409dd4d0 <.text+0x9dc4d0>
14098a0ae: 4d 89 e6                    	mov	r14, r12
14098a0b1: 80 bd e6 04 00 00 08        	cmp	byte ptr [rbp + 0x4e6], 0x8
14098a0b8: 74 0c                       	je	0x14098a0c6 <.text+0x9890c6>
14098a0ba: 48 8d 8d d0 04 00 00        	lea	rcx, [rbp + 0x4d0]
14098a0c1: e8 ea 83 25 00              	call	0x140be24b0 <.text+0xbe14b0>
14098a0c6: 88 5c 24 28                 	mov	byte ptr [rsp + 0x28], bl
14098a0ca: c6 44 24 20 00              	mov	byte ptr [rsp + 0x20], 0x0
14098a0cf: 48 8d 4d d8                 	lea	rcx, [rbp - 0x28]
14098a0d3: 4c 89 fa                    	mov	rdx, r15
14098a0d6: 4d 89 e8                    	mov	r8, r13
14098a0d9: 4c 89 ee                    	mov	rsi, r13
14098a0dc: 4c 8b ad a0 05 00 00        	mov	r13, qword ptr [rbp + 0x5a0]
14098a0e3: 4d 89 e9                    	mov	r9, r13
14098a0e6: e8 55 f5 09 00              	call	0x140a29640 <.text+0xa28640>
14098a0eb: 49 8b 85 50 14 00 00        	mov	rax, qword ptr [r13 + 0x1450]
14098a0f2: 48 89 85 c0 05 00 00        	mov	qword ptr [rbp + 0x5c0], rax
14098a0f9: 4c 89 7c 24 30              	mov	qword ptr [rsp + 0x30], r15
14098a0fe: 48 89 74 24 28              	mov	qword ptr [rsp + 0x28], rsi
14098a103: 48 c7 44 24 20 24 00 00 00  	mov	qword ptr [rsp + 0x20], 0x24
14098a10c: 48 8d 15 9d 09 14 02        	lea	rdx, [rip + 0x214099d]  # 0x142acaab0
14098a113: 4c 8d 0d a9 09 14 02        	lea	r9, [rip + 0x21409a9]   # 0x142acaac3
14098a11a: 4c 89 ff                    	mov	rdi, r15
14098a11d: 4c 8d bd b0 03 00 00        	lea	r15, [rbp + 0x3b0]
14098a124: 41 b8 13 00 00 00           	mov	r8d, 0x13
14098a12a: 4c 89 f9                    	mov	rcx, r15
14098a12d: e8 6e 91 c7 00              	call	0x1416032a0 <.text+0x16022a0>
14098a132: 4c 8d a5 20 02 00 00        	lea	r12, [rbp + 0x220]
14098a139: 49 b8 0e 23 a2 3a 94 79 5e 9b       	movabs	r8, -0x64a1866bc55ddcf2
14098a143: 4c 89 e1                    	mov	rcx, r12
14098a146: 4c 89 fa                    	mov	rdx, r15
14098a149: 4d 89 f1                    	mov	r9, r14
14098a14c: e8 6f 39 fe ff              	call	0x14096dac0 <.text+0x96cac0>
14098a151: 49 b8 e3 c5 90 19 c5 9d fe 86       	movabs	r8, -0x7901623ae66f3a1d
14098a15b: 4c 89 f9                    	mov	rcx, r15
14098a15e: 4c 89 e2                    	mov	rdx, r12
14098a161: e8 aa 2f c8 00              	call	0x14160d110 <.text+0x160c110>
14098a166: 49 b8 ae 0c 04 2a 5e 89 66 40       	movabs	r8, 0x4066895e2a040cae
14098a170: 4c 89 e1                    	mov	rcx, r12
14098a173: 4c 89 fa                    	mov	rdx, r15
14098a176: 4c 8d bd 40 05 00 00        	lea	r15, [rbp + 0x540]
14098a17d: 4d 89 f9                    	mov	r9, r15
14098a180: e8 db 3b fe ff              	call	0x14096dd60 <.text+0x96cd60>
14098a185: 4c 8d b5 b0 03 00 00        	lea	r14, [rbp + 0x3b0]
14098a18c: 49 b8 84 ac 8a 23 de bb 37 53       	movabs	r8, 0x5337bbde238aac84
14098a196: 4c 89 f1                    	mov	rcx, r14
14098a199: 4c 89 e2                    	mov	rdx, r12
14098a19c: 4c 8d 4d 10                 	lea	r9, [rbp + 0x10]
14098a1a0: e8 1b 39 fe ff              	call	0x14096dac0 <.text+0x96cac0>
14098a1a5: c6 85 28 02 00 00 00        	mov	byte ptr [rbp + 0x228], 0x0
14098a1ac: 88 9d 29 02 00 00           	mov	byte ptr [rbp + 0x229], bl
14098a1b2: 48 8b 85 c0 05 00 00        	mov	rax, qword ptr [rbp + 0x5c0]
14098a1b9: 48 89 85 20 02 00 00        	mov	qword ptr [rbp + 0x220], rax
14098a1c0: 4c 8d 85 20 02 00 00        	lea	r8, [rbp + 0x220]
14098a1c7: 4c 89 f1                    	mov	rcx, r14
14098a1ca: 4c 8d 75 f0                 	lea	r14, [rbp - 0x10]
14098a1ce: 4c 89 f2                    	mov	rdx, r14
14098a1d1: e8 4a 19 c8 00              	call	0x14160bb20 <.text+0x160ab20>
14098a1d6: 4c 8d a5 60 05 00 00        	lea	r12, [rbp + 0x560]
14098a1dd: 4c 89 64 24 38              	mov	qword ptr [rsp + 0x38], r12
14098a1e2: 4c 89 7c 24 30              	mov	qword ptr [rsp + 0x30], r15
14098a1e7: 4c 89 74 24 28              	mov	qword ptr [rsp + 0x28], r14
14098a1ec: 88 5c 24 20                 	mov	byte ptr [rsp + 0x20], bl
14098a1f0: 48 89 f9                    	mov	rcx, rdi
14098a1f3: 48 89 f2                    	mov	rdx, rsi
14098a1f6: 4d 89 e8                    	mov	r8, r13
14098a1f9: 45 31 c9                    	xor	r9d, r9d
14098a1fc: e8 af f5 09 00              	call	0x140a297b0 <.text+0xa287b0>
14098a201: 4c 89 64 24 48              	mov	qword ptr [rsp + 0x48], r12
14098a206: 4c 89 7c 24 40              	mov	qword ptr [rsp + 0x40], r15
14098a20b: 48 8d 45 10                 	lea	rax, [rbp + 0x10]
14098a20f: 48 89 44 24 38              	mov	qword ptr [rsp + 0x38], rax
14098a214: 4c 89 74 24 30              	mov	qword ptr [rsp + 0x30], r14
14098a219: 4c 8d bd 08 02 00 00        	lea	r15, [rbp + 0x208]
14098a220: 4c 89 7c 24 28              	mov	qword ptr [rsp + 0x28], r15
14098a225: 88 5c 24 20                 	mov	byte ptr [rsp + 0x20], bl
14098a229: 48 89 f9                    	mov	rcx, rdi
14098a22c: 48 89 f2                    	mov	rdx, rsi
14098a22f: 4d 89 e8                    	mov	r8, r13
14098a232: 45 31 c9                    	xor	r9d, r9d
14098a235: e8 f6 f6 09 00              	call	0x140a29930 <.text+0xa28930>
14098a23a: 48 89 7c 24 30              	mov	qword ptr [rsp + 0x30], rdi
14098a23f: 48 89 74 24 28              	mov	qword ptr [rsp + 0x28], rsi
14098a244: 48 c7 44 24 20 30 00 00 00  	mov	qword ptr [rsp + 0x20], 0x30
14098a24d: 48 8d 15 93 09 14 02        	lea	rdx, [rip + 0x2140993]  # 0x142acabe7
14098a254: 4c 8d 0d b3 09 14 02        	lea	r9, [rip + 0x21409b3]   # 0x142acac0e
14098a25b: 48 8d b5 b0 03 00 00        	lea	rsi, [rbp + 0x3b0]
14098a262: 41 b8 27 00 00 00           	mov	r8d, 0x27
14098a268: 48 89 f1                    	mov	rcx, rsi
14098a26b: e8 30 90 c7 00              	call	0x1416032a0 <.text+0x16022a0>
14098a270: 48 8d bd 20 02 00 00        	lea	rdi, [rbp + 0x220]
14098a277: 48 89 f9                    	mov	rcx, rdi
14098a27a: 48 89 f2                    	mov	rdx, rsi
14098a27d: 49 b8 0e 23 a2 3a 94 79 5e 9b       	movabs	r8, -0x64a1866bc55ddcf2
14098a287: 4d 89 f9                    	mov	r9, r15
14098a28a: e8 31 38 fe ff              	call	0x14096dac0 <.text+0x96cac0>
14098a28f: 48 89 f1                    	mov	rcx, rsi
14098a292: 48 89 fa                    	mov	rdx, rdi
14098a295: 49 b8 e3 c5 90 19 c5 9d fe 86       	movabs	r8, -0x7901623ae66f3a1d
14098a29f: e8 6c 2e c8 00              	call	0x14160d110 <.text+0x160c110>
14098a2a4: 48 89 f9                    	mov	rcx, rdi
14098a2a7: 48 89 f2                    	mov	rdx, rsi
14098a2aa: 49 b8 84 ac 8a 23 de bb 37 53       	movabs	r8, 0x5337bbde238aac84
14098a2b4: 4c 8d 4d 10                 	lea	r9, [rbp + 0x10]
14098a2b8: e8 03 38 fe ff              	call	0x14096dac0 <.text+0x96cac0>
14098a2bd: c7 44 24 20 01 00 00 00     	mov	dword ptr [rsp + 0x20], 0x1
14098a2c5: 49 b8 e8 cd 33 3e 1e 56 c6 5f       	movabs	r8, 0x5fc6561e3e33cde8
14098a2cf: 48 89 f1                    	mov	rcx, rsi
14098a2d2: 48 89 fa                    	mov	rdx, rdi
14098a2d5: 4c 8d 4d d8                 	lea	r9, [rbp - 0x28]
14098a2d9: e8 22 3d fe ff              	call	0x14096e000 <.text+0x96d000>
14098a2de: 49 b8 80 8c 9c 6a 6a c3 f9 cd       	movabs	r8, -0x32063c9595637380
14098a2e8: 48 89 f9                    	mov	rcx, rdi
14098a2eb: 48 89 f2                    	mov	rdx, rsi
14098a2ee: 41 b9 01 00 00 00           	mov	r9d, 0x1
14098a2f4: e8 67 df c7 00              	call	0x141608260 <.text+0x1607260>
14098a2f9: 48 89 f9                    	mov	rcx, rdi
14098a2fc: 4c 89 f2                    	mov	rdx, r14
14098a2ff: 45 31 c0                    	xor	r8d, r8d
14098a302: 41 89 d9                    	mov	r9d, ebx
14098a305: e8 96 0f c8 00              	call	0x14160b2a0 <.text+0x160a2a0>
14098a30a: 0f 28 b5 d0 05 00 00        	movaps	xmm6, xmmword ptr [rbp + 0x5d0]
14098a311: 48 81 c4 68 06 00 00        	add	rsp, 0x668
14098a318: 5b                          	pop	rbx
14098a319: 5f                          	pop	rdi
14098a31a: 5e                          	pop	rsi
14098a31b: 41 5c                       	pop	r12
14098a31d: 41 5d                       	pop	r13
14098a31f: 41 5e                       	pop	r14
14098a321: 41 5f                       	pop	r15
14098a323: 5d                          	pop	rbp
14098a324: c3                          	ret
14098a325: 48 c7 85 b0 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x3b0], 0x0
14098a330: 48 8d 05 d9 ea 12 02        	lea	rax, [rip + 0x212ead9]  # 0x142ab8e10
14098a337: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
14098a33c: 4c 8d 05 3d e9 12 02        	lea	r8, [rip + 0x212e93d]   # 0x142ab8c80
14098a343: 48 8d 95 20 02 00 00        	lea	rdx, [rbp + 0x220]
14098a34a: 4c 8d 8d b0 03 00 00        	lea	r9, [rbp + 0x3b0]
14098a351: 31 c9                       	xor	ecx, ecx
14098a353: e8 3e f2 f4 01              	call	0x1428d9596 <.text+0x28d8596>
14098a358: 4c 8d 05 71 35 2f 02        	lea	r8, [rip + 0x22f3571]   # 0x142c7d8d0
14098a35f: b9 01 00 00 00              	mov	ecx, 0x1
14098a364: ba 04 00 00 00              	mov	edx, 0x4
14098a369: e8 75 ec f4 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
14098a36e: e9 f4 f2 ff ff              	jmp	0x140989667 <.text+0x988667>
14098a373: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
14098a380: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14098a385: 55                          	push	rbp
14098a386: 41 57                       	push	r15
14098a388: 41 56                       	push	r14
14098a38a: 41 55                       	push	r13
14098a38c: 41 54                       	push	r12
14098a38e: 56                          	push	rsi
14098a38f: 57                          	push	rdi
14098a390: 53                          	push	rbx
14098a391: 48 83 ec 68                 	sub	rsp, 0x68
14098a395: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14098a39c: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
14098a3a1: 80 bd e6 04 00 00 08        	cmp	byte ptr [rbp + 0x4e6], 0x8
14098a3a8: 74 0c                       	je	0x14098a3b6 <.text+0x9893b6>
14098a3aa: 48 8d 8d d0 04 00 00        	lea	rcx, [rbp + 0x4d0]
14098a3b1: e8 fa 80 25 00              	call	0x140be24b0 <.text+0xbe14b0>
14098a3b6: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
14098a3bb: 48 83 c4 68                 	add	rsp, 0x68
14098a3bf: 5b                          	pop	rbx
14098a3c0: 5f                          	pop	rdi
14098a3c1: 5e                          	pop	rsi
14098a3c2: 41 5c                       	pop	r12
14098a3c4: 41 5d                       	pop	r13
14098a3c6: 41 5e                       	pop	r14
14098a3c8: 41 5f                       	pop	r15
14098a3ca: 5d                          	pop	rbp
14098a3cb: c3                          	ret
14098a3cc: 0f 1f 40 00                 	nop	dword ptr [rax]
14098a3d0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14098a3d5: 55                          	push	rbp
14098a3d6: 41 57                       	push	r15
14098a3d8: 41 56                       	push	r14
14098a3da: 41 55                       	push	r13
14098a3dc: 41 54                       	push	r12
14098a3de: 56                          	push	rsi
14098a3df: 57                          	push	rdi
14098a3e0: 53                          	push	rbx
14098a3e1: 48 83 ec 68                 	sub	rsp, 0x68
14098a3e5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14098a3ec: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
14098a3f1: 80 bd 56 03 00 00 08        	cmp	byte ptr [rbp + 0x356], 0x8
14098a3f8: 74 0c                       	je	0x14098a406 <.text+0x989406>
14098a3fa: 48 8d 8d 40 03 00 00        	lea	rcx, [rbp + 0x340]
14098a401: e8 aa 80 25 00              	call	0x140be24b0 <.text+0xbe14b0>
14098a406: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
14098a40b: 48 83 c4 68                 	add	rsp, 0x68
14098a40f: 5b                          	pop	rbx
14098a410: 5f                          	pop	rdi
14098a411: 5e                          	pop	rsi
14098a412: 41 5c                       	pop	r12
14098a414: 41 5d                       	pop	r13
14098a416: 41 5e                       	pop	r14
14098a418: 41 5f                       	pop	r15
14098a41a: 5d                          	pop	rbp
14098a41b: c3                          	ret
14098a41c: 0f 1f 40 00                 	nop	dword ptr [rax]
14098a420: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14098a425: 55                          	push	rbp
14098a426: 41 57                       	push	r15
14098a428: 41 56                       	push	r14
14098a42a: 41 55                       	push	r13
14098a42c: 41 54                       	push	r12
14098a42e: 56                          	push	rsi
14098a42f: 57                          	push	rdi
14098a430: 53                          	push	rbx
14098a431: 48 83 ec 68                 	sub	rsp, 0x68
14098a435: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14098a43c: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
14098a441: 48 8d 8d 08 02 00 00        	lea	rcx, [rbp + 0x208]
14098a448: e8 63 80 25 00              	call	0x140be24b0 <.text+0xbe14b0>
14098a44d: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
14098a452: 48 83 c4 68                 	add	rsp, 0x68
14098a456: 5b                          	pop	rbx
14098a457: 5f                          	pop	rdi
14098a458: 5e                          	pop	rsi
14098a459: 41 5c                       	pop	r12
14098a45b: 41 5d                       	pop	r13
14098a45d: 41 5e                       	pop	r14
14098a45f: 41 5f                       	pop	r15
14098a461: 5d                          	pop	rbp
14098a462: c3                          	ret
14098a463: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
14098a470: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14098a475: 55                          	push	rbp
14098a476: 41 57                       	push	r15
14098a478: 41 56                       	push	r14
14098a47a: 41 55                       	push	r13
14098a47c: 41 54                       	push	r12
14098a47e: 56                          	push	rsi
14098a47f: 57                          	push	rdi
14098a480: 53                          	push	rbx
14098a481: 48 83 ec 68                 	sub	rsp, 0x68
14098a485: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14098a48c: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
14098a491: 48 8d 8d 40 05 00 00        	lea	rcx, [rbp + 0x540]
14098a498: e8 13 80 25 00              	call	0x140be24b0 <.text+0xbe14b0>
14098a49d: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
14098a4a2: 48 83 c4 68                 	add	rsp, 0x68
14098a4a6: 5b                          	pop	rbx
14098a4a7: 5f                          	pop	rdi
14098a4a8: 5e                          	pop	rsi
14098a4a9: 41 5c                       	pop	r12
14098a4ab: 41 5d                       	pop	r13
14098a4ad: 41 5e                       	pop	r14
14098a4af: 41 5f                       	pop	r15
14098a4b1: 5d                          	pop	rbp
14098a4b2: c3                          	ret
14098a4b3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
14098a4c0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14098a4c5: 55                          	push	rbp
14098a4c6: 41 57                       	push	r15
14098a4c8: 41 56                       	push	r14
14098a4ca: 41 55                       	push	r13
14098a4cc: 41 54                       	push	r12
14098a4ce: 56                          	push	rsi
14098a4cf: 57                          	push	rdi
14098a4d0: 53                          	push	rbx
14098a4d1: 48 83 ec 68                 	sub	rsp, 0x68
14098a4d5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14098a4dc: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
14098a4e1: 80 bd 66 01 00 00 08        	cmp	byte ptr [rbp + 0x166], 0x8
14098a4e8: 74 0c                       	je	0x14098a4f6 <.text+0x9894f6>
14098a4ea: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
14098a4f1: e8 ba 7f 25 00              	call	0x140be24b0 <.text+0xbe14b0>
14098a4f6: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
14098a4fb: 48 83 c4 68                 	add	rsp, 0x68
14098a4ff: 5b                          	pop	rbx
14098a500: 5f                          	pop	rdi
14098a501: 5e                          	pop	rsi
14098a502: 41 5c                       	pop	r12
14098a504: 41 5d                       	pop	r13
14098a506: 41 5e                       	pop	r14
14098a508: 41 5f                       	pop	r15
14098a50a: 5d                          	pop	rbp
14098a50b: c3                          	ret
14098a50c: 0f 1f 40 00                 	nop	dword ptr [rax]
14098a510: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14098a515: 55                          	push	rbp
14098a516: 41 57                       	push	r15
14098a518: 41 56                       	push	r14
14098a51a: 41 55                       	push	r13
14098a51c: 41 54                       	push	r12
14098a51e: 56                          	push	rsi
14098a51f: 57                          	push	rdi
14098a520: 53                          	push	rbx
14098a521: 48 83 ec 68                 	sub	rsp, 0x68
14098a525: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14098a52c: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
14098a531: 48 8b 85 90 05 00 00        	mov	rax, qword ptr [rbp + 0x590]
14098a538: 48 8b 50 08                 	mov	rdx, qword ptr [rax + 0x8]
14098a53c: 48 85 d2                    	test	rdx, rdx
14098a53f: 74 17                       	je	0x14098a558 <.text+0x989558>
14098a541: 48 8b 85 90 05 00 00        	mov	rax, qword ptr [rbp + 0x590]
14098a548: 4c 8b 40 10                 	mov	r8, qword ptr [rax + 0x10]
14098a54c: 48 8b 8d 88 05 00 00        	mov	rcx, qword ptr [rbp + 0x588]
14098a553: e8 c8 96 c8 ff              	call	0x140613c20 <.text+0x612c20>
14098a558: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
14098a55d: 48 83 c4 68                 	add	rsp, 0x68
14098a561: 5b                          	pop	rbx
14098a562: 5f                          	pop	rdi
14098a563: 5e                          	pop	rsi
14098a564: 41 5c                       	pop	r12
14098a566: 41 5d                       	pop	r13
14098a568: 41 5e                       	pop	r14
14098a56a: 41 5f                       	pop	r15
14098a56c: 5d                          	pop	rbp
14098a56d: c3                          	ret
14098a56e: 66 90                       	nop
14098a570: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14098a575: 55                          	push	rbp
14098a576: 41 57                       	push	r15
14098a578: 41 56                       	push	r14
14098a57a: 41 55                       	push	r13
14098a57c: 41 54                       	push	r12
14098a57e: 56                          	push	rsi
14098a57f: 57                          	push	rdi
14098a580: 53                          	push	rbx
14098a581: 48 83 ec 68                 	sub	rsp, 0x68
14098a585: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14098a58c: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
14098a591: 80 bd be 05 00 00 00        	cmp	byte ptr [rbp + 0x5be], 0x0
14098a598: 74 22                       	je	0x14098a5bc <.text+0x9895bc>
14098a59a: 48 8d 8d 20 02 00 00        	lea	rcx, [rbp + 0x220]
14098a5a1: e8 0a ca f0 ff              	call	0x140896fb0 <.text+0x895fb0>
14098a5a6: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
14098a5ab: 48 83 c4 68                 	add	rsp, 0x68
14098a5af: 5b                          	pop	rbx
14098a5b0: 5f                          	pop	rdi
14098a5b1: 5e                          	pop	rsi
14098a5b2: 41 5c                       	pop	r12
14098a5b4: 41 5d                       	pop	r13
14098a5b6: 41 5e                       	pop	r14
14098a5b8: 41 5f                       	pop	r15
14098a5ba: 5d                          	pop	rbp
14098a5bb: c3                          	ret
14098a5bc: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
14098a5c1: 48 83 c4 68                 	add	rsp, 0x68
14098a5c5: 5b                          	pop	rbx
14098a5c6: 5f                          	pop	rdi
14098a5c7: 5e                          	pop	rsi
14098a5c8: 41 5c                       	pop	r12
14098a5ca: 41 5d                       	pop	r13
14098a5cc: 41 5e                       	pop	r14
14098a5ce: 41 5f                       	pop	r15
14098a5d0: 5d                          	pop	rbp
14098a5d1: c3                          	ret
14098a5d2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
14098a5e0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14098a5e5: 55                          	push	rbp
14098a5e6: 41 57                       	push	r15
14098a5e8: 41 56                       	push	r14
14098a5ea: 41 55                       	push	r13
14098a5ec: 41 54                       	push	r12
14098a5ee: 56                          	push	rsi
14098a5ef: 57                          	push	rdi
14098a5f0: 53                          	push	rbx
14098a5f1: 48 83 ec 68                 	sub	rsp, 0x68
14098a5f5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14098a5fc: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
14098a601: 80 bd 66 01 00 00 08        	cmp	byte ptr [rbp + 0x166], 0x8
14098a608: 74 0c                       	je	0x14098a616 <.text+0x989616>
14098a60a: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
14098a611: e8 9a 7e 25 00              	call	0x140be24b0 <.text+0xbe14b0>
14098a616: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
14098a61b: 48 83 c4 68                 	add	rsp, 0x68
14098a61f: 5b                          	pop	rbx
14098a620: 5f                          	pop	rdi
14098a621: 5e                          	pop	rsi
14098a622: 41 5c                       	pop	r12
14098a624: 41 5d                       	pop	r13
14098a626: 41 5e                       	pop	r14
14098a628: 41 5f                       	pop	r15
14098a62a: 5d                          	pop	rbp
14098a62b: c3                          	ret
14098a62c: 0f 1f 40 00                 	nop	dword ptr [rax]
14098a630: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14098a635: 55                          	push	rbp
14098a636: 41 57                       	push	r15
14098a638: 41 56                       	push	r14
14098a63a: 41 55                       	push	r13
14098a63c: 41 54                       	push	r12
14098a63e: 56                          	push	rsi
14098a63f: 57                          	push	rdi
14098a640: 53                          	push	rbx
14098a641: 48 83 ec 68                 	sub	rsp, 0x68
14098a645: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14098a64c: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
14098a651: 80 bd 66 01 00 00 08        	cmp	byte ptr [rbp + 0x166], 0x8
14098a658: 74 0c                       	je	0x14098a666 <.text+0x989666>
14098a65a: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
14098a661: e8 4a 7e 25 00              	call	0x140be24b0 <.text+0xbe14b0>
14098a666: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
14098a66b: 48 83 c4 68                 	add	rsp, 0x68
14098a66f: 5b                          	pop	rbx
14098a670: 5f                          	pop	rdi
14098a671: 5e                          	pop	rsi
14098a672: 41 5c                       	pop	r12
14098a674: 41 5d                       	pop	r13
14098a676: 41 5e                       	pop	r14
14098a678: 41 5f                       	pop	r15
14098a67a: 5d                          	pop	rbp
14098a67b: c3                          	ret
14098a67c: 0f 1f 40 00                 	nop	dword ptr [rax]
14098a680: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14098a685: 55                          	push	rbp
14098a686: 41 57                       	push	r15
14098a688: 41 56                       	push	r14
14098a68a: 41 55                       	push	r13
14098a68c: 41 54                       	push	r12
14098a68e: 56                          	push	rsi
14098a68f: 57                          	push	rdi
14098a690: 53                          	push	rbx
14098a691: 48 83 ec 68                 	sub	rsp, 0x68
14098a695: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14098a69c: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
14098a6a1: 48 8d 8d b0 03 00 00        	lea	rcx, [rbp + 0x3b0]
14098a6a8: e8 03 c9 f0 ff              	call	0x140896fb0 <.text+0x895fb0>
14098a6ad: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
14098a6b2: 48 83 c4 68                 	add	rsp, 0x68
14098a6b6: 5b                          	pop	rbx
14098a6b7: 5f                          	pop	rdi
14098a6b8: 5e                          	pop	rsi
14098a6b9: 41 5c                       	pop	r12
14098a6bb: 41 5d                       	pop	r13
14098a6bd: 41 5e                       	pop	r14
14098a6bf: 41 5f                       	pop	r15
14098a6c1: 5d                          	pop	rbp
14098a6c2: c3                          	ret
14098a6c3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
14098a6d0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14098a6d5: 55                          	push	rbp
14098a6d6: 41 57                       	push	r15
14098a6d8: 41 56                       	push	r14
14098a6da: 41 55                       	push	r13
14098a6dc: 41 54                       	push	r12
14098a6de: 56                          	push	rsi
14098a6df: 57                          	push	rdi
14098a6e0: 53                          	push	rbx
14098a6e1: 48 83 ec 68                 	sub	rsp, 0x68
14098a6e5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14098a6ec: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
14098a6f1: 48 8d 8d b0 03 00 00        	lea	rcx, [rbp + 0x3b0]
14098a6f8: e8 b3 c8 f0 ff              	call	0x140896fb0 <.text+0x895fb0>
14098a6fd: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
14098a702: 48 83 c4 68                 	add	rsp, 0x68
14098a706: 5b                          	pop	rbx
14098a707: 5f                          	pop	rdi
14098a708: 5e                          	pop	rsi
14098a709: 41 5c                       	pop	r12
14098a70b: 41 5d                       	pop	r13
14098a70d: 41 5e                       	pop	r14
14098a70f: 41 5f                       	pop	r15
14098a711: 5d                          	pop	rbp
14098a712: c3                          	ret
14098a713: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
14098a720: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14098a725: 55                          	push	rbp
14098a726: 41 57                       	push	r15
14098a728: 41 56                       	push	r14
14098a72a: 41 55                       	push	r13
14098a72c: 41 54                       	push	r12
14098a72e: 56                          	push	rsi
14098a72f: 57                          	push	rdi
14098a730: 53                          	push	rbx
14098a731: 48 83 ec 68                 	sub	rsp, 0x68
14098a735: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14098a73c: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
14098a741: 48 8b 8d c8 01 00 00        	mov	rcx, qword ptr [rbp + 0x1c8]
14098a748: 48 8b 95 88 05 00 00        	mov	rdx, qword ptr [rbp + 0x588]
14098a74f: 48 8b bd c0 05 00 00        	mov	rdi, qword ptr [rbp + 0x5c0]
14098a756: 48 8b b5 90 05 00 00        	mov	rsi, qword ptr [rbp + 0x590]
14098a75d: 80 bd bf 05 00 00 00        	cmp	byte ptr [rbp + 0x5bf], 0x0
14098a764: 74 45                       	je	0x14098a7ab <.text+0x9897ab>
14098a766: 48 85 d2                    	test	rdx, rdx
14098a769: 74 0f                       	je	0x14098a77a <.text+0x98977a>
14098a76b: 48 c1 e2 02                 	shl	rdx, 0x2
14098a76f: 41 b8 04 00 00 00           	mov	r8d, 0x4
14098a775: e8 a6 94 c8 ff              	call	0x140613c20 <.text+0x612c20>
14098a77a: 48 85 ff                    	test	rdi, rdi
14098a77d: 74 16                       	je	0x14098a795 <.text+0x989795>
14098a77f: 48 c1 e7 02                 	shl	rdi, 0x2
14098a783: 48 8d 14 bf                 	lea	rdx, [rdi + 4*rdi]
14098a787: 41 b8 01 00 00 00           	mov	r8d, 0x1
14098a78d: 48 89 f1                    	mov	rcx, rsi
14098a790: e8 8b 94 c8 ff              	call	0x140613c20 <.text+0x612c20>
14098a795: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
14098a79a: 48 83 c4 68                 	add	rsp, 0x68
14098a79e: 5b                          	pop	rbx
14098a79f: 5f                          	pop	rdi
14098a7a0: 5e                          	pop	rsi
14098a7a1: 41 5c                       	pop	r12
14098a7a3: 41 5d                       	pop	r13
14098a7a5: 41 5e                       	pop	r14
14098a7a7: 41 5f                       	pop	r15
14098a7a9: 5d                          	pop	rbp
14098a7aa: c3                          	ret
14098a7ab: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
14098a7b0: 48 83 c4 68                 	add	rsp, 0x68
14098a7b4: 5b                          	pop	rbx
14098a7b5: 5f                          	pop	rdi
14098a7b6: 5e                          	pop	rsi
14098a7b7: 41 5c                       	pop	r12
14098a7b9: 41 5d                       	pop	r13
14098a7bb: 41 5e                       	pop	r14
14098a7bd: 41 5f                       	pop	r15
14098a7bf: 5d                          	pop	rbp
14098a7c0: c3                          	ret
14098a7c1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
14098a7d0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14098a7d5: 55                          	push	rbp
14098a7d6: 41 57                       	push	r15
14098a7d8: 41 56                       	push	r14
14098a7da: 41 55                       	push	r13
14098a7dc: 41 54                       	push	r12
14098a7de: 56                          	push	rsi
14098a7df: 57                          	push	rdi
14098a7e0: 53                          	push	rbx
14098a7e1: 48 83 ec 68                 	sub	rsp, 0x68
14098a7e5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14098a7ec: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
14098a7f1: 80 bd 56 03 00 00 08        	cmp	byte ptr [rbp + 0x356], 0x8
14098a7f8: 74 0c                       	je	0x14098a806 <.text+0x989806>
14098a7fa: 48 8d 8d 40 03 00 00        	lea	rcx, [rbp + 0x340]
14098a801: e8 aa 7c 25 00              	call	0x140be24b0 <.text+0xbe14b0>
14098a806: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
14098a80b: 48 83 c4 68                 	add	rsp, 0x68
14098a80f: 5b                          	pop	rbx
14098a810: 5f                          	pop	rdi
14098a811: 5e                          	pop	rsi
14098a812: 41 5c                       	pop	r12
14098a814: 41 5d                       	pop	r13
14098a816: 41 5e                       	pop	r14
14098a818: 41 5f                       	pop	r15
14098a81a: 5d                          	pop	rbp
14098a81b: c3                          	ret
14098a81c: 0f 1f 40 00                 	nop	dword ptr [rax]
14098a820: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14098a825: 55                          	push	rbp
14098a826: 41 57                       	push	r15
14098a828: 41 56                       	push	r14
14098a82a: 41 55                       	push	r13
14098a82c: 41 54                       	push	r12
14098a82e: 56                          	push	rsi
14098a82f: 57                          	push	rdi
14098a830: 53                          	push	rbx
14098a831: 48 83 ec 68                 	sub	rsp, 0x68
14098a835: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14098a83c: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
14098a841: 80 bd e6 04 00 00 08        	cmp	byte ptr [rbp + 0x4e6], 0x8
14098a848: 74 0c                       	je	0x14098a856 <.text+0x989856>
14098a84a: 48 8d 8d d0 04 00 00        	lea	rcx, [rbp + 0x4d0]
14098a851: e8 5a 7c 25 00              	call	0x140be24b0 <.text+0xbe14b0>
14098a856: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
14098a85b: 48 83 c4 68                 	add	rsp, 0x68
14098a85f: 5b                          	pop	rbx
14098a860: 5f                          	pop	rdi
14098a861: 5e                          	pop	rsi
14098a862: 41 5c                       	pop	r12
14098a864: 41 5d                       	pop	r13
14098a866: 41 5e                       	pop	r14
14098a868: 41 5f                       	pop	r15
14098a86a: 5d                          	pop	rbp
14098a86b: c3                          	ret
14098a86c: 0f 1f 40 00                 	nop	dword ptr [rax]
14098a870: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14098a875: 55                          	push	rbp
14098a876: 41 57                       	push	r15
14098a878: 41 56                       	push	r14
14098a87a: 41 55                       	push	r13
14098a87c: 41 54                       	push	r12
14098a87e: 56                          	push	rsi
14098a87f: 57                          	push	rdi
14098a880: 53                          	push	rbx
14098a881: 48 83 ec 68                 	sub	rsp, 0x68
14098a885: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14098a88c: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
14098a891: 80 bd e6 04 00 00 08        	cmp	byte ptr [rbp + 0x4e6], 0x8
14098a898: 74 0c                       	je	0x14098a8a6 <.text+0x9898a6>
14098a89a: 48 8d 8d d0 04 00 00        	lea	rcx, [rbp + 0x4d0]
14098a8a1: e8 0a 7c 25 00              	call	0x140be24b0 <.text+0xbe14b0>
14098a8a6: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
14098a8ab: 48 83 c4 68                 	add	rsp, 0x68
14098a8af: 5b                          	pop	rbx
14098a8b0: 5f                          	pop	rdi
14098a8b1: 5e                          	pop	rsi
14098a8b2: 41 5c                       	pop	r12
14098a8b4: 41 5d                       	pop	r13
14098a8b6: 41 5e                       	pop	r14
14098a8b8: 41 5f                       	pop	r15
14098a8ba: 5d                          	pop	rbp
14098a8bb: c3                          	ret
14098a8bc: 0f 1f 40 00                 	nop	dword ptr [rax]
14098a8c0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14098a8c5: 55                          	push	rbp
14098a8c6: 41 57                       	push	r15
14098a8c8: 41 56                       	push	r14
14098a8ca: 41 55                       	push	r13
14098a8cc: 41 54                       	push	r12
14098a8ce: 56                          	push	rsi
14098a8cf: 57                          	push	rdi
14098a8d0: 53                          	push	rbx
14098a8d1: 48 83 ec 68                 	sub	rsp, 0x68
14098a8d5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14098a8dc: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
14098a8e1: 80 bd 56 03 00 00 08        	cmp	byte ptr [rbp + 0x356], 0x8
14098a8e8: 74 0c                       	je	0x14098a8f6 <.text+0x9898f6>
14098a8ea: 48 8d 8d 40 03 00 00        	lea	rcx, [rbp + 0x340]
14098a8f1: e8 ba 7b 25 00              	call	0x140be24b0 <.text+0xbe14b0>
14098a8f6: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
14098a8fb: 48 83 c4 68                 	add	rsp, 0x68
14098a8ff: 5b                          	pop	rbx
14098a900: 5f                          	pop	rdi
14098a901: 5e                          	pop	rsi
14098a902: 41 5c                       	pop	r12
14098a904: 41 5d                       	pop	r13
14098a906: 41 5e                       	pop	r14
14098a908: 41 5f                       	pop	r15
14098a90a: 5d                          	pop	rbp
14098a90b: c3                          	ret
14098a90c: 0f 1f 40 00                 	nop	dword ptr [rax]
14098a910: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14098a915: 55                          	push	rbp
14098a916: 41 57                       	push	r15
14098a918: 41 56                       	push	r14
14098a91a: 41 55                       	push	r13
14098a91c: 41 54                       	push	r12
14098a91e: 56                          	push	rsi
14098a91f: 57                          	push	rdi
14098a920: 53                          	push	rbx
14098a921: 48 83 ec 68                 	sub	rsp, 0x68
14098a925: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14098a92c: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
14098a931: 80 bd e6 04 00 00 08        	cmp	byte ptr [rbp + 0x4e6], 0x8
14098a938: 74 0c                       	je	0x14098a946 <.text+0x989946>
14098a93a: 48 8d 8d d0 04 00 00        	lea	rcx, [rbp + 0x4d0]
14098a941: e8 6a 7b 25 00              	call	0x140be24b0 <.text+0xbe14b0>
14098a946: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
14098a94b: 48 83 c4 68                 	add	rsp, 0x68
14098a94f: 5b                          	pop	rbx
14098a950: 5f                          	pop	rdi
14098a951: 5e                          	pop	rsi
14098a952: 41 5c                       	pop	r12
14098a954: 41 5d                       	pop	r13
14098a956: 41 5e                       	pop	r14
14098a958: 41 5f                       	pop	r15
14098a95a: 5d                          	pop	rbp
14098a95b: c3                          	ret
14098a95c: 0f 1f 40 00                 	nop	dword ptr [rax]
14098a960: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14098a965: 55                          	push	rbp
14098a966: 41 57                       	push	r15
14098a968: 41 56                       	push	r14
14098a96a: 41 55                       	push	r13
14098a96c: 41 54                       	push	r12
14098a96e: 56                          	push	rsi
14098a96f: 57                          	push	rdi
14098a970: 53                          	push	rbx
14098a971: 48 83 ec 68                 	sub	rsp, 0x68
14098a975: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14098a97c: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
14098a981: 48 8b 8d 88 05 00 00        	mov	rcx, qword ptr [rbp + 0x588]
14098a988: 48 8b 95 90 05 00 00        	mov	rdx, qword ptr [rbp + 0x590]
14098a98f: e8 6c 3f 6a ff              	call	0x14002e900 <.text+0x2d900>
14098a994: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
14098a999: 48 83 c4 68                 	add	rsp, 0x68
14098a99d: 5b                          	pop	rbx
14098a99e: 5f                          	pop	rdi
14098a99f: 5e                          	pop	rsi
14098a9a0: 41 5c                       	pop	r12
14098a9a2: 41 5d                       	pop	r13
14098a9a4: 41 5e                       	pop	r14
14098a9a6: 41 5f                       	pop	r15
14098a9a8: 5d                          	pop	rbp
14098a9a9: c3                          	ret
14098a9aa: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
14098a9b0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14098a9b5: 55                          	push	rbp
14098a9b6: 41 57                       	push	r15
14098a9b8: 41 56                       	push	r14
14098a9ba: 41 55                       	push	r13
14098a9bc: 41 54                       	push	r12
14098a9be: 56                          	push	rsi
14098a9bf: 57                          	push	rdi
14098a9c0: 53                          	push	rbx
14098a9c1: 48 83 ec 68                 	sub	rsp, 0x68
14098a9c5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14098a9cc: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
14098a9d1: 48 8d 8d b0 03 00 00        	lea	rcx, [rbp + 0x3b0]
14098a9d8: e8 d3 c5 f0 ff              	call	0x140896fb0 <.text+0x895fb0>
14098a9dd: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
14098a9e2: 48 83 c4 68                 	add	rsp, 0x68
14098a9e6: 5b                          	pop	rbx
14098a9e7: 5f                          	pop	rdi
14098a9e8: 5e                          	pop	rsi
14098a9e9: 41 5c                       	pop	r12
14098a9eb: 41 5d                       	pop	r13
14098a9ed: 41 5e                       	pop	r14
14098a9ef: 41 5f                       	pop	r15
14098a9f1: 5d                          	pop	rbp
14098a9f2: c3                          	ret
14098a9f3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
14098aa00: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14098aa05: 55                          	push	rbp
14098aa06: 41 57                       	push	r15
14098aa08: 41 56                       	push	r14
14098aa0a: 41 55                       	push	r13
14098aa0c: 41 54                       	push	r12
14098aa0e: 56                          	push	rsi
14098aa0f: 57                          	push	rdi
14098aa10: 53                          	push	rbx
14098aa11: 48 83 ec 68                 	sub	rsp, 0x68
14098aa15: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14098aa1c: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
14098aa21: 48 8d 8d b0 03 00 00        	lea	rcx, [rbp + 0x3b0]
14098aa28: e8 83 c5 f0 ff              	call	0x140896fb0 <.text+0x895fb0>
14098aa2d: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
14098aa32: 48 83 c4 68                 	add	rsp, 0x68
14098aa36: 5b                          	pop	rbx
14098aa37: 5f                          	pop	rdi
14098aa38: 5e                          	pop	rsi
14098aa39: 41 5c                       	pop	r12
14098aa3b: 41 5d                       	pop	r13
14098aa3d: 41 5e                       	pop	r14
14098aa3f: 41 5f                       	pop	r15
14098aa41: 5d                          	pop	rbp
14098aa42: c3                          	ret
14098aa43: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
14098aa50: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14098aa55: 55                          	push	rbp
14098aa56: 41 57                       	push	r15
14098aa58: 41 56                       	push	r14
14098aa5a: 41 55                       	push	r13
14098aa5c: 41 54                       	push	r12
14098aa5e: 56                          	push	rsi
14098aa5f: 57                          	push	rdi
14098aa60: 53                          	push	rbx
14098aa61: 48 83 ec 68                 	sub	rsp, 0x68
14098aa65: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14098aa6c: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
14098aa71: 48 8d 8d b0 03 00 00        	lea	rcx, [rbp + 0x3b0]
14098aa78: e8 33 c5 f0 ff              	call	0x140896fb0 <.text+0x895fb0>
14098aa7d: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
14098aa82: 48 83 c4 68                 	add	rsp, 0x68
14098aa86: 5b                          	pop	rbx
14098aa87: 5f                          	pop	rdi
14098aa88: 5e                          	pop	rsi
14098aa89: 41 5c                       	pop	r12
14098aa8b: 41 5d                       	pop	r13
14098aa8d: 41 5e                       	pop	r14
14098aa8f: 41 5f                       	pop	r15
14098aa91: 5d                          	pop	rbp
14098aa92: c3                          	ret
14098aa93: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
14098aaa0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14098aaa5: 55                          	push	rbp
14098aaa6: 41 57                       	push	r15
14098aaa8: 41 56                       	push	r14
14098aaaa: 41 55                       	push	r13
14098aaac: 41 54                       	push	r12
14098aaae: 56                          	push	rsi
14098aaaf: 57                          	push	rdi
14098aab0: 53                          	push	rbx
14098aab1: 48 83 ec 68                 	sub	rsp, 0x68
14098aab5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14098aabc: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
14098aac1: 48 8d 8d 20 02 00 00        	lea	rcx, [rbp + 0x220]
14098aac8: e8 e3 c4 f0 ff              	call	0x140896fb0 <.text+0x895fb0>
14098aacd: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
14098aad2: 48 83 c4 68                 	add	rsp, 0x68
14098aad6: 5b                          	pop	rbx
14098aad7: 5f                          	pop	rdi
14098aad8: 5e                          	pop	rsi
14098aad9: 41 5c                       	pop	r12
14098aadb: 41 5d                       	pop	r13
14098aadd: 41 5e                       	pop	r14
14098aadf: 41 5f                       	pop	r15
14098aae1: 5d                          	pop	rbp
14098aae2: c3                          	ret
