
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1414f8ff0: 41 56                       	push	r14
1414f8ff2: 56                          	push	rsi
1414f8ff3: 57                          	push	rdi
1414f8ff4: 53                          	push	rbx
1414f8ff5: 48 81 ec 88 00 00 00        	sub	rsp, 0x88
1414f8ffc: f6 01 01                    	test	byte ptr [rcx], 0x1
1414f8fff: 0f 84 f6 01 00 00           	je	0x1414f91fb <.text+0x14f81fb>
1414f9005: 48 89 ce                    	mov	rsi, rcx
1414f9008: 48 8b 82 a0 05 00 00        	mov	rax, qword ptr [rdx + 0x5a0]
1414f900f: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
1414f9014: 48 39 41 18                 	cmp	qword ptr [rcx + 0x18], rax
1414f9018: 0f 85 e1 00 00 00           	jne	0x1414f90ff <.text+0x14f80ff>
1414f901e: 48 8b 82 08 01 00 00        	mov	rax, qword ptr [rdx + 0x108]
1414f9025: 8b 8e 58 01 00 00           	mov	ecx, dword ptr [rsi + 0x158]
1414f902b: 89 86 58 01 00 00           	mov	dword ptr [rsi + 0x158], eax
1414f9031: 48 39 c8                    	cmp	rax, rcx
1414f9034: 0f 82 0f 01 00 00           	jb	0x1414f9149 <.text+0x14f8149>
1414f903a: bb 01 00 00 00              	mov	ebx, 0x1
1414f903f: f0                          	lock
1414f9040: 0f c1 9a b0 05 00 00        	xadd	dword ptr [rdx + 0x5b0], ebx
1414f9047: 48 8b 46 08                 	mov	rax, qword ptr [rsi + 0x8]
1414f904b: 4c 8b 82 18 02 00 00        	mov	r8, qword ptr [rdx + 0x218]
1414f9052: 49 39 c0                    	cmp	r8, rax
1414f9055: 0f 86 fd 00 00 00           	jbe	0x1414f9158 <.text+0x14f8158>
1414f905b: 4c 8b 8a 10 02 00 00        	mov	r9, qword ptr [rdx + 0x210]
1414f9062: 49 8b 0c c1                 	mov	rcx, qword ptr [r9 + 8*rax]
1414f9066: 48 85 c9                    	test	rcx, rcx
1414f9069: 0f 84 e9 00 00 00           	je	0x1414f9158 <.text+0x14f8158>
1414f906f: 48 8b 82 e0 01 00 00        	mov	rax, qword ptr [rdx + 0x1e0]
1414f9076: 48 f7 d1                    	not	rcx
1414f9079: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1414f907d: 48 c1 e1 05                 	shl	rcx, 0x5
1414f9081: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
1414f9087: 0f 84 cb 00 00 00           	je	0x1414f9158 <.text+0x14f8158>
1414f908d: 48 8b 56 10                 	mov	rdx, qword ptr [rsi + 0x10]
1414f9091: 49 39 d0                    	cmp	r8, rdx
1414f9094: 0f 86 dc 00 00 00           	jbe	0x1414f9176 <.text+0x14f8176>
1414f909a: 49 8b 14 d1                 	mov	rdx, qword ptr [r9 + 8*rdx]
1414f909e: 48 85 d2                    	test	rdx, rdx
1414f90a1: 0f 84 cf 00 00 00           	je	0x1414f9176 <.text+0x14f8176>
1414f90a7: 48 f7 d2                    	not	rdx
1414f90aa: 48 8d 14 52                 	lea	rdx, [rdx + 2*rdx]
1414f90ae: 48 c1 e2 05                 	shl	rdx, 0x5
1414f90b2: 48 83 7c 10 20 00           	cmp	qword ptr [rax + rdx + 0x20], 0x0
1414f90b8: 0f 84 b8 00 00 00           	je	0x1414f9176 <.text+0x14f8176>
1414f90be: 48 01 c1                    	add	rcx, rax
1414f90c1: 4c 8b 71 10                 	mov	r14, qword ptr [rcx + 0x10]
1414f90c5: 48 01 c2                    	add	rdx, rax
1414f90c8: 48 8b 7a 10                 	mov	rdi, qword ptr [rdx + 0x10]
1414f90cc: 49 8b 16                    	mov	rdx, qword ptr [r14]
1414f90cf: 48 83 c2 10                 	add	rdx, 0x10
1414f90d3: 48 89 f9                    	mov	rcx, rdi
1414f90d6: e8 75 9a dc ff              	call	0x1412c2b50 <.text+0x12c1b50>
1414f90db: 49 8b 16                    	mov	rdx, qword ptr [r14]
1414f90de: 48 83 c2 10                 	add	rdx, 0x10
1414f90e2: 48 89 f9                    	mov	rcx, rdi
1414f90e5: e8 06 54 dc ff              	call	0x1412be4f0 <.text+0x12bd4f0>
1414f90ea: 89 9e 50 01 00 00           	mov	dword ptr [rsi + 0x150], ebx
1414f90f0: 31 c0                       	xor	eax, eax
1414f90f2: 48 81 c4 88 00 00 00        	add	rsp, 0x88
1414f90f9: 5b                          	pop	rbx
1414f90fa: 5f                          	pop	rdi
1414f90fb: 5e                          	pop	rsi
1414f90fc: 41 5e                       	pop	r14
1414f90fe: c3                          	ret
1414f90ff: 48 8d 05 5a e8 75 01        	lea	rax, [rip + 0x175e85a]  # 0x142c57960
1414f9106: 48 89 44 24 58              	mov	qword ptr [rsp + 0x58], rax
1414f910b: 48 c7 44 24 60 01 00 00 00  	mov	qword ptr [rsp + 0x60], 0x1
1414f9114: 48 c7 44 24 68 08 00 00 00  	mov	qword ptr [rsp + 0x68], 0x8
1414f911d: 0f 57 c0                    	xorps	xmm0, xmm0
1414f9120: 0f 11 44 24 70              	movups	xmmword ptr [rsp + 0x70], xmm0
1414f9125: 48 83 c6 18                 	add	rsi, 0x18
1414f9129: 48 8d 05 40 e8 75 01        	lea	rax, [rip + 0x175e840]  # 0x142c57970
1414f9130: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1414f9135: 4c 8d 44 24 28              	lea	r8, [rsp + 0x28]
1414f913a: 4c 8d 4c 24 58              	lea	r9, [rsp + 0x58]
1414f913f: 31 c9                       	xor	ecx, ecx
1414f9141: 48 89 f2                    	mov	rdx, rsi
1414f9144: e8 98 4f 3d 01              	call	0x1428ce0e1 <.text+0x28cd0e1>
1414f9149: 4c 8d 05 38 e8 75 01        	lea	r8, [rip + 0x175e838]   # 0x142c57988
1414f9150: 48 89 c2                    	mov	rdx, rax
1414f9153: e8 d8 06 3e 01              	call	0x1428d9830 <.text+0x28d8830>
1414f9158: 48 81 c6 b8 00 00 00        	add	rsi, 0xb8
1414f915f: 48 8d 05 85 dd 76 01        	lea	rax, [rip + 0x176dd85]  # 0x142c66eeb
1414f9166: 48 89 44 24 48              	mov	qword ptr [rsp + 0x48], rax
1414f916b: 48 c7 44 24 50 2c 00 00 00  	mov	qword ptr [rsp + 0x50], 0x2c
1414f9174: eb 1c                       	jmp	0x1414f9192 <.text+0x14f8192>
1414f9176: 48 81 c6 b8 00 00 00        	add	rsi, 0xb8
1414f917d: 48 8d 05 9c f8 76 01        	lea	rax, [rip + 0x176f89c]  # 0x142c68a20
1414f9184: 48 89 44 24 48              	mov	qword ptr [rsp + 0x48], rax
1414f9189: 48 c7 44 24 50 3d 00 00 00  	mov	qword ptr [rsp + 0x50], 0x3d
1414f9192: 48 89 74 24 28              	mov	qword ptr [rsp + 0x28], rsi
1414f9197: 48 8d 05 32 29 b4 fe        	lea	rax, [rip - 0x14bd6ce]  # 0x14003bad0 <.text+0x3aad0>
1414f919e: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
1414f91a3: 48 8d 44 24 48              	lea	rax, [rsp + 0x48]
1414f91a8: 48 89 44 24 38              	mov	qword ptr [rsp + 0x38], rax
1414f91ad: 48 8d 05 dc 0c c1 fe        	lea	rax, [rip - 0x13ef324]  # 0x140109e90 <.text+0x108e90>
1414f91b4: 48 89 44 24 40              	mov	qword ptr [rsp + 0x40], rax
1414f91b9: 48 8d 05 c0 cc 76 01        	lea	rax, [rip + 0x176ccc0]  # 0x142c65e80
1414f91c0: 48 89 44 24 58              	mov	qword ptr [rsp + 0x58], rax
1414f91c5: 48 c7 44 24 60 02 00 00 00  	mov	qword ptr [rsp + 0x60], 0x2
1414f91ce: 48 c7 44 24 78 00 00 00 00  	mov	qword ptr [rsp + 0x78], 0x0
1414f91d7: 48 8d 44 24 28              	lea	rax, [rsp + 0x28]
1414f91dc: 48 89 44 24 68              	mov	qword ptr [rsp + 0x68], rax
1414f91e1: 48 c7 44 24 70 02 00 00 00  	mov	qword ptr [rsp + 0x70], 0x2
1414f91ea: 48 8d 15 17 cd 76 01        	lea	rdx, [rip + 0x176cd17]  # 0x142c65f08
1414f91f1: 48 8d 4c 24 58              	lea	rcx, [rsp + 0x58]
1414f91f6: e8 95 01 3e 01              	call	0x1428d9390 <.text+0x28d8390>
1414f91fb: 48 8d 0d a8 99 75 01        	lea	rcx, [rip + 0x17599a8]  # 0x142c52baa
1414f9202: 4c 8d 05 cf e6 75 01        	lea	r8, [rip + 0x175e6cf]   # 0x142c578d8
1414f9209: ba 59 00 00 00              	mov	edx, 0x59
1414f920e: e8 1d 01 3e 01              	call	0x1428d9330 <.text+0x28d8330>
1414f9213: cc                          	int3
