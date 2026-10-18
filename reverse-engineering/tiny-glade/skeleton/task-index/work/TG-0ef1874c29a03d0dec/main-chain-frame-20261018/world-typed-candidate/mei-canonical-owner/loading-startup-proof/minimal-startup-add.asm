
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b65f80: 56                          	push	rsi
140b65f81: 57                          	push	rdi
140b65f82: 53                          	push	rbx
140b65f83: 48 81 ec c0 00 00 00        	sub	rsp, 0xc0
140b65f8a: 48 89 ce                    	mov	rsi, rcx
140b65f8d: 48 83 c1 38                 	add	rcx, 0x38
140b65f91: 48 ba 84 a7 2b 96 51 d2 1b 8e       	movabs	rdx, -0x71e42dae69d4587c
140b65f9b: 49 b8 9b 52 e6 c0 74 23 78 6a       	movabs	r8, 0x6a782374c0e6529b
140b65fa5: e8 46 77 57 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
140b65faa: 48 85 c0                    	test	rax, rax
140b65fad: 0f 84 9b 00 00 00           	je	0x140b6604e <.text+0xb6504e>
140b65fb3: 48 39 96 18 02 00 00        	cmp	qword ptr [rsi + 0x218], rdx
140b65fba: 0f 86 8e 00 00 00           	jbe	0x140b6604e <.text+0xb6504e>
140b65fc0: 48 8b 86 10 02 00 00        	mov	rax, qword ptr [rsi + 0x210]
140b65fc7: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
140b65fcb: 48 85 c9                    	test	rcx, rcx
140b65fce: 74 7e                       	je	0x140b6604e <.text+0xb6504e>
140b65fd0: 48 8b 86 e0 01 00 00        	mov	rax, qword ptr [rsi + 0x1e0]
140b65fd7: 48 f7 d1                    	not	rcx
140b65fda: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
140b65fde: 48 c1 e1 05                 	shl	rcx, 0x5
140b65fe2: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
140b65fe8: 74 64                       	je	0x140b6604e <.text+0xb6504e>
140b65fea: 48 01 c8                    	add	rax, rcx
140b65fed: 48 8b 48 10                 	mov	rcx, qword ptr [rax + 0x10]
140b65ff1: 8b 96 b0 05 00 00           	mov	edx, dword ptr [rsi + 0x5b0]
140b65ff7: 89 50 5c                    	mov	dword ptr [rax + 0x5c], edx
140b65ffa: e8 21 bc b3 00              	call	0x1416a1c20 <.text+0x16a0c20>
140b65fff: 48 89 c7                    	mov	rdi, rax
140b66002: 48 8d 5c 24 50              	lea	rbx, [rsp + 0x50]
140b66007: 48 89 d9                    	mov	rcx, rbx
140b6600a: e8 31 52 89 00              	call	0x1413fb240 <.text+0x13fa240>
140b6600f: 48 8d 4c 24 30              	lea	rcx, [rsp + 0x30]
140b66014: 48 89 fa                    	mov	rdx, rdi
140b66017: 49 89 d8                    	mov	r8, rbx
140b6601a: 45 31 c9                    	xor	r9d, r9d
140b6601d: e8 0e 8d b3 00              	call	0x14169ed30 <.text+0x169dd30>
140b66022: 48 8b 54 24 30              	mov	rdx, qword ptr [rsp + 0x30]
140b66027: 48 85 d2                    	test	rdx, rdx
140b6602a: 74 14                       	je	0x140b66040 <.text+0xb65040>
140b6602c: 48 8b 4c 24 38              	mov	rcx, qword ptr [rsp + 0x38]
140b66031: 48 c1 e2 04                 	shl	rdx, 0x4
140b66035: 41 b8 08 00 00 00           	mov	r8d, 0x8
140b6603b: e8 e0 db aa ff              	call	0x140613c20 <.text+0x612c20>
140b66040: 48 89 f0                    	mov	rax, rsi
140b66043: 48 81 c4 c0 00 00 00        	add	rsp, 0xc0
140b6604a: 5b                          	pop	rbx
140b6604b: 5f                          	pop	rdi
140b6604c: 5e                          	pop	rsi
140b6604d: c3                          	ret
140b6604e: 48 8d 05 6b cc f8 01        	lea	rax, [rip + 0x1f8cc6b]  # 0x142af2cc0
140b66055: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
140b6605a: 48 c7 44 24 38 27 00 00 00  	mov	qword ptr [rsp + 0x38], 0x27
140b66063: 48 8d 44 24 30              	lea	rax, [rsp + 0x30]
140b66068: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
140b6606d: 48 8d 05 1c 3e 5a ff        	lea	rax, [rip - 0xa5c1e4]   # 0x140109e90 <.text+0x108e90>
140b66074: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
140b66079: 48 8d 05 70 cd f8 01        	lea	rax, [rip + 0x1f8cd70]  # 0x142af2df0
140b66080: 48 89 44 24 50              	mov	qword ptr [rsp + 0x50], rax
140b66085: 48 c7 44 24 58 02 00 00 00  	mov	qword ptr [rsp + 0x58], 0x2
140b6608e: 48 c7 44 24 70 00 00 00 00  	mov	qword ptr [rsp + 0x70], 0x0
140b66097: 48 8d 44 24 20              	lea	rax, [rsp + 0x20]
140b6609c: 48 89 44 24 60              	mov	qword ptr [rsp + 0x60], rax
140b660a1: 48 c7 44 24 68 01 00 00 00  	mov	qword ptr [rsp + 0x68], 0x1
140b660aa: 48 8d 15 f7 cb f8 01        	lea	rdx, [rip + 0x1f8cbf7]  # 0x142af2ca8
140b660b1: 48 8d 4c 24 50              	lea	rcx, [rsp + 0x50]
140b660b6: e8 d5 32 d7 01              	call	0x1428d9390 <.text+0x28d8390>
140b660bb: cc                          	int3
