
/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a172d0: 55                          	push	rbp
140a172d1: 41 57                       	push	r15
140a172d3: 41 56                       	push	r14
140a172d5: 41 55                       	push	r13
140a172d7: 41 54                       	push	r12
140a172d9: 56                          	push	rsi
140a172da: 57                          	push	rdi
140a172db: 53                          	push	rbx
140a172dc: 48 81 ec 78 02 00 00        	sub	rsp, 0x278
140a172e3: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
140a172eb: 48 c7 85 f0 01 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x1f0], -0x2
140a172f6: 4c 89 cf                    	mov	rdi, r9
140a172f9: 4d 89 c7                    	mov	r15, r8
140a172fc: 41 89 d6                    	mov	r14d, edx
140a172ff: 48 89 ce                    	mov	rsi, rcx
140a17302: 80 b9 99 02 00 00 00        	cmp	byte ptr [rcx + 0x299], 0x0
140a17309: 74 19                       	je	0x140a17324 <.text+0xa16324>
140a1730b: 48 85 ff                    	test	rdi, rdi
140a1730e: 74 0d                       	je	0x140a1731d <.text+0xa1631d>
140a17310: 4c 89 f9                    	mov	rcx, r15
140a17313: 31 d2                       	xor	edx, edx
140a17315: 49 89 f8                    	mov	r8, rdi
140a17318: e8 03 c9 f0 01              	call	0x142923c20 <.text+0x2922c20>
140a1731d: c6 86 99 02 00 00 00        	mov	byte ptr [rsi + 0x299], 0x0
140a17324: 44 88 b6 98 02 00 00        	mov	byte ptr [rsi + 0x298], r14b
140a1732b: e8 d0 f6 1f 00              	call	0x140c16a00 <.text+0xc15a00>
140a17330: 84 c0                       	test	al, al
140a17332: 74 62                       	je	0x140a17396 <.text+0xa16396>
140a17334: 4c 8d 25 6e 0f 0b 02        	lea	r12, [rip + 0x20b0f6e]  # 0x142ac82a9
140a1733b: bb 82 00 00 00              	mov	ebx, 0x82
140a17340: ba 82 00 00 00              	mov	edx, 0x82
140a17345: 4c 89 e1                    	mov	rcx, r12
140a17348: e8 83 d6 ff ff              	call	0x140a149d0 <.text+0xa139d0>
140a1734d: 48 83 f8 01                 	cmp	rax, 0x1
140a17351: 0f 85 b7 00 00 00           	jne	0x140a1740e <.text+0xa1640e>
140a17357: 49 89 d1                    	mov	r9, rdx
140a1735a: 48 85 d2                    	test	rdx, rdx
140a1735d: 74 40                       	je	0x140a1739f <.text+0xa1639f>
140a1735f: 49 81 f9 82 00 00 00        	cmp	r9, 0x82
140a17366: 73 35                       	jae	0x140a1739d <.text+0xa1639d>
140a17368: 48 8d 05 3a 0f 0b 02        	lea	rax, [rip + 0x20b0f3a]  # 0x142ac82a9
140a1736f: 41 80 3c 01 bf              	cmp	byte ptr [r9 + rax], -0x41
140a17374: 7f 29                       	jg	0x140a1739f <.text+0xa1639f>
140a17376: 48 8d 05 43 11 0b 02        	lea	rax, [rip + 0x20b1143]  # 0x142ac84c0
140a1737d: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
140a17382: 48 8d 0d 20 0f 0b 02        	lea	rcx, [rip + 0x20b0f20]  # 0x142ac82a9
140a17389: ba 82 00 00 00              	mov	edx, 0x82
140a1738e: 45 31 c0                    	xor	r8d, r8d
140a17391: e8 aa 25 ec 01              	call	0x1428d9940 <.text+0x28d8940>
140a17396: 31 c0                       	xor	eax, eax
140a17398: e9 e7 01 00 00              	jmp	0x140a17584 <.text+0xa16584>
140a1739d: 75 d7                       	jne	0x140a17376 <.text+0xa16376>
140a1739f: 4c 8d 25 03 0f 0b 02        	lea	r12, [rip + 0x20b0f03]  # 0x142ac82a9
140a173a6: 4c 89 e1                    	mov	rcx, r12
140a173a9: 4c 89 ca                    	mov	rdx, r9
140a173ac: e8 1f d6 ff ff              	call	0x140a149d0 <.text+0xa139d0>
140a173b1: 48 83 f8 01                 	cmp	rax, 0x1
140a173b5: 75 57                       	jne	0x140a1740e <.text+0xa1640e>
140a173b7: 49 89 d0                    	mov	r8, rdx
140a173ba: 49 83 c0 02                 	add	r8, 0x2
140a173be: 74 3c                       	je	0x140a173fc <.text+0xa163fc>
140a173c0: 49 81 f8 82 00 00 00        	cmp	r8, 0x82
140a173c7: 73 31                       	jae	0x140a173fa <.text+0xa163fa>
140a173c9: 48 8d 05 d9 0e 0b 02        	lea	rax, [rip + 0x20b0ed9]  # 0x142ac82a9
140a173d0: 41 80 3c 00 bf              	cmp	byte ptr [r8 + rax], -0x41
140a173d5: 7f 25                       	jg	0x140a173fc <.text+0xa163fc>
140a173d7: 48 8d 05 fa 10 0b 02        	lea	rax, [rip + 0x20b10fa]  # 0x142ac84d8
140a173de: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
140a173e3: 48 8d 0d bf 0e 0b 02        	lea	rcx, [rip + 0x20b0ebf]  # 0x142ac82a9
140a173ea: ba 82 00 00 00              	mov	edx, 0x82
140a173ef: 41 b9 82 00 00 00           	mov	r9d, 0x82
140a173f5: e8 46 25 ec 01              	call	0x1428d9940 <.text+0x28d8940>
140a173fa: 75 db                       	jne	0x140a173d7 <.text+0xa163d7>
140a173fc: bb 80 00 00 00              	mov	ebx, 0x80
140a17401: 48 29 d3                    	sub	rbx, rdx
140a17404: 4c 8d 25 9e 0e 0b 02        	lea	r12, [rip + 0x20b0e9e]  # 0x142ac82a9
140a1740b: 4d 01 c4                    	add	r12, r8
140a1740e: 4c 8d 05 24 11 0b 02        	lea	r8, [rip + 0x20b1124]   # 0x142ac8539
140a17415: 4c 8d 2d ec 10 0b 02        	lea	r13, [rip + 0x20b10ec]  # 0x142ac8508
140a1741c: 44 88 b5 e8 01 00 00        	mov	byte ptr [rbp + 0x1e8], r14b
140a17423: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a17430: 41 0f be 40 ff              	movsx	eax, byte ptr [r8 - 0x1]
140a17435: 85 c0                       	test	eax, eax
140a17437: 78 17                       	js	0x140a17450 <.text+0xa16450>
140a17439: 49 ff c8                    	dec	r8
140a1743c: 83 f8 5c                    	cmp	eax, 0x5c
140a1743f: 75 63                       	jne	0x140a174a4 <.text+0xa164a4>
140a17441: eb 76                       	jmp	0x140a174b9 <.text+0xa164b9>
140a17443: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a17450: 41 0f b6 48 fe              	movzx	ecx, byte ptr [r8 - 0x2]
140a17455: 80 f9 c0                    	cmp	cl, -0x40
140a17458: 7d 23                       	jge	0x140a1747d <.text+0xa1647d>
140a1745a: 41 0f b6 50 fd              	movzx	edx, byte ptr [r8 - 0x3]
140a1745f: 80 fa c0                    	cmp	dl, -0x40
140a17462: 7d 22                       	jge	0x140a17486 <.text+0xa16486>
140a17464: 45 0f b6 48 fc              	movzx	r9d, byte ptr [r8 - 0x4]
140a17469: 49 83 c0 fc                 	add	r8, -0x4
140a1746d: 41 83 e1 07                 	and	r9d, 0x7
140a17471: 41 c1 e1 06                 	shl	r9d, 0x6
140a17475: 83 e2 3f                    	and	edx, 0x3f
140a17478: 44 09 ca                    	or	edx, r9d
140a1747b: eb 10                       	jmp	0x140a1748d <.text+0xa1648d>
140a1747d: 49 83 c0 fe                 	add	r8, -0x2
140a17481: 83 e1 1f                    	and	ecx, 0x1f
140a17484: eb 0f                       	jmp	0x140a17495 <.text+0xa16495>
140a17486: 49 83 c0 fd                 	add	r8, -0x3
140a1748a: 83 e2 0f                    	and	edx, 0xf
140a1748d: c1 e2 06                    	shl	edx, 0x6
140a17490: 83 e1 3f                    	and	ecx, 0x3f
140a17493: 09 d1                       	or	ecx, edx
140a17495: c1 e1 06                    	shl	ecx, 0x6
140a17498: 24 3f                       	and	al, 0x3f
140a1749a: 0f b6 c0                    	movzx	eax, al
140a1749d: 09 c8                       	or	eax, ecx
140a1749f: 83 f8 5c                    	cmp	eax, 0x5c
140a174a2: 74 15                       	je	0x140a174b9 <.text+0xa164b9>
140a174a4: 83 f8 2f                    	cmp	eax, 0x2f
140a174a7: 74 10                       	je	0x140a174b9 <.text+0xa164b9>
140a174a9: 4d 39 e8                    	cmp	r8, r13
140a174ac: 75 82                       	jne	0x140a17430 <.text+0xa16430>
140a174ae: 4d 89 fe                    	mov	r14, r15
140a174b1: 41 bf 31 00 00 00           	mov	r15d, 0x31
140a174b7: eb 50                       	jmp	0x140a17509 <.text+0xa16509>
140a174b9: 4c 8d 2d 48 10 0b 02        	lea	r13, [rip + 0x20b1048]  # 0x142ac8508
140a174c0: 4d 29 e8                    	sub	r8, r13
140a174c3: 49 ff c0                    	inc	r8
140a174c6: 74 32                       	je	0x140a174fa <.text+0xa164fa>
140a174c8: 49 83 f8 31                 	cmp	r8, 0x31
140a174cc: 73 2a                       	jae	0x140a174f8 <.text+0xa164f8>
140a174ce: 43 80 3c 28 bf              	cmp	byte ptr [r8 + r13], -0x41
140a174d3: 7f 25                       	jg	0x140a174fa <.text+0xa164fa>
140a174d5: 48 8d 05 c4 0f 0b 02        	lea	rax, [rip + 0x20b0fc4]  # 0x142ac84a0
140a174dc: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
140a174e1: 48 8d 0d 20 10 0b 02        	lea	rcx, [rip + 0x20b1020]  # 0x142ac8508
140a174e8: ba 31 00 00 00              	mov	edx, 0x31
140a174ed: 41 b9 31 00 00 00           	mov	r9d, 0x31
140a174f3: e8 48 24 ec 01              	call	0x1428d9940 <.text+0x28d8940>
140a174f8: 75 db                       	jne	0x140a174d5 <.text+0xa164d5>
140a174fa: 4d 89 fe                    	mov	r14, r15
140a174fd: 41 bf 31 00 00 00           	mov	r15d, 0x31
140a17503: 4d 29 c7                    	sub	r15, r8
140a17506: 4d 01 c5                    	add	r13, r8
140a17509: e8 22 0c 20 00              	call	0x140c18130 <.text+0xc17130>
140a1750e: 48 89 c1                    	mov	rcx, rax
140a17511: 48 8b 00                    	mov	rax, qword ptr [rax]
140a17514: 48 83 f8 01                 	cmp	rax, 0x1
140a17518: 0f 85 dc 01 00 00           	jne	0x140a176fa <.text+0xa166fa>
140a1751e: 48 83 c1 08                 	add	rcx, 0x8
140a17522: 48 83 39 00                 	cmp	qword ptr [rcx], 0x0
140a17526: 0f 85 f2 01 00 00           	jne	0x140a1771e <.text+0xa1671e>
140a1752c: 48 c7 01 ff ff ff ff        	mov	qword ptr [rcx], -0x1
140a17533: 48 89 8d e0 01 00 00        	mov	qword ptr [rbp + 0x1e0], rcx
140a1753a: 48 83 c1 08                 	add	rcx, 0x8
140a1753e: 4c 89 7c 24 20              	mov	qword ptr [rsp + 0x20], r15
140a17543: 48 c7 44 24 30 00 00 00 00  	mov	qword ptr [rsp + 0x30], 0x0
140a1754c: 48 c7 44 24 28 01 00 00 00  	mov	qword ptr [rsp + 0x28], 0x1
140a17555: 4c 89 e2                    	mov	rdx, r12
140a17558: 49 89 d8                    	mov	r8, rbx
140a1755b: 4d 89 e9                    	mov	r9, r13
140a1755e: e8 dd f7 1f 00              	call	0x140c16d40 <.text+0xc15d40>
140a17563: 48 8b 8d e0 01 00 00        	mov	rcx, qword ptr [rbp + 0x1e0]
140a1756a: 48 ff 01                    	inc	qword ptr [rcx]
140a1756d: 48 89 85 d8 01 00 00        	mov	qword ptr [rbp + 0x1d8], rax
140a17574: b8 01 00 00 00              	mov	eax, 0x1
140a17579: 4d 89 f7                    	mov	r15, r14
140a1757c: 44 0f b6 b5 e8 01 00 00     	movzx	r14d, byte ptr [rbp + 0x1e8]
140a17584: 48 8d 4e 08                 	lea	rcx, [rsi + 0x8]
140a17588: 48 8d 56 20                 	lea	rdx, [rsi + 0x20]
140a1758c: 45 84 f6                    	test	r14b, r14b
140a1758f: 48 0f 45 ca                 	cmovne	rcx, rdx
140a17593: 48 8d 56 10                 	lea	rdx, [rsi + 0x10]
140a17597: 4c 8d 46 28                 	lea	r8, [rsi + 0x28]
140a1759b: 4c 0f 44 c2                 	cmove	r8, rdx
140a1759f: 48 89 85 d0 01 00 00        	mov	qword ptr [rbp + 0x1d0], rax
140a175a6: 49 8b 00                    	mov	rax, qword ptr [r8]
140a175a9: 48 8b 09                    	mov	rcx, qword ptr [rcx]
140a175ac: 0f 10 46 50                 	movups	xmm0, xmmword ptr [rsi + 0x50]
140a175b0: 4c 8d b6 98 00 00 00        	lea	r14, [rsi + 0x98]
140a175b7: 48 8d 56 30                 	lea	rdx, [rsi + 0x30]
140a175bb: 48 89 4d c0                 	mov	qword ptr [rbp - 0x40], rcx
140a175bf: 48 89 45 c8                 	mov	qword ptr [rbp - 0x38], rax
140a175c3: 0f 11 45 d0                 	movups	xmmword ptr [rbp - 0x30], xmm0
140a175c7: 48 89 55 e0                 	mov	qword ptr [rbp - 0x20], rdx
140a175cb: 4c 89 7d e8                 	mov	qword ptr [rbp - 0x18], r15
140a175cf: 48 89 7d f0                 	mov	qword ptr [rbp - 0x10], rdi
140a175d3: 48 8d 55 c0                 	lea	rdx, [rbp - 0x40]
140a175d7: 4c 89 f1                    	mov	rcx, r14
140a175da: e8 d1 86 0c 00              	call	0x140adfcb0 <.text+0xadecb0>
140a175df: 48 b8 02 00 00 00 00 00 00 80       	movabs	rax, -0x7ffffffffffffffe
140a175e9: 48 39 46 60                 	cmp	qword ptr [rsi + 0x60], rax
140a175ed: 7c 26                       	jl	0x140a17615 <.text+0xa16615>
140a175ef: 48 8d 7e 60                 	lea	rdi, [rsi + 0x60]
140a175f3: 48 8d 5d c0                 	lea	rbx, [rbp - 0x40]
140a175f7: 41 b8 00 02 00 00           	mov	r8d, 0x200
140a175fd: 48 89 d9                    	mov	rcx, rbx
140a17600: 4c 89 f2                    	mov	rdx, r14
140a17603: e8 f8 c5 f0 01              	call	0x142923c00 <.text+0x2922c00>
140a17608: 48 89 f9                    	mov	rcx, rdi
140a1760b: 48 89 da                    	mov	rdx, rbx
140a1760e: e8 3d bd a3 00              	call	0x141453350 <.text+0x1452350>
140a17613: eb 67                       	jmp	0x140a1767c <.text+0xa1667c>
140a17615: 48 8b 7e 68                 	mov	rdi, qword ptr [rsi + 0x68]
140a17619: 48 8d 5f 10                 	lea	rbx, [rdi + 0x10]
140a1761d: b1 01                       	mov	cl, 0x1
140a1761f: 31 c0                       	xor	eax, eax
140a17621: f0                          	lock
140a17622: 0f b0 4f 10                 	cmpxchg	byte ptr [rdi + 0x10], cl
140a17626: 0f 85 fe 00 00 00           	jne	0x140a1772a <.text+0xa1672a>
140a1762c: 48 89 d9                    	mov	rcx, rbx
140a1762f: e8 3c a6 26 00              	call	0x140c81c70 <.text+0xc80c70>
140a17634: 48 89 9d e8 01 00 00        	mov	qword ptr [rbp + 0x1e8], rbx
140a1763b: 48 83 c7 18                 	add	rdi, 0x18
140a1763f: 48 8d 5d c0                 	lea	rbx, [rbp - 0x40]
140a17643: 41 b8 00 02 00 00           	mov	r8d, 0x200
140a17649: 48 89 d9                    	mov	rcx, rbx
140a1764c: 4c 89 f2                    	mov	rdx, r14
140a1764f: e8 ac c5 f0 01              	call	0x142923c00 <.text+0x2922c00>
140a17654: 48 89 f9                    	mov	rcx, rdi
140a17657: 48 89 da                    	mov	rdx, rbx
140a1765a: e8 f1 bc a3 00              	call	0x141453350 <.text+0x1452350>
140a1765f: 48 8b bd e8 01 00 00        	mov	rdi, qword ptr [rbp + 0x1e8]
140a17666: 48 89 f9                    	mov	rcx, rdi
140a17669: e8 a2 a7 26 00              	call	0x140c81e10 <.text+0xc80e10>
140a1766e: 31 c9                       	xor	ecx, ecx
140a17670: b0 01                       	mov	al, 0x1
140a17672: f0                          	lock
140a17673: 0f b0 0f                    	cmpxchg	byte ptr [rdi], cl
140a17676: 0f 85 c1 00 00 00           	jne	0x140a1773d <.text+0xa1673d>
140a1767c: c6 86 9a 02 00 00 01        	mov	byte ptr [rsi + 0x29a], 0x1
140a17683: 48 83 bd d0 01 00 00 00     	cmp	qword ptr [rbp + 0x1d0], 0x0
140a1768b: 74 43                       	je	0x140a176d0 <.text+0xa166d0>
140a1768d: e8 9e 0a 20 00              	call	0x140c18130 <.text+0xc17130>
140a17692: 48 89 c1                    	mov	rcx, rax
140a17695: 48 8b 00                    	mov	rax, qword ptr [rax]
140a17698: 48 83 f8 01                 	cmp	rax, 0x1
140a1769c: 75 46                       	jne	0x140a176e4 <.text+0xa166e4>
140a1769e: 48 83 c1 08                 	add	rcx, 0x8
140a176a2: 48 83 39 00                 	cmp	qword ptr [rcx], 0x0
140a176a6: 75 76                       	jne	0x140a1771e <.text+0xa1671e>
140a176a8: 48 c7 01 ff ff ff ff        	mov	qword ptr [rcx], -0x1
140a176af: 48 89 8d e8 01 00 00        	mov	qword ptr [rbp + 0x1e8], rcx
140a176b6: 48 83 c1 08                 	add	rcx, 0x8
140a176ba: 48 8b 95 d8 01 00 00        	mov	rdx, qword ptr [rbp + 0x1d8]
140a176c1: e8 2a fa 1f 00              	call	0x140c170f0 <.text+0xc160f0>
140a176c6: 48 8b 85 e8 01 00 00        	mov	rax, qword ptr [rbp + 0x1e8]
140a176cd: 48 ff 00                    	inc	qword ptr [rax]
140a176d0: 48 81 c4 78 02 00 00        	add	rsp, 0x278
140a176d7: 5b                          	pop	rbx
140a176d8: 5f                          	pop	rdi
140a176d9: 5e                          	pop	rsi
140a176da: 41 5c                       	pop	r12
140a176dc: 41 5d                       	pop	r13
140a176de: 41 5e                       	pop	r14
140a176e0: 41 5f                       	pop	r15
140a176e2: 5d                          	pop	rbp
140a176e3: c3                          	ret
140a176e4: 48 85 c0                    	test	rax, rax
140a176e7: 75 29                       	jne	0x140a17712 <.text+0xa16712>
140a176e9: 31 d2                       	xor	edx, edx
140a176eb: e8 e0 10 ee 01              	call	0x1428f87d0 <.text+0x28f77d0>
140a176f0: 48 89 c1                    	mov	rcx, rax
140a176f3: 48 85 c0                    	test	rax, rax
140a176f6: 75 aa                       	jne	0x140a176a2 <.text+0xa166a2>
140a176f8: eb 18                       	jmp	0x140a17712 <.text+0xa16712>
140a176fa: 48 85 c0                    	test	rax, rax
140a176fd: 75 13                       	jne	0x140a17712 <.text+0xa16712>
140a176ff: 31 d2                       	xor	edx, edx
140a17701: e8 ca 10 ee 01              	call	0x1428f87d0 <.text+0x28f77d0>
140a17706: 48 89 c1                    	mov	rcx, rax
140a17709: 48 85 c0                    	test	rax, rax
140a1770c: 0f 85 10 fe ff ff           	jne	0x140a17522 <.text+0xa16522>
140a17712: 48 8d 0d 87 33 22 02        	lea	rcx, [rip + 0x2223387]  # 0x142c3aaa0
140a17719: e8 b2 09 ec 01              	call	0x1428d80d0 <.text+0x28d70d0>
140a1771e: 48 8d 0d 4b 34 22 02        	lea	rcx, [rip + 0x222344b]  # 0x142c3ab70
140a17725: e8 26 1b ec 01              	call	0x1428d9250 <.text+0x28d8250>
140a1772a: 48 89 d9                    	mov	rcx, rbx
140a1772d: 41 b8 00 ca 9a 3b           	mov	r8d, 0x3b9aca00
140a17733: e8 d8 5a eb 01              	call	0x1428cd210 <.text+0x28cc210>
140a17738: e9 ef fe ff ff              	jmp	0x140a1762c <.text+0xa1662c>
140a1773d: 48 89 f9                    	mov	rcx, rdi
140a17740: 31 d2                       	xor	edx, edx
140a17742: e8 49 64 eb 01              	call	0x1428cdb90 <.text+0x28ccb90>
140a17747: e9 30 ff ff ff              	jmp	0x140a1767c <.text+0xa1667c>
140a1774c: 0f 1f 40 00                 	nop	dword ptr [rax]
140a17750: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a17755: 55                          	push	rbp
140a17756: 41 57                       	push	r15
140a17758: 41 56                       	push	r14
140a1775a: 41 55                       	push	r13
140a1775c: 41 54                       	push	r12
140a1775e: 56                          	push	rsi
140a1775f: 57                          	push	rdi
140a17760: 53                          	push	rbx
140a17761: 48 83 ec 38                 	sub	rsp, 0x38
140a17765: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a1776c: 48 8b 85 e0 01 00 00        	mov	rax, qword ptr [rbp + 0x1e0]
140a17773: 48 ff 00                    	inc	qword ptr [rax]
140a17776: 48 83 c4 38                 	add	rsp, 0x38
140a1777a: 5b                          	pop	rbx
140a1777b: 5f                          	pop	rdi
140a1777c: 5e                          	pop	rsi
140a1777d: 41 5c                       	pop	r12
140a1777f: 41 5d                       	pop	r13
140a17781: 41 5e                       	pop	r14
140a17783: 41 5f                       	pop	r15
140a17785: 5d                          	pop	rbp
140a17786: c3                          	ret
140a17787: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
140a17790: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a17795: 55                          	push	rbp
140a17796: 41 57                       	push	r15
140a17798: 41 56                       	push	r14
140a1779a: 41 55                       	push	r13
140a1779c: 41 54                       	push	r12
140a1779e: 56                          	push	rsi
140a1779f: 57                          	push	rdi
140a177a0: 53                          	push	rbx
140a177a1: 48 83 ec 38                 	sub	rsp, 0x38
140a177a5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a177ac: 48 83 bd d0 01 00 00 00     	cmp	qword ptr [rbp + 0x1d0], 0x0
140a177b4: 74 2f                       	je	0x140a177e5 <.text+0xa167e5>
140a177b6: 48 8d 85 d8 01 00 00        	lea	rax, [rbp + 0x1d8]
140a177bd: 48 89 85 c8 01 00 00        	mov	qword ptr [rbp + 0x1c8], rax
140a177c4: 48 8d 85 c8 01 00 00        	lea	rax, [rbp + 0x1c8]
140a177cb: 48 89 85 c0 01 00 00        	mov	qword ptr [rbp + 0x1c0], rax
140a177d2: 48 8d 0d 67 0c 0b 02        	lea	rcx, [rip + 0x20b0c67]  # 0x142ac8440
140a177d9: 48 8d 95 c0 01 00 00        	lea	rdx, [rbp + 0x1c0]
140a177e0: e8 eb 7a 91 00              	call	0x14132f2d0 <.text+0x132e2d0>
140a177e5: 90                          	nop
140a177e6: 48 83 c4 38                 	add	rsp, 0x38
140a177ea: 5b                          	pop	rbx
140a177eb: 5f                          	pop	rdi
140a177ec: 5e                          	pop	rsi
140a177ed: 41 5c                       	pop	r12
140a177ef: 41 5d                       	pop	r13
140a177f1: 41 5e                       	pop	r14
140a177f3: 41 5f                       	pop	r15
140a177f5: 5d                          	pop	rbp
140a177f6: c3                          	ret
140a177f7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
140a17800: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a17805: 55                          	push	rbp
140a17806: 41 57                       	push	r15
140a17808: 41 56                       	push	r14
140a1780a: 41 55                       	push	r13
140a1780c: 41 54                       	push	r12
140a1780e: 56                          	push	rsi
140a1780f: 57                          	push	rdi
140a17810: 53                          	push	rbx
140a17811: 48 83 ec 38                 	sub	rsp, 0x38
140a17815: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a1781c: 48 8b 85 e8 01 00 00        	mov	rax, qword ptr [rbp + 0x1e8]
140a17823: 48 ff 00                    	inc	qword ptr [rax]
140a17826: 48 83 c4 38                 	add	rsp, 0x38
140a1782a: 5b                          	pop	rbx
140a1782b: 5f                          	pop	rdi
140a1782c: 5e                          	pop	rsi
140a1782d: 41 5c                       	pop	r12
140a1782f: 41 5d                       	pop	r13
140a17831: 41 5e                       	pop	r14
140a17833: 41 5f                       	pop	r15
140a17835: 5d                          	pop	rbp
140a17836: c3                          	ret
140a17837: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
140a17840: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a17845: 55                          	push	rbp
140a17846: 41 57                       	push	r15
140a17848: 41 56                       	push	r14
140a1784a: 41 55                       	push	r13
140a1784c: 41 54                       	push	r12
140a1784e: 56                          	push	rsi
140a1784f: 57                          	push	rdi
140a17850: 53                          	push	rbx
140a17851: 48 83 ec 38                 	sub	rsp, 0x38
140a17855: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a1785c: 48 8b 8d e8 01 00 00        	mov	rcx, qword ptr [rbp + 0x1e8]
140a17863: e8 c8 a3 74 ff              	call	0x140161c30 <.text+0x160c30>
140a17868: 90                          	nop
140a17869: 48 83 c4 38                 	add	rsp, 0x38
140a1786d: 5b                          	pop	rbx
140a1786e: 5f                          	pop	rdi
140a1786f: 5e                          	pop	rsi
140a17870: 41 5c                       	pop	r12
140a17872: 41 5d                       	pop	r13
140a17874: 41 5e                       	pop	r14
140a17876: 41 5f                       	pop	r15
140a17878: 5d                          	pop	rbp
140a17879: c3                          	ret
