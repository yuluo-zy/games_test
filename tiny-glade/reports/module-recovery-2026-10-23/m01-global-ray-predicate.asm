
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408cab80: 56                          	push	rsi
1408cab81: 57                          	push	rdi
1408cab82: 48 83 ec 28                 	sub	rsp, 0x28
1408cab86: 48 89 ce                    	mov	rsi, rcx
1408cab89: 48 8b 39                    	mov	rdi, qword ptr [rcx]
1408cab8c: 48 8b 49 08                 	mov	rcx, qword ptr [rcx + 0x8]
1408cab90: e8 ab 8d 2c 00              	call	0x140b93940 <.text+0xb92940>
1408cab95: 48 85 c0                    	test	rax, rax
1408cab98: 74 72                       	je	0x1408cac0c <.text+0x8c9c0c>
1408cab9a: 48 8b 00                    	mov	rax, qword ptr [rax]
1408cab9d: 89 c1                       	mov	ecx, eax
1408cab9f: 48 39 4f 10                 	cmp	qword ptr [rdi + 0x10], rcx
1408caba3: 76 4f                       	jbe	0x1408cabf4 <.text+0x8c9bf4>
1408caba5: 48 8b 57 08                 	mov	rdx, qword ptr [rdi + 0x8]
1408caba9: 48 85 d2                    	test	rdx, rdx
1408cabac: 74 46                       	je	0x1408cabf4 <.text+0x8c9bf4>
1408cabae: 48 c1 e8 20                 	shr	rax, 0x20
1408cabb2: 83 c8 01                    	or	eax, 0x1
1408cabb5: 48 8d 0c 89                 	lea	rcx, [rcx + 4*rcx]
1408cabb9: 39 44 ca 20                 	cmp	dword ptr [rdx + 8*rcx + 0x20], eax
1408cabbd: 75 35                       	jne	0x1408cabf4 <.text+0x8c9bf4>
1408cabbf: 48 8d 0c ca                 	lea	rcx, [rdx + 8*rcx]
1408cabc3: 0f b6 11                    	movzx	edx, byte ptr [rcx]
1408cabc6: b0 01                       	mov	al, 0x1
1408cabc8: 83 fa 08                    	cmp	edx, 0x8
1408cabcb: 77 20                       	ja	0x1408cabed <.text+0x8c9bed>
1408cabcd: 41 b8 05 01 00 00           	mov	r8d, 0x105
1408cabd3: 41 0f a3 d0                 	bt	r8d, edx
1408cabd7: 73 14                       	jae	0x1408cabed <.text+0x8c9bed>
1408cabd9: 48 8b 56 10                 	mov	rdx, qword ptr [rsi + 0x10]
1408cabdd: 83 3a 01                    	cmp	dword ptr [rdx], 0x1
1408cabe0: 75 0b                       	jne	0x1408cabed <.text+0x8c9bed>
1408cabe2: 48 8b 41 08                 	mov	rax, qword ptr [rcx + 0x8]
1408cabe6: 48 39 42 08                 	cmp	qword ptr [rdx + 0x8], rax
1408cabea: 0f 95 c0                    	setne	al
1408cabed: 48 83 c4 28                 	add	rsp, 0x28
1408cabf1: 5f                          	pop	rdi
1408cabf2: 5e                          	pop	rsi
1408cabf3: c3                          	ret
1408cabf4: 48 8d 0d e5 2c 3a 02        	lea	rcx, [rip + 0x23a2ce5]  # 0x142c6d8e0
1408cabfb: 4c 8d 05 76 90 1d 02        	lea	r8, [rip + 0x21d9076]   # 0x142aa3c78
1408cac02: ba 18 00 00 00              	mov	edx, 0x18
1408cac07: e8 19 db 00 02              	call	0x1428d8725 <.text+0x28d7725>
1408cac0c: 48 8d 0d 4d 90 1d 02        	lea	rcx, [rip + 0x21d904d]  # 0x142aa3c60
1408cac13: e8 f8 e6 00 02              	call	0x1428d9310 <.text+0x28d8310>
