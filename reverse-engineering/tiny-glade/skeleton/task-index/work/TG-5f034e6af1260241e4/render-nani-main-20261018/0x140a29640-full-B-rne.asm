
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a29640: 41 57                       	push	r15
140a29642: 41 56                       	push	r14
140a29644: 56                          	push	rsi
140a29645: 57                          	push	rdi
140a29646: 55                          	push	rbp
140a29647: 53                          	push	rbx
140a29648: 48 81 ec 98 03 00 00        	sub	rsp, 0x398
140a2964f: 4c 89 c7                    	mov	rdi, r8
140a29652: 48 89 d3                    	mov	rbx, rdx
140a29655: 48 89 ce                    	mov	rsi, rcx
140a29658: 0f b6 ac 24 f8 03 00 00     	movzx	ebp, byte ptr [rsp + 0x3f8]
140a29660: 44 0f b6 bc 24 f0 03 00 00  	movzx	r15d, byte ptr [rsp + 0x3f0]
140a29669: 4d 8b b1 50 14 00 00        	mov	r14, qword ptr [r9 + 0x1450]
140a29670: 48 8d 4c 24 60              	lea	rcx, [rsp + 0x60]
140a29675: ba 00 6c 00 00              	mov	edx, 0x6c00
140a2967a: 41 b8 20 00 00 00           	mov	r8d, 0x20
140a29680: e8 9b b7 1a 00              	call	0x140bd4e20 <.text+0xbd3e20>
140a29685: 0f 10 44 24 60              	movups	xmm0, xmmword ptr [rsp + 0x60]
140a2968a: 0f 29 84 24 00 02 00 00     	movaps	xmmword ptr [rsp + 0x200], xmm0
140a29692: 48 8d 8c 24 80 00 00 00     	lea	rcx, [rsp + 0x80]
140a2969a: 48 8d 94 24 00 02 00 00     	lea	rdx, [rsp + 0x200]
140a296a2: e8 59 d6 1a 00              	call	0x140bd6d00 <.text+0xbd5d00>
140a296a7: 48 8d 05 87 13 0a 02        	lea	rax, [rip + 0x20a1387]  # 0x142acaa35
140a296ae: 48 89 44 24 70              	mov	qword ptr [rsp + 0x70], rax
140a296b3: 48 c7 44 24 78 1e 00 00 00  	mov	qword ptr [rsp + 0x78], 0x1e
140a296bc: 48 8d 54 24 70              	lea	rdx, [rsp + 0x70]
140a296c1: 48 89 d9                    	mov	rcx, rbx
140a296c4: e8 07 fe 1b 00              	call	0x140be94d0 <.text+0xbe84d0>
140a296c9: 0f 10 44 24 60              	movups	xmm0, xmmword ptr [rsp + 0x60]
140a296ce: 0f 29 44 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm0
140a296d3: 89 44 24 50                 	mov	dword ptr [rsp + 0x50], eax
140a296d7: 48 89 5c 24 30              	mov	qword ptr [rsp + 0x30], rbx
140a296dc: 48 89 7c 24 28              	mov	qword ptr [rsp + 0x28], rdi
140a296e1: 48 c7 44 24 20 3b 00 00 00  	mov	qword ptr [rsp + 0x20], 0x3b
140a296ea: 48 8d 15 62 13 0a 02        	lea	rdx, [rip + 0x20a1362]  # 0x142acaa53
140a296f1: 4c 8d 0d 7d 13 0a 02        	lea	r9, [rip + 0x20a137d]   # 0x142acaa75
140a296f8: 48 8d 7c 24 70              	lea	rdi, [rsp + 0x70]
140a296fd: 41 b8 22 00 00 00           	mov	r8d, 0x22
140a29703: 48 89 f9                    	mov	rcx, rdi
140a29706: e8 95 9b bd 00              	call	0x1416032a0 <.text+0x16022a0>
140a2970b: 48 8d 9c 24 00 02 00 00     	lea	rbx, [rsp + 0x200]
140a29713: 49 b8 e3 c5 90 19 c5 9d fe 86       	movabs	r8, -0x7901623ae66f3a1d
140a2971d: 48 89 d9                    	mov	rcx, rbx
140a29720: 48 89 fa                    	mov	rdx, rdi
140a29723: e8 e8 39 be 00              	call	0x14160d110 <.text+0x160c110>
140a29728: 48 8d 7c 24 70              	lea	rdi, [rsp + 0x70]
140a2972d: 49 b8 df 8b be ec 2e 03 24 d0       	movabs	r8, -0x2fdbfcd113417421
140a29737: 48 89 f9                    	mov	rcx, rdi
140a2973a: 48 89 da                    	mov	rdx, rbx
140a2973d: e8 4e e9 bd 00              	call	0x141608090 <.text+0x1607090>
140a29742: 48 8d 9c 24 00 02 00 00     	lea	rbx, [rsp + 0x200]
140a2974a: 49 b8 e8 cd 33 3e 1e 56 c6 5f       	movabs	r8, 0x5fc6561e3e33cde8
140a29754: 4c 8d 4c 24 40              	lea	r9, [rsp + 0x40]
140a29759: 48 89 d9                    	mov	rcx, rbx
140a2975c: 48 89 fa                    	mov	rdx, rdi
140a2975f: e8 fc 45 f4 ff              	call	0x14096dd60 <.text+0x96cd60>
140a29764: 44 88 7c 24 78              	mov	byte ptr [rsp + 0x78], r15b
140a29769: 40 88 6c 24 79              	mov	byte ptr [rsp + 0x79], bpl
140a2976e: 4c 89 74 24 70              	mov	qword ptr [rsp + 0x70], r14
140a29773: 48 8d 54 24 70              	lea	rdx, [rsp + 0x70]
140a29778: 48 89 d9                    	mov	rcx, rbx
140a2977b: e8 20 09 be 00              	call	0x14160a0a0 <.text+0x16090a0>
140a29780: 48 8b 44 24 50              	mov	rax, qword ptr [rsp + 0x50]
140a29785: 48 89 46 10                 	mov	qword ptr [rsi + 0x10], rax
140a29789: 0f 28 44 24 40              	movaps	xmm0, xmmword ptr [rsp + 0x40]
140a2978e: 0f 11 06                    	movups	xmmword ptr [rsi], xmm0
140a29791: 48 89 f0                    	mov	rax, rsi
140a29794: 48 81 c4 98 03 00 00        	add	rsp, 0x398
140a2979b: 5b                          	pop	rbx
140a2979c: 5d                          	pop	rbp
140a2979d: 5f                          	pop	rdi
140a2979e: 5e                          	pop	rsi
140a2979f: 41 5e                       	pop	r14
140a297a1: 41 5f                       	pop	r15
140a297a3: c3                          	ret
