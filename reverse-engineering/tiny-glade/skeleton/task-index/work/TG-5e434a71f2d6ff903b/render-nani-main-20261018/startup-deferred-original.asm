
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a97a30: 41 57                       	push	r15
140a97a32: 41 56                       	push	r14
140a97a34: 56                          	push	rsi
140a97a35: 57                          	push	rdi
140a97a36: 55                          	push	rbp
140a97a37: 53                          	push	rbx
140a97a38: 48 83 ec 38                 	sub	rsp, 0x38
140a97a3c: 4c 89 ce                    	mov	rsi, r9
140a97a3f: 48 8b 19                    	mov	rbx, qword ptr [rcx]
140a97a42: 48 8b 0b                    	mov	rcx, qword ptr [rbx]
140a97a45: 48 83 c1 10                 	add	rcx, 0x10
140a97a49: 49 8b 38                    	mov	rdi, qword ptr [r8]
140a97a4c: 4c 8b 32                    	mov	r14, qword ptr [rdx]
140a97a4f: 4c 89 74 24 30              	mov	qword ptr [rsp + 0x30], r14
140a97a54: 48 89 7c 24 28              	mov	qword ptr [rsp + 0x28], rdi
140a97a59: 48 c7 44 24 20 13 00 00 00  	mov	qword ptr [rsp + 0x20], 0x13
140a97a62: 48 8d 15 1f 37 04 02        	lea	rdx, [rip + 0x204371f]  # 0x142adb188
140a97a69: 4c 8d 0d 41 37 04 02        	lea	r9, [rip + 0x2043741]   # 0x142adb1b1
140a97a70: 41 b8 29 00 00 00           	mov	r8d, 0x29
140a97a76: e8 c5 83 0a 00              	call	0x140b3fe40 <.text+0xb3ee40>
140a97a7b: 8b 6e 1c                    	mov	ebp, dword ptr [rsi + 0x1c]
140a97a7e: 4c 8b 7e 10                 	mov	r15, qword ptr [rsi + 0x10]
140a97a82: 41 89 2f                    	mov	dword ptr [r15], ebp
140a97a85: 48 8b 36                    	mov	rsi, qword ptr [rsi]
140a97a88: 48 c7 46 10 01 00 00 00     	mov	qword ptr [rsi + 0x10], 0x1
140a97a90: 48 89 46 18                 	mov	qword ptr [rsi + 0x18], rax
140a97a94: 48 8b 0b                    	mov	rcx, qword ptr [rbx]
140a97a97: 48 83 c1 10                 	add	rcx, 0x10
140a97a9b: 4c 89 74 24 30              	mov	qword ptr [rsp + 0x30], r14
140a97aa0: 48 89 7c 24 28              	mov	qword ptr [rsp + 0x28], rdi
140a97aa5: 48 c7 44 24 20 14 00 00 00  	mov	qword ptr [rsp + 0x20], 0x14
140a97aae: 48 8d 15 0f 37 04 02        	lea	rdx, [rip + 0x204370f]  # 0x142adb1c4
140a97ab5: 4c 8d 0d 32 37 04 02        	lea	r9, [rip + 0x2043732]   # 0x142adb1ee
140a97abc: 41 b8 2a 00 00 00           	mov	r8d, 0x2a
140a97ac2: e8 79 83 0a 00              	call	0x140b3fe40 <.text+0xb3ee40>
140a97ac7: 41 89 2f                    	mov	dword ptr [r15], ebp
140a97aca: 48 c7 46 30 01 00 00 00     	mov	qword ptr [rsi + 0x30], 0x1
140a97ad2: 48 89 46 38                 	mov	qword ptr [rsi + 0x38], rax
140a97ad6: 48 8b 0b                    	mov	rcx, qword ptr [rbx]
140a97ad9: 48 83 c1 10                 	add	rcx, 0x10
140a97add: 4c 89 74 24 30              	mov	qword ptr [rsp + 0x30], r14
140a97ae2: 48 89 7c 24 28              	mov	qword ptr [rsp + 0x28], rdi
140a97ae7: 48 c7 44 24 20 13 00 00 00  	mov	qword ptr [rsp + 0x20], 0x13
140a97af0: 48 8d 15 0b 37 04 02        	lea	rdx, [rip + 0x204370b]  # 0x142adb202
140a97af7: 4c 8d 0d 2d 37 04 02        	lea	r9, [rip + 0x204372d]   # 0x142adb22b
140a97afe: 41 b8 29 00 00 00           	mov	r8d, 0x29
140a97b04: e8 37 83 0a 00              	call	0x140b3fe40 <.text+0xb3ee40>
140a97b09: 41 89 2f                    	mov	dword ptr [r15], ebp
140a97b0c: 48 c7 06 01 00 00 00        	mov	qword ptr [rsi], 0x1
140a97b13: 48 89 46 08                 	mov	qword ptr [rsi + 0x8], rax
140a97b17: 48 8b 0b                    	mov	rcx, qword ptr [rbx]
140a97b1a: 48 83 c1 10                 	add	rcx, 0x10
140a97b1e: 4c 89 74 24 30              	mov	qword ptr [rsp + 0x30], r14
140a97b23: 48 89 7c 24 28              	mov	qword ptr [rsp + 0x28], rdi
140a97b28: 48 c7 44 24 20 13 00 00 00  	mov	qword ptr [rsp + 0x20], 0x13
140a97b31: 48 8d 15 06 37 04 02        	lea	rdx, [rip + 0x2043706]  # 0x142adb23e
140a97b38: 4c 8d 0d 28 37 04 02        	lea	r9, [rip + 0x2043728]   # 0x142adb267
140a97b3f: 41 b8 29 00 00 00           	mov	r8d, 0x29
140a97b45: e8 f6 82 0a 00              	call	0x140b3fe40 <.text+0xb3ee40>
140a97b4a: 41 89 2f                    	mov	dword ptr [r15], ebp
140a97b4d: 48 c7 46 20 01 00 00 00     	mov	qword ptr [rsi + 0x20], 0x1
140a97b55: 48 89 46 28                 	mov	qword ptr [rsi + 0x28], rax
140a97b59: 48 83 c4 38                 	add	rsp, 0x38
140a97b5d: 5b                          	pop	rbx
140a97b5e: 5d                          	pop	rbp
140a97b5f: 5f                          	pop	rdi
140a97b60: 5e                          	pop	rsi
140a97b61: 41 5e                       	pop	r14
140a97b63: 41 5f                       	pop	r15
140a97b65: c3                          	ret
