
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a29280: 41 57                       	push	r15
140a29282: 41 56                       	push	r14
140a29284: 41 55                       	push	r13
140a29286: 41 54                       	push	r12
140a29288: 56                          	push	rsi
140a29289: 57                          	push	rdi
140a2928a: 55                          	push	rbp
140a2928b: 53                          	push	rbx
140a2928c: 48 81 ec 68 0b 00 00        	sub	rsp, 0xb68
140a29293: 4c 89 c3                    	mov	rbx, r8
140a29296: 4c 89 44 24 58              	mov	qword ptr [rsp + 0x58], r8
140a2929b: 49 89 d5                    	mov	r13, rdx
140a2929e: 48 89 4c 24 68              	mov	qword ptr [rsp + 0x68], rcx
140a292a3: 48 8b bc 24 f0 0b 00 00     	mov	rdi, qword ptr [rsp + 0xbf0]
140a292ab: 48 8b b4 24 f8 0b 00 00     	mov	rsi, qword ptr [rsp + 0xbf8]
140a292b3: 4c 8b b4 24 e8 0b 00 00     	mov	r14, qword ptr [rsp + 0xbe8]
140a292bb: 49 8b 81 50 14 00 00        	mov	rax, qword ptr [r9 + 0x1450]
140a292c2: 48 89 44 24 60              	mov	qword ptr [rsp + 0x60], rax
140a292c7: 8b 50 60                    	mov	edx, dword ptr [rax + 0x60]
140a292ca: 48 c1 e2 02                 	shl	rdx, 0x2
140a292ce: 48 8d 8c 24 98 03 00 00     	lea	rcx, [rsp + 0x398]
140a292d6: 41 b8 20 00 00 00           	mov	r8d, 0x20
140a292dc: e8 3f bb 1a 00              	call	0x140bd4e20 <.text+0xbd3e20>
140a292e1: 0f 10 84 24 98 03 00 00     	movups	xmm0, xmmword ptr [rsp + 0x398]
140a292e9: 0f 29 44 24 70              	movaps	xmmword ptr [rsp + 0x70], xmm0
140a292ee: 48 8d 8c 24 18 02 00 00     	lea	rcx, [rsp + 0x218]
140a292f6: 48 8d 54 24 70              	lea	rdx, [rsp + 0x70]
140a292fb: e8 00 da 1a 00              	call	0x140bd6d00 <.text+0xbd5d00>
140a29300: 48 8d 05 8b 16 0a 02        	lea	rax, [rip + 0x20a168b]  # 0x142aca992
140a29307: 48 89 84 24 08 02 00 00     	mov	qword ptr [rsp + 0x208], rax
140a2930f: 48 c7 84 24 10 02 00 00 15 00 00 00 	mov	qword ptr [rsp + 0x210], 0x15
140a2931b: 48 8d 94 24 08 02 00 00     	lea	rdx, [rsp + 0x208]
140a29323: 4c 89 e9                    	mov	rcx, r13
140a29326: e8 a5 01 1c 00              	call	0x140be94d0 <.text+0xbe84d0>
140a2932b: 0f 10 84 24 98 03 00 00     	movups	xmm0, xmmword ptr [rsp + 0x398]
140a29333: 0f 29 44 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm0
140a29338: 89 44 24 50                 	mov	dword ptr [rsp + 0x50], eax
140a2933c: e8 0f 3b 1c 00              	call	0x140bece50 <.text+0xbebe50>
140a29341: 89 44 24 3c                 	mov	dword ptr [rsp + 0x3c], eax
140a29345: 4c 89 6c 24 30              	mov	qword ptr [rsp + 0x30], r13
140a2934a: 48 89 5c 24 28              	mov	qword ptr [rsp + 0x28], rbx
140a2934f: 48 c7 44 24 20 29 00 00 00  	mov	qword ptr [rsp + 0x20], 0x29
140a29358: 48 8d 15 48 16 0a 02        	lea	rdx, [rip + 0x20a1648]  # 0x142aca9a7
140a2935f: 4c 8d 0d 61 16 0a 02        	lea	r9, [rip + 0x20a1661]   # 0x142aca9c7
140a29366: 48 8d 9c 24 08 02 00 00     	lea	rbx, [rsp + 0x208]
140a2936e: 41 b8 20 00 00 00           	mov	r8d, 0x20
140a29374: 48 89 d9                    	mov	rcx, rbx
140a29377: e8 24 9f bd 00              	call	0x1416032a0 <.text+0x16022a0>
140a2937c: 4c 8d 7c 24 70              	lea	r15, [rsp + 0x70]
140a29381: 49 b8 e3 c5 90 19 c5 9d fe 86       	movabs	r8, -0x7901623ae66f3a1d
140a2938b: 4c 89 f9                    	mov	rcx, r15
140a2938e: 48 89 da                    	mov	rdx, rbx
140a29391: e8 7a 3d be 00              	call	0x14160d110 <.text+0x160c110>
140a29396: 48 8d 9c 24 98 03 00 00     	lea	rbx, [rsp + 0x398]
140a2939e: 49 b8 84 ac 8a 23 de bb 37 53       	movabs	r8, 0x5337bbde238aac84
140a293a8: 48 89 d9                    	mov	rcx, rbx
140a293ab: 4c 89 fa                    	mov	rdx, r15
140a293ae: 4d 89 f1                    	mov	r9, r14
140a293b1: e8 0a 47 f4 ff              	call	0x14096dac0 <.text+0x96cac0>
140a293b6: 4c 8d bc 24 d8 09 00 00     	lea	r15, [rsp + 0x9d8]
140a293be: 49 b8 9b ec 49 c2 df d9 05 d0       	movabs	r8, -0x2ffa26203db61365
140a293c8: 4c 89 f9                    	mov	rcx, r15
140a293cb: 48 89 da                    	mov	rdx, rbx
140a293ce: e8 5d 32 be 00              	call	0x14160c630 <.text+0x160b630>
140a293d3: 4c 8d b4 24 48 08 00 00     	lea	r14, [rsp + 0x848]
140a293db: 49 b8 3b 42 1d a4 2e da fa 77       	movabs	r8, 0x77fada2ea41d423b
140a293e5: 4c 89 f1                    	mov	rcx, r14
140a293e8: 4c 89 fa                    	mov	rdx, r15
140a293eb: 49 89 f1                    	mov	r9, rsi
140a293ee: e8 cd 46 f4 ff              	call	0x14096dac0 <.text+0x96cac0>
140a293f3: c7 44 24 20 01 00 00 00     	mov	dword ptr [rsp + 0x20], 0x1
140a293fb: 48 8d b4 24 b8 06 00 00     	lea	rsi, [rsp + 0x6b8]
140a29403: 49 b8 01 f1 2c d7 c9 c4 e3 fe       	movabs	r8, -0x11c3b3628d30eff
140a2940d: 48 89 f1                    	mov	rcx, rsi
140a29410: 4c 89 f2                    	mov	rdx, r14
140a29413: 49 89 f9                    	mov	r9, rdi
140a29416: e8 e5 4b f4 ff              	call	0x14096e000 <.text+0x96d000>
140a2941b: c7 44 24 20 01 00 00 00     	mov	dword ptr [rsp + 0x20], 0x1
140a29423: 4c 8d a4 24 28 05 00 00     	lea	r12, [rsp + 0x528]
140a2942b: 49 b8 0e 23 a2 3a 94 79 5e 9b       	movabs	r8, -0x64a1866bc55ddcf2
140a29435: 4c 8d 4c 24 40              	lea	r9, [rsp + 0x40]
140a2943a: 4c 89 e1                    	mov	rcx, r12
140a2943d: 48 89 f2                    	mov	rdx, rsi
140a29440: e8 bb 4b f4 ff              	call	0x14096e000 <.text+0x96d000>
140a29445: 48 8d ac 24 08 02 00 00     	lea	rbp, [rsp + 0x208]
140a2944d: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a29453: 48 89 e9                    	mov	rcx, rbp
140a29456: 4c 89 e2                    	mov	rdx, r12
140a29459: e8 a2 a7 ef 01              	call	0x142923c00 <.text+0x2922c00>
140a2945e: 49 b8 df 8b be ec 2e 03 24 d0       	movabs	r8, -0x2fdbfcd113417421
140a29468: 4c 89 e1                    	mov	rcx, r12
140a2946b: 48 89 ea                    	mov	rdx, rbp
140a2946e: 44 8b 4c 24 3c              	mov	r9d, dword ptr [rsp + 0x3c]
140a29473: e8 d8 ef bd 00              	call	0x141608450 <.text+0x1607450>
140a29478: 48 8d ac 24 08 02 00 00     	lea	rbp, [rsp + 0x208]
140a29480: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a29486: 48 89 e9                    	mov	rcx, rbp
140a29489: 4c 89 e2                    	mov	rdx, r12
140a2948c: e8 6f a7 ef 01              	call	0x142923c00 <.text+0x2922c00>
140a29491: c6 44 24 7a 00              	mov	byte ptr [rsp + 0x7a], 0x0
140a29496: 0f b6 84 24 d0 0b 00 00     	movzx	eax, byte ptr [rsp + 0xbd0]
140a2949e: 88 44 24 78                 	mov	byte ptr [rsp + 0x78], al
140a294a2: 0f b6 84 24 d8 0b 00 00     	movzx	eax, byte ptr [rsp + 0xbd8]
140a294aa: 88 44 24 79                 	mov	byte ptr [rsp + 0x79], al
140a294ae: 48 8b 7c 24 60              	mov	rdi, qword ptr [rsp + 0x60]
140a294b3: 48 89 7c 24 70              	mov	qword ptr [rsp + 0x70], rdi
140a294b8: 4c 8d 44 24 70              	lea	r8, [rsp + 0x70]
140a294bd: 48 89 e9                    	mov	rcx, rbp
140a294c0: 48 8b 94 24 e0 0b 00 00     	mov	rdx, qword ptr [rsp + 0xbe0]
140a294c8: e8 23 29 be 00              	call	0x14160bdf0 <.text+0x160adf0>
140a294cd: 4c 89 6c 24 30              	mov	qword ptr [rsp + 0x30], r13
140a294d2: 48 8b 44 24 58              	mov	rax, qword ptr [rsp + 0x58]
140a294d7: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
140a294dc: 48 c7 44 24 20 27 00 00 00  	mov	qword ptr [rsp + 0x20], 0x27
140a294e5: 48 8d 15 04 15 0a 02        	lea	rdx, [rip + 0x20a1504]  # 0x142aca9f0
140a294ec: 4c 8d 0d 1b 15 0a 02        	lea	r9, [rip + 0x20a151b]   # 0x142acaa0e
140a294f3: 41 b8 1e 00 00 00           	mov	r8d, 0x1e
140a294f9: 4c 8d ac 24 08 02 00 00     	lea	r13, [rsp + 0x208]
140a29501: 4c 89 e9                    	mov	rcx, r13
140a29504: e8 97 9d bd 00              	call	0x1416032a0 <.text+0x16022a0>
140a29509: 48 8d 5c 24 70              	lea	rbx, [rsp + 0x70]
140a2950e: 48 89 d9                    	mov	rcx, rbx
140a29511: 4c 89 ea                    	mov	rdx, r13
140a29514: 49 b8 e3 c5 90 19 c5 9d fe 86       	movabs	r8, -0x7901623ae66f3a1d
140a2951e: e8 ed 3b be 00              	call	0x14160d110 <.text+0x160c110>
140a29523: 4c 8d ac 24 98 03 00 00     	lea	r13, [rsp + 0x398]
140a2952b: 4c 89 e9                    	mov	rcx, r13
140a2952e: 48 89 da                    	mov	rdx, rbx
140a29531: 49 b8 84 ac 8a 23 de bb 37 53       	movabs	r8, 0x5337bbde238aac84
140a2953b: 4c 8b 8c 24 e8 0b 00 00     	mov	r9, qword ptr [rsp + 0xbe8]
140a29543: e8 78 45 f4 ff              	call	0x14096dac0 <.text+0x96cac0>
140a29548: 4c 89 f9                    	mov	rcx, r15
140a2954b: 4c 89 ea                    	mov	rdx, r13
140a2954e: 49 b8 9b ec 49 c2 df d9 05 d0       	movabs	r8, -0x2ffa26203db61365
140a29558: e8 d3 30 be 00              	call	0x14160c630 <.text+0x160b630>
140a2955d: 4c 89 f1                    	mov	rcx, r14
140a29560: 4c 89 fa                    	mov	rdx, r15
140a29563: 49 b8 3b 42 1d a4 2e da fa 77       	movabs	r8, 0x77fada2ea41d423b
140a2956d: 4c 8b 8c 24 f8 0b 00 00     	mov	r9, qword ptr [rsp + 0xbf8]
140a29575: e8 46 45 f4 ff              	call	0x14096dac0 <.text+0x96cac0>
140a2957a: c7 44 24 20 01 00 00 00     	mov	dword ptr [rsp + 0x20], 0x1
140a29582: 48 89 f1                    	mov	rcx, rsi
140a29585: 4c 89 f2                    	mov	rdx, r14
140a29588: 49 b8 01 f1 2c d7 c9 c4 e3 fe       	movabs	r8, -0x11c3b3628d30eff
140a29592: 4c 8b 8c 24 f0 0b 00 00     	mov	r9, qword ptr [rsp + 0xbf0]
140a2959a: e8 61 4a f4 ff              	call	0x14096e000 <.text+0x96d000>
140a2959f: c7 44 24 20 01 00 00 00     	mov	dword ptr [rsp + 0x20], 0x1
140a295a7: 4c 89 e1                    	mov	rcx, r12
140a295aa: 48 89 f2                    	mov	rdx, rsi
140a295ad: 49 b8 0e 23 a2 3a 94 79 5e 9b       	movabs	r8, -0x64a1866bc55ddcf2
140a295b7: 4c 8d 4c 24 40              	lea	r9, [rsp + 0x40]
140a295bc: e8 3f 4a f4 ff              	call	0x14096e000 <.text+0x96d000>
140a295c1: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a295c7: 48 89 e9                    	mov	rcx, rbp
140a295ca: 4c 89 e2                    	mov	rdx, r12
140a295cd: e8 2e a6 ef 01              	call	0x142923c00 <.text+0x2922c00>
140a295d2: c6 44 24 7a 01              	mov	byte ptr [rsp + 0x7a], 0x1
140a295d7: 0f b6 84 24 d0 0b 00 00     	movzx	eax, byte ptr [rsp + 0xbd0]
140a295df: 88 44 24 78                 	mov	byte ptr [rsp + 0x78], al
140a295e3: 0f b6 84 24 d8 0b 00 00     	movzx	eax, byte ptr [rsp + 0xbd8]
140a295eb: 88 44 24 79                 	mov	byte ptr [rsp + 0x79], al
140a295ef: 48 89 7c 24 70              	mov	qword ptr [rsp + 0x70], rdi
140a295f4: 48 89 e9                    	mov	rcx, rbp
140a295f7: 48 8b 94 24 e0 0b 00 00     	mov	rdx, qword ptr [rsp + 0xbe0]
140a295ff: 4c 8d 44 24 70              	lea	r8, [rsp + 0x70]
140a29604: e8 e7 27 be 00              	call	0x14160bdf0 <.text+0x160adf0>
140a29609: 48 8b 4c 24 50              	mov	rcx, qword ptr [rsp + 0x50]
140a2960e: 48 8b 44 24 68              	mov	rax, qword ptr [rsp + 0x68]
140a29613: 48 89 48 10                 	mov	qword ptr [rax + 0x10], rcx
140a29617: 0f 28 44 24 40              	movaps	xmm0, xmmword ptr [rsp + 0x40]
140a2961c: 0f 11 00                    	movups	xmmword ptr [rax], xmm0
140a2961f: 48 81 c4 68 0b 00 00        	add	rsp, 0xb68
140a29626: 5b                          	pop	rbx
140a29627: 5d                          	pop	rbp
140a29628: 5f                          	pop	rdi
140a29629: 5e                          	pop	rsi
140a2962a: 41 5c                       	pop	r12
140a2962c: 41 5d                       	pop	r13
140a2962e: 41 5e                       	pop	r14
140a29630: 41 5f                       	pop	r15
140a29632: c3                          	ret
