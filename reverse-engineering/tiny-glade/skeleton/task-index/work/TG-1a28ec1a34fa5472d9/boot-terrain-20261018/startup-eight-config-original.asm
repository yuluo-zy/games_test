
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14099b8b0: 55                          	push	rbp
14099b8b1: 56                          	push	rsi
14099b8b2: 57                          	push	rdi
14099b8b3: 53                          	push	rbx
14099b8b4: 48 81 ec b8 04 00 00        	sub	rsp, 0x4b8
14099b8bb: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
14099b8c3: 48 c7 85 30 04 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x430], -0x2
14099b8ce: 48 89 ce                    	mov	rsi, rcx
14099b8d1: 0f b6 05 31 93 c9 02        	movzx	eax, byte ptr [rip + 0x2c99331] # 0x143634c09
14099b8d8: b9 80 03 00 00              	mov	ecx, 0x380
14099b8dd: ba 08 00 00 00              	mov	edx, 0x8
14099b8e2: e8 29 83 c7 ff              	call	0x140613c10 <.text+0x612c10>
14099b8e7: 48 85 c0                    	test	rax, rax
14099b8ea: 0f 84 f1 04 00 00           	je	0x14099bde1 <.text+0x99ade1>
14099b8f0: 48 89 85 28 04 00 00        	mov	qword ptr [rbp + 0x428], rax
14099b8f7: 0f 57 c0                    	xorps	xmm0, xmm0
14099b8fa: 0f 11 85 08 03 00 00        	movups	xmmword ptr [rbp + 0x308], xmm0
14099b901: 0f 11 85 20 03 00 00        	movups	xmmword ptr [rbp + 0x320], xmm0
14099b908: 0f 11 85 38 03 00 00        	movups	xmmword ptr [rbp + 0x338], xmm0
14099b90f: 0f 11 85 50 03 00 00        	movups	xmmword ptr [rbp + 0x350], xmm0
14099b916: 0f 11 85 68 03 00 00        	movups	xmmword ptr [rbp + 0x368], xmm0
14099b91d: c7 85 78 03 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x378], 0x0
14099b927: 0f 11 85 a0 03 00 00        	movups	xmmword ptr [rbp + 0x3a0], xmm0
14099b92e: 0f 11 85 b8 03 00 00        	movups	xmmword ptr [rbp + 0x3b8], xmm0
14099b935: 0f 11 85 d0 03 00 00        	movups	xmmword ptr [rbp + 0x3d0], xmm0
14099b93c: 0f 11 85 e8 03 00 00        	movups	xmmword ptr [rbp + 0x3e8], xmm0
14099b943: 0f 11 85 00 04 00 00        	movups	xmmword ptr [rbp + 0x400], xmm0
14099b94a: c7 85 10 04 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x410], 0x0
14099b954: 48 c7 85 b0 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x2b0], 0x0
14099b95f: 48 c7 85 e8 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x2e8], 0x0
14099b96a: 48 c7 85 f0 02 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x2f0], 0x8
14099b975: 48 c7 85 f8 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x2f8], 0x0
14099b980: 48 c7 85 00 03 00 00 10 00 00 00    	mov	qword ptr [rbp + 0x300], 0x10
14099b98b: 48 c7 85 18 03 00 00 10 00 00 00    	mov	qword ptr [rbp + 0x318], 0x10
14099b996: 48 c7 85 30 03 00 00 10 00 00 00    	mov	qword ptr [rbp + 0x330], 0x10
14099b9a1: 48 c7 85 48 03 00 00 10 00 00 00    	mov	qword ptr [rbp + 0x348], 0x10
14099b9ac: 48 c7 85 60 03 00 00 10 00 00 00    	mov	qword ptr [rbp + 0x360], 0x10
14099b9b7: 48 bb 00 00 00 00 00 00 00 80       	movabs	rbx, -0x8000000000000000
14099b9c1: 48 89 9d 80 03 00 00        	mov	qword ptr [rbp + 0x380], rbx
14099b9c8: 48 8d 05 d9 00 12 02        	lea	rax, [rip + 0x21200d9]  # 0x142abbaa8
14099b9cf: 48 89 85 88 03 00 00        	mov	qword ptr [rbp + 0x388], rax
14099b9d6: 48 c7 85 90 03 00 00 45 00 00 00    	mov	qword ptr [rbp + 0x390], 0x45
14099b9e1: 48 c7 85 98 03 00 00 10 00 00 00    	mov	qword ptr [rbp + 0x398], 0x10
14099b9ec: 48 c7 85 b0 03 00 00 10 00 00 00    	mov	qword ptr [rbp + 0x3b0], 0x10
14099b9f7: 48 c7 85 c8 03 00 00 10 00 00 00    	mov	qword ptr [rbp + 0x3c8], 0x10
14099ba02: 48 c7 85 e0 03 00 00 10 00 00 00    	mov	qword ptr [rbp + 0x3e0], 0x10
14099ba0d: 48 c7 85 f8 03 00 00 10 00 00 00    	mov	qword ptr [rbp + 0x3f8], 0x10
14099ba18: c7 85 18 04 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x418], 0x0
14099ba22: 66 c7 85 1c 04 00 00 01 00  	mov	word ptr [rbp + 0x41c], 0x1
14099ba2b: c7 85 20 04 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x420], 0x0
14099ba35: 0f b6 05 cd 91 c9 02        	movzx	eax, byte ptr [rip + 0x2c991cd] # 0x143634c09
14099ba3c: b9 78 01 00 00              	mov	ecx, 0x178
14099ba41: ba 08 00 00 00              	mov	edx, 0x8
14099ba46: e8 c5 81 c7 ff              	call	0x140613c10 <.text+0x612c10>
14099ba4b: 48 85 c0                    	test	rax, rax
14099ba4e: 0f 84 9c 03 00 00           	je	0x14099bdf0 <.text+0x99adf0>
14099ba54: 48 89 c7                    	mov	rdi, rax
14099ba57: 48 8d 95 b0 02 00 00        	lea	rdx, [rbp + 0x2b0]
14099ba5e: 41 b8 78 01 00 00           	mov	r8d, 0x178
14099ba64: 48 89 c1                    	mov	rcx, rax
14099ba67: e8 94 81 f8 01              	call	0x142923c00 <.text+0x2922c00>
14099ba6c: 4c 8d 05 cd fc 11 02        	lea	r8, [rip + 0x211fccd]   # 0x142abb740
14099ba73: 48 8d 8d 40 02 00 00        	lea	rcx, [rbp + 0x240]
14099ba7a: 48 89 fa                    	mov	rdx, rdi
14099ba7d: e8 8e 90 34 00              	call	0x140ce4b10 <.text+0xce3b10>
14099ba82: 48 8d 8d d0 01 00 00        	lea	rcx, [rbp + 0x1d0]
14099ba89: e8 72 57 0d 00              	call	0x140a71200 <.text+0xa70200>
14099ba8e: 48 8d 8d 60 01 00 00        	lea	rcx, [rbp + 0x160]
14099ba95: e8 96 63 07 00              	call	0x140a11e30 <.text+0xa10e30>
14099ba9a: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
14099baa1: e8 3a 96 dc 00              	call	0x1417650e0 <.text+0x17640e0>
14099baa6: 48 8d 8d 80 00 00 00        	lea	rcx, [rbp + 0x80]
14099baad: e8 be e8 dc 00              	call	0x14176a370 <.text+0x1769370>
14099bab2: 48 8d 4d 10                 	lea	rcx, [rbp + 0x10]
14099bab6: e8 55 0b 0b 00              	call	0x140a4c610 <.text+0xa4b610>
14099babb: 48 8d 4d a0                 	lea	rcx, [rbp - 0x60]
14099babf: e8 4c f6 cb 00              	call	0x14165b110 <.text+0x165a110>
14099bac4: 48 8d 8d b0 02 00 00        	lea	rcx, [rbp + 0x2b0]
14099bacb: e8 20 96 f2 ff              	call	0x1408c50f0 <.text+0x8c40f0>
14099bad0: 0f 10 85 a0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2a0]
14099bad7: 48 8b 85 28 04 00 00        	mov	rax, qword ptr [rbp + 0x428]
14099bade: 0f 11 40 60                 	movups	xmmword ptr [rax + 0x60], xmm0
14099bae2: 0f 10 85 90 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x290]
14099bae9: 0f 11 40 50                 	movups	xmmword ptr [rax + 0x50], xmm0
14099baed: 0f 10 85 80 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x280]
14099baf4: 0f 11 40 40                 	movups	xmmword ptr [rax + 0x40], xmm0
14099baf8: 0f 10 85 40 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x240]
14099baff: 0f 10 8d 50 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x250]
14099bb06: 0f 10 95 60 02 00 00        	movups	xmm2, xmmword ptr [rbp + 0x260]
14099bb0d: 0f 10 9d 70 02 00 00        	movups	xmm3, xmmword ptr [rbp + 0x270]
14099bb14: 0f 11 58 30                 	movups	xmmword ptr [rax + 0x30], xmm3
14099bb18: 0f 11 50 20                 	movups	xmmword ptr [rax + 0x20], xmm2
14099bb1c: 0f 11 48 10                 	movups	xmmword ptr [rax + 0x10], xmm1
14099bb20: 0f 11 00                    	movups	xmmword ptr [rax], xmm0
14099bb23: 0f 10 85 d0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1d0]
14099bb2a: 0f 10 8d e0 01 00 00        	movups	xmm1, xmmword ptr [rbp + 0x1e0]
14099bb31: 0f 10 95 f0 01 00 00        	movups	xmm2, xmmword ptr [rbp + 0x1f0]
14099bb38: 0f 10 9d 00 02 00 00        	movups	xmm3, xmmword ptr [rbp + 0x200]
14099bb3f: 0f 11 40 70                 	movups	xmmword ptr [rax + 0x70], xmm0
14099bb43: 0f 11 88 80 00 00 00        	movups	xmmword ptr [rax + 0x80], xmm1
14099bb4a: 0f 11 90 90 00 00 00        	movups	xmmword ptr [rax + 0x90], xmm2
14099bb51: 0f 11 98 a0 00 00 00        	movups	xmmword ptr [rax + 0xa0], xmm3
14099bb58: 0f 10 85 10 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x210]
14099bb5f: 0f 11 80 b0 00 00 00        	movups	xmmword ptr [rax + 0xb0], xmm0
14099bb66: 0f 10 85 20 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x220]
14099bb6d: 0f 11 80 c0 00 00 00        	movups	xmmword ptr [rax + 0xc0], xmm0
14099bb74: 0f 10 85 30 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x230]
14099bb7b: 0f 11 80 d0 00 00 00        	movups	xmmword ptr [rax + 0xd0], xmm0
14099bb82: 0f 10 85 c0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1c0]
14099bb89: 0f 11 80 40 01 00 00        	movups	xmmword ptr [rax + 0x140], xmm0
14099bb90: 0f 10 85 b0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1b0]
14099bb97: 0f 11 80 30 01 00 00        	movups	xmmword ptr [rax + 0x130], xmm0
14099bb9e: 0f 10 85 a0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1a0]
14099bba5: 0f 11 80 20 01 00 00        	movups	xmmword ptr [rax + 0x120], xmm0
14099bbac: 0f 10 85 60 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x160]
14099bbb3: 0f 10 8d 70 01 00 00        	movups	xmm1, xmmword ptr [rbp + 0x170]
14099bbba: 0f 10 95 80 01 00 00        	movups	xmm2, xmmword ptr [rbp + 0x180]
14099bbc1: 0f 10 9d 90 01 00 00        	movups	xmm3, xmmword ptr [rbp + 0x190]
14099bbc8: 0f 11 98 10 01 00 00        	movups	xmmword ptr [rax + 0x110], xmm3
14099bbcf: 0f 11 90 00 01 00 00        	movups	xmmword ptr [rax + 0x100], xmm2
14099bbd6: 0f 11 88 f0 00 00 00        	movups	xmmword ptr [rax + 0xf0], xmm1
14099bbdd: 0f 11 80 e0 00 00 00        	movups	xmmword ptr [rax + 0xe0], xmm0
14099bbe4: 0f 10 85 f0 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xf0]
14099bbeb: 0f 10 8d 00 01 00 00        	movups	xmm1, xmmword ptr [rbp + 0x100]
14099bbf2: 0f 10 95 10 01 00 00        	movups	xmm2, xmmword ptr [rbp + 0x110]
14099bbf9: 0f 10 9d 20 01 00 00        	movups	xmm3, xmmword ptr [rbp + 0x120]
14099bc00: 0f 11 80 50 01 00 00        	movups	xmmword ptr [rax + 0x150], xmm0
14099bc07: 0f 11 88 60 01 00 00        	movups	xmmword ptr [rax + 0x160], xmm1
14099bc0e: 0f 11 90 70 01 00 00        	movups	xmmword ptr [rax + 0x170], xmm2
14099bc15: 0f 11 98 80 01 00 00        	movups	xmmword ptr [rax + 0x180], xmm3
14099bc1c: 0f 10 85 30 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x130]
14099bc23: 0f 11 80 90 01 00 00        	movups	xmmword ptr [rax + 0x190], xmm0
14099bc2a: 0f 10 85 40 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x140]
14099bc31: 0f 11 80 a0 01 00 00        	movups	xmmword ptr [rax + 0x1a0], xmm0
14099bc38: 0f 10 85 50 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x150]
14099bc3f: 0f 11 80 b0 01 00 00        	movups	xmmword ptr [rax + 0x1b0], xmm0
14099bc46: 0f 10 85 80 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0x80]
14099bc4d: 0f 10 8d 90 00 00 00        	movups	xmm1, xmmword ptr [rbp + 0x90]
14099bc54: 0f 10 95 a0 00 00 00        	movups	xmm2, xmmword ptr [rbp + 0xa0]
14099bc5b: 0f 10 9d b0 00 00 00        	movups	xmm3, xmmword ptr [rbp + 0xb0]
14099bc62: 0f 11 80 c0 01 00 00        	movups	xmmword ptr [rax + 0x1c0], xmm0
14099bc69: 0f 11 88 d0 01 00 00        	movups	xmmword ptr [rax + 0x1d0], xmm1
14099bc70: 0f 11 90 e0 01 00 00        	movups	xmmword ptr [rax + 0x1e0], xmm2
14099bc77: 0f 11 98 f0 01 00 00        	movups	xmmword ptr [rax + 0x1f0], xmm3
14099bc7e: 0f 10 85 c0 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xc0]
14099bc85: 0f 11 80 00 02 00 00        	movups	xmmword ptr [rax + 0x200], xmm0
14099bc8c: 0f 10 85 e0 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xe0]
14099bc93: 0f 11 80 20 02 00 00        	movups	xmmword ptr [rax + 0x220], xmm0
14099bc9a: 0f 10 85 d0 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xd0]
14099bca1: 0f 11 80 10 02 00 00        	movups	xmmword ptr [rax + 0x210], xmm0
14099bca8: 0f 10 45 70                 	movups	xmm0, xmmword ptr [rbp + 0x70]
14099bcac: 0f 11 80 90 02 00 00        	movups	xmmword ptr [rax + 0x290], xmm0
14099bcb3: 0f 10 45 60                 	movups	xmm0, xmmword ptr [rbp + 0x60]
14099bcb7: 0f 11 80 80 02 00 00        	movups	xmmword ptr [rax + 0x280], xmm0
14099bcbe: 0f 10 45 50                 	movups	xmm0, xmmword ptr [rbp + 0x50]
14099bcc2: 0f 11 80 70 02 00 00        	movups	xmmword ptr [rax + 0x270], xmm0
14099bcc9: 0f 10 45 10                 	movups	xmm0, xmmword ptr [rbp + 0x10]
14099bccd: 0f 10 4d 20                 	movups	xmm1, xmmword ptr [rbp + 0x20]
14099bcd1: 0f 10 55 30                 	movups	xmm2, xmmword ptr [rbp + 0x30]
14099bcd5: 0f 10 5d 40                 	movups	xmm3, xmmword ptr [rbp + 0x40]
14099bcd9: 0f 11 98 60 02 00 00        	movups	xmmword ptr [rax + 0x260], xmm3
14099bce0: 0f 11 90 50 02 00 00        	movups	xmmword ptr [rax + 0x250], xmm2
14099bce7: 0f 11 88 40 02 00 00        	movups	xmmword ptr [rax + 0x240], xmm1
14099bcee: 0f 11 80 30 02 00 00        	movups	xmmword ptr [rax + 0x230], xmm0
14099bcf5: 0f 10 45 a0                 	movups	xmm0, xmmword ptr [rbp - 0x60]
14099bcf9: 0f 10 4d b0                 	movups	xmm1, xmmword ptr [rbp - 0x50]
14099bcfd: 0f 10 55 c0                 	movups	xmm2, xmmword ptr [rbp - 0x40]
14099bd01: 0f 10 5d d0                 	movups	xmm3, xmmword ptr [rbp - 0x30]
14099bd05: 0f 11 80 a0 02 00 00        	movups	xmmword ptr [rax + 0x2a0], xmm0
14099bd0c: 0f 11 88 b0 02 00 00        	movups	xmmword ptr [rax + 0x2b0], xmm1
14099bd13: 0f 11 90 c0 02 00 00        	movups	xmmword ptr [rax + 0x2c0], xmm2
14099bd1a: 0f 11 98 d0 02 00 00        	movups	xmmword ptr [rax + 0x2d0], xmm3
14099bd21: 0f 10 45 e0                 	movups	xmm0, xmmword ptr [rbp - 0x20]
14099bd25: 0f 11 80 e0 02 00 00        	movups	xmmword ptr [rax + 0x2e0], xmm0
14099bd2c: 0f 10 45 f0                 	movups	xmm0, xmmword ptr [rbp - 0x10]
14099bd30: 0f 11 80 f0 02 00 00        	movups	xmmword ptr [rax + 0x2f0], xmm0
14099bd37: 0f 10 45 00                 	movups	xmm0, xmmword ptr [rbp]
14099bd3b: 0f 11 80 00 03 00 00        	movups	xmmword ptr [rax + 0x300], xmm0
14099bd42: 0f 10 85 b0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2b0]
14099bd49: 0f 10 8d c0 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x2c0]
14099bd50: 0f 10 95 d0 02 00 00        	movups	xmm2, xmmword ptr [rbp + 0x2d0]
14099bd57: 0f 10 9d e0 02 00 00        	movups	xmm3, xmmword ptr [rbp + 0x2e0]
14099bd5e: 0f 11 80 10 03 00 00        	movups	xmmword ptr [rax + 0x310], xmm0
14099bd65: 0f 11 88 20 03 00 00        	movups	xmmword ptr [rax + 0x320], xmm1
14099bd6c: 0f 11 90 30 03 00 00        	movups	xmmword ptr [rax + 0x330], xmm2
14099bd73: 0f 11 98 40 03 00 00        	movups	xmmword ptr [rax + 0x340], xmm3
14099bd7a: 0f 10 85 f0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2f0]
14099bd81: 0f 11 80 50 03 00 00        	movups	xmmword ptr [rax + 0x350], xmm0
14099bd88: 0f 10 85 10 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x310]
14099bd8f: 0f 11 80 70 03 00 00        	movups	xmmword ptr [rax + 0x370], xmm0
14099bd96: 0f 10 85 00 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x300]
14099bd9d: 0f 11 80 60 03 00 00        	movups	xmmword ptr [rax + 0x360], xmm0
14099bda4: 48 c7 46 08 08 00 00 00     	mov	qword ptr [rsi + 0x8], 0x8
14099bdac: 48 89 46 10                 	mov	qword ptr [rsi + 0x10], rax
14099bdb0: 48 c7 46 18 08 00 00 00     	mov	qword ptr [rsi + 0x18], 0x8
14099bdb8: 48 c7 46 20 00 00 00 00     	mov	qword ptr [rsi + 0x20], 0x0
14099bdc0: 48 c7 46 28 08 00 00 00     	mov	qword ptr [rsi + 0x28], 0x8
14099bdc8: 0f 57 c0                    	xorps	xmm0, xmm0
14099bdcb: 0f 11 46 30                 	movups	xmmword ptr [rsi + 0x30], xmm0
14099bdcf: 48 89 1e                    	mov	qword ptr [rsi], rbx
14099bdd2: 48 89 f0                    	mov	rax, rsi
14099bdd5: 48 81 c4 b8 04 00 00        	add	rsp, 0x4b8
14099bddc: 5b                          	pop	rbx
14099bddd: 5f                          	pop	rdi
14099bdde: 5e                          	pop	rsi
14099bddf: 5d                          	pop	rbp
14099bde0: c3                          	ret
14099bde1: b9 08 00 00 00              	mov	ecx, 0x8
14099bde6: ba 80 03 00 00              	mov	edx, 0x380
14099bdeb: e8 10 d2 f3 01              	call	0x1428d9000 <.text+0x28d8000>
14099bdf0: b9 08 00 00 00              	mov	ecx, 0x8
14099bdf5: ba 78 01 00 00              	mov	edx, 0x178
14099bdfa: e8 01 d2 f3 01              	call	0x1428d9000 <.text+0x28d8000>
14099bdff: 0f 0b                       	ud2
14099be01: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
14099be10: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14099be15: 55                          	push	rbp
14099be16: 56                          	push	rsi
14099be17: 57                          	push	rdi
14099be18: 53                          	push	rbx
14099be19: 48 83 ec 28                 	sub	rsp, 0x28
14099be1d: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14099be24: 48 8d 8d b0 02 00 00        	lea	rcx, [rbp + 0x2b0]
14099be2b: e8 c0 dc c4 ff              	call	0x1405e9af0 <.text+0x5e8af0>
14099be30: 90                          	nop
14099be31: 48 83 c4 28                 	add	rsp, 0x28
14099be35: 5b                          	pop	rbx
14099be36: 5f                          	pop	rdi
14099be37: 5e                          	pop	rsi
14099be38: 5d                          	pop	rbp
14099be39: c3                          	ret
14099be3a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
14099be40: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14099be45: 55                          	push	rbp
14099be46: 56                          	push	rsi
14099be47: 57                          	push	rdi
14099be48: 53                          	push	rbx
14099be49: 48 83 ec 28                 	sub	rsp, 0x28
14099be4d: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14099be54: ba 80 03 00 00              	mov	edx, 0x380
14099be59: 41 b8 08 00 00 00           	mov	r8d, 0x8
14099be5f: 48 8b 8d 28 04 00 00        	mov	rcx, qword ptr [rbp + 0x428]
14099be66: e8 b5 7d c7 ff              	call	0x140613c20 <.text+0x612c20>
14099be6b: 90                          	nop
14099be6c: 48 83 c4 28                 	add	rsp, 0x28
14099be70: 5b                          	pop	rbx
14099be71: 5f                          	pop	rdi
14099be72: 5e                          	pop	rsi
14099be73: 5d                          	pop	rbp
14099be74: c3                          	ret
14099be75: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
14099be80: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14099be85: 55                          	push	rbp
14099be86: 56                          	push	rsi
14099be87: 57                          	push	rdi
14099be88: 53                          	push	rbx
14099be89: 48 83 ec 28                 	sub	rsp, 0x28
14099be8d: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14099be94: 48 8d 8d 40 02 00 00        	lea	rcx, [rbp + 0x240]
14099be9b: e8 e0 d9 68 ff              	call	0x140029880 <.text+0x28880>
14099bea0: 90                          	nop
14099bea1: 48 83 c4 28                 	add	rsp, 0x28
14099bea5: 5b                          	pop	rbx
14099bea6: 5f                          	pop	rdi
14099bea7: 5e                          	pop	rsi
14099bea8: 5d                          	pop	rbp
14099bea9: c3                          	ret
14099beaa: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
14099beb0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14099beb5: 55                          	push	rbp
14099beb6: 56                          	push	rsi
14099beb7: 57                          	push	rdi
14099beb8: 53                          	push	rbx
14099beb9: 48 83 ec 28                 	sub	rsp, 0x28
14099bebd: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14099bec4: 48 8d 8d d0 01 00 00        	lea	rcx, [rbp + 0x1d0]
14099becb: e8 b0 d9 68 ff              	call	0x140029880 <.text+0x28880>
14099bed0: 90                          	nop
14099bed1: 48 83 c4 28                 	add	rsp, 0x28
14099bed5: 5b                          	pop	rbx
14099bed6: 5f                          	pop	rdi
14099bed7: 5e                          	pop	rsi
14099bed8: 5d                          	pop	rbp
14099bed9: c3                          	ret
14099beda: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
14099bee0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14099bee5: 55                          	push	rbp
14099bee6: 56                          	push	rsi
14099bee7: 57                          	push	rdi
14099bee8: 53                          	push	rbx
14099bee9: 48 83 ec 28                 	sub	rsp, 0x28
14099beed: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14099bef4: 48 8d 8d 60 01 00 00        	lea	rcx, [rbp + 0x160]
14099befb: e8 80 d9 68 ff              	call	0x140029880 <.text+0x28880>
14099bf00: 90                          	nop
14099bf01: 48 83 c4 28                 	add	rsp, 0x28
14099bf05: 5b                          	pop	rbx
14099bf06: 5f                          	pop	rdi
14099bf07: 5e                          	pop	rsi
14099bf08: 5d                          	pop	rbp
14099bf09: c3                          	ret
14099bf0a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
14099bf10: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14099bf15: 55                          	push	rbp
14099bf16: 56                          	push	rsi
14099bf17: 57                          	push	rdi
14099bf18: 53                          	push	rbx
14099bf19: 48 83 ec 28                 	sub	rsp, 0x28
14099bf1d: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14099bf24: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
14099bf2b: e8 50 d9 68 ff              	call	0x140029880 <.text+0x28880>
14099bf30: 90                          	nop
14099bf31: 48 83 c4 28                 	add	rsp, 0x28
14099bf35: 5b                          	pop	rbx
14099bf36: 5f                          	pop	rdi
14099bf37: 5e                          	pop	rsi
14099bf38: 5d                          	pop	rbp
14099bf39: c3                          	ret
14099bf3a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
14099bf40: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14099bf45: 55                          	push	rbp
14099bf46: 56                          	push	rsi
14099bf47: 57                          	push	rdi
14099bf48: 53                          	push	rbx
14099bf49: 48 83 ec 28                 	sub	rsp, 0x28
14099bf4d: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14099bf54: 48 8d 8d 80 00 00 00        	lea	rcx, [rbp + 0x80]
14099bf5b: e8 20 d9 68 ff              	call	0x140029880 <.text+0x28880>
14099bf60: 90                          	nop
14099bf61: 48 83 c4 28                 	add	rsp, 0x28
14099bf65: 5b                          	pop	rbx
14099bf66: 5f                          	pop	rdi
14099bf67: 5e                          	pop	rsi
14099bf68: 5d                          	pop	rbp
14099bf69: c3                          	ret
14099bf6a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
14099bf70: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14099bf75: 55                          	push	rbp
14099bf76: 56                          	push	rsi
14099bf77: 57                          	push	rdi
14099bf78: 53                          	push	rbx
14099bf79: 48 83 ec 28                 	sub	rsp, 0x28
14099bf7d: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14099bf84: 48 8d 4d 10                 	lea	rcx, [rbp + 0x10]
14099bf88: e8 f3 d8 68 ff              	call	0x140029880 <.text+0x28880>
14099bf8d: 90                          	nop
14099bf8e: 48 83 c4 28                 	add	rsp, 0x28
14099bf92: 5b                          	pop	rbx
14099bf93: 5f                          	pop	rdi
14099bf94: 5e                          	pop	rsi
14099bf95: 5d                          	pop	rbp
14099bf96: c3                          	ret
14099bf97: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
14099bfa0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14099bfa5: 55                          	push	rbp
14099bfa6: 56                          	push	rsi
14099bfa7: 57                          	push	rdi
14099bfa8: 53                          	push	rbx
14099bfa9: 48 83 ec 28                 	sub	rsp, 0x28
14099bfad: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14099bfb4: 48 8d 4d a0                 	lea	rcx, [rbp - 0x60]
14099bfb8: e8 c3 d8 68 ff              	call	0x140029880 <.text+0x28880>
14099bfbd: 90                          	nop
14099bfbe: 48 83 c4 28                 	add	rsp, 0x28
14099bfc2: 5b                          	pop	rbx
14099bfc3: 5f                          	pop	rdi
14099bfc4: 5e                          	pop	rsi
14099bfc5: 5d                          	pop	rbp
14099bfc6: c3                          	ret
