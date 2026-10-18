
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a28630: 55                          	push	rbp
140a28631: 41 57                       	push	r15
140a28633: 41 56                       	push	r14
140a28635: 41 55                       	push	r13
140a28637: 41 54                       	push	r12
140a28639: 56                          	push	rsi
140a2863a: 57                          	push	rdi
140a2863b: 53                          	push	rbx
140a2863c: 48 81 ec 88 06 00 00        	sub	rsp, 0x688
140a28643: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
140a2864b: 48 c7 85 00 06 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x600], -0x2
140a28656: 48 89 d0                    	mov	rax, rdx
140a28659: 48 89 ce                    	mov	rsi, rcx
140a2865c: 48 8b 9d 90 06 00 00        	mov	rbx, qword ptr [rbp + 0x690]
140a28663: 4c 8b b5 88 06 00 00        	mov	r14, qword ptr [rbp + 0x688]
140a2866a: 48 8b bd 80 06 00 00        	mov	rdi, qword ptr [rbp + 0x680]
140a28671: 4c 8b bd 78 06 00 00        	mov	r15, qword ptr [rbp + 0x678]
140a28678: 48 8b 8d 70 06 00 00        	mov	rcx, qword ptr [rbp + 0x670]
140a2867f: 48 89 4c 24 30              	mov	qword ptr [rsp + 0x30], rcx
140a28684: 4c 89 4c 24 28              	mov	qword ptr [rsp + 0x28], r9
140a28689: 4c 89 44 24 20              	mov	qword ptr [rsp + 0x20], r8
140a2868e: 48 8d 15 eb 22 0a 02        	lea	rdx, [rip + 0x20a22eb]  # 0x142aca980
140a28695: 4c 8d a5 d8 02 00 00        	lea	r12, [rbp + 0x2d8]
140a2869c: 41 b8 12 00 00 00           	mov	r8d, 0x12
140a286a2: 4c 89 e1                    	mov	rcx, r12
140a286a5: 49 89 c1                    	mov	r9, rax
140a286a8: e8 c3 7e bd 00              	call	0x141600570 <.text+0x15ff570>
140a286ad: 4c 8d ad 68 04 00 00        	lea	r13, [rbp + 0x468]
140a286b4: 49 b8 4d f3 05 c9 b1 38 6c 68       	movabs	r8, 0x686c38b1c905f34d
140a286be: 4c 89 e9                    	mov	rcx, r13
140a286c1: 4c 89 e2                    	mov	rdx, r12
140a286c4: 41 b9 01 00 00 00           	mov	r9d, 0x1
140a286ca: e8 51 03 be 00              	call	0x141608a20 <.text+0x1607a20>
140a286cf: 49 b8 9b ec 49 c2 df d9 05 d0       	movabs	r8, -0x2ffa26203db61365
140a286d9: 4c 89 e1                    	mov	rcx, r12
140a286dc: 4c 89 ea                    	mov	rdx, r13
140a286df: 41 b9 01 00 00 00           	mov	r9d, 0x1
140a286e5: e8 76 fb bd 00              	call	0x141608260 <.text+0x1607260>
140a286ea: c7 44 24 20 01 00 00 00     	mov	dword ptr [rsp + 0x20], 0x1
140a286f2: 49 b8 3b 42 1d a4 2e da fa 77       	movabs	r8, 0x77fada2ea41d423b
140a286fc: 4c 89 e9                    	mov	rcx, r13
140a286ff: 4c 89 e2                    	mov	rdx, r12
140a28702: 4d 89 f9                    	mov	r9, r15
140a28705: e8 f6 58 f4 ff              	call	0x14096e000 <.text+0x96d000>
140a2870a: c7 44 24 20 01 00 00 00     	mov	dword ptr [rsp + 0x20], 0x1
140a28712: 4c 8d bd d8 02 00 00        	lea	r15, [rbp + 0x2d8]
140a28719: 49 b8 75 a5 7b 13 be 93 b5 7a       	movabs	r8, 0x7ab593be137ba575
140a28723: 4c 89 f9                    	mov	rcx, r15
140a28726: 4c 89 ea                    	mov	rdx, r13
140a28729: 49 89 f9                    	mov	r9, rdi
140a2872c: e8 cf 58 f4 ff              	call	0x14096e000 <.text+0x96d000>
140a28731: 48 8d bd 68 04 00 00        	lea	rdi, [rbp + 0x468]
140a28738: 49 b8 bd cc bd d5 b2 ae 75 2a       	movabs	r8, 0x2a75aeb2d5bdccbd
140a28742: 48 89 f9                    	mov	rcx, rdi
140a28745: 4c 89 fa                    	mov	rdx, r15
140a28748: 4d 89 f1                    	mov	r9, r14
140a2874b: e8 50 5b f4 ff              	call	0x14096e2a0 <.text+0x96d2a0>
140a28750: 48 8d 4d b8                 	lea	rcx, [rbp - 0x48]
140a28754: 49 b8 f1 97 e4 bf 72 72 8a 66       	movabs	r8, 0x668a7272bfe497f1
140a2875e: 48 89 fa                    	mov	rdx, rdi
140a28761: 49 89 d9                    	mov	r9, rbx
140a28764: e8 37 5b f4 ff              	call	0x14096e2a0 <.text+0x96d2a0>
140a28769: 48 8d 9d d8 02 00 00        	lea	rbx, [rbp + 0x2d8]
140a28770: 48 8d 55 b8                 	lea	rdx, [rbp - 0x48]
140a28774: 49 b8 ed b8 c7 43 bc 36 ab 35       	movabs	r8, 0x35ab36bc43c7b8ed
140a2877e: 48 89 d9                    	mov	rcx, rbx
140a28781: 83 bd 98 06 00 00 00        	cmp	dword ptr [rbp + 0x698], 0x0
140a28788: 74 35                       	je	0x140a287bf <.text+0xa277bf>
140a2878a: e8 e1 45 be 00              	call	0x14160cd70 <.text+0x160bd70>
140a2878f: 48 8d 8d 48 01 00 00        	lea	rcx, [rbp + 0x148]
140a28796: 49 b8 c5 67 7f 3b 90 a9 83 37       	movabs	r8, 0x3783a9903b7f67c5
140a287a0: 48 89 da                    	mov	rdx, rbx
140a287a3: 41 b9 01 00 00 00           	mov	r9d, 0x1
140a287a9: e8 32 0a be 00              	call	0x1416091e0 <.text+0x16081e0>
140a287ae: 4c 8b b5 a0 06 00 00        	mov	r14, qword ptr [rbp + 0x6a0]
140a287b5: 4d 85 f6                    	test	r14, r14
140a287b8: 75 39                       	jne	0x140a287f3 <.text+0xa277f3>
140a287ba: e9 df 00 00 00              	jmp	0x140a2889e <.text+0xa2789e>
140a287bf: e8 cc 57 be 00              	call	0x14160df90 <.text+0x160cf90>
140a287c4: 48 8d 8d 48 01 00 00        	lea	rcx, [rbp + 0x148]
140a287cb: 49 b8 c5 67 7f 3b 90 a9 83 37       	movabs	r8, 0x3783a9903b7f67c5
140a287d5: 48 89 da                    	mov	rdx, rbx
140a287d8: 41 b9 01 00 00 00           	mov	r9d, 0x1
140a287de: e8 1d 06 be 00              	call	0x141608e00 <.text+0x1607e00>
140a287e3: 4c 8b b5 a0 06 00 00        	mov	r14, qword ptr [rbp + 0x6a0]
140a287ea: 4d 85 f6                    	test	r14, r14
140a287ed: 0f 84 ab 00 00 00           	je	0x140a2889e <.text+0xa2789e>
140a287f3: 48 8d 95 48 01 00 00        	lea	rdx, [rbp + 0x148]
140a287fa: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a28800: 48 89 d9                    	mov	rcx, rbx
140a28803: e8 f8 b3 ef 01              	call	0x142923c00 <.text+0x2922c00>
140a28808: 49 8d 46 08                 	lea	rax, [r14 + 0x8]
140a2880c: 48 89 85 f8 05 00 00        	mov	qword ptr [rbp + 0x5f8], rax
140a28813: c7 85 68 04 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x468], 0x2
140a2881d: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
140a28822: 48 8d 8d f8 05 00 00        	lea	rcx, [rbp + 0x5f8]
140a28829: 49 b8 69 a5 a8 89 2f 81 b6 62       	movabs	r8, 0x62b6812f89a8a569
140a28833: 48 89 da                    	mov	rdx, rbx
140a28836: 41 b1 19                    	mov	r9b, 0x19
140a28839: e8 02 5a ec ff              	call	0x1408ee240 <.text+0x8ed240>
140a2883e: 48 8d 8d 68 04 00 00        	lea	rcx, [rbp + 0x468]
140a28845: 48 8d 95 d8 02 00 00        	lea	rdx, [rbp + 0x2d8]
140a2884c: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a28852: e8 a9 b3 ef 01              	call	0x142923c00 <.text+0x2922c00>
140a28857: 80 bd 9e 05 00 00 08        	cmp	byte ptr [rbp + 0x59e], 0x8
140a2885e: 74 25                       	je	0x140a28885 <.text+0xa27885>
140a28860: 4d 8b 0e                    	mov	r9, qword ptr [r14]
140a28863: 48 8d 8d 78 05 00 00        	lea	rcx, [rbp + 0x578]
140a2886a: ba e8 00 00 00              	mov	edx, 0xe8
140a2886f: 48 03 95 a0 05 00 00        	add	rdx, qword ptr [rbp + 0x5a0]
140a28876: 49 b8 d3 41 03 96 13 36 d2 e8       	movabs	r8, -0x172dc9ec69fcbe2d
140a28880: e8 db 2e f4 ff              	call	0x14096b760 <.text+0x96a760>
140a28885: 48 8d 8d 48 01 00 00        	lea	rcx, [rbp + 0x148]
140a2888c: 48 8d 95 68 04 00 00        	lea	rdx, [rbp + 0x468]
140a28893: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a28899: e8 62 b3 ef 01              	call	0x142923c00 <.text+0x2922c00>
140a2889e: 48 8d 95 48 01 00 00        	lea	rdx, [rbp + 0x148]
140a288a5: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a288ab: 48 89 f1                    	mov	rcx, rsi
140a288ae: e8 4d b3 ef 01              	call	0x142923c00 <.text+0x2922c00>
140a288b3: 48 89 f0                    	mov	rax, rsi
140a288b6: 48 81 c4 88 06 00 00        	add	rsp, 0x688
140a288bd: 5b                          	pop	rbx
140a288be: 5f                          	pop	rdi
140a288bf: 5e                          	pop	rsi
140a288c0: 41 5c                       	pop	r12
140a288c2: 41 5d                       	pop	r13
140a288c4: 41 5e                       	pop	r14
140a288c6: 41 5f                       	pop	r15
140a288c8: 5d                          	pop	rbp
140a288c9: c3                          	ret
140a288ca: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
140a288d0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a288d5: 55                          	push	rbp
140a288d6: 41 57                       	push	r15
140a288d8: 41 56                       	push	r14
140a288da: 41 55                       	push	r13
140a288dc: 41 54                       	push	r12
140a288de: 56                          	push	rsi
140a288df: 57                          	push	rdi
140a288e0: 53                          	push	rbx
140a288e1: 48 83 ec 38                 	sub	rsp, 0x38
140a288e5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a288ec: 48 8d 8d d8 02 00 00        	lea	rcx, [rbp + 0x2d8]
140a288f3: e8 b8 e6 e6 ff              	call	0x140896fb0 <.text+0x895fb0>
140a288f8: 90                          	nop
140a288f9: 48 83 c4 38                 	add	rsp, 0x38
140a288fd: 5b                          	pop	rbx
140a288fe: 5f                          	pop	rdi
140a288ff: 5e                          	pop	rsi
140a28900: 41 5c                       	pop	r12
140a28902: 41 5d                       	pop	r13
140a28904: 41 5e                       	pop	r14
140a28906: 41 5f                       	pop	r15
140a28908: 5d                          	pop	rbp
140a28909: c3                          	ret
140a2890a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
140a28910: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a28915: 55                          	push	rbp
140a28916: 41 57                       	push	r15
140a28918: 41 56                       	push	r14
140a2891a: 41 55                       	push	r13
140a2891c: 41 54                       	push	r12
140a2891e: 56                          	push	rsi
140a2891f: 57                          	push	rdi
140a28920: 53                          	push	rbx
140a28921: 48 83 ec 38                 	sub	rsp, 0x38
140a28925: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a2892c: 48 8d 8d 68 04 00 00        	lea	rcx, [rbp + 0x468]
140a28933: e8 78 e6 e6 ff              	call	0x140896fb0 <.text+0x895fb0>
140a28938: 90                          	nop
140a28939: 48 83 c4 38                 	add	rsp, 0x38
140a2893d: 5b                          	pop	rbx
140a2893e: 5f                          	pop	rdi
140a2893f: 5e                          	pop	rsi
140a28940: 41 5c                       	pop	r12
140a28942: 41 5d                       	pop	r13
140a28944: 41 5e                       	pop	r14
140a28946: 41 5f                       	pop	r15
140a28948: 5d                          	pop	rbp
140a28949: c3                          	ret
