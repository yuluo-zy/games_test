
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1403db340: 55                          	push	rbp
1403db341: 41 57                       	push	r15
1403db343: 41 56                       	push	r14
1403db345: 41 55                       	push	r13
1403db347: 41 54                       	push	r12
1403db349: 56                          	push	rsi
1403db34a: 57                          	push	rdi
1403db34b: 53                          	push	rbx
1403db34c: 48 81 ec 78 0c 00 00        	sub	rsp, 0xc78
1403db353: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
1403db35b: 0f 29 b5 e0 0b 00 00        	movaps	xmmword ptr [rbp + 0xbe0], xmm6
1403db362: 48 c7 85 d8 0b 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0xbd8], -0x2
1403db36d: e8 6e 82 98 00              	call	0x140d635e0 <.text+0xd625e0>
1403db372: 48 89 05 0f 98 25 03        	mov	qword ptr [rip + 0x325980f], rax # 0x143634b88
1403db379: 89 15 29 fd 24 03           	mov	dword ptr [rip + 0x324fd29], edx # 0x14362b0a8
1403db37f: e8 dc 69 8b 00              	call	0x140c91d60 <.text+0xc90d60>
1403db384: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
1403db38b: e8 30 81 f7 ff              	call	0x1403534c0 <.text+0x3524c0>
1403db390: 4c 8b b5 c8 05 00 00        	mov	r14, qword ptr [rbp + 0x5c8]
1403db397: 4c 89 f0                    	mov	rax, r14
1403db39a: 48 f7 d8                    	neg	rax
1403db39d: 0f 80 f8 00 00 00           	jo	0x1403db49b <.text+0x3da49b>
1403db3a3: 48 8d 85 c8 05 00 00        	lea	rax, [rbp + 0x5c8]
1403db3aa: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
1403db3ad: 0f 10 48 10                 	movups	xmm1, xmmword ptr [rax + 0x10]
1403db3b1: 0f 29 8d 20 06 00 00        	movaps	xmmword ptr [rbp + 0x620], xmm1
1403db3b8: 0f 29 85 10 06 00 00        	movaps	xmmword ptr [rbp + 0x610], xmm0
1403db3bf: 48 8d 8d 10 06 00 00        	lea	rcx, [rbp + 0x610]
1403db3c6: e8 05 bb 99 00              	call	0x140d76ed0 <.text+0xd75ed0>
1403db3cb: 49 89 d0                    	mov	r8, rdx
1403db3ce: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
1403db3d5: 48 89 c2                    	mov	rdx, rax
1403db3d8: e8 d3 da 97 00              	call	0x140d58eb0 <.text+0xd57eb0>
1403db3dd: 48 8b 95 10 06 00 00        	mov	rdx, qword ptr [rbp + 0x610]
1403db3e4: 48 85 d2                    	test	rdx, rdx
1403db3e7: 74 12                       	je	0x1403db3fb <.text+0x3da3fb>
1403db3e9: 48 8b 8d 18 06 00 00        	mov	rcx, qword ptr [rbp + 0x618]
1403db3f0: 41 b8 01 00 00 00           	mov	r8d, 0x1
1403db3f6: e8 25 88 23 00              	call	0x140613c20 <.text+0x612c20>
1403db3fb: 48 8b 85 90 02 00 00        	mov	rax, qword ptr [rbp + 0x290]
1403db402: 48 8b b5 98 02 00 00        	mov	rsi, qword ptr [rbp + 0x298]
1403db409: 48 89 c1                    	mov	rcx, rax
1403db40c: 48 f7 d9                    	neg	rcx
1403db40f: 0f 81 16 02 00 00           	jno	0x1403db62b <.text+0x3da62b>
1403db415: 48 89 75 b0                 	mov	qword ptr [rbp - 0x50], rsi
1403db419: c6 85 b7 0b 00 00 01        	mov	byte ptr [rbp + 0xbb7], 0x1
1403db420: 48 8d 8d 20 09 00 00        	lea	rcx, [rbp + 0x920]
1403db427: e8 d4 b2 97 00              	call	0x140d56700 <.text+0xd55700>
1403db42c: 0f 10 85 20 09 00 00        	movups	xmm0, xmmword ptr [rbp + 0x920]
1403db433: 0f 10 8d 30 09 00 00        	movups	xmm1, xmmword ptr [rbp + 0x930]
1403db43a: 0f 10 95 40 09 00 00        	movups	xmm2, xmmword ptr [rbp + 0x940]
1403db441: 0f 29 95 10 07 00 00        	movaps	xmmword ptr [rbp + 0x710], xmm2
1403db448: 0f 29 8d 00 07 00 00        	movaps	xmmword ptr [rbp + 0x700], xmm1
1403db44f: 0f 29 85 f0 06 00 00        	movaps	xmmword ptr [rbp + 0x6f0], xmm0
1403db456: 48 8d 05 23 86 5f 02        	lea	rax, [rip + 0x25f8623]  # 0x1429d3a80
1403db45d: 48 89 85 20 09 00 00        	mov	qword ptr [rbp + 0x920], rax
1403db464: 48 c7 85 28 09 00 00 12 00 00 00    	mov	qword ptr [rbp + 0x928], 0x12
1403db46f: 48 89 b5 30 09 00 00        	mov	qword ptr [rbp + 0x930], rsi
1403db476: c6 85 b7 0b 00 00 00        	mov	byte ptr [rbp + 0xbb7], 0x0
1403db47d: 48 8d 8d 20 09 00 00        	lea	rcx, [rbp + 0x920]
1403db484: 48 8d 95 f0 06 00 00        	lea	rdx, [rbp + 0x6f0]
1403db48b: e8 a0 80 4c 02              	call	0x1428a3530 <.text+0x28a2530>
1403db490: 48 89 c6                    	mov	rsi, rax
1403db493: 41 b7 01                    	mov	r15b, 0x1
1403db496: e9 41 16 00 00              	jmp	0x1403dcadc <.text+0x3dbadc>
1403db49b: c6 85 c5 0b 00 00 01        	mov	byte ptr [rbp + 0xbc5], 0x1
1403db4a2: c6 85 c4 0b 00 00 01        	mov	byte ptr [rbp + 0xbc4], 0x1
1403db4a9: c6 85 c3 0b 00 00 01        	mov	byte ptr [rbp + 0xbc3], 0x1
1403db4b0: c6 85 c2 0b 00 00 01        	mov	byte ptr [rbp + 0xbc2], 0x1
1403db4b7: 48 8d 8d 20 09 00 00        	lea	rcx, [rbp + 0x920]
1403db4be: e8 8d cb 97 00              	call	0x140d58050 <.text+0xd57050>
1403db4c3: 48 8d b5 20 09 00 00        	lea	rsi, [rbp + 0x920]
1403db4ca: 48 89 f1                    	mov	rcx, rsi
1403db4cd: e8 fe b9 99 00              	call	0x140d76ed0 <.text+0xd75ed0>
1403db4d2: 49 89 d0                    	mov	r8, rdx
1403db4d5: 48 c7 44 24 20 07 00 00 00  	mov	qword ptr [rsp + 0x20], 0x7
1403db4de: 4c 8d 0d 43 07 58 02        	lea	r9, [rip + 0x2580743]   # 0x14295bc28
1403db4e5: 48 8d bd f0 06 00 00        	lea	rdi, [rbp + 0x6f0]
1403db4ec: 48 89 f9                    	mov	rcx, rdi
1403db4ef: 48 89 c2                    	mov	rdx, rax
1403db4f2: e8 19 73 98 00              	call	0x140d62810 <.text+0xd61810>
1403db4f7: 48 8b 85 f0 06 00 00        	mov	rax, qword ptr [rbp + 0x6f0]
1403db4fe: 48 89 85 88 0b 00 00        	mov	qword ptr [rbp + 0xb88], rax
1403db505: 48 8b 85 f8 06 00 00        	mov	rax, qword ptr [rbp + 0x6f8]
1403db50c: 48 89 85 58 0b 00 00        	mov	qword ptr [rbp + 0xb58], rax
1403db513: 0f 10 85 00 07 00 00        	movups	xmm0, xmmword ptr [rbp + 0x700]
1403db51a: 0f 29 85 80 02 00 00        	movaps	xmmword ptr [rbp + 0x280], xmm0
1403db521: 48 8b 95 20 09 00 00        	mov	rdx, qword ptr [rbp + 0x920]
1403db528: 48 85 d2                    	test	rdx, rdx
1403db52b: 74 12                       	je	0x1403db53f <.text+0x3da53f>
1403db52d: 48 8b 8d 28 09 00 00        	mov	rcx, qword ptr [rbp + 0x928]
1403db534: 41 b8 01 00 00 00           	mov	r8d, 0x1
1403db53a: e8 e1 86 23 00              	call	0x140613c20 <.text+0x612c20>
1403db53f: 48 8b 8d f0 04 00 00        	mov	rcx, qword ptr [rbp + 0x4f0]
1403db546: 48 89 c8                    	mov	rax, rcx
1403db549: 48 f7 d8                    	neg	rax
1403db54c: 41 0f 90 c7                 	seto	r15b
1403db550: 0f 81 21 01 00 00           	jno	0x1403db677 <.text+0x3da677>
1403db556: 80 bd 03 06 00 00 00        	cmp	byte ptr [rbp + 0x603], 0x0
1403db55d: 74 61                       	je	0x1403db5c0 <.text+0x3da5c0>
1403db55f: 48 8d 0d 78 89 5f 02        	lea	rcx, [rip + 0x25f8978]  # 0x1429d3ede
1403db566: 48 89 4d b0                 	mov	qword ptr [rbp - 0x50], rcx
1403db56a: 48 c7 45 b8 26 00 00 00     	mov	qword ptr [rbp - 0x48], 0x26
1403db572: 4c 8d 05 64 89 5f 02        	lea	r8, [rip + 0x25f8964]   # 0x1429d3edd
1403db579: 4c 89 85 90 02 00 00        	mov	qword ptr [rbp + 0x290], r8
1403db580: 48 c7 85 98 02 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x298], 0x1
1403db58b: c6 85 d0 0b 00 00 01        	mov	byte ptr [rbp + 0xbd0], 0x1
1403db592: c6 85 cf 0b 00 00 01        	mov	byte ptr [rbp + 0xbcf], 0x1
1403db599: c6 85 ce 0b 00 00 01        	mov	byte ptr [rbp + 0xbce], 0x1
1403db5a0: c6 85 cd 0b 00 00 01        	mov	byte ptr [rbp + 0xbcd], 0x1
1403db5a7: ba 26 00 00 00              	mov	edx, 0x26
1403db5ac: 41 b9 01 00 00 00           	mov	r9d, 0x1
1403db5b2: e8 d9 10 99 00              	call	0x140d6c690 <.text+0xd6b690>
1403db5b7: 48 85 c0                    	test	rax, rax
1403db5ba: 0f 85 7a 24 00 00           	jne	0x1403dda3a <.text+0x3dca3a>
1403db5c0: c6 85 d0 0b 00 00 01        	mov	byte ptr [rbp + 0xbd0], 0x1
1403db5c7: c6 85 cf 0b 00 00 01        	mov	byte ptr [rbp + 0xbcf], 0x1
1403db5ce: c6 85 ce 0b 00 00 01        	mov	byte ptr [rbp + 0xbce], 0x1
1403db5d5: c6 85 cd 0b 00 00 01        	mov	byte ptr [rbp + 0xbcd], 0x1
1403db5dc: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
1403db5e3: e8 c8 de 70 00              	call	0x140ae94b0 <.text+0xae84b0>
1403db5e8: 83 bd 90 02 00 00 03        	cmp	dword ptr [rbp + 0x290], 0x3
1403db5ef: 0f 85 ec 05 00 00           	jne	0x1403dbbe1 <.text+0x3dabe1>
1403db5f5: 48 8b 85 98 02 00 00        	mov	rax, qword ptr [rbp + 0x298]
1403db5fc: 48 89 85 90 0b 00 00        	mov	qword ptr [rbp + 0xb90], rax
1403db603: f6 85 d0 04 00 00 01        	test	byte ptr [rbp + 0x4d0], 0x1
1403db60a: 0f 84 16 12 00 00           	je	0x1403dc826 <.text+0x3db826>
1403db610: 48 8b b5 d8 04 00 00        	mov	rsi, qword ptr [rbp + 0x4d8]
1403db617: 31 c0                       	xor	eax, eax
1403db619: 48 3b 85 08 05 00 00        	cmp	rax, qword ptr [rbp + 0x508]
1403db620: 0f 81 2f 12 00 00           	jno	0x1403dc855 <.text+0x3db855>
1403db626: e9 4e 12 00 00              	jmp	0x1403dc879 <.text+0x3db879>
1403db62b: 48 8b 8d a0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2a0]
1403db632: 48 89 85 f0 01 00 00        	mov	qword ptr [rbp + 0x1f0], rax
1403db639: 48 89 b5 f8 01 00 00        	mov	qword ptr [rbp + 0x1f8], rsi
1403db640: 48 89 8d 00 02 00 00        	mov	qword ptr [rbp + 0x200], rcx
1403db647: c6 85 c5 0b 00 00 01        	mov	byte ptr [rbp + 0xbc5], 0x1
1403db64e: c6 85 c4 0b 00 00 01        	mov	byte ptr [rbp + 0xbc4], 0x1
1403db655: c6 85 c3 0b 00 00 01        	mov	byte ptr [rbp + 0xbc3], 0x1
1403db65c: c6 85 c2 0b 00 00 00        	mov	byte ptr [rbp + 0xbc2], 0x0
1403db663: 48 8d 8d f0 01 00 00        	lea	rcx, [rbp + 0x1f0]
1403db66a: e8 d1 b4 80 00              	call	0x140be6b40 <.text+0xbe5b40>
1403db66f: 41 b7 01                    	mov	r15b, 0x1
1403db672: e9 63 14 00 00              	jmp	0x1403dcada <.text+0x3dbada>
1403db677: 48 89 8d 98 0b 00 00        	mov	qword ptr [rbp + 0xb98], rcx
1403db67e: 48 8b 85 f8 04 00 00        	mov	rax, qword ptr [rbp + 0x4f8]
1403db685: 48 89 85 a8 0b 00 00        	mov	qword ptr [rbp + 0xba8], rax
1403db68c: 4c 8b a5 00 05 00 00        	mov	r12, qword ptr [rbp + 0x500]
1403db693: 0f 57 c0                    	xorps	xmm0, xmm0
1403db696: 0f 11 85 20 07 00 00        	movups	xmmword ptr [rbp + 0x720], xmm0
1403db69d: 48 c7 85 f0 06 00 00 03 00 00 00    	mov	qword ptr [rbp + 0x6f0], 0x3
1403db6a8: 48 c7 85 f8 06 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x6f8], 0x0
1403db6b3: 48 c7 85 00 07 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x700], 0x8
1403db6be: 0f 11 85 08 07 00 00        	movups	xmmword ptr [rbp + 0x708], xmm0
1403db6c5: 48 c7 85 18 07 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x718], 0x8
1403db6d0: 48 c7 85 30 07 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x730], 0x8
1403db6db: 48 c7 85 38 07 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x738], 0x0
1403db6e6: 48 c7 85 40 07 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x740], 0x1
1403db6f1: 48 8d 05 08 81 5f 02        	lea	rax, [rip + 0x25f8108]  # 0x1429d3800
1403db6f8: 48 89 85 48 07 00 00        	mov	qword ptr [rbp + 0x748], rax
1403db6ff: c6 85 b8 0b 00 00 01        	mov	byte ptr [rbp + 0xbb8], 0x1
1403db706: e8 c5 fd 97 00              	call	0x140d5b4d0 <.text+0xd5a4d0>
1403db70b: 48 c7 85 20 09 00 00 06 00 00 00    	mov	qword ptr [rbp + 0x920], 0x6
1403db716: 49 bd 00 00 00 00 00 00 00 80       	movabs	r13, -0x8000000000000000
1403db720: 4c 89 ad 28 09 00 00        	mov	qword ptr [rbp + 0x928], r13
1403db727: 48 8d 1d 50 82 5f 02        	lea	rbx, [rip + 0x25f8250]  # 0x1429d397e
1403db72e: 48 89 9d 30 09 00 00        	mov	qword ptr [rbp + 0x930], rbx
1403db735: 48 c7 85 38 09 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x938], 0x1
1403db740: 48 89 85 40 09 00 00        	mov	qword ptr [rbp + 0x940], rax
1403db747: 48 8b b5 08 07 00 00        	mov	rsi, qword ptr [rbp + 0x708]
1403db74e: 48 3b b5 f8 06 00 00        	cmp	rsi, qword ptr [rbp + 0x6f8]
1403db755: 75 13                       	jne	0x1403db76a <.text+0x3da76a>
1403db757: 48 8d 8d f8 06 00 00        	lea	rcx, [rbp + 0x6f8]
1403db75e: 48 8d 15 83 80 5f 02        	lea	rdx, [rip + 0x25f8083]  # 0x1429d37e8
1403db765: e8 06 74 18 00              	call	0x140562b70 <.text+0x561b70>
1403db76a: 48 8b 85 00 07 00 00        	mov	rax, qword ptr [rbp + 0x700]
1403db771: 48 8d 0c 76                 	lea	rcx, [rsi + 2*rsi]
1403db775: 48 c1 e1 05                 	shl	rcx, 0x5
1403db779: 0f 10 85 20 09 00 00        	movups	xmm0, xmmword ptr [rbp + 0x920]
1403db780: 0f 10 8d 30 09 00 00        	movups	xmm1, xmmword ptr [rbp + 0x930]
1403db787: 0f 10 95 40 09 00 00        	movups	xmm2, xmmword ptr [rbp + 0x940]
1403db78e: 0f 10 9d 50 09 00 00        	movups	xmm3, xmmword ptr [rbp + 0x950]
1403db795: 0f 10 a5 60 09 00 00        	movups	xmm4, xmmword ptr [rbp + 0x960]
1403db79c: 0f 10 ad 70 09 00 00        	movups	xmm5, xmmword ptr [rbp + 0x970]
1403db7a3: 0f 11 6c 08 50              	movups	xmmword ptr [rax + rcx + 0x50], xmm5
1403db7a8: 0f 11 64 08 40              	movups	xmmword ptr [rax + rcx + 0x40], xmm4
1403db7ad: 0f 11 5c 08 30              	movups	xmmword ptr [rax + rcx + 0x30], xmm3
1403db7b2: 0f 11 54 08 20              	movups	xmmword ptr [rax + rcx + 0x20], xmm2
1403db7b7: 0f 11 4c 08 10              	movups	xmmword ptr [rax + rcx + 0x10], xmm1
1403db7bc: 0f 11 04 08                 	movups	xmmword ptr [rax + rcx], xmm0
1403db7c0: 48 ff c6                    	inc	rsi
1403db7c3: 48 89 b5 08 07 00 00        	mov	qword ptr [rbp + 0x708], rsi
1403db7ca: 0f 10 85 f0 06 00 00        	movups	xmm0, xmmword ptr [rbp + 0x6f0]
1403db7d1: 0f 10 8d 10 07 00 00        	movups	xmm1, xmmword ptr [rbp + 0x710]
1403db7d8: 0f 10 95 20 07 00 00        	movups	xmm2, xmmword ptr [rbp + 0x720]
1403db7df: 0f 10 9d 30 07 00 00        	movups	xmm3, xmmword ptr [rbp + 0x730]
1403db7e6: 0f 29 85 20 02 00 00        	movaps	xmmword ptr [rbp + 0x220], xmm0
1403db7ed: 0f 29 8d 40 02 00 00        	movaps	xmmword ptr [rbp + 0x240], xmm1
1403db7f4: 0f 29 95 50 02 00 00        	movaps	xmmword ptr [rbp + 0x250], xmm2
1403db7fb: 0f 29 9d 60 02 00 00        	movaps	xmmword ptr [rbp + 0x260], xmm3
1403db802: 0f 10 85 40 07 00 00        	movups	xmm0, xmmword ptr [rbp + 0x740]
1403db809: 0f 29 85 70 02 00 00        	movaps	xmmword ptr [rbp + 0x270], xmm0
1403db810: 48 8b 85 00 07 00 00        	mov	rax, qword ptr [rbp + 0x700]
1403db817: 48 89 85 30 02 00 00        	mov	qword ptr [rbp + 0x230], rax
1403db81e: 48 8b 85 08 07 00 00        	mov	rax, qword ptr [rbp + 0x708]
1403db825: 48 89 85 38 02 00 00        	mov	qword ptr [rbp + 0x238], rax
1403db82c: c6 85 20 09 00 00 01        	mov	byte ptr [rbp + 0x920], 0x1
1403db833: c6 85 bd 0b 00 00 01        	mov	byte ptr [rbp + 0xbbd], 0x1
1403db83a: c6 85 bc 0b 00 00 01        	mov	byte ptr [rbp + 0xbbc], 0x1
1403db841: 48 8d 15 4a 82 5f 02        	lea	rdx, [rip + 0x25f824a]  # 0x1429d3a92
1403db848: 48 8d 8d 20 09 00 00        	lea	rcx, [rbp + 0x920]
1403db84f: 41 b8 03 00 00 00           	mov	r8d, 0x3
1403db855: e8 f6 e8 97 00              	call	0x140d5a150 <.text+0xd59150>
1403db85a: 89 c1                       	mov	ecx, eax
1403db85c: 83 e1 03                    	and	ecx, 0x3
1403db85f: 83 f9 01                    	cmp	ecx, 0x1
1403db862: 75 6b                       	jne	0x1403db8cf <.text+0x3da8cf>
1403db864: 48 89 c1                    	mov	rcx, rax
1403db867: 48 ff c9                    	dec	rcx
1403db86a: 48 89 8d 60 0b 00 00        	mov	qword ptr [rbp + 0xb60], rcx
1403db871: 48 8b 48 ff                 	mov	rcx, qword ptr [rax - 0x1]
1403db875: 48 89 8d a0 0b 00 00        	mov	qword ptr [rbp + 0xba0], rcx
1403db87c: 48 8b 40 07                 	mov	rax, qword ptr [rax + 0x7]
1403db880: 48 89 85 68 0b 00 00        	mov	qword ptr [rbp + 0xb68], rax
1403db887: 48 8b 00                    	mov	rax, qword ptr [rax]
1403db88a: 48 85 c0                    	test	rax, rax
1403db88d: 74 09                       	je	0x1403db898 <.text+0x3da898>
1403db88f: 48 8b 8d a0 0b 00 00        	mov	rcx, qword ptr [rbp + 0xba0]
1403db896: ff d0                       	call	rax
1403db898: 48 8b 8d a0 0b 00 00        	mov	rcx, qword ptr [rbp + 0xba0]
1403db89f: 48 8b 85 68 0b 00 00        	mov	rax, qword ptr [rbp + 0xb68]
1403db8a6: 48 8b 50 08                 	mov	rdx, qword ptr [rax + 0x8]
1403db8aa: 48 85 d2                    	test	rdx, rdx
1403db8ad: 74 09                       	je	0x1403db8b8 <.text+0x3da8b8>
1403db8af: 4c 8b 40 10                 	mov	r8, qword ptr [rax + 0x10]
1403db8b3: e8 68 83 23 00              	call	0x140613c20 <.text+0x612c20>
1403db8b8: ba 18 00 00 00              	mov	edx, 0x18
1403db8bd: 41 b8 08 00 00 00           	mov	r8d, 0x8
1403db8c3: 48 8b 8d 60 0b 00 00        	mov	rcx, qword ptr [rbp + 0xb60]
1403db8ca: e8 51 83 23 00              	call	0x140613c20 <.text+0x612c20>
1403db8cf: 48 8d 8d f8 06 00 00        	lea	rcx, [rbp + 0x6f8]
1403db8d6: 48 c7 85 f8 06 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x6f8], 0x0
1403db8e1: 48 c7 85 00 07 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x700], 0x8
1403db8ec: 0f 57 f6                    	xorps	xmm6, xmm6
1403db8ef: 0f 11 b5 08 07 00 00        	movups	xmmword ptr [rbp + 0x708], xmm6
1403db8f6: 48 c7 85 f0 06 00 00 05 00 00 00    	mov	qword ptr [rbp + 0x6f0], 0x5
1403db901: 48 c7 85 18 07 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x718], 0x8
1403db90c: 0f 11 b5 20 07 00 00        	movups	xmmword ptr [rbp + 0x720], xmm6
1403db913: 48 c7 85 30 07 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x730], 0x8
1403db91e: 0f 11 b5 38 07 00 00        	movups	xmmword ptr [rbp + 0x738], xmm6
1403db925: 0f 28 85 70 02 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x270]
1403db92c: 0f 29 85 70 09 00 00        	movaps	xmmword ptr [rbp + 0x970], xmm0
1403db933: 0f 28 85 60 02 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x260]
1403db93a: 0f 29 85 60 09 00 00        	movaps	xmmword ptr [rbp + 0x960], xmm0
1403db941: 0f 28 85 20 02 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x220]
1403db948: 0f 28 8d 30 02 00 00        	movaps	xmm1, xmmword ptr [rbp + 0x230]
1403db94f: 0f 28 95 40 02 00 00        	movaps	xmm2, xmmword ptr [rbp + 0x240]
1403db956: 0f 28 9d 50 02 00 00        	movaps	xmm3, xmmword ptr [rbp + 0x250]
1403db95d: 0f 29 9d 50 09 00 00        	movaps	xmmword ptr [rbp + 0x950], xmm3
1403db964: 0f 29 95 40 09 00 00        	movaps	xmmword ptr [rbp + 0x940], xmm2
1403db96b: 0f 29 8d 30 09 00 00        	movaps	xmmword ptr [rbp + 0x930], xmm1
1403db972: 0f 29 85 20 09 00 00        	movaps	xmmword ptr [rbp + 0x920], xmm0
1403db979: 48 8d 15 68 7e 5f 02        	lea	rdx, [rip + 0x25f7e68]  # 0x1429d37e8
1403db980: e8 eb 71 18 00              	call	0x140562b70 <.text+0x561b70>
1403db985: 48 8b 85 00 07 00 00        	mov	rax, qword ptr [rbp + 0x700]
1403db98c: 0f 28 85 20 09 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x920]
1403db993: 0f 28 8d 30 09 00 00        	movaps	xmm1, xmmword ptr [rbp + 0x930]
1403db99a: 0f 28 95 40 09 00 00        	movaps	xmm2, xmmword ptr [rbp + 0x940]
1403db9a1: 0f 28 9d 50 09 00 00        	movaps	xmm3, xmmword ptr [rbp + 0x950]
1403db9a8: 0f 28 a5 60 09 00 00        	movaps	xmm4, xmmword ptr [rbp + 0x960]
1403db9af: 0f 28 ad 70 09 00 00        	movaps	xmm5, xmmword ptr [rbp + 0x970]
1403db9b6: 0f 11 68 50                 	movups	xmmword ptr [rax + 0x50], xmm5
1403db9ba: 0f 11 60 40                 	movups	xmmword ptr [rax + 0x40], xmm4
1403db9be: 0f 11 58 30                 	movups	xmmword ptr [rax + 0x30], xmm3
1403db9c2: 0f 11 50 20                 	movups	xmmword ptr [rax + 0x20], xmm2
1403db9c6: 0f 11 48 10                 	movups	xmmword ptr [rax + 0x10], xmm1
1403db9ca: 0f 11 00                    	movups	xmmword ptr [rax], xmm0
1403db9cd: 48 c7 85 08 07 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x708], 0x1
1403db9d8: 0f 10 85 f0 06 00 00        	movups	xmm0, xmmword ptr [rbp + 0x6f0]
1403db9df: 0f 10 8d 10 07 00 00        	movups	xmm1, xmmword ptr [rbp + 0x710]
1403db9e6: 0f 10 95 20 07 00 00        	movups	xmm2, xmmword ptr [rbp + 0x720]
1403db9ed: 0f 10 9d 30 07 00 00        	movups	xmm3, xmmword ptr [rbp + 0x730]
1403db9f4: 0f 29 85 90 06 00 00        	movaps	xmmword ptr [rbp + 0x690], xmm0
1403db9fb: 0f 29 8d b0 06 00 00        	movaps	xmmword ptr [rbp + 0x6b0], xmm1
1403dba02: 0f 29 95 c0 06 00 00        	movaps	xmmword ptr [rbp + 0x6c0], xmm2
1403dba09: 0f 29 9d d0 06 00 00        	movaps	xmmword ptr [rbp + 0x6d0], xmm3
1403dba10: 0f 10 85 40 07 00 00        	movups	xmm0, xmmword ptr [rbp + 0x740]
1403dba17: 0f 29 85 e0 06 00 00        	movaps	xmmword ptr [rbp + 0x6e0], xmm0
1403dba1e: 48 8b 85 00 07 00 00        	mov	rax, qword ptr [rbp + 0x700]
1403dba25: 48 89 85 a0 06 00 00        	mov	qword ptr [rbp + 0x6a0], rax
1403dba2c: 48 8b 85 08 07 00 00        	mov	rax, qword ptr [rbp + 0x708]
1403dba33: 48 89 85 a8 06 00 00        	mov	qword ptr [rbp + 0x6a8], rax
1403dba3a: c7 85 90 02 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x290], 0x0
1403dba44: 0f 11 b5 98 02 00 00        	movups	xmmword ptr [rbp + 0x298], xmm6
1403dba4b: c7 85 a8 02 00 00 07 00 00 00       	mov	dword ptr [rbp + 0x2a8], 0x7
1403dba55: 48 c7 85 ac 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x2ac], 0x0
1403dba60: 66 c7 85 b4 02 00 00 00 00  	mov	word ptr [rbp + 0x2b4], 0x0
1403dba69: c6 85 b1 02 00 00 01        	mov	byte ptr [rbp + 0x2b1], 0x1
1403dba70: c6 85 b4 02 00 00 01        	mov	byte ptr [rbp + 0x2b4], 0x1
1403dba77: c6 85 b3 02 00 00 01        	mov	byte ptr [rbp + 0x2b3], 0x1
1403dba7e: c6 85 d7 0b 00 00 01        	mov	byte ptr [rbp + 0xbd7], 0x1
1403dba85: c6 85 d6 0b 00 00 01        	mov	byte ptr [rbp + 0xbd6], 0x1
1403dba8c: 48 8d 15 02 80 5f 02        	lea	rdx, [rip + 0x25f8002]  # 0x1429d3a95
1403dba93: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
1403dba9a: 41 b8 1a 00 00 00           	mov	r8d, 0x1a
1403dbaa0: e8 1b e3 97 00              	call	0x140d59dc0 <.text+0xd58dc0>
1403dbaa5: 48 85 c0                    	test	rax, rax
1403dbaa8: 0f 84 e4 01 00 00           	je	0x1403dbc92 <.text+0x3dac92>
1403dbaae: 48 89 55 b0                 	mov	qword ptr [rbp - 0x50], rdx
1403dbab2: 48 8d 45 b0                 	lea	rax, [rbp - 0x50]
1403dbab6: 48 89 85 f0 06 00 00        	mov	qword ptr [rbp + 0x6f0], rax
1403dbabd: 48 8d 05 1c f2 97 00        	lea	rax, [rip + 0x97f21c]   # 0x140d5ace0 <.text+0xd59ce0>
1403dbac4: 48 89 85 f8 06 00 00        	mov	qword ptr [rbp + 0x6f8], rax
1403dbacb: 48 8d 05 0e 80 5f 02        	lea	rax, [rip + 0x25f800e]  # 0x1429d3ae0
1403dbad2: 48 89 85 20 09 00 00        	mov	qword ptr [rbp + 0x920], rax
1403dbad9: 48 c7 85 28 09 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x928], 0x2
1403dbae4: 48 c7 85 40 09 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x940], 0x0
1403dbaef: 48 89 bd 30 09 00 00        	mov	qword ptr [rbp + 0x930], rdi
1403dbaf6: 48 c7 85 38 09 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x938], 0x1
1403dbb01: 48 8d 8d 20 09 00 00        	lea	rcx, [rbp + 0x920]
1403dbb08: e8 93 0f 98 00              	call	0x140d5caa0 <.text+0xd5baa0>
1403dbb0d: 0f 28 85 90 06 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x690]
1403dbb14: 0f 28 8d a0 06 00 00        	movaps	xmm1, xmmword ptr [rbp + 0x6a0]
1403dbb1b: 0f 28 95 b0 06 00 00        	movaps	xmm2, xmmword ptr [rbp + 0x6b0]
1403dbb22: 0f 28 9d c0 06 00 00        	movaps	xmm3, xmmword ptr [rbp + 0x6c0]
1403dbb29: 0f 29 85 30 06 00 00        	movaps	xmmword ptr [rbp + 0x630], xmm0
1403dbb30: 0f 29 8d 40 06 00 00        	movaps	xmmword ptr [rbp + 0x640], xmm1
1403dbb37: 0f 29 95 50 06 00 00        	movaps	xmmword ptr [rbp + 0x650], xmm2
1403dbb3e: 0f 29 9d 60 06 00 00        	movaps	xmmword ptr [rbp + 0x660], xmm3
1403dbb45: 0f 28 85 d0 06 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x6d0]
1403dbb4c: 0f 29 85 70 06 00 00        	movaps	xmmword ptr [rbp + 0x670], xmm0
1403dbb53: 0f 28 85 e0 06 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x6e0]
1403dbb5a: 0f 29 85 80 06 00 00        	movaps	xmmword ptr [rbp + 0x680], xmm0
1403dbb61: 48 8b 45 b0                 	mov	rax, qword ptr [rbp - 0x50]
1403dbb65: 89 c1                       	mov	ecx, eax
1403dbb67: 83 e1 03                    	and	ecx, 0x3
1403dbb6a: 83 f9 01                    	cmp	ecx, 0x1
1403dbb6d: 0f 85 b7 03 00 00           	jne	0x1403dbf2a <.text+0x3daf2a>
1403dbb73: 48 8d 48 ff                 	lea	rcx, [rax - 0x1]
1403dbb77: 48 89 8d 60 0b 00 00        	mov	qword ptr [rbp + 0xb60], rcx
1403dbb7e: 48 8b 48 ff                 	mov	rcx, qword ptr [rax - 0x1]
1403dbb82: 48 89 8d a0 0b 00 00        	mov	qword ptr [rbp + 0xba0], rcx
1403dbb89: 48 8b 40 07                 	mov	rax, qword ptr [rax + 0x7]
1403dbb8d: 48 89 85 68 0b 00 00        	mov	qword ptr [rbp + 0xb68], rax
1403dbb94: 48 8b 00                    	mov	rax, qword ptr [rax]
1403dbb97: 48 85 c0                    	test	rax, rax
1403dbb9a: 74 09                       	je	0x1403dbba5 <.text+0x3daba5>
1403dbb9c: 48 8b 8d a0 0b 00 00        	mov	rcx, qword ptr [rbp + 0xba0]
1403dbba3: ff d0                       	call	rax
1403dbba5: 48 8b 8d a0 0b 00 00        	mov	rcx, qword ptr [rbp + 0xba0]
1403dbbac: 48 8b 85 68 0b 00 00        	mov	rax, qword ptr [rbp + 0xb68]
1403dbbb3: 48 8b 50 08                 	mov	rdx, qword ptr [rax + 0x8]
1403dbbb7: 48 85 d2                    	test	rdx, rdx
1403dbbba: 74 09                       	je	0x1403dbbc5 <.text+0x3dabc5>
1403dbbbc: 4c 8b 40 10                 	mov	r8, qword ptr [rax + 0x10]
1403dbbc0: e8 5b 80 23 00              	call	0x140613c20 <.text+0x612c20>
1403dbbc5: ba 18 00 00 00              	mov	edx, 0x18
1403dbbca: 41 b8 08 00 00 00           	mov	r8d, 0x8
1403dbbd0: 48 8b 8d 60 0b 00 00        	mov	rcx, qword ptr [rbp + 0xb60]
1403dbbd7: e8 44 80 23 00              	call	0x140613c20 <.text+0x612c20>
1403dbbdc: e9 49 03 00 00              	jmp	0x1403dbf2a <.text+0x3daf2a>
1403dbbe1: 0f 10 85 90 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x290]
1403dbbe8: 0f 10 8d a0 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x2a0]
1403dbbef: 0f 29 8d 00 07 00 00        	movaps	xmmword ptr [rbp + 0x700], xmm1
1403dbbf6: 0f 29 85 f0 06 00 00        	movaps	xmmword ptr [rbp + 0x6f0], xmm0
1403dbbfd: 48 89 7d b0                 	mov	qword ptr [rbp - 0x50], rdi
1403dbc01: 48 8d 05 58 f4 ff ff        	lea	rax, [rip - 0xba8]      # 0x1403db060 <.text+0x3da060>
1403dbc08: 48 89 45 b8                 	mov	qword ptr [rbp - 0x48], rax
1403dbc0c: 48 8d 05 1d 83 5f 02        	lea	rax, [rip + 0x25f831d]  # 0x1429d3f30
1403dbc13: 48 89 85 20 09 00 00        	mov	qword ptr [rbp + 0x920], rax
1403dbc1a: 48 c7 85 28 09 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x928], 0x1
1403dbc25: 48 8d 45 b0                 	lea	rax, [rbp - 0x50]
1403dbc29: 48 89 85 30 09 00 00        	mov	qword ptr [rbp + 0x930], rax
1403dbc30: 48 c7 85 38 09 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x938], 0x1
1403dbc3b: 48 c7 85 40 09 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x940], 0x0
1403dbc46: 48 8d 8d d8 01 00 00        	lea	rcx, [rbp + 0x1d8]
1403dbc4d: 48 8d 95 20 09 00 00        	lea	rdx, [rbp + 0x920]
1403dbc54: e8 37 e3 99 00              	call	0x140d79f90 <.text+0xd78f90>
1403dbc59: 48 8d 8d d8 01 00 00        	lea	rcx, [rbp + 0x1d8]
1403dbc60: e8 bb 6e 4c 02              	call	0x1428a2b20 <.text+0x28a1b20>
1403dbc65: 48 89 c6                    	mov	rsi, rax
1403dbc68: 48 8b 95 f8 06 00 00        	mov	rdx, qword ptr [rbp + 0x6f8]
1403dbc6f: 48 85 d2                    	test	rdx, rdx
1403dbc72: 74 12                       	je	0x1403dbc86 <.text+0x3dac86>
1403dbc74: 48 8b 8d 00 07 00 00        	mov	rcx, qword ptr [rbp + 0x700]
1403dbc7b: 41 b8 01 00 00 00           	mov	r8d, 0x1
1403dbc81: e8 9a 7f 23 00              	call	0x140613c20 <.text+0x612c20>
1403dbc86: 48 8b 95 88 0b 00 00        	mov	rdx, qword ptr [rbp + 0xb88]
1403dbc8d: e9 9e 0c 00 00              	jmp	0x1403dc930 <.text+0x3db930>
1403dbc92: 0f 57 c0                    	xorps	xmm0, xmm0
1403dbc95: 0f 11 85 20 07 00 00        	movups	xmmword ptr [rbp + 0x720], xmm0
1403dbc9c: 48 c7 85 f0 06 00 00 04 00 00 00    	mov	qword ptr [rbp + 0x6f0], 0x4
1403dbca7: 48 8d 8d f8 06 00 00        	lea	rcx, [rbp + 0x6f8]
1403dbcae: 48 c7 85 f8 06 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x6f8], 0x0
1403dbcb9: 48 c7 85 00 07 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x700], 0x8
1403dbcc4: 0f 11 85 08 07 00 00        	movups	xmmword ptr [rbp + 0x708], xmm0
1403dbccb: 48 c7 85 18 07 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x718], 0x8
1403dbcd6: 48 c7 85 30 07 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x730], 0x8
1403dbce1: 48 c7 85 38 07 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x738], 0x0
1403dbcec: 48 c7 85 40 07 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x740], 0x1
1403dbcf7: 48 8d 05 32 7b 5f 02        	lea	rax, [rip + 0x25f7b32]  # 0x1429d3830
1403dbcfe: 48 89 85 48 07 00 00        	mov	qword ptr [rbp + 0x748], rax
1403dbd05: 48 c7 85 20 09 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x920], 0x8
1403dbd10: 4c 89 ad 28 09 00 00        	mov	qword ptr [rbp + 0x928], r13
1403dbd17: 48 89 9d 30 09 00 00        	mov	qword ptr [rbp + 0x930], rbx
1403dbd1e: 48 c7 85 38 09 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x938], 0x1
1403dbd29: 48 89 95 40 09 00 00        	mov	qword ptr [rbp + 0x940], rdx
1403dbd30: 48 8d 15 b1 7a 5f 02        	lea	rdx, [rip + 0x25f7ab1]  # 0x1429d37e8
1403dbd37: e8 34 6e 18 00              	call	0x140562b70 <.text+0x561b70>
1403dbd3c: 48 8b 85 00 07 00 00        	mov	rax, qword ptr [rbp + 0x700]
1403dbd43: 0f 10 85 20 09 00 00        	movups	xmm0, xmmword ptr [rbp + 0x920]
1403dbd4a: 0f 10 8d 30 09 00 00        	movups	xmm1, xmmword ptr [rbp + 0x930]
1403dbd51: 0f 10 95 40 09 00 00        	movups	xmm2, xmmword ptr [rbp + 0x940]
1403dbd58: 0f 10 9d 50 09 00 00        	movups	xmm3, xmmword ptr [rbp + 0x950]
1403dbd5f: 0f 10 a5 60 09 00 00        	movups	xmm4, xmmword ptr [rbp + 0x960]
1403dbd66: 0f 10 ad 70 09 00 00        	movups	xmm5, xmmword ptr [rbp + 0x970]
1403dbd6d: 0f 11 68 50                 	movups	xmmword ptr [rax + 0x50], xmm5
1403dbd71: 0f 11 60 40                 	movups	xmmword ptr [rax + 0x40], xmm4
1403dbd75: 0f 11 58 30                 	movups	xmmword ptr [rax + 0x30], xmm3
1403dbd79: 0f 11 50 20                 	movups	xmmword ptr [rax + 0x20], xmm2
1403dbd7d: 0f 11 48 10                 	movups	xmmword ptr [rax + 0x10], xmm1
1403dbd81: 0f 11 00                    	movups	xmmword ptr [rax], xmm0
1403dbd84: 48 c7 85 08 07 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x708], 0x1
1403dbd8f: 0f 10 85 40 07 00 00        	movups	xmm0, xmmword ptr [rbp + 0x740]
1403dbd96: 0f 29 85 70 09 00 00        	movaps	xmmword ptr [rbp + 0x970], xmm0
1403dbd9d: 0f 10 85 f0 06 00 00        	movups	xmm0, xmmword ptr [rbp + 0x6f0]
1403dbda4: 0f 10 8d 10 07 00 00        	movups	xmm1, xmmword ptr [rbp + 0x710]
1403dbdab: 0f 10 95 20 07 00 00        	movups	xmm2, xmmword ptr [rbp + 0x720]
1403dbdb2: 0f 10 9d 30 07 00 00        	movups	xmm3, xmmword ptr [rbp + 0x730]
1403dbdb9: 0f 29 9d 60 09 00 00        	movaps	xmmword ptr [rbp + 0x960], xmm3
1403dbdc0: 0f 29 95 50 09 00 00        	movaps	xmmword ptr [rbp + 0x950], xmm2
1403dbdc7: 0f 29 8d 40 09 00 00        	movaps	xmmword ptr [rbp + 0x940], xmm1
1403dbdce: 48 8b 85 00 07 00 00        	mov	rax, qword ptr [rbp + 0x700]
1403dbdd5: 48 89 85 30 09 00 00        	mov	qword ptr [rbp + 0x930], rax
1403dbddc: 48 8b 85 08 07 00 00        	mov	rax, qword ptr [rbp + 0x708]
1403dbde3: 48 89 85 38 09 00 00        	mov	qword ptr [rbp + 0x938], rax
1403dbdea: 0f 29 85 20 09 00 00        	movaps	xmmword ptr [rbp + 0x920], xmm0
1403dbdf1: 0f 28 85 90 06 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x690]
1403dbdf8: 0f 28 8d a0 06 00 00        	movaps	xmm1, xmmword ptr [rbp + 0x6a0]
1403dbdff: 0f 28 95 b0 06 00 00        	movaps	xmm2, xmmword ptr [rbp + 0x6b0]
1403dbe06: 0f 28 9d c0 06 00 00        	movaps	xmm3, xmmword ptr [rbp + 0x6c0]
1403dbe0d: 0f 29 8d 00 07 00 00        	movaps	xmmword ptr [rbp + 0x700], xmm1
1403dbe14: 0f 29 85 f0 06 00 00        	movaps	xmmword ptr [rbp + 0x6f0], xmm0
1403dbe1b: 0f 28 85 e0 06 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x6e0]
1403dbe22: 0f 29 85 40 07 00 00        	movaps	xmmword ptr [rbp + 0x740], xmm0
1403dbe29: 0f 28 85 d0 06 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x6d0]
1403dbe30: 0f 29 85 30 07 00 00        	movaps	xmmword ptr [rbp + 0x730], xmm0
1403dbe37: 0f 29 9d 20 07 00 00        	movaps	xmmword ptr [rbp + 0x720], xmm3
1403dbe3e: 0f 29 95 10 07 00 00        	movaps	xmmword ptr [rbp + 0x710], xmm2
1403dbe45: 48 8b b5 08 07 00 00        	mov	rsi, qword ptr [rbp + 0x708]
1403dbe4c: 48 3b b5 f8 06 00 00        	cmp	rsi, qword ptr [rbp + 0x6f8]
1403dbe53: 75 13                       	jne	0x1403dbe68 <.text+0x3dae68>
1403dbe55: 48 8d 8d f8 06 00 00        	lea	rcx, [rbp + 0x6f8]
1403dbe5c: 48 8d 15 85 79 5f 02        	lea	rdx, [rip + 0x25f7985]  # 0x1429d37e8
1403dbe63: e8 08 6d 18 00              	call	0x140562b70 <.text+0x561b70>
1403dbe68: 48 8b 85 00 07 00 00        	mov	rax, qword ptr [rbp + 0x700]
1403dbe6f: 48 8d 0c 76                 	lea	rcx, [rsi + 2*rsi]
1403dbe73: 48 c1 e1 05                 	shl	rcx, 0x5
1403dbe77: 0f 28 85 20 09 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x920]
1403dbe7e: 0f 28 8d 30 09 00 00        	movaps	xmm1, xmmword ptr [rbp + 0x930]
1403dbe85: 0f 28 95 40 09 00 00        	movaps	xmm2, xmmword ptr [rbp + 0x940]
1403dbe8c: 0f 28 9d 50 09 00 00        	movaps	xmm3, xmmword ptr [rbp + 0x950]
1403dbe93: 0f 28 a5 60 09 00 00        	movaps	xmm4, xmmword ptr [rbp + 0x960]
1403dbe9a: 0f 28 ad 70 09 00 00        	movaps	xmm5, xmmword ptr [rbp + 0x970]
1403dbea1: 0f 11 6c 08 50              	movups	xmmword ptr [rax + rcx + 0x50], xmm5
1403dbea6: 0f 11 64 08 40              	movups	xmmword ptr [rax + rcx + 0x40], xmm4
1403dbeab: 0f 11 5c 08 30              	movups	xmmword ptr [rax + rcx + 0x30], xmm3
1403dbeb0: 0f 11 54 08 20              	movups	xmmword ptr [rax + rcx + 0x20], xmm2
1403dbeb5: 0f 11 4c 08 10              	movups	xmmword ptr [rax + rcx + 0x10], xmm1
1403dbeba: 0f 11 04 08                 	movups	xmmword ptr [rax + rcx], xmm0
1403dbebe: 48 ff c6                    	inc	rsi
1403dbec1: 48 89 b5 08 07 00 00        	mov	qword ptr [rbp + 0x708], rsi
1403dbec8: 0f 28 85 f0 06 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x6f0]
1403dbecf: 0f 28 8d 10 07 00 00        	movaps	xmm1, xmmword ptr [rbp + 0x710]
1403dbed6: 0f 28 95 20 07 00 00        	movaps	xmm2, xmmword ptr [rbp + 0x720]
1403dbedd: 0f 28 9d 30 07 00 00        	movaps	xmm3, xmmword ptr [rbp + 0x730]
1403dbee4: 0f 29 85 30 06 00 00        	movaps	xmmword ptr [rbp + 0x630], xmm0
1403dbeeb: 0f 29 8d 50 06 00 00        	movaps	xmmword ptr [rbp + 0x650], xmm1
1403dbef2: 0f 29 95 60 06 00 00        	movaps	xmmword ptr [rbp + 0x660], xmm2
1403dbef9: 0f 29 9d 70 06 00 00        	movaps	xmmword ptr [rbp + 0x670], xmm3
1403dbf00: 0f 28 85 40 07 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x740]
1403dbf07: 0f 29 85 80 06 00 00        	movaps	xmmword ptr [rbp + 0x680], xmm0
1403dbf0e: 48 8b 85 00 07 00 00        	mov	rax, qword ptr [rbp + 0x700]
1403dbf15: 48 89 85 40 06 00 00        	mov	qword ptr [rbp + 0x640], rax
1403dbf1c: 48 8b 85 08 07 00 00        	mov	rax, qword ptr [rbp + 0x708]
1403dbf23: 48 89 85 48 06 00 00        	mov	qword ptr [rbp + 0x648], rax
1403dbf2a: 0f 28 85 80 06 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x680]
1403dbf31: 0f 29 85 70 09 00 00        	movaps	xmmword ptr [rbp + 0x970], xmm0
1403dbf38: 0f 28 85 70 06 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x670]
1403dbf3f: 0f 29 85 60 09 00 00        	movaps	xmmword ptr [rbp + 0x960], xmm0
1403dbf46: 0f 28 85 30 06 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x630]
1403dbf4d: 48 8b 85 40 06 00 00        	mov	rax, qword ptr [rbp + 0x640]
1403dbf54: 48 8b 8d 48 06 00 00        	mov	rcx, qword ptr [rbp + 0x648]
1403dbf5b: 0f 28 8d 50 06 00 00        	movaps	xmm1, xmmword ptr [rbp + 0x650]
1403dbf62: 0f 28 95 60 06 00 00        	movaps	xmm2, xmmword ptr [rbp + 0x660]
1403dbf69: 0f 29 95 50 09 00 00        	movaps	xmmword ptr [rbp + 0x950], xmm2
1403dbf70: 0f 29 8d 40 09 00 00        	movaps	xmmword ptr [rbp + 0x940], xmm1
1403dbf77: 48 89 85 30 09 00 00        	mov	qword ptr [rbp + 0x930], rax
1403dbf7e: 48 89 8d 38 09 00 00        	mov	qword ptr [rbp + 0x938], rcx
1403dbf85: 0f 29 85 20 09 00 00        	movaps	xmmword ptr [rbp + 0x920], xmm0
1403dbf8c: c6 85 d7 0b 00 00 00        	mov	byte ptr [rbp + 0xbd7], 0x0
1403dbf93: c6 85 d6 0b 00 00 01        	mov	byte ptr [rbp + 0xbd6], 0x1
1403dbf9a: 48 8d 8d 20 09 00 00        	lea	rcx, [rbp + 0x920]
1403dbfa1: e8 9a db 29 00              	call	0x140679b40 <.text+0x678b40>
1403dbfa6: 84 c0                       	test	al, al
1403dbfa8: 74 6c                       	je	0x1403dc016 <.text+0x3db016>
1403dbfaa: 48 8d 85 bb 0b 00 00        	lea	rax, [rbp + 0xbbb]
1403dbfb1: 48 89 85 f0 06 00 00        	mov	qword ptr [rbp + 0x6f0], rax
1403dbfb8: 48 8d 05 61 f0 ff ff        	lea	rax, [rip - 0xf9f]      # 0x1403db020 <.text+0x3da020>
1403dbfbf: 48 89 85 f8 06 00 00        	mov	qword ptr [rbp + 0x6f8], rax
1403dbfc6: 48 8d 05 63 7b 5f 02        	lea	rax, [rip + 0x25f7b63]  # 0x1429d3b30
1403dbfcd: 48 89 85 20 09 00 00        	mov	qword ptr [rbp + 0x920], rax
1403dbfd4: 48 c7 85 28 09 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x928], 0x2
1403dbfdf: 48 c7 85 40 09 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x940], 0x0
1403dbfea: 48 89 bd 30 09 00 00        	mov	qword ptr [rbp + 0x930], rdi
1403dbff1: 48 c7 85 38 09 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x938], 0x1
1403dbffc: c6 85 d7 0b 00 00 00        	mov	byte ptr [rbp + 0xbd7], 0x0
1403dc003: c6 85 d6 0b 00 00 01        	mov	byte ptr [rbp + 0xbd6], 0x1
1403dc00a: 48 8d 8d 20 09 00 00        	lea	rcx, [rbp + 0x920]
1403dc011: e8 8a 0a 98 00              	call	0x140d5caa0 <.text+0xd5baa0>
1403dc016: 48 8b 05 8b 03 25 03        	mov	rax, qword ptr [rip + 0x325038b] # 0x14362c3a8
1403dc01d: 48 8b 00                    	mov	rax, qword ptr [rax]
1403dc020: 48 83 f8 03                 	cmp	rax, 0x3
1403dc024: 0f 82 d2 00 00 00           	jb	0x1403dc0fc <.text+0x3db0fc>
1403dc02a: c6 85 d7 0b 00 00 00        	mov	byte ptr [rbp + 0xbd7], 0x0
1403dc031: c6 85 d6 0b 00 00 01        	mov	byte ptr [rbp + 0xbd6], 0x1
1403dc038: 48 8d 0d 49 7b 5f 02        	lea	rcx, [rip + 0x25f7b49]  # 0x1429d3b88
1403dc03f: e8 3c 1d 2a 00              	call	0x14067dd80 <.text+0x67cd80>
1403dc044: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
1403dc047: 8b 40 10                    	mov	eax, dword ptr [rax + 0x10]
1403dc04a: 48 c7 85 50 09 00 00 03 00 00 00    	mov	qword ptr [rbp + 0x950], 0x3
1403dc055: 48 8d 0d e4 79 5f 02        	lea	rcx, [rip + 0x25f79e4]  # 0x1429d3a40
1403dc05c: 48 89 8d 58 09 00 00        	mov	qword ptr [rbp + 0x958], rcx
1403dc063: 48 c7 85 60 09 00 00 0a 00 00 00    	mov	qword ptr [rbp + 0x960], 0xa
1403dc06e: 48 8d 15 03 7b 5f 02        	lea	rdx, [rip + 0x25f7b03]  # 0x1429d3b78
1403dc075: 48 89 95 70 09 00 00        	mov	qword ptr [rbp + 0x970], rdx
1403dc07c: 48 c7 85 78 09 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x978], 0x1
1403dc087: 48 c7 85 80 09 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x980], 0x8
1403dc092: 48 c7 85 20 09 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x920], 0x0
1403dc09d: 0f 57 c9                    	xorps	xmm1, xmm1
1403dc0a0: 0f 11 8d 88 09 00 00        	movups	xmmword ptr [rbp + 0x988], xmm1
1403dc0a7: 48 89 8d 28 09 00 00        	mov	qword ptr [rbp + 0x928], rcx
1403dc0ae: 48 c7 85 30 09 00 00 0a 00 00 00    	mov	qword ptr [rbp + 0x930], 0xa
1403dc0b9: 48 c7 85 38 09 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x938], 0x0
1403dc0c4: 0f 11 85 40 09 00 00        	movups	xmmword ptr [rbp + 0x940], xmm0
1403dc0cb: c7 85 68 09 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x968], 0x1
1403dc0d5: 89 85 6c 09 00 00           	mov	dword ptr [rbp + 0x96c], eax
1403dc0db: c6 85 d7 0b 00 00 00        	mov	byte ptr [rbp + 0xbd7], 0x0
1403dc0e2: c6 85 d6 0b 00 00 01        	mov	byte ptr [rbp + 0xbd6], 0x1
1403dc0e9: 48 8d 8d bb 0b 00 00        	lea	rcx, [rbp + 0xbbb]
1403dc0f0: 48 8d 95 20 09 00 00        	lea	rdx, [rbp + 0x920]
1403dc0f7: e8 34 e2 96 00              	call	0x140d4a330 <.text+0xd49330>
1403dc0fc: 4c 89 e7                    	mov	rdi, r12
1403dc0ff: 4c 89 b5 a0 0b 00 00        	mov	qword ptr [rbp + 0xba0], r14
1403dc106: 44 88 bd 68 0b 00 00        	mov	byte ptr [rbp + 0xb68], r15b
1403dc10d: 48 8b 05 84 ff 24 03        	mov	rax, qword ptr [rip + 0x324ff84] # 0x14362c098
1403dc114: 0f b6 30                    	movzx	esi, byte ptr [rax]
1403dc117: 0f b6 05 eb 8a 25 03        	movzx	eax, byte ptr [rip + 0x3258aeb] # 0x143634c09
1403dc11e: b9 10 00 00 00              	mov	ecx, 0x10
1403dc123: ba 08 00 00 00              	mov	edx, 0x8
1403dc128: e8 e3 7a 23 00              	call	0x140613c10 <.text+0x612c10>
1403dc12d: 48 85 c0                    	test	rax, rax
1403dc130: 0f 84 e2 18 00 00           	je	0x1403dda18 <.text+0x3dca18>
1403dc136: 48 89 c3                    	mov	rbx, rax
1403dc139: 48 8d 0d bf 7a 5f 02        	lea	rcx, [rip + 0x25f7abf]  # 0x1429d3bff
1403dc140: 48 8d 05 59 7a 5f 02        	lea	rax, [rip + 0x25f7a59]  # 0x1429d3ba0
1403dc147: 40 84 f6                    	test	sil, sil
1403dc14a: 48 0f 45 c1                 	cmovne	rax, rcx
1403dc14e: 48 c7 03 01 00 00 00        	mov	qword ptr [rbx], 0x1
1403dc155: 48 c7 43 08 01 00 00 00     	mov	qword ptr [rbx + 0x8], 0x1
1403dc15d: 4c 89 ad 50 07 00 00        	mov	qword ptr [rbp + 0x750], r13
1403dc164: 48 be 01 00 00 00 00 00 00 80       	movabs	rsi, -0x7fffffffffffffff
1403dc16e: 48 89 b5 e8 07 00 00        	mov	qword ptr [rbp + 0x7e8], rsi
1403dc175: 48 89 b5 00 08 00 00        	mov	qword ptr [rbp + 0x800], rsi
1403dc17c: f3 0f 10 05 80 97 54 02     	movss	xmm0, dword ptr [rip + 0x2549780] # 0x142925904
1403dc184: 0f 13 85 b8 08 00 00        	movlps	qword ptr [rbp + 0x8b8], xmm0
1403dc18b: 48 c7 85 70 08 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x870], 0x0
1403dc196: 48 c7 85 80 08 00 00 64 00 00 00    	mov	qword ptr [rbp + 0x880], 0x64
1403dc1a1: 48 89 b5 18 08 00 00        	mov	qword ptr [rbp + 0x818], rsi
1403dc1a8: 48 c7 85 f0 06 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x6f0], 0x0
1403dc1b3: 48 c7 85 f8 06 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x6f8], 0x8
1403dc1be: 0f 57 c0                    	xorps	xmm0, xmm0
1403dc1c1: 0f 11 85 00 07 00 00        	movups	xmmword ptr [rbp + 0x700], xmm0
1403dc1c8: 48 c7 85 10 07 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x710], 0x8
1403dc1d3: 0f 11 85 18 07 00 00        	movups	xmmword ptr [rbp + 0x718], xmm0
1403dc1da: 48 c7 85 28 07 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x728], 0x8
1403dc1e5: 48 c7 85 30 07 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x730], 0x0
1403dc1f0: c7 85 c0 08 00 00 00 00 00 01       	mov	dword ptr [rbp + 0x8c0], 0x1000000
1403dc1fa: 48 c7 85 88 08 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x888], 0x0
1403dc205: 48 c7 85 98 08 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x898], 0x0
1403dc210: 48 c7 85 a8 08 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x8a8], 0x0
1403dc21b: 48 89 b5 30 08 00 00        	mov	qword ptr [rbp + 0x830], rsi
1403dc222: 48 89 b5 48 08 00 00        	mov	qword ptr [rbp + 0x848], rsi
1403dc229: 48 c7 85 60 08 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x860], 0x2
1403dc234: c7 85 68 08 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x868], 0x0
1403dc23e: 48 c7 85 38 07 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x738], 0x0
1403dc249: 48 c7 85 40 07 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x740], 0x8
1403dc254: 48 c7 85 48 07 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x748], 0x0
1403dc25f: c7 85 c4 08 00 00 00 00 00 01       	mov	dword ptr [rbp + 0x8c4], 0x1000000
1403dc269: 4c 89 ad d0 07 00 00        	mov	qword ptr [rbp + 0x7d0], r13
1403dc270: 48 8d 0d 08 77 5f 02        	lea	rcx, [rip + 0x25f7708]  # 0x1429d397f
1403dc277: 48 89 8d d8 07 00 00        	mov	qword ptr [rbp + 0x7d8], rcx
1403dc27e: 48 c7 85 e0 07 00 00 12 00 00 00    	mov	qword ptr [rbp + 0x7e0], 0x12
1403dc289: 0f 10 85 50 07 00 00        	movups	xmm0, xmmword ptr [rbp + 0x750]
1403dc290: 0f 10 8d 60 07 00 00        	movups	xmm1, xmmword ptr [rbp + 0x760]
1403dc297: 0f 10 95 70 07 00 00        	movups	xmm2, xmmword ptr [rbp + 0x770]
1403dc29e: 0f 10 9d 80 07 00 00        	movups	xmm3, xmmword ptr [rbp + 0x780]
1403dc2a5: 0f 29 85 f0 02 00 00        	movaps	xmmword ptr [rbp + 0x2f0], xmm0
1403dc2ac: 0f 29 8d 00 03 00 00        	movaps	xmmword ptr [rbp + 0x300], xmm1
1403dc2b3: 0f 29 95 10 03 00 00        	movaps	xmmword ptr [rbp + 0x310], xmm2
1403dc2ba: 0f 29 9d 20 03 00 00        	movaps	xmmword ptr [rbp + 0x320], xmm3
1403dc2c1: 0f 10 85 90 07 00 00        	movups	xmm0, xmmword ptr [rbp + 0x790]
1403dc2c8: 0f 29 85 30 03 00 00        	movaps	xmmword ptr [rbp + 0x330], xmm0
1403dc2cf: 0f 10 85 a0 07 00 00        	movups	xmm0, xmmword ptr [rbp + 0x7a0]
1403dc2d6: 0f 29 85 40 03 00 00        	movaps	xmmword ptr [rbp + 0x340], xmm0
1403dc2dd: 0f 10 85 b0 07 00 00        	movups	xmm0, xmmword ptr [rbp + 0x7b0]
1403dc2e4: 0f 29 85 50 03 00 00        	movaps	xmmword ptr [rbp + 0x350], xmm0
1403dc2eb: 0f 10 85 c0 07 00 00        	movups	xmm0, xmmword ptr [rbp + 0x7c0]
1403dc2f2: 0f 29 85 60 03 00 00        	movaps	xmmword ptr [rbp + 0x360], xmm0
1403dc2f9: 48 8b 8d 10 08 00 00        	mov	rcx, qword ptr [rbp + 0x810]
1403dc300: 48 89 8d 50 0a 00 00        	mov	qword ptr [rbp + 0xa50], rcx
1403dc307: 0f 10 85 00 08 00 00        	movups	xmm0, xmmword ptr [rbp + 0x800]
1403dc30e: 0f 11 85 40 0a 00 00        	movups	xmmword ptr [rbp + 0xa40], xmm0
1403dc315: 4c 8b bd 78 08 00 00        	mov	r15, qword ptr [rbp + 0x878]
1403dc31c: 48 8b 8d 28 08 00 00        	mov	rcx, qword ptr [rbp + 0x828]
1403dc323: 48 89 8d 68 0a 00 00        	mov	qword ptr [rbp + 0xa68], rcx
1403dc32a: 0f 10 85 18 08 00 00        	movups	xmm0, xmmword ptr [rbp + 0x818]
1403dc331: 0f 11 85 58 0a 00 00        	movups	xmmword ptr [rbp + 0xa58], xmm0
1403dc338: 48 8b 8d 00 07 00 00        	mov	rcx, qword ptr [rbp + 0x700]
1403dc33f: 48 89 8d a0 02 00 00        	mov	qword ptr [rbp + 0x2a0], rcx
1403dc346: 0f 10 85 f0 06 00 00        	movups	xmm0, xmmword ptr [rbp + 0x6f0]
1403dc34d: 0f 29 85 90 02 00 00        	movaps	xmmword ptr [rbp + 0x290], xmm0
1403dc354: 48 8b 8d 18 07 00 00        	mov	rcx, qword ptr [rbp + 0x718]
1403dc35b: 48 89 8d b8 02 00 00        	mov	qword ptr [rbp + 0x2b8], rcx
1403dc362: 0f 10 85 08 07 00 00        	movups	xmm0, xmmword ptr [rbp + 0x708]
1403dc369: 0f 11 85 a8 02 00 00        	movups	xmmword ptr [rbp + 0x2a8], xmm0
1403dc370: 48 8b 8d 30 07 00 00        	mov	rcx, qword ptr [rbp + 0x730]
1403dc377: 48 89 8d d0 02 00 00        	mov	qword ptr [rbp + 0x2d0], rcx
1403dc37e: 0f 10 85 20 07 00 00        	movups	xmm0, xmmword ptr [rbp + 0x720]
1403dc385: 0f 29 85 c0 02 00 00        	movaps	xmmword ptr [rbp + 0x2c0], xmm0
1403dc38c: 4d 89 ee                    	mov	r14, r13
1403dc38f: 4c 8b ad a0 08 00 00        	mov	r13, qword ptr [rbp + 0x8a0]
1403dc396: 4c 8b a5 b0 08 00 00        	mov	r12, qword ptr [rbp + 0x8b0]
1403dc39d: 48 8b 8d 40 08 00 00        	mov	rcx, qword ptr [rbp + 0x840]
1403dc3a4: 48 89 8d 80 0a 00 00        	mov	qword ptr [rbp + 0xa80], rcx
1403dc3ab: 0f 10 85 30 08 00 00        	movups	xmm0, xmmword ptr [rbp + 0x830]
1403dc3b2: 0f 11 85 70 0a 00 00        	movups	xmmword ptr [rbp + 0xa70], xmm0
1403dc3b9: 48 8b 8d 58 08 00 00        	mov	rcx, qword ptr [rbp + 0x858]
1403dc3c0: 48 89 8d 98 0a 00 00        	mov	qword ptr [rbp + 0xa98], rcx
1403dc3c7: 0f 10 85 48 08 00 00        	movups	xmm0, xmmword ptr [rbp + 0x848]
1403dc3ce: 0f 11 85 88 0a 00 00        	movups	xmmword ptr [rbp + 0xa88], xmm0
1403dc3d5: 48 8b 8d 48 07 00 00        	mov	rcx, qword ptr [rbp + 0x748]
1403dc3dc: 48 89 8d e8 02 00 00        	mov	qword ptr [rbp + 0x2e8], rcx
1403dc3e3: 0f 10 85 38 07 00 00        	movups	xmm0, xmmword ptr [rbp + 0x738]
1403dc3ea: 0f 11 85 d8 02 00 00        	movups	xmmword ptr [rbp + 0x2d8], xmm0
1403dc3f1: 48 8b 8d e0 07 00 00        	mov	rcx, qword ptr [rbp + 0x7e0]
1403dc3f8: 48 89 8d 80 03 00 00        	mov	qword ptr [rbp + 0x380], rcx
1403dc3ff: 0f 10 85 d0 07 00 00        	movups	xmm0, xmmword ptr [rbp + 0x7d0]
1403dc406: 0f 29 85 70 03 00 00        	movaps	xmmword ptr [rbp + 0x370], xmm0
1403dc40d: 48 89 85 20 09 00 00        	mov	qword ptr [rbp + 0x920], rax
1403dc414: 48 c7 85 28 09 00 00 5f 00 00 00    	mov	qword ptr [rbp + 0x928], 0x5f
1403dc41f: 48 8d 8d 30 09 00 00        	lea	rcx, [rbp + 0x930]
1403dc426: 48 8d 95 90 02 00 00        	lea	rdx, [rbp + 0x290]
1403dc42d: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1403dc433: e8 c8 77 54 02              	call	0x142923c00 <.text+0x2922c00>
1403dc438: 4c 89 b5 28 0a 00 00        	mov	qword ptr [rbp + 0xa28], r14
1403dc43f: 48 8d 05 18 78 5f 02        	lea	rax, [rip + 0x25f7818]  # 0x1429d3c5e
1403dc446: 48 89 85 30 0a 00 00        	mov	qword ptr [rbp + 0xa30], rax
1403dc44d: 48 c7 85 38 0a 00 00 13 00 00 00    	mov	qword ptr [rbp + 0xa38], 0x13
1403dc458: 48 c7 85 a0 0a 00 00 02 00 00 00    	mov	qword ptr [rbp + 0xaa0], 0x2
1403dc463: c7 85 a8 0a 00 00 00 00 00 00       	mov	dword ptr [rbp + 0xaa8], 0x0
1403dc46d: 48 c7 85 b0 0a 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xab0], 0x0
1403dc478: 4c 89 bd b8 0a 00 00        	mov	qword ptr [rbp + 0xab8], r15
1403dc47f: 48 c7 85 c0 0a 00 00 64 00 00 00    	mov	qword ptr [rbp + 0xac0], 0x64
1403dc48a: 48 89 9d c8 0a 00 00        	mov	qword ptr [rbp + 0xac8], rbx
1403dc491: 48 8d 05 e0 77 5f 02        	lea	rax, [rip + 0x25f77e0]  # 0x1429d3c78
1403dc498: 48 89 85 d0 0a 00 00        	mov	qword ptr [rbp + 0xad0], rax
1403dc49f: 48 c7 85 d8 0a 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xad8], 0x0
1403dc4aa: 4c 89 ad e0 0a 00 00        	mov	qword ptr [rbp + 0xae0], r13
1403dc4b1: 48 c7 85 e8 0a 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xae8], 0x0
1403dc4bc: 4c 89 a5 f0 0a 00 00        	mov	qword ptr [rbp + 0xaf0], r12
1403dc4c3: 48 c7 85 f8 0a 00 00 00 00 80 3f    	mov	qword ptr [rbp + 0xaf8], 0x3f800000
1403dc4ce: 48 b8 00 00 00 00 01 00 00 00       	movabs	rax, 0x100000000
1403dc4d8: 48 89 85 ff 0a 00 00        	mov	qword ptr [rbp + 0xaff], rax
1403dc4df: c6 85 07 0b 00 00 01        	mov	byte ptr [rbp + 0xb07], 0x1
1403dc4e6: 48 8d 8d 20 09 00 00        	lea	rcx, [rbp + 0x920]
1403dc4ed: e8 ee 8a d9 ff              	call	0x140174fe0 <.text+0x173fe0>
1403dc4f2: 48 89 85 70 0b 00 00        	mov	qword ptr [rbp + 0xb70], rax
1403dc4f9: 48 8b 95 e8 07 00 00        	mov	rdx, qword ptr [rbp + 0x7e8]
1403dc500: 48 b8 02 00 00 00 00 00 00 80       	movabs	rax, -0x7ffffffffffffffe
1403dc50a: 48 39 c2                    	cmp	rdx, rax
1403dc50d: 7c 17                       	jl	0x1403dc526 <.text+0x3db526>
1403dc50f: 48 85 d2                    	test	rdx, rdx
1403dc512: 74 12                       	je	0x1403dc526 <.text+0x3db526>
1403dc514: 48 8b 8d f0 07 00 00        	mov	rcx, qword ptr [rbp + 0x7f0]
1403dc51b: 41 b8 01 00 00 00           	mov	r8d, 0x1
1403dc521: e8 fa 76 23 00              	call	0x140613c20 <.text+0x612c20>
1403dc526: 48 8b 85 88 08 00 00        	mov	rax, qword ptr [rbp + 0x888]
1403dc52d: 48 85 c0                    	test	rax, rax
1403dc530: 44 0f b6 bd 68 0b 00 00     	movzx	r15d, byte ptr [rbp + 0xb68]
1403dc538: 4c 8b b5 a0 0b 00 00        	mov	r14, qword ptr [rbp + 0xba0]
1403dc53f: 74 19                       	je	0x1403dc55a <.text+0x3db55a>
1403dc541: f0                          	lock
1403dc542: 48 ff 08                    	dec	qword ptr [rax]
1403dc545: 75 13                       	jne	0x1403dc55a <.text+0x3db55a>
1403dc547: c6 85 c1 0b 00 00 01        	mov	byte ptr [rbp + 0xbc1], 0x1
1403dc54e: 48 8d 8d 88 08 00 00        	lea	rcx, [rbp + 0x888]
1403dc555: e8 76 34 27 00              	call	0x14064f9d0 <.text+0x64e9d0>
1403dc55a: 0f 28 85 80 02 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x280]
1403dc561: 0f 11 85 30 09 00 00        	movups	xmmword ptr [rbp + 0x930], xmm0
1403dc568: 48 8b 85 88 0b 00 00        	mov	rax, qword ptr [rbp + 0xb88]
1403dc56f: 48 89 85 20 09 00 00        	mov	qword ptr [rbp + 0x920], rax
1403dc576: 48 8b 85 58 0b 00 00        	mov	rax, qword ptr [rbp + 0xb58]
1403dc57d: 48 89 85 28 09 00 00        	mov	qword ptr [rbp + 0x928], rax
1403dc584: 48 c7 44 24 20 88 13 00 00  	mov	qword ptr [rsp + 0x20], 0x1388
1403dc58d: 48 8d 8d f0 06 00 00        	lea	rcx, [rbp + 0x6f0]
1403dc594: 4c 8d 8d 20 09 00 00        	lea	r9, [rbp + 0x920]
1403dc59b: 48 8b 95 a8 0b 00 00        	mov	rdx, qword ptr [rbp + 0xba8]
1403dc5a2: 49 89 f8                    	mov	r8, rdi
1403dc5a5: e8 d6 28 ed ff              	call	0x1402aee80 <.text+0x2ade80>
1403dc5aa: 48 8b 95 98 0b 00 00        	mov	rdx, qword ptr [rbp + 0xb98]
1403dc5b1: 48 85 d2                    	test	rdx, rdx
1403dc5b4: 74 12                       	je	0x1403dc5c8 <.text+0x3db5c8>
1403dc5b6: 41 b8 01 00 00 00           	mov	r8d, 0x1
1403dc5bc: 48 8b 8d a8 0b 00 00        	mov	rcx, qword ptr [rbp + 0xba8]
1403dc5c3: e8 58 76 23 00              	call	0x140613c20 <.text+0x612c20>
1403dc5c8: 48 39 b5 f0 06 00 00        	cmp	qword ptr [rbp + 0x6f0], rsi
1403dc5cf: 0f 85 2d 01 00 00           	jne	0x1403dc702 <.text+0x3db702>
1403dc5d5: 48 8b 85 18 07 00 00        	mov	rax, qword ptr [rbp + 0x718]
1403dc5dc: 48 89 85 b0 02 00 00        	mov	qword ptr [rbp + 0x2b0], rax
1403dc5e3: 0f 10 85 f8 06 00 00        	movups	xmm0, xmmword ptr [rbp + 0x6f8]
1403dc5ea: 0f 10 8d 08 07 00 00        	movups	xmm1, xmmword ptr [rbp + 0x708]
1403dc5f1: 0f 29 8d a0 02 00 00        	movaps	xmmword ptr [rbp + 0x2a0], xmm1
1403dc5f8: 0f 29 85 90 02 00 00        	movaps	xmmword ptr [rbp + 0x290], xmm0
1403dc5ff: 48 8b 05 a2 fd 24 03        	mov	rax, qword ptr [rip + 0x324fda2] # 0x14362c3a8
1403dc606: 48 8b 00                    	mov	rax, qword ptr [rax]
1403dc609: 48 85 c0                    	test	rax, rax
1403dc60c: 0f 84 d8 00 00 00           	je	0x1403dc6ea <.text+0x3db6ea>
1403dc612: 48 8d 85 90 02 00 00        	lea	rax, [rbp + 0x290]
1403dc619: 48 89 45 b0                 	mov	qword ptr [rbp - 0x50], rax
1403dc61d: 48 8d 05 8c 2e 00 00        	lea	rax, [rip + 0x2e8c]     # 0x1403df4b0 <.text+0x3de4b0>
1403dc624: 48 89 45 b8                 	mov	qword ptr [rbp - 0x48], rax
1403dc628: 48 8d 0d 41 77 5f 02        	lea	rcx, [rip + 0x25f7741]  # 0x1429d3d70
1403dc62f: e8 4c 17 2a 00              	call	0x14067dd80 <.text+0x67cd80>
1403dc634: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
1403dc637: 8b 40 10                    	mov	eax, dword ptr [rax + 0x10]
1403dc63a: 48 c7 85 50 09 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x950], 0x1
1403dc645: 48 8d 0d f4 73 5f 02        	lea	rcx, [rip + 0x25f73f4]  # 0x1429d3a40
1403dc64c: 48 89 8d 58 09 00 00        	mov	qword ptr [rbp + 0x958], rcx
1403dc653: 48 c7 85 60 09 00 00 0a 00 00 00    	mov	qword ptr [rbp + 0x960], 0xa
1403dc65e: 48 8d 15 fb 76 5f 02        	lea	rdx, [rip + 0x25f76fb]  # 0x1429d3d60
1403dc665: 48 89 95 70 09 00 00        	mov	qword ptr [rbp + 0x970], rdx
1403dc66c: 48 c7 85 78 09 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x978], 0x1
1403dc677: 48 8d 55 b0                 	lea	rdx, [rbp - 0x50]
1403dc67b: 48 89 95 80 09 00 00        	mov	qword ptr [rbp + 0x980], rdx
1403dc682: 48 c7 85 88 09 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x988], 0x1
1403dc68d: 48 c7 85 90 09 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x990], 0x0
1403dc698: 48 c7 85 20 09 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x920], 0x0
1403dc6a3: 48 89 8d 28 09 00 00        	mov	qword ptr [rbp + 0x928], rcx
1403dc6aa: 48 c7 85 30 09 00 00 0a 00 00 00    	mov	qword ptr [rbp + 0x930], 0xa
1403dc6b5: 48 c7 85 38 09 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x938], 0x0
1403dc6c0: 0f 11 85 40 09 00 00        	movups	xmmword ptr [rbp + 0x940], xmm0
1403dc6c7: c7 85 68 09 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x968], 0x1
1403dc6d1: 89 85 6c 09 00 00           	mov	dword ptr [rbp + 0x96c], eax
1403dc6d7: 48 8d 8d bb 0b 00 00        	lea	rcx, [rbp + 0xbbb]
1403dc6de: 48 8d 95 20 09 00 00        	lea	rdx, [rbp + 0x920]
1403dc6e5: e8 46 dc 96 00              	call	0x140d4a330 <.text+0xd49330>
1403dc6ea: c6 85 c1 0b 00 00 00        	mov	byte ptr [rbp + 0xbc1], 0x0
1403dc6f1: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
1403dc6f8: e8 73 e8 ff ff              	call	0x1403daf70 <.text+0x3d9f70>
1403dc6fd: e9 a5 03 00 00              	jmp	0x1403dcaa7 <.text+0x3dbaa7>
1403dc702: 48 8d 8d 20 09 00 00        	lea	rcx, [rbp + 0x920]
1403dc709: 48 8d 95 f0 06 00 00        	lea	rdx, [rbp + 0x6f0]
1403dc710: 41 b8 30 02 00 00           	mov	r8d, 0x230
1403dc716: e8 e5 74 54 02              	call	0x142923c00 <.text+0x2922c00>
1403dc71b: 48 8b 8d 20 09 00 00        	mov	rcx, qword ptr [rbp + 0x920]
1403dc722: 48 89 c8                    	mov	rax, rcx
1403dc725: 48 f7 d8                    	neg	rax
1403dc728: 0f 81 22 02 00 00           	jno	0x1403dc950 <.text+0x3db950>
1403dc72e: 48 8b 05 73 fc 24 03        	mov	rax, qword ptr [rip + 0x324fc73] # 0x14362c3a8
1403dc735: 48 8b 00                    	mov	rax, qword ptr [rax]
1403dc738: 48 83 f8 03                 	cmp	rax, 0x3
1403dc73c: 0f 82 65 03 00 00           	jb	0x1403dcaa7 <.text+0x3dbaa7>
1403dc742: 48 89 8d 60 0b 00 00        	mov	qword ptr [rbp + 0xb60], rcx
1403dc749: 48 8d 0d c8 75 5f 02        	lea	rcx, [rip + 0x25f75c8]  # 0x1429d3d18
1403dc750: e8 2b 16 2a 00              	call	0x14067dd80 <.text+0x67cd80>
1403dc755: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
1403dc758: 8b 40 10                    	mov	eax, dword ptr [rax + 0x10]
1403dc75b: 48 c7 85 c0 02 00 00 03 00 00 00    	mov	qword ptr [rbp + 0x2c0], 0x3
1403dc766: 48 8d 0d d3 72 5f 02        	lea	rcx, [rip + 0x25f72d3]  # 0x1429d3a40
1403dc76d: 48 89 8d c8 02 00 00        	mov	qword ptr [rbp + 0x2c8], rcx
1403dc774: 48 c7 85 d0 02 00 00 0a 00 00 00    	mov	qword ptr [rbp + 0x2d0], 0xa
1403dc77f: 48 8d 15 82 75 5f 02        	lea	rdx, [rip + 0x25f7582]  # 0x1429d3d08
1403dc786: 48 89 95 e0 02 00 00        	mov	qword ptr [rbp + 0x2e0], rdx
1403dc78d: 48 c7 85 e8 02 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x2e8], 0x1
1403dc798: 48 c7 85 f0 02 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x2f0], 0x8
1403dc7a3: 48 c7 85 90 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x290], 0x0
1403dc7ae: 0f 57 c9                    	xorps	xmm1, xmm1
1403dc7b1: 0f 11 8d f8 02 00 00        	movups	xmmword ptr [rbp + 0x2f8], xmm1
1403dc7b8: 48 89 8d 98 02 00 00        	mov	qword ptr [rbp + 0x298], rcx
1403dc7bf: 48 c7 85 a0 02 00 00 0a 00 00 00    	mov	qword ptr [rbp + 0x2a0], 0xa
1403dc7ca: 48 c7 85 a8 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x2a8], 0x0
1403dc7d5: 0f 11 85 b0 02 00 00        	movups	xmmword ptr [rbp + 0x2b0], xmm0
1403dc7dc: c7 85 d8 02 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x2d8], 0x1
1403dc7e6: 89 85 dc 02 00 00           	mov	dword ptr [rbp + 0x2dc], eax
1403dc7ec: 48 8d 8d bb 0b 00 00        	lea	rcx, [rbp + 0xbbb]
1403dc7f3: 48 8d 95 90 02 00 00        	lea	rdx, [rbp + 0x290]
1403dc7fa: e8 31 db 96 00              	call	0x140d4a330 <.text+0xd49330>
1403dc7ff: 31 c0                       	xor	eax, eax
1403dc801: 48 3b 85 20 09 00 00        	cmp	rax, qword ptr [rbp + 0x920]
1403dc808: 0f 80 99 02 00 00           	jo	0x1403dcaa7 <.text+0x3dbaa7>
1403dc80e: c6 85 c1 0b 00 00 00        	mov	byte ptr [rbp + 0xbc1], 0x0
1403dc815: 48 8d 8d 20 09 00 00        	lea	rcx, [rbp + 0x920]
1403dc81c: e8 6f 37 22 00              	call	0x1405fff90 <.text+0x5fef90>
1403dc821: e9 81 02 00 00              	jmp	0x1403dcaa7 <.text+0x3dbaa7>
1403dc826: c6 85 c0 0b 00 00 01        	mov	byte ptr [rbp + 0xbc0], 0x1
1403dc82d: c6 85 bf 0b 00 00 01        	mov	byte ptr [rbp + 0xbbf], 0x1
1403dc834: c6 85 be 0b 00 00 01        	mov	byte ptr [rbp + 0xbbe], 0x1
1403dc83b: 48 8d 8d 90 0b 00 00        	lea	rcx, [rbp + 0xb90]
1403dc842: e8 a9 ce 70 00              	call	0x140ae96f0 <.text+0xae86f0>
1403dc847: 48 89 c6                    	mov	rsi, rax
1403dc84a: 31 c0                       	xor	eax, eax
1403dc84c: 48 3b 85 08 05 00 00        	cmp	rax, qword ptr [rbp + 0x508]
1403dc853: 70 24                       	jo	0x1403dc879 <.text+0x3db879>
1403dc855: 48 8d 8d 08 05 00 00        	lea	rcx, [rbp + 0x508]
1403dc85c: c6 85 c0 0b 00 00 01        	mov	byte ptr [rbp + 0xbc0], 0x1
1403dc863: c6 85 bf 0b 00 00 01        	mov	byte ptr [rbp + 0xbbf], 0x1
1403dc86a: c6 85 be 0b 00 00 01        	mov	byte ptr [rbp + 0xbbe], 0x1
1403dc871: e8 5a a6 99 00              	call	0x140d76ed0 <.text+0xd75ed0>
1403dc876: 49 89 d0                    	mov	r8, rdx
1403dc879: c6 85 c0 0b 00 00 01        	mov	byte ptr [rbp + 0xbc0], 0x1
1403dc880: c6 85 bf 0b 00 00 01        	mov	byte ptr [rbp + 0xbbf], 0x1
1403dc887: c6 85 be 0b 00 00 01        	mov	byte ptr [rbp + 0xbbe], 0x1
1403dc88e: 48 89 f1                    	mov	rcx, rsi
1403dc891: 48 89 c2                    	mov	rdx, rax
1403dc894: e8 77 16 7c 00              	call	0x140b9df10 <.text+0xb9cf10>
1403dc899: 48 89 c6                    	mov	rsi, rax
1403dc89c: 48 85 c0                    	test	rax, rax
1403dc89f: 75 42                       	jne	0x1403dc8e3 <.text+0x3db8e3>
1403dc8a1: c6 85 c0 0b 00 00 01        	mov	byte ptr [rbp + 0xbc0], 0x1
1403dc8a8: c6 85 bf 0b 00 00 01        	mov	byte ptr [rbp + 0xbbf], 0x1
1403dc8af: c6 85 be 0b 00 00 01        	mov	byte ptr [rbp + 0xbbe], 0x1
1403dc8b6: 48 8d 8d 20 09 00 00        	lea	rcx, [rbp + 0x920]
1403dc8bd: 48 8d 95 c0 04 00 00        	lea	rdx, [rbp + 0x4c0]
1403dc8c4: e8 c7 70 f7 ff              	call	0x140353990 <.text+0x352990>
1403dc8c9: 48 8b bd 20 09 00 00        	mov	rdi, qword ptr [rbp + 0x920]
1403dc8d0: 48 8b b5 28 09 00 00        	mov	rsi, qword ptr [rbp + 0x928]
1403dc8d7: 48 89 f8                    	mov	rax, rdi
1403dc8da: 48 f7 d8                    	neg	rax
1403dc8dd: 0f 81 14 05 00 00           	jno	0x1403dcdf7 <.text+0x3dbdf7>
1403dc8e3: 40 b7 01                    	mov	dil, 0x1
1403dc8e6: 48 8b 85 90 0b 00 00        	mov	rax, qword ptr [rbp + 0xb90]
1403dc8ed: 48 85 c0                    	test	rax, rax
1403dc8f0: 74 2e                       	je	0x1403dc920 <.text+0x3db920>
1403dc8f2: f0                          	lock
1403dc8f3: 48 ff 08                    	dec	qword ptr [rax]
1403dc8f6: 75 28                       	jne	0x1403dc920 <.text+0x3db920>
1403dc8f8: 40 88 bd d0 0b 00 00        	mov	byte ptr [rbp + 0xbd0], dil
1403dc8ff: c6 85 cf 0b 00 00 01        	mov	byte ptr [rbp + 0xbcf], 0x1
1403dc906: c6 85 ce 0b 00 00 01        	mov	byte ptr [rbp + 0xbce], 0x1
1403dc90d: c6 85 cd 0b 00 00 01        	mov	byte ptr [rbp + 0xbcd], 0x1
1403dc914: 48 8d 8d 90 0b 00 00        	lea	rcx, [rbp + 0xb90]
1403dc91b: e8 30 8a 7a 00              	call	0x140b85350 <.text+0xb84350>
1403dc920: 40 84 ff                    	test	dil, dil
1403dc923: 48 8b 95 88 0b 00 00        	mov	rdx, qword ptr [rbp + 0xb88]
1403dc92a: 0f 84 ac 01 00 00           	je	0x1403dcadc <.text+0x3dbadc>
1403dc930: 48 85 d2                    	test	rdx, rdx
1403dc933: 0f 84 a3 01 00 00           	je	0x1403dcadc <.text+0x3dbadc>
1403dc939: 41 b8 01 00 00 00           	mov	r8d, 0x1
1403dc93f: 48 8b 8d 58 0b 00 00        	mov	rcx, qword ptr [rbp + 0xb58]
1403dc946: e8 d5 72 23 00              	call	0x140613c20 <.text+0x612c20>
1403dc94b: e9 8c 01 00 00              	jmp	0x1403dcadc <.text+0x3dbadc>
1403dc950: 48 89 8d 60 0b 00 00        	mov	qword ptr [rbp + 0xb60], rcx
1403dc957: 48 8d 4d b0                 	lea	rcx, [rbp - 0x50]
1403dc95b: 48 8d 95 f0 06 00 00        	lea	rdx, [rbp + 0x6f0]
1403dc962: 41 b8 28 02 00 00           	mov	r8d, 0x228
1403dc968: e8 93 72 54 02              	call	0x142923c00 <.text+0x2922c00>
1403dc96d: 0f b6 9d 48 0b 00 00        	movzx	ebx, byte ptr [rbp + 0xb48]
1403dc974: 0f b6 b5 49 0b 00 00        	movzx	esi, byte ptr [rbp + 0xb49]
1403dc97b: 48 8b 05 26 fa 24 03        	mov	rax, qword ptr [rip + 0x324fa26] # 0x14362c3a8
1403dc982: 48 8b 00                    	mov	rax, qword ptr [rax]
1403dc985: 48 83 f8 03                 	cmp	rax, 0x3
1403dc989: 0f 82 c4 00 00 00           	jb	0x1403dca53 <.text+0x3dba53>
1403dc98f: c6 85 b9 0b 00 00 01        	mov	byte ptr [rbp + 0xbb9], 0x1
1403dc996: 48 8d 0d 33 73 5f 02        	lea	rcx, [rip + 0x25f7333]  # 0x1429d3cd0
1403dc99d: e8 de 13 2a 00              	call	0x14067dd80 <.text+0x67cd80>
1403dc9a2: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
1403dc9a5: 8b 40 10                    	mov	eax, dword ptr [rax + 0x10]
1403dc9a8: 48 c7 85 c0 02 00 00 03 00 00 00    	mov	qword ptr [rbp + 0x2c0], 0x3
1403dc9b3: 48 8d 0d 86 70 5f 02        	lea	rcx, [rip + 0x25f7086]  # 0x1429d3a40
1403dc9ba: 48 89 8d c8 02 00 00        	mov	qword ptr [rbp + 0x2c8], rcx
1403dc9c1: 48 c7 85 d0 02 00 00 0a 00 00 00    	mov	qword ptr [rbp + 0x2d0], 0xa
1403dc9cc: 48 8d 15 ed 72 5f 02        	lea	rdx, [rip + 0x25f72ed]  # 0x1429d3cc0
1403dc9d3: 48 89 95 e0 02 00 00        	mov	qword ptr [rbp + 0x2e0], rdx
1403dc9da: 48 c7 85 e8 02 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x2e8], 0x1
1403dc9e5: 48 c7 85 f0 02 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x2f0], 0x8
1403dc9f0: 48 c7 85 90 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x290], 0x0
1403dc9fb: 0f 57 c9                    	xorps	xmm1, xmm1
1403dc9fe: 0f 11 8d f8 02 00 00        	movups	xmmword ptr [rbp + 0x2f8], xmm1
1403dca05: 48 89 8d 98 02 00 00        	mov	qword ptr [rbp + 0x298], rcx
1403dca0c: 48 c7 85 a0 02 00 00 0a 00 00 00    	mov	qword ptr [rbp + 0x2a0], 0xa
1403dca17: 48 c7 85 a8 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x2a8], 0x0
1403dca22: 0f 11 85 b0 02 00 00        	movups	xmmword ptr [rbp + 0x2b0], xmm0
1403dca29: c7 85 d8 02 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x2d8], 0x1
1403dca33: 89 85 dc 02 00 00           	mov	dword ptr [rbp + 0x2dc], eax
1403dca39: c6 85 b9 0b 00 00 01        	mov	byte ptr [rbp + 0xbb9], 0x1
1403dca40: 48 8d 8d bb 0b 00 00        	lea	rcx, [rbp + 0xbbb]
1403dca47: 48 8d 95 90 02 00 00        	lea	rdx, [rbp + 0x290]
1403dca4e: e8 dd d8 96 00              	call	0x140d4a330 <.text+0xd49330>
1403dca53: 48 8d bd 90 02 00 00        	lea	rdi, [rbp + 0x290]
1403dca5a: 48 8d 55 b0                 	lea	rdx, [rbp - 0x50]
1403dca5e: 41 b8 28 02 00 00           	mov	r8d, 0x228
1403dca64: 48 89 f9                    	mov	rcx, rdi
1403dca67: e8 94 71 54 02              	call	0x142923c00 <.text+0x2922c00>
1403dca6c: 44 0f b6 8d 00 06 00 00     	movzx	r9d, byte ptr [rbp + 0x600]
1403dca74: c6 85 b9 0b 00 00 00        	mov	byte ptr [rbp + 0xbb9], 0x0
1403dca7b: 48 89 f9                    	mov	rcx, rdi
1403dca7e: 89 da                       	mov	edx, ebx
1403dca80: 41 89 f0                    	mov	r8d, esi
1403dca83: e8 b8 1e fd ff              	call	0x1403ae940 <.text+0x3ad940>
1403dca88: 48 8b 8d 70 0b 00 00        	mov	rcx, qword ptr [rbp + 0xb70]
1403dca8f: 48 83 c1 10                 	add	rcx, 0x10
1403dca93: c6 85 b9 0b 00 00 00        	mov	byte ptr [rbp + 0xbb9], 0x0
1403dca9a: ba 1e 00 00 00              	mov	edx, 0x1e
1403dca9f: 45 31 c0                    	xor	r8d, r8d
1403dcaa2: e8 d9 c7 28 00              	call	0x140669280 <.text+0x668280>
1403dcaa7: 48 8d 8d 70 0b 00 00        	lea	rcx, [rbp + 0xb70]
1403dcaae: e8 8d 1c 28 00              	call	0x14065e740 <.text+0x65d740>
1403dcab3: 48 8b 85 70 0b 00 00        	mov	rax, qword ptr [rbp + 0xb70]
1403dcaba: f0                          	lock
1403dcabb: 48 ff 08                    	dec	qword ptr [rax]
1403dcabe: 75 1a                       	jne	0x1403dcada <.text+0x3dbada>
1403dcac0: c6 85 d7 0b 00 00 00        	mov	byte ptr [rbp + 0xbd7], 0x0
1403dcac7: c6 85 d6 0b 00 00 00        	mov	byte ptr [rbp + 0xbd6], 0x0
1403dcace: 48 8d 8d 70 0b 00 00        	lea	rcx, [rbp + 0xb70]
1403dcad5: e8 46 1e 29 00              	call	0x14066e920 <.text+0x66d920>
1403dcada: 31 f6                       	xor	esi, esi
1403dcadc: 48 8b 95 f0 04 00 00        	mov	rdx, qword ptr [rbp + 0x4f0]
1403dcae3: 48 89 d0                    	mov	rax, rdx
1403dcae6: 48 f7 d8                    	neg	rax
1403dcae9: 0f 91 c0                    	setno	al
1403dcaec: 41 84 c7                    	test	r15b, al
1403dcaef: 74 17                       	je	0x1403dcb08 <.text+0x3dbb08>
1403dcaf1: 48 85 d2                    	test	rdx, rdx
1403dcaf4: 74 12                       	je	0x1403dcb08 <.text+0x3dbb08>
1403dcaf6: 48 8b 8d f8 04 00 00        	mov	rcx, qword ptr [rbp + 0x4f8]
1403dcafd: 41 b8 01 00 00 00           	mov	r8d, 0x1
1403dcb03: e8 18 71 23 00              	call	0x140613c20 <.text+0x612c20>
1403dcb08: 48 8b 95 08 05 00 00        	mov	rdx, qword ptr [rbp + 0x508]
1403dcb0f: 48 89 d0                    	mov	rax, rdx
1403dcb12: 48 f7 d8                    	neg	rax
1403dcb15: 70 14                       	jo	0x1403dcb2b <.text+0x3dbb2b>
1403dcb17: 73 12                       	jae	0x1403dcb2b <.text+0x3dbb2b>
1403dcb19: 48 8b 8d 10 05 00 00        	mov	rcx, qword ptr [rbp + 0x510]
1403dcb20: 41 b8 01 00 00 00           	mov	r8d, 0x1
1403dcb26: e8 f5 70 23 00              	call	0x140613c20 <.text+0x612c20>
1403dcb2b: 48 8b 95 28 05 00 00        	mov	rdx, qword ptr [rbp + 0x528]
1403dcb32: 48 89 d0                    	mov	rax, rdx
1403dcb35: 48 f7 d8                    	neg	rax
1403dcb38: 70 14                       	jo	0x1403dcb4e <.text+0x3dbb4e>
1403dcb3a: 73 12                       	jae	0x1403dcb4e <.text+0x3dbb4e>
1403dcb3c: 48 8b 8d 30 05 00 00        	mov	rcx, qword ptr [rbp + 0x530]
1403dcb43: 41 b8 01 00 00 00           	mov	r8d, 0x1
1403dcb49: e8 d2 70 23 00              	call	0x140613c20 <.text+0x612c20>
1403dcb4e: 48 8b 95 48 05 00 00        	mov	rdx, qword ptr [rbp + 0x548]
1403dcb55: 48 89 d0                    	mov	rax, rdx
1403dcb58: 48 f7 d8                    	neg	rax
1403dcb5b: 70 14                       	jo	0x1403dcb71 <.text+0x3dbb71>
1403dcb5d: 73 12                       	jae	0x1403dcb71 <.text+0x3dbb71>
1403dcb5f: 48 8b 8d 50 05 00 00        	mov	rcx, qword ptr [rbp + 0x550]
1403dcb66: 41 b8 01 00 00 00           	mov	r8d, 0x1
1403dcb6c: e8 af 70 23 00              	call	0x140613c20 <.text+0x612c20>
1403dcb71: 48 8b 95 68 05 00 00        	mov	rdx, qword ptr [rbp + 0x568]
1403dcb78: 48 89 d0                    	mov	rax, rdx
1403dcb7b: 48 f7 d8                    	neg	rax
1403dcb7e: 70 14                       	jo	0x1403dcb94 <.text+0x3dbb94>
1403dcb80: 73 12                       	jae	0x1403dcb94 <.text+0x3dbb94>
1403dcb82: 48 8b 8d 70 05 00 00        	mov	rcx, qword ptr [rbp + 0x570]
1403dcb89: 41 b8 01 00 00 00           	mov	r8d, 0x1
1403dcb8f: e8 8c 70 23 00              	call	0x140613c20 <.text+0x612c20>
1403dcb94: 48 8b 95 88 05 00 00        	mov	rdx, qword ptr [rbp + 0x588]
1403dcb9b: 48 89 d0                    	mov	rax, rdx
1403dcb9e: 48 f7 d8                    	neg	rax
1403dcba1: 70 14                       	jo	0x1403dcbb7 <.text+0x3dbbb7>
1403dcba3: 73 12                       	jae	0x1403dcbb7 <.text+0x3dbbb7>
1403dcba5: 48 8b 8d 90 05 00 00        	mov	rcx, qword ptr [rbp + 0x590]
1403dcbac: 41 b8 01 00 00 00           	mov	r8d, 0x1
1403dcbb2: e8 69 70 23 00              	call	0x140613c20 <.text+0x612c20>
1403dcbb7: 48 8b 95 a8 05 00 00        	mov	rdx, qword ptr [rbp + 0x5a8]
1403dcbbe: 48 89 d0                    	mov	rax, rdx
1403dcbc1: 48 f7 d8                    	neg	rax
1403dcbc4: 70 14                       	jo	0x1403dcbda <.text+0x3dbbda>
1403dcbc6: 73 12                       	jae	0x1403dcbda <.text+0x3dbbda>
1403dcbc8: 48 8b 8d b0 05 00 00        	mov	rcx, qword ptr [rbp + 0x5b0]
1403dcbcf: 41 b8 01 00 00 00           	mov	r8d, 0x1
1403dcbd5: e8 46 70 23 00              	call	0x140613c20 <.text+0x612c20>
1403dcbda: 49 f7 de                    	neg	r14
1403dcbdd: 0f 90 c0                    	seto	al
1403dcbe0: 48 8b 95 c8 05 00 00        	mov	rdx, qword ptr [rbp + 0x5c8]
1403dcbe7: 48 89 d1                    	mov	rcx, rdx
1403dcbea: 48 f7 d9                    	neg	rcx
1403dcbed: 0f 91 c1                    	setno	cl
1403dcbf0: 84 c8                       	test	al, cl
1403dcbf2: 74 17                       	je	0x1403dcc0b <.text+0x3dbc0b>
1403dcbf4: 48 85 d2                    	test	rdx, rdx
1403dcbf7: 74 12                       	je	0x1403dcc0b <.text+0x3dbc0b>
1403dcbf9: 48 8b 8d d0 05 00 00        	mov	rcx, qword ptr [rbp + 0x5d0]
1403dcc00: 41 b8 01 00 00 00           	mov	r8d, 0x1
1403dcc06: e8 15 70 23 00              	call	0x140613c20 <.text+0x612c20>
1403dcc0b: 48 8b 95 e8 05 00 00        	mov	rdx, qword ptr [rbp + 0x5e8]
1403dcc12: 48 89 d0                    	mov	rax, rdx
1403dcc15: 48 f7 d8                    	neg	rax
1403dcc18: 70 14                       	jo	0x1403dcc2e <.text+0x3dbc2e>
1403dcc1a: 73 12                       	jae	0x1403dcc2e <.text+0x3dbc2e>
1403dcc1c: 48 8b 8d f0 05 00 00        	mov	rcx, qword ptr [rbp + 0x5f0]
1403dcc23: 41 b8 01 00 00 00           	mov	r8d, 0x1
1403dcc29: e8 f2 6f 23 00              	call	0x140613c20 <.text+0x612c20>
1403dcc2e: 48 85 f6                    	test	rsi, rsi
1403dcc31: 0f 84 a5 01 00 00           	je	0x1403dcddc <.text+0x3dbddc>
1403dcc37: 48 89 75 b0                 	mov	qword ptr [rbp - 0x50], rsi
1403dcc3b: 48 8d 75 b0                 	lea	rsi, [rbp - 0x50]
1403dcc3f: 48 89 b5 f0 06 00 00        	mov	qword ptr [rbp + 0x6f0], rsi
1403dcc46: 48 8d 3d d3 ff 96 00        	lea	rdi, [rip + 0x96ffd3]   # 0x140d4cc20 <.text+0xd4bc20>
1403dcc4d: 48 89 bd f8 06 00 00        	mov	qword ptr [rbp + 0x6f8], rdi
1403dcc54: 48 8d 05 f5 6d 5f 02        	lea	rax, [rip + 0x25f6df5]  # 0x1429d3a50
1403dcc5b: 48 89 85 20 09 00 00        	mov	qword ptr [rbp + 0x920], rax
1403dcc62: 48 c7 85 28 09 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x928], 0x2
1403dcc6d: 48 c7 85 40 09 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x940], 0x0
1403dcc78: 48 8d 85 f0 06 00 00        	lea	rax, [rbp + 0x6f0]
1403dcc7f: 48 89 85 30 09 00 00        	mov	qword ptr [rbp + 0x930], rax
1403dcc86: 48 c7 85 38 09 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x938], 0x1
1403dcc91: 48 8d 8d 20 09 00 00        	lea	rcx, [rbp + 0x920]
1403dcc98: e8 03 fe 97 00              	call	0x140d5caa0 <.text+0xd5baa0>
1403dcc9d: 0f 57 c0                    	xorps	xmm0, xmm0
1403dcca0: 0f 11 85 30 09 00 00        	movups	xmmword ptr [rbp + 0x930], xmm0
1403dcca7: 48 c7 85 20 09 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x920], 0x0
1403dccb2: 48 c7 85 28 09 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x928], 0x1
1403dccbd: 48 c7 85 40 09 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x940], 0x1
1403dccc8: 48 c7 85 48 09 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x948], 0x0
1403dccd3: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
1403dccdd: 48 89 85 50 09 00 00        	mov	qword ptr [rbp + 0x950], rax
1403dcce4: c7 85 80 09 00 00 0d 00 00 00       	mov	dword ptr [rbp + 0x980], 0xd
1403dccee: c6 85 a0 09 00 00 02        	mov	byte ptr [rbp + 0x9a0], 0x2
1403dccf5: 4c 8d 05 74 6d 5f 02        	lea	r8, [rip + 0x25f6d74]   # 0x1429d3a70
1403dccfc: 48 8d 8d f0 06 00 00        	lea	rcx, [rbp + 0x6f0]
1403dcd03: 48 8d 95 20 09 00 00        	lea	rdx, [rbp + 0x920]
1403dcd0a: 41 b9 10 00 00 00           	mov	r9d, 0x10
1403dcd10: e8 4b 38 7a 00              	call	0x140b80560 <.text+0xb7f560>
1403dcd15: 48 89 b5 90 02 00 00        	mov	qword ptr [rbp + 0x290], rsi
1403dcd1c: 48 89 bd 98 02 00 00        	mov	qword ptr [rbp + 0x298], rdi
1403dcd23: 48 8d 05 a6 6c 5f 02        	lea	rax, [rip + 0x25f6ca6]  # 0x1429d39d0
1403dcd2a: 48 89 85 20 09 00 00        	mov	qword ptr [rbp + 0x920], rax
1403dcd31: 48 c7 85 28 09 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x928], 0x1
1403dcd3c: 48 8d 85 90 02 00 00        	lea	rax, [rbp + 0x290]
1403dcd43: 48 89 85 30 09 00 00        	mov	qword ptr [rbp + 0x930], rax
1403dcd4a: 48 c7 85 38 09 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x938], 0x1
1403dcd55: 48 c7 85 40 09 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x940], 0x0
1403dcd60: 48 8d 8d 08 02 00 00        	lea	rcx, [rbp + 0x208]
1403dcd67: 48 8d 95 20 09 00 00        	lea	rdx, [rbp + 0x920]
1403dcd6e: e8 1d d2 99 00              	call	0x140d79f90 <.text+0xd78f90>
1403dcd73: 48 8b 85 08 02 00 00        	mov	rax, qword ptr [rbp + 0x208]
1403dcd7a: 48 89 85 88 0b 00 00        	mov	qword ptr [rbp + 0xb88], rax
1403dcd81: 4c 8b 85 10 02 00 00        	mov	r8, qword ptr [rbp + 0x210]
1403dcd88: 4c 8b 8d 18 02 00 00        	mov	r9, qword ptr [rbp + 0x218]
1403dcd8f: 48 8d 8d 20 09 00 00        	lea	rcx, [rbp + 0x920]
1403dcd96: 48 8d 95 f0 06 00 00        	lea	rdx, [rbp + 0x6f0]
1403dcd9d: 4c 89 85 58 0b 00 00        	mov	qword ptr [rbp + 0xb58], r8
1403dcda4: e8 b7 38 7a 00              	call	0x140b80660 <.text+0xb7f660>
1403dcda9: 48 8d 8d 20 09 00 00        	lea	rcx, [rbp + 0x920]
1403dcdb0: e8 7b 3a 7a 00              	call	0x140b80830 <.text+0xb7f830>
1403dcdb5: 48 8b 95 88 0b 00 00        	mov	rdx, qword ptr [rbp + 0xb88]
1403dcdbc: 48 85 d2                    	test	rdx, rdx
1403dcdbf: 74 12                       	je	0x1403dcdd3 <.text+0x3dbdd3>
1403dcdc1: 41 b8 01 00 00 00           	mov	r8d, 0x1
1403dcdc7: 48 8b 8d 58 0b 00 00        	mov	rcx, qword ptr [rbp + 0xb58]
1403dcdce: e8 4d 6e 23 00              	call	0x140613c20 <.text+0x612c20>
1403dcdd3: 48 8d 4d b0                 	lea	rcx, [rbp - 0x50]
1403dcdd7: e8 54 fe 96 00              	call	0x140d4cc30 <.text+0xd4bc30>
1403dcddc: 0f 28 b5 e0 0b 00 00        	movaps	xmm6, xmmword ptr [rbp + 0xbe0]
1403dcde3: 48 81 c4 78 0c 00 00        	add	rsp, 0xc78
1403dcdea: 5b                          	pop	rbx
1403dcdeb: 5f                          	pop	rdi
1403dcdec: 5e                          	pop	rsi
1403dcded: 41 5c                       	pop	r12
1403dcdef: 41 5d                       	pop	r13
1403dcdf1: 41 5e                       	pop	r14
1403dcdf3: 41 5f                       	pop	r15
1403dcdf5: 5d                          	pop	rbp
1403dcdf6: c3                          	ret
1403dcdf7: 48 8d 95 30 09 00 00        	lea	rdx, [rbp + 0x930]
1403dcdfe: 48 8d 8d a0 02 00 00        	lea	rcx, [rbp + 0x2a0]
1403dce05: 41 b8 18 01 00 00           	mov	r8d, 0x118
1403dce0b: e8 f0 6d 54 02              	call	0x142923c00 <.text+0x2922c00>
1403dce10: 48 89 bd 90 02 00 00        	mov	qword ptr [rbp + 0x290], rdi
1403dce17: 48 89 b5 98 02 00 00        	mov	qword ptr [rbp + 0x298], rsi
1403dce1e: 48 83 bd 90 0b 00 00 00     	cmp	qword ptr [rbp + 0xb90], 0x0
1403dce26: 0f 84 34 01 00 00           	je	0x1403dcf60 <.text+0x3dbf60>
1403dce2c: c6 85 cc 0b 00 00 01        	mov	byte ptr [rbp + 0xbcc], 0x1
1403dce33: c6 85 cb 0b 00 00 01        	mov	byte ptr [rbp + 0xbcb], 0x1
1403dce3a: c6 85 ca 0b 00 00 01        	mov	byte ptr [rbp + 0xbca], 0x1
1403dce41: c6 85 c9 0b 00 00 01        	mov	byte ptr [rbp + 0xbc9], 0x1
1403dce48: 48 8d 8d 90 0b 00 00        	lea	rcx, [rbp + 0xb90]
1403dce4f: e8 2c 0f 2a 00              	call	0x14067dd80 <.text+0x67cd80>
1403dce54: 48 89 c6                    	mov	rsi, rax
1403dce57: e8 14 67 54 02              	call	0x142923570 <.text+0x2922570>
1403dce5c: 48 8b 0e                    	mov	rcx, qword ptr [rsi]
1403dce5f: f0                          	lock
1403dce60: 48 ff 01                    	inc	qword ptr [rcx]
1403dce63: 0f 8e 6b 0c 00 00           	jle	0x1403ddad4 <.text+0x3dcad4>
1403dce69: 48 89 8d 20 09 00 00        	mov	qword ptr [rbp + 0x920], rcx
1403dce70: 48 89 85 28 09 00 00        	mov	qword ptr [rbp + 0x928], rax
1403dce77: 48 89 c1                    	mov	rcx, rax
1403dce7a: e8 71 66 54 02              	call	0x1429234f0 <.text+0x29224f0>
1403dce7f: 84 c0                       	test	al, al
1403dce81: 74 56                       	je	0x1403dced9 <.text+0x3dbed9>
1403dce83: 48 8b 85 20 09 00 00        	mov	rax, qword ptr [rbp + 0x920]
1403dce8a: f0                          	lock
1403dce8b: 48 ff 08                    	dec	qword ptr [rax]
1403dce8e: 75 28                       	jne	0x1403dceb8 <.text+0x3dbeb8>
1403dce90: c6 85 cc 0b 00 00 01        	mov	byte ptr [rbp + 0xbcc], 0x1
1403dce97: c6 85 cb 0b 00 00 01        	mov	byte ptr [rbp + 0xbcb], 0x1
1403dce9e: c6 85 ca 0b 00 00 01        	mov	byte ptr [rbp + 0xbca], 0x1
1403dcea5: c6 85 c9 0b 00 00 01        	mov	byte ptr [rbp + 0xbc9], 0x1
1403dceac: 48 8d 8d 20 09 00 00        	lea	rcx, [rbp + 0x920]
1403dceb3: e8 98 84 7a 00              	call	0x140b85350 <.text+0xb84350>
1403dceb8: 80 bd ac 03 00 00 06        	cmp	byte ptr [rbp + 0x3ac], 0x6
1403dcebf: 74 4f                       	je	0x1403dcf10 <.text+0x3dbf10>
1403dcec1: 80 bd ad 03 00 00 06        	cmp	byte ptr [rbp + 0x3ad], 0x6
1403dcec8: 74 56                       	je	0x1403dcf20 <.text+0x3dbf20>
1403dceca: 83 bd 24 03 00 00 00        	cmp	dword ptr [rbp + 0x324], 0x0
1403dced1: 0f 85 89 00 00 00           	jne	0x1403dcf60 <.text+0x3dbf60>
1403dced7: eb 57                       	jmp	0x1403dcf30 <.text+0x3dbf30>
1403dced9: 48 8b 85 20 09 00 00        	mov	rax, qword ptr [rbp + 0x920]
1403dcee0: f0                          	lock
1403dcee1: 48 ff 08                    	dec	qword ptr [rax]
1403dcee4: 75 7a                       	jne	0x1403dcf60 <.text+0x3dbf60>
1403dcee6: c6 85 cc 0b 00 00 01        	mov	byte ptr [rbp + 0xbcc], 0x1
1403dceed: c6 85 cb 0b 00 00 01        	mov	byte ptr [rbp + 0xbcb], 0x1
1403dcef4: c6 85 ca 0b 00 00 01        	mov	byte ptr [rbp + 0xbca], 0x1
1403dcefb: c6 85 c9 0b 00 00 01        	mov	byte ptr [rbp + 0xbc9], 0x1
1403dcf02: 48 8d 8d 20 09 00 00        	lea	rcx, [rbp + 0x920]
1403dcf09: e8 42 84 7a 00              	call	0x140b85350 <.text+0xb84350>
1403dcf0e: eb 50                       	jmp	0x1403dcf60 <.text+0x3dbf60>
1403dcf10: c6 85 ac 03 00 00 03        	mov	byte ptr [rbp + 0x3ac], 0x3
1403dcf17: 80 bd ad 03 00 00 06        	cmp	byte ptr [rbp + 0x3ad], 0x6
1403dcf1e: 75 aa                       	jne	0x1403dceca <.text+0x3dbeca>
1403dcf20: c6 85 ad 03 00 00 03        	mov	byte ptr [rbp + 0x3ad], 0x3
1403dcf27: 83 bd 24 03 00 00 00        	cmp	dword ptr [rbp + 0x324], 0x0
1403dcf2e: 75 30                       	jne	0x1403dcf60 <.text+0x3dbf60>
1403dcf30: c6 85 3b 03 00 00 01        	mov	byte ptr [rbp + 0x33b], 0x1
1403dcf37: c6 85 53 03 00 00 03        	mov	byte ptr [rbp + 0x353], 0x3
1403dcf3e: c7 85 18 03 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x318], 0x0
1403dcf48: 66 c7 85 39 03 00 00 00 01  	mov	word ptr [rbp + 0x339], 0x100
1403dcf51: f3 0f 10 05 db 00 5f 02     	movss	xmm0, dword ptr [rip + 0x25f00db] # 0x1429cd034
1403dcf59: 0f 13 85 28 03 00 00        	movlps	qword ptr [rbp + 0x328], xmm0
1403dcf60: c7 85 24 03 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x324], 0x1
1403dcf6a: 0f b6 85 76 03 00 00        	movzx	eax, byte ptr [rbp + 0x376]
1403dcf71: 48 8b 0d 20 f1 24 03        	mov	rcx, qword ptr [rip + 0x324f120] # 0x14362c098
1403dcf78: 88 01                       	mov	byte ptr [rcx], al
1403dcf7a: 80 bd 70 03 00 00 00        	cmp	byte ptr [rbp + 0x370], 0x0
1403dcf81: 0f 84 ad 00 00 00           	je	0x1403dd034 <.text+0x3dc034>
1403dcf87: 48 8b 85 88 0b 00 00        	mov	rax, qword ptr [rbp + 0xb88]
1403dcf8e: 48 89 85 20 09 00 00        	mov	qword ptr [rbp + 0x920], rax
1403dcf95: 48 8b 85 58 0b 00 00        	mov	rax, qword ptr [rbp + 0xb58]
1403dcf9c: 48 89 85 28 09 00 00        	mov	qword ptr [rbp + 0x928], rax
1403dcfa3: 0f 28 85 80 02 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x280]
1403dcfaa: 0f 11 85 30 09 00 00        	movups	xmmword ptr [rbp + 0x930], xmm0
1403dcfb1: 48 c7 85 40 09 00 00 88 13 00 00    	mov	qword ptr [rbp + 0x940], 0x1388
1403dcfbc: 48 c7 85 48 09 00 00 b8 0b 00 00    	mov	qword ptr [rbp + 0x948], 0xbb8
1403dcfc7: 44 0f b6 85 71 03 00 00     	movzx	r8d, byte ptr [rbp + 0x371]
1403dcfcf: c6 85 cc 0b 00 00 01        	mov	byte ptr [rbp + 0xbcc], 0x1
1403dcfd6: c6 85 cb 0b 00 00 00        	mov	byte ptr [rbp + 0xbcb], 0x0
1403dcfdd: c6 85 ca 0b 00 00 01        	mov	byte ptr [rbp + 0xbca], 0x1
1403dcfe4: c6 85 c9 0b 00 00 01        	mov	byte ptr [rbp + 0xbc9], 0x1
1403dcfeb: 48 8d 8d f0 06 00 00        	lea	rcx, [rbp + 0x6f0]
1403dcff2: 48 8d 95 20 09 00 00        	lea	rdx, [rbp + 0x920]
1403dcff9: e8 b2 de d7 ff              	call	0x14015aeb0 <.text+0x159eb0>
1403dcffe: 0f 10 85 f0 06 00 00        	movups	xmm0, xmmword ptr [rbp + 0x6f0]
1403dd005: 0f 10 8d 00 07 00 00        	movups	xmm1, xmmword ptr [rbp + 0x700]
1403dd00c: 0f 29 85 90 06 00 00        	movaps	xmmword ptr [rbp + 0x690], xmm0
1403dd013: 0f 29 8d a0 06 00 00        	movaps	xmmword ptr [rbp + 0x6a0], xmm1
1403dd01a: 48 8b 85 10 07 00 00        	mov	rax, qword ptr [rbp + 0x710]
1403dd021: 48 89 85 b0 06 00 00        	mov	qword ptr [rbp + 0x6b0], rax
1403dd028: c7 85 a8 0b 00 00 00 00 00 00       	mov	dword ptr [rbp + 0xba8], 0x0
1403dd032: eb 13                       	jmp	0x1403dd047 <.text+0x3dc047>
1403dd034: 48 c7 85 90 06 00 00 0b 00 00 00    	mov	qword ptr [rbp + 0x690], 0xb
1403dd03f: b0 01                       	mov	al, 0x1
1403dd041: 89 85 a8 0b 00 00           	mov	dword ptr [rbp + 0xba8], eax
1403dd047: c6 85 d5 0b 00 00 01        	mov	byte ptr [rbp + 0xbd5], 0x1
1403dd04e: c6 85 d4 0b 00 00 01        	mov	byte ptr [rbp + 0xbd4], 0x1
1403dd055: c6 85 d3 0b 00 00 01        	mov	byte ptr [rbp + 0xbd3], 0x1
1403dd05c: c6 85 d2 0b 00 00 01        	mov	byte ptr [rbp + 0xbd2], 0x1
1403dd063: c6 85 d1 0b 00 00 01        	mov	byte ptr [rbp + 0xbd1], 0x1
1403dd06a: 40 b7 01                    	mov	dil, 0x1
1403dd06d: b9 03 00 00 00              	mov	ecx, 0x3
1403dd072: b2 01                       	mov	dl, 0x1
1403dd074: e8 e7 76 f0 ff              	call	0x1402e4760 <.text+0x2e3760>
1403dd079: 48 89 c6                    	mov	rsi, rax
1403dd07c: b3 01                       	mov	bl, 0x1
1403dd07e: 48 85 c0                    	test	rax, rax
1403dd081: 74 55                       	je	0x1403dd0d8 <.text+0x3dc0d8>
1403dd083: 48 8b 85 90 06 00 00        	mov	rax, qword ptr [rbp + 0x690]
1403dd08a: 48 83 f8 0a                 	cmp	rax, 0xa
1403dd08e: 0f 84 bd 02 00 00           	je	0x1403dd351 <.text+0x3dc351>
1403dd094: 83 f8 0b                    	cmp	eax, 0xb
1403dd097: 8b bd a8 0b 00 00           	mov	edi, dword ptr [rbp + 0xba8]
1403dd09d: 0f 84 e1 02 00 00           	je	0x1403dd384 <.text+0x3dc384>
1403dd0a3: 84 db                       	test	bl, bl
1403dd0a5: 0f 84 d9 02 00 00           	je	0x1403dd384 <.text+0x3dc384>
1403dd0ab: c6 85 cc 0b 00 00 01        	mov	byte ptr [rbp + 0xbcc], 0x1
1403dd0b2: 40 88 bd cb 0b 00 00        	mov	byte ptr [rbp + 0xbcb], dil
1403dd0b9: c6 85 ca 0b 00 00 01        	mov	byte ptr [rbp + 0xbca], 0x1
1403dd0c0: c6 85 c9 0b 00 00 01        	mov	byte ptr [rbp + 0xbc9], 0x1
1403dd0c7: 48 8d 8d 90 06 00 00        	lea	rcx, [rbp + 0x690]
1403dd0ce: e8 9d de ff ff              	call	0x1403daf70 <.text+0x3d9f70>
1403dd0d3: e9 ac 02 00 00              	jmp	0x1403dd384 <.text+0x3dc384>
1403dd0d8: c6 85 20 09 00 00 01        	mov	byte ptr [rbp + 0x920], 0x1
1403dd0df: c6 85 d5 0b 00 00 01        	mov	byte ptr [rbp + 0xbd5], 0x1
1403dd0e6: c6 85 d4 0b 00 00 01        	mov	byte ptr [rbp + 0xbd4], 0x1
1403dd0ed: c6 85 d3 0b 00 00 01        	mov	byte ptr [rbp + 0xbd3], 0x1
1403dd0f4: c6 85 d2 0b 00 00 01        	mov	byte ptr [rbp + 0xbd2], 0x1
1403dd0fb: c6 85 d1 0b 00 00 01        	mov	byte ptr [rbp + 0xbd1], 0x1
1403dd102: 48 8d 15 89 69 5f 02        	lea	rdx, [rip + 0x25f6989]  # 0x1429d3a92
1403dd109: 48 8d b5 20 09 00 00        	lea	rsi, [rbp + 0x920]
1403dd110: 41 b8 03 00 00 00           	mov	r8d, 0x3
1403dd116: 48 89 f1                    	mov	rcx, rsi
1403dd119: e8 32 d0 97 00              	call	0x140d5a150 <.text+0xd59150>
1403dd11e: 48 85 c0                    	test	rax, rax
1403dd121: 0f 84 f0 00 00 00           	je	0x1403dd217 <.text+0x3dc217>
1403dd127: 48 89 85 30 06 00 00        	mov	qword ptr [rbp + 0x630], rax
1403dd12e: 48 8b 05 73 f2 24 03        	mov	rax, qword ptr [rip + 0x324f273] # 0x14362c3a8
1403dd135: 48 8b 00                    	mov	rax, qword ptr [rax]
1403dd138: 48 83 f8 02                 	cmp	rax, 0x2
1403dd13c: 0f 82 a6 00 00 00           	jb	0x1403dd1e8 <.text+0x3dc1e8>
1403dd142: 48 8d 85 30 06 00 00        	lea	rax, [rbp + 0x630]
1403dd149: 48 89 45 b0                 	mov	qword ptr [rbp - 0x50], rax
1403dd14d: 48 8d 05 8c db 97 00        	lea	rax, [rip + 0x97db8c]   # 0x140d5ace0 <.text+0xd59ce0>
1403dd154: 48 89 45 b8                 	mov	qword ptr [rbp - 0x48], rax
1403dd158: 48 8d 05 61 6c 5f 02        	lea	rax, [rip + 0x25f6c61]  # 0x1429d3dc0
1403dd15f: 48 89 85 20 09 00 00        	mov	qword ptr [rbp + 0x920], rax
1403dd166: 48 c7 85 28 09 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x928], 0x1
1403dd171: 48 c7 85 40 09 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x940], 0x0
1403dd17c: 48 8d 45 b0                 	lea	rax, [rbp - 0x50]
1403dd180: 48 89 85 30 09 00 00        	mov	qword ptr [rbp + 0x930], rax
1403dd187: 48 c7 85 38 09 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x938], 0x1
1403dd192: 48 8d 0d 37 6c 5f 02        	lea	rcx, [rip + 0x25f6c37]  # 0x1429d3dd0
1403dd199: e8 e2 0b 2a 00              	call	0x14067dd80 <.text+0x67cd80>
1403dd19e: 48 8d 0d 9b 68 5f 02        	lea	rcx, [rip + 0x25f689b]  # 0x1429d3a40
1403dd1a5: 48 89 8d f0 06 00 00        	mov	qword ptr [rbp + 0x6f0], rcx
1403dd1ac: 48 c7 85 f8 06 00 00 0a 00 00 00    	mov	qword ptr [rbp + 0x6f8], 0xa
1403dd1b7: 48 89 8d 00 07 00 00        	mov	qword ptr [rbp + 0x700], rcx
1403dd1be: 48 c7 85 08 07 00 00 0a 00 00 00    	mov	qword ptr [rbp + 0x708], 0xa
1403dd1c9: 48 89 85 10 07 00 00        	mov	qword ptr [rbp + 0x710], rax
1403dd1d0: 48 8d 8d 20 09 00 00        	lea	rcx, [rbp + 0x920]
1403dd1d7: 4c 8d 85 f0 06 00 00        	lea	r8, [rbp + 0x6f0]
1403dd1de: ba 02 00 00 00              	mov	edx, 0x2
1403dd1e3: e8 48 42 0d 00              	call	0x1404b1430 <.text+0x4b0430>
1403dd1e8: c6 85 d5 0b 00 00 01        	mov	byte ptr [rbp + 0xbd5], 0x1
1403dd1ef: c6 85 d4 0b 00 00 01        	mov	byte ptr [rbp + 0xbd4], 0x1
1403dd1f6: c6 85 d3 0b 00 00 01        	mov	byte ptr [rbp + 0xbd3], 0x1
1403dd1fd: c6 85 d2 0b 00 00 01        	mov	byte ptr [rbp + 0xbd2], 0x1
1403dd204: c6 85 d1 0b 00 00 01        	mov	byte ptr [rbp + 0xbd1], 0x1
1403dd20b: 48 8d 8d 30 06 00 00        	lea	rcx, [rbp + 0x630]
1403dd212: e8 e9 a7 20 00              	call	0x1405e7a00 <.text+0x5e6a00>
1403dd217: 48 8b 3d 8a f1 24 03        	mov	rdi, qword ptr [rip + 0x324f18a] # 0x14362c3a8
1403dd21e: 48 8b 07                    	mov	rax, qword ptr [rdi]
1403dd221: 48 83 f8 04                 	cmp	rax, 0x4
1403dd225: 0f 82 ec 00 00 00           	jb	0x1403dd317 <.text+0x3dc317>
1403dd22b: 48 8d 05 b6 6b 5f 02        	lea	rax, [rip + 0x25f6bb6]  # 0x1429d3de8
1403dd232: 48 89 45 b0                 	mov	qword ptr [rbp - 0x50], rax
1403dd236: 48 8d 05 53 cc d2 ff        	lea	rax, [rip - 0x2d33ad]   # 0x140109e90 <.text+0x108e90>
1403dd23d: 48 89 45 b8                 	mov	qword ptr [rbp - 0x48], rax
1403dd241: 48 8d 05 c0 6b 5f 02        	lea	rax, [rip + 0x25f6bc0]  # 0x1429d3e08
1403dd248: 48 89 85 20 09 00 00        	mov	qword ptr [rbp + 0x920], rax
1403dd24f: 48 c7 85 28 09 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x928], 0x1
1403dd25a: 48 c7 85 40 09 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x940], 0x0
1403dd265: 48 8d 45 b0                 	lea	rax, [rbp - 0x50]
1403dd269: 48 89 85 30 09 00 00        	mov	qword ptr [rbp + 0x930], rax
1403dd270: 48 c7 85 38 09 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x938], 0x1
1403dd27b: c6 85 d5 0b 00 00 01        	mov	byte ptr [rbp + 0xbd5], 0x1
1403dd282: c6 85 d4 0b 00 00 01        	mov	byte ptr [rbp + 0xbd4], 0x1
1403dd289: c6 85 d3 0b 00 00 01        	mov	byte ptr [rbp + 0xbd3], 0x1
1403dd290: c6 85 d2 0b 00 00 01        	mov	byte ptr [rbp + 0xbd2], 0x1
1403dd297: c6 85 d1 0b 00 00 01        	mov	byte ptr [rbp + 0xbd1], 0x1
1403dd29e: 48 8d 0d 73 6b 5f 02        	lea	rcx, [rip + 0x25f6b73]  # 0x1429d3e18
1403dd2a5: e8 d6 0a 2a 00              	call	0x14067dd80 <.text+0x67cd80>
1403dd2aa: 48 8d 0d 8f 67 5f 02        	lea	rcx, [rip + 0x25f678f]  # 0x1429d3a40
1403dd2b1: 48 89 8d f0 06 00 00        	mov	qword ptr [rbp + 0x6f0], rcx
1403dd2b8: 48 c7 85 f8 06 00 00 0a 00 00 00    	mov	qword ptr [rbp + 0x6f8], 0xa
1403dd2c3: 48 89 8d 00 07 00 00        	mov	qword ptr [rbp + 0x700], rcx
1403dd2ca: 48 c7 85 08 07 00 00 0a 00 00 00    	mov	qword ptr [rbp + 0x708], 0xa
1403dd2d5: 48 89 85 10 07 00 00        	mov	qword ptr [rbp + 0x710], rax
1403dd2dc: c6 85 d5 0b 00 00 01        	mov	byte ptr [rbp + 0xbd5], 0x1
1403dd2e3: c6 85 d4 0b 00 00 01        	mov	byte ptr [rbp + 0xbd4], 0x1
1403dd2ea: c6 85 d3 0b 00 00 01        	mov	byte ptr [rbp + 0xbd3], 0x1
1403dd2f1: c6 85 d2 0b 00 00 01        	mov	byte ptr [rbp + 0xbd2], 0x1
1403dd2f8: c6 85 d1 0b 00 00 01        	mov	byte ptr [rbp + 0xbd1], 0x1
1403dd2ff: 48 8d 8d 20 09 00 00        	lea	rcx, [rbp + 0x920]
1403dd306: 4c 8d 85 f0 06 00 00        	lea	r8, [rbp + 0x6f0]
1403dd30d: ba 04 00 00 00              	mov	edx, 0x4
1403dd312: e8 19 41 0d 00              	call	0x1404b1430 <.text+0x4b0430>
1403dd317: 48 8b 85 90 06 00 00        	mov	rax, qword ptr [rbp + 0x690]
1403dd31e: 48 83 f8 0a                 	cmp	rax, 0xa
1403dd322: 0f 84 82 00 00 00           	je	0x1403dd3aa <.text+0x3dc3aa>
1403dd328: 83 f8 0b                    	cmp	eax, 0xb
1403dd32b: 0f 85 9e 00 00 00           	jne	0x1403dd3cf <.text+0x3dc3cf>
1403dd331: 48 c7 85 70 0b 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xb70], 0x0
1403dd33c: b0 01                       	mov	al, 0x1
1403dd33e: 89 85 98 0b 00 00           	mov	dword ptr [rbp + 0xb98], eax
1403dd344: b0 01                       	mov	al, 0x1
1403dd346: 89 85 a0 0b 00 00           	mov	dword ptr [rbp + 0xba0], eax
1403dd34c: e9 dd 01 00 00              	jmp	0x1403dd52e <.text+0x3dc52e>
1403dd351: 40 84 ff                    	test	dil, dil
1403dd354: 8b bd a8 0b 00 00           	mov	edi, dword ptr [rbp + 0xba8]
1403dd35a: 74 28                       	je	0x1403dd384 <.text+0x3dc384>
1403dd35c: 48 8d 8d 98 06 00 00        	lea	rcx, [rbp + 0x698]
1403dd363: c6 85 cc 0b 00 00 01        	mov	byte ptr [rbp + 0xbcc], 0x1
1403dd36a: 40 88 bd cb 0b 00 00        	mov	byte ptr [rbp + 0xbcb], dil
1403dd371: c6 85 ca 0b 00 00 01        	mov	byte ptr [rbp + 0xbca], 0x1
1403dd378: c6 85 c9 0b 00 00 01        	mov	byte ptr [rbp + 0xbc9], 0x1
1403dd37f: e8 8c 7f 21 00              	call	0x1405f5310 <.text+0x5f4310>
1403dd384: 40 88 bd c0 0b 00 00        	mov	byte ptr [rbp + 0xbc0], dil
1403dd38b: c6 85 bf 0b 00 00 01        	mov	byte ptr [rbp + 0xbbf], 0x1
1403dd392: c6 85 be 0b 00 00 01        	mov	byte ptr [rbp + 0xbbe], 0x1
1403dd399: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
1403dd3a0: e8 ab 50 21 00              	call	0x1405f2450 <.text+0x5f1450>
1403dd3a5: e9 3c f5 ff ff              	jmp	0x1403dc8e6 <.text+0x3db8e6>
1403dd3aa: 0f 10 85 98 06 00 00        	movups	xmm0, xmmword ptr [rbp + 0x698]
1403dd3b1: 0f 29 85 70 0b 00 00        	movaps	xmmword ptr [rbp + 0xb70], xmm0
1403dd3b8: b0 01                       	mov	al, 0x1
1403dd3ba: 89 85 a0 0b 00 00           	mov	dword ptr [rbp + 0xba0], eax
1403dd3c0: c7 85 98 0b 00 00 00 00 00 00       	mov	dword ptr [rbp + 0xb98], 0x0
1403dd3ca: e9 5f 01 00 00              	jmp	0x1403dd52e <.text+0x3dc52e>
1403dd3cf: 48 8b 85 b0 06 00 00        	mov	rax, qword ptr [rbp + 0x6b0]
1403dd3d6: 48 89 85 50 06 00 00        	mov	qword ptr [rbp + 0x650], rax
1403dd3dd: 0f 28 85 90 06 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x690]
1403dd3e4: 0f 28 8d a0 06 00 00        	movaps	xmm1, xmmword ptr [rbp + 0x6a0]
1403dd3eb: 0f 29 8d 40 06 00 00        	movaps	xmmword ptr [rbp + 0x640], xmm1
1403dd3f2: 0f 29 85 30 06 00 00        	movaps	xmmword ptr [rbp + 0x630], xmm0
1403dd3f9: 48 8b 07                    	mov	rax, qword ptr [rdi]
1403dd3fc: 48 85 c0                    	test	rax, rax
1403dd3ff: 0f 84 dd 00 00 00           	je	0x1403dd4e2 <.text+0x3dc4e2>
1403dd405: 48 8d 85 30 06 00 00        	lea	rax, [rbp + 0x630]
1403dd40c: 48 89 85 20 02 00 00        	mov	qword ptr [rbp + 0x220], rax
1403dd413: 48 8d 05 a6 e3 d7 ff        	lea	rax, [rip - 0x281c5a]   # 0x14015b7c0 <.text+0x15a7c0>
1403dd41a: 48 89 85 28 02 00 00        	mov	qword ptr [rbp + 0x228], rax
1403dd421: 48 c7 85 20 09 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x920], 0x2
1403dd42c: 48 c7 85 30 09 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x930], 0x2
1403dd437: 48 c7 85 40 09 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x940], 0x0
1403dd442: 48 b8 20 00 00 00 04 00 00 00       	movabs	rax, 0x400000020
1403dd44c: 48 89 85 48 09 00 00        	mov	qword ptr [rbp + 0x948], rax
1403dd453: c6 85 50 09 00 00 03        	mov	byte ptr [rbp + 0x950], 0x3
1403dd45a: 48 8d 05 f7 69 5f 02        	lea	rax, [rip + 0x25f69f7]  # 0x1429d3e58
1403dd461: 48 89 85 f0 06 00 00        	mov	qword ptr [rbp + 0x6f0], rax
1403dd468: 48 c7 85 f8 06 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x6f8], 0x1
1403dd473: 48 89 b5 10 07 00 00        	mov	qword ptr [rbp + 0x710], rsi
1403dd47a: 48 c7 85 18 07 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x718], 0x1
1403dd485: 48 8d 85 20 02 00 00        	lea	rax, [rbp + 0x220]
1403dd48c: 48 89 85 00 07 00 00        	mov	qword ptr [rbp + 0x700], rax
1403dd493: 48 c7 85 08 07 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x708], 0x1
1403dd49e: 48 8d 0d c3 69 5f 02        	lea	rcx, [rip + 0x25f69c3]  # 0x1429d3e68
1403dd4a5: e8 d6 08 2a 00              	call	0x14067dd80 <.text+0x67cd80>
1403dd4aa: 48 8d 0d 8f 65 5f 02        	lea	rcx, [rip + 0x25f658f]  # 0x1429d3a40
1403dd4b1: 48 89 4d b0                 	mov	qword ptr [rbp - 0x50], rcx
1403dd4b5: 48 c7 45 b8 0a 00 00 00     	mov	qword ptr [rbp - 0x48], 0xa
1403dd4bd: 48 89 4d c0                 	mov	qword ptr [rbp - 0x40], rcx
1403dd4c1: 48 c7 45 c8 0a 00 00 00     	mov	qword ptr [rbp - 0x38], 0xa
1403dd4c9: 48 89 45 d0                 	mov	qword ptr [rbp - 0x30], rax
1403dd4cd: 48 8d 8d f0 06 00 00        	lea	rcx, [rbp + 0x6f0]
1403dd4d4: 4c 8d 45 b0                 	lea	r8, [rbp - 0x50]
1403dd4d8: ba 01 00 00 00              	mov	edx, 0x1
1403dd4dd: e8 4e 3f 0d 00              	call	0x1404b1430 <.text+0x4b0430>
1403dd4e2: 48 c7 85 70 0b 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xb70], 0x0
1403dd4ed: c6 85 d5 0b 00 00 01        	mov	byte ptr [rbp + 0xbd5], 0x1
1403dd4f4: c6 85 d4 0b 00 00 01        	mov	byte ptr [rbp + 0xbd4], 0x1
1403dd4fb: c6 85 d3 0b 00 00 01        	mov	byte ptr [rbp + 0xbd3], 0x1
1403dd502: c6 85 d2 0b 00 00 01        	mov	byte ptr [rbp + 0xbd2], 0x1
1403dd509: c6 85 d1 0b 00 00 00        	mov	byte ptr [rbp + 0xbd1], 0x0
1403dd510: 48 8d 8d 30 06 00 00        	lea	rcx, [rbp + 0x630]
1403dd517: e8 54 da ff ff              	call	0x1403daf70 <.text+0x3d9f70>
1403dd51c: b0 01                       	mov	al, 0x1
1403dd51e: 89 85 98 0b 00 00           	mov	dword ptr [rbp + 0xb98], eax
1403dd524: c7 85 a0 0b 00 00 00 00 00 00       	mov	dword ptr [rbp + 0xba0], 0x0
1403dd52e: c6 85 c8 0b 00 00 01        	mov	byte ptr [rbp + 0xbc8], 0x1
1403dd535: c6 85 c7 0b 00 00 01        	mov	byte ptr [rbp + 0xbc7], 0x1
1403dd53c: c6 85 c6 0b 00 00 01        	mov	byte ptr [rbp + 0xbc6], 0x1
1403dd543: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
1403dd54a: e8 01 71 f7 ff              	call	0x140354650 <.text+0x353650>
1403dd54f: c6 85 c8 0b 00 00 01        	mov	byte ptr [rbp + 0xbc8], 0x1
1403dd556: c6 85 c7 0b 00 00 01        	mov	byte ptr [rbp + 0xbc7], 0x1
1403dd55d: c6 85 c6 0b 00 00 01        	mov	byte ptr [rbp + 0xbc6], 0x1
1403dd564: 48 8d 0d 4d 69 5f 02        	lea	rcx, [rip + 0x25f694d]  # 0x1429d3eb8
1403dd56b: 4c 8d 05 6b 69 5f 02        	lea	r8, [rip + 0x25f696b]   # 0x1429d3edd
1403dd572: ba 25 00 00 00              	mov	edx, 0x25
1403dd577: 41 b9 01 00 00 00           	mov	r9d, 0x1
1403dd57d: e8 6e 12 c5 ff              	call	0x14002e7f0 <.text+0x2d7f0>
1403dd582: 31 c0                       	xor	eax, eax
1403dd584: 48 3b 85 68 05 00 00        	cmp	rax, qword ptr [rbp + 0x568]
1403dd58b: 71 41                       	jno	0x1403dd5ce <.text+0x3dc5ce>
1403dd58d: 48 8b 8d 70 0b 00 00        	mov	rcx, qword ptr [rbp + 0xb70]
1403dd594: 48 8b 95 78 0b 00 00        	mov	rdx, qword ptr [rbp + 0xb78]
1403dd59b: 48 85 c9                    	test	rcx, rcx
1403dd59e: 74 14                       	je	0x1403dd5b4 <.text+0x3dc5b4>
1403dd5a0: f0                          	lock
1403dd5a1: 48 ff 01                    	inc	qword ptr [rcx]
1403dd5a4: 0f 8e 2a 05 00 00           	jle	0x1403ddad4 <.text+0x3dcad4>
1403dd5aa: f0                          	lock
1403dd5ab: 48 ff 02                    	inc	qword ptr [rdx]
1403dd5ae: 0f 8e 20 05 00 00           	jle	0x1403ddad4 <.text+0x3dcad4>
1403dd5b4: c6 85 c8 0b 00 00 01        	mov	byte ptr [rbp + 0xbc8], 0x1
1403dd5bb: c6 85 c7 0b 00 00 01        	mov	byte ptr [rbp + 0xbc7], 0x1
1403dd5c2: c6 85 c6 0b 00 00 01        	mov	byte ptr [rbp + 0xbc6], 0x1
1403dd5c9: e8 e2 df cf ff              	call	0x1400db5b0 <.text+0xda5b0>
1403dd5ce: 48 8b 85 90 0b 00 00        	mov	rax, qword ptr [rbp + 0xb90]
1403dd5d5: 48 85 c0                    	test	rax, rax
1403dd5d8: 48 8d 8d 90 0b 00 00        	lea	rcx, [rbp + 0xb90]
1403dd5df: 48 0f 44 c8                 	cmove	rcx, rax
1403dd5e3: 31 d2                       	xor	edx, edx
1403dd5e5: 48 3b 95 58 03 00 00        	cmp	rdx, qword ptr [rbp + 0x358]
1403dd5ec: 4c 8b 85 68 03 00 00        	mov	r8, qword ptr [rbp + 0x368]
1403dd5f3: 70 07                       	jo	0x1403dd5fc <.text+0x3dc5fc>
1403dd5f5: 48 8b 95 60 03 00 00        	mov	rdx, qword ptr [rbp + 0x360]
1403dd5fc: c6 85 c8 0b 00 00 01        	mov	byte ptr [rbp + 0xbc8], 0x1
1403dd603: c6 85 c7 0b 00 00 01        	mov	byte ptr [rbp + 0xbc7], 0x1
1403dd60a: c6 85 c6 0b 00 00 01        	mov	byte ptr [rbp + 0xbc6], 0x1
1403dd611: e8 1a 5c f4 ff              	call	0x140323230 <.text+0x322230>
1403dd616: 0f b6 85 72 03 00 00        	movzx	eax, byte ptr [rbp + 0x372]
1403dd61d: 48 8b 0d fc e9 24 03        	mov	rcx, qword ptr [rip + 0x324e9fc] # 0x14362c020
1403dd624: 88 01                       	mov	byte ptr [rcx], al
1403dd626: c6 85 c8 0b 00 00 01        	mov	byte ptr [rbp + 0xbc8], 0x1
1403dd62d: c6 85 c7 0b 00 00 01        	mov	byte ptr [rbp + 0xbc7], 0x1
1403dd634: c6 85 c6 0b 00 00 01        	mov	byte ptr [rbp + 0xbc6], 0x1
1403dd63b: e8 50 94 80 00              	call	0x140be6a90 <.text+0xbe5a90>
1403dd640: 80 bd 06 06 00 00 00        	cmp	byte ptr [rbp + 0x606], 0x0
1403dd647: 74 6a                       	je	0x1403dd6b3 <.text+0x3dc6b3>
1403dd649: c6 85 c8 0b 00 00 01        	mov	byte ptr [rbp + 0xbc8], 0x1
1403dd650: c6 85 c7 0b 00 00 01        	mov	byte ptr [rbp + 0xbc7], 0x1
1403dd657: c6 85 c6 0b 00 00 01        	mov	byte ptr [rbp + 0xbc6], 0x1
1403dd65e: e8 4d 7c e7 ff              	call	0x1402552b0 <.text+0x2542b0>
1403dd663: 48 89 c6                    	mov	rsi, rax
1403dd666: 48 83 bd 70 0b 00 00 00     	cmp	qword ptr [rbp + 0xb70], 0x0
1403dd66e: 8b bd 98 0b 00 00           	mov	edi, dword ptr [rbp + 0xb98]
1403dd674: 8b 9d a0 0b 00 00           	mov	ebx, dword ptr [rbp + 0xba0]
1403dd67a: 0f 84 03 fa ff ff           	je	0x1403dd083 <.text+0x3dc083>
1403dd680: c6 85 d5 0b 00 00 01        	mov	byte ptr [rbp + 0xbd5], 0x1
1403dd687: c6 85 d4 0b 00 00 01        	mov	byte ptr [rbp + 0xbd4], 0x1
1403dd68e: c6 85 d3 0b 00 00 01        	mov	byte ptr [rbp + 0xbd3], 0x1
1403dd695: 40 88 bd d2 0b 00 00        	mov	byte ptr [rbp + 0xbd2], dil
1403dd69c: 88 9d d1 0b 00 00           	mov	byte ptr [rbp + 0xbd1], bl
1403dd6a2: 48 8d 8d 70 0b 00 00        	lea	rcx, [rbp + 0xb70]
1403dd6a9: e8 62 7c 21 00              	call	0x1405f5310 <.text+0x5f4310>
1403dd6ae: e9 d0 f9 ff ff              	jmp	0x1403dd083 <.text+0x3dc083>
1403dd6b3: 80 bd 05 06 00 00 00        	cmp	byte ptr [rbp + 0x605], 0x0
1403dd6ba: 74 3b                       	je	0x1403dd6f7 <.text+0x3dc6f7>
1403dd6bc: 48 8d 95 90 02 00 00        	lea	rdx, [rbp + 0x290]
1403dd6c3: 41 b8 28 01 00 00           	mov	r8d, 0x128
1403dd6c9: 48 89 f1                    	mov	rcx, rsi
1403dd6cc: e8 2f 65 54 02              	call	0x142923c00 <.text+0x2922c00>
1403dd6d1: c6 85 c8 0b 00 00 00        	mov	byte ptr [rbp + 0xbc8], 0x0
1403dd6d8: c6 85 c7 0b 00 00 01        	mov	byte ptr [rbp + 0xbc7], 0x1
1403dd6df: c6 85 c6 0b 00 00 01        	mov	byte ptr [rbp + 0xbc6], 0x1
1403dd6e6: 48 89 f1                    	mov	rcx, rsi
1403dd6e9: e8 c2 9d 00 00              	call	0x1403e74b0 <.text+0x3e64b0>
1403dd6ee: b3 01                       	mov	bl, 0x1
1403dd6f0: 31 f6                       	xor	esi, esi
1403dd6f2: e9 cf 00 00 00              	jmp	0x1403dd7c6 <.text+0x3dc7c6>
1403dd6f7: 48 8b 85 e8 05 00 00        	mov	rax, qword ptr [rbp + 0x5e8]
1403dd6fe: 48 89 c1                    	mov	rcx, rax
1403dd701: 48 f7 d9                    	neg	rcx
1403dd704: 0f 81 81 00 00 00           	jno	0x1403dd78b <.text+0x3dc78b>
1403dd70a: 48 8d b5 f0 06 00 00        	lea	rsi, [rbp + 0x6f0]
1403dd711: 48 8d 95 90 02 00 00        	lea	rdx, [rbp + 0x290]
1403dd718: 41 b8 28 01 00 00           	mov	r8d, 0x128
1403dd71e: 48 89 f1                    	mov	rcx, rsi
1403dd721: e8 da 64 54 02              	call	0x142923c00 <.text+0x2922c00>
1403dd726: 48 8d bd 20 09 00 00        	lea	rdi, [rbp + 0x920]
1403dd72d: 48 8d 95 c0 04 00 00        	lea	rdx, [rbp + 0x4c0]
1403dd734: 41 b8 50 01 00 00           	mov	r8d, 0x150
1403dd73a: 48 89 f9                    	mov	rcx, rdi
1403dd73d: e8 be 64 54 02              	call	0x142923c00 <.text+0x2922c00>
1403dd742: 4c 8b 85 90 0b 00 00        	mov	r8, qword ptr [rbp + 0xb90]
1403dd749: 4c 8b 8d 70 0b 00 00        	mov	r9, qword ptr [rbp + 0xb70]
1403dd750: 48 8b 85 78 0b 00 00        	mov	rax, qword ptr [rbp + 0xb78]
1403dd757: c6 85 c8 0b 00 00 00        	mov	byte ptr [rbp + 0xbc8], 0x0
1403dd75e: c6 85 c7 0b 00 00 00        	mov	byte ptr [rbp + 0xbc7], 0x0
1403dd765: c6 85 c6 0b 00 00 00        	mov	byte ptr [rbp + 0xbc6], 0x0
1403dd76c: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1403dd771: 48 89 f1                    	mov	rcx, rsi
1403dd774: 48 89 fa                    	mov	rdx, rdi
1403dd777: e8 f4 63 1b 00              	call	0x140593b70 <.text+0x592b70>
1403dd77c: 31 db                       	xor	ebx, ebx
1403dd77e: 31 f6                       	xor	esi, esi
1403dd780: 8b bd a8 0b 00 00           	mov	edi, dword ptr [rbp + 0xba8]
1403dd786: e9 07 01 00 00              	jmp	0x1403dd892 <.text+0x3dc892>
1403dd78b: 0f 10 85 f0 05 00 00        	movups	xmm0, xmmword ptr [rbp + 0x5f0]
1403dd792: 0f 11 85 28 09 00 00        	movups	xmmword ptr [rbp + 0x928], xmm0
1403dd799: 48 89 85 20 09 00 00        	mov	qword ptr [rbp + 0x920], rax
1403dd7a0: c6 85 c8 0b 00 00 01        	mov	byte ptr [rbp + 0xbc8], 0x1
1403dd7a7: c6 85 c7 0b 00 00 00        	mov	byte ptr [rbp + 0xbc7], 0x0
1403dd7ae: c6 85 c6 0b 00 00 01        	mov	byte ptr [rbp + 0xbc6], 0x1
1403dd7b5: 48 8d 8d 20 09 00 00        	lea	rcx, [rbp + 0x920]
1403dd7bc: e8 7f 19 00 00              	call	0x1403df140 <.text+0x3de140>
1403dd7c1: 40 b6 01                    	mov	sil, 0x1
1403dd7c4: 31 db                       	xor	ebx, ebx
1403dd7c6: 48 83 bd 70 0b 00 00 00     	cmp	qword ptr [rbp + 0xb70], 0x0
1403dd7ce: 8b 85 98 0b 00 00           	mov	eax, dword ptr [rbp + 0xb98]
1403dd7d4: 8b 8d a0 0b 00 00           	mov	ecx, dword ptr [rbp + 0xba0]
1403dd7da: 74 2c                       	je	0x1403dd808 <.text+0x3dc808>
1403dd7dc: 40 88 b5 d5 0b 00 00        	mov	byte ptr [rbp + 0xbd5], sil
1403dd7e3: 88 9d d4 0b 00 00           	mov	byte ptr [rbp + 0xbd4], bl
1403dd7e9: c6 85 d3 0b 00 00 01        	mov	byte ptr [rbp + 0xbd3], 0x1
1403dd7f0: 88 85 d2 0b 00 00           	mov	byte ptr [rbp + 0xbd2], al
1403dd7f6: 88 8d d1 0b 00 00           	mov	byte ptr [rbp + 0xbd1], cl
1403dd7fc: 48 8d 8d 70 0b 00 00        	lea	rcx, [rbp + 0xb70]
1403dd803: e8 08 7b 21 00              	call	0x1405f5310 <.text+0x5f4310>
1403dd808: 40 84 f6                    	test	sil, sil
1403dd80b: 74 43                       	je	0x1403dd850 <.text+0x3dc850>
1403dd80d: 89 9d 98 0b 00 00           	mov	dword ptr [rbp + 0xb98], ebx
1403dd813: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
1403dd81a: e8 61 03 22 00              	call	0x1405fdb80 <.text+0x5fcb80>
1403dd81f: 48 8b 95 58 03 00 00        	mov	rdx, qword ptr [rbp + 0x358]
1403dd826: 48 89 d0                    	mov	rax, rdx
1403dd829: 48 f7 d8                    	neg	rax
1403dd82c: 70 14                       	jo	0x1403dd842 <.text+0x3dc842>
1403dd82e: 73 12                       	jae	0x1403dd842 <.text+0x3dc842>
1403dd830: 48 8b 8d 60 03 00 00        	mov	rcx, qword ptr [rbp + 0x360]
1403dd837: 41 b8 01 00 00 00           	mov	r8d, 0x1
1403dd83d: e8 de 63 23 00              	call	0x140613c20 <.text+0x612c20>
1403dd842: 8b bd a8 0b 00 00           	mov	edi, dword ptr [rbp + 0xba8]
1403dd848: 8b 9d 98 0b 00 00           	mov	ebx, dword ptr [rbp + 0xb98]
1403dd84e: eb 06                       	jmp	0x1403dd856 <.text+0x3dc856>
1403dd850: 8b bd a8 0b 00 00           	mov	edi, dword ptr [rbp + 0xba8]
1403dd856: 48 8b 85 90 0b 00 00        	mov	rax, qword ptr [rbp + 0xb90]
1403dd85d: 40 b6 01                    	mov	sil, 0x1
1403dd860: 48 85 c0                    	test	rax, rax
1403dd863: 74 2d                       	je	0x1403dd892 <.text+0x3dc892>
1403dd865: f0                          	lock
1403dd866: 48 ff 08                    	dec	qword ptr [rax]
1403dd869: 75 27                       	jne	0x1403dd892 <.text+0x3dc892>
1403dd86b: 40 88 bd d0 0b 00 00        	mov	byte ptr [rbp + 0xbd0], dil
1403dd872: 88 9d cf 0b 00 00           	mov	byte ptr [rbp + 0xbcf], bl
1403dd878: c6 85 ce 0b 00 00 01        	mov	byte ptr [rbp + 0xbce], 0x1
1403dd87f: c6 85 cd 0b 00 00 01        	mov	byte ptr [rbp + 0xbcd], 0x1
1403dd886: 48 8d 8d 90 0b 00 00        	lea	rcx, [rbp + 0xb90]
1403dd88d: e8 be 7a 7a 00              	call	0x140b85350 <.text+0xb84350>
1403dd892: 48 8b 95 88 0b 00 00        	mov	rdx, qword ptr [rbp + 0xb88]
1403dd899: 48 85 d2                    	test	rdx, rdx
1403dd89c: 0f 95 c0                    	setne	al
1403dd89f: 40 84 c7                    	test	dil, al
1403dd8a2: 74 12                       	je	0x1403dd8b6 <.text+0x3dc8b6>
1403dd8a4: 41 b8 01 00 00 00           	mov	r8d, 0x1
1403dd8aa: 48 8b 8d 58 0b 00 00        	mov	rcx, qword ptr [rbp + 0xb58]
1403dd8b1: e8 6a 63 23 00              	call	0x140613c20 <.text+0x612c20>
1403dd8b6: 48 8b 95 f0 04 00 00        	mov	rdx, qword ptr [rbp + 0x4f0]
1403dd8bd: 48 89 d0                    	mov	rax, rdx
1403dd8c0: 48 f7 d8                    	neg	rax
1403dd8c3: 0f 91 c0                    	setno	al
1403dd8c6: 40 84 c6                    	test	sil, al
1403dd8c9: 74 17                       	je	0x1403dd8e2 <.text+0x3dc8e2>
1403dd8cb: 48 85 d2                    	test	rdx, rdx
1403dd8ce: 74 12                       	je	0x1403dd8e2 <.text+0x3dc8e2>
1403dd8d0: 48 8b 8d f8 04 00 00        	mov	rcx, qword ptr [rbp + 0x4f8]
1403dd8d7: 41 b8 01 00 00 00           	mov	r8d, 0x1
1403dd8dd: e8 3e 63 23 00              	call	0x140613c20 <.text+0x612c20>
1403dd8e2: 40 84 f6                    	test	sil, sil
1403dd8e5: 0f 84 f5 00 00 00           	je	0x1403dd9e0 <.text+0x3dc9e0>
1403dd8eb: 48 8b 95 08 05 00 00        	mov	rdx, qword ptr [rbp + 0x508]
1403dd8f2: 48 89 d0                    	mov	rax, rdx
1403dd8f5: 48 f7 d8                    	neg	rax
1403dd8f8: 70 14                       	jo	0x1403dd90e <.text+0x3dc90e>
1403dd8fa: 73 12                       	jae	0x1403dd90e <.text+0x3dc90e>
1403dd8fc: 48 8b 8d 10 05 00 00        	mov	rcx, qword ptr [rbp + 0x510]
1403dd903: 41 b8 01 00 00 00           	mov	r8d, 0x1
1403dd909: e8 12 63 23 00              	call	0x140613c20 <.text+0x612c20>
1403dd90e: 48 8b 95 28 05 00 00        	mov	rdx, qword ptr [rbp + 0x528]
1403dd915: 48 89 d0                    	mov	rax, rdx
1403dd918: 48 f7 d8                    	neg	rax
1403dd91b: 70 14                       	jo	0x1403dd931 <.text+0x3dc931>
1403dd91d: 73 12                       	jae	0x1403dd931 <.text+0x3dc931>
1403dd91f: 48 8b 8d 30 05 00 00        	mov	rcx, qword ptr [rbp + 0x530]
1403dd926: 41 b8 01 00 00 00           	mov	r8d, 0x1
1403dd92c: e8 ef 62 23 00              	call	0x140613c20 <.text+0x612c20>
1403dd931: 48 8b 95 48 05 00 00        	mov	rdx, qword ptr [rbp + 0x548]
1403dd938: 48 89 d0                    	mov	rax, rdx
1403dd93b: 48 f7 d8                    	neg	rax
1403dd93e: 70 14                       	jo	0x1403dd954 <.text+0x3dc954>
1403dd940: 73 12                       	jae	0x1403dd954 <.text+0x3dc954>
1403dd942: 48 8b 8d 50 05 00 00        	mov	rcx, qword ptr [rbp + 0x550]
1403dd949: 41 b8 01 00 00 00           	mov	r8d, 0x1
1403dd94f: e8 cc 62 23 00              	call	0x140613c20 <.text+0x612c20>
1403dd954: 48 8b 95 68 05 00 00        	mov	rdx, qword ptr [rbp + 0x568]
1403dd95b: 48 89 d0                    	mov	rax, rdx
1403dd95e: 48 f7 d8                    	neg	rax
1403dd961: 70 14                       	jo	0x1403dd977 <.text+0x3dc977>
1403dd963: 73 12                       	jae	0x1403dd977 <.text+0x3dc977>
1403dd965: 48 8b 8d 70 05 00 00        	mov	rcx, qword ptr [rbp + 0x570]
1403dd96c: 41 b8 01 00 00 00           	mov	r8d, 0x1
1403dd972: e8 a9 62 23 00              	call	0x140613c20 <.text+0x612c20>
1403dd977: 48 8b 95 88 05 00 00        	mov	rdx, qword ptr [rbp + 0x588]
1403dd97e: 48 89 d0                    	mov	rax, rdx
1403dd981: 48 f7 d8                    	neg	rax
1403dd984: 70 14                       	jo	0x1403dd99a <.text+0x3dc99a>
1403dd986: 73 12                       	jae	0x1403dd99a <.text+0x3dc99a>
1403dd988: 48 8b 8d 90 05 00 00        	mov	rcx, qword ptr [rbp + 0x590]
1403dd98f: 41 b8 01 00 00 00           	mov	r8d, 0x1
1403dd995: e8 86 62 23 00              	call	0x140613c20 <.text+0x612c20>
1403dd99a: 48 8b 95 a8 05 00 00        	mov	rdx, qword ptr [rbp + 0x5a8]
1403dd9a1: 48 89 d0                    	mov	rax, rdx
1403dd9a4: 48 f7 d8                    	neg	rax
1403dd9a7: 70 14                       	jo	0x1403dd9bd <.text+0x3dc9bd>
1403dd9a9: 73 12                       	jae	0x1403dd9bd <.text+0x3dc9bd>
1403dd9ab: 48 8b 8d b0 05 00 00        	mov	rcx, qword ptr [rbp + 0x5b0]
1403dd9b2: 41 b8 01 00 00 00           	mov	r8d, 0x1
1403dd9b8: e8 63 62 23 00              	call	0x140613c20 <.text+0x612c20>
1403dd9bd: 48 8b 95 c8 05 00 00        	mov	rdx, qword ptr [rbp + 0x5c8]
1403dd9c4: 48 89 d0                    	mov	rax, rdx
1403dd9c7: 48 f7 d8                    	neg	rax
1403dd9ca: 70 14                       	jo	0x1403dd9e0 <.text+0x3dc9e0>
1403dd9cc: 73 12                       	jae	0x1403dd9e0 <.text+0x3dc9e0>
1403dd9ce: 48 8b 8d d0 05 00 00        	mov	rcx, qword ptr [rbp + 0x5d0]
1403dd9d5: 41 b8 01 00 00 00           	mov	r8d, 0x1
1403dd9db: e8 40 62 23 00              	call	0x140613c20 <.text+0x612c20>
1403dd9e0: 84 db                       	test	bl, bl
1403dd9e2: 0f 84 f4 f3 ff ff           	je	0x1403dcddc <.text+0x3dbddc>
1403dd9e8: 48 8b 95 e8 05 00 00        	mov	rdx, qword ptr [rbp + 0x5e8]
1403dd9ef: 48 89 d0                    	mov	rax, rdx
1403dd9f2: 48 f7 d8                    	neg	rax
1403dd9f5: 0f 80 e1 f3 ff ff           	jo	0x1403dcddc <.text+0x3dbddc>
1403dd9fb: 0f 83 db f3 ff ff           	jae	0x1403dcddc <.text+0x3dbddc>
1403dda01: 48 8b 8d f0 05 00 00        	mov	rcx, qword ptr [rbp + 0x5f0]
1403dda08: 41 b8 01 00 00 00           	mov	r8d, 0x1
1403dda0e: e8 0d 62 23 00              	call	0x140613c20 <.text+0x612c20>
1403dda13: e9 c4 f3 ff ff              	jmp	0x1403dcddc <.text+0x3dbddc>
1403dda18: c6 85 d7 0b 00 00 00        	mov	byte ptr [rbp + 0xbd7], 0x0
1403dda1f: c6 85 d6 0b 00 00 01        	mov	byte ptr [rbp + 0xbd6], 0x1
1403dda26: b9 08 00 00 00              	mov	ecx, 0x8
1403dda2b: ba 10 00 00 00              	mov	edx, 0x10
1403dda30: e8 cb b5 4f 02              	call	0x1428d9000 <.text+0x28d8000>
1403dda35: e9 9a 00 00 00              	jmp	0x1403ddad4 <.text+0x3dcad4>
1403dda3a: 48 89 85 30 06 00 00        	mov	qword ptr [rbp + 0x630], rax
1403dda41: 48 8d 45 b0                 	lea	rax, [rbp - 0x50]
1403dda45: 48 89 85 20 09 00 00        	mov	qword ptr [rbp + 0x920], rax
1403dda4c: 48 8d 05 7d 0e c5 ff        	lea	rax, [rip - 0x3af183]   # 0x14002e8d0 <.text+0x2d8d0>
1403dda53: 48 89 85 28 09 00 00        	mov	qword ptr [rbp + 0x928], rax
1403dda5a: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
1403dda61: 48 89 8d 30 09 00 00        	mov	qword ptr [rbp + 0x930], rcx
1403dda68: 48 89 85 38 09 00 00        	mov	qword ptr [rbp + 0x938], rax
1403dda6f: 48 8d 85 30 06 00 00        	lea	rax, [rbp + 0x630]
1403dda76: 48 89 85 40 09 00 00        	mov	qword ptr [rbp + 0x940], rax
1403dda7d: 48 8d 05 8c d5 97 00        	lea	rax, [rip + 0x97d58c]   # 0x140d5b010 <.text+0xd5a010>
1403dda84: 48 89 85 48 09 00 00        	mov	qword ptr [rbp + 0x948], rax
1403dda8b: 48 8d 05 ee ac 54 02        	lea	rax, [rip + 0x254acee]  # 0x142928780
1403dda92: 48 89 85 f0 06 00 00        	mov	qword ptr [rbp + 0x6f0], rax
1403dda99: 48 c7 85 f8 06 00 00 03 00 00 00    	mov	qword ptr [rbp + 0x6f8], 0x3
1403ddaa4: 48 c7 85 10 07 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x710], 0x0
1403ddaaf: 48 89 b5 00 07 00 00        	mov	qword ptr [rbp + 0x700], rsi
1403ddab6: 48 c7 85 08 07 00 00 03 00 00 00    	mov	qword ptr [rbp + 0x708], 0x3
1403ddac1: 48 8d 15 30 ad 54 02        	lea	rdx, [rip + 0x254ad30]  # 0x1429287f8
1403ddac8: 48 8d 8d f0 06 00 00        	lea	rcx, [rbp + 0x6f0]
1403ddacf: e8 bc b8 4f 02              	call	0x1428d9390 <.text+0x28d8390>
