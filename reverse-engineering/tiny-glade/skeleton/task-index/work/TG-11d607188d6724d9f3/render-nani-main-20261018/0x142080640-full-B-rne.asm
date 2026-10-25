
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
142080640: 55                          	push	rbp
142080641: 41 57                       	push	r15
142080643: 41 56                       	push	r14
142080645: 41 54                       	push	r12
142080647: 56                          	push	rsi
142080648: 57                          	push	rdi
142080649: 53                          	push	rbx
14208064a: 48 83 ec 70                 	sub	rsp, 0x70
14208064e: 48 8d 6c 24 70              	lea	rbp, [rsp + 0x70]
142080653: 48 c7 45 f8 fe ff ff ff     	mov	qword ptr [rbp - 0x8], -0x2
14208065b: 48 89 d6                    	mov	rsi, rdx
14208065e: 48 89 cf                    	mov	rdi, rcx
142080661: e8 9a 63 b9 fe              	call	0x140c16a00 <.text+0xc15a00>
142080666: 84 c0                       	test	al, al
142080668: 88 45 f0                    	mov	byte ptr [rbp - 0x10], al
14208066b: 74 5f                       	je	0x1420806cc <.text+0x207f6cc>
14208066d: 4c 8d 35 16 2c f5 00        	lea	r14, [rip + 0xf52c16]   # 0x142fd328a
142080674: bb 40 00 00 00              	mov	ebx, 0x40
142080679: ba 40 00 00 00              	mov	edx, 0x40
14208067e: 4c 89 f1                    	mov	rcx, r14
142080681: e8 ea e7 fe ff              	call	0x14206ee70 <.text+0x206de70>
142080686: 48 83 f8 01                 	cmp	rax, 0x1
14208068a: 0f 85 b1 00 00 00           	jne	0x142080741 <.text+0x207f741>
142080690: 49 89 d1                    	mov	r9, rdx
142080693: 48 85 d2                    	test	rdx, rdx
142080696: 74 3d                       	je	0x1420806d5 <.text+0x207f6d5>
142080698: 49 83 f9 40                 	cmp	r9, 0x40
14208069c: 73 35                       	jae	0x1420806d3 <.text+0x207f6d3>
14208069e: 48 8d 05 e5 2b f5 00        	lea	rax, [rip + 0xf52be5]   # 0x142fd328a
1420806a5: 41 80 3c 01 bf              	cmp	byte ptr [r9 + rax], -0x41
1420806aa: 7f 29                       	jg	0x1420806d5 <.text+0x207f6d5>
1420806ac: 48 8d 05 25 2d f5 00        	lea	rax, [rip + 0xf52d25]   # 0x142fd33d8
1420806b3: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1420806b8: 48 8d 0d cb 2b f5 00        	lea	rcx, [rip + 0xf52bcb]   # 0x142fd328a
1420806bf: ba 40 00 00 00              	mov	edx, 0x40
1420806c4: 45 31 c0                    	xor	r8d, r8d
1420806c7: e8 74 92 85 00              	call	0x1428d9940 <.text+0x28d8940>
1420806cc: 31 c0                       	xor	eax, eax
1420806ce: e9 ba 01 00 00              	jmp	0x14208088d <.text+0x207f88d>
1420806d3: 75 d7                       	jne	0x1420806ac <.text+0x207f6ac>
1420806d5: 4c 8d 35 ae 2b f5 00        	lea	r14, [rip + 0xf52bae]   # 0x142fd328a
1420806dc: 4c 89 f1                    	mov	rcx, r14
1420806df: 4c 89 ca                    	mov	rdx, r9
1420806e2: e8 89 e7 fe ff              	call	0x14206ee70 <.text+0x206de70>
1420806e7: 48 83 f8 01                 	cmp	rax, 0x1
1420806eb: 75 54                       	jne	0x142080741 <.text+0x207f741>
1420806ed: 49 89 d0                    	mov	r8, rdx
1420806f0: 49 83 c0 02                 	add	r8, 0x2
1420806f4: 74 39                       	je	0x14208072f <.text+0x207f72f>
1420806f6: 49 83 f8 40                 	cmp	r8, 0x40
1420806fa: 73 31                       	jae	0x14208072d <.text+0x207f72d>
1420806fc: 48 8d 05 87 2b f5 00        	lea	rax, [rip + 0xf52b87]   # 0x142fd328a
142080703: 41 80 3c 00 bf              	cmp	byte ptr [r8 + rax], -0x41
142080708: 7f 25                       	jg	0x14208072f <.text+0x207f72f>
14208070a: 48 8d 05 df 2c f5 00        	lea	rax, [rip + 0xf52cdf]   # 0x142fd33f0
142080711: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
142080716: 48 8d 0d 6d 2b f5 00        	lea	rcx, [rip + 0xf52b6d]   # 0x142fd328a
14208071d: ba 40 00 00 00              	mov	edx, 0x40
142080722: 41 b9 40 00 00 00           	mov	r9d, 0x40
142080728: e8 13 92 85 00              	call	0x1428d9940 <.text+0x28d8940>
14208072d: 75 db                       	jne	0x14208070a <.text+0x207f70a>
14208072f: bb 3e 00 00 00              	mov	ebx, 0x3e
142080734: 48 29 d3                    	sub	rbx, rdx
142080737: 4c 8d 35 4c 2b f5 00        	lea	r14, [rip + 0xf52b4c]   # 0x142fd328a
14208073e: 4d 01 c6                    	add	r14, r8
142080741: 4c 8d 05 16 2d f5 00        	lea	r8, [rip + 0xf52d16]    # 0x142fd345e
142080748: 4c 8d 3d e1 2c f5 00        	lea	r15, [rip + 0xf52ce1]   # 0x142fd3430
14208074f: 90                          	nop
142080750: 41 0f be 40 ff              	movsx	eax, byte ptr [r8 - 0x1]
142080755: 85 c0                       	test	eax, eax
142080757: 78 17                       	js	0x142080770 <.text+0x207f770>
142080759: 49 ff c8                    	dec	r8
14208075c: 83 f8 5c                    	cmp	eax, 0x5c
14208075f: 75 63                       	jne	0x1420807c4 <.text+0x207f7c4>
142080761: eb 73                       	jmp	0x1420807d6 <.text+0x207f7d6>
142080763: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
142080770: 41 0f b6 48 fe              	movzx	ecx, byte ptr [r8 - 0x2]
142080775: 80 f9 c0                    	cmp	cl, -0x40
142080778: 7d 23                       	jge	0x14208079d <.text+0x207f79d>
14208077a: 41 0f b6 50 fd              	movzx	edx, byte ptr [r8 - 0x3]
14208077f: 80 fa c0                    	cmp	dl, -0x40
142080782: 7d 22                       	jge	0x1420807a6 <.text+0x207f7a6>
142080784: 45 0f b6 48 fc              	movzx	r9d, byte ptr [r8 - 0x4]
142080789: 49 83 c0 fc                 	add	r8, -0x4
14208078d: 41 83 e1 07                 	and	r9d, 0x7
142080791: 41 c1 e1 06                 	shl	r9d, 0x6
142080795: 83 e2 3f                    	and	edx, 0x3f
142080798: 44 09 ca                    	or	edx, r9d
14208079b: eb 10                       	jmp	0x1420807ad <.text+0x207f7ad>
14208079d: 49 83 c0 fe                 	add	r8, -0x2
1420807a1: 83 e1 1f                    	and	ecx, 0x1f
1420807a4: eb 0f                       	jmp	0x1420807b5 <.text+0x207f7b5>
1420807a6: 49 83 c0 fd                 	add	r8, -0x3
1420807aa: 83 e2 0f                    	and	edx, 0xf
1420807ad: c1 e2 06                    	shl	edx, 0x6
1420807b0: 83 e1 3f                    	and	ecx, 0x3f
1420807b3: 09 d1                       	or	ecx, edx
1420807b5: c1 e1 06                    	shl	ecx, 0x6
1420807b8: 24 3f                       	and	al, 0x3f
1420807ba: 0f b6 c0                    	movzx	eax, al
1420807bd: 09 c8                       	or	eax, ecx
1420807bf: 83 f8 5c                    	cmp	eax, 0x5c
1420807c2: 74 12                       	je	0x1420807d6 <.text+0x207f7d6>
1420807c4: 83 f8 2f                    	cmp	eax, 0x2f
1420807c7: 74 0d                       	je	0x1420807d6 <.text+0x207f7d6>
1420807c9: 4d 39 f8                    	cmp	r8, r15
1420807cc: 75 82                       	jne	0x142080750 <.text+0x207f750>
1420807ce: 41 bc 2e 00 00 00           	mov	r12d, 0x2e
1420807d4: eb 4d                       	jmp	0x142080823 <.text+0x207f823>
