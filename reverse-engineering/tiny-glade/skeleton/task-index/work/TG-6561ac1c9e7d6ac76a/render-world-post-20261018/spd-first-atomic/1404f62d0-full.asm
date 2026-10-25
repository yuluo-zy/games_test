
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1404f62d0: 55                          	push	rbp
1404f62d1: 41 57                       	push	r15
1404f62d3: 41 56                       	push	r14
1404f62d5: 41 55                       	push	r13
1404f62d7: 41 54                       	push	r12
1404f62d9: 56                          	push	rsi
1404f62da: 57                          	push	rdi
1404f62db: 53                          	push	rbx
1404f62dc: 48 81 ec b8 01 00 00        	sub	rsp, 0x1b8
1404f62e3: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
1404f62eb: 48 c7 85 30 01 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x130], -0x2
1404f62f6: 48 89 d7                    	mov	rdi, rdx
1404f62f9: 48 89 ce                    	mov	rsi, rcx
1404f62fc: 4c 89 c1                    	mov	rcx, r8
1404f62ff: 4c 89 ca                    	mov	rdx, r9
1404f6302: e8 39 93 71 00              	call	0x140c0f640 <.text+0xc0e640>
1404f6307: 48 89 85 10 01 00 00        	mov	qword ptr [rbp + 0x110], rax
1404f630e: 48 83 bf c8 00 00 00 00     	cmp	qword ptr [rdi + 0xc8], 0x0
1404f6316: 0f 84 2a 01 00 00           	je	0x1404f6446 <.text+0x4f5446>
1404f631c: 48 8d 8f d0 00 00 00        	lea	rcx, [rdi + 0xd0]
1404f6323: 48 8d 9d 10 01 00 00        	lea	rbx, [rbp + 0x110]
1404f632a: 48 89 da                    	mov	rdx, rbx
1404f632d: e8 0e b3 0b 00              	call	0x1405b1640 <.text+0x5b0640>
1404f6332: 4c 8b bf b0 00 00 00        	mov	r15, qword ptr [rdi + 0xb0]
1404f6339: 48 8b 8f b8 00 00 00        	mov	rcx, qword ptr [rdi + 0xb8]
1404f6340: 48 89 ca                    	mov	rdx, rcx
1404f6343: 48 21 c2                    	and	rdx, rax
1404f6346: 48 c1 e8 39                 	shr	rax, 0x39
1404f634a: 66 0f 6e c0                 	movd	xmm0, eax
1404f634e: 66 0f 60 c0                 	punpcklbw	xmm0, xmm0      # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1404f6352: f2 0f 70 c0 00              	pshuflw	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1404f6357: 66 0f 70 c0 00              	pshufd	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0]
1404f635c: 49 8d 47 98                 	lea	rax, [r15 - 0x68]
1404f6360: 45 31 c0                    	xor	r8d, r8d
1404f6363: 4c 8b b5 10 01 00 00        	mov	r14, qword ptr [rbp + 0x110]
1404f636a: 66 0f 76 c9                 	pcmpeqd	xmm1, xmm1
1404f636e: f3 41 0f 6f 14 17           	movdqu	xmm2, xmmword ptr [r15 + rdx]
1404f6374: 66 0f 6f da                 	movdqa	xmm3, xmm2
1404f6378: 66 0f 74 d8                 	pcmpeqb	xmm3, xmm0
1404f637c: 66 44 0f d7 cb              	pmovmskb	r9d, xmm3
1404f6381: 45 85 c9                    	test	r9d, r9d
1404f6384: 74 2a                       	je	0x1404f63b0 <.text+0x4f53b0>
1404f6386: f3 45 0f bc d1              	tzcnt	r10d, r9d
1404f638b: 49 01 d2                    	add	r10, rdx
1404f638e: 49 21 ca                    	and	r10, rcx
1404f6391: 49 f7 da                    	neg	r10
1404f6394: 4d 6b d2 68                 	imul	r10, r10, 0x68
1404f6398: 4e 3b 34 10                 	cmp	r14, qword ptr [rax + r10]
1404f639c: 74 34                       	je	0x1404f63d2 <.text+0x4f53d2>
1404f639e: 45 8d 51 ff                 	lea	r10d, [r9 - 0x1]
1404f63a2: 66 45 21 ca                 	and	r10w, r9w
1404f63a6: 45 89 d1                    	mov	r9d, r10d
1404f63a9: 75 db                       	jne	0x1404f6386 <.text+0x4f5386>
1404f63ab: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1404f63b0: 66 0f 74 d1                 	pcmpeqb	xmm2, xmm1
1404f63b4: 66 44 0f d7 ca              	pmovmskb	r9d, xmm2
1404f63b9: 45 85 c9                    	test	r9d, r9d
1404f63bc: 0f 85 87 00 00 00           	jne	0x1404f6449 <.text+0x4f5449>
1404f63c2: 4c 01 c2                    	add	rdx, r8
1404f63c5: 48 83 c2 10                 	add	rdx, 0x10
1404f63c9: 49 83 c0 10                 	add	r8, 0x10
1404f63cd: 48 21 ca                    	and	rdx, rcx
1404f63d0: eb 9c                       	jmp	0x1404f636e <.text+0x4f536e>
1404f63d2: 43 83 7c 17 a0 02           	cmp	dword ptr [r15 + r10 - 0x60], 0x2
1404f63d8: 0f 85 97 02 00 00           	jne	0x1404f6675 <.text+0x4f5675>
1404f63de: 4d 01 d7                    	add	r15, r10
1404f63e1: 41 80 7f fe 07              	cmp	byte ptr [r15 - 0x2], 0x7
1404f63e6: 0f 85 b9 02 00 00           	jne	0x1404f66a5 <.text+0x4f56a5>
1404f63ec: 49 8d 57 c8                 	lea	rdx, [r15 - 0x38]
1404f63f0: 45 0f b6 4f a4              	movzx	r9d, byte ptr [r15 - 0x5c]
1404f63f5: 48 8d 4d 70                 	lea	rcx, [rbp + 0x70]
1404f63f9: 49 89 f8                    	mov	r8, rdi
1404f63fc: e8 bf eb 6d 00              	call	0x140bd4fc0 <.text+0xbd3fc0>
1404f6401: 8b 85 80 00 00 00           	mov	eax, dword ptr [rbp + 0x80]
1404f6407: 0f 10 45 70                 	movups	xmm0, xmmword ptr [rbp + 0x70]
1404f640b: 0f 11 45 48                 	movups	xmmword ptr [rbp + 0x48], xmm0
1404f640f: 41 c7 47 a0 01 00 00 00     	mov	dword ptr [r15 - 0x60], 0x1
1404f6417: 0f 10 45 44                 	movups	xmm0, xmmword ptr [rbp + 0x44]
1404f641b: 41 0f 11 47 a4              	movups	xmmword ptr [r15 - 0x5c], xmm0
1404f6420: 8b 4d 54                    	mov	ecx, dword ptr [rbp + 0x54]
1404f6423: 41 89 4f b4                 	mov	dword ptr [r15 - 0x4c], ecx
1404f6427: 41 89 47 b8                 	mov	dword ptr [r15 - 0x48], eax
1404f642b: 48 8b 85 80 00 00 00        	mov	rax, qword ptr [rbp + 0x80]
1404f6432: 48 89 46 10                 	mov	qword ptr [rsi + 0x10], rax
1404f6436: 0f 10 45 70                 	movups	xmm0, xmmword ptr [rbp + 0x70]
1404f643a: 0f 11 06                    	movups	xmmword ptr [rsi], xmm0
1404f643d: c6 46 18 01                 	mov	byte ptr [rsi + 0x18], 0x1
1404f6441: e9 18 02 00 00              	jmp	0x1404f665e <.text+0x4f565e>
1404f6446: 49 89 c6                    	mov	r14, rax
1404f6449: 4c 8b bd a0 01 00 00        	mov	r15, qword ptr [rbp + 0x1a0]
1404f6450: 48 8d 9f b0 00 00 00        	lea	rbx, [rdi + 0xb0]
1404f6457: 49 8b 0f                    	mov	rcx, qword ptr [r15]
1404f645a: 48 85 c9                    	test	rcx, rcx
1404f645d: 0f 88 bb 02 00 00           	js	0x1404f671e <.text+0x4f571e>
1404f6463: 4c 8b af a8 00 00 00        	mov	r13, qword ptr [rdi + 0xa8]
1404f646a: 4d 8b 66 f8                 	mov	r12, qword ptr [r14 - 0x8]
1404f646e: 74 35                       	je	0x1404f64a5 <.text+0x4f54a5>
1404f6470: 0f b6 05 92 e7 13 03        	movzx	eax, byte ptr [rip + 0x313e792] # 0x143634c09
1404f6477: b8 01 00 00 00              	mov	eax, 0x1
1404f647c: 48 89 85 20 01 00 00        	mov	qword ptr [rbp + 0x120], rax
1404f6483: ba 01 00 00 00              	mov	edx, 0x1
1404f6488: 49 89 cf                    	mov	r15, rcx
1404f648b: e8 b0 d7 11 00              	call	0x140613c40 <.text+0x612c40>
1404f6490: 48 85 c0                    	test	rax, rax
1404f6493: 0f 84 93 02 00 00           	je	0x1404f672c <.text+0x4f572c>
1404f6499: 4c 89 f9                    	mov	rcx, r15
1404f649c: 4c 8b bd a0 01 00 00        	mov	r15, qword ptr [rbp + 0x1a0]
1404f64a3: eb 05                       	jmp	0x1404f64aa <.text+0x4f54aa>
1404f64a5: b8 01 00 00 00              	mov	eax, 0x1
1404f64aa: f3 41 0f 6f 07              	movdqu	xmm0, xmmword ptr [r15]
1404f64af: 66 0f 7f 45 c0              	movdqa	xmmword ptr [rbp - 0x40], xmm0
1404f64b4: 48 89 8d 20 01 00 00        	mov	qword ptr [rbp + 0x120], rcx
1404f64bb: 48 89 4c 24 30              	mov	qword ptr [rsp + 0x30], rcx
1404f64c0: 48 89 85 18 01 00 00        	mov	qword ptr [rbp + 0x118], rax
1404f64c7: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
1404f64cc: 48 8d 45 c0                 	lea	rax, [rbp - 0x40]
1404f64d0: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1404f64d5: 48 8d 8d 88 00 00 00        	lea	rcx, [rbp + 0x88]
1404f64dc: 4c 89 ea                    	mov	rdx, r13
1404f64df: 4d 89 f0                    	mov	r8, r14
1404f64e2: 4d 89 e1                    	mov	r9, r12
1404f64e5: e8 c6 2e 0c 00              	call	0x1405b93b0 <.text+0x5b83b0>
1404f64ea: 48 8d 4d d0                 	lea	rcx, [rbp - 0x30]
1404f64ee: 48 8d 95 88 00 00 00        	lea	rdx, [rbp + 0x88]
1404f64f5: 4d 89 f8                    	mov	r8, r15
1404f64f8: e8 63 b4 e1 ff              	call	0x140311960 <.text+0x310960>
1404f64fd: 0f b6 45 f6                 	movzx	eax, byte ptr [rbp - 0xa]
1404f6501: 48 8b 4d d0                 	mov	rcx, qword ptr [rbp - 0x30]
1404f6505: 3c 05                       	cmp	al, 0x5
1404f6507: 75 2e                       	jne	0x1404f6537 <.text+0x4f5537>
1404f6509: 48 89 0e                    	mov	qword ptr [rsi], rcx
1404f650c: c6 46 18 02                 	mov	byte ptr [rsi + 0x18], 0x2
1404f6510: 48 8b 95 20 01 00 00        	mov	rdx, qword ptr [rbp + 0x120]
1404f6517: 48 85 d2                    	test	rdx, rdx
1404f651a: 0f 84 3e 01 00 00           	je	0x1404f665e <.text+0x4f565e>
1404f6520: 41 b8 01 00 00 00           	mov	r8d, 0x1
1404f6526: 48 8b 8d 18 01 00 00        	mov	rcx, qword ptr [rbp + 0x118]
1404f652d: e8 ee d6 11 00              	call	0x140613c20 <.text+0x612c20>
1404f6532: e9 27 01 00 00              	jmp	0x1404f665e <.text+0x4f565e>
1404f6537: 0f 10 45 d8                 	movups	xmm0, xmmword ptr [rbp - 0x28]
1404f653b: f3 0f 6f 4d e6              	movdqu	xmm1, xmmword ptr [rbp - 0x1a]
1404f6540: f3 0f 7f 8d fe 00 00 00     	movdqu	xmmword ptr [rbp + 0xfe], xmm1
1404f6548: 0f 11 85 f0 00 00 00        	movups	xmmword ptr [rbp + 0xf0], xmm0
1404f654f: 0f b6 55 f7                 	movzx	edx, byte ptr [rbp - 0x9]
1404f6553: 48 89 8d e8 00 00 00        	mov	qword ptr [rbp + 0xe8], rcx
1404f655a: 88 85 0e 01 00 00           	mov	byte ptr [rbp + 0x10e], al
1404f6560: 88 95 0f 01 00 00           	mov	byte ptr [rbp + 0x10f], dl
1404f6566: 48 8b 95 20 01 00 00        	mov	rdx, qword ptr [rbp + 0x120]
1404f656d: 48 85 d2                    	test	rdx, rdx
1404f6570: 74 12                       	je	0x1404f6584 <.text+0x4f5584>
1404f6572: 41 b8 01 00 00 00           	mov	r8d, 0x1
1404f6578: 48 8b 8d 18 01 00 00        	mov	rcx, qword ptr [rbp + 0x118]
1404f657f: e8 9c d6 11 00              	call	0x140613c20 <.text+0x612c20>
1404f6584: c6 85 2f 01 00 00 01        	mov	byte ptr [rbp + 0x12f], 0x1
1404f658b: 48 8d 4d 58                 	lea	rcx, [rbp + 0x58]
1404f658f: 48 8d 95 e8 00 00 00        	lea	rdx, [rbp + 0xe8]
1404f6596: 49 89 f8                    	mov	r8, rdi
1404f6599: 45 31 c9                    	xor	r9d, r9d
1404f659c: e8 1f ea 6d 00              	call	0x140bd4fc0 <.text+0xbd3fc0>
1404f65a1: 48 8b 85 08 01 00 00        	mov	rax, qword ptr [rbp + 0x108]
1404f65a8: 48 89 85 d0 00 00 00        	mov	qword ptr [rbp + 0xd0], rax
1404f65af: 0f 10 85 e8 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xe8]
1404f65b6: f3 0f 6f 8d f8 00 00 00     	movdqu	xmm1, xmmword ptr [rbp + 0xf8]
1404f65be: f3 0f 7f 8d c0 00 00 00     	movdqu	xmmword ptr [rbp + 0xc0], xmm1
1404f65c6: 0f 11 85 b0 00 00 00        	movups	xmmword ptr [rbp + 0xb0], xmm0
1404f65cd: 8b 45 68                    	mov	eax, dword ptr [rbp + 0x68]
1404f65d0: 0f 10 45 58                 	movups	xmm0, xmmword ptr [rbp + 0x58]
1404f65d4: 0f 11 45 34                 	movups	xmmword ptr [rbp + 0x34], xmm0
1404f65d8: c6 85 e6 00 00 00 07        	mov	byte ptr [rbp + 0xe6], 0x7
1404f65df: c7 85 88 00 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x88], 0x1
1404f65e9: 0f 10 45 30                 	movups	xmm0, xmmword ptr [rbp + 0x30]
1404f65ed: 0f 11 85 8c 00 00 00        	movups	xmmword ptr [rbp + 0x8c], xmm0
1404f65f4: 8b 4d 40                    	mov	ecx, dword ptr [rbp + 0x40]
1404f65f7: 89 8d 9c 00 00 00           	mov	dword ptr [rbp + 0x9c], ecx
1404f65fd: 89 85 a0 00 00 00           	mov	dword ptr [rbp + 0xa0], eax
1404f6603: 4c 8b 85 10 01 00 00        	mov	r8, qword ptr [rbp + 0x110]
1404f660a: c6 85 2f 01 00 00 00        	mov	byte ptr [rbp + 0x12f], 0x0
1404f6611: 48 8d 4d d0                 	lea	rcx, [rbp - 0x30]
1404f6615: 4c 8d 8d 88 00 00 00        	lea	r9, [rbp + 0x88]
1404f661c: 48 89 da                    	mov	rdx, rbx
1404f661f: e8 2c 29 dd ff              	call	0x1402c8f50 <.text+0x2c7f50>
1404f6624: 83 7d d0 03                 	cmp	dword ptr [rbp - 0x30], 0x3
1404f6628: 74 1f                       	je	0x1404f6649 <.text+0x4f5649>
1404f662a: 80 7d 2e 07                 	cmp	byte ptr [rbp + 0x2e], 0x7
1404f662e: 74 09                       	je	0x1404f6639 <.text+0x4f5639>
1404f6630: 48 8d 4d f8                 	lea	rcx, [rbp - 0x8]
1404f6634: e8 47 55 b4 ff              	call	0x14003bb80 <.text+0x3ab80>
1404f6639: c6 85 2f 01 00 00 00        	mov	byte ptr [rbp + 0x12f], 0x0
1404f6640: 48 8d 4d f8                 	lea	rcx, [rbp - 0x8]
1404f6644: e8 67 be 6e 00              	call	0x140be24b0 <.text+0xbe14b0>
1404f6649: 48 8b 45 68                 	mov	rax, qword ptr [rbp + 0x68]
1404f664d: 48 89 46 10                 	mov	qword ptr [rsi + 0x10], rax
1404f6651: f3 0f 6f 45 58              	movdqu	xmm0, xmmword ptr [rbp + 0x58]
1404f6656: f3 0f 7f 06                 	movdqu	xmmword ptr [rsi], xmm0
1404f665a: c6 46 18 00                 	mov	byte ptr [rsi + 0x18], 0x0
1404f665e: 48 89 f0                    	mov	rax, rsi
1404f6661: 48 81 c4 b8 01 00 00        	add	rsp, 0x1b8
1404f6668: 5b                          	pop	rbx
1404f6669: 5f                          	pop	rdi
1404f666a: 5e                          	pop	rsi
1404f666b: 41 5c                       	pop	r12
1404f666d: 41 5d                       	pop	r13
1404f666f: 41 5e                       	pop	r14
1404f6671: 41 5f                       	pop	r15
1404f6673: 5d                          	pop	rbp
1404f6674: c3                          	ret
1404f6675: 48 89 9d e8 00 00 00        	mov	qword ptr [rbp + 0xe8], rbx
1404f667c: 48 8d 05 ed 90 71 00        	lea	rax, [rip + 0x7190ed]   # 0x140c0f770 <.text+0xc0e770>
1404f6683: 48 89 85 f0 00 00 00        	mov	qword ptr [rbp + 0xf0], rax
1404f668a: 48 8d 05 7f 67 4f 02        	lea	rax, [rip + 0x24f677f]  # 0x1429ece10
1404f6691: 48 89 85 88 00 00 00        	mov	qword ptr [rbp + 0x88], rax
1404f6698: 48 c7 85 90 00 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x90], 0x1
1404f66a3: eb 2e                       	jmp	0x1404f66d3 <.text+0x4f56d3>
1404f66a5: 48 89 9d e8 00 00 00        	mov	qword ptr [rbp + 0xe8], rbx
1404f66ac: 48 8d 05 bd 90 71 00        	lea	rax, [rip + 0x7190bd]   # 0x140c0f770 <.text+0xc0e770>
1404f66b3: 48 89 85 f0 00 00 00        	mov	qword ptr [rbp + 0xf0], rax
1404f66ba: 48 8d 05 07 67 4f 02        	lea	rax, [rip + 0x24f6707]  # 0x1429ecdc8
1404f66c1: 48 89 85 88 00 00 00        	mov	qword ptr [rbp + 0x88], rax
1404f66c8: 48 c7 85 90 00 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x90], 0x2
1404f66d3: 48 8d 85 e8 00 00 00        	lea	rax, [rbp + 0xe8]
1404f66da: 48 89 85 98 00 00 00        	mov	qword ptr [rbp + 0x98], rax
1404f66e1: 48 c7 85 a0 00 00 00 01 00 00 00    	mov	qword ptr [rbp + 0xa0], 0x1
1404f66ec: 48 c7 85 a8 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xa8], 0x0
1404f66f7: 48 8d 7d d0                 	lea	rdi, [rbp - 0x30]
1404f66fb: 48 8d 95 88 00 00 00        	lea	rdx, [rbp + 0x88]
1404f6702: 48 89 f9                    	mov	rcx, rdi
1404f6705: e8 86 38 88 00              	call	0x140d79f90 <.text+0xd78f90>
1404f670a: 48 89 f9                    	mov	rcx, rdi
1404f670d: e8 0e c4 3a 02              	call	0x1428a2b20 <.text+0x28a1b20>
1404f6712: 48 89 06                    	mov	qword ptr [rsi], rax
1404f6715: c6 46 18 02                 	mov	byte ptr [rsi + 0x18], 0x2
1404f6719: e9 40 ff ff ff              	jmp	0x1404f665e <.text+0x4f565e>
1404f671e: 49 89 cf                    	mov	r15, rcx
1404f6721: 48 c7 85 20 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x120], 0x0
1404f672c: 4c 8d 05 ed 66 4f 02        	lea	r8, [rip + 0x24f66ed]   # 0x1429ece20
1404f6733: 48 8b 8d 20 01 00 00        	mov	rcx, qword ptr [rbp + 0x120]
1404f673a: 4c 89 fa                    	mov	rdx, r15
1404f673d: e8 a1 28 3e 02              	call	0x1428d8fe3 <.text+0x28d7fe3>
1404f6742: cc                          	int3
1404f6743: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
1404f6750: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1404f6755: 55                          	push	rbp
1404f6756: 41 57                       	push	r15
1404f6758: 41 56                       	push	r14
1404f675a: 41 55                       	push	r13
1404f675c: 41 54                       	push	r12
1404f675e: 56                          	push	rsi
1404f675f: 57                          	push	rdi
1404f6760: 53                          	push	rbx
1404f6761: 48 83 ec 38                 	sub	rsp, 0x38
1404f6765: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1404f676c: 48 83 bd 20 01 00 00 00     	cmp	qword ptr [rbp + 0x120], 0x0
1404f6774: 74 19                       	je	0x1404f678f <.text+0x4f578f>
1404f6776: 41 b8 01 00 00 00           	mov	r8d, 0x1
1404f677c: 48 8b 8d 18 01 00 00        	mov	rcx, qword ptr [rbp + 0x118]
1404f6783: 48 8b 95 20 01 00 00        	mov	rdx, qword ptr [rbp + 0x120]
1404f678a: e8 91 d4 11 00              	call	0x140613c20 <.text+0x612c20>
1404f678f: 90                          	nop
1404f6790: 48 83 c4 38                 	add	rsp, 0x38
1404f6794: 5b                          	pop	rbx
1404f6795: 5f                          	pop	rdi
1404f6796: 5e                          	pop	rsi
1404f6797: 41 5c                       	pop	r12
1404f6799: 41 5d                       	pop	r13
1404f679b: 41 5e                       	pop	r14
1404f679d: 41 5f                       	pop	r15
1404f679f: 5d                          	pop	rbp
1404f67a0: c3                          	ret
1404f67a1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
1404f67b0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1404f67b5: 55                          	push	rbp
1404f67b6: 41 57                       	push	r15
1404f67b8: 41 56                       	push	r14
1404f67ba: 41 55                       	push	r13
1404f67bc: 41 54                       	push	r12
1404f67be: 56                          	push	rsi
1404f67bf: 57                          	push	rdi
1404f67c0: 53                          	push	rbx
1404f67c1: 48 83 ec 38                 	sub	rsp, 0x38
1404f67c5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1404f67cc: 80 bd 2f 01 00 00 00        	cmp	byte ptr [rbp + 0x12f], 0x0
1404f67d3: 74 0c                       	je	0x1404f67e1 <.text+0x4f57e1>
1404f67d5: 48 8d 8d e8 00 00 00        	lea	rcx, [rbp + 0xe8]
1404f67dc: e8 cf bc 6e 00              	call	0x140be24b0 <.text+0xbe14b0>
1404f67e1: 90                          	nop
1404f67e2: 48 83 c4 38                 	add	rsp, 0x38
1404f67e6: 5b                          	pop	rbx
1404f67e7: 5f                          	pop	rdi
1404f67e8: 5e                          	pop	rsi
1404f67e9: 41 5c                       	pop	r12
1404f67eb: 41 5d                       	pop	r13
1404f67ed: 41 5e                       	pop	r14
1404f67ef: 41 5f                       	pop	r15
1404f67f1: 5d                          	pop	rbp
1404f67f2: c3                          	ret
1404f67f3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
1404f6800: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1404f6805: 55                          	push	rbp
1404f6806: 41 57                       	push	r15
1404f6808: 41 56                       	push	r14
1404f680a: 41 55                       	push	r13
1404f680c: 41 54                       	push	r12
1404f680e: 56                          	push	rsi
1404f680f: 57                          	push	rdi
1404f6810: 53                          	push	rbx
1404f6811: 48 83 ec 38                 	sub	rsp, 0x38
1404f6815: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1404f681c: c6 85 2f 01 00 00 00        	mov	byte ptr [rbp + 0x12f], 0x0
1404f6823: 48 8d 4d f8                 	lea	rcx, [rbp - 0x8]
1404f6827: e8 84 bc 6e 00              	call	0x140be24b0 <.text+0xbe14b0>
1404f682c: c6 85 2f 01 00 00 00        	mov	byte ptr [rbp + 0x12f], 0x0
1404f6833: 48 83 c4 38                 	add	rsp, 0x38
1404f6837: 5b                          	pop	rbx
1404f6838: 5f                          	pop	rdi
1404f6839: 5e                          	pop	rsi
1404f683a: 41 5c                       	pop	r12
1404f683c: 41 5d                       	pop	r13
1404f683e: 41 5e                       	pop	r14
1404f6840: 41 5f                       	pop	r15
1404f6842: 5d                          	pop	rbp
1404f6843: c3                          	ret
