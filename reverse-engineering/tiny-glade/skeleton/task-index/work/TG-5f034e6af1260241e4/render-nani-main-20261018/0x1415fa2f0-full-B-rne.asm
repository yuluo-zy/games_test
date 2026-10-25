
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1415fa2f0: 55                          	push	rbp
1415fa2f1: 41 57                       	push	r15
1415fa2f3: 41 56                       	push	r14
1415fa2f5: 41 55                       	push	r13
1415fa2f7: 41 54                       	push	r12
1415fa2f9: 56                          	push	rsi
1415fa2fa: 57                          	push	rdi
1415fa2fb: 53                          	push	rbx
1415fa2fc: 48 81 ec e8 00 00 00        	sub	rsp, 0xe8
1415fa303: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
1415fa30b: 48 c7 45 60 fe ff ff ff     	mov	qword ptr [rbp + 0x60], -0x2
1415fa313: 48 89 55 30                 	mov	qword ptr [rbp + 0x30], rdx
1415fa317: 4c 89 45 38                 	mov	qword ptr [rbp + 0x38], r8
1415fa31b: 4c 8b 19                    	mov	r11, qword ptr [rcx]
1415fa31e: 4c 8b 51 08                 	mov	r10, qword ptr [rcx + 0x8]
1415fa322: 48 8b 41 10                 	mov	rax, qword ptr [rcx + 0x10]
1415fa326: 48 8d 14 80                 	lea	rdx, [rax + 4*rax]
1415fa32a: 49 8d 14 92                 	lea	rdx, [r10 + 4*rdx]
1415fa32e: 4c 8b 41 18                 	mov	r8, qword ptr [rcx + 0x18]
1415fa332: 4c 8b 49 20                 	mov	r9, qword ptr [rcx + 0x20]
1415fa336: 48 8b 79 28                 	mov	rdi, qword ptr [rcx + 0x28]
1415fa33a: 49 8d 0c b9                 	lea	rcx, [r9 + 4*rdi]
1415fa33e: 0f 57 c0                    	xorps	xmm0, xmm0
1415fa341: 0f 11 45 e8                 	movups	xmmword ptr [rbp - 0x18], xmm0
1415fa345: 48 c7 45 f8 00 00 00 00     	mov	qword ptr [rbp - 0x8], 0x0
1415fa34d: 4c 89 55 a8                 	mov	qword ptr [rbp - 0x58], r10
1415fa351: 4c 89 55 58                 	mov	qword ptr [rbp + 0x58], r10
1415fa355: 4c 89 55 b0                 	mov	qword ptr [rbp - 0x50], r10
1415fa359: 4c 89 5d 28                 	mov	qword ptr [rbp + 0x28], r11
1415fa35d: 4c 89 5d b8                 	mov	qword ptr [rbp - 0x48], r11
1415fa361: 48 89 55 c0                 	mov	qword ptr [rbp - 0x40], rdx
1415fa365: 4c 89 4d 50                 	mov	qword ptr [rbp + 0x50], r9
1415fa369: 4c 89 4d c8                 	mov	qword ptr [rbp - 0x38], r9
1415fa36d: 4c 89 45 20                 	mov	qword ptr [rbp + 0x20], r8
1415fa371: 4c 89 45 d8                 	mov	qword ptr [rbp - 0x28], r8
1415fa375: 48 89 4d e0                 	mov	qword ptr [rbp - 0x20], rcx
1415fa379: 48 85 c0                    	test	rax, rax
1415fa37c: 0f 84 a1 00 00 00           	je	0x1415fa423 <.text+0x15f9423>
1415fa382: 48 c1 e7 02                 	shl	rdi, 0x2
1415fa386: 48 8b 4d 50                 	mov	rcx, qword ptr [rbp + 0x50]
1415fa38a: 48 8d 51 04                 	lea	rdx, [rcx + 0x4]
1415fa38e: 48 8b 4d 58                 	mov	rcx, qword ptr [rbp + 0x58]
1415fa392: 48 83 c1 14                 	add	rcx, 0x14
1415fa396: 48 c1 e0 02                 	shl	rax, 0x2
1415fa39a: 48 8d 34 80                 	lea	rsi, [rax + 4*rax]
1415fa39e: 49 89 ee                    	mov	r14, rbp
1415fa3a1: 49 bd 01 00 00 00 01 00 00 00       	movabs	r13, 0x100000001
1415fa3ab: 4c 8d 7d 30                 	lea	r15, [rbp + 0x30]
1415fa3af: 4c 8d 65 14                 	lea	r12, [rbp + 0x14]
1415fa3b3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
1415fa3c0: 48 85 ff                    	test	rdi, rdi
1415fa3c3: 74 5e                       	je	0x1415fa423 <.text+0x15f9423>
1415fa3c5: 48 89 4d 48                 	mov	qword ptr [rbp + 0x48], rcx
1415fa3c9: 48 8d 41 ec                 	lea	rax, [rcx - 0x14]
1415fa3cd: 48 89 55 40                 	mov	qword ptr [rbp + 0x40], rdx
1415fa3d1: 8b 5a fc                    	mov	ebx, dword ptr [rdx - 0x4]
1415fa3d4: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
1415fa3d7: 8b 40 10                    	mov	eax, dword ptr [rax + 0x10]
1415fa3da: 89 45 10                    	mov	dword ptr [rbp + 0x10], eax
1415fa3dd: 0f 29 45 00                 	movaps	xmmword ptr [rbp], xmm0
1415fa3e1: 48 8b 4d 30                 	mov	rcx, qword ptr [rbp + 0x30]
1415fa3e5: 48 8b 55 38                 	mov	rdx, qword ptr [rbp + 0x38]
1415fa3e9: 41 b9 14 00 00 00           	mov	r9d, 0x14
1415fa3ef: 4d 89 f0                    	mov	r8, r14
1415fa3f2: e8 59 d6 0f 00              	call	0x1416f7a50 <.text+0x16f6a50>
1415fa3f7: 89 5d 14                    	mov	dword ptr [rbp + 0x14], ebx
1415fa3fa: 4c 89 6d 18                 	mov	qword ptr [rbp + 0x18], r13
1415fa3fe: 4c 89 f9                    	mov	rcx, r15
1415fa401: 4c 89 e2                    	mov	rdx, r12
1415fa404: e8 97 4b 37 ff              	call	0x14096efa0 <.text+0x96dfa0>
1415fa409: 48 83 c7 fc                 	add	rdi, -0x4
1415fa40d: 48 8b 55 40                 	mov	rdx, qword ptr [rbp + 0x40]
1415fa411: 48 83 c2 04                 	add	rdx, 0x4
1415fa415: 48 8b 4d 48                 	mov	rcx, qword ptr [rbp + 0x48]
1415fa419: 48 83 c1 14                 	add	rcx, 0x14
1415fa41d: 48 83 c6 ec                 	add	rsi, -0x14
1415fa421: 75 9d                       	jne	0x1415fa3c0 <.text+0x15f93c0>
1415fa423: 48 8b 45 28                 	mov	rax, qword ptr [rbp + 0x28]
1415fa427: 48 85 c0                    	test	rax, rax
1415fa42a: 74 17                       	je	0x1415fa443 <.text+0x15f9443>
1415fa42c: 48 c1 e0 02                 	shl	rax, 0x2
1415fa430: 48 8d 14 80                 	lea	rdx, [rax + 4*rax]
1415fa434: 41 b8 01 00 00 00           	mov	r8d, 0x1
1415fa43a: 48 8b 4d 58                 	mov	rcx, qword ptr [rbp + 0x58]
1415fa43e: e8 dd 97 01 ff              	call	0x140613c20 <.text+0x612c20>
1415fa443: 48 8b 55 20                 	mov	rdx, qword ptr [rbp + 0x20]
1415fa447: 48 85 d2                    	test	rdx, rdx
1415fa44a: 74 13                       	je	0x1415fa45f <.text+0x15f945f>
1415fa44c: 48 c1 e2 02                 	shl	rdx, 0x2
1415fa450: 41 b8 04 00 00 00           	mov	r8d, 0x4
1415fa456: 48 8b 4d 50                 	mov	rcx, qword ptr [rbp + 0x50]
1415fa45a: e8 c1 97 01 ff              	call	0x140613c20 <.text+0x612c20>
1415fa45f: 90                          	nop
1415fa460: 48 81 c4 e8 00 00 00        	add	rsp, 0xe8
1415fa467: 5b                          	pop	rbx
1415fa468: 5f                          	pop	rdi
1415fa469: 5e                          	pop	rsi
1415fa46a: 41 5c                       	pop	r12
1415fa46c: 41 5d                       	pop	r13
1415fa46e: 41 5e                       	pop	r14
1415fa470: 41 5f                       	pop	r15
1415fa472: 5d                          	pop	rbp
1415fa473: c3                          	ret
1415fa474: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
1415fa480: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1415fa485: 55                          	push	rbp
1415fa486: 41 57                       	push	r15
1415fa488: 41 56                       	push	r14
1415fa48a: 41 55                       	push	r13
1415fa48c: 41 54                       	push	r12
1415fa48e: 56                          	push	rsi
1415fa48f: 57                          	push	rdi
1415fa490: 53                          	push	rbx
1415fa491: 48 83 ec 28                 	sub	rsp, 0x28
1415fa495: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1415fa49c: 48 8b 45 48                 	mov	rax, qword ptr [rbp + 0x48]
1415fa4a0: 48 89 45 b0                 	mov	qword ptr [rbp - 0x50], rax
1415fa4a4: 48 8b 45 40                 	mov	rax, qword ptr [rbp + 0x40]
1415fa4a8: 48 89 45 d0                 	mov	qword ptr [rbp - 0x30], rax
1415fa4ac: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
1415fa4b0: e8 eb a4 28 ff              	call	0x1408849a0 <.text+0x8839a0>
1415fa4b5: 90                          	nop
1415fa4b6: 48 83 c4 28                 	add	rsp, 0x28
1415fa4ba: 5b                          	pop	rbx
1415fa4bb: 5f                          	pop	rdi
1415fa4bc: 5e                          	pop	rsi
1415fa4bd: 41 5c                       	pop	r12
1415fa4bf: 41 5d                       	pop	r13
1415fa4c1: 41 5e                       	pop	r14
1415fa4c3: 41 5f                       	pop	r15
1415fa4c5: 5d                          	pop	rbp
1415fa4c6: c3                          	ret
