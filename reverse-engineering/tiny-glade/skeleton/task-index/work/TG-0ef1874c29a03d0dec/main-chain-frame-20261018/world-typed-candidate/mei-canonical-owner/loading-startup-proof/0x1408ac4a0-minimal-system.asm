
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408ac4a0: 41 56                       	push	r14
1408ac4a2: 56                          	push	rsi
1408ac4a3: 57                          	push	rdi
1408ac4a4: 53                          	push	rbx
1408ac4a5: 48 81 ec 88 00 00 00        	sub	rsp, 0x88
1408ac4ac: bb 01 00 00 00              	mov	ebx, 0x1
1408ac4b1: f0                          	lock
1408ac4b2: 0f c1 9a b0 05 00 00        	xadd	dword ptr [rdx + 0x5b0], ebx
1408ac4b9: f6 01 01                    	test	byte ptr [rcx], 0x1
1408ac4bc: 0f 84 4e 01 00 00           	je	0x1408ac610 <.text+0x8ab610>
1408ac4c2: 48 89 ce                    	mov	rsi, rcx
1408ac4c5: 48 8b 41 08                 	mov	rax, qword ptr [rcx + 0x8]
1408ac4c9: 4c 8b 82 18 02 00 00        	mov	r8, qword ptr [rdx + 0x218]
1408ac4d0: 49 39 c0                    	cmp	r8, rax
1408ac4d3: 0f 86 94 00 00 00           	jbe	0x1408ac56d <.text+0x8ab56d>
1408ac4d9: 4c 8b 8a 10 02 00 00        	mov	r9, qword ptr [rdx + 0x210]
1408ac4e0: 49 8b 0c c1                 	mov	rcx, qword ptr [r9 + 8*rax]
1408ac4e4: 48 85 c9                    	test	rcx, rcx
1408ac4e7: 0f 84 80 00 00 00           	je	0x1408ac56d <.text+0x8ab56d>
1408ac4ed: 48 8b 82 e0 01 00 00        	mov	rax, qword ptr [rdx + 0x1e0]
1408ac4f4: 48 f7 d1                    	not	rcx
1408ac4f7: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1408ac4fb: 48 c1 e1 05                 	shl	rcx, 0x5
1408ac4ff: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
1408ac505: 74 66                       	je	0x1408ac56d <.text+0x8ab56d>
1408ac507: 48 8b 56 10                 	mov	rdx, qword ptr [rsi + 0x10]
1408ac50b: 49 39 d0                    	cmp	r8, rdx
1408ac50e: 76 7b                       	jbe	0x1408ac58b <.text+0x8ab58b>
1408ac510: 49 8b 14 d1                 	mov	rdx, qword ptr [r9 + 8*rdx]
1408ac514: 48 85 d2                    	test	rdx, rdx
1408ac517: 74 72                       	je	0x1408ac58b <.text+0x8ab58b>
1408ac519: 48 f7 d2                    	not	rdx
1408ac51c: 48 8d 14 52                 	lea	rdx, [rdx + 2*rdx]
1408ac520: 48 c1 e2 05                 	shl	rdx, 0x5
1408ac524: 48 83 7c 10 20 00           	cmp	qword ptr [rax + rdx + 0x20], 0x0
1408ac52a: 74 5f                       	je	0x1408ac58b <.text+0x8ab58b>
1408ac52c: 48 01 c1                    	add	rcx, rax
1408ac52f: 4c 8b 71 10                 	mov	r14, qword ptr [rcx + 0x10]
1408ac533: 48 01 c2                    	add	rdx, rax
1408ac536: 48 8b 7a 10                 	mov	rdi, qword ptr [rdx + 0x10]
1408ac53a: 49 8b 16                    	mov	rdx, qword ptr [r14]
1408ac53d: 48 83 c2 10                 	add	rdx, 0x10
1408ac541: 48 89 f9                    	mov	rcx, rdi
1408ac544: e8 07 66 a1 00              	call	0x1412c2b50 <.text+0x12c1b50>
1408ac549: 49 8b 16                    	mov	rdx, qword ptr [r14]
1408ac54c: 48 83 c2 10                 	add	rdx, 0x10
1408ac550: 48 89 f9                    	mov	rcx, rdi
1408ac553: e8 98 1f a1 00              	call	0x1412be4f0 <.text+0x12bd4f0>
1408ac558: 89 9e 50 01 00 00           	mov	dword ptr [rsi + 0x150], ebx
1408ac55e: 31 c0                       	xor	eax, eax
1408ac560: 48 81 c4 88 00 00 00        	add	rsp, 0x88
1408ac567: 5b                          	pop	rbx
1408ac568: 5f                          	pop	rdi
1408ac569: 5e                          	pop	rsi
1408ac56a: 41 5e                       	pop	r14
1408ac56c: c3                          	ret
1408ac56d: 48 81 c6 b8 00 00 00        	add	rsi, 0xb8
1408ac574: 48 8d 05 bd fa 1e 02        	lea	rax, [rip + 0x21efabd]  # 0x142a9c038
1408ac57b: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
1408ac580: 48 c7 44 24 30 2c 00 00 00  	mov	qword ptr [rsp + 0x30], 0x2c
1408ac589: eb 1c                       	jmp	0x1408ac5a7 <.text+0x8ab5a7>
1408ac58b: 48 81 c6 b8 00 00 00        	add	rsi, 0xb8
1408ac592: 48 8d 05 ff fb 1e 02        	lea	rax, [rip + 0x21efbff]  # 0x142a9c198
1408ac599: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
1408ac59e: 48 c7 44 24 30 3d 00 00 00  	mov	qword ptr [rsp + 0x30], 0x3d
1408ac5a7: 48 89 74 24 38              	mov	qword ptr [rsp + 0x38], rsi
1408ac5ac: 48 8d 05 1d f5 78 ff        	lea	rax, [rip - 0x870ae3]   # 0x14003bad0 <.text+0x3aad0>
1408ac5b3: 48 89 44 24 40              	mov	qword ptr [rsp + 0x40], rax
1408ac5b8: 48 8d 44 24 28              	lea	rax, [rsp + 0x28]
1408ac5bd: 48 89 44 24 48              	mov	qword ptr [rsp + 0x48], rax
1408ac5c2: 48 8d 05 c7 d8 85 ff        	lea	rax, [rip - 0x7a2739]   # 0x140109e90 <.text+0x108e90>
1408ac5c9: 48 89 44 24 50              	mov	qword ptr [rsp + 0x50], rax
1408ac5ce: 48 8d 05 e3 f8 1e 02        	lea	rax, [rip + 0x21ef8e3]  # 0x142a9beb8
1408ac5d5: 48 89 44 24 58              	mov	qword ptr [rsp + 0x58], rax
1408ac5da: 48 c7 44 24 60 02 00 00 00  	mov	qword ptr [rsp + 0x60], 0x2
1408ac5e3: 48 c7 44 24 78 00 00 00 00  	mov	qword ptr [rsp + 0x78], 0x0
1408ac5ec: 48 8d 44 24 38              	lea	rax, [rsp + 0x38]
1408ac5f1: 48 89 44 24 68              	mov	qword ptr [rsp + 0x68], rax
1408ac5f6: 48 c7 44 24 70 02 00 00 00  	mov	qword ptr [rsp + 0x70], 0x2
1408ac5ff: 48 8d 15 3a f9 1e 02        	lea	rdx, [rip + 0x21ef93a]  # 0x142a9bf40
1408ac606: 48 8d 4c 24 58              	lea	rcx, [rsp + 0x58]
1408ac60b: e8 80 cd 02 02              	call	0x1428d9390 <.text+0x28d8390>
1408ac610: 48 8d 0d 11 01 1f 02        	lea	rcx, [rip + 0x21f0111]  # 0x142a9c728
1408ac617: 4c 8d 05 6a 01 1f 02        	lea	r8, [rip + 0x21f016a]   # 0x142a9c788
1408ac61e: ba 59 00 00 00              	mov	edx, 0x59
1408ac623: e8 08 cd 02 02              	call	0x1428d9330 <.text+0x28d8330>
1408ac628: cc                          	int3
