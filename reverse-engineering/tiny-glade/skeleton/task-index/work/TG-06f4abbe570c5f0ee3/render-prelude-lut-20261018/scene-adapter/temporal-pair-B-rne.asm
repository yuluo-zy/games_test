
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1404f7450: 41 57                       	push	r15
1404f7452: 41 56                       	push	r14
1404f7454: 41 55                       	push	r13
1404f7456: 41 54                       	push	r12
1404f7458: 56                          	push	rsi
1404f7459: 57                          	push	rdi
1404f745a: 55                          	push	rbp
1404f745b: 53                          	push	rbx
1404f745c: 48 81 ec f8 00 00 00        	sub	rsp, 0xf8
1404f7463: 48 89 d7                    	mov	rdi, rdx
1404f7466: 48 89 ce                    	mov	rsi, rcx
1404f7469: 48 8b 9c 24 60 01 00 00     	mov	rbx, qword ptr [rsp + 0x160]
1404f7471: 4c 89 c1                    	mov	rcx, r8
1404f7474: 4c 89 ca                    	mov	rdx, r9
1404f7477: e8 c4 81 71 00              	call	0x140c0f640 <.text+0xc0e640>
1404f747c: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
1404f7481: 4c 8d 64 24 28              	lea	r12, [rsp + 0x28]
1404f7486: 4c 89 64 24 68              	mov	qword ptr [rsp + 0x68], r12
1404f748b: 4c 8d 2d 5e 82 71 00        	lea	r13, [rip + 0x71825e]   # 0x140c0f6f0 <.text+0xc0e6f0>
1404f7492: 4c 89 6c 24 70              	mov	qword ptr [rsp + 0x70], r13
1404f7497: 48 8d 05 8a 5a 4f 02        	lea	rax, [rip + 0x24f5a8a]  # 0x1429ecf28
1404f749e: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
1404f74a3: 48 c7 44 24 38 02 00 00 00  	mov	qword ptr [rsp + 0x38], 0x2
1404f74ac: 4c 8d 7c 24 68              	lea	r15, [rsp + 0x68]
1404f74b1: 4c 89 7c 24 40              	mov	qword ptr [rsp + 0x40], r15
1404f74b6: 48 c7 44 24 48 01 00 00 00  	mov	qword ptr [rsp + 0x48], 0x1
1404f74bf: 48 c7 44 24 50 00 00 00 00  	mov	qword ptr [rsp + 0x50], 0x0
1404f74c8: 4c 8d b4 24 e0 00 00 00     	lea	r14, [rsp + 0xe0]
1404f74d0: 48 8d 54 24 30              	lea	rdx, [rsp + 0x30]
1404f74d5: 4c 89 f1                    	mov	rcx, r14
1404f74d8: e8 b3 2a 88 00              	call	0x140d79f90 <.text+0xd78f90>
1404f74dd: 0f 10 03                    	movups	xmm0, xmmword ptr [rbx]
1404f74e0: 0f 10 4b 10                 	movups	xmm1, xmmword ptr [rbx + 0x10]
1404f74e4: 0f 29 4c 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm1
1404f74e9: 0f 29 44 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm0
1404f74ee: 48 8d 4c 24 78              	lea	rcx, [rsp + 0x78]
1404f74f3: 4c 8d 4c 24 30              	lea	r9, [rsp + 0x30]
1404f74f8: 48 89 fa                    	mov	rdx, rdi
1404f74fb: 4d 89 f0                    	mov	r8, r14
1404f74fe: e8 4d f3 ff ff              	call	0x1404f6850 <.text+0x4f5850>
1404f7503: 0f b6 ac 24 9c 00 00 00     	movzx	ebp, byte ptr [rsp + 0x9c]
1404f750b: 48 8b 44 24 78              	mov	rax, qword ptr [rsp + 0x78]
1404f7510: 40 80 fd 02                 	cmp	bpl, 0x2
1404f7514: 75 0e                       	jne	0x1404f7524 <.text+0x4f6524>
1404f7516: 48 89 46 08                 	mov	qword ptr [rsi + 0x8], rax
1404f751a: b9 01 00 00 00              	mov	ecx, 0x1
1404f751f: e9 75 01 00 00              	jmp	0x1404f7699 <.text+0x4f6699>
1404f7524: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1404f7529: 0f 10 84 24 80 00 00 00     	movups	xmm0, xmmword ptr [rsp + 0x80]
1404f7531: 0f 10 8c 24 8c 00 00 00     	movups	xmm1, xmmword ptr [rsp + 0x8c]
1404f7539: 0f 11 8c 24 ac 00 00 00     	movups	xmmword ptr [rsp + 0xac], xmm1
1404f7541: 0f 29 84 24 a0 00 00 00     	movaps	xmmword ptr [rsp + 0xa0], xmm0
1404f7549: 4c 89 64 24 68              	mov	qword ptr [rsp + 0x68], r12
1404f754e: 4c 89 6c 24 70              	mov	qword ptr [rsp + 0x70], r13
1404f7553: 48 8d 05 f6 59 4f 02        	lea	rax, [rip + 0x24f59f6]  # 0x1429ecf50
1404f755a: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
1404f755f: 48 c7 44 24 38 02 00 00 00  	mov	qword ptr [rsp + 0x38], 0x2
1404f7568: 4c 89 7c 24 40              	mov	qword ptr [rsp + 0x40], r15
1404f756d: 48 c7 44 24 48 01 00 00 00  	mov	qword ptr [rsp + 0x48], 0x1
1404f7576: 48 c7 44 24 50 00 00 00 00  	mov	qword ptr [rsp + 0x50], 0x0
1404f757f: 4c 8d b4 24 e0 00 00 00     	lea	r14, [rsp + 0xe0]
1404f7587: 48 8d 54 24 30              	lea	rdx, [rsp + 0x30]
1404f758c: 4c 89 f1                    	mov	rcx, r14
1404f758f: e8 fc 29 88 00              	call	0x140d79f90 <.text+0xd78f90>
1404f7594: 48 8d 4c 24 78              	lea	rcx, [rsp + 0x78]
1404f7599: 48 89 fa                    	mov	rdx, rdi
1404f759c: 4d 89 f0                    	mov	r8, r14
1404f759f: 49 89 d9                    	mov	r9, rbx
1404f75a2: e8 a9 f2 ff ff              	call	0x1404f6850 <.text+0x4f5850>
1404f75a7: 44 0f b6 b4 24 9c 00 00 00  	movzx	r14d, byte ptr [rsp + 0x9c]
1404f75b0: 48 8b 5c 24 78              	mov	rbx, qword ptr [rsp + 0x78]
1404f75b5: 41 80 fe 02                 	cmp	r14b, 0x2
1404f75b9: 75 0e                       	jne	0x1404f75c9 <.text+0x4f65c9>
1404f75bb: 48 89 5e 08                 	mov	qword ptr [rsi + 0x8], rbx
1404f75bf: b9 01 00 00 00              	mov	ecx, 0x1
1404f75c4: e9 d0 00 00 00              	jmp	0x1404f7699 <.text+0x4f6699>
1404f75c9: 0f 10 84 24 80 00 00 00     	movups	xmm0, xmmword ptr [rsp + 0x80]
1404f75d1: 0f 10 8c 24 8c 00 00 00     	movups	xmm1, xmmword ptr [rsp + 0x8c]
1404f75d9: 0f 11 8c 24 cc 00 00 00     	movups	xmmword ptr [rsp + 0xcc], xmm1
1404f75e1: 0f 29 84 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm0
1404f75e9: 48 81 c7 b0 00 00 00        	add	rdi, 0xb0
1404f75f0: 48 8b 54 24 28              	mov	rdx, qword ptr [rsp + 0x28]
1404f75f5: 48 89 f9                    	mov	rcx, rdi
1404f75f8: e8 d3 58 6f 00              	call	0x140beced0 <.text+0xbebed0>
1404f75fd: 4c 8d 4e 30                 	lea	r9, [rsi + 0x30]
1404f7601: 4c 8d 46 0c                 	lea	r8, [rsi + 0xc]
1404f7605: 41 0f b6 d6                 	movzx	edx, r14b
1404f7609: 31 c9                       	xor	ecx, ecx
1404f760b: 40 f6 c5 01                 	test	bpl, 0x1
1404f760f: 0f 44 d1                    	cmove	edx, ecx
1404f7612: 85 c0                       	test	eax, eax
1404f7614: 74 41                       	je	0x1404f7657 <.text+0x4f6657>
1404f7616: 0f 10 84 24 ac 00 00 00     	movups	xmm0, xmmword ptr [rsp + 0xac]
1404f761e: 41 0f 11 41 0c              	movups	xmmword ptr [r9 + 0xc], xmm0
1404f7623: 0f 28 84 24 a0 00 00 00     	movaps	xmm0, xmmword ptr [rsp + 0xa0]
1404f762b: 41 0f 11 01                 	movups	xmmword ptr [r9], xmm0
1404f762f: 0f 10 84 24 cc 00 00 00     	movups	xmm0, xmmword ptr [rsp + 0xcc]
1404f7637: 41 0f 11 40 0c              	movups	xmmword ptr [r8 + 0xc], xmm0
1404f763c: 0f 28 84 24 c0 00 00 00     	movaps	xmm0, xmmword ptr [rsp + 0xc0]
1404f7644: 41 0f 11 00                 	movups	xmmword ptr [r8], xmm0
1404f7648: 48 89 5e 04                 	mov	qword ptr [rsi + 0x4], rbx
1404f764c: 48 8b 44 24 20              	mov	rax, qword ptr [rsp + 0x20]
1404f7651: 48 89 46 28                 	mov	qword ptr [rsi + 0x28], rax
1404f7655: eb 3f                       	jmp	0x1404f7696 <.text+0x4f6696>
1404f7657: 0f 10 84 24 cc 00 00 00     	movups	xmm0, xmmword ptr [rsp + 0xcc]
1404f765f: 41 0f 11 41 0c              	movups	xmmword ptr [r9 + 0xc], xmm0
1404f7664: 0f 28 84 24 c0 00 00 00     	movaps	xmm0, xmmword ptr [rsp + 0xc0]
1404f766c: 41 0f 11 01                 	movups	xmmword ptr [r9], xmm0
1404f7670: 0f 10 84 24 ac 00 00 00     	movups	xmm0, xmmword ptr [rsp + 0xac]
1404f7678: 41 0f 11 40 0c              	movups	xmmword ptr [r8 + 0xc], xmm0
1404f767d: 0f 28 84 24 a0 00 00 00     	movaps	xmm0, xmmword ptr [rsp + 0xa0]
1404f7685: 41 0f 11 00                 	movups	xmmword ptr [r8], xmm0
1404f7689: 48 8b 44 24 20              	mov	rax, qword ptr [rsp + 0x20]
1404f768e: 48 89 46 04                 	mov	qword ptr [rsi + 0x4], rax
1404f7692: 48 89 5e 28                 	mov	qword ptr [rsi + 0x28], rbx
1404f7696: 88 56 4c                    	mov	byte ptr [rsi + 0x4c], dl
1404f7699: 89 0e                       	mov	dword ptr [rsi], ecx
1404f769b: 48 89 f0                    	mov	rax, rsi
1404f769e: 48 81 c4 f8 00 00 00        	add	rsp, 0xf8
1404f76a5: 5b                          	pop	rbx
1404f76a6: 5d                          	pop	rbp
1404f76a7: 5f                          	pop	rdi
1404f76a8: 5e                          	pop	rsi
1404f76a9: 41 5c                       	pop	r12
1404f76ab: 41 5d                       	pop	r13
1404f76ad: 41 5e                       	pop	r14
1404f76af: 41 5f                       	pop	r15
1404f76b1: c3                          	ret
