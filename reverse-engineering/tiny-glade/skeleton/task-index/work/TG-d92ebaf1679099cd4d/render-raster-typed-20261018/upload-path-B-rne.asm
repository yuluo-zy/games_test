
/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1403d8bd0: 55                          	push	rbp
1403d8bd1: 41 57                       	push	r15
1403d8bd3: 41 56                       	push	r14
1403d8bd5: 41 55                       	push	r13
1403d8bd7: 41 54                       	push	r12
1403d8bd9: 56                          	push	rsi
1403d8bda: 57                          	push	rdi
1403d8bdb: 53                          	push	rbx
1403d8bdc: 48 81 ec b8 03 00 00        	sub	rsp, 0x3b8
1403d8be3: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
1403d8beb: 48 c7 85 30 03 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x330], -0x2
1403d8bf6: 44 89 c3                    	mov	ebx, r8d
1403d8bf9: 48 89 d6                    	mov	rsi, rdx
1403d8bfc: 48 89 cf                    	mov	rdi, rcx
1403d8bff: 48 8d 4a 38                 	lea	rcx, [rdx + 0x38]
1403d8c03: 48 ba de d2 58 3d d2 87 c9 85       	movabs	rdx, -0x7a36782dc2a72d22
1403d8c0d: 49 b8 3b 20 77 1f 23 ab c6 2e       	movabs	r8, 0x2ec6ab231f77203b
1403d8c17: e8 d4 4a d0 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1403d8c1c: 48 85 c0                    	test	rax, rax
1403d8c1f: 0f 84 c4 02 00 00           	je	0x1403d8ee9 <.text+0x3d7ee9>
1403d8c25: 48 39 96 18 02 00 00        	cmp	qword ptr [rsi + 0x218], rdx
1403d8c2c: 0f 86 b7 02 00 00           	jbe	0x1403d8ee9 <.text+0x3d7ee9>
1403d8c32: 48 8b 86 10 02 00 00        	mov	rax, qword ptr [rsi + 0x210]
1403d8c39: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1403d8c3d: 48 85 c0                    	test	rax, rax
1403d8c40: 0f 84 a3 02 00 00           	je	0x1403d8ee9 <.text+0x3d7ee9>
1403d8c46: 4c 8b be e0 01 00 00        	mov	r15, qword ptr [rsi + 0x1e0]
1403d8c4d: 48 f7 d0                    	not	rax
1403d8c50: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1403d8c54: 48 c1 e0 05                 	shl	rax, 0x5
1403d8c58: 49 83 7c 07 20 00           	cmp	qword ptr [r15 + rax + 0x20], 0x0
1403d8c5e: 0f 84 85 02 00 00           	je	0x1403d8ee9 <.text+0x3d7ee9>
1403d8c64: 49 01 c7                    	add	r15, rax
1403d8c67: 4d 8b 77 10                 	mov	r14, qword ptr [r15 + 0x10]
1403d8c6b: 44 8b a6 b0 05 00 00        	mov	r12d, dword ptr [rsi + 0x5b0]
1403d8c72: 4c 89 f1                    	mov	rcx, r14
1403d8c75: e8 06 51 2a 00              	call	0x14067dd80 <.text+0x67cd80>
1403d8c7a: 80 b8 9a 02 00 00 00        	cmp	byte ptr [rax + 0x29a], 0x0
1403d8c81: 0f 84 4d 02 00 00           	je	0x1403d8ed4 <.text+0x3d7ed4>
1403d8c87: 48 8d 8d 48 01 00 00        	lea	rcx, [rbp + 0x148]
1403d8c8e: 89 da                       	mov	edx, ebx
1403d8c90: e8 fb 0b 68 00              	call	0x140a59890 <.text+0xa58890>
1403d8c95: 8b 85 50 01 00 00           	mov	eax, dword ptr [rbp + 0x150]
1403d8c9b: 89 85 1c 03 00 00           	mov	dword ptr [rbp + 0x31c], eax
1403d8ca1: 83 f8 4c                    	cmp	eax, 0x4c
1403d8ca4: 74 17                       	je	0x1403d8cbd <.text+0x3d7cbd>
1403d8ca6: 83 f8 61                    	cmp	eax, 0x61
1403d8ca9: 0f 85 46 02 00 00           	jne	0x1403d8ef5 <.text+0x3d7ef5>
1403d8caf: b8 07 00 00 00              	mov	eax, 0x7
1403d8cb4: 48 8d 0d 5d a9 5f 02        	lea	rcx, [rip + 0x25fa95d]  # 0x1429d3618
1403d8cbb: eb 0c                       	jmp	0x1403d8cc9 <.text+0x3d7cc9>
1403d8cbd: b8 04 00 00 00              	mov	eax, 0x4
1403d8cc2: 48 8d 0d 56 a9 5f 02        	lea	rcx, [rip + 0x25fa956]  # 0x1429d361f
1403d8cc9: 48 89 8d f0 02 00 00        	mov	qword ptr [rbp + 0x2f0], rcx
1403d8cd0: 48 89 85 f8 02 00 00        	mov	qword ptr [rbp + 0x2f8], rax
1403d8cd7: 45 89 67 5c                 	mov	dword ptr [r15 + 0x5c], r12d
1403d8cdb: 4c 89 f1                    	mov	rcx, r14
1403d8cde: e8 9d 50 2a 00              	call	0x14067dd80 <.text+0x67cd80>
1403d8ce3: 66 c7 80 9a 02 00 00 00 01  	mov	word ptr [rax + 0x29a], 0x100
1403d8cec: 4c 8d bd f0 02 00 00        	lea	r15, [rbp + 0x2f0]
1403d8cf3: 4c 89 bd 08 03 00 00        	mov	qword ptr [rbp + 0x308], r15
1403d8cfa: 4c 8d 25 8f 11 d3 ff        	lea	r12, [rip - 0x2cee71]   # 0x140109e90 <.text+0x108e90>
1403d8d01: 4c 89 a5 10 03 00 00        	mov	qword ptr [rbp + 0x310], r12
1403d8d08: 48 8d 05 31 a9 5f 02        	lea	rax, [rip + 0x25fa931]  # 0x1429d3640
1403d8d0f: 48 89 85 48 01 00 00        	mov	qword ptr [rbp + 0x148], rax
1403d8d16: 48 c7 85 50 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x150], 0x1
1403d8d21: 4c 8d ad 08 03 00 00        	lea	r13, [rbp + 0x308]
1403d8d28: 4c 89 ad 58 01 00 00        	mov	qword ptr [rbp + 0x158], r13
1403d8d2f: 48 c7 85 60 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x160], 0x1
1403d8d3a: 48 c7 85 68 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x168], 0x0
1403d8d45: 4c 8d 75 b8                 	lea	r14, [rbp - 0x48]
1403d8d49: 48 8d 95 48 01 00 00        	lea	rdx, [rbp + 0x148]
1403d8d50: 4c 89 f1                    	mov	rcx, r14
1403d8d53: e8 38 12 9a 00              	call	0x140d79f90 <.text+0xd78f90>
1403d8d58: 48 89 7c 24 28              	mov	qword ptr [rsp + 0x28], rdi
1403d8d5d: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
1403d8d62: 48 8d 15 e7 a8 5f 02        	lea	rdx, [rip + 0x25fa8e7]  # 0x1429d3650
1403d8d69: 48 8d b5 48 01 00 00        	lea	rsi, [rbp + 0x148]
1403d8d70: 41 b8 11 00 00 00           	mov	r8d, 0x11
1403d8d76: 48 89 f1                    	mov	rcx, rsi
1403d8d79: 4d 89 f1                    	mov	r9, r14
1403d8d7c: e8 1f e9 1a 00              	call	0x1405876a0 <.text+0x5866a0>
1403d8d81: 48 8d 4d b8                 	lea	rcx, [rbp - 0x48]
1403d8d85: 49 b8 bb d5 b0 36 71 91 52 2f       	movabs	r8, 0x2f52917136b0d5bb
1403d8d8f: 48 89 f2                    	mov	rdx, rsi
1403d8d92: e8 29 fe 1a 00              	call	0x140588bc0 <.text+0x587bc0>
1403d8d97: 4c 89 bd 08 03 00 00        	mov	qword ptr [rbp + 0x308], r15
1403d8d9e: 4c 89 a5 10 03 00 00        	mov	qword ptr [rbp + 0x310], r12
1403d8da5: 48 8d 05 c4 a8 5f 02        	lea	rax, [rip + 0x25fa8c4]  # 0x1429d3670
1403d8dac: 48 89 85 48 01 00 00        	mov	qword ptr [rbp + 0x148], rax
1403d8db3: 48 c7 85 50 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x150], 0x1
1403d8dbe: 4c 89 ad 58 01 00 00        	mov	qword ptr [rbp + 0x158], r13
1403d8dc5: 48 c7 85 60 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x160], 0x1
1403d8dd0: 48 c7 85 68 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x168], 0x0
1403d8ddb: 48 8d 8d d8 02 00 00        	lea	rcx, [rbp + 0x2d8]
1403d8de2: 48 89 f2                    	mov	rdx, rsi
1403d8de5: e8 a6 11 9a 00              	call	0x140d79f90 <.text+0xd78f90>
1403d8dea: 48 8b 85 d8 02 00 00        	mov	rax, qword ptr [rbp + 0x2d8]
1403d8df1: 48 89 85 20 03 00 00        	mov	qword ptr [rbp + 0x320], rax
1403d8df8: 48 8b 8d e0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e0]
1403d8dff: 48 8b 95 e8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2e8]
1403d8e06: c6 85 2f 03 00 00 01        	mov	byte ptr [rbp + 0x32f], 0x1
1403d8e0d: 48 89 8d 00 03 00 00        	mov	qword ptr [rbp + 0x300], rcx
1403d8e14: e8 47 e8 8a 00              	call	0x140c87660 <.text+0xc86660>
1403d8e19: 88 9d 08 03 00 00           	mov	byte ptr [rbp + 0x308], bl
1403d8e1f: c7 85 48 01 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x148], 0x2
1403d8e29: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
1403d8e2e: 48 8d 8d 08 03 00 00        	lea	rcx, [rbp + 0x308]
1403d8e35: 48 8d 55 b8                 	lea	rdx, [rbp - 0x48]
1403d8e39: 49 89 c0                    	mov	r8, rax
1403d8e3c: 41 b1 32                    	mov	r9b, 0x32
1403d8e3f: e8 6c af f8 ff              	call	0x140363db0 <.text+0x362db0>
1403d8e44: 48 8d 8d 48 01 00 00        	lea	rcx, [rbp + 0x148]
1403d8e4b: 48 8d 55 b8                 	lea	rdx, [rbp - 0x48]
1403d8e4f: 41 b8 90 01 00 00           	mov	r8d, 0x190
1403d8e55: e8 a6 ad 54 02              	call	0x142923c00 <.text+0x2922c00>
1403d8e5a: 80 bd 7e 02 00 00 08        	cmp	byte ptr [rbp + 0x27e], 0x8
1403d8e61: 74 28                       	je	0x1403d8e8b <.text+0x3d7e8b>
1403d8e63: 48 8d 8d 58 02 00 00        	lea	rcx, [rbp + 0x258]
1403d8e6a: ba e8 00 00 00              	mov	edx, 0xe8
1403d8e6f: 48 03 95 80 02 00 00        	add	rdx, qword ptr [rbp + 0x280]
1403d8e76: 49 b8 b9 dc 72 6d 28 75 61 b6       	movabs	r8, -0x499e8ad7928d2347
1403d8e80: 41 b9 00 02 00 00           	mov	r9d, 0x200
1403d8e86: e8 e5 66 0b 00              	call	0x14048f570 <.text+0x48e570>
1403d8e8b: 48 8d 75 b8                 	lea	rsi, [rbp - 0x48]
1403d8e8f: 48 8d 95 48 01 00 00        	lea	rdx, [rbp + 0x148]
1403d8e96: 41 b8 90 01 00 00           	mov	r8d, 0x190
1403d8e9c: 48 89 f1                    	mov	rcx, rsi
1403d8e9f: e8 5c ad 54 02              	call	0x142923c00 <.text+0x2922c00>
1403d8ea4: 48 ba 80 00 00 00 00 02 00 00       	movabs	rdx, 0x20000000080
1403d8eae: 48 89 f1                    	mov	rcx, rsi
1403d8eb1: e8 ca 5d 59 00              	call	0x14096ec80 <.text+0x96dc80>
1403d8eb6: 48 8b 95 20 03 00 00        	mov	rdx, qword ptr [rbp + 0x320]
1403d8ebd: 48 85 d2                    	test	rdx, rdx
1403d8ec0: 74 12                       	je	0x1403d8ed4 <.text+0x3d7ed4>
1403d8ec2: 41 b8 01 00 00 00           	mov	r8d, 0x1
1403d8ec8: 48 8b 8d 00 03 00 00        	mov	rcx, qword ptr [rbp + 0x300]
1403d8ecf: e8 4c ad 23 00              	call	0x140613c20 <.text+0x612c20>
1403d8ed4: 90                          	nop
1403d8ed5: 48 81 c4 b8 03 00 00        	add	rsp, 0x3b8
1403d8edc: 5b                          	pop	rbx
1403d8edd: 5f                          	pop	rdi
1403d8ede: 5e                          	pop	rsi
1403d8edf: 41 5c                       	pop	r12
1403d8ee1: 41 5d                       	pop	r13
1403d8ee3: 41 5e                       	pop	r14
1403d8ee5: 41 5f                       	pop	r15
1403d8ee7: 5d                          	pop	rbp
1403d8ee8: c3                          	ret
1403d8ee9: 48 8d 0d 10 a7 5f 02        	lea	rcx, [rip + 0x25fa710]  # 0x1429d3600
1403d8ef0: e8 1b 04 50 02              	call	0x1428d9310 <.text+0x28d8310>
1403d8ef5: 48 8d 85 1c 03 00 00        	lea	rax, [rbp + 0x31c]
1403d8efc: 48 89 45 b8                 	mov	qword ptr [rbp - 0x48], rax
1403d8f00: 48 8d 05 79 ee 8a 00        	lea	rax, [rip + 0x8aee79]   # 0x140c87d80 <.text+0xc86d80>
1403d8f07: 48 89 45 c0                 	mov	qword ptr [rbp - 0x40], rax
1403d8f0b: 48 8d 05 86 a7 5f 02        	lea	rax, [rip + 0x25fa786]  # 0x1429d3698
1403d8f12: 48 89 85 48 01 00 00        	mov	qword ptr [rbp + 0x148], rax
1403d8f19: 48 c7 85 50 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x150], 0x1
1403d8f24: 48 c7 85 68 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x168], 0x0
1403d8f2f: 48 8d 45 b8                 	lea	rax, [rbp - 0x48]
1403d8f33: 48 89 85 58 01 00 00        	mov	qword ptr [rbp + 0x158], rax
1403d8f3a: 48 c7 85 60 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x160], 0x1
1403d8f45: 48 8d 15 5c a7 5f 02        	lea	rdx, [rip + 0x25fa75c]  # 0x1429d36a8
1403d8f4c: 48 8d 8d 48 01 00 00        	lea	rcx, [rbp + 0x148]
1403d8f53: e8 38 04 50 02              	call	0x1428d9390 <.text+0x28d8390>
1403d8f58: cc                          	int3
1403d8f59: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
1403d8f60: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1403d8f65: 55                          	push	rbp
1403d8f66: 41 57                       	push	r15
1403d8f68: 41 56                       	push	r14
1403d8f6a: 41 55                       	push	r13
1403d8f6c: 41 54                       	push	r12
1403d8f6e: 56                          	push	rsi
1403d8f6f: 57                          	push	rdi
1403d8f70: 53                          	push	rbx
1403d8f71: 48 83 ec 38                 	sub	rsp, 0x38
1403d8f75: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1403d8f7c: 48 8d 4d b8                 	lea	rcx, [rbp - 0x48]
1403d8f80: e8 9b 05 23 00              	call	0x140609520 <.text+0x608520>
1403d8f85: 90                          	nop
1403d8f86: 48 83 c4 38                 	add	rsp, 0x38
1403d8f8a: 5b                          	pop	rbx
1403d8f8b: 5f                          	pop	rdi
1403d8f8c: 5e                          	pop	rsi
1403d8f8d: 41 5c                       	pop	r12
1403d8f8f: 41 5d                       	pop	r13
1403d8f91: 41 5e                       	pop	r14
1403d8f93: 41 5f                       	pop	r15
1403d8f95: 5d                          	pop	rbp
1403d8f96: c3                          	ret
1403d8f97: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1403d8fa0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1403d8fa5: 55                          	push	rbp
1403d8fa6: 41 57                       	push	r15
1403d8fa8: 41 56                       	push	r14
1403d8faa: 41 55                       	push	r13
1403d8fac: 41 54                       	push	r12
1403d8fae: 56                          	push	rsi
1403d8faf: 57                          	push	rdi
1403d8fb0: 53                          	push	rbx
1403d8fb1: 48 83 ec 38                 	sub	rsp, 0x38
1403d8fb5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1403d8fbc: 48 83 bd 20 03 00 00 00     	cmp	qword ptr [rbp + 0x320], 0x0
1403d8fc4: 74 19                       	je	0x1403d8fdf <.text+0x3d7fdf>
1403d8fc6: 41 b8 01 00 00 00           	mov	r8d, 0x1
1403d8fcc: 48 8b 8d 00 03 00 00        	mov	rcx, qword ptr [rbp + 0x300]
1403d8fd3: 48 8b 95 20 03 00 00        	mov	rdx, qword ptr [rbp + 0x320]
1403d8fda: e8 41 ac 23 00              	call	0x140613c20 <.text+0x612c20>
1403d8fdf: 90                          	nop
1403d8fe0: 48 83 c4 38                 	add	rsp, 0x38
1403d8fe4: 5b                          	pop	rbx
1403d8fe5: 5f                          	pop	rdi
1403d8fe6: 5e                          	pop	rsi
1403d8fe7: 41 5c                       	pop	r12
1403d8fe9: 41 5d                       	pop	r13
1403d8feb: 41 5e                       	pop	r14
1403d8fed: 41 5f                       	pop	r15
1403d8fef: 5d                          	pop	rbp
1403d8ff0: c3                          	ret
1403d8ff1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
1403d9000: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1403d9005: 55                          	push	rbp
1403d9006: 41 57                       	push	r15
1403d9008: 41 56                       	push	r14
1403d900a: 41 55                       	push	r13
1403d900c: 41 54                       	push	r12
1403d900e: 56                          	push	rsi
1403d900f: 57                          	push	rdi
1403d9010: 53                          	push	rbx
1403d9011: 48 83 ec 38                 	sub	rsp, 0x38
1403d9015: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1403d901c: 80 bd 2f 03 00 00 00        	cmp	byte ptr [rbp + 0x32f], 0x0
1403d9023: 74 1b                       	je	0x1403d9040 <.text+0x3d8040>
1403d9025: 48 8d 4d b8                 	lea	rcx, [rbp - 0x48]
1403d9029: e8 f2 04 23 00              	call	0x140609520 <.text+0x608520>
1403d902e: 90                          	nop
1403d902f: 48 83 c4 38                 	add	rsp, 0x38
1403d9033: 5b                          	pop	rbx
1403d9034: 5f                          	pop	rdi
1403d9035: 5e                          	pop	rsi
1403d9036: 41 5c                       	pop	r12
1403d9038: 41 5d                       	pop	r13
1403d903a: 41 5e                       	pop	r14
1403d903c: 41 5f                       	pop	r15
1403d903e: 5d                          	pop	rbp
1403d903f: c3                          	ret
1403d9040: 48 83 c4 38                 	add	rsp, 0x38
1403d9044: 5b                          	pop	rbx
1403d9045: 5f                          	pop	rdi
1403d9046: 5e                          	pop	rsi
1403d9047: 41 5c                       	pop	r12
1403d9049: 41 5d                       	pop	r13
1403d904b: 41 5e                       	pop	r14
1403d904d: 41 5f                       	pop	r15
1403d904f: 5d                          	pop	rbp
1403d9050: c3                          	ret
1403d9051: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
1403d9060: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1403d9065: 55                          	push	rbp
1403d9066: 41 57                       	push	r15
1403d9068: 41 56                       	push	r14
1403d906a: 41 55                       	push	r13
1403d906c: 41 54                       	push	r12
1403d906e: 56                          	push	rsi
1403d906f: 57                          	push	rdi
1403d9070: 53                          	push	rbx
1403d9071: 48 83 ec 38                 	sub	rsp, 0x38
1403d9075: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1403d907c: c6 85 2f 03 00 00 00        	mov	byte ptr [rbp + 0x32f], 0x0
1403d9083: 48 8d 4d b8                 	lea	rcx, [rbp - 0x48]
1403d9087: e8 94 04 23 00              	call	0x140609520 <.text+0x608520>
1403d908c: c6 85 2f 03 00 00 00        	mov	byte ptr [rbp + 0x32f], 0x0
1403d9093: 48 83 c4 38                 	add	rsp, 0x38
1403d9097: 5b                          	pop	rbx
1403d9098: 5f                          	pop	rdi
1403d9099: 5e                          	pop	rsi
1403d909a: 41 5c                       	pop	r12
1403d909c: 41 5d                       	pop	r13
1403d909e: 41 5e                       	pop	r14
1403d90a0: 41 5f                       	pop	r15
1403d90a2: 5d                          	pop	rbp
1403d90a3: c3                          	ret
1403d90a4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
1403d90b0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1403d90b5: 55                          	push	rbp
1403d90b6: 41 57                       	push	r15
1403d90b8: 41 56                       	push	r14
1403d90ba: 41 55                       	push	r13
1403d90bc: 41 54                       	push	r12
1403d90be: 56                          	push	rsi
1403d90bf: 57                          	push	rdi
1403d90c0: 53                          	push	rbx
1403d90c1: 48 83 ec 38                 	sub	rsp, 0x38
1403d90c5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1403d90cc: 48 8d 8d 48 01 00 00        	lea	rcx, [rbp + 0x148]
1403d90d3: e8 48 04 23 00              	call	0x140609520 <.text+0x608520>
1403d90d8: 90                          	nop
1403d90d9: 48 83 c4 38                 	add	rsp, 0x38
1403d90dd: 5b                          	pop	rbx
1403d90de: 5f                          	pop	rdi
1403d90df: 5e                          	pop	rsi
1403d90e0: 41 5c                       	pop	r12
1403d90e2: 41 5d                       	pop	r13
1403d90e4: 41 5e                       	pop	r14
1403d90e6: 41 5f                       	pop	r15
1403d90e8: 5d                          	pop	rbp
1403d90e9: c3                          	ret
