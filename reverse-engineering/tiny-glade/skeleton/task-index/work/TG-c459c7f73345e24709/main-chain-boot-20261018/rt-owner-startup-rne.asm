BOUND 0x14059dbe8..0x14059dc3b
14059dbe8: 0f b6 05 1a 70 09 03        	movzx	eax, byte ptr [rip + 0x309701a] # 0x143634c09
14059dbef: b9 b0 00 00 00              	mov	ecx, 0xb0
14059dbf4: ba 08 00 00 00              	mov	edx, 0x8
14059dbf9: e8 12 60 07 00              	call	0x140613c10 <.text+0x612c10>
14059dbfe: 48 85 c0                    	test	rax, rax
14059dc01: 0f 84 e6 8e 00 00           	je	0x1405a6aed <.text+0x5a5aed>
14059dc07: 48 c7 00 01 00 00 00        	mov	qword ptr [rax], 0x1
14059dc0e: 48 c7 40 08 01 00 00 00     	mov	qword ptr [rax + 0x8], 0x1
14059dc16: 48 c7 40 10 00 00 00 00     	mov	qword ptr [rax + 0x10], 0x0
14059dc1e: c6 40 18 00                 	mov	byte ptr [rax + 0x18], 0x0
14059dc22: 4c 89 68 20                 	mov	qword ptr [rax + 0x20], r13
14059dc26: c6 80 a0 00 00 00 02        	mov	byte ptr [rax + 0xa0], 0x2
14059dc2d: c6 80 a8 00 00 00 00        	mov	byte ptr [rax + 0xa8], 0x0
14059dc34: 48 89 83 58 01 00 00        	mov	qword ptr [rbx + 0x158], rax
BOUND 0x1405a0066..0x1405a0105
1405a0066: 48 8b 8b 58 01 00 00        	mov	rcx, qword ptr [rbx + 0x158]
1405a006d: f0                          	lock
1405a006e: 48 ff 01                    	inc	qword ptr [rcx]
1405a0071: 0f 8e c9 6f 00 00           	jle	0x1405a7040 <.text+0x5a6040>
1405a0077: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a007e: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a0085: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a008c: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a0093: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a009a: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a00a1: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a00a8: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a00af: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a00b6: e8 c5 dc 0d 00              	call	0x14067dd80 <.text+0x67cd80>
1405a00bb: c6 83 aa 00 00 00 01        	mov	byte ptr [rbx + 0xaa], 0x1
1405a00c2: c6 83 ab 00 00 00 01        	mov	byte ptr [rbx + 0xab], 0x1
1405a00c9: c6 83 ac 00 00 00 01        	mov	byte ptr [rbx + 0xac], 0x1
1405a00d0: c6 83 ad 00 00 00 01        	mov	byte ptr [rbx + 0xad], 0x1
1405a00d7: c6 83 ae 00 00 00 00        	mov	byte ptr [rbx + 0xae], 0x0
1405a00de: c6 83 af 00 00 00 01        	mov	byte ptr [rbx + 0xaf], 0x1
1405a00e5: c6 83 b0 00 00 00 00        	mov	byte ptr [rbx + 0xb0], 0x0
1405a00ec: c6 83 b1 00 00 00 01        	mov	byte ptr [rbx + 0xb1], 0x1
1405a00f3: c6 83 b2 00 00 00 01        	mov	byte ptr [rbx + 0xb2], 0x1
1405a00fa: 48 89 f1                    	mov	rcx, rsi
1405a00fd: 48 89 c2                    	mov	rdx, rax
1405a0100: e8 9b 89 b9 ff              	call	0x140138aa0 <.text+0x137aa0>
BOUND 0x1405a3f71..0x1405a406b
1405a3f71: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
1405a3f7b: 48 89 83 10 0b 00 00        	mov	qword ptr [rbx + 0xb10], rax
1405a3f82: 48 c7 83 00 0b 00 00 00 00 00 00    	mov	qword ptr [rbx + 0xb00], 0x0
1405a3f8d: c6 83 28 0b 00 00 00        	mov	byte ptr [rbx + 0xb28], 0x0
1405a3f94: 0f b6 05 6e 0c 09 03        	movzx	eax, byte ptr [rip + 0x3090c6e] # 0x143634c09
1405a3f9b: b9 12 00 00 00              	mov	ecx, 0x12
1405a3fa0: ba 01 00 00 00              	mov	edx, 0x1
1405a3fa5: e8 66 fc 06 00              	call	0x140613c10 <.text+0x612c10>
1405a3faa: 48 85 c0                    	test	rax, rax
1405a3fad: 48 89 83 28 01 00 00        	mov	qword ptr [rbx + 0x128], rax
1405a3fb4: 0f 84 48 2e 00 00           	je	0x1405a6e02 <.text+0x5a5e02>
1405a3fba: 0f 10 05 2f bb 45 02        	movups	xmm0, xmmword ptr [rip + 0x245bb2f] # 0x1429ffaf0
1405a3fc1: 0f 11 00                    	movups	xmmword ptr [rax], xmm0
1405a3fc4: 66 c7 40 10 65 72           	mov	word ptr [rax + 0x10], 0x7265
1405a3fca: 48 c7 83 90 25 00 00 12 00 00 00    	mov	qword ptr [rbx + 0x2590], 0x12
1405a3fd5: 48 89 83 98 25 00 00        	mov	qword ptr [rbx + 0x2598], rax
1405a3fdc: 48 c7 83 a0 25 00 00 12 00 00 00    	mov	qword ptr [rbx + 0x25a0], 0x12
1405a3fe7: 48 8d 8b 40 04 00 00        	lea	rcx, [rbx + 0x440]
1405a3fee: 48 8d 93 00 0b 00 00        	lea	rdx, [rbx + 0xb00]
1405a3ff5: 4c 8d b3 90 25 00 00        	lea	r14, [rbx + 0x2590]
1405a3ffc: 4d 89 f0                    	mov	r8, r14
1405a3fff: e8 ec 18 7b 00              	call	0x140d558f0 <.text+0xd548f0>
1405a4004: 4c 8b 83 58 01 00 00        	mov	r8, qword ptr [rbx + 0x158]
1405a400b: c6 83 d4 00 00 00 01        	mov	byte ptr [rbx + 0xd4], 0x1
1405a4012: c6 83 d5 00 00 00 01        	mov	byte ptr [rbx + 0xd5], 0x1
1405a4019: c6 83 d6 00 00 00 01        	mov	byte ptr [rbx + 0xd6], 0x1
1405a4020: c6 83 d7 00 00 00 00        	mov	byte ptr [rbx + 0xd7], 0x0
1405a4027: c6 83 d8 00 00 00 01        	mov	byte ptr [rbx + 0xd8], 0x1
1405a402e: 48 8d 8b 00 0b 00 00        	lea	rcx, [rbx + 0xb00]
1405a4035: 48 8d 93 40 04 00 00        	lea	rdx, [rbx + 0x440]
1405a403c: e8 4f 6e ce ff              	call	0x14028ae90 <.text+0x289e90>
1405a4041: 48 83 bb 00 0b 00 00 00     	cmp	qword ptr [rbx + 0xb00], 0x0
1405a4049: 0f 84 ce 2d 00 00           	je	0x1405a6e1d <.text+0x5a5e1d>
1405a404f: 48 8b 83 10 0b 00 00        	mov	rax, qword ptr [rbx + 0xb10]
1405a4056: 48 89 83 a0 02 00 00        	mov	qword ptr [rbx + 0x2a0], rax
1405a405d: 0f 10 83 00 0b 00 00        	movups	xmm0, xmmword ptr [rbx + 0xb00]
1405a4064: 0f 29 83 90 02 00 00        	movaps	xmmword ptr [rbx + 0x290], xmm0
BOUND 0x14059fc62..0x14059fc77
14059fc62: 4c 8d 05 27 fc 45 02        	lea	r8, [rip + 0x245fc27]   # 0x1429ff890
14059fc69: 48 8d 8b 90 25 00 00        	lea	rcx, [rbx + 0x2590]
14059fc70: b2 2c                       	mov	dl, 0x2c
14059fc72: e8 b9 7d e2 ff              	call	0x1403c7a30 <.text+0x3c6a30>
