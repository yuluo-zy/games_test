
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408cd430: 56                          	push	rsi
1408cd431: 57                          	push	rdi
1408cd432: 53                          	push	rbx
1408cd433: 48 81 ec a0 00 00 00        	sub	rsp, 0xa0
1408cd43a: 0f 29 b4 24 90 00 00 00     	movaps	xmmword ptr [rsp + 0x90], xmm6
1408cd442: 48 89 ce                    	mov	rsi, rcx
1408cd445: 8b 84 24 e8 00 00 00        	mov	eax, dword ptr [rsp + 0xe8]
1408cd44c: f3 0f 10 b4 24 e0 00 00 00  	movss	xmm6, dword ptr [rsp + 0xe0]
1408cd455: a8 20                       	test	al, 0x20
1408cd457: 75 2b                       	jne	0x1408cd484 <.text+0x8cc484>
1408cd459: 89 44 24 30                 	mov	dword ptr [rsp + 0x30], eax
1408cd45d: f3 0f 11 74 24 28           	movss	dword ptr [rsp + 0x28], xmm6
1408cd463: 48 89 f1                    	mov	rcx, rsi
1408cd466: 45 31 c9                    	xor	r9d, r9d
1408cd469: e8 f2 07 00 00              	call	0x1408cdc60 <.text+0x8ccc60>
1408cd46e: 48 89 f0                    	mov	rax, rsi
1408cd471: 0f 28 b4 24 90 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x90]
1408cd479: 48 81 c4 a0 00 00 00        	add	rsp, 0xa0
1408cd480: 5b                          	pop	rbx
1408cd481: 5f                          	pop	rdi
1408cd482: 5e                          	pop	rsi
1408cd483: c3                          	ret
1408cd484: 4c 89 cf                    	mov	rdi, r9
1408cd487: 89 44 24 30                 	mov	dword ptr [rsp + 0x30], eax
1408cd48b: f3 0f 11 74 24 28           	movss	dword ptr [rsp + 0x28], xmm6
1408cd491: 48 8d 4c 24 50              	lea	rcx, [rsp + 0x50]
1408cd496: 4c 89 c3                    	mov	rbx, r8
1408cd499: 45 31 c9                    	xor	r9d, r9d
1408cd49c: e8 bf 07 00 00              	call	0x1408cdc60 <.text+0x8ccc60>
1408cd4a1: 83 bc 24 84 00 00 00 02     	cmp	dword ptr [rsp + 0x84], 0x2
1408cd4a9: 75 5d                       	jne	0x1408cd508 <.text+0x8cc508>
1408cd4ab: 48 8d 4c 24 50              	lea	rcx, [rsp + 0x50]
1408cd4b0: 48 89 fa                    	mov	rdx, rdi
1408cd4b3: 49 89 d8                    	mov	r8, rbx
1408cd4b6: 0f 28 de                    	movaps	xmm3, xmm6
1408cd4b9: e8 62 c5 11 00              	call	0x1409e9a20 <.text+0x9e8a20>
1408cd4be: f6 44 24 50 01              	test	byte ptr [rsp + 0x50], 0x1
1408cd4c3: 0f 84 a1 00 00 00           	je	0x1408cd56a <.text+0x8cc56a>
1408cd4c9: f3 0f 10 44 24 54           	movss	xmm0, dword ptr [rsp + 0x54]
1408cd4cf: 8b 44 24 60                 	mov	eax, dword ptr [rsp + 0x60]
1408cd4d3: 89 46 20                    	mov	dword ptr [rsi + 0x20], eax
1408cd4d6: 48 8b 44 24 58              	mov	rax, qword ptr [rsp + 0x58]
1408cd4db: 48 89 46 18                 	mov	qword ptr [rsi + 0x18], rax
1408cd4df: c6 06 0b                    	mov	byte ptr [rsi], 0xb
1408cd4e2: 48 b8 00 00 00 00 00 00 80 3f       	movabs	rax, 0x3f80000000000000
1408cd4ec: 48 89 46 24                 	mov	qword ptr [rsi + 0x24], rax
1408cd4f0: c7 46 2c 00 00 00 00        	mov	dword ptr [rsi + 0x2c], 0x0
1408cd4f7: f3 0f 11 46 30              	movss	dword ptr [rsi + 0x30], xmm0
1408cd4fc: c7 46 34 00 00 00 00        	mov	dword ptr [rsi + 0x34], 0x0
1408cd503: e9 66 ff ff ff              	jmp	0x1408cd46e <.text+0x8cc46e>
1408cd508: f3 0f 10 b4 24 80 00 00 00  	movss	xmm6, dword ptr [rsp + 0x80]
1408cd511: 48 8d 4c 24 3c              	lea	rcx, [rsp + 0x3c]
1408cd516: 48 89 fa                    	mov	rdx, rdi
1408cd519: 49 89 d8                    	mov	r8, rbx
1408cd51c: 0f 28 de                    	movaps	xmm3, xmm6
1408cd51f: e8 fc c4 11 00              	call	0x1409e9a20 <.text+0x9e8a20>
1408cd524: f6 44 24 3c 01              	test	byte ptr [rsp + 0x3c], 0x1
1408cd529: 74 4b                       	je	0x1408cd576 <.text+0x8cc576>
1408cd52b: f3 0f 10 44 24 40           	movss	xmm0, dword ptr [rsp + 0x40]
1408cd531: 8b 44 24 4c                 	mov	eax, dword ptr [rsp + 0x4c]
1408cd535: 89 46 20                    	mov	dword ptr [rsi + 0x20], eax
1408cd538: 48 8b 44 24 44              	mov	rax, qword ptr [rsp + 0x44]
1408cd53d: 48 89 46 18                 	mov	qword ptr [rsi + 0x18], rax
1408cd541: c6 06 0b                    	mov	byte ptr [rsi], 0xb
1408cd544: 48 b8 00 00 00 00 00 00 80 3f       	movabs	rax, 0x3f80000000000000
1408cd54e: 48 89 46 24                 	mov	qword ptr [rsi + 0x24], rax
1408cd552: c7 46 2c 00 00 00 00        	mov	dword ptr [rsi + 0x2c], 0x0
1408cd559: c7 46 34 00 00 00 00        	mov	dword ptr [rsi + 0x34], 0x0
1408cd560: f3 0f 11 46 30              	movss	dword ptr [rsi + 0x30], xmm0
1408cd565: e9 04 ff ff ff              	jmp	0x1408cd46e <.text+0x8cc46e>
1408cd56a: c7 46 34 02 00 00 00        	mov	dword ptr [rsi + 0x34], 0x2
1408cd571: e9 f8 fe ff ff              	jmp	0x1408cd46e <.text+0x8cc46e>
1408cd576: 48 8d 84 24 84 00 00 00     	lea	rax, [rsp + 0x84]
1408cd57e: 0f 10 44 24 50              	movups	xmm0, xmmword ptr [rsp + 0x50]
1408cd583: 0f 10 4c 24 60              	movups	xmm1, xmmword ptr [rsp + 0x60]
1408cd588: 0f 10 54 24 70              	movups	xmm2, xmmword ptr [rsp + 0x70]
1408cd58d: 0f 11 56 20                 	movups	xmmword ptr [rsi + 0x20], xmm2
1408cd591: 0f 11 4e 10                 	movups	xmmword ptr [rsi + 0x10], xmm1
1408cd595: 0f 11 06                    	movups	xmmword ptr [rsi], xmm0
1408cd598: 48 8b 08                    	mov	rcx, qword ptr [rax]
1408cd59b: 48 89 4e 34                 	mov	qword ptr [rsi + 0x34], rcx
1408cd59f: 8b 40 08                    	mov	eax, dword ptr [rax + 0x8]
1408cd5a2: 89 46 3c                    	mov	dword ptr [rsi + 0x3c], eax
1408cd5a5: f3 0f 11 76 30              	movss	dword ptr [rsi + 0x30], xmm6
1408cd5aa: e9 bf fe ff ff              	jmp	0x1408cd46e <.text+0x8cc46e>
