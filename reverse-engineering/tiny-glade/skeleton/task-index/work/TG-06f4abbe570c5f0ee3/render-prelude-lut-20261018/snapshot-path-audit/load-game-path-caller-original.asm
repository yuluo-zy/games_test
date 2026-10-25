
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14146d700: 00 48 89                    	add	byte ptr [rax - 0x77], cl
14146d703: 8d 30                       	lea	esi, [rax]
14146d705: 09 00                       	or	dword ptr [rax], eax
14146d707: 00 c6                       	add	dh, al
14146d709: 85 76 09                    	test	dword ptr [rsi + 0x9], esi
14146d70c: 00 00                       	add	byte ptr [rax], al
14146d70e: 01 48 8d                    	add	dword ptr [rax - 0x73], ecx
14146d711: 8d 60 03                    	lea	esp, [rax + 0x3]
14146d714: 00 00                       	add	byte ptr [rax], al
14146d716: 48 89 95 e0 08 00 00        	mov	qword ptr [rbp + 0x8e0], rdx
14146d71d: e8 de 88 6d ff              	call	0x140b46000 <.text+0xb45000>
14146d722: 48 8d 8d 60 03 00 00        	lea	rcx, [rbp + 0x360]
14146d729: e8 a2 97 90 ff              	call	0x140d76ed0 <.text+0xd75ed0>
14146d72e: 49 89 d0                    	mov	r8, rdx
14146d731: 48 c7 44 24 20 0b 00 00 00  	mov	qword ptr [rsp + 0x20], 0xb
14146d73a: 4c 8d 0d 2d 6a 7f 01        	lea	r9, [rip + 0x17f6a2d]   # 0x142c6416e
14146d741: 48 8d 8d b0 01 00 00        	lea	rcx, [rbp + 0x1b0]
14146d748: 48 89 c2                    	mov	rdx, rax
14146d74b: e8 c0 50 8f ff              	call	0x140d62810 <.text+0xd61810>
14146d750: c7 85 b0 05 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x5b0], 0x0
14146d75a: 0f 57 c0                    	xorps	xmm0, xmm0
14146d75d: 0f 11 85 b8 05 00 00        	movups	xmmword ptr [rbp + 0x5b8], xmm0
14146d764: c7 85 c8 05 00 00 07 00 00 00       	mov	dword ptr [rbp + 0x5c8], 0x7
14146d76e: 48 c7 85 cc 05 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x5cc], 0x0
14146d779: 66 c7 85 d4 05 00 00 00 00  	mov	word ptr [rbp + 0x5d4], 0x0
14146d782: c6 85 d1 05 00 00 01        	mov	byte ptr [rbp + 0x5d1], 0x1
14146d789: c6 85 d4 05 00 00 01        	mov	byte ptr [rbp + 0x5d4], 0x1
14146d790: c6 85 d3 05 00 00 01        	mov	byte ptr [rbp + 0x5d3], 0x1
14146d797: 48 8d 8d b0 01 00 00        	lea	rcx, [rbp + 0x1b0]
14146d79e: e8 2d 97 90 ff              	call	0x140d76ed0 <.text+0xd75ed0>
14146d7a3: 49 89 d0                    	mov	r8, rdx
14146d7a6: 48 8d 8d b0 05 00 00        	lea	rcx, [rbp + 0x5b0]
14146d7ad: 48 89 c2                    	mov	rdx, rax
14146d7b0: e8 0b c6 8e ff              	call	0x140d59dc0 <.text+0xd58dc0>
14146d7b5: 49 89 c4                    	mov	r12, rax
14146d7b8: 49 89 d7                    	mov	r15, rdx
14146d7bb: 48 8b 95 b0 01 00 00        	mov	rdx, qword ptr [rbp + 0x1b0]
14146d7c2: 48 85 d2                    	test	rdx, rdx
14146d7c5: 74 12                       	je	0x14146d7d9 <.text+0x146c7d9>
14146d7c7: 48 8b 8d b8 01 00 00        	mov	rcx, qword ptr [rbp + 0x1b8]
14146d7ce: 41 b8 01 00 00 00           	mov	r8d, 0x1
14146d7d4: e8 47 64 1a ff              	call	0x140613c20 <.text+0x612c20>
14146d7d9: 4d 85 e4                    	test	r12, r12
14146d7dc: 0f 84 e6 01 00 00           	je	0x14146d9c8 <.text+0x146c9c8>
14146d7e2: 44 89 f8                    	mov	eax, r15d
14146d7e5: 83 e0 03                    	and	eax, 0x3
14146d7e8: 83 f8 01                    	cmp	eax, 0x1
14146d7eb: 4c 8b a5 00 09 00 00        	mov	r12, qword ptr [rbp + 0x900]
14146d7f2: 0f 85 e0 01 00 00           	jne	0x14146d9d8 <.text+0x146c9d8>
14146d7f8: 4c 89 f8                    	mov	rax, r15
14146d7fb: 48 ff c8                    	dec	rax
14146d7fe: 48 89 85 10 09 00 00        	mov	qword ptr [rbp + 0x910], rax
14146d805: 49 8b 47 ff                 	mov	rax, qword ptr [r15 - 0x1]
14146d809: 48 89 85 38 09 00 00        	mov	qword ptr [rbp + 0x938], rax
14146d810: 49 8b 47 07                 	mov	rax, qword ptr [r15 + 0x7]
14146d814: 48 89 85 48 09 00 00        	mov	qword ptr [rbp + 0x948], rax
14146d81b: 48 8b 00                    	mov	rax, qword ptr [rax]
14146d81e: 48 85 c0                    	test	rax, rax
14146d821: 74 09                       	je	0x14146d82c <.text+0x146c82c>
14146d823: 48 8b 8d 38 09 00 00        	mov	rcx, qword ptr [rbp + 0x938]
14146d82a: ff d0                       	call	rax
14146d82c: 48 8b 8d 38 09 00 00        	mov	rcx, qword ptr [rbp + 0x938]
14146d833: 48 8b 85 48 09 00 00        	mov	rax, qword ptr [rbp + 0x948]
14146d83a: 48 8b 50 08                 	mov	rdx, qword ptr [rax + 0x8]
14146d83e: 48 85 d2                    	test	rdx, rdx
14146d841: 74 09                       	je	0x14146d84c <.text+0x146c84c>
14146d843: 4c 8b 40 10                 	mov	r8, qword ptr [rax + 0x10]
14146d847: e8 d4 63 1a ff              	call	0x140613c20 <.text+0x612c20>
14146d84c: ba 18 00 00 00              	mov	edx, 0x18
14146d851: 41 b8 08 00 00 00           	mov	r8d, 0x8
14146d857: 48 8b 8d 10 09 00 00        	mov	rcx, qword ptr [rbp + 0x910]
14146d85e: e8 bd 63 1a ff              	call	0x140613c20 <.text+0x612c20>
14146d863: e9 70 01 00 00              	jmp	0x14146d9d8 <.text+0x146c9d8>
14146d868: 4c 89 85 00 09 00 00        	mov	qword ptr [rbp + 0x900], r8
14146d86f: 45 8b 60 4c                 	mov	r12d, dword ptr [r8 + 0x4c]
14146d873: 45 85 e4                    	test	r12d, r12d
14146d876: 0f 88 a9 00 00 00           	js	0x14146d925 <.text+0x146c925>
14146d87c: 49 8d 47 20                 	lea	rax, [r15 + 0x20]
14146d880: 48 89 85 28 09 00 00        	mov	qword ptr [rbp + 0x928], rax
14146d887: 45 31 ed                    	xor	r13d, r13d
14146d88a: eb 29                       	jmp	0x14146d8b5 <.text+0x146c8b5>
14146d88c: 0f 1f 40 00                 	nop	dword ptr [rax]
14146d890: 41 ff c5                    	inc	r13d
14146d893: 49 8b 47 28                 	mov	rax, qword ptr [r15 + 0x28]
14146d897: 48 89 f9                    	mov	rcx, rdi
14146d89a: 48 c1 e1 04                 	shl	rcx, 0x4
14146d89e: 48 89 34 08                 	mov	qword ptr [rax + rcx], rsi
14146d8a2: c7 44 08 08 01 01 01 03     	mov	dword ptr [rax + rcx + 0x8], 0x3010101
14146d8aa: 48 ff c7                    	inc	rdi
14146d8ad: 49 89 7f 30                 	mov	qword ptr [r15 + 0x30], rdi
14146d8b1: 49 ff 47 40                 	inc	qword ptr [r15 + 0x40]
14146d8b5: 45 39 e5                    	cmp	r13d, r12d
14146d8b8: 0f 8d ea 00 00 00           	jge	0x14146d9a8 <.text+0x146c9a8>
14146d8be: 41 89 1e                    	mov	dword ptr [r14], ebx
14146d8c1: 49 8b 7f 30                 	mov	rdi, qword ptr [r15 + 0x30]
14146d8c5: 49 8b 77 40                 	mov	rsi, qword ptr [r15 + 0x40]
14146d8c9: 49 3b 7f 20                 	cmp	rdi, qword ptr [r15 + 0x20]
14146d8cd: 75 c1                       	jne	0x14146d890 <.text+0x146c890>
14146d8cf: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
14146d8d9: 48 89 85 30 09 00 00        	mov	qword ptr [rbp + 0x930], rax
14146d8e0: c6 85 76 09 00 00 01        	mov	byte ptr [rbp + 0x976], 0x1
14146d8e7: 48 8b 8d 28 09 00 00        	mov	rcx, qword ptr [rbp + 0x928]
14146d8ee: 48 8d 15 53 c8 7c 01        	lea	rdx, [rip + 0x17cc853]  # 0x142c3a148
14146d8f5: e8 36 4d 0f ff              	call	0x140562630 <.text+0x561630>
14146d8fa: eb 94                       	jmp	0x14146d890 <.text+0x146c890>
14146d8fc: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
14146d906: 48 89 85 30 09 00 00        	mov	qword ptr [rbp + 0x930], rax
14146d90d: c6 85 76 09 00 00 01        	mov	byte ptr [rbp + 0x976], 0x1
14146d914: 48 8d 8d 60 03 00 00        	lea	rcx, [rbp + 0x360]
14146d91b: e8 d0 d9 40 ff              	call	0x14087b2f0 <.text+0x87a2f0>
14146d920: e9 00 1d 00 00              	jmp	0x14146f625 <.text+0x146e625>
14146d925: 41 f7 dc                    	neg	r12d
14146d928: 49 8d 47 20                 	lea	rax, [r15 + 0x20]
14146d92c: 48 89 85 28 09 00 00        	mov	qword ptr [rbp + 0x928], rax
14146d933: 45 31 ed                    	xor	r13d, r13d
14146d936: eb 2d                       	jmp	0x14146d965 <.text+0x146c965>
14146d938: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
14146d940: 41 ff c5                    	inc	r13d
14146d943: 49 8b 47 28                 	mov	rax, qword ptr [r15 + 0x28]
14146d947: 48 89 f9                    	mov	rcx, rdi
14146d94a: 48 c1 e1 04                 	shl	rcx, 0x4
14146d94e: 48 89 34 08                 	mov	qword ptr [rax + rcx], rsi
14146d952: c7 44 08 08 01 00 01 03     	mov	dword ptr [rax + rcx + 0x8], 0x3010001
14146d95a: 48 ff c7                    	inc	rdi
14146d95d: 49 89 7f 30                 	mov	qword ptr [r15 + 0x30], rdi
14146d961: 49 ff 47 40                 	inc	qword ptr [r15 + 0x40]
14146d965: 45 39 e5                    	cmp	r13d, r12d
14146d968: 7d 3e                       	jge	0x14146d9a8 <.text+0x146c9a8>
14146d96a: 41 89 1e                    	mov	dword ptr [r14], ebx
14146d96d: 49 8b 7f 30                 	mov	rdi, qword ptr [r15 + 0x30]
14146d971: 49 8b 77 40                 	mov	rsi, qword ptr [r15 + 0x40]
14146d975: 49 3b 7f 20                 	cmp	rdi, qword ptr [r15 + 0x20]
14146d979: 75 c5                       	jne	0x14146d940 <.text+0x146c940>
14146d97b: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
14146d985: 48 89 85 30 09 00 00        	mov	qword ptr [rbp + 0x930], rax
14146d98c: c6 85 76 09 00 00 01        	mov	byte ptr [rbp + 0x976], 0x1
14146d993: 48 8b 8d 28 09 00 00        	mov	rcx, qword ptr [rbp + 0x928]
14146d99a: 48 8d 15 a7 c7 7c 01        	lea	rdx, [rip + 0x17cc7a7]  # 0x142c3a148
14146d9a1: e8 8a 4c 0f ff              	call	0x140562630 <.text+0x561630>
14146d9a6: eb 98                       	jmp	0x14146d940 <.text+0x146c940>
14146d9a8: 48 8b 8d 58 09 00 00        	mov	rcx, qword ptr [rbp + 0x958]
14146d9af: 8b 41 1c                    	mov	eax, dword ptr [rcx + 0x1c]
14146d9b2: 48 8b 49 10                 	mov	rcx, qword ptr [rcx + 0x10]
14146d9b6: 89 01                       	mov	dword ptr [rcx], eax
14146d9b8: 48 8b 85 00 09 00 00        	mov	rax, qword ptr [rbp + 0x900]
14146d9bf: c6 40 48 02                 	mov	byte ptr [rax + 0x48], 0x2
14146d9c3: e9 5d 1c 00 00              	jmp	0x14146f625 <.text+0x146e625>
14146d9c8: 4c 89 f9                    	mov	rcx, r15
14146d9cb: ff 15 1f 4a d2 01           	call	qword ptr [rip + 0x1d24a1f] # 0x1431923f0
14146d9d1: 4c 8b a5 00 09 00 00        	mov	r12, qword ptr [rbp + 0x900]
14146d9d8: 48 8b 95 60 03 00 00        	mov	rdx, qword ptr [rbp + 0x360]
14146d9df: 48 85 d2                    	test	rdx, rdx
14146d9e2: 74 12                       	je	0x14146d9f6 <.text+0x146c9f6>
14146d9e4: 48 8b 8d 68 03 00 00        	mov	rcx, qword ptr [rbp + 0x368]
14146d9eb: 41 b8 01 00 00 00           	mov	r8d, 0x1
14146d9f1: e8 2a 62 1a ff              	call	0x140613c20 <.text+0x612c20>
14146d9f6: 84 db                       	test	bl, bl
14146d9f8: 0f 84 4f 01 00 00           	je	0x14146db4d <.text+0x146cb4d>
14146d9fe: 48 8b 05 a3 e9 1b 02        	mov	rax, qword ptr [rip + 0x21be9a3] # 0x14362c3a8
14146da05: 48 8b 00                    	mov	rax, qword ptr [rax]
14146da08: 48 83 f8 03                 	cmp	rax, 0x3
14146da0c: 0f 82 dc 00 00 00           	jb	0x14146daee <.text+0x146caee>
14146da12: 48 be 00 00 00 00 00 00 00 80       	movabs	rsi, -0x8000000000000000
14146da1c: 48 89 b5 30 09 00 00        	mov	qword ptr [rbp + 0x930], rsi
14146da23: c6 85 76 09 00 00 01        	mov	byte ptr [rbp + 0x976], 0x1
14146da2a: 48 8d 0d 87 67 7f 01        	lea	rcx, [rip + 0x17f6787]  # 0x142c641b8
14146da31: e8 4a 03 21 ff              	call	0x14067dd80 <.text+0x67cd80>
14146da36: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
14146da39: 8b 40 10                    	mov	eax, dword ptr [rax + 0x10]
14146da3c: 48 c7 85 e0 05 00 00 03 00 00 00    	mov	qword ptr [rbp + 0x5e0], 0x3
14146da47: 48 8d 0d fa 66 7f 01        	lea	rcx, [rip + 0x17f66fa]  # 0x142c64148
14146da4e: 48 89 8d e8 05 00 00        	mov	qword ptr [rbp + 0x5e8], rcx
14146da55: 48 c7 85 f0 05 00 00 26 00 00 00    	mov	qword ptr [rbp + 0x5f0], 0x26
14146da60: 48 8d 15 41 67 7f 01        	lea	rdx, [rip + 0x17f6741]  # 0x142c641a8
14146da67: 48 89 95 00 06 00 00        	mov	qword ptr [rbp + 0x600], rdx
14146da6e: 48 c7 85 08 06 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x608], 0x1
14146da79: 48 c7 85 10 06 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x610], 0x8
14146da84: 48 c7 85 b0 05 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x5b0], 0x0
14146da8f: 0f 57 c9                    	xorps	xmm1, xmm1
14146da92: 0f 11 8d 18 06 00 00        	movups	xmmword ptr [rbp + 0x618], xmm1
14146da99: 48 89 8d b8 05 00 00        	mov	qword ptr [rbp + 0x5b8], rcx
14146daa0: 48 c7 85 c0 05 00 00 26 00 00 00    	mov	qword ptr [rbp + 0x5c0], 0x26
14146daab: 48 c7 85 c8 05 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x5c8], 0x0
14146dab6: 0f 11 85 d0 05 00 00        	movups	xmmword ptr [rbp + 0x5d0], xmm0
14146dabd: c7 85 f8 05 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x5f8], 0x1
14146dac7: 89 85 fc 05 00 00           	mov	dword ptr [rbp + 0x5fc], eax
14146dacd: 48 89 b5 30 09 00 00        	mov	qword ptr [rbp + 0x930], rsi
14146dad4: c6 85 76 09 00 00 01        	mov	byte ptr [rbp + 0x976], 0x1
14146dadb: 48 8d 8d 72 09 00 00        	lea	rcx, [rbp + 0x972]
14146dae2: 48 8d 95 b0 05 00 00        	lea	rdx, [rbp + 0x5b0]
14146dae9: e8 42 c8 8d ff              	call	0x140d4a330 <.text+0xd49330>
14146daee: 48 8b 85 78 05 00 00        	mov	rax, qword ptr [rbp + 0x578]
14146daf5: 8b 8d c8 08 00 00           	mov	ecx, dword ptr [rbp + 0x8c8]
14146dafb: 89 08                       	mov	dword ptr [rax], ecx
14146dafd: 49 8b 75 30                 	mov	rsi, qword ptr [r13 + 0x30]
14146db01: 49 8b 7d 40                 	mov	rdi, qword ptr [r13 + 0x40]
14146db05: 49 3b 75 20                 	cmp	rsi, qword ptr [r13 + 0x20]
14146db09: 75 28                       	jne	0x14146db33 <.text+0x146cb33>
14146db0b: 49 8d 4d 20                 	lea	rcx, [r13 + 0x20]
14146db0f: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
14146db19: 48 89 85 30 09 00 00        	mov	qword ptr [rbp + 0x930], rax
14146db20: c6 85 76 09 00 00 01        	mov	byte ptr [rbp + 0x976], 0x1
14146db27: 48 8d 15 1a c6 7c 01        	lea	rdx, [rip + 0x17cc61a]  # 0x142c3a148
14146db2e: e8 3d 4d 0f ff              	call	0x140562870 <.text+0x561870>
14146db33: 49 8b 45 28                 	mov	rax, qword ptr [r13 + 0x28]
14146db37: 48 89 3c f0                 	mov	qword ptr [rax + 8*rsi], rdi
14146db3b: 48 ff c6                    	inc	rsi
14146db3e: 49 89 75 30                 	mov	qword ptr [r13 + 0x30], rsi
14146db42: 49 ff 45 40                 	inc	qword ptr [r13 + 0x40]
14146db46: 4c 8b a5 00 09 00 00        	mov	r12, qword ptr [rbp + 0x900]
14146db4d: 48 8b 8d 58 09 00 00        	mov	rcx, qword ptr [rbp + 0x958]
14146db54: 8b 41 1c                    	mov	eax, dword ptr [rcx + 0x1c]
14146db57: 48 8b 49 10                 	mov	rcx, qword ptr [rcx + 0x10]
14146db5b: 89 85 cc 08 00 00           	mov	dword ptr [rbp + 0x8cc], eax
14146db61: 48 89 8d 88 05 00 00        	mov	qword ptr [rbp + 0x588], rcx
14146db68: 89 01                       	mov	dword ptr [rcx], eax
14146db6a: 49 8b 74 24 18              	mov	rsi, qword ptr [r12 + 0x18]
14146db6f: 49 8b 7c 24 38              	mov	rdi, qword ptr [r12 + 0x38]
14146db74: 49 8d 44 24 20              	lea	rax, [r12 + 0x20]
14146db79: 49 8b 4c 24 40              	mov	rcx, qword ptr [r12 + 0x40]
14146db7e: 48 89 8d 10 09 00 00        	mov	qword ptr [rbp + 0x910], rcx
14146db85: 48 bb 00 00 00 00 00 00 00 80       	movabs	rbx, -0x8000000000000000
14146db8f: 49 89 5c 24 18              	mov	qword ptr [r12 + 0x18], rbx
14146db94: 48 89 f2                    	mov	rdx, rsi
14146db97: 48 31 da                    	xor	rdx, rbx
14146db9a: 48 83 fa 02                 	cmp	rdx, 0x2
14146db9e: b9 02 00 00 00              	mov	ecx, 0x2
14146dba3: 48 0f 42 ca                 	cmovb	rcx, rdx
14146dba7: 48 83 f9 01                 	cmp	rcx, 0x1
14146dbab: 0f 84 85 05 00 00           	je	0x14146e136 <.text+0x146d136>
14146dbb1: 48 83 f9 02                 	cmp	rcx, 0x2
14146dbb5: 0f 85 26 1e 00 00           	jne	0x14146f9e1 <.text+0x146e9e1>
14146dbbb: 48 8b 48 10                 	mov	rcx, qword ptr [rax + 0x10]
14146dbbf: 48 89 8d a0 05 00 00        	mov	qword ptr [rbp + 0x5a0], rcx
14146dbc6: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
14146dbc9: 0f 29 85 90 05 00 00        	movaps	xmmword ptr [rbp + 0x590], xmm0
14146dbd0: 48 89 b5 30 08 00 00        	mov	qword ptr [rbp + 0x830], rsi
14146dbd7: 0f 28 85 90 05 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x590]
14146dbde: 0f 11 85 38 08 00 00        	movups	xmmword ptr [rbp + 0x838], xmm0
14146dbe5: 48 8b 85 a0 05 00 00        	mov	rax, qword ptr [rbp + 0x5a0]
14146dbec: 48 89 85 48 08 00 00        	mov	qword ptr [rbp + 0x848], rax
14146dbf3: 45 84 f6                    	test	r14b, r14b
14146dbf6: 74 0a                       	je	0x14146dc02 <.text+0x146cc02>
14146dbf8: c7 87 c0 00 00 00 1b 00 00 00       	mov	dword ptr [rdi + 0xc0], 0x1b
14146dc02: 48 89 bd 48 09 00 00        	mov	qword ptr [rbp + 0x948], rdi
14146dc09: 48 89 9d d0 08 00 00        	mov	qword ptr [rbp + 0x8d0], rbx
14146dc10: c6 85 71 09 00 00 01        	mov	byte ptr [rbp + 0x971], 0x1
14146dc17: c6 85 70 09 00 00 01        	mov	byte ptr [rbp + 0x970], 0x1
14146dc1e: 48 8d 8d 30 08 00 00        	lea	rcx, [rbp + 0x830]
14146dc25: e8 a6 92 90 ff              	call	0x140d76ed0 <.text+0xd75ed0>
14146dc2a: 49 89 c4                    	mov	r12, rax
14146dc2d: 49 89 d6                    	mov	r14, rdx
14146dc30: 48 8b 85 b0 08 00 00        	mov	rax, qword ptr [rbp + 0x8b0]
14146dc37: 8b 8d b8 08 00 00           	mov	ecx, dword ptr [rbp + 0x8b8]
14146dc3d: 89 08                       	mov	dword ptr [rax], ecx
14146dc3f: 48 8b 85 60 05 00 00        	mov	rax, qword ptr [rbp + 0x560]
14146dc46: 8b 8d c0 08 00 00           	mov	ecx, dword ptr [rbp + 0x8c0]
14146dc4c: 89 08                       	mov	dword ptr [rax], ecx
14146dc4e: 48 8b 85 d8 08 00 00        	mov	rax, qword ptr [rbp + 0x8d8]
14146dc55: 8b 8d 0c 09 00 00           	mov	ecx, dword ptr [rbp + 0x90c]
14146dc5b: 89 08                       	mov	dword ptr [rax], ecx
14146dc5d: 48 8b 85 70 05 00 00        	mov	rax, qword ptr [rbp + 0x570]
14146dc64: 8b 8d c4 08 00 00           	mov	ecx, dword ptr [rbp + 0x8c4]
14146dc6a: 89 08                       	mov	dword ptr [rax], ecx
14146dc6c: 48 8b 85 28 05 00 00        	mov	rax, qword ptr [rbp + 0x528]
14146dc73: 8b 8d 5c 08 00 00           	mov	ecx, dword ptr [rbp + 0x85c]
14146dc79: 89 08                       	mov	dword ptr [rax], ecx
14146dc7b: 48 8b 8d 48 09 00 00        	mov	rcx, qword ptr [rbp + 0x948]
14146dc82: 8b 81 c0 00 00 00           	mov	eax, dword ptr [rcx + 0xc0]
14146dc88: 83 f8 1a                    	cmp	eax, 0x1a
14146dc8b: 48 8b 05 76 e6 1b 02        	mov	rax, qword ptr [rip + 0x21be676] # 0x14362c308
14146dc92: 0f 9d 00                    	setge	byte ptr [rax]
14146dc95: 0f 9f 05 75 e3 1b 02        	setg	byte ptr [rip + 0x21be375] # 0x14362c011
14146dc9c: 48 8d 91 c4 00 00 00        	lea	rdx, [rcx + 0xc4]
14146dca3: 48 89 9d 40 09 00 00        	mov	qword ptr [rbp + 0x940], rbx
14146dcaa: c6 85 77 09 00 00 01        	mov	byte ptr [rbp + 0x977], 0x1
14146dcb1: 48 8b 8d a8 08 00 00        	mov	rcx, qword ptr [rbp + 0x8a8]
14146dcb8: 41 b0 01                    	mov	r8b, 0x1
14146dcbb: e8 60 38 4a ff              	call	0x140911520 <.text+0x910520>
14146dcc0: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
14146dcca: 48 89 85 40 09 00 00        	mov	qword ptr [rbp + 0x940], rax
14146dcd1: c6 85 77 09 00 00 01        	mov	byte ptr [rbp + 0x977], 0x1
14146dcd8: 48 8b 8d a8 08 00 00        	mov	rcx, qword ptr [rbp + 0x8a8]
14146dcdf: e8 6c 3e 4a ff              	call	0x140911b50 <.text+0x910b50>
14146dce4: 4c 8b 85 48 09 00 00        	mov	r8, qword ptr [rbp + 0x948]
14146dceb: 41 0f 10 80 94 00 00 00     	movups	xmm0, xmmword ptr [r8 + 0x94]
14146dcf3: 41 0f 10 88 a4 00 00 00     	movups	xmm1, xmmword ptr [r8 + 0xa4]
14146dcfb: 41 0f 10 90 b0 00 00 00     	movups	xmm2, xmmword ptr [r8 + 0xb0]
14146dd03: 48 8b 85 58 05 00 00        	mov	rax, qword ptr [rbp + 0x558]
14146dd0a: 0f 11 50 1c                 	movups	xmmword ptr [rax + 0x1c], xmm2
14146dd0e: 0f 11 48 10                 	movups	xmmword ptr [rax + 0x10], xmm1
14146dd12: 0f 11 00                    	movups	xmmword ptr [rax], xmm0
14146dd15: 49 8b 40 30                 	mov	rax, qword ptr [r8 + 0x30]
14146dd19: 48 89 85 50 09 00 00        	mov	qword ptr [rbp + 0x950], rax
14146dd20: 49 8b 40 38                 	mov	rax, qword ptr [r8 + 0x38]
14146dd24: 48 89 85 60 09 00 00        	mov	qword ptr [rbp + 0x960], rax
14146dd2b: 49 8b 70 40                 	mov	rsi, qword ptr [r8 + 0x40]
14146dd2f: 41 8b 80 90 00 00 00        	mov	eax, dword ptr [r8 + 0x90]
14146dd36: 48 8b 8d 68 05 00 00        	mov	rcx, qword ptr [rbp + 0x568]
14146dd3d: 89 41 40                    	mov	dword ptr [rcx + 0x40], eax
14146dd40: 41 0f 10 80 80 00 00 00     	movups	xmm0, xmmword ptr [r8 + 0x80]
14146dd48: 0f 11 41 30                 	movups	xmmword ptr [rcx + 0x30], xmm0
14146dd4c: 41 0f 10 40 70              	movups	xmm0, xmmword ptr [r8 + 0x70]
14146dd51: 0f 11 41 20                 	movups	xmmword ptr [rcx + 0x20], xmm0
14146dd55: 41 0f 10 40 60              	movups	xmm0, xmmword ptr [r8 + 0x60]
14146dd5a: 0f 11 41 10                 	movups	xmmword ptr [rcx + 0x10], xmm0
14146dd5e: 41 0f 10 40 50              	movups	xmm0, xmmword ptr [r8 + 0x50]
14146dd63: 0f 11 01                    	movups	xmmword ptr [rcx], xmm0
14146dd66: 41 8b 80 e8 00 00 00        	mov	eax, dword ptr [r8 + 0xe8]
14146dd6d: 89 85 38 09 00 00           	mov	dword ptr [rbp + 0x938], eax
14146dd73: 41 8b 80 ec 00 00 00        	mov	eax, dword ptr [r8 + 0xec]
14146dd7a: 89 85 58 09 00 00           	mov	dword ptr [rbp + 0x958], eax
14146dd80: 41 0f 10 00                 	movups	xmm0, xmmword ptr [r8]
14146dd84: 0f 29 45 b0                 	movaps	xmmword ptr [rbp - 0x50], xmm0
14146dd88: 4d 8b 48 10                 	mov	r9, qword ptr [r8 + 0x10]
14146dd8c: 49 8b 40 18                 	mov	rax, qword ptr [r8 + 0x18]
14146dd90: 49 8b 48 20                 	mov	rcx, qword ptr [r8 + 0x20]
14146dd94: 49 8b 50 28                 	mov	rdx, qword ptr [r8 + 0x28]
14146dd98: 4d 8b 50 48                 	mov	r10, qword ptr [r8 + 0x48]
14146dd9c: 4c 89 95 b8 08 00 00        	mov	qword ptr [rbp + 0x8b8], r10
14146dda3: 45 8b 80 c0 00 00 00        	mov	r8d, dword ptr [r8 + 0xc0]
14146ddaa: 44 89 85 0c 09 00 00        	mov	dword ptr [rbp + 0x90c], r8d
14146ddb1: 48 c1 e2 04                 	shl	rdx, 0x4
14146ddb5: 48 01 ca                    	add	rdx, rcx
14146ddb8: 48 89 8d b0 05 00 00        	mov	qword ptr [rbp + 0x5b0], rcx
14146ddbf: 48 89 8d b8 05 00 00        	mov	qword ptr [rbp + 0x5b8], rcx
14146ddc6: 48 89 85 c0 05 00 00        	mov	qword ptr [rbp + 0x5c0], rax
14146ddcd: 48 89 95 c8 05 00 00        	mov	qword ptr [rbp + 0x5c8], rdx
14146ddd4: 0f 29 85 b0 01 00 00        	movaps	xmmword ptr [rbp + 0x1b0], xmm0
14146dddb: 4c 89 8d b0 08 00 00        	mov	qword ptr [rbp + 0x8b0], r9
14146dde2: 4c 89 8d c0 01 00 00        	mov	qword ptr [rbp + 0x1c0], r9
14146dde9: 4c 8d 05 68 ae 7c 01        	lea	r8, [rip + 0x17cae68]   # 0x142c38c58
14146ddf0: 48 8d 8d 60 03 00 00        	lea	rcx, [rbp + 0x360]
14146ddf7: 48 8d 95 b0 05 00 00        	lea	rdx, [rbp + 0x5b0]
14146ddfe: e8 0d 97 2c 00              	call	0x141737510 <.text+0x1736510>
14146de03: 48 89 b5 d8 08 00 00        	mov	qword ptr [rbp + 0x8d8], rsi
14146de0a: 0f 10 85 60 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x360]
14146de11: 0f 11 85 c8 05 00 00        	movups	xmmword ptr [rbp + 0x5c8], xmm0
14146de18: 48 8b 85 70 03 00 00        	mov	rax, qword ptr [rbp + 0x370]
14146de1f: 48 89 85 d8 05 00 00        	mov	qword ptr [rbp + 0x5d8], rax
14146de26: 0f 28 85 b0 01 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x1b0]
14146de2d: 0f 29 85 b0 05 00 00        	movaps	xmmword ptr [rbp + 0x5b0], xmm0
14146de34: 48 8b 85 c0 01 00 00        	mov	rax, qword ptr [rbp + 0x1c0]
14146de3b: 48 89 85 c0 05 00 00        	mov	qword ptr [rbp + 0x5c0], rax
14146de42: 48 8b 8d 18 09 00 00        	mov	rcx, qword ptr [rbp + 0x918]
14146de49: e8 a2 ce 41 ff              	call	0x14088acf0 <.text+0x889cf0>
14146de4e: 0f 28 85 b0 05 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x5b0]
14146de55: 0f 28 8d c0 05 00 00        	movaps	xmm1, xmmword ptr [rbp + 0x5c0]
14146de5c: 0f 28 95 d0 05 00 00        	movaps	xmm2, xmmword ptr [rbp + 0x5d0]
14146de63: 48 8b 8d 18 09 00 00        	mov	rcx, qword ptr [rbp + 0x918]
14146de6a: 0f 11 51 20                 	movups	xmmword ptr [rcx + 0x20], xmm2
14146de6e: 0f 11 49 10                 	movups	xmmword ptr [rcx + 0x10], xmm1
14146de72: 0f 11 01                    	movups	xmmword ptr [rcx], xmm0
14146de75: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
14146de7f: 48 89 41 30                 	mov	qword ptr [rcx + 0x30], rax
14146de83: 48 8b 85 b0 08 00 00        	mov	rax, qword ptr [rbp + 0x8b0]
14146de8a: 48 89 41 58                 	mov	qword ptr [rcx + 0x58], rax
14146de8e: 48 8b 85 b8 08 00 00        	mov	rax, qword ptr [rbp + 0x8b8]
14146de95: 48 89 41 60                 	mov	qword ptr [rcx + 0x60], rax
14146de99: 8b 85 38 09 00 00           	mov	eax, dword ptr [rbp + 0x938]
14146de9f: 89 41 68                    	mov	dword ptr [rcx + 0x68], eax
14146dea2: 8b 85 58 09 00 00           	mov	eax, dword ptr [rbp + 0x958]
14146dea8: 89 41 6c                    	mov	dword ptr [rcx + 0x6c], eax
14146deab: 8b 85 0c 09 00 00           	mov	eax, dword ptr [rbp + 0x90c]
14146deb1: 89 41 70                    	mov	dword ptr [rcx + 0x70], eax
14146deb4: 48 0f ba a5 10 09 00 00 38  	bt	qword ptr [rbp + 0x910], 0x38
14146debd: 73 63                       	jae	0x14146df22 <.text+0x146cf22>
14146debf: 48 8b 85 78 05 00 00        	mov	rax, qword ptr [rbp + 0x578]
14146dec6: 8b 8d c8 08 00 00           	mov	ecx, dword ptr [rbp + 0x8c8]
14146decc: 89 08                       	mov	dword ptr [rax], ecx
14146dece: 49 8b 75 30                 	mov	rsi, qword ptr [r13 + 0x30]
14146ded2: 49 8b 7d 40                 	mov	rdi, qword ptr [r13 + 0x40]
14146ded6: 49 3b 75 20                 	cmp	rsi, qword ptr [r13 + 0x20]
14146deda: 48 8b 85 60 09 00 00        	mov	rax, qword ptr [rbp + 0x960]
14146dee1: 48 8b 95 50 09 00 00        	mov	rdx, qword ptr [rbp + 0x950]
14146dee8: 75 25                       	jne	0x14146df0f <.text+0x146cf0f>
14146deea: 49 8d 4d 20                 	lea	rcx, [r13 + 0x20]
14146deee: 48 89 95 40 09 00 00        	mov	qword ptr [rbp + 0x940], rdx
14146def5: 48 89 85 20 09 00 00        	mov	qword ptr [rbp + 0x920], rax
14146defc: c6 85 77 09 00 00 00        	mov	byte ptr [rbp + 0x977], 0x0
14146df03: 48 8d 15 3e c2 7c 01        	lea	rdx, [rip + 0x17cc23e]  # 0x142c3a148
14146df0a: e8 61 49 0f ff              	call	0x140562870 <.text+0x561870>
14146df0f: 49 8b 45 28                 	mov	rax, qword ptr [r13 + 0x28]
14146df13: 48 89 3c f0                 	mov	qword ptr [rax + 8*rsi], rdi
14146df17: 48 ff c6                    	inc	rsi
14146df1a: 49 89 75 30                 	mov	qword ptr [r13 + 0x30], rsi
14146df1e: 49 ff 45 40                 	inc	qword ptr [r13 + 0x40]
14146df22: 80 bd 28 09 00 00 00        	cmp	byte ptr [rbp + 0x928], 0x0
14146df29: 75 7b                       	jne	0x14146dfa6 <.text+0x146cfa6>
14146df2b: 4c 8b bd 60 0a 00 00        	mov	r15, qword ptr [rbp + 0xa60]
14146df32: 48 8b 85 50 09 00 00        	mov	rax, qword ptr [rbp + 0x950]
14146df39: 48 89 85 40 09 00 00        	mov	qword ptr [rbp + 0x940], rax
14146df40: 48 8b 85 60 09 00 00        	mov	rax, qword ptr [rbp + 0x960]
14146df47: 48 89 85 20 09 00 00        	mov	qword ptr [rbp + 0x920], rax
14146df4e: c6 85 77 09 00 00 00        	mov	byte ptr [rbp + 0x977], 0x0
14146df55: 4c 89 7c 24 20              	mov	qword ptr [rsp + 0x20], r15
14146df5a: 48 8d 8d b0 05 00 00        	lea	rcx, [rbp + 0x5b0]
14146df61: 48 8b 95 18 09 00 00        	mov	rdx, qword ptr [rbp + 0x918]
14146df68: 4d 89 e0                    	mov	r8, r12
14146df6b: 4d 89 f1                    	mov	r9, r14
14146df6e: e8 dd 8e 33 00              	call	0x1417a6e50 <.text+0x17a5e50>
14146df73: 48 8b b5 b8 05 00 00        	mov	rsi, qword ptr [rbp + 0x5b8]
14146df7a: 48 83 fe 03                 	cmp	rsi, 0x3
14146df7e: 74 26                       	je	0x14146dfa6 <.text+0x146cfa6>
14146df80: 8b bd b0 05 00 00           	mov	edi, dword ptr [rbp + 0x5b0]
14146df86: 48 8d 95 c0 05 00 00        	lea	rdx, [rbp + 0x5c0]
14146df8d: 48 8d 4d b0                 	lea	rcx, [rbp - 0x50]
14146df91: 41 b8 a8 01 00 00           	mov	r8d, 0x1a8
14146df97: e8 64 5c 4b 01              	call	0x142923c00 <.text+0x2922c00>
14146df9c: 48 83 fe 02                 	cmp	rsi, 0x2
14146dfa0: 0f 85 8b 0b 00 00           	jne	0x14146eb31 <.text+0x146db31>
14146dfa6: 48 8b bd 18 09 00 00        	mov	rdi, qword ptr [rbp + 0x918]
14146dfad: c7 47 68 ff ff ff ff        	mov	dword ptr [rdi + 0x68], 0xffffffff
14146dfb4: 48 8b 47 10                 	mov	rax, qword ptr [rdi + 0x10]
14146dfb8: 48 85 c0                    	test	rax, rax
14146dfbb: 0f 84 e5 07 00 00           	je	0x14146e7a6 <.text+0x146d7a6>
14146dfc1: 48 8b 4f 08                 	mov	rcx, qword ptr [rdi + 0x8]
14146dfc5: 48 8d 04 80                 	lea	rax, [rax + 4*rax]
14146dfc9: 48 8d 04 c1                 	lea	rax, [rcx + 8*rax]
14146dfcd: 48 89 85 38 09 00 00        	mov	qword ptr [rbp + 0x938], rax
14146dfd4: 45 31 c0                    	xor	r8d, r8d
14146dfd7: 49 ba 01 00 00 00 00 00 00 80       	movabs	r10, -0x7fffffffffffffff
14146dfe1: 49 bb 10 00 00 00 00 00 00 80       	movabs	r11, -0x7ffffffffffffff0
14146dfeb: 48 8d 3d fe 6b 7f 01        	lea	rdi, [rip + 0x17f6bfe]  # 0x142c64bf0
14146dff2: 45 31 c9                    	xor	r9d, r9d
14146dff5: eb 21                       	jmp	0x14146e018 <.text+0x146d018>
14146dff7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
14146e000: 48 8b 8d 28 09 00 00        	mov	rcx, qword ptr [rbp + 0x928]
14146e007: 48 83 c1 28                 	add	rcx, 0x28
14146e00b: 48 3b 8d 38 09 00 00        	cmp	rcx, qword ptr [rbp + 0x938]
14146e012: 0f 84 91 07 00 00           	je	0x14146e7a9 <.text+0x146d7a9>
14146e018: 48 8b 41 10                 	mov	rax, qword ptr [rcx + 0x10]
14146e01c: 48 85 c0                    	test	rax, rax
14146e01f: 48 89 8d 28 09 00 00        	mov	qword ptr [rbp + 0x928], rcx
14146e026: 74 d8                       	je	0x14146e000 <.text+0x146d000>
14146e028: 4c 8b 61 08                 	mov	r12, qword ptr [rcx + 0x8]
14146e02c: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
14146e030: 49 8d 04 c4                 	lea	rax, [r12 + 8*rax]
14146e034: 48 89 85 58 09 00 00        	mov	qword ptr [rbp + 0x958], rax
14146e03b: eb 2e                       	jmp	0x14146e06b <.text+0x146d06b>
14146e03d: 0f 1f 00                    	nop	dword ptr [rax]
14146e040: 49 8b 44 24 08              	mov	rax, qword ptr [r12 + 0x8]
14146e045: 0f b6 50 10                 	movzx	edx, byte ptr [rax + 0x10]
14146e049: 48 8d 0d 78 6b 7f 01        	lea	rcx, [rip + 0x17f6b78]  # 0x142c64bc8
14146e050: 48 03 04 d1                 	add	rax, qword ptr [rcx + 8*rdx]
14146e054: 48 8b 00                    	mov	rax, qword ptr [rax]
14146e057: 4c 39 c8                    	cmp	rax, r9
14146e05a: 4c 0f 47 c8                 	cmova	r9, rax
14146e05e: 49 83 c4 18                 	add	r12, 0x18
