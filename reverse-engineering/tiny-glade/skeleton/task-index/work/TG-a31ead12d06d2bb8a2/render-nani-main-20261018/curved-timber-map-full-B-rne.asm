
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1414783b0: 48 83 ec 28                 	sub	rsp, 0x28
1414783b4: 0f 29 7c 24 10              	movaps	xmmword ptr [rsp + 0x10], xmm7
1414783b9: 0f 29 34 24                 	movaps	xmmword ptr [rsp], xmm6
1414783bd: 8b 41 08                    	mov	eax, dword ptr [rcx + 0x8]
1414783c0: 44 8b 41 0c                 	mov	r8d, dword ptr [rcx + 0xc]
1414783c4: 44 39 c0                    	cmp	eax, r8d
1414783c7: 0f 83 c4 00 00 00           	jae	0x141478491 <.text+0x1477491>
1414783cd: 48 8b 09                    	mov	rcx, qword ptr [rcx]
1414783d0: 8b 09                       	mov	ecx, dword ptr [rcx]
1414783d2: f3 48 0f 2a c1              	cvtsi2ss	xmm0, rcx
1414783d7: 48 8b 4a 08                 	mov	rcx, qword ptr [rdx + 0x8]
1414783db: 4c 8b 4a 10                 	mov	r9, qword ptr [rdx + 0x10]
1414783df: 4c 8d 14 49                 	lea	r10, [rcx + 2*rcx]
1414783e3: 4f 8d 4c 91 50              	lea	r9, [r9 + 4*r10 + 0x50]
1414783e8: f3 0f 10 0d d8 13 7a 01     	movss	xmm1, dword ptr [rip + 0x17a13d8] # 0x142c197c8
1414783f0: f3 0f 10 15 38 df 60 01     	movss	xmm2, dword ptr [rip + 0x160df38] # 0x142a86330
1414783f8: f3 0f 10 1d 70 d4 4a 01     	movss	xmm3, dword ptr [rip + 0x14ad470] # 0x142925870
141478400: f3 0f 10 25 b0 64 6a 01     	movss	xmm4, dword ptr [rip + 0x16a64b0] # 0x142b1e8b8
141478408: f3 0f 10 2d bc 13 7a 01     	movss	xmm5, dword ptr [rip + 0x17a13bc] # 0x142c197cc
141478410: f3 0f 10 35 ec d4 4a 01     	movss	xmm6, dword ptr [rip + 0x14ad4ec] # 0x142925904
141478418: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
141478420: 41 89 c2                    	mov	r10d, eax
141478423: 0f 57 ff                    	xorps	xmm7, xmm7
141478426: f3 49 0f 2a fa              	cvtsi2ss	xmm7, r10
14147842b: ff c0                       	inc	eax
14147842d: f3 0f 5e f8                 	divss	xmm7, xmm0
141478431: 49 c7 41 b0 00 00 00 00     	mov	qword ptr [r9 - 0x50], 0x0
141478439: f3 41 0f 11 79 b8           	movss	dword ptr [r9 - 0x48], xmm7
14147843f: 41 0f 13 49 bc              	movlps	qword ptr [r9 - 0x44], xmm1
141478444: f3 41 0f 11 79 c4           	movss	dword ptr [r9 - 0x3c], xmm7
14147844a: 41 0f 13 51 c8              	movlps	qword ptr [r9 - 0x38], xmm2
14147844f: f3 41 0f 11 79 d0           	movss	dword ptr [r9 - 0x30], xmm7
141478455: 41 0f 13 59 d4              	movlps	qword ptr [r9 - 0x2c], xmm3
14147845a: f3 41 0f 11 79 dc           	movss	dword ptr [r9 - 0x24], xmm7
141478460: 41 0f 13 61 e0              	movlps	qword ptr [r9 - 0x20], xmm4
141478465: f3 41 0f 11 79 e8           	movss	dword ptr [r9 - 0x18], xmm7
14147846b: 41 0f 13 69 ec              	movlps	qword ptr [r9 - 0x14], xmm5
141478470: f3 41 0f 11 79 f4           	movss	dword ptr [r9 - 0xc], xmm7
141478476: 41 0f 13 71 f8              	movlps	qword ptr [r9 - 0x8], xmm6
14147847b: f3 41 0f 11 39              	movss	dword ptr [r9], xmm7
141478480: 48 83 c1 07                 	add	rcx, 0x7
141478484: 49 83 c1 54                 	add	r9, 0x54
141478488: 41 39 c0                    	cmp	r8d, eax
14147848b: 75 93                       	jne	0x141478420 <.text+0x1477420>
14147848d: 48 89 4a 08                 	mov	qword ptr [rdx + 0x8], rcx
141478491: 0f 28 34 24                 	movaps	xmm6, xmmword ptr [rsp]
141478495: 0f 28 7c 24 10              	movaps	xmm7, xmmword ptr [rsp + 0x10]
14147849a: 48 83 c4 28                 	add	rsp, 0x28
14147849e: c3                          	ret
