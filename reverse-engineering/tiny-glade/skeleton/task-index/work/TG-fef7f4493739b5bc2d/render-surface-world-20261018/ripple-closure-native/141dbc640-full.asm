
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141dbc640: 41 57                       	push	r15
141dbc642: 41 56                       	push	r14
141dbc644: 41 54                       	push	r12
141dbc646: 56                          	push	rsi
141dbc647: 57                          	push	rdi
141dbc648: 53                          	push	rbx
141dbc649: 48 83 ec 58                 	sub	rsp, 0x58
141dbc64d: 48 89 ce                    	mov	rsi, rcx
141dbc650: 48 8b 02                    	mov	rax, qword ptr [rdx]
141dbc653: 48 85 c0                    	test	rax, rax
141dbc656: 0f 84 d5 02 00 00           	je	0x141dbc931 <.text+0x1dbb931>
141dbc65c: 48 89 d7                    	mov	rdi, rdx
141dbc65f: 48 ff c8                    	dec	rax
141dbc662: 48 89 02                    	mov	qword ptr [rdx], rax
141dbc665: 48 8d 42 08                 	lea	rax, [rdx + 0x8]
141dbc669: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
141dbc66e: 48 8d 5c 24 44              	lea	rbx, [rsp + 0x44]
141dbc673: 4c 8d 74 24 20              	lea	r14, [rsp + 0x20]
141dbc678: 4c 8d 7c 24 28              	lea	r15, [rsp + 0x28]
141dbc67d: eb 57                       	jmp	0x141dbc6d6 <.text+0x1dbb6d6>
141dbc67f: 90                          	nop
141dbc680: 4c 8b 47 50                 	mov	r8, qword ptr [rdi + 0x50]
141dbc684: 48 8b 57 70                 	mov	rdx, qword ptr [rdi + 0x70]
141dbc688: 48 8b 47 78                 	mov	rax, qword ptr [rdi + 0x78]
141dbc68c: 4d 8d 04 c8                 	lea	r8, [r8 + 8*rcx]
141dbc690: 41 89 ca                    	mov	r10d, ecx
141dbc693: 4f 8d 0c 52                 	lea	r9, [r10 + 2*r10]
141dbc697: 49 c1 e1 04                 	shl	r9, 0x4
141dbc69b: 49 01 d1                    	add	r9, rdx
141dbc69e: 4c 01 d0                    	add	rax, r10
141dbc6a1: 49 8b 10                    	mov	rdx, qword ptr [r8]
141dbc6a4: 48 ff c1                    	inc	rcx
141dbc6a7: 48 89 8f 88 00 00 00        	mov	qword ptr [rdi + 0x88], rcx
141dbc6ae: 4c 89 4c 24 28              	mov	qword ptr [rsp + 0x28], r9
141dbc6b3: 48 89 54 24 30              	mov	qword ptr [rsp + 0x30], rdx
141dbc6b8: 48 89 44 24 38              	mov	qword ptr [rsp + 0x38], rax
141dbc6bd: 48 89 d9                    	mov	rcx, rbx
141dbc6c0: 4c 89 f2                    	mov	rdx, r14
141dbc6c3: 4d 89 f8                    	mov	r8, r15
141dbc6c6: e8 05 3a fd ff              	call	0x141d900d0 <.text+0x1d8f0d0>
141dbc6cb: f6 44 24 44 01              	test	byte ptr [rsp + 0x44], 0x1
141dbc6d0: 0f 85 6d 02 00 00           	jne	0x141dbc943 <.text+0x1dbb943>
141dbc6d6: 48 8b 57 28                 	mov	rdx, qword ptr [rdi + 0x28]
141dbc6da: 48 8b 47 38                 	mov	rax, qword ptr [rdi + 0x38]
141dbc6de: 48 8b 8f 88 00 00 00        	mov	rcx, qword ptr [rdi + 0x88]
141dbc6e5: 4c 8b 8f 80 00 00 00        	mov	r9, qword ptr [rdi + 0x80]
141dbc6ec: 4c 8b 47 48                 	mov	r8, qword ptr [rdi + 0x48]
141dbc6f0: 80 bf 90 00 00 00 00        	cmp	byte ptr [rdi + 0x90], 0x0
141dbc6f7: 0f 84 93 00 00 00           	je	0x141dbc790 <.text+0x1dbb790>
141dbc6fd: 4c 39 c9                    	cmp	rcx, r9
141dbc700: 0f 85 7a ff ff ff           	jne	0x141dbc680 <.text+0x1dbb680>
141dbc706: 48 8b 4f 40                 	mov	rcx, qword ptr [rdi + 0x40]
141dbc70a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
141dbc710: 4c 39 c1                    	cmp	rcx, r8
141dbc713: 0f 84 18 02 00 00           	je	0x141dbc931 <.text+0x1dbb931>
141dbc719: 4c 8d 59 04                 	lea	r11, [rcx + 0x4]
141dbc71d: 4c 89 5f 40                 	mov	qword ptr [rdi + 0x40], r11
141dbc721: 8b 09                       	mov	ecx, dword ptr [rcx]
141dbc723: 4c 8b 4a 08                 	mov	r9, qword ptr [rdx + 0x8]
141dbc727: 4c 8d 14 c9                 	lea	r10, [rcx + 8*rcx]
141dbc72b: 4b 83 7c d1 10 00           	cmp	qword ptr [r9 + 8*r10 + 0x10], 0x0
141dbc731: 4c 89 d9                    	mov	rcx, r11
141dbc734: 74 da                       	je	0x141dbc710 <.text+0x1dbb710>
141dbc736: 4b 8d 0c d1                 	lea	rcx, [r9 + 8*r10]
141dbc73a: 48 8b 90 10 01 00 00        	mov	rdx, qword ptr [rax + 0x110]
141dbc741: 4c 8b 41 40                 	mov	r8, qword ptr [rcx + 0x40]
141dbc745: 49 39 d0                    	cmp	r8, rdx
141dbc748: 0f 86 03 01 00 00           	jbe	0x141dbc851 <.text+0x1dbb851>
141dbc74e: 4c 8b 49 38                 	mov	r9, qword ptr [rcx + 0x38]
141dbc752: 49 8b 14 d1                 	mov	rdx, qword ptr [r9 + 8*rdx]
141dbc756: 48 85 d2                    	test	rdx, rdx
141dbc759: 0f 84 f2 00 00 00           	je	0x141dbc851 <.text+0x1dbb851>
141dbc75f: 4c 8b 49 18                 	mov	r9, qword ptr [rcx + 0x18]
141dbc763: 48 f7 d2                    	not	rdx
141dbc766: 48 8d 14 52                 	lea	rdx, [rdx + 2*rdx]
141dbc76a: 48 c1 e2 04                 	shl	rdx, 0x4
141dbc76e: 49 8b 54 11 10              	mov	rdx, qword ptr [r9 + rdx + 0x10]
141dbc773: 48 8b 80 18 01 00 00        	mov	rax, qword ptr [rax + 0x118]
141dbc77a: 48 89 57 70                 	mov	qword ptr [rdi + 0x70], rdx
141dbc77e: 49 39 c0                    	cmp	r8, rax
141dbc781: 0f 87 dc 00 00 00           	ja	0x141dbc863 <.text+0x1dbb863>
141dbc787: e9 2f 01 00 00              	jmp	0x141dbc8bb <.text+0x1dbb8bb>
141dbc78c: 0f 1f 40 00                 	nop	dword ptr [rax]
141dbc790: 4c 39 c9                    	cmp	rcx, r9
141dbc793: 0f 85 a7 00 00 00           	jne	0x141dbc840 <.text+0x1dbb840>
141dbc799: 4c 8b 4f 30                 	mov	r9, qword ptr [rdi + 0x30]
141dbc79d: 4c 8b 57 40                 	mov	r10, qword ptr [rdi + 0x40]
141dbc7a1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
141dbc7b0: 4d 39 c2                    	cmp	r10, r8
141dbc7b3: 0f 84 78 01 00 00           	je	0x141dbc931 <.text+0x1dbb931>
141dbc7b9: 4d 8d 62 04                 	lea	r12, [r10 + 0x4]
141dbc7bd: 4c 89 67 40                 	mov	qword ptr [rdi + 0x40], r12
141dbc7c1: 45 8b 12                    	mov	r10d, dword ptr [r10]
141dbc7c4: 49 8b 49 08                 	mov	rcx, qword ptr [r9 + 0x8]
141dbc7c8: 4f 8d 1c 92                 	lea	r11, [r10 + 4*r10]
141dbc7cc: 49 c1 e3 05                 	shl	r11, 0x5
141dbc7d0: 4a 83 7c 19 58 00           	cmp	qword ptr [rcx + r11 + 0x58], 0x0
141dbc7d6: 4d 89 e2                    	mov	r10, r12
141dbc7d9: 74 d5                       	je	0x141dbc7b0 <.text+0x1dbb7b0>
141dbc7db: 4c 01 d9                    	add	rcx, r11
141dbc7de: 44 8b 81 94 00 00 00        	mov	r8d, dword ptr [rcx + 0x94]
141dbc7e5: 4c 8b 4a 08                 	mov	r9, qword ptr [rdx + 0x8]
141dbc7e9: 4f 8d 14 c0                 	lea	r10, [r8 + 8*r8]
141dbc7ed: 4f 8d 04 d1                 	lea	r8, [r9 + 8*r10]
141dbc7f1: 48 8b 90 10 01 00 00        	mov	rdx, qword ptr [rax + 0x110]
141dbc7f8: 4f 8b 4c d1 40              	mov	r9, qword ptr [r9 + 8*r10 + 0x40]
141dbc7fd: 49 39 d1                    	cmp	r9, rdx
141dbc800: 0f 86 80 00 00 00           	jbe	0x141dbc886 <.text+0x1dbb886>
141dbc806: 4d 8b 50 38                 	mov	r10, qword ptr [r8 + 0x38]
141dbc80a: 49 8b 14 d2                 	mov	rdx, qword ptr [r10 + 8*rdx]
141dbc80e: 48 85 d2                    	test	rdx, rdx
141dbc811: 74 73                       	je	0x141dbc886 <.text+0x1dbb886>
141dbc813: 4d 8b 50 18                 	mov	r10, qword ptr [r8 + 0x18]
141dbc817: 48 f7 d2                    	not	rdx
141dbc81a: 48 8d 14 52                 	lea	rdx, [rdx + 2*rdx]
141dbc81e: 48 c1 e2 04                 	shl	rdx, 0x4
141dbc822: 49 8b 54 12 10              	mov	rdx, qword ptr [r10 + rdx + 0x10]
141dbc827: 48 8b 80 18 01 00 00        	mov	rax, qword ptr [rax + 0x118]
141dbc82e: 48 89 57 70                 	mov	qword ptr [rdi + 0x70], rdx
141dbc832: 49 39 c1                    	cmp	r9, rax
141dbc835: 77 61                       	ja	0x141dbc898 <.text+0x1dbb898>
141dbc837: e9 b2 00 00 00              	jmp	0x141dbc8ee <.text+0x1dbb8ee>
141dbc83c: 0f 1f 40 00                 	nop	dword ptr [rax]
141dbc840: 4c 8b 4f 60                 	mov	r9, qword ptr [rdi + 0x60]
141dbc844: 48 8b 57 70                 	mov	rdx, qword ptr [rdi + 0x70]
141dbc848: 48 8b 47 78                 	mov	rax, qword ptr [rdi + 0x78]
141dbc84c: e9 cb 00 00 00              	jmp	0x141dbc91c <.text+0x1dbb91c>
141dbc851: 31 d2                       	xor	edx, edx
141dbc853: 48 8b 80 18 01 00 00        	mov	rax, qword ptr [rax + 0x118]
141dbc85a: 48 89 57 70                 	mov	qword ptr [rdi + 0x70], rdx
141dbc85e: 49 39 c0                    	cmp	r8, rax
141dbc861: 76 58                       	jbe	0x141dbc8bb <.text+0x1dbb8bb>
141dbc863: 4c 8b 41 38                 	mov	r8, qword ptr [rcx + 0x38]
141dbc867: 49 8b 04 c0                 	mov	rax, qword ptr [r8 + 8*rax]
141dbc86b: 48 85 c0                    	test	rax, rax
141dbc86e: 74 4b                       	je	0x141dbc8bb <.text+0x1dbb8bb>
141dbc870: 4c 8b 41 18                 	mov	r8, qword ptr [rcx + 0x18]
141dbc874: 48 f7 d0                    	not	rax
141dbc877: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
141dbc87b: 48 c1 e0 04                 	shl	rax, 0x4
141dbc87f: 49 8b 44 00 10              	mov	rax, qword ptr [r8 + rax + 0x10]
141dbc884: eb 37                       	jmp	0x141dbc8bd <.text+0x1dbb8bd>
141dbc886: 31 d2                       	xor	edx, edx
141dbc888: 48 8b 80 18 01 00 00        	mov	rax, qword ptr [rax + 0x118]
141dbc88f: 48 89 57 70                 	mov	qword ptr [rdi + 0x70], rdx
141dbc893: 49 39 c1                    	cmp	r9, rax
141dbc896: 76 56                       	jbe	0x141dbc8ee <.text+0x1dbb8ee>
141dbc898: 4d 8b 48 38                 	mov	r9, qword ptr [r8 + 0x38]
141dbc89c: 49 8b 04 c1                 	mov	rax, qword ptr [r9 + 8*rax]
141dbc8a0: 48 85 c0                    	test	rax, rax
141dbc8a3: 74 49                       	je	0x141dbc8ee <.text+0x1dbb8ee>
141dbc8a5: 4d 8b 40 18                 	mov	r8, qword ptr [r8 + 0x18]
141dbc8a9: 48 f7 d0                    	not	rax
141dbc8ac: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
141dbc8b0: 48 c1 e0 04                 	shl	rax, 0x4
141dbc8b4: 49 8b 44 00 10              	mov	rax, qword ptr [r8 + rax + 0x10]
141dbc8b9: eb 35                       	jmp	0x141dbc8f0 <.text+0x1dbb8f0>
141dbc8bb: 31 c0                       	xor	eax, eax
141dbc8bd: 48 89 47 78                 	mov	qword ptr [rdi + 0x78], rax
141dbc8c1: 4c 8b 41 08                 	mov	r8, qword ptr [rcx + 0x8]
141dbc8c5: 4c 8b 49 10                 	mov	r9, qword ptr [rcx + 0x10]
141dbc8c9: 4c 89 47 50                 	mov	qword ptr [rdi + 0x50], r8
141dbc8cd: 4c 89 4f 58                 	mov	qword ptr [rdi + 0x58], r9
141dbc8d1: 48 8b 49 10                 	mov	rcx, qword ptr [rcx + 0x10]
141dbc8d5: 48 89 8f 80 00 00 00        	mov	qword ptr [rdi + 0x80], rcx
141dbc8dc: 48 c7 87 88 00 00 00 00 00 00 00    	mov	qword ptr [rdi + 0x88], 0x0
141dbc8e7: 31 c9                       	xor	ecx, ecx
141dbc8e9: e9 9e fd ff ff              	jmp	0x141dbc68c <.text+0x1dbb68c>
141dbc8ee: 31 c0                       	xor	eax, eax
141dbc8f0: 48 89 47 78                 	mov	qword ptr [rdi + 0x78], rax
141dbc8f4: 4c 8b 49 50                 	mov	r9, qword ptr [rcx + 0x50]
141dbc8f8: 4c 8b 41 58                 	mov	r8, qword ptr [rcx + 0x58]
141dbc8fc: 4c 89 4f 60                 	mov	qword ptr [rdi + 0x60], r9
141dbc900: 4c 89 47 68                 	mov	qword ptr [rdi + 0x68], r8
141dbc904: 48 8b 49 58                 	mov	rcx, qword ptr [rcx + 0x58]
141dbc908: 48 89 8f 80 00 00 00        	mov	qword ptr [rdi + 0x80], rcx
141dbc90f: 48 c7 87 88 00 00 00 00 00 00 00    	mov	qword ptr [rdi + 0x88], 0x0
141dbc91a: 31 c9                       	xor	ecx, ecx
141dbc91c: 49 89 ca                    	mov	r10, rcx
141dbc91f: 49 c1 e2 04                 	shl	r10, 0x4
141dbc923: 4f 8d 04 11                 	lea	r8, [r9 + r10]
141dbc927: 47 8b 54 11 08              	mov	r10d, dword ptr [r9 + r10 + 0x8]
141dbc92c: e9 62 fd ff ff              	jmp	0x141dbc693 <.text+0x1dbb693>
141dbc931: 31 c0                       	xor	eax, eax
141dbc933: 89 06                       	mov	dword ptr [rsi], eax
141dbc935: 48 83 c4 58                 	add	rsp, 0x58
141dbc939: 5b                          	pop	rbx
141dbc93a: 5f                          	pop	rdi
141dbc93b: 5e                          	pop	rsi
141dbc93c: 41 5c                       	pop	r12
141dbc93e: 41 5e                       	pop	r14
141dbc940: 41 5f                       	pop	r15
141dbc942: c3                          	ret
141dbc943: 0f 10 44 24 48              	movups	xmm0, xmmword ptr [rsp + 0x48]
141dbc948: 0f 11 46 04                 	movups	xmmword ptr [rsi + 0x4], xmm0
141dbc94c: b8 01 00 00 00              	mov	eax, 0x1
141dbc951: eb e0                       	jmp	0x141dbc933 <.text+0x1dbb933>
