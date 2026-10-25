
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a28f70: 55                          	push	rbp
140a28f71: 41 57                       	push	r15
140a28f73: 41 56                       	push	r14
140a28f75: 41 55                       	push	r13
140a28f77: 41 54                       	push	r12
140a28f79: 56                          	push	rsi
140a28f7a: 57                          	push	rdi
140a28f7b: 53                          	push	rbx
140a28f7c: 48 81 ec 88 06 00 00        	sub	rsp, 0x688
140a28f83: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
140a28f8b: 48 c7 85 00 06 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x600], -0x2
140a28f96: 89 d0                       	mov	eax, edx
140a28f98: 48 89 ce                    	mov	rsi, rcx
140a28f9b: 48 8b 9d 88 06 00 00        	mov	rbx, qword ptr [rbp + 0x688]
140a28fa2: 4c 8b b5 80 06 00 00        	mov	r14, qword ptr [rbp + 0x680]
140a28fa9: 48 8b bd 78 06 00 00        	mov	rdi, qword ptr [rbp + 0x678]
140a28fb0: 4c 8b bd 70 06 00 00        	mov	r15, qword ptr [rbp + 0x670]
140a28fb7: 4c 89 4c 24 28              	mov	qword ptr [rsp + 0x28], r9
140a28fbc: 4c 89 44 24 20              	mov	qword ptr [rsp + 0x20], r8
140a28fc1: 48 8d 15 b8 19 0a 02        	lea	rdx, [rip + 0x20a19b8]  # 0x142aca980
140a28fc8: 4c 8d a5 d8 02 00 00        	lea	r12, [rbp + 0x2d8]
140a28fcf: 41 b8 12 00 00 00           	mov	r8d, 0x12
140a28fd5: 4c 89 e1                    	mov	rcx, r12
140a28fd8: 41 89 c1                    	mov	r9d, eax
140a28fdb: e8 30 3f bd 00              	call	0x1415fcf10 <.text+0x15fbf10>
140a28fe0: 4c 8d ad 68 04 00 00        	lea	r13, [rbp + 0x468]
140a28fe7: 49 b8 4d f3 05 c9 b1 38 6c 68       	movabs	r8, 0x686c38b1c905f34d
140a28ff1: 4c 89 e9                    	mov	rcx, r13
140a28ff4: 4c 89 e2                    	mov	rdx, r12
140a28ff7: 41 b9 01 00 00 00           	mov	r9d, 0x1
140a28ffd: e8 1e fa bd 00              	call	0x141608a20 <.text+0x1607a20>
140a29002: 49 b8 9b ec 49 c2 df d9 05 d0       	movabs	r8, -0x2ffa26203db61365
140a2900c: 4c 89 e1                    	mov	rcx, r12
140a2900f: 4c 89 ea                    	mov	rdx, r13
140a29012: 41 b9 01 00 00 00           	mov	r9d, 0x1
140a29018: e8 43 f2 bd 00              	call	0x141608260 <.text+0x1607260>
140a2901d: c7 44 24 20 01 00 00 00     	mov	dword ptr [rsp + 0x20], 0x1
140a29025: 49 b8 3b 42 1d a4 2e da fa 77       	movabs	r8, 0x77fada2ea41d423b
140a2902f: 4c 89 e9                    	mov	rcx, r13
140a29032: 4c 89 e2                    	mov	rdx, r12
140a29035: 4d 89 f9                    	mov	r9, r15
140a29038: e8 c3 4f f4 ff              	call	0x14096e000 <.text+0x96d000>
140a2903d: c7 44 24 20 01 00 00 00     	mov	dword ptr [rsp + 0x20], 0x1
140a29045: 4c 8d bd d8 02 00 00        	lea	r15, [rbp + 0x2d8]
140a2904c: 49 b8 75 a5 7b 13 be 93 b5 7a       	movabs	r8, 0x7ab593be137ba575
140a29056: 4c 89 f9                    	mov	rcx, r15
140a29059: 4c 89 ea                    	mov	rdx, r13
140a2905c: 49 89 f9                    	mov	r9, rdi
140a2905f: e8 9c 4f f4 ff              	call	0x14096e000 <.text+0x96d000>
140a29064: 48 8d bd 68 04 00 00        	lea	rdi, [rbp + 0x468]
140a2906b: 49 b8 bd cc bd d5 b2 ae 75 2a       	movabs	r8, 0x2a75aeb2d5bdccbd
140a29075: 48 89 f9                    	mov	rcx, rdi
140a29078: 4c 89 fa                    	mov	rdx, r15
140a2907b: 4d 89 f1                    	mov	r9, r14
140a2907e: e8 1d 52 f4 ff              	call	0x14096e2a0 <.text+0x96d2a0>
140a29083: 48 8d 4d b8                 	lea	rcx, [rbp - 0x48]
140a29087: 49 b8 f1 97 e4 bf 72 72 8a 66       	movabs	r8, 0x668a7272bfe497f1
140a29091: 48 89 fa                    	mov	rdx, rdi
140a29094: 49 89 d9                    	mov	r9, rbx
140a29097: e8 04 52 f4 ff              	call	0x14096e2a0 <.text+0x96d2a0>
140a2909c: 48 8d 9d d8 02 00 00        	lea	rbx, [rbp + 0x2d8]
140a290a3: 48 8d 55 b8                 	lea	rdx, [rbp - 0x48]
140a290a7: 49 b8 ed b8 c7 43 bc 36 ab 35       	movabs	r8, 0x35ab36bc43c7b8ed
140a290b1: 48 89 d9                    	mov	rcx, rbx
140a290b4: 83 bd 90 06 00 00 00        	cmp	dword ptr [rbp + 0x690], 0x0
140a290bb: 74 35                       	je	0x140a290f2 <.text+0xa280f2>
140a290bd: e8 ae 3c be 00              	call	0x14160cd70 <.text+0x160bd70>
140a290c2: 48 8d 8d 48 01 00 00        	lea	rcx, [rbp + 0x148]
140a290c9: 49 b8 c5 67 7f 3b 90 a9 83 37       	movabs	r8, 0x3783a9903b7f67c5
140a290d3: 48 89 da                    	mov	rdx, rbx
140a290d6: 41 b9 01 00 00 00           	mov	r9d, 0x1
140a290dc: e8 ff 00 be 00              	call	0x1416091e0 <.text+0x16081e0>
140a290e1: 4c 8b b5 98 06 00 00        	mov	r14, qword ptr [rbp + 0x698]
140a290e8: 4d 85 f6                    	test	r14, r14
140a290eb: 75 39                       	jne	0x140a29126 <.text+0xa28126>
140a290ed: e9 df 00 00 00              	jmp	0x140a291d1 <.text+0xa281d1>
140a290f2: e8 99 4e be 00              	call	0x14160df90 <.text+0x160cf90>
140a290f7: 48 8d 8d 48 01 00 00        	lea	rcx, [rbp + 0x148]
140a290fe: 49 b8 c5 67 7f 3b 90 a9 83 37       	movabs	r8, 0x3783a9903b7f67c5
140a29108: 48 89 da                    	mov	rdx, rbx
140a2910b: 41 b9 01 00 00 00           	mov	r9d, 0x1
140a29111: e8 ea fc bd 00              	call	0x141608e00 <.text+0x1607e00>
140a29116: 4c 8b b5 98 06 00 00        	mov	r14, qword ptr [rbp + 0x698]
140a2911d: 4d 85 f6                    	test	r14, r14
140a29120: 0f 84 ab 00 00 00           	je	0x140a291d1 <.text+0xa281d1>
140a29126: 48 8d 95 48 01 00 00        	lea	rdx, [rbp + 0x148]
140a2912d: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a29133: 48 89 d9                    	mov	rcx, rbx
140a29136: e8 c5 aa ef 01              	call	0x142923c00 <.text+0x2922c00>
140a2913b: 49 8d 46 08                 	lea	rax, [r14 + 0x8]
140a2913f: 48 89 85 f8 05 00 00        	mov	qword ptr [rbp + 0x5f8], rax
140a29146: c7 85 68 04 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x468], 0x2
140a29150: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
140a29155: 48 8d 8d f8 05 00 00        	lea	rcx, [rbp + 0x5f8]
140a2915c: 49 b8 69 a5 a8 89 2f 81 b6 62       	movabs	r8, 0x62b6812f89a8a569
140a29166: 48 89 da                    	mov	rdx, rbx
140a29169: 41 b1 19                    	mov	r9b, 0x19
140a2916c: e8 cf 50 ec ff              	call	0x1408ee240 <.text+0x8ed240>
140a29171: 48 8d 8d 68 04 00 00        	lea	rcx, [rbp + 0x468]
140a29178: 48 8d 95 d8 02 00 00        	lea	rdx, [rbp + 0x2d8]
140a2917f: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a29185: e8 76 aa ef 01              	call	0x142923c00 <.text+0x2922c00>
140a2918a: 80 bd 9e 05 00 00 08        	cmp	byte ptr [rbp + 0x59e], 0x8
140a29191: 74 25                       	je	0x140a291b8 <.text+0xa281b8>
140a29193: 4d 8b 0e                    	mov	r9, qword ptr [r14]
140a29196: 48 8d 8d 78 05 00 00        	lea	rcx, [rbp + 0x578]
140a2919d: ba e8 00 00 00              	mov	edx, 0xe8
140a291a2: 48 03 95 a0 05 00 00        	add	rdx, qword ptr [rbp + 0x5a0]
140a291a9: 49 b8 d3 41 03 96 13 36 d2 e8       	movabs	r8, -0x172dc9ec69fcbe2d
140a291b3: e8 a8 25 f4 ff              	call	0x14096b760 <.text+0x96a760>
140a291b8: 48 8d 8d 48 01 00 00        	lea	rcx, [rbp + 0x148]
140a291bf: 48 8d 95 68 04 00 00        	lea	rdx, [rbp + 0x468]
140a291c6: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a291cc: e8 2f aa ef 01              	call	0x142923c00 <.text+0x2922c00>
140a291d1: 48 8d 95 48 01 00 00        	lea	rdx, [rbp + 0x148]
140a291d8: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a291de: 48 89 f1                    	mov	rcx, rsi
140a291e1: e8 1a aa ef 01              	call	0x142923c00 <.text+0x2922c00>
140a291e6: 48 89 f0                    	mov	rax, rsi
140a291e9: 48 81 c4 88 06 00 00        	add	rsp, 0x688
140a291f0: 5b                          	pop	rbx
140a291f1: 5f                          	pop	rdi
140a291f2: 5e                          	pop	rsi
140a291f3: 41 5c                       	pop	r12
140a291f5: 41 5d                       	pop	r13
140a291f7: 41 5e                       	pop	r14
140a291f9: 41 5f                       	pop	r15
140a291fb: 5d                          	pop	rbp
140a291fc: c3                          	ret
140a291fd: 0f 1f 00                    	nop	dword ptr [rax]
140a29200: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a29205: 55                          	push	rbp
140a29206: 41 57                       	push	r15
140a29208: 41 56                       	push	r14
140a2920a: 41 55                       	push	r13
140a2920c: 41 54                       	push	r12
140a2920e: 56                          	push	rsi
140a2920f: 57                          	push	rdi
140a29210: 53                          	push	rbx
140a29211: 48 83 ec 38                 	sub	rsp, 0x38
140a29215: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a2921c: 48 8d 8d d8 02 00 00        	lea	rcx, [rbp + 0x2d8]
140a29223: e8 88 dd e6 ff              	call	0x140896fb0 <.text+0x895fb0>
140a29228: 90                          	nop
140a29229: 48 83 c4 38                 	add	rsp, 0x38
140a2922d: 5b                          	pop	rbx
140a2922e: 5f                          	pop	rdi
140a2922f: 5e                          	pop	rsi
140a29230: 41 5c                       	pop	r12
140a29232: 41 5d                       	pop	r13
140a29234: 41 5e                       	pop	r14
140a29236: 41 5f                       	pop	r15
140a29238: 5d                          	pop	rbp
140a29239: c3                          	ret
140a2923a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
140a29240: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a29245: 55                          	push	rbp
140a29246: 41 57                       	push	r15
140a29248: 41 56                       	push	r14
140a2924a: 41 55                       	push	r13
140a2924c: 41 54                       	push	r12
140a2924e: 56                          	push	rsi
140a2924f: 57                          	push	rdi
140a29250: 53                          	push	rbx
140a29251: 48 83 ec 38                 	sub	rsp, 0x38
140a29255: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a2925c: 48 8d 8d 68 04 00 00        	lea	rcx, [rbp + 0x468]
140a29263: e8 48 dd e6 ff              	call	0x140896fb0 <.text+0x895fb0>
140a29268: 90                          	nop
140a29269: 48 83 c4 38                 	add	rsp, 0x38
140a2926d: 5b                          	pop	rbx
140a2926e: 5f                          	pop	rdi
140a2926f: 5e                          	pop	rsi
140a29270: 41 5c                       	pop	r12
140a29272: 41 5d                       	pop	r13
140a29274: 41 5e                       	pop	r14
140a29276: 41 5f                       	pop	r15
140a29278: 5d                          	pop	rbp
140a29279: c3                          	ret
