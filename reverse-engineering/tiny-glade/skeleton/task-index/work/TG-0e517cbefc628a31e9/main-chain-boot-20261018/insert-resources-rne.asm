
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140940d80: 56                          	push	rsi
140940d81: 57                          	push	rdi
140940d82: 48 81 ec d8 00 00 00        	sub	rsp, 0xd8
140940d89: 0f 29 b4 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm6
140940d91: e8 aa 9b bf 00              	call	0x14153a940 <.text+0x1539940>
140940d96: 48 89 c6                    	mov	rsi, rax
140940d99: 48 8d 7c 24 20              	lea	rdi, [rsp + 0x20]
140940d9e: 48 89 f9                    	mov	rcx, rdi
140940da1: e8 5a 67 35 00              	call	0x140c97500 <.text+0xc96500>
140940da6: 48 89 f1                    	mov	rcx, rsi
140940da9: 48 89 fa                    	mov	rdx, rdi
140940dac: e8 ff 67 bf 00              	call	0x1415375b0 <.text+0x15365b0>
140940db1: c7 44 24 20 00 00 00 00     	mov	dword ptr [rsp + 0x20], 0x0
140940db9: 0f 57 f6                    	xorps	xmm6, xmm6
140940dbc: 0f 11 74 24 30              	movups	xmmword ptr [rsp + 0x30], xmm6
140940dc1: 48 8d 54 24 20              	lea	rdx, [rsp + 0x20]
140940dc6: 48 89 c1                    	mov	rcx, rax
140940dc9: e8 b2 3e c0 00              	call	0x141544c80 <.text+0x1543c80>
140940dce: 0f 57 c9                    	xorps	xmm1, xmm1
140940dd1: 0f 57 d2                    	xorps	xmm2, xmm2
140940dd4: 48 89 c1                    	mov	rcx, rax
140940dd7: e8 24 be bf 00              	call	0x14153cc00 <.text+0x153bc00>
140940ddc: c7 44 24 28 00 00 00 00     	mov	dword ptr [rsp + 0x28], 0x0
140940de4: 48 c7 44 24 20 00 00 00 00  	mov	qword ptr [rsp + 0x20], 0x0
140940ded: f2 0f 10 05 3b c5 08 02     	movsd	xmm0, qword ptr [rip + 0x208c53b] # 0x1429cd330
140940df5: f2 0f 11 44 24 2c           	movsd	qword ptr [rsp + 0x2c], xmm0
140940dfb: c7 44 24 34 00 00 80 bf     	mov	dword ptr [rsp + 0x34], 0xbf800000
140940e03: 48 8d 54 24 20              	lea	rdx, [rsp + 0x20]
140940e08: 48 89 c1                    	mov	rcx, rax
140940e0b: e8 a0 47 bf 00              	call	0x1415355b0 <.text+0x15345b0>
140940e10: 0f 29 74 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm6
140940e15: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140940e1a: 0f 29 74 24 20              	movaps	xmmword ptr [rsp + 0x20], xmm6
140940e1f: 48 c7 44 24 50 00 00 00 00  	mov	qword ptr [rsp + 0x50], 0x0
140940e28: 48 8d 54 24 20              	lea	rdx, [rsp + 0x20]
140940e2d: 48 89 c1                    	mov	rcx, rax
140940e30: e8 4b 85 bf 00              	call	0x141539380 <.text+0x1538380>
140940e35: c7 44 24 20 00 00 00 00     	mov	dword ptr [rsp + 0x20], 0x0
140940e3d: 48 8d 54 24 20              	lea	rdx, [rsp + 0x20]
140940e42: 48 89 c1                    	mov	rcx, rax
140940e45: e8 26 c9 bf 00              	call	0x14153d770 <.text+0x153c770>
140940e4a: c7 44 24 20 00 00 00 00     	mov	dword ptr [rsp + 0x20], 0x0
140940e52: 48 8d 54 24 20              	lea	rdx, [rsp + 0x20]
140940e57: 48 89 c1                    	mov	rcx, rax
140940e5a: e8 d1 51 bf 00              	call	0x141536030 <.text+0x1535030>
140940e5f: c7 44 24 30 00 00 00 00     	mov	dword ptr [rsp + 0x30], 0x0
140940e67: c7 44 24 38 00 00 00 00     	mov	dword ptr [rsp + 0x38], 0x0
140940e6f: c7 44 24 40 00 00 00 00     	mov	dword ptr [rsp + 0x40], 0x0
140940e77: c7 44 24 48 00 00 00 00     	mov	dword ptr [rsp + 0x48], 0x0
140940e7f: 48 c7 44 24 20 00 00 00 00  	mov	qword ptr [rsp + 0x20], 0x0
140940e88: 48 8d 54 24 20              	lea	rdx, [rsp + 0x20]
140940e8d: 48 89 c1                    	mov	rcx, rax
140940e90: e8 5b 95 bf 00              	call	0x14153a3f0 <.text+0x15393f0>
140940e95: 48 c7 44 24 48 00 00 00 00  	mov	qword ptr [rsp + 0x48], 0x0
140940e9e: 0f 11 74 24 30              	movups	xmmword ptr [rsp + 0x30], xmm6
140940ea3: 48 c7 44 24 20 00 00 00 00  	mov	qword ptr [rsp + 0x20], 0x0
140940eac: c7 44 24 28 00 00 00 00     	mov	dword ptr [rsp + 0x28], 0x0
140940eb4: 66 c7 44 24 50 01 00        	mov	word ptr [rsp + 0x50], 0x1
140940ebb: c7 44 24 40 00 00 00 00     	mov	dword ptr [rsp + 0x40], 0x0
140940ec3: 48 8d 54 24 20              	lea	rdx, [rsp + 0x20]
140940ec8: 48 89 c1                    	mov	rcx, rax
140940ecb: e8 a0 ab bf 00              	call	0x14153ba70 <.text+0x153aa70>
140940ed0: 66 c7 44 24 48 cf 00        	mov	word ptr [rsp + 0x48], 0xcf
140940ed7: c7 44 24 20 00 00 00 00     	mov	dword ptr [rsp + 0x20], 0x0
140940edf: 0f 28 05 9a 14 17 02        	movaps	xmm0, xmmword ptr [rip + 0x217149a] # 0x142ab2380
140940ee6: 0f 11 44 24 28              	movups	xmmword ptr [rsp + 0x28], xmm0
140940eeb: f2 0f 10 05 9d 14 17 02     	movsd	xmm0, qword ptr [rip + 0x217149d] # 0x142ab2390
140940ef3: 0f 11 44 24 38              	movups	xmmword ptr [rsp + 0x38], xmm0
140940ef8: 48 8d 54 24 20              	lea	rdx, [rsp + 0x20]
140940efd: 48 89 c1                    	mov	rcx, rax
140940f00: e8 9b 8f bf 00              	call	0x141539ea0 <.text+0x1538ea0>
140940f05: f2 0f 10 05 a3 36 ff 01     	movsd	xmm0, qword ptr [rip + 0x1ff36a3] # 0x1429345b0
140940f0d: 0f 29 44 24 20              	movaps	xmmword ptr [rsp + 0x20], xmm0
140940f12: 48 8d 54 24 20              	lea	rdx, [rsp + 0x20]
140940f17: 48 89 c1                    	mov	rcx, rax
140940f1a: e8 d1 d2 bf 00              	call	0x14153e1f0 <.text+0x153d1f0>
140940f1f: c6 44 24 34 00              	mov	byte ptr [rsp + 0x34], 0x0
140940f24: 48 c7 44 24 2c 00 00 00 00  	mov	qword ptr [rsp + 0x2c], 0x0
140940f2d: 48 b9 00 00 00 00 08 00 00 00       	movabs	rcx, 0x800000000
140940f37: 48 89 4c 24 38              	mov	qword ptr [rsp + 0x38], rcx
140940f3c: c7 44 24 40 00 00 00 00     	mov	dword ptr [rsp + 0x40], 0x0
140940f44: c6 44 24 62 01              	mov	byte ptr [rsp + 0x62], 0x1
140940f49: 66 c7 44 24 60 00 00        	mov	word ptr [rsp + 0x60], 0x0
140940f50: c7 44 24 20 00 00 00 00     	mov	dword ptr [rsp + 0x20], 0x0
140940f58: 0f 11 74 24 44              	movups	xmmword ptr [rsp + 0x44], xmm6
140940f5d: 48 c7 44 24 54 00 00 00 00  	mov	qword ptr [rsp + 0x54], 0x0
140940f66: c7 44 24 5c 00 00 80 3f     	mov	dword ptr [rsp + 0x5c], 0x3f800000
140940f6e: c7 44 24 28 a3 86 01 00     	mov	dword ptr [rsp + 0x28], 0x186a3
140940f76: 48 8d 54 24 20              	lea	rdx, [rsp + 0x20]
140940f7b: 48 89 c1                    	mov	rcx, rax
140940f7e: e8 cd d6 c0 00              	call	0x14154e650 <.text+0x154d650>
140940f83: 0f 11 b4 24 8c 00 00 00     	movups	xmmword ptr [rsp + 0x8c], xmm6
140940f8b: 0f 29 b4 24 80 00 00 00     	movaps	xmmword ptr [rsp + 0x80], xmm6
140940f93: 0f 29 74 24 70              	movaps	xmmword ptr [rsp + 0x70], xmm6
140940f98: 0f 29 74 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm6
140940f9d: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
140940fa2: 0f 29 74 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm6
140940fa7: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140940fac: 0f 29 74 24 20              	movaps	xmmword ptr [rsp + 0x20], xmm6
140940fb1: 48 8d 54 24 20              	lea	rdx, [rsp + 0x20]
140940fb6: 48 89 c1                    	mov	rcx, rax
140940fb9: e8 42 a3 c0 00              	call	0x14154b300 <.text+0x154a300>
140940fbe: 48 c7 44 24 20 00 00 00 00  	mov	qword ptr [rsp + 0x20], 0x0
140940fc7: 48 c7 44 24 28 08 00 00 00  	mov	qword ptr [rsp + 0x28], 0x8
140940fd0: 48 c7 44 24 30 00 00 00 00  	mov	qword ptr [rsp + 0x30], 0x0
140940fd9: 48 8d 54 24 20              	lea	rdx, [rsp + 0x20]
140940fde: 48 89 c1                    	mov	rcx, rax
140940fe1: e8 9a 77 bf 00              	call	0x141538780 <.text+0x1537780>
140940fe6: 48 89 c6                    	mov	rsi, rax
140940fe9: e8 52 c7 40 00              	call	0x140d4d740 <.text+0xd4c740>
140940fee: 48 89 c7                    	mov	rdi, rax
140940ff1: f6 00 01                    	test	byte ptr [rax], 0x1
140940ff4: 74 11                       	je	0x140941007 <.text+0x940007>
140940ff6: 48 89 f9                    	mov	rcx, rdi
140940ff9: 48 83 c1 08                 	add	rcx, 0x8
140940ffd: 48 8b 47 08                 	mov	rax, qword ptr [rdi + 0x8]
140941001: 48 8b 57 10                 	mov	rdx, qword ptr [rdi + 0x10]
140941005: eb 17                       	jmp	0x14094101e <.text+0x94001e>
140941007: e8 f4 3d 42 00              	call	0x140d64e00 <.text+0xd63e00>
14094100c: 48 89 f9                    	mov	rcx, rdi
14094100f: 48 83 c1 08                 	add	rcx, 0x8
140941013: 48 c7 07 01 00 00 00        	mov	qword ptr [rdi], 0x1
14094101a: 48 89 57 10                 	mov	qword ptr [rdi + 0x10], rdx
14094101e: 4c 8d 40 01                 	lea	r8, [rax + 0x1]
140941022: 4c 89 01                    	mov	qword ptr [rcx], r8
140941025: 0f 10 05 d4 14 17 02        	movups	xmm0, xmmword ptr [rip + 0x21714d4] # 0x142ab2500
14094102c: 0f 29 44 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm0
140941031: 0f 10 05 b8 14 17 02        	movups	xmm0, xmmword ptr [rip + 0x21714b8] # 0x142ab24f0
140941038: 0f 29 44 24 20              	movaps	xmmword ptr [rsp + 0x20], xmm0
14094103d: 48 89 44 24 40              	mov	qword ptr [rsp + 0x40], rax
140941042: 48 89 54 24 48              	mov	qword ptr [rsp + 0x48], rdx
140941047: 48 8d 54 24 20              	lea	rdx, [rsp + 0x20]
14094104c: 48 89 f1                    	mov	rcx, rsi
14094104f: e8 7c f8 bf 00              	call	0x1415408d0 <.text+0x153f8d0>
140941054: 48 89 c6                    	mov	rsi, rax
140941057: 48 8d 7c 24 20              	lea	rdi, [rsp + 0x20]
14094105c: 48 89 f9                    	mov	rcx, rdi
14094105f: 31 d2                       	xor	edx, edx
140941061: e8 1a 85 b0 00              	call	0x141449580 <.text+0x1448580>
140941066: 0f 10 05 73 1f 1a 02        	movups	xmm0, xmmword ptr [rip + 0x21a1f73] # 0x142ae2fe0
14094106d: 0f 11 44 24 60              	movups	xmmword ptr [rsp + 0x60], xmm0
140941072: 0f 10 05 57 1f 1a 02        	movups	xmm0, xmmword ptr [rip + 0x21a1f57] # 0x142ae2fd0
140941079: 0f 11 44 24 50              	movups	xmmword ptr [rsp + 0x50], xmm0
14094107e: 48 89 f1                    	mov	rcx, rsi
140941081: 48 89 fa                    	mov	rdx, rdi
140941084: e8 07 9e bf 00              	call	0x14153ae90 <.text+0x1539e90>
140941089: 48 89 c6                    	mov	rsi, rax
14094108c: f2 0f 10 05 0c 13 17 02     	movsd	xmm0, qword ptr [rip + 0x217130c] # 0x142ab23a0
140941094: f2 0f 11 84 24 a0 00 00 00  	movsd	qword ptr [rsp + 0xa0], xmm0
14094109d: c7 84 24 a8 00 00 00 00 00 c0 41    	mov	dword ptr [rsp + 0xa8], 0x41c00000
1409410a8: c7 84 24 b8 00 00 00 00 00 00 00    	mov	dword ptr [rsp + 0xb8], 0x0
1409410b3: 48 c7 84 24 b0 00 00 00 00 00 00 00 	mov	qword ptr [rsp + 0xb0], 0x0
1409410bf: 48 8d 7c 24 20              	lea	rdi, [rsp + 0x20]
1409410c4: 48 8d 94 24 a0 00 00 00     	lea	rdx, [rsp + 0xa0]
1409410cc: 4c 8d 84 24 b0 00 00 00     	lea	r8, [rsp + 0xb0]
1409410d4: 48 89 f9                    	mov	rcx, rdi
1409410d7: e8 74 74 1d 00              	call	0x140b18550 <.text+0xb17550>
1409410dc: 48 89 f1                    	mov	rcx, rsi
1409410df: 48 89 fa                    	mov	rdx, rdi
1409410e2: e8 a9 c4 c0 00              	call	0x14154d590 <.text+0x154c590>
1409410e7: 48 89 c1                    	mov	rcx, rax
1409410ea: 31 d2                       	xor	edx, edx
1409410ec: 0f 28 b4 24 c0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xc0]
1409410f4: 48 81 c4 d8 00 00 00        	add	rsp, 0xd8
1409410fb: 5f                          	pop	rdi
1409410fc: 5e                          	pop	rsi
1409410fd: e9 be e9 be 00              	jmp	0x14152fac0 <.text+0x152eac0>
