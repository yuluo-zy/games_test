
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a5a020: 55                          	push	rbp
140a5a021: 41 57                       	push	r15
140a5a023: 41 56                       	push	r14
140a5a025: 41 54                       	push	r12
140a5a027: 56                          	push	rsi
140a5a028: 57                          	push	rdi
140a5a029: 53                          	push	rbx
140a5a02a: 48 81 ec 10 04 00 00        	sub	rsp, 0x410
140a5a031: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
140a5a039: 48 c7 85 88 03 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x388], -0x2
140a5a044: 4c 89 cb                    	mov	rbx, r9
140a5a047: 4d 89 c6                    	mov	r14, r8
140a5a04a: 49 89 d7                    	mov	r15, rdx
140a5a04d: 48 89 ce                    	mov	rsi, rcx
140a5a050: 49 8d 49 04                 	lea	rcx, [r9 + 0x4]
140a5a054: e8 37 b7 17 00              	call	0x140bd5790 <.text+0xbd4790>
140a5a059: 48 89 c7                    	mov	rdi, rax
140a5a05c: 48 8d 8d 30 03 00 00        	lea	rcx, [rbp + 0x330]
140a5a063: 48 89 c2                    	mov	rdx, rax
140a5a066: 41 b8 4c 00 00 00           	mov	r8d, 0x4c
140a5a06c: e8 2f b2 17 00              	call	0x140bd52a0 <.text+0xbd42a0>
140a5a071: 0f 10 85 30 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x330]
140a5a078: 0f 10 8d 40 03 00 00        	movups	xmm1, xmmword ptr [rbp + 0x340]
140a5a07f: 0f 29 4d d0                 	movaps	xmmword ptr [rbp - 0x30], xmm1
140a5a083: 0f 29 45 c0                 	movaps	xmmword ptr [rbp - 0x40], xmm0
140a5a087: 48 8d 8d 60 01 00 00        	lea	rcx, [rbp + 0x160]
140a5a08e: 48 8d 55 c0                 	lea	rdx, [rbp - 0x40]
140a5a092: e8 49 cc 17 00              	call	0x140bd6ce0 <.text+0xbd5ce0>
140a5a097: 48 8d 05 ca 90 07 02        	lea	rax, [rip + 0x20790ca]  # 0x142ad3168
140a5a09e: 48 89 85 50 01 00 00        	mov	qword ptr [rbp + 0x150], rax
140a5a0a5: 48 c7 85 58 01 00 00 03 00 00 00    	mov	qword ptr [rbp + 0x158], 0x3
140a5a0b0: 48 8d 95 50 01 00 00        	lea	rdx, [rbp + 0x150]
140a5a0b7: 4c 89 f9                    	mov	rcx, r15
140a5a0ba: e8 11 f4 18 00              	call	0x140be94d0 <.text+0xbe84d0>
140a5a0bf: 0f 10 85 30 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x330]
140a5a0c6: 0f 10 8d 40 03 00 00        	movups	xmm1, xmmword ptr [rbp + 0x340]
140a5a0cd: 0f 11 8d 20 03 00 00        	movups	xmmword ptr [rbp + 0x320], xmm1
140a5a0d4: 0f 11 85 10 03 00 00        	movups	xmmword ptr [rbp + 0x310], xmm0
140a5a0db: 89 85 0c 03 00 00           	mov	dword ptr [rbp + 0x30c], eax
140a5a0e1: 0f 29 8d 60 01 00 00        	movaps	xmmword ptr [rbp + 0x160], xmm1
140a5a0e8: 0f 29 85 50 01 00 00        	movaps	xmmword ptr [rbp + 0x150], xmm0
140a5a0ef: 48 b8 08 00 00 00 08 00 00 00       	movabs	rax, 0x800000008
140a5a0f9: 48 89 85 30 03 00 00        	mov	qword ptr [rbp + 0x330], rax
140a5a100: c7 85 38 03 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x338], 0x1
140a5a10a: 4c 8d 65 c0                 	lea	r12, [rbp - 0x40]
140a5a10e: 48 8d 95 50 01 00 00        	lea	rdx, [rbp + 0x150]
140a5a115: 4c 8d 85 30 03 00 00        	lea	r8, [rbp + 0x330]
140a5a11c: 4c 89 e1                    	mov	rcx, r12
140a5a11f: e8 bc b4 17 00              	call	0x140bd55e0 <.text+0xbd45e0>
140a5a124: 48 8d 8d 30 03 00 00        	lea	rcx, [rbp + 0x330]
140a5a12b: 4c 89 e2                    	mov	rdx, r12
140a5a12e: 41 b8 53 00 00 00           	mov	r8d, 0x53
140a5a134: e8 f7 b2 17 00              	call	0x140bd5430 <.text+0xbd4430>
140a5a139: 0f 10 85 30 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x330]
140a5a140: 0f 10 8d 40 03 00 00        	movups	xmm1, xmmword ptr [rbp + 0x340]
140a5a147: 0f 29 4d d0                 	movaps	xmmword ptr [rbp - 0x30], xmm1
140a5a14b: 0f 29 45 c0                 	movaps	xmmword ptr [rbp - 0x40], xmm0
140a5a14f: 48 8d 8d 60 01 00 00        	lea	rcx, [rbp + 0x160]
140a5a156: 4c 89 e2                    	mov	rdx, r12
140a5a159: e8 82 cb 17 00              	call	0x140bd6ce0 <.text+0xbd5ce0>
140a5a15e: 48 8d 05 06 90 07 02        	lea	rax, [rip + 0x2079006]  # 0x142ad316b
140a5a165: 48 89 85 50 01 00 00        	mov	qword ptr [rbp + 0x150], rax
140a5a16c: 48 c7 85 58 01 00 00 09 00 00 00    	mov	qword ptr [rbp + 0x158], 0x9
140a5a177: 48 8d 95 50 01 00 00        	lea	rdx, [rbp + 0x150]
140a5a17e: 4c 89 f9                    	mov	rcx, r15
140a5a181: e8 4a f3 18 00              	call	0x140be94d0 <.text+0xbe84d0>
140a5a186: 0f 10 85 30 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x330]
140a5a18d: 0f 10 8d 40 03 00 00        	movups	xmm1, xmmword ptr [rbp + 0x340]
140a5a194: 0f 11 85 ec 02 00 00        	movups	xmmword ptr [rbp + 0x2ec], xmm0
140a5a19b: 0f 11 8d fc 02 00 00        	movups	xmmword ptr [rbp + 0x2fc], xmm1
140a5a1a2: 89 85 e8 02 00 00           	mov	dword ptr [rbp + 0x2e8], eax
140a5a1a8: 4c 89 7c 24 30              	mov	qword ptr [rsp + 0x30], r15
140a5a1ad: 4c 89 74 24 28              	mov	qword ptr [rsp + 0x28], r14
140a5a1b2: 48 c7 44 24 20 14 00 00 00  	mov	qword ptr [rsp + 0x20], 0x14
140a5a1bb: 48 8d 15 b2 8f 07 02        	lea	rdx, [rip + 0x2078fb2]  # 0x142ad3174
140a5a1c2: 4c 8d 0d b6 8f 07 02        	lea	r9, [rip + 0x2078fb6]   # 0x142ad317f
140a5a1c9: 4c 8d b5 50 01 00 00        	lea	r14, [rbp + 0x150]
140a5a1d0: 41 b8 0b 00 00 00           	mov	r8d, 0xb
140a5a1d6: 4c 89 f1                    	mov	rcx, r14
140a5a1d9: e8 c2 90 ba 00              	call	0x1416032a0 <.text+0x16022a0>
140a5a1de: 48 89 9d 30 03 00 00        	mov	qword ptr [rbp + 0x330], rbx
140a5a1e5: c7 45 c0 02 00 00 00        	mov	dword ptr [rbp - 0x40], 0x2
140a5a1ec: 4c 89 64 24 20              	mov	qword ptr [rsp + 0x20], r12
140a5a1f1: 48 8d 8d 30 03 00 00        	lea	rcx, [rbp + 0x330]
140a5a1f8: 49 b8 fc fb 49 a0 9d 57 61 48       	movabs	r8, 0x4861579da049fbfc
140a5a202: 4c 89 f2                    	mov	rdx, r14
140a5a205: 41 b1 19                    	mov	r9b, 0x19
140a5a208: e8 33 40 e9 ff              	call	0x1408ee240 <.text+0x8ed240>
140a5a20d: 48 8d 5d c0                 	lea	rbx, [rbp - 0x40]
140a5a211: 48 8d 95 50 01 00 00        	lea	rdx, [rbp + 0x150]
140a5a218: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5a21e: 48 89 d9                    	mov	rcx, rbx
140a5a221: e8 da 99 ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5a226: 49 b8 b0 08 4b de c6 54 65 c2       	movabs	r8, -0x3d9aab3921b4f750
140a5a230: 4c 89 f1                    	mov	rcx, r14
140a5a233: 48 89 da                    	mov	rdx, rbx
140a5a236: e8 85 3b bb 00              	call	0x14160ddc0 <.text+0x160cdc0>
140a5a23b: 48 8d 85 0c 03 00 00        	lea	rax, [rbp + 0x30c]
140a5a242: 48 89 85 30 03 00 00        	mov	qword ptr [rbp + 0x330], rax
140a5a249: c7 45 c0 02 00 00 00        	mov	dword ptr [rbp - 0x40], 0x2
140a5a250: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
140a5a255: 48 8d 8d 30 03 00 00        	lea	rcx, [rbp + 0x330]
140a5a25c: 49 b8 45 43 6f 04 48 15 d6 4e       	movabs	r8, 0x4ed61548046f4345
140a5a266: 4c 89 f2                    	mov	rdx, r14
140a5a269: 41 b1 32                    	mov	r9b, 0x32
140a5a26c: e8 8f 3a e9 ff              	call	0x1408edd00 <.text+0x8ecd00>
140a5a271: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5a277: 48 89 d9                    	mov	rcx, rbx
140a5a27a: 4c 89 f2                    	mov	rdx, r14
140a5a27d: e8 7e 99 ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5a282: 48 8d 85 e8 02 00 00        	lea	rax, [rbp + 0x2e8]
140a5a289: 48 89 85 30 03 00 00        	mov	qword ptr [rbp + 0x330], rax
140a5a290: c7 85 50 01 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x150], 0x2
140a5a29a: 4c 89 74 24 20              	mov	qword ptr [rsp + 0x20], r14
140a5a29f: 48 8d 8d 30 03 00 00        	lea	rcx, [rbp + 0x330]
140a5a2a6: 49 b8 65 51 c4 27 6d 7d 37 01       	movabs	r8, 0x1377d6d27c45165
140a5a2b0: 48 89 da                    	mov	rdx, rbx
140a5a2b3: 41 b1 32                    	mov	r9b, 0x32
140a5a2b6: e8 45 3a e9 ff              	call	0x1408edd00 <.text+0x8ecd00>
140a5a2bb: 48 89 fb                    	mov	rbx, rdi
140a5a2be: 48 c1 eb 20                 	shr	rbx, 0x20
140a5a2c2: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
140a5a2c9: 48 8d 55 c0                 	lea	rdx, [rbp - 0x40]
140a5a2cd: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5a2d3: e8 28 99 ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5a2d8: 0f 10 85 f0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1f0]
140a5a2df: 0f 29 85 70 03 00 00        	movaps	xmmword ptr [rbp + 0x370], xmm0
140a5a2e6: 0f 10 85 e0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1e0]
140a5a2ed: 0f 29 85 60 03 00 00        	movaps	xmmword ptr [rbp + 0x360], xmm0
140a5a2f4: 0f 10 85 d0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1d0]
140a5a2fb: 0f 29 85 50 03 00 00        	movaps	xmmword ptr [rbp + 0x350], xmm0
140a5a302: 0f 10 85 c0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1c0]
140a5a309: 0f 29 85 40 03 00 00        	movaps	xmmword ptr [rbp + 0x340], xmm0
140a5a310: 0f 10 85 b0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1b0]
140a5a317: 0f 29 85 30 03 00 00        	movaps	xmmword ptr [rbp + 0x330], xmm0
140a5a31e: 48 8b 85 88 02 00 00        	mov	rax, qword ptr [rbp + 0x288]
140a5a325: 48 8b 8d d0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2d0]
140a5a32c: 48 8b 95 60 01 00 00        	mov	rdx, qword ptr [rbp + 0x160]
140a5a333: 48 89 55 d0                 	mov	qword ptr [rbp - 0x30], rdx
140a5a337: 0f 10 85 50 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x150]
140a5a33e: 0f 29 45 c0                 	movaps	xmmword ptr [rbp - 0x40], xmm0
140a5a342: 0f 10 85 68 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x168]
140a5a349: 0f 29 45 f0                 	movaps	xmmword ptr [rbp - 0x10], xmm0
140a5a34d: 48 8b 95 78 01 00 00        	mov	rdx, qword ptr [rbp + 0x178]
140a5a354: 48 89 55 00                 	mov	qword ptr [rbp], rdx
140a5a358: 0f 10 85 80 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x180]
140a5a35f: 0f 11 45 d8                 	movups	xmmword ptr [rbp - 0x28], xmm0
140a5a363: 48 8b 95 90 01 00 00        	mov	rdx, qword ptr [rbp + 0x190]
140a5a36a: 48 89 55 e8                 	mov	qword ptr [rbp - 0x18], rdx
140a5a36e: 0f 10 85 00 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x200]
140a5a375: 0f 10 8d 10 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x210]
140a5a37c: 0f 10 95 20 02 00 00        	movups	xmm2, xmmword ptr [rbp + 0x220]
140a5a383: 0f 10 9d 30 02 00 00        	movups	xmm3, xmmword ptr [rbp + 0x230]
140a5a38a: 0f 11 45 38                 	movups	xmmword ptr [rbp + 0x38], xmm0
140a5a38e: 0f 11 4d 48                 	movups	xmmword ptr [rbp + 0x48], xmm1
140a5a392: 0f 11 55 58                 	movups	xmmword ptr [rbp + 0x58], xmm2
140a5a396: 0f 11 5d 68                 	movups	xmmword ptr [rbp + 0x68], xmm3
140a5a39a: 0f 10 85 40 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x240]
140a5a3a1: 0f 11 45 78                 	movups	xmmword ptr [rbp + 0x78], xmm0
140a5a3a5: 0f 10 85 50 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x250]
140a5a3ac: 0f 11 85 88 00 00 00        	movups	xmmword ptr [rbp + 0x88], xmm0
140a5a3b3: 48 8b 95 a8 01 00 00        	mov	rdx, qword ptr [rbp + 0x1a8]
140a5a3ba: 48 89 55 18                 	mov	qword ptr [rbp + 0x18], rdx
140a5a3be: 0f 10 85 98 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x198]
140a5a3c5: 0f 11 45 08                 	movups	xmmword ptr [rbp + 0x8], xmm0
140a5a3c9: 48 89 8d a0 00 00 00        	mov	qword ptr [rbp + 0xa0], rcx
140a5a3d0: 48 89 85 98 00 00 00        	mov	qword ptr [rbp + 0x98], rax
140a5a3d7: c7 45 20 01 00 00 00        	mov	dword ptr [rbp + 0x20], 0x1
140a5a3de: 89 7d 24                    	mov	dword ptr [rbp + 0x24], edi
140a5a3e1: 89 5d 28                    	mov	dword ptr [rbp + 0x28], ebx
140a5a3e4: c7 45 2c 01 00 00 00        	mov	dword ptr [rbp + 0x2c], 0x1
140a5a3eb: 48 8d 8d 30 03 00 00        	lea	rcx, [rbp + 0x330]
140a5a3f2: 48 8d 55 c0                 	lea	rdx, [rbp - 0x40]
140a5a3f6: e8 d5 30 f8 ff              	call	0x1409dd4d0 <.text+0x9dc4d0>
140a5a3fb: 80 bd 86 02 00 00 08        	cmp	byte ptr [rbp + 0x286], 0x8
140a5a402: 74 0c                       	je	0x140a5a410 <.text+0xa59410>
140a5a404: 48 8d 8d 70 02 00 00        	lea	rcx, [rbp + 0x270]
140a5a40b: e8 a0 80 18 00              	call	0x140be24b0 <.text+0xbe14b0>
140a5a410: 8b 85 2c 03 00 00           	mov	eax, dword ptr [rbp + 0x32c]
140a5a416: 89 46 20                    	mov	dword ptr [rsi + 0x20], eax
140a5a419: 0f 10 85 0c 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x30c]
140a5a420: 0f 10 8d 1c 03 00 00        	movups	xmm1, xmmword ptr [rbp + 0x31c]
140a5a427: 0f 11 4e 10                 	movups	xmmword ptr [rsi + 0x10], xmm1
140a5a42b: 0f 11 06                    	movups	xmmword ptr [rsi], xmm0
140a5a42e: 0f 10 85 e8 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2e8]
140a5a435: 0f 10 8d f8 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x2f8]
140a5a43c: 0f 11 46 24                 	movups	xmmword ptr [rsi + 0x24], xmm0
140a5a440: 0f 11 4e 34                 	movups	xmmword ptr [rsi + 0x34], xmm1
140a5a444: 8b 85 08 03 00 00           	mov	eax, dword ptr [rbp + 0x308]
140a5a44a: 89 46 44                    	mov	dword ptr [rsi + 0x44], eax
140a5a44d: 48 89 f0                    	mov	rax, rsi
140a5a450: 48 81 c4 10 04 00 00        	add	rsp, 0x410
140a5a457: 5b                          	pop	rbx
140a5a458: 5f                          	pop	rdi
140a5a459: 5e                          	pop	rsi
140a5a45a: 41 5c                       	pop	r12
140a5a45c: 41 5e                       	pop	r14
140a5a45e: 41 5f                       	pop	r15
140a5a460: 5d                          	pop	rbp
140a5a461: c3                          	ret
140a5a462: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
140a5a470: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5a475: 55                          	push	rbp
140a5a476: 41 57                       	push	r15
140a5a478: 41 56                       	push	r14
140a5a47a: 41 54                       	push	r12
140a5a47c: 56                          	push	rsi
140a5a47d: 57                          	push	rdi
140a5a47e: 53                          	push	rbx
140a5a47f: 48 83 ec 40                 	sub	rsp, 0x40
140a5a483: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5a48a: 80 bd 86 02 00 00 08        	cmp	byte ptr [rbp + 0x286], 0x8
140a5a491: 74 0c                       	je	0x140a5a49f <.text+0xa5949f>
140a5a493: 48 8d 8d 70 02 00 00        	lea	rcx, [rbp + 0x270]
140a5a49a: e8 11 80 18 00              	call	0x140be24b0 <.text+0xbe14b0>
140a5a49f: 90                          	nop
140a5a4a0: 48 83 c4 40                 	add	rsp, 0x40
140a5a4a4: 5b                          	pop	rbx
140a5a4a5: 5f                          	pop	rdi
140a5a4a6: 5e                          	pop	rsi
140a5a4a7: 41 5c                       	pop	r12
140a5a4a9: 41 5e                       	pop	r14
140a5a4ab: 41 5f                       	pop	r15
140a5a4ad: 5d                          	pop	rbp
140a5a4ae: c3                          	ret
140a5a4af: 90                          	nop
140a5a4b0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5a4b5: 55                          	push	rbp
140a5a4b6: 41 57                       	push	r15
140a5a4b8: 41 56                       	push	r14
140a5a4ba: 41 54                       	push	r12
140a5a4bc: 56                          	push	rsi
140a5a4bd: 57                          	push	rdi
140a5a4be: 53                          	push	rbx
140a5a4bf: 48 83 ec 40                 	sub	rsp, 0x40
140a5a4c3: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5a4ca: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
140a5a4d1: e8 da ca e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5a4d6: 90                          	nop
140a5a4d7: 48 83 c4 40                 	add	rsp, 0x40
140a5a4db: 5b                          	pop	rbx
140a5a4dc: 5f                          	pop	rdi
140a5a4dd: 5e                          	pop	rsi
140a5a4de: 41 5c                       	pop	r12
140a5a4e0: 41 5e                       	pop	r14
140a5a4e2: 41 5f                       	pop	r15
140a5a4e4: 5d                          	pop	rbp
140a5a4e5: c3                          	ret
140a5a4e6: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
140a5a4f0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5a4f5: 55                          	push	rbp
140a5a4f6: 41 57                       	push	r15
140a5a4f8: 41 56                       	push	r14
140a5a4fa: 41 54                       	push	r12
140a5a4fc: 56                          	push	rsi
140a5a4fd: 57                          	push	rdi
140a5a4fe: 53                          	push	rbx
140a5a4ff: 48 83 ec 40                 	sub	rsp, 0x40
140a5a503: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5a50a: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
140a5a511: e8 9a ca e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5a516: 90                          	nop
140a5a517: 48 83 c4 40                 	add	rsp, 0x40
140a5a51b: 5b                          	pop	rbx
140a5a51c: 5f                          	pop	rdi
140a5a51d: 5e                          	pop	rsi
140a5a51e: 41 5c                       	pop	r12
140a5a520: 41 5e                       	pop	r14
140a5a522: 41 5f                       	pop	r15
140a5a524: 5d                          	pop	rbp
140a5a525: c3                          	ret
140a5a526: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
140a5a530: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5a535: 55                          	push	rbp
140a5a536: 41 57                       	push	r15
140a5a538: 41 56                       	push	r14
140a5a53a: 41 54                       	push	r12
140a5a53c: 56                          	push	rsi
140a5a53d: 57                          	push	rdi
140a5a53e: 53                          	push	rbx
140a5a53f: 48 83 ec 40                 	sub	rsp, 0x40
140a5a543: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5a54a: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
140a5a54e: e8 5d ca e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5a553: 90                          	nop
140a5a554: 48 83 c4 40                 	add	rsp, 0x40
140a5a558: 5b                          	pop	rbx
140a5a559: 5f                          	pop	rdi
140a5a55a: 5e                          	pop	rsi
140a5a55b: 41 5c                       	pop	r12
140a5a55d: 41 5e                       	pop	r14
140a5a55f: 41 5f                       	pop	r15
140a5a561: 5d                          	pop	rbp
140a5a562: c3                          	ret
