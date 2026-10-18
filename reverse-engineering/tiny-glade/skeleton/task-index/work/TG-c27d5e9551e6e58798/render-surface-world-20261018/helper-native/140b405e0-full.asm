
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b405e0: 55                          	push	rbp
140b405e1: 41 57                       	push	r15
140b405e3: 41 56                       	push	r14
140b405e5: 56                          	push	rsi
140b405e6: 57                          	push	rdi
140b405e7: 53                          	push	rbx
140b405e8: 48 81 ec 68 02 00 00        	sub	rsp, 0x268
140b405ef: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
140b405f7: 48 c7 85 e0 01 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x1e0], -0x2
140b40602: 4d 89 c6                    	mov	r14, r8
140b40605: 4d 85 c0                    	test	r8, r8
140b40608: 0f 88 69 02 00 00           	js	0x140b40877 <.text+0xb3f877>
140b4060e: 4c 89 ce                    	mov	rsi, r9
140b40611: 74 2d                       	je	0x140b40640 <.text+0xb3f640>
140b40613: 48 89 d3                    	mov	rbx, rdx
140b40616: 0f b6 05 ec 45 af 02        	movzx	eax, byte ptr [rip + 0x2af45ec] # 0x143634c09
140b4061d: bf 01 00 00 00              	mov	edi, 0x1
140b40622: ba 01 00 00 00              	mov	edx, 0x1
140b40627: 4c 89 f1                    	mov	rcx, r14
140b4062a: e8 e1 35 ad ff              	call	0x140613c10 <.text+0x612c10>
140b4062f: 48 85 c0                    	test	rax, rax
140b40632: 0f 84 41 02 00 00           	je	0x140b40879 <.text+0xb3f879>
140b40638: 49 89 c7                    	mov	r15, rax
140b4063b: 48 89 da                    	mov	rdx, rbx
140b4063e: eb 06                       	jmp	0x140b40646 <.text+0xb3f646>
140b40640: 41 bf 01 00 00 00           	mov	r15d, 0x1
140b40646: 4c 89 bd d8 01 00 00        	mov	qword ptr [rbp + 0x1d8], r15
140b4064d: 4c 89 f9                    	mov	rcx, r15
140b40650: 4d 89 f0                    	mov	r8, r14
140b40653: e8 a8 35 de 01              	call	0x142923c00 <.text+0x2922c00>
140b40658: 4c 89 b5 50 01 00 00        	mov	qword ptr [rbp + 0x150], r14
140b4065f: 4c 89 bd 58 01 00 00        	mov	qword ptr [rbp + 0x158], r15
140b40666: 4c 89 b5 60 01 00 00        	mov	qword ptr [rbp + 0x160], r14
140b4066d: 48 8d 8d 80 00 00 00        	lea	rcx, [rbp + 0x80]
140b40674: 48 8d 95 50 01 00 00        	lea	rdx, [rbp + 0x150]
140b4067b: 4c 89 b5 d0 01 00 00        	mov	qword ptr [rbp + 0x1d0], r14
140b40682: e8 09 b7 23 00              	call	0x140d7bd90 <.text+0xd7ad90>
140b40687: 48 8b 85 60 01 00 00        	mov	rax, qword ptr [rbp + 0x160]
140b4068e: 48 89 85 40 01 00 00        	mov	qword ptr [rbp + 0x140], rax
140b40695: 0f 10 85 50 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x150]
140b4069c: 0f 29 85 30 01 00 00        	movaps	xmmword ptr [rbp + 0x130], xmm0
140b406a3: 0f 10 85 80 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0x80]
140b406aa: 0f 29 85 00 01 00 00        	movaps	xmmword ptr [rbp + 0x100], xmm0
140b406b1: 48 8b 85 90 00 00 00        	mov	rax, qword ptr [rbp + 0x90]
140b406b8: 48 89 85 10 01 00 00        	mov	qword ptr [rbp + 0x110], rax
140b406bf: 48 c7 85 18 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x118], 0x0
140b406ca: 48 c7 85 20 01 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x120], 0x8
140b406d5: 48 c7 85 28 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x128], 0x0
140b406e0: 48 8d 8d 80 00 00 00        	lea	rcx, [rbp + 0x80]
140b406e7: 48 8d 95 00 01 00 00        	lea	rdx, [rbp + 0x100]
140b406ee: e8 bd 7c df ff              	call	0x1409383b0 <.text+0x9373b0>
140b406f3: 48 83 bd 80 00 00 00 00     	cmp	qword ptr [rbp + 0x80], 0x0
140b406fb: 0f 84 8a 01 00 00           	je	0x140b4088b <.text+0xb3f88b>
140b40701: 0f 10 85 f0 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xf0]
140b40708: 0f 29 85 c0 01 00 00        	movaps	xmmword ptr [rbp + 0x1c0], xmm0
140b4070f: 0f 10 85 e0 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xe0]
140b40716: 0f 29 85 b0 01 00 00        	movaps	xmmword ptr [rbp + 0x1b0], xmm0
140b4071d: 0f 10 85 d0 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xd0]
140b40724: 0f 29 85 a0 01 00 00        	movaps	xmmword ptr [rbp + 0x1a0], xmm0
140b4072b: 0f 10 85 c0 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xc0]
140b40732: 0f 29 85 90 01 00 00        	movaps	xmmword ptr [rbp + 0x190], xmm0
140b40739: 0f 10 85 80 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0x80]
140b40740: 0f 10 8d 90 00 00 00        	movups	xmm1, xmmword ptr [rbp + 0x90]
140b40747: 0f 10 95 a0 00 00 00        	movups	xmm2, xmmword ptr [rbp + 0xa0]
140b4074e: 0f 10 9d b0 00 00 00        	movups	xmm3, xmmword ptr [rbp + 0xb0]
140b40755: 0f 29 9d 80 01 00 00        	movaps	xmmword ptr [rbp + 0x180], xmm3
140b4075c: 0f 29 95 70 01 00 00        	movaps	xmmword ptr [rbp + 0x170], xmm2
140b40763: 0f 29 8d 60 01 00 00        	movaps	xmmword ptr [rbp + 0x160], xmm1
140b4076a: 0f 29 85 50 01 00 00        	movaps	xmmword ptr [rbp + 0x150], xmm0
140b40771: 48 8b 85 00 01 00 00        	mov	rax, qword ptr [rbp + 0x100]
140b40778: 0f 10 85 08 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x108]
140b4077f: 0f 10 8d 18 01 00 00        	movups	xmm1, xmmword ptr [rbp + 0x118]
140b40786: 0f 10 95 28 01 00 00        	movups	xmm2, xmmword ptr [rbp + 0x128]
140b4078d: 0f 10 9d 38 01 00 00        	movups	xmm3, xmmword ptr [rbp + 0x138]
140b40794: 0f 29 9d b0 00 00 00        	movaps	xmmword ptr [rbp + 0xb0], xmm3
140b4079b: 0f 29 95 a0 00 00 00        	movaps	xmmword ptr [rbp + 0xa0], xmm2
140b407a2: 0f 29 8d 90 00 00 00        	movaps	xmmword ptr [rbp + 0x90], xmm1
140b407a9: 0f 29 85 80 00 00 00        	movaps	xmmword ptr [rbp + 0x80], xmm0
140b407b0: 48 89 c1                    	mov	rcx, rax
140b407b3: 48 f7 d9                    	neg	rcx
140b407b6: 0f 80 0a 01 00 00           	jo	0x140b408c6 <.text+0xb3f8c6>
140b407bc: 48 8b 8d 50 02 00 00        	mov	rcx, qword ptr [rbp + 0x250]
140b407c3: 48 8b 95 40 02 00 00        	mov	rdx, qword ptr [rbp + 0x240]
140b407ca: 48 89 45 a8                 	mov	qword ptr [rbp - 0x58], rax
140b407ce: 0f 28 85 80 00 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x80]
140b407d5: 0f 28 8d 90 00 00 00        	movaps	xmm1, xmmword ptr [rbp + 0x90]
140b407dc: 0f 28 95 a0 00 00 00        	movaps	xmm2, xmmword ptr [rbp + 0xa0]
140b407e3: 0f 28 9d b0 00 00 00        	movaps	xmm3, xmmword ptr [rbp + 0xb0]
140b407ea: 0f 11 45 b0                 	movups	xmmword ptr [rbp - 0x50], xmm0
140b407ee: 0f 11 4d c0                 	movups	xmmword ptr [rbp - 0x40], xmm1
140b407f2: 0f 11 55 d0                 	movups	xmmword ptr [rbp - 0x30], xmm2
140b407f6: 0f 11 5d e0                 	movups	xmmword ptr [rbp - 0x20], xmm3
140b407fa: 48 89 75 f0                 	mov	qword ptr [rbp - 0x10], rsi
140b407fe: 48 89 55 f8                 	mov	qword ptr [rbp - 0x8], rdx
140b40802: 0f 28 85 50 01 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x150]
140b40809: 0f 28 8d 60 01 00 00        	movaps	xmm1, xmmword ptr [rbp + 0x160]
140b40810: 0f 28 95 70 01 00 00        	movaps	xmm2, xmmword ptr [rbp + 0x170]
140b40817: 0f 28 9d 80 01 00 00        	movaps	xmm3, xmmword ptr [rbp + 0x180]
140b4081e: 0f 11 45 00                 	movups	xmmword ptr [rbp], xmm0
140b40822: 0f 11 4d 10                 	movups	xmmword ptr [rbp + 0x10], xmm1
140b40826: 0f 11 55 20                 	movups	xmmword ptr [rbp + 0x20], xmm2
140b4082a: 0f 11 5d 30                 	movups	xmmword ptr [rbp + 0x30], xmm3
140b4082e: 0f 28 85 90 01 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x190]
140b40835: 0f 11 45 40                 	movups	xmmword ptr [rbp + 0x40], xmm0
140b40839: 0f 28 85 a0 01 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x1a0]
140b40840: 0f 11 45 50                 	movups	xmmword ptr [rbp + 0x50], xmm0
140b40844: 0f 28 85 b0 01 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x1b0]
140b4084b: 0f 11 45 60                 	movups	xmmword ptr [rbp + 0x60], xmm0
140b4084f: 0f 28 85 c0 01 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x1c0]
140b40856: 0f 11 45 70                 	movups	xmmword ptr [rbp + 0x70], xmm0
140b4085a: 48 8d 55 a8                 	lea	rdx, [rbp - 0x58]
140b4085e: 41 b0 01                    	mov	r8b, 0x1
140b40861: e8 8a 66 df ff              	call	0x140936ef0 <.text+0x935ef0>
140b40866: 90                          	nop
140b40867: 48 81 c4 68 02 00 00        	add	rsp, 0x268
140b4086e: 5b                          	pop	rbx
140b4086f: 5f                          	pop	rdi
140b40870: 5e                          	pop	rsi
140b40871: 41 5e                       	pop	r14
140b40873: 41 5f                       	pop	r15
140b40875: 5d                          	pop	rbp
140b40876: c3                          	ret
140b40877: 31 ff                       	xor	edi, edi
140b40879: 4c 8d 05 00 3e f8 01        	lea	r8, [rip + 0x1f83e00]   # 0x142ac4680
140b40880: 48 89 f9                    	mov	rcx, rdi
140b40883: 4c 89 f2                    	mov	rdx, r14
140b40886: e8 58 87 d9 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
140b4088b: 48 8b 85 88 00 00 00        	mov	rax, qword ptr [rbp + 0x88]
140b40892: 48 89 85 50 01 00 00        	mov	qword ptr [rbp + 0x150], rax
140b40899: 48 8d 05 f0 09 f7 01        	lea	rax, [rip + 0x1f709f0]  # 0x142ab1290
140b408a0: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
140b408a5: 48 8d 0d 54 03 f7 01        	lea	rcx, [rip + 0x1f70354]  # 0x142ab0c00
140b408ac: 4c 8d 0d 2d 03 f7 01        	lea	r9, [rip + 0x1f7032d]   # 0x142ab0be0
140b408b3: 4c 8d 85 50 01 00 00        	lea	r8, [rbp + 0x150]
140b408ba: ba 2b 00 00 00              	mov	edx, 0x2b
140b408bf: e8 9c 8e d9 01              	call	0x1428d9760 <.text+0x28d8760>
140b408c4: 0f 0b                       	ud2
140b408c6: 48 8d 0d 7b e6 fa 01        	lea	rcx, [rip + 0x1fae67b]  # 0x142aeef48
140b408cd: e8 3e 8a d9 01              	call	0x1428d9310 <.text+0x28d8310>
140b408d2: cc                          	int3
140b408d3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140b408e0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140b408e5: 55                          	push	rbp
140b408e6: 41 57                       	push	r15
140b408e8: 41 56                       	push	r14
140b408ea: 56                          	push	rsi
140b408eb: 57                          	push	rdi
140b408ec: 53                          	push	rbx
140b408ed: 48 83 ec 28                 	sub	rsp, 0x28
140b408f1: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140b408f8: 48 8b 8d d8 01 00 00        	mov	rcx, qword ptr [rbp + 0x1d8]
140b408ff: 48 8b 85 d0 01 00 00        	mov	rax, qword ptr [rbp + 0x1d0]
140b40906: 48 85 c0                    	test	rax, rax
140b40909: 74 0e                       	je	0x140b40919 <.text+0xb3f919>
140b4090b: 48 89 c2                    	mov	rdx, rax
140b4090e: 41 b8 01 00 00 00           	mov	r8d, 0x1
140b40914: e8 07 33 ad ff              	call	0x140613c20 <.text+0x612c20>
140b40919: 90                          	nop
140b4091a: 48 83 c4 28                 	add	rsp, 0x28
140b4091e: 5b                          	pop	rbx
140b4091f: 5f                          	pop	rdi
140b40920: 5e                          	pop	rsi
140b40921: 41 5e                       	pop	r14
140b40923: 41 5f                       	pop	r15
140b40925: 5d                          	pop	rbp
140b40926: c3                          	ret
140b40927: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
140b40930: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140b40935: 55                          	push	rbp
140b40936: 41 57                       	push	r15
140b40938: 41 56                       	push	r14
140b4093a: 56                          	push	rsi
140b4093b: 57                          	push	rdi
140b4093c: 53                          	push	rbx
140b4093d: 48 83 ec 28                 	sub	rsp, 0x28
140b40941: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140b40948: 48 8d 8d 00 01 00 00        	lea	rcx, [rbp + 0x100]
140b4094f: e8 6c d5 d4 ff              	call	0x14088dec0 <.text+0x88cec0>
140b40954: 90                          	nop
140b40955: 48 83 c4 28                 	add	rsp, 0x28
140b40959: 5b                          	pop	rbx
140b4095a: 5f                          	pop	rdi
140b4095b: 5e                          	pop	rsi
140b4095c: 41 5e                       	pop	r14
140b4095e: 41 5f                       	pop	r15
140b40960: 5d                          	pop	rbp
140b40961: c3                          	ret
140b40962: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
140b40970: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140b40975: 55                          	push	rbp
140b40976: 41 57                       	push	r15
140b40978: 41 56                       	push	r14
140b4097a: 56                          	push	rsi
140b4097b: 57                          	push	rdi
140b4097c: 53                          	push	rbx
140b4097d: 48 83 ec 28                 	sub	rsp, 0x28
140b40981: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140b40988: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
140b4098f: e8 9c c2 20 00              	call	0x140d4cc30 <.text+0xd4bc30>
140b40994: 90                          	nop
140b40995: 48 83 c4 28                 	add	rsp, 0x28
140b40999: 5b                          	pop	rbx
140b4099a: 5f                          	pop	rdi
140b4099b: 5e                          	pop	rsi
140b4099c: 41 5e                       	pop	r14
140b4099e: 41 5f                       	pop	r15
140b409a0: 5d                          	pop	rbp
140b409a1: c3                          	ret
