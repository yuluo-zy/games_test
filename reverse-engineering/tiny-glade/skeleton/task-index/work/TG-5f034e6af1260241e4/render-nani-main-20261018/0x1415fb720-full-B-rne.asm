
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1415fb720: 41 57                       	push	r15
1415fb722: 41 56                       	push	r14
1415fb724: 41 55                       	push	r13
1415fb726: 41 54                       	push	r12
1415fb728: 56                          	push	rsi
1415fb729: 57                          	push	rdi
1415fb72a: 55                          	push	rbp
1415fb72b: 53                          	push	rbx
1415fb72c: 48 83 ec 58                 	sub	rsp, 0x58
1415fb730: 44 0f b6 41 08              	movzx	r8d, byte ptr [rcx + 0x8]
1415fb735: 0f b6 41 09                 	movzx	eax, byte ptr [rcx + 0x9]
1415fb739: 48 89 44 24 50              	mov	qword ptr [rsp + 0x50], rax
1415fb73e: 41 38 c0                    	cmp	r8b, al
1415fb741: 0f 83 34 03 00 00           	jae	0x1415fba7b <.text+0x15faa7b>
1415fb747: 48 8b 32                    	mov	rsi, qword ptr [rdx]
1415fb74a: 48 8b 42 08                 	mov	rax, qword ptr [rdx + 0x8]
1415fb74e: 48 89 44 24 48              	mov	qword ptr [rsp + 0x48], rax
1415fb753: 8b 5a 10                    	mov	ebx, dword ptr [rdx + 0x10]
1415fb756: 4c 8b 29                    	mov	r13, qword ptr [rcx]
1415fb759: 49 8d 40 01                 	lea	rax, [r8 + 0x1]
1415fb75d: 48 89 44 24 40              	mov	qword ptr [rsp + 0x40], rax
1415fb762: 41 c1 e0 04                 	shl	r8d, 0x4
1415fb766: 49 81 c0 40 14 00 00        	add	r8, 0x1440
1415fb76d: 31 c0                       	xor	eax, eax
1415fb76f: 4c 89 44 24 38              	mov	qword ptr [rsp + 0x38], r8
1415fb774: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
1415fb780: 44 0f b6 f0                 	movzx	r14d, al
1415fb784: 48 8d 15 ed 31 68 01        	lea	rdx, [rip + 0x16831ed]  # 0x142c7e978
1415fb78b: 4a 63 0c b2                 	movsxd	rcx, dword ptr [rdx + 4*r14]
1415fb78f: 48 01 d1                    	add	rcx, rdx
1415fb792: ff e1                       	jmp	rcx
1415fb794: b1 01                       	mov	cl, 0x1
1415fb796: 80 3d ff 95 03 02 00        	cmp	byte ptr [rip + 0x20395ff], 0x0 # 0x143634d9c
1415fb79d: 0f 84 dd 01 00 00           	je	0x1415fb980 <.text+0x15fa980>
1415fb7a3: e9 f1 01 00 00              	jmp	0x1415fb999 <.text+0x15fa999>
1415fb7a8: b1 11                       	mov	cl, 0x11
1415fb7aa: 80 3d eb 95 03 02 00        	cmp	byte ptr [rip + 0x20395eb], 0x0 # 0x143634d9c
1415fb7b1: 0f 84 c9 01 00 00           	je	0x1415fb980 <.text+0x15fa980>
1415fb7b7: e9 dd 01 00 00              	jmp	0x1415fb999 <.text+0x15fa999>
1415fb7bc: b1 0f                       	mov	cl, 0xf
1415fb7be: 80 3d d7 95 03 02 00        	cmp	byte ptr [rip + 0x20395d7], 0x0 # 0x143634d9c
1415fb7c5: 0f 84 b5 01 00 00           	je	0x1415fb980 <.text+0x15fa980>
1415fb7cb: e9 c9 01 00 00              	jmp	0x1415fb999 <.text+0x15fa999>
1415fb7d0: b1 0c                       	mov	cl, 0xc
1415fb7d2: 80 3d c3 95 03 02 00        	cmp	byte ptr [rip + 0x20395c3], 0x0 # 0x143634d9c
1415fb7d9: 0f 84 a1 01 00 00           	je	0x1415fb980 <.text+0x15fa980>
1415fb7df: e9 b5 01 00 00              	jmp	0x1415fb999 <.text+0x15fa999>
1415fb7e4: b1 05                       	mov	cl, 0x5
1415fb7e6: 80 3d af 95 03 02 00        	cmp	byte ptr [rip + 0x20395af], 0x0 # 0x143634d9c
1415fb7ed: 0f 84 8d 01 00 00           	je	0x1415fb980 <.text+0x15fa980>
1415fb7f3: e9 a1 01 00 00              	jmp	0x1415fb999 <.text+0x15fa999>
1415fb7f8: b1 0d                       	mov	cl, 0xd
1415fb7fa: 80 3d 9b 95 03 02 00        	cmp	byte ptr [rip + 0x203959b], 0x0 # 0x143634d9c
1415fb801: 0f 84 79 01 00 00           	je	0x1415fb980 <.text+0x15fa980>
1415fb807: e9 8d 01 00 00              	jmp	0x1415fb999 <.text+0x15fa999>
1415fb80c: b1 0a                       	mov	cl, 0xa
1415fb80e: 80 3d 87 95 03 02 00        	cmp	byte ptr [rip + 0x2039587], 0x0 # 0x143634d9c
1415fb815: 0f 84 65 01 00 00           	je	0x1415fb980 <.text+0x15fa980>
1415fb81b: e9 79 01 00 00              	jmp	0x1415fb999 <.text+0x15fa999>
1415fb820: b1 1a                       	mov	cl, 0x1a
1415fb822: 80 3d 73 95 03 02 00        	cmp	byte ptr [rip + 0x2039573], 0x0 # 0x143634d9c
1415fb829: 0f 84 51 01 00 00           	je	0x1415fb980 <.text+0x15fa980>
1415fb82f: e9 65 01 00 00              	jmp	0x1415fb999 <.text+0x15fa999>
1415fb834: b1 19                       	mov	cl, 0x19
1415fb836: 80 3d 5f 95 03 02 00        	cmp	byte ptr [rip + 0x203955f], 0x0 # 0x143634d9c
1415fb83d: 0f 84 3d 01 00 00           	je	0x1415fb980 <.text+0x15fa980>
1415fb843: e9 51 01 00 00              	jmp	0x1415fb999 <.text+0x15fa999>
1415fb848: b1 03                       	mov	cl, 0x3
1415fb84a: 80 3d 4b 95 03 02 00        	cmp	byte ptr [rip + 0x203954b], 0x0 # 0x143634d9c
1415fb851: 0f 84 29 01 00 00           	je	0x1415fb980 <.text+0x15fa980>
1415fb857: e9 3d 01 00 00              	jmp	0x1415fb999 <.text+0x15fa999>
1415fb85c: b1 10                       	mov	cl, 0x10
1415fb85e: 80 3d 37 95 03 02 00        	cmp	byte ptr [rip + 0x2039537], 0x0 # 0x143634d9c
1415fb865: 0f 84 15 01 00 00           	je	0x1415fb980 <.text+0x15fa980>
1415fb86b: e9 29 01 00 00              	jmp	0x1415fb999 <.text+0x15fa999>
1415fb870: b1 04                       	mov	cl, 0x4
1415fb872: 80 3d 23 95 03 02 00        	cmp	byte ptr [rip + 0x2039523], 0x0 # 0x143634d9c
1415fb879: 0f 84 01 01 00 00           	je	0x1415fb980 <.text+0x15fa980>
1415fb87f: e9 15 01 00 00              	jmp	0x1415fb999 <.text+0x15fa999>
1415fb884: b1 08                       	mov	cl, 0x8
1415fb886: 80 3d 0f 95 03 02 00        	cmp	byte ptr [rip + 0x203950f], 0x0 # 0x143634d9c
1415fb88d: 0f 84 ed 00 00 00           	je	0x1415fb980 <.text+0x15fa980>
1415fb893: e9 01 01 00 00              	jmp	0x1415fb999 <.text+0x15fa999>
1415fb898: b1 02                       	mov	cl, 0x2
1415fb89a: 80 3d fb 94 03 02 00        	cmp	byte ptr [rip + 0x20394fb], 0x0 # 0x143634d9c
1415fb8a1: 0f 84 d9 00 00 00           	je	0x1415fb980 <.text+0x15fa980>
1415fb8a7: e9 ed 00 00 00              	jmp	0x1415fb999 <.text+0x15fa999>
1415fb8ac: b1 12                       	mov	cl, 0x12
1415fb8ae: 80 3d e7 94 03 02 00        	cmp	byte ptr [rip + 0x20394e7], 0x0 # 0x143634d9c
1415fb8b5: 0f 84 c5 00 00 00           	je	0x1415fb980 <.text+0x15fa980>
1415fb8bb: e9 d9 00 00 00              	jmp	0x1415fb999 <.text+0x15fa999>
1415fb8c0: b1 15                       	mov	cl, 0x15
1415fb8c2: 80 3d d3 94 03 02 00        	cmp	byte ptr [rip + 0x20394d3], 0x0 # 0x143634d9c
1415fb8c9: 0f 84 b1 00 00 00           	je	0x1415fb980 <.text+0x15fa980>
1415fb8cf: e9 c5 00 00 00              	jmp	0x1415fb999 <.text+0x15fa999>
1415fb8d4: b1 06                       	mov	cl, 0x6
1415fb8d6: 80 3d bf 94 03 02 00        	cmp	byte ptr [rip + 0x20394bf], 0x0 # 0x143634d9c
1415fb8dd: 0f 84 9d 00 00 00           	je	0x1415fb980 <.text+0x15fa980>
1415fb8e3: e9 b1 00 00 00              	jmp	0x1415fb999 <.text+0x15fa999>
1415fb8e8: b1 16                       	mov	cl, 0x16
1415fb8ea: 80 3d ab 94 03 02 00        	cmp	byte ptr [rip + 0x20394ab], 0x0 # 0x143634d9c
1415fb8f1: 0f 84 89 00 00 00           	je	0x1415fb980 <.text+0x15fa980>
1415fb8f7: e9 9d 00 00 00              	jmp	0x1415fb999 <.text+0x15fa999>
1415fb8fc: b1 0e                       	mov	cl, 0xe
1415fb8fe: 80 3d 97 94 03 02 00        	cmp	byte ptr [rip + 0x2039497], 0x0 # 0x143634d9c
1415fb905: 74 79                       	je	0x1415fb980 <.text+0x15fa980>
1415fb907: e9 8d 00 00 00              	jmp	0x1415fb999 <.text+0x15fa999>
1415fb90c: b1 07                       	mov	cl, 0x7
1415fb90e: 80 3d 87 94 03 02 00        	cmp	byte ptr [rip + 0x2039487], 0x0 # 0x143634d9c
1415fb915: 74 69                       	je	0x1415fb980 <.text+0x15fa980>
1415fb917: e9 7d 00 00 00              	jmp	0x1415fb999 <.text+0x15fa999>
1415fb91c: b1 13                       	mov	cl, 0x13
1415fb91e: 80 3d 77 94 03 02 00        	cmp	byte ptr [rip + 0x2039477], 0x0 # 0x143634d9c
1415fb925: 74 59                       	je	0x1415fb980 <.text+0x15fa980>
1415fb927: eb 70                       	jmp	0x1415fb999 <.text+0x15fa999>
1415fb929: b1 0b                       	mov	cl, 0xb
1415fb92b: 80 3d 6a 94 03 02 00        	cmp	byte ptr [rip + 0x203946a], 0x0 # 0x143634d9c
1415fb932: 74 4c                       	je	0x1415fb980 <.text+0x15fa980>
1415fb934: eb 63                       	jmp	0x1415fb999 <.text+0x15fa999>
1415fb936: b1 09                       	mov	cl, 0x9
1415fb938: 80 3d 5d 94 03 02 00        	cmp	byte ptr [rip + 0x203945d], 0x0 # 0x143634d9c
1415fb93f: 74 3f                       	je	0x1415fb980 <.text+0x15fa980>
1415fb941: eb 56                       	jmp	0x1415fb999 <.text+0x15fa999>
1415fb943: b1 17                       	mov	cl, 0x17
1415fb945: 80 3d 50 94 03 02 00        	cmp	byte ptr [rip + 0x2039450], 0x0 # 0x143634d9c
1415fb94c: 74 32                       	je	0x1415fb980 <.text+0x15fa980>
1415fb94e: eb 49                       	jmp	0x1415fb999 <.text+0x15fa999>
1415fb950: b1 14                       	mov	cl, 0x14
1415fb952: 80 3d 43 94 03 02 00        	cmp	byte ptr [rip + 0x2039443], 0x0 # 0x143634d9c
1415fb959: 74 25                       	je	0x1415fb980 <.text+0x15fa980>
1415fb95b: eb 3c                       	jmp	0x1415fb999 <.text+0x15fa999>
1415fb95d: b1 18                       	mov	cl, 0x18
1415fb95f: 80 3d 36 94 03 02 00        	cmp	byte ptr [rip + 0x2039436], 0x0 # 0x143634d9c
1415fb966: 74 18                       	je	0x1415fb980 <.text+0x15fa980>
1415fb968: eb 2f                       	jmp	0x1415fb999 <.text+0x15fa999>
1415fb96a: b1 1b                       	mov	cl, 0x1b
1415fb96c: 80 3d 29 94 03 02 00        	cmp	byte ptr [rip + 0x2039429], 0x0 # 0x143634d9c
1415fb973: 75 24                       	jne	0x1415fb999 <.text+0x15fa999>
1415fb975: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
1415fb980: 3c 01                       	cmp	al, 0x1
1415fb982: 89 c8                       	mov	eax, ecx
1415fb984: 0f 87 f6 fd ff ff           	ja	0x1415fb780 <.text+0x15fa780>
1415fb98a: 80 3d 0c 94 03 02 00        	cmp	byte ptr [rip + 0x203940c], 0x0 # 0x143634d9d
1415fb991: 89 c8                       	mov	eax, ecx
1415fb993: 0f 85 e7 fd ff ff           	jne	0x1415fb780 <.text+0x15fa780>
1415fb999: 88 4c 24 2f                 	mov	byte ptr [rsp + 0x2f], cl
1415fb99d: 48 8b 7c 24 40              	mov	rdi, qword ptr [rsp + 0x40]
1415fb9a2: 4a 8d 2c b7                 	lea	rbp, [rdi + 4*r14]
1415fb9a6: 41 c1 e6 06                 	shl	r14d, 0x6
1415fb9aa: 4d 01 c6                    	add	r14, r8
1415fb9ad: 0f 1f 00                    	nop	dword ptr [rax]
1415fb9b0: 48 8d 4d ff                 	lea	rcx, [rbp - 0x1]
1415fb9b4: 49 8b 55 58                 	mov	rdx, qword ptr [r13 + 0x58]
1415fb9b8: 48 39 ca                    	cmp	rdx, rcx
1415fb9bb: 0f 86 72 02 00 00           	jbe	0x1415fbc33 <.text+0x15fac33>
1415fb9c1: 49 8b 45 50                 	mov	rax, qword ptr [r13 + 0x50]
1415fb9c5: 48 39 d5                    	cmp	rbp, rdx
1415fb9c8: 73 03                       	jae	0x1415fb9cd <.text+0x15fa9cd>
1415fb9ca: 8b 0c a8                    	mov	ecx, dword ptr [rax + 4*rbp]
1415fb9cd: 48 39 d5                    	cmp	rbp, rdx
1415fb9d0: 72 04                       	jb	0x1415fb9d6 <.text+0x15fa9d6>
1415fb9d2: 41 8b 4d 6c                 	mov	ecx, dword ptr [r13 + 0x6c]
1415fb9d6: 8b 44 a8 fc                 	mov	eax, dword ptr [rax + 4*rbp - 0x4]
1415fb9da: 89 44 24 30                 	mov	dword ptr [rsp + 0x30], eax
1415fb9de: 89 4c 24 34                 	mov	dword ptr [rsp + 0x34], ecx
1415fb9e2: 4c 8d 44 24 30              	lea	r8, [rsp + 0x30]
1415fb9e7: 41 b9 08 00 00 00           	mov	r9d, 0x8
1415fb9ed: 48 89 f1                    	mov	rcx, rsi
1415fb9f0: 48 8b 54 24 48              	mov	rdx, qword ptr [rsp + 0x48]
1415fb9f5: e8 56 c0 0f 00              	call	0x1416f7a50 <.text+0x16f6a50>
1415fb9fa: 4c 8b 26                    	mov	r12, qword ptr [rsi]
1415fb9fd: 4c 8b 7e 08                 	mov	r15, qword ptr [rsi + 0x8]
1415fba01: 41 80 bf ba 0a 00 00 01     	cmp	byte ptr [r15 + 0xaba], 0x1
1415fba09: 75 0d                       	jne	0x1415fba18 <.text+0x15faa18>
1415fba0b: 49 8b 54 24 48              	mov	rdx, qword ptr [r12 + 0x48]
1415fba10: 4c 89 f9                    	mov	rcx, r15
1415fba13: e8 78 26 5f ff              	call	0x140bee090 <.text+0xbed090>
1415fba18: f0                          	lock
1415fba19: 49 ff 87 a0 0a 00 00        	inc	qword ptr [r15 + 0xaa0]
1415fba20: 4d 8b 64 24 48              	mov	r12, qword ptr [r12 + 0x48]
1415fba25: 48 8b 4e 10                 	mov	rcx, qword ptr [rsi + 0x10]
1415fba29: 89 da                       	mov	edx, ebx
1415fba2b: e8 a0 bf 5e ff              	call	0x140be79d0 <.text+0xbe69d0>
1415fba30: 4c 89 f9                    	mov	rcx, r15
1415fba33: 48 89 c2                    	mov	rdx, rax
1415fba36: 45 31 c0                    	xor	r8d, r8d
1415fba39: e8 32 3a 5f ff              	call	0x140bef470 <.text+0xbee470>
1415fba3e: a8 01                       	test	al, 0x1
1415fba40: 0f 84 f9 01 00 00           	je	0x1415fbc3f <.text+0x15fac3f>
1415fba46: 4c 89 e1                    	mov	rcx, r12
1415fba49: 4d 89 f0                    	mov	r8, r14
1415fba4c: 41 ff 97 68 03 00 00        	call	qword ptr [r15 + 0x368]
1415fba53: 48 ff c5                    	inc	rbp
1415fba56: 49 83 c6 10                 	add	r14, 0x10
1415fba5a: 48 8d 47 01                 	lea	rax, [rdi + 0x1]
1415fba5e: 48 3b 7c 24 50              	cmp	rdi, qword ptr [rsp + 0x50]
1415fba63: 48 89 c7                    	mov	rdi, rax
1415fba66: 0f 82 44 ff ff ff           	jb	0x1415fb9b0 <.text+0x15fa9b0>
1415fba6c: 0f b6 44 24 2f              	movzx	eax, byte ptr [rsp + 0x2f]
1415fba71: 4c 8b 44 24 38              	mov	r8, qword ptr [rsp + 0x38]
1415fba76: e9 05 fd ff ff              	jmp	0x1415fb780 <.text+0x15fa780>
1415fba7b: f6 05 1a 93 03 02 01        	test	byte ptr [rip + 0x203931a], 0x1 # 0x143634d9c
1415fba82: 0f 84 8a 00 00 00           	je	0x1415fbb12 <.text+0x15fab12>
1415fba88: 31 c9                       	xor	ecx, ecx
1415fba8a: 48 8d 05 77 2e 68 01        	lea	rax, [rip + 0x1682e77]  # 0x142c7e908
1415fba91: eb 0d                       	jmp	0x1415fbaa0 <.text+0x15faaa0>
1415fba93: b1 18                       	mov	cl, 0x18
1415fba95: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
1415fbaa0: 0f b6 c9                    	movzx	ecx, cl
1415fbaa3: 48 63 14 88                 	movsxd	rdx, dword ptr [rax + 4*rcx]
1415fbaa7: 48 01 c2                    	add	rdx, rax
1415fbaaa: b1 02                       	mov	cl, 0x2
1415fbaac: ff e2                       	jmp	rdx
1415fbaae: b1 01                       	mov	cl, 0x1
1415fbab0: eb ee                       	jmp	0x1415fbaa0 <.text+0x15faaa0>
1415fbab2: b1 14                       	mov	cl, 0x14
1415fbab4: eb ea                       	jmp	0x1415fbaa0 <.text+0x15faaa0>
1415fbab6: b1 0f                       	mov	cl, 0xf
1415fbab8: eb e6                       	jmp	0x1415fbaa0 <.text+0x15faaa0>
1415fbaba: b1 1a                       	mov	cl, 0x1a
1415fbabc: eb e2                       	jmp	0x1415fbaa0 <.text+0x15faaa0>
1415fbabe: b1 16                       	mov	cl, 0x16
1415fbac0: eb de                       	jmp	0x1415fbaa0 <.text+0x15faaa0>
1415fbac2: b1 09                       	mov	cl, 0x9
1415fbac4: eb da                       	jmp	0x1415fbaa0 <.text+0x15faaa0>
1415fbac6: b1 07                       	mov	cl, 0x7
1415fbac8: eb d6                       	jmp	0x1415fbaa0 <.text+0x15faaa0>
1415fbaca: b1 03                       	mov	cl, 0x3
1415fbacc: eb d2                       	jmp	0x1415fbaa0 <.text+0x15faaa0>
1415fbace: b1 08                       	mov	cl, 0x8
1415fbad0: eb ce                       	jmp	0x1415fbaa0 <.text+0x15faaa0>
1415fbad2: b1 19                       	mov	cl, 0x19
1415fbad4: eb ca                       	jmp	0x1415fbaa0 <.text+0x15faaa0>
1415fbad6: b1 0a                       	mov	cl, 0xa
1415fbad8: eb c6                       	jmp	0x1415fbaa0 <.text+0x15faaa0>
1415fbada: b1 13                       	mov	cl, 0x13
1415fbadc: eb c2                       	jmp	0x1415fbaa0 <.text+0x15faaa0>
1415fbade: b1 06                       	mov	cl, 0x6
1415fbae0: eb be                       	jmp	0x1415fbaa0 <.text+0x15faaa0>
1415fbae2: b1 05                       	mov	cl, 0x5
1415fbae4: eb ba                       	jmp	0x1415fbaa0 <.text+0x15faaa0>
1415fbae6: b1 04                       	mov	cl, 0x4
1415fbae8: eb b6                       	jmp	0x1415fbaa0 <.text+0x15faaa0>
1415fbaea: b1 1b                       	mov	cl, 0x1b
1415fbaec: eb b2                       	jmp	0x1415fbaa0 <.text+0x15faaa0>
1415fbaee: b1 0e                       	mov	cl, 0xe
1415fbaf0: eb ae                       	jmp	0x1415fbaa0 <.text+0x15faaa0>
1415fbaf2: b1 15                       	mov	cl, 0x15
1415fbaf4: eb aa                       	jmp	0x1415fbaa0 <.text+0x15faaa0>
1415fbaf6: b1 0d                       	mov	cl, 0xd
1415fbaf8: eb a6                       	jmp	0x1415fbaa0 <.text+0x15faaa0>
1415fbafa: b1 11                       	mov	cl, 0x11
1415fbafc: eb a2                       	jmp	0x1415fbaa0 <.text+0x15faaa0>
1415fbafe: b1 10                       	mov	cl, 0x10
1415fbb00: eb 9e                       	jmp	0x1415fbaa0 <.text+0x15faaa0>
1415fbb02: b1 12                       	mov	cl, 0x12
1415fbb04: eb 9a                       	jmp	0x1415fbaa0 <.text+0x15faaa0>
1415fbb06: b1 0b                       	mov	cl, 0xb
1415fbb08: eb 96                       	jmp	0x1415fbaa0 <.text+0x15faaa0>
1415fbb0a: b1 17                       	mov	cl, 0x17
1415fbb0c: eb 92                       	jmp	0x1415fbaa0 <.text+0x15faaa0>
1415fbb0e: b1 0c                       	mov	cl, 0xc
1415fbb10: eb 8e                       	jmp	0x1415fbaa0 <.text+0x15faaa0>
1415fbb12: f6 05 84 92 03 02 01        	test	byte ptr [rip + 0x2039284], 0x1 # 0x143634d9d
1415fbb19: 0f 84 83 00 00 00           	je	0x1415fbba2 <.text+0x15faba2>
1415fbb1f: 31 c9                       	xor	ecx, ecx
1415fbb21: 48 8d 05 70 2d 68 01        	lea	rax, [rip + 0x1682d70]  # 0x142c7e898
1415fbb28: eb 06                       	jmp	0x1415fbb30 <.text+0x15fab30>
1415fbb2a: b1 18                       	mov	cl, 0x18
1415fbb2c: 0f 1f 40 00                 	nop	dword ptr [rax]
1415fbb30: 0f b6 c9                    	movzx	ecx, cl
1415fbb33: 48 63 14 88                 	movsxd	rdx, dword ptr [rax + 4*rcx]
1415fbb37: 48 01 c2                    	add	rdx, rax
1415fbb3a: b1 02                       	mov	cl, 0x2
1415fbb3c: ff e2                       	jmp	rdx
1415fbb3e: b1 01                       	mov	cl, 0x1
1415fbb40: eb ee                       	jmp	0x1415fbb30 <.text+0x15fab30>
1415fbb42: b1 14                       	mov	cl, 0x14
1415fbb44: eb ea                       	jmp	0x1415fbb30 <.text+0x15fab30>
1415fbb46: b1 0f                       	mov	cl, 0xf
1415fbb48: eb e6                       	jmp	0x1415fbb30 <.text+0x15fab30>
1415fbb4a: b1 1a                       	mov	cl, 0x1a
1415fbb4c: eb e2                       	jmp	0x1415fbb30 <.text+0x15fab30>
1415fbb4e: b1 16                       	mov	cl, 0x16
1415fbb50: eb de                       	jmp	0x1415fbb30 <.text+0x15fab30>
1415fbb52: b1 09                       	mov	cl, 0x9
1415fbb54: eb da                       	jmp	0x1415fbb30 <.text+0x15fab30>
1415fbb56: b1 07                       	mov	cl, 0x7
1415fbb58: eb d6                       	jmp	0x1415fbb30 <.text+0x15fab30>
1415fbb5a: b1 03                       	mov	cl, 0x3
1415fbb5c: eb d2                       	jmp	0x1415fbb30 <.text+0x15fab30>
1415fbb5e: b1 08                       	mov	cl, 0x8
1415fbb60: eb ce                       	jmp	0x1415fbb30 <.text+0x15fab30>
1415fbb62: b1 19                       	mov	cl, 0x19
1415fbb64: eb ca                       	jmp	0x1415fbb30 <.text+0x15fab30>
1415fbb66: b1 0a                       	mov	cl, 0xa
1415fbb68: eb c6                       	jmp	0x1415fbb30 <.text+0x15fab30>
1415fbb6a: b1 13                       	mov	cl, 0x13
1415fbb6c: eb c2                       	jmp	0x1415fbb30 <.text+0x15fab30>
1415fbb6e: b1 06                       	mov	cl, 0x6
1415fbb70: eb be                       	jmp	0x1415fbb30 <.text+0x15fab30>
1415fbb72: b1 05                       	mov	cl, 0x5
1415fbb74: eb ba                       	jmp	0x1415fbb30 <.text+0x15fab30>
1415fbb76: b1 04                       	mov	cl, 0x4
1415fbb78: eb b6                       	jmp	0x1415fbb30 <.text+0x15fab30>
1415fbb7a: b1 1b                       	mov	cl, 0x1b
1415fbb7c: eb b2                       	jmp	0x1415fbb30 <.text+0x15fab30>
1415fbb7e: b1 0e                       	mov	cl, 0xe
1415fbb80: eb ae                       	jmp	0x1415fbb30 <.text+0x15fab30>
1415fbb82: b1 15                       	mov	cl, 0x15
1415fbb84: eb aa                       	jmp	0x1415fbb30 <.text+0x15fab30>
1415fbb86: b1 0d                       	mov	cl, 0xd
1415fbb88: eb a6                       	jmp	0x1415fbb30 <.text+0x15fab30>
1415fbb8a: b1 11                       	mov	cl, 0x11
1415fbb8c: eb a2                       	jmp	0x1415fbb30 <.text+0x15fab30>
1415fbb8e: b1 10                       	mov	cl, 0x10
1415fbb90: eb 9e                       	jmp	0x1415fbb30 <.text+0x15fab30>
1415fbb92: b1 12                       	mov	cl, 0x12
1415fbb94: eb 9a                       	jmp	0x1415fbb30 <.text+0x15fab30>
1415fbb96: b1 0b                       	mov	cl, 0xb
1415fbb98: eb 96                       	jmp	0x1415fbb30 <.text+0x15fab30>
1415fbb9a: b1 17                       	mov	cl, 0x17
1415fbb9c: eb 92                       	jmp	0x1415fbb30 <.text+0x15fab30>
1415fbb9e: b1 0c                       	mov	cl, 0xc
1415fbba0: eb 8e                       	jmp	0x1415fbb30 <.text+0x15fab30>
1415fbba2: 31 c9                       	xor	ecx, ecx
1415fbba4: 48 8d 05 7d 2c 68 01        	lea	rax, [rip + 0x1682c7d]  # 0x142c7e828
1415fbbab: eb 03                       	jmp	0x1415fbbb0 <.text+0x15fabb0>
1415fbbad: b1 18                       	mov	cl, 0x18
1415fbbaf: 90                          	nop
1415fbbb0: 0f b6 c9                    	movzx	ecx, cl
1415fbbb3: 48 63 14 88                 	movsxd	rdx, dword ptr [rax + 4*rcx]
1415fbbb7: 48 01 c2                    	add	rdx, rax
1415fbbba: b1 02                       	mov	cl, 0x2
1415fbbbc: ff e2                       	jmp	rdx
1415fbbbe: b1 01                       	mov	cl, 0x1
1415fbbc0: eb ee                       	jmp	0x1415fbbb0 <.text+0x15fabb0>
1415fbbc2: b1 14                       	mov	cl, 0x14
1415fbbc4: eb ea                       	jmp	0x1415fbbb0 <.text+0x15fabb0>
1415fbbc6: b1 0f                       	mov	cl, 0xf
1415fbbc8: eb e6                       	jmp	0x1415fbbb0 <.text+0x15fabb0>
1415fbbca: b1 1a                       	mov	cl, 0x1a
1415fbbcc: eb e2                       	jmp	0x1415fbbb0 <.text+0x15fabb0>
1415fbbce: b1 16                       	mov	cl, 0x16
1415fbbd0: eb de                       	jmp	0x1415fbbb0 <.text+0x15fabb0>
1415fbbd2: b1 09                       	mov	cl, 0x9
1415fbbd4: eb da                       	jmp	0x1415fbbb0 <.text+0x15fabb0>
1415fbbd6: b1 07                       	mov	cl, 0x7
1415fbbd8: eb d6                       	jmp	0x1415fbbb0 <.text+0x15fabb0>
1415fbbda: b1 03                       	mov	cl, 0x3
1415fbbdc: eb d2                       	jmp	0x1415fbbb0 <.text+0x15fabb0>
1415fbbde: b1 08                       	mov	cl, 0x8
1415fbbe0: eb ce                       	jmp	0x1415fbbb0 <.text+0x15fabb0>
1415fbbe2: b1 19                       	mov	cl, 0x19
1415fbbe4: eb ca                       	jmp	0x1415fbbb0 <.text+0x15fabb0>
1415fbbe6: b1 0a                       	mov	cl, 0xa
1415fbbe8: eb c6                       	jmp	0x1415fbbb0 <.text+0x15fabb0>
1415fbbea: b1 13                       	mov	cl, 0x13
1415fbbec: eb c2                       	jmp	0x1415fbbb0 <.text+0x15fabb0>
1415fbbee: b1 06                       	mov	cl, 0x6
1415fbbf0: eb be                       	jmp	0x1415fbbb0 <.text+0x15fabb0>
1415fbbf2: b1 05                       	mov	cl, 0x5
1415fbbf4: eb ba                       	jmp	0x1415fbbb0 <.text+0x15fabb0>
1415fbbf6: b1 04                       	mov	cl, 0x4
1415fbbf8: eb b6                       	jmp	0x1415fbbb0 <.text+0x15fabb0>
1415fbbfa: b1 1b                       	mov	cl, 0x1b
1415fbbfc: eb b2                       	jmp	0x1415fbbb0 <.text+0x15fabb0>
1415fbbfe: b1 0e                       	mov	cl, 0xe
1415fbc00: eb ae                       	jmp	0x1415fbbb0 <.text+0x15fabb0>
1415fbc02: b1 15                       	mov	cl, 0x15
1415fbc04: eb aa                       	jmp	0x1415fbbb0 <.text+0x15fabb0>
1415fbc06: b1 0d                       	mov	cl, 0xd
1415fbc08: eb a6                       	jmp	0x1415fbbb0 <.text+0x15fabb0>
1415fbc0a: b1 11                       	mov	cl, 0x11
1415fbc0c: eb a2                       	jmp	0x1415fbbb0 <.text+0x15fabb0>
1415fbc0e: b1 10                       	mov	cl, 0x10
1415fbc10: eb 9e                       	jmp	0x1415fbbb0 <.text+0x15fabb0>
1415fbc12: b1 12                       	mov	cl, 0x12
1415fbc14: eb 9a                       	jmp	0x1415fbbb0 <.text+0x15fabb0>
1415fbc16: b1 0b                       	mov	cl, 0xb
1415fbc18: eb 96                       	jmp	0x1415fbbb0 <.text+0x15fabb0>
1415fbc1a: b1 17                       	mov	cl, 0x17
1415fbc1c: eb 92                       	jmp	0x1415fbbb0 <.text+0x15fabb0>
1415fbc1e: b1 0c                       	mov	cl, 0xc
1415fbc20: eb 8e                       	jmp	0x1415fbbb0 <.text+0x15fabb0>
1415fbc22: 48 83 c4 58                 	add	rsp, 0x58
1415fbc26: 5b                          	pop	rbx
1415fbc27: 5d                          	pop	rbp
1415fbc28: 5f                          	pop	rdi
1415fbc29: 5e                          	pop	rsi
1415fbc2a: 41 5c                       	pop	r12
1415fbc2c: 41 5d                       	pop	r13
1415fbc2e: 41 5e                       	pop	r14
1415fbc30: 41 5f                       	pop	r15
1415fbc32: c3                          	ret
1415fbc33: 4c 8d 05 d6 4c 4a 01        	lea	r8, [rip + 0x14a4cd6]   # 0x142aa0910
1415fbc3a: e8 d9 d8 2d 01              	call	0x1428d9518 <.text+0x28d8518>
1415fbc3f: 48 8d 0d 6a a5 4b 01        	lea	rcx, [rip + 0x14ba56a]  # 0x142ab61b0
1415fbc46: e8 c5 d6 2d 01              	call	0x1428d9310 <.text+0x28d8310>
1415fbc4b: cc                          	int3
