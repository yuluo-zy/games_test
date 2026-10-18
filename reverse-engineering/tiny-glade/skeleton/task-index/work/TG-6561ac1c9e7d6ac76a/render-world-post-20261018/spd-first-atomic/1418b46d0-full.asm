
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1418b46d0: 55                          	push	rbp
1418b46d1: 41 57                       	push	r15
1418b46d3: 41 56                       	push	r14
1418b46d5: 41 55                       	push	r13
1418b46d7: 41 54                       	push	r12
1418b46d9: 56                          	push	rsi
1418b46da: 57                          	push	rdi
1418b46db: 53                          	push	rbx
1418b46dc: 48 81 ec 38 01 00 00        	sub	rsp, 0x138
1418b46e3: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
1418b46eb: 48 c7 85 b0 00 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0xb0], -0x2
1418b46f6: 44 8b 79 20                 	mov	r15d, dword ptr [rcx + 0x20]
1418b46fa: 8b 41 24                    	mov	eax, dword ptr [rcx + 0x24]
1418b46fd: 89 85 ac 00 00 00           	mov	dword ptr [rbp + 0xac], eax
1418b4703: 4c 8b 29                    	mov	r13, qword ptr [rcx]
1418b4706: 48 8b 71 08                 	mov	rsi, qword ptr [rcx + 0x8]
1418b470a: 48 8b 59 10                 	mov	rbx, qword ptr [rcx + 0x10]
1418b470e: 48 8b 79 18                 	mov	rdi, qword ptr [rcx + 0x18]
1418b4712: 48 8b 02                    	mov	rax, qword ptr [rdx]
1418b4715: 48 89 85 a0 00 00 00        	mov	qword ptr [rbp + 0xa0], rax
1418b471c: 4c 8b 42 08                 	mov	r8, qword ptr [rdx + 0x8]
1418b4720: 4c 8b 72 10                 	mov	r14, qword ptr [rdx + 0x10]
1418b4724: 44 3b bd ac 00 00 00        	cmp	r15d, dword ptr [rbp + 0xac]
1418b472b: 0f 83 cc 00 00 00           	jae	0x1418b47fd <.text+0x18b37fd>
1418b4731: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
1418b4740: 4c 89 85 98 00 00 00        	mov	qword ptr [rbp + 0x98], r8
1418b4747: 41 0f 10 45 00              	movups	xmm0, xmmword ptr [r13]
1418b474c: 41 0f 10 4d 10              	movups	xmm1, xmmword ptr [r13 + 0x10]
1418b4751: 41 0f 10 55 20              	movups	xmm2, xmmword ptr [r13 + 0x20]
1418b4756: 41 0f 10 5d 2c              	movups	xmm3, xmmword ptr [r13 + 0x2c]
1418b475b: 0f 11 5d 7c                 	movups	xmmword ptr [rbp + 0x7c], xmm3
1418b475f: 0f 29 55 70                 	movaps	xmmword ptr [rbp + 0x70], xmm2
1418b4763: 0f 29 4d 60                 	movaps	xmmword ptr [rbp + 0x60], xmm1
1418b4767: 0f 29 45 50                 	movaps	xmmword ptr [rbp + 0x50], xmm0
1418b476b: 44 89 bd 80 00 00 00        	mov	dword ptr [rbp + 0x80], r15d
1418b4772: 48 b8 01 00 00 00 01 00 00 00       	movabs	rax, 0x100000001
1418b477c: 48 89 45 70                 	mov	qword ptr [rbp + 0x70], rax
1418b4780: c6 85 88 00 00 00 00        	mov	byte ptr [rbp + 0x88], 0x0
1418b4787: 8b 53 3c                    	mov	edx, dword ptr [rbx + 0x3c]
1418b478a: 48 89 f1                    	mov	rcx, rsi
1418b478d: e8 fe 25 e4 ff              	call	0x1416f6d90 <.text+0x16f5d90>
1418b4792: 48 8d 8d 8c 00 00 00        	lea	rcx, [rbp + 0x8c]
1418b4799: 89 c2                       	mov	edx, eax
1418b479b: e8 30 c6 35 ff              	call	0x140c10dd0 <.text+0xc0fdd0>
1418b47a0: 44 8b a5 94 00 00 00        	mov	r12d, dword ptr [rbp + 0x94]
1418b47a7: 48 8b 56 10                 	mov	rdx, qword ptr [rsi + 0x10]
1418b47ab: 44 8b 4b 3c                 	mov	r9d, dword ptr [rbx + 0x3c]
1418b47af: 48 8d 45 50                 	lea	rax, [rbp + 0x50]
1418b47b3: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1418b47b8: 48 89 e9                    	mov	rcx, rbp
1418b47bb: 49 89 f8                    	mov	r8, rdi
1418b47be: e8 9d 32 33 ff              	call	0x140be7a60 <.text+0xbe6a60>
1418b47c3: 83 7d 00 0b                 	cmp	dword ptr [rbp], 0xb
1418b47c7: 75 52                       	jne	0x1418b481b <.text+0x18b381b>
1418b47c9: 41 ff c7                    	inc	r15d
1418b47cc: 48 8b 45 08                 	mov	rax, qword ptr [rbp + 0x8]
1418b47d0: 4c 8b 85 98 00 00 00        	mov	r8, qword ptr [rbp + 0x98]
1418b47d7: 4b 8d 0c 40                 	lea	rcx, [r8 + 2*r8]
1418b47db: 49 c7 04 ce 00 00 00 00     	mov	qword ptr [r14 + 8*rcx], 0x0
1418b47e3: 49 89 44 ce 08              	mov	qword ptr [r14 + 8*rcx + 0x8], rax
1418b47e8: 4d 89 64 ce 10              	mov	qword ptr [r14 + 8*rcx + 0x10], r12
1418b47ed: 49 ff c0                    	inc	r8
1418b47f0: 44 3b bd ac 00 00 00        	cmp	r15d, dword ptr [rbp + 0xac]
1418b47f7: 0f 82 43 ff ff ff           	jb	0x1418b4740 <.text+0x18b3740>
1418b47fd: 48 8b 85 a0 00 00 00        	mov	rax, qword ptr [rbp + 0xa0]
1418b4804: 4c 89 00                    	mov	qword ptr [rax], r8
1418b4807: 48 81 c4 38 01 00 00        	add	rsp, 0x138
1418b480e: 5b                          	pop	rbx
1418b480f: 5f                          	pop	rdi
1418b4810: 5e                          	pop	rsi
1418b4811: 41 5c                       	pop	r12
1418b4813: 41 5d                       	pop	r13
1418b4815: 41 5e                       	pop	r14
1418b4817: 41 5f                       	pop	r15
1418b4819: 5d                          	pop	rbp
1418b481a: c3                          	ret
1418b481b: 0f 10 45 40                 	movups	xmm0, xmmword ptr [rbp + 0x40]
1418b481f: 0f 29 45 f0                 	movaps	xmmword ptr [rbp - 0x10], xmm0
1418b4823: 0f 10 45 00                 	movups	xmm0, xmmword ptr [rbp]
1418b4827: 0f 10 4d 10                 	movups	xmm1, xmmword ptr [rbp + 0x10]
1418b482b: 0f 10 55 20                 	movups	xmm2, xmmword ptr [rbp + 0x20]
1418b482f: 0f 10 5d 30                 	movups	xmm3, xmmword ptr [rbp + 0x30]
1418b4833: 0f 29 5d e0                 	movaps	xmmword ptr [rbp - 0x20], xmm3
1418b4837: 0f 29 55 d0                 	movaps	xmmword ptr [rbp - 0x30], xmm2
1418b483b: 0f 29 4d c0                 	movaps	xmmword ptr [rbp - 0x40], xmm1
1418b483f: 0f 29 45 b0                 	movaps	xmmword ptr [rbp - 0x50], xmm0
1418b4843: 48 8d 05 2e ee 44 01        	lea	rax, [rip + 0x144ee2e]  # 0x142d03678
1418b484a: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1418b484f: 48 8d 0d 9a ea 44 01        	lea	rcx, [rip + 0x144ea9a]  # 0x142d032f0
1418b4856: 4c 8d 0d c3 ea 44 01        	lea	r9, [rip + 0x144eac3]   # 0x142d03320
1418b485d: 4c 8d 45 b0                 	lea	r8, [rbp - 0x50]
1418b4861: ba 2b 00 00 00              	mov	edx, 0x2b
1418b4866: e8 f5 4e 02 01              	call	0x1428d9760 <.text+0x28d8760>
1418b486b: 0f 0b                       	ud2
1418b486d: 0f 1f 00                    	nop	dword ptr [rax]
1418b4870: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1418b4875: 55                          	push	rbp
1418b4876: 41 57                       	push	r15
1418b4878: 41 56                       	push	r14
1418b487a: 41 55                       	push	r13
1418b487c: 41 54                       	push	r12
1418b487e: 56                          	push	rsi
1418b487f: 57                          	push	rdi
1418b4880: 53                          	push	rbx
1418b4881: 48 83 ec 28                 	sub	rsp, 0x28
1418b4885: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1418b488c: 48 8b 85 a0 00 00 00        	mov	rax, qword ptr [rbp + 0xa0]
1418b4893: 48 8b 8d 98 00 00 00        	mov	rcx, qword ptr [rbp + 0x98]
1418b489a: 48 89 08                    	mov	qword ptr [rax], rcx
1418b489d: 48 83 c4 28                 	add	rsp, 0x28
1418b48a1: 5b                          	pop	rbx
1418b48a2: 5f                          	pop	rdi
1418b48a3: 5e                          	pop	rsi
1418b48a4: 41 5c                       	pop	r12
1418b48a6: 41 5d                       	pop	r13
1418b48a8: 41 5e                       	pop	r14
1418b48aa: 41 5f                       	pop	r15
1418b48ac: 5d                          	pop	rbp
1418b48ad: c3                          	ret
1418b48ae: 66 90                       	nop
1418b48b0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1418b48b5: 55                          	push	rbp
1418b48b6: 41 57                       	push	r15
1418b48b8: 41 56                       	push	r14
1418b48ba: 41 55                       	push	r13
1418b48bc: 41 54                       	push	r12
1418b48be: 56                          	push	rsi
1418b48bf: 57                          	push	rdi
1418b48c0: 53                          	push	rbx
1418b48c1: 48 83 ec 28                 	sub	rsp, 0x28
1418b48c5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1418b48cc: 48 8d 4d b0                 	lea	rcx, [rbp - 0x50]
1418b48d0: e8 9b a1 f1 fe              	call	0x1407cea70 <.text+0x7cda70>
1418b48d5: 90                          	nop
1418b48d6: 48 83 c4 28                 	add	rsp, 0x28
1418b48da: 5b                          	pop	rbx
1418b48db: 5f                          	pop	rdi
1418b48dc: 5e                          	pop	rsi
1418b48dd: 41 5c                       	pop	r12
1418b48df: 41 5d                       	pop	r13
1418b48e1: 41 5e                       	pop	r14
1418b48e3: 41 5f                       	pop	r15
1418b48e5: 5d                          	pop	rbp
1418b48e6: c3                          	ret
