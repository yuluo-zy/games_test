
/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14090e310: 55                          	push	rbp
14090e311: 41 57                       	push	r15
14090e313: 41 56                       	push	r14
14090e315: 41 54                       	push	r12
14090e317: 56                          	push	rsi
14090e318: 57                          	push	rdi
14090e319: 53                          	push	rbx
14090e31a: 48 81 ec 80 03 00 00        	sub	rsp, 0x380
14090e321: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
14090e329: 48 c7 85 f8 02 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x2f8], -0x2
14090e334: 4c 89 8d e0 02 00 00        	mov	qword ptr [rbp + 0x2e0], r9
14090e33b: 4c 89 c7                    	mov	rdi, r8
14090e33e: 48 89 d6                    	mov	rsi, rdx
14090e341: 48 89 cb                    	mov	rbx, rcx
14090e344: 48 8b 85 60 03 00 00        	mov	rax, qword ptr [rbp + 0x360]
14090e34b: 48 89 85 e8 02 00 00        	mov	qword ptr [rbp + 0x2e8], rax
14090e352: 4c 8b 71 08                 	mov	r14, qword ptr [rcx + 0x8]
14090e356: 49 8b 40 18                 	mov	rax, qword ptr [r8 + 0x18]
14090e35a: c6 85 f7 02 00 00 01        	mov	byte ptr [rbp + 0x2f7], 0x1
14090e361: 48 89 d1                    	mov	rcx, rdx
14090e364: ff d0                       	call	rax
14090e366: 49 89 c7                    	mov	r15, rax
14090e369: 4d 8b 66 10                 	mov	r12, qword ptr [r14 + 0x10]
14090e36d: 4d 3b 26                    	cmp	r12, qword ptr [r14]
14090e370: 75 16                       	jne	0x14090e388 <.text+0x90d388>
14090e372: c6 85 f7 02 00 00 01        	mov	byte ptr [rbp + 0x2f7], 0x1
14090e379: 48 8d 15 80 d3 19 02        	lea	rdx, [rip + 0x219d380]  # 0x142aab700
14090e380: 4c 89 f1                    	mov	rcx, r14
14090e383: e8 e8 44 c5 ff              	call	0x140562870 <.text+0x561870>
14090e388: 49 8b 46 08                 	mov	rax, qword ptr [r14 + 0x8]
14090e38c: 4e 89 3c e0                 	mov	qword ptr [rax + 8*r12], r15
14090e390: 49 ff c4                    	inc	r12
14090e393: 4d 89 66 10                 	mov	qword ptr [r14 + 0x10], r12
14090e397: 48 8b 03                    	mov	rax, qword ptr [rbx]
14090e39a: 48 8b 4b 10                 	mov	rcx, qword ptr [rbx + 0x10]
14090e39e: c6 85 f7 02 00 00 01        	mov	byte ptr [rbp + 0x2f7], 0x1
14090e3a5: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
14090e3aa: 48 89 4c 24 28              	mov	qword ptr [rsp + 0x28], rcx
14090e3af: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
14090e3b4: 48 8d 15 5d d3 19 02        	lea	rdx, [rip + 0x219d35d]  # 0x142aab718
14090e3bb: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
14090e3c2: 41 b8 03 00 00 00           	mov	r8d, 0x3
14090e3c8: 49 89 f1                    	mov	r9, rsi
14090e3cb: e8 a0 21 cf 00              	call	0x141600570 <.text+0x15ff570>
14090e3d0: 48 8d 75 c0                 	lea	rsi, [rbp - 0x40]
14090e3d4: 48 8d 95 50 01 00 00        	lea	rdx, [rbp + 0x150]
14090e3db: 41 b8 90 01 00 00           	mov	r8d, 0x190
14090e3e1: 48 89 f1                    	mov	rcx, rsi
14090e3e4: e8 17 58 01 02              	call	0x142923c00 <.text+0x2922c00>
14090e3e9: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
14090e3f0: 48 89 f2                    	mov	rdx, rsi
14090e3f3: 48 8b 85 e8 02 00 00        	mov	rax, qword ptr [rbp + 0x2e8]
14090e3fa: ff 50 18                    	call	qword ptr [rax + 0x18]
14090e3fd: 48 8b 85 e8 02 00 00        	mov	rax, qword ptr [rbp + 0x2e8]
14090e404: 48 8b 50 08                 	mov	rdx, qword ptr [rax + 0x8]
14090e408: 48 85 d2                    	test	rdx, rdx
14090e40b: 74 21                       	je	0x14090e42e <.text+0x90d42e>
14090e40d: 4c 8b 40 10                 	mov	r8, qword ptr [rax + 0x10]
14090e411: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
14090e418: 48 81 c4 80 03 00 00        	add	rsp, 0x380
14090e41f: 5b                          	pop	rbx
14090e420: 5f                          	pop	rdi
14090e421: 5e                          	pop	rsi
14090e422: 41 5c                       	pop	r12
14090e424: 41 5e                       	pop	r14
14090e426: 41 5f                       	pop	r15
14090e428: 5d                          	pop	rbp
14090e429: e9 f2 57 d0 ff              	jmp	0x140613c20 <.text+0x612c20>
14090e42e: 90                          	nop
14090e42f: 48 81 c4 80 03 00 00        	add	rsp, 0x380
14090e436: 5b                          	pop	rbx
14090e437: 5f                          	pop	rdi
14090e438: 5e                          	pop	rsi
14090e439: 41 5c                       	pop	r12
14090e43b: 41 5e                       	pop	r14
14090e43d: 41 5f                       	pop	r15
14090e43f: 5d                          	pop	rbp
14090e440: c3                          	ret
14090e441: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
14090e450: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14090e455: 55                          	push	rbp
14090e456: 41 57                       	push	r15
14090e458: 41 56                       	push	r14
14090e45a: 41 54                       	push	r12
14090e45c: 56                          	push	rsi
14090e45d: 57                          	push	rdi
14090e45e: 53                          	push	rbx
14090e45f: 48 83 ec 40                 	sub	rsp, 0x40
14090e463: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14090e46a: 48 8b 85 e8 02 00 00        	mov	rax, qword ptr [rbp + 0x2e8]
14090e471: 48 8b 50 08                 	mov	rdx, qword ptr [rax + 0x8]
14090e475: 48 85 d2                    	test	rdx, rdx
14090e478: 74 17                       	je	0x14090e491 <.text+0x90d491>
14090e47a: 48 8b 85 e8 02 00 00        	mov	rax, qword ptr [rbp + 0x2e8]
14090e481: 4c 8b 40 10                 	mov	r8, qword ptr [rax + 0x10]
14090e485: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
14090e48c: e8 8f 57 d0 ff              	call	0x140613c20 <.text+0x612c20>
14090e491: c6 85 f7 02 00 00 00        	mov	byte ptr [rbp + 0x2f7], 0x0
14090e498: 48 83 c4 40                 	add	rsp, 0x40
14090e49c: 5b                          	pop	rbx
14090e49d: 5f                          	pop	rdi
14090e49e: 5e                          	pop	rsi
14090e49f: 41 5c                       	pop	r12
14090e4a1: 41 5e                       	pop	r14
14090e4a3: 41 5f                       	pop	r15
14090e4a5: 5d                          	pop	rbp
14090e4a6: c3                          	ret
14090e4a7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
14090e4b0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14090e4b5: 55                          	push	rbp
14090e4b6: 41 57                       	push	r15
14090e4b8: 41 56                       	push	r14
14090e4ba: 41 54                       	push	r12
14090e4bc: 56                          	push	rsi
14090e4bd: 57                          	push	rdi
14090e4be: 53                          	push	rbx
14090e4bf: 48 83 ec 40                 	sub	rsp, 0x40
14090e4c3: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14090e4ca: 80 bd f7 02 00 00 00        	cmp	byte ptr [rbp + 0x2f7], 0x0
14090e4d1: 75 0f                       	jne	0x14090e4e2 <.text+0x90d4e2>
14090e4d3: 48 83 c4 40                 	add	rsp, 0x40
14090e4d7: 5b                          	pop	rbx
14090e4d8: 5f                          	pop	rdi
14090e4d9: 5e                          	pop	rsi
14090e4da: 41 5c                       	pop	r12
14090e4dc: 41 5e                       	pop	r14
14090e4de: 41 5f                       	pop	r15
14090e4e0: 5d                          	pop	rbp
14090e4e1: c3                          	ret
14090e4e2: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
14090e4e9: 48 8b 95 e8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2e8]
14090e4f0: e8 0b 04 72 ff              	call	0x14002e900 <.text+0x2d900>
14090e4f5: eb dc                       	jmp	0x14090e4d3 <.text+0x90d4d3>
