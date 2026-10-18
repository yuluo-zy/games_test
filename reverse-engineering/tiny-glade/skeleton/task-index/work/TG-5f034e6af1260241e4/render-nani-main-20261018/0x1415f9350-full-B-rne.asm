
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1415f9350: 55                          	push	rbp
1415f9351: 41 57                       	push	r15
1415f9353: 41 56                       	push	r14
1415f9355: 41 54                       	push	r12
1415f9357: 56                          	push	rsi
1415f9358: 57                          	push	rdi
1415f9359: 53                          	push	rbx
1415f935a: 48 83 ec 70                 	sub	rsp, 0x70
1415f935e: 48 8d 6c 24 70              	lea	rbp, [rsp + 0x70]
1415f9363: 48 c7 45 f8 fe ff ff ff     	mov	qword ptr [rbp - 0x8], -0x2
1415f936b: 48 89 55 b8                 	mov	qword ptr [rbp - 0x48], rdx
1415f936f: 4c 89 45 c0                 	mov	qword ptr [rbp - 0x40], r8
1415f9373: 48 8b 01                    	mov	rax, qword ptr [rcx]
1415f9376: 48 89 45 e8                 	mov	qword ptr [rbp - 0x18], rax
1415f937a: 48 8b 41 08                 	mov	rax, qword ptr [rcx + 0x8]
1415f937e: 48 89 45 e0                 	mov	qword ptr [rbp - 0x20], rax
1415f9382: 48 89 4d c8                 	mov	qword ptr [rbp - 0x38], rcx
1415f9386: 48 8b 41 10                 	mov	rax, qword ptr [rcx + 0x10]
1415f938a: 48 85 c0                    	test	rax, rax
1415f938d: 74 61                       	je	0x1415f93f0 <.text+0x15f83f0>
1415f938f: 4c 8b 7d e0                 	mov	r15, qword ptr [rbp - 0x20]
1415f9393: 49 8d 1c c7                 	lea	rbx, [r15 + 8*rax]
1415f9397: 48 8d 75 d4                 	lea	rsi, [rbp - 0x2c]
1415f939b: 49 be 01 00 00 00 01 00 00 00       	movabs	r14, 0x100000001
1415f93a5: 48 8d 7d b8                 	lea	rdi, [rbp - 0x48]
1415f93a9: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
1415f93b0: 41 8b 07                    	mov	eax, dword ptr [r15]
1415f93b3: 45 8b 67 04                 	mov	r12d, dword ptr [r15 + 0x4]
1415f93b7: 89 45 d4                    	mov	dword ptr [rbp - 0x2c], eax
1415f93ba: 44 89 65 d8                 	mov	dword ptr [rbp - 0x28], r12d
1415f93be: 48 8b 4d b8                 	mov	rcx, qword ptr [rbp - 0x48]
1415f93c2: 48 8b 55 c0                 	mov	rdx, qword ptr [rbp - 0x40]
1415f93c6: 41 b9 08 00 00 00           	mov	r9d, 0x8
1415f93cc: 49 89 f0                    	mov	r8, rsi
1415f93cf: e8 7c e6 0f 00              	call	0x1416f7a50 <.text+0x16f6a50>
1415f93d4: 44 89 65 d4                 	mov	dword ptr [rbp - 0x2c], r12d
1415f93d8: 4c 89 75 d8                 	mov	qword ptr [rbp - 0x28], r14
1415f93dc: 48 89 f9                    	mov	rcx, rdi
1415f93df: 48 89 f2                    	mov	rdx, rsi
1415f93e2: e8 b9 5b 37 ff              	call	0x14096efa0 <.text+0x96dfa0>
1415f93e7: 49 83 c7 08                 	add	r15, 0x8
1415f93eb: 49 39 df                    	cmp	r15, rbx
1415f93ee: 75 c0                       	jne	0x1415f93b0 <.text+0x15f83b0>
1415f93f0: 48 8b 55 e8                 	mov	rdx, qword ptr [rbp - 0x18]
1415f93f4: 48 85 d2                    	test	rdx, rdx
1415f93f7: 74 13                       	je	0x1415f940c <.text+0x15f840c>
1415f93f9: 48 c1 e2 03                 	shl	rdx, 0x3
1415f93fd: 41 b8 04 00 00 00           	mov	r8d, 0x4
1415f9403: 48 8b 4d e0                 	mov	rcx, qword ptr [rbp - 0x20]
1415f9407: e8 14 a8 01 ff              	call	0x140613c20 <.text+0x612c20>
1415f940c: 48 8b 45 c8                 	mov	rax, qword ptr [rbp - 0x38]
1415f9410: 48 8b 48 18                 	mov	rcx, qword ptr [rax + 0x18]
1415f9414: 48 89 4d e8                 	mov	qword ptr [rbp - 0x18], rcx
1415f9418: 48 8b 48 20                 	mov	rcx, qword ptr [rax + 0x20]
1415f941c: 48 89 4d e0                 	mov	qword ptr [rbp - 0x20], rcx
1415f9420: 48 8b 40 28                 	mov	rax, qword ptr [rax + 0x28]
1415f9424: 48 85 c0                    	test	rax, rax
1415f9427: 74 6f                       	je	0x1415f9498 <.text+0x15f8498>
1415f9429: 4c 8b 7d e0                 	mov	r15, qword ptr [rbp - 0x20]
1415f942d: 49 8d 1c c7                 	lea	rbx, [r15 + 8*rax]
1415f9431: 48 8d 75 d4                 	lea	rsi, [rbp - 0x2c]
1415f9435: 49 be 01 00 00 00 01 00 00 00       	movabs	r14, 0x100000001
1415f943f: 48 8d 7d b8                 	lea	rdi, [rbp - 0x48]
1415f9443: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
1415f9450: 41 8b 07                    	mov	eax, dword ptr [r15]
1415f9453: 45 8b 67 04                 	mov	r12d, dword ptr [r15 + 0x4]
1415f9457: 44 89 e1                    	mov	ecx, r12d
1415f945a: 81 c9 00 00 00 80           	or	ecx, 0x80000000
1415f9460: 89 45 d4                    	mov	dword ptr [rbp - 0x2c], eax
1415f9463: 89 4d d8                    	mov	dword ptr [rbp - 0x28], ecx
1415f9466: 48 8b 4d b8                 	mov	rcx, qword ptr [rbp - 0x48]
1415f946a: 48 8b 55 c0                 	mov	rdx, qword ptr [rbp - 0x40]
1415f946e: 41 b9 08 00 00 00           	mov	r9d, 0x8
1415f9474: 49 89 f0                    	mov	r8, rsi
1415f9477: e8 d4 e5 0f 00              	call	0x1416f7a50 <.text+0x16f6a50>
1415f947c: 44 89 65 d4                 	mov	dword ptr [rbp - 0x2c], r12d
1415f9480: 4c 89 75 d8                 	mov	qword ptr [rbp - 0x28], r14
1415f9484: 48 89 f9                    	mov	rcx, rdi
1415f9487: 48 89 f2                    	mov	rdx, rsi
1415f948a: e8 11 5b 37 ff              	call	0x14096efa0 <.text+0x96dfa0>
1415f948f: 49 83 c7 08                 	add	r15, 0x8
1415f9493: 49 39 df                    	cmp	r15, rbx
1415f9496: 75 b8                       	jne	0x1415f9450 <.text+0x15f8450>
1415f9498: 48 8b 55 e8                 	mov	rdx, qword ptr [rbp - 0x18]
1415f949c: 48 85 d2                    	test	rdx, rdx
1415f949f: 74 13                       	je	0x1415f94b4 <.text+0x15f84b4>
1415f94a1: 48 c1 e2 03                 	shl	rdx, 0x3
1415f94a5: 41 b8 04 00 00 00           	mov	r8d, 0x4
1415f94ab: 48 8b 4d e0                 	mov	rcx, qword ptr [rbp - 0x20]
1415f94af: e8 6c a7 01 ff              	call	0x140613c20 <.text+0x612c20>
1415f94b4: 90                          	nop
1415f94b5: 48 83 c4 70                 	add	rsp, 0x70
1415f94b9: 5b                          	pop	rbx
1415f94ba: 5f                          	pop	rdi
1415f94bb: 5e                          	pop	rsi
1415f94bc: 41 5c                       	pop	r12
1415f94be: 41 5e                       	pop	r14
1415f94c0: 41 5f                       	pop	r15
1415f94c2: 5d                          	pop	rbp
1415f94c3: c3                          	ret
1415f94c4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
1415f94d0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1415f94d5: 55                          	push	rbp
1415f94d6: 41 57                       	push	r15
1415f94d8: 41 56                       	push	r14
1415f94da: 41 54                       	push	r12
1415f94dc: 56                          	push	rsi
1415f94dd: 57                          	push	rdi
1415f94de: 53                          	push	rbx
1415f94df: 48 83 ec 20                 	sub	rsp, 0x20
1415f94e3: 48 8d 6a 70                 	lea	rbp, [rdx + 0x70]
1415f94e7: 48 83 7d e8 00              	cmp	qword ptr [rbp - 0x18], 0x0
1415f94ec: 74 17                       	je	0x1415f9505 <.text+0x15f8505>
1415f94ee: 48 8b 55 e8                 	mov	rdx, qword ptr [rbp - 0x18]
1415f94f2: 48 c1 e2 03                 	shl	rdx, 0x3
1415f94f6: 41 b8 04 00 00 00           	mov	r8d, 0x4
1415f94fc: 48 8b 4d e0                 	mov	rcx, qword ptr [rbp - 0x20]
1415f9500: e8 1b a7 01 ff              	call	0x140613c20 <.text+0x612c20>
1415f9505: c6 45 f7 00                 	mov	byte ptr [rbp - 0x9], 0x0
1415f9509: 48 83 c4 20                 	add	rsp, 0x20
1415f950d: 5b                          	pop	rbx
1415f950e: 5f                          	pop	rdi
1415f950f: 5e                          	pop	rsi
1415f9510: 41 5c                       	pop	r12
1415f9512: 41 5e                       	pop	r14
1415f9514: 41 5f                       	pop	r15
1415f9516: 5d                          	pop	rbp
1415f9517: c3                          	ret
1415f9518: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
1415f9520: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1415f9525: 55                          	push	rbp
1415f9526: 41 57                       	push	r15
1415f9528: 41 56                       	push	r14
1415f952a: 41 54                       	push	r12
1415f952c: 56                          	push	rsi
1415f952d: 57                          	push	rdi
1415f952e: 53                          	push	rbx
1415f952f: 48 83 ec 20                 	sub	rsp, 0x20
1415f9533: 48 8d 6a 70                 	lea	rbp, [rdx + 0x70]
1415f9537: 48 83 7d e8 00              	cmp	qword ptr [rbp - 0x18], 0x0
1415f953c: 74 17                       	je	0x1415f9555 <.text+0x15f8555>
1415f953e: 48 8b 55 e8                 	mov	rdx, qword ptr [rbp - 0x18]
1415f9542: 48 c1 e2 03                 	shl	rdx, 0x3
1415f9546: 41 b8 04 00 00 00           	mov	r8d, 0x4
1415f954c: 48 8b 4d e0                 	mov	rcx, qword ptr [rbp - 0x20]
1415f9550: e8 cb a6 01 ff              	call	0x140613c20 <.text+0x612c20>
1415f9555: c6 45 f7 01                 	mov	byte ptr [rbp - 0x9], 0x1
1415f9559: 48 83 c4 20                 	add	rsp, 0x20
1415f955d: 5b                          	pop	rbx
1415f955e: 5f                          	pop	rdi
1415f955f: 5e                          	pop	rsi
1415f9560: 41 5c                       	pop	r12
1415f9562: 41 5e                       	pop	r14
1415f9564: 41 5f                       	pop	r15
1415f9566: 5d                          	pop	rbp
1415f9567: c3                          	ret
1415f9568: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
1415f9570: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1415f9575: 55                          	push	rbp
1415f9576: 41 57                       	push	r15
1415f9578: 41 56                       	push	r14
1415f957a: 41 54                       	push	r12
1415f957c: 56                          	push	rsi
1415f957d: 57                          	push	rdi
1415f957e: 53                          	push	rbx
1415f957f: 48 83 ec 20                 	sub	rsp, 0x20
1415f9583: 48 8d 6a 70                 	lea	rbp, [rdx + 0x70]
1415f9587: 80 7d f7 00                 	cmp	byte ptr [rbp - 0x9], 0x0
1415f958b: 75 24                       	jne	0x1415f95b1 <.text+0x15f85b1>
1415f958d: 48 8b 45 c8                 	mov	rax, qword ptr [rbp - 0x38]
1415f9591: 48 8b 50 18                 	mov	rdx, qword ptr [rax + 0x18]
1415f9595: 48 85 d2                    	test	rdx, rdx
1415f9598: 74 17                       	je	0x1415f95b1 <.text+0x15f85b1>
1415f959a: 48 8b 45 c8                 	mov	rax, qword ptr [rbp - 0x38]
1415f959e: 48 8b 48 20                 	mov	rcx, qword ptr [rax + 0x20]
1415f95a2: 48 c1 e2 03                 	shl	rdx, 0x3
1415f95a6: 41 b8 04 00 00 00           	mov	r8d, 0x4
1415f95ac: e8 6f a6 01 ff              	call	0x140613c20 <.text+0x612c20>
1415f95b1: 90                          	nop
1415f95b2: 48 83 c4 20                 	add	rsp, 0x20
1415f95b6: 5b                          	pop	rbx
1415f95b7: 5f                          	pop	rdi
1415f95b8: 5e                          	pop	rsi
1415f95b9: 41 5c                       	pop	r12
1415f95bb: 41 5e                       	pop	r14
1415f95bd: 41 5f                       	pop	r15
1415f95bf: 5d                          	pop	rbp
1415f95c0: c3                          	ret
