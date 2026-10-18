
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a29930: 41 57                       	push	r15
140a29932: 41 56                       	push	r14
140a29934: 41 55                       	push	r13
140a29936: 41 54                       	push	r12
140a29938: 56                          	push	rsi
140a29939: 57                          	push	rdi
140a2993a: 55                          	push	rbp
140a2993b: 53                          	push	rbx
140a2993c: 48 81 ec 58 03 00 00        	sub	rsp, 0x358
140a29943: 44 89 cb                    	mov	ebx, r9d
140a29946: 4d 89 c5                    	mov	r13, r8
140a29949: 4c 8b b4 24 e8 03 00 00     	mov	r14, qword ptr [rsp + 0x3e8]
140a29951: 4c 8b bc 24 e0 03 00 00     	mov	r15, qword ptr [rsp + 0x3e0]
140a29959: 4c 8b a4 24 c8 03 00 00     	mov	r12, qword ptr [rsp + 0x3c8]
140a29961: 49 8b b8 50 14 00 00        	mov	rdi, qword ptr [r8 + 0x1450]
140a29968: 48 89 4c 24 30              	mov	qword ptr [rsp + 0x30], rcx
140a2996d: 48 89 54 24 28              	mov	qword ptr [rsp + 0x28], rdx
140a29972: 48 c7 44 24 20 21 00 00 00  	mov	qword ptr [rsp + 0x20], 0x21
140a2997b: 48 8d 15 31 12 0a 02        	lea	rdx, [rip + 0x20a1231]  # 0x142acabb3
140a29982: 4c 8d 0d 3d 12 0a 02        	lea	r9, [rip + 0x20a123d]   # 0x142acabc6
140a29989: 48 8d 6c 24 38              	lea	rbp, [rsp + 0x38]
140a2998e: 41 b8 13 00 00 00           	mov	r8d, 0x13
140a29994: 48 89 e9                    	mov	rcx, rbp
140a29997: e8 04 99 bd 00              	call	0x1416032a0 <.text+0x16022a0>
140a2999c: 49 81 c5 58 14 00 00        	add	r13, 0x1458
140a299a3: 48 8d b4 24 c8 01 00 00     	lea	rsi, [rsp + 0x1c8]
140a299ab: 49 b8 bd cc bd d5 b2 ae 75 2a       	movabs	r8, 0x2a75aeb2d5bdccbd
140a299b5: 48 89 f1                    	mov	rcx, rsi
140a299b8: 48 89 ea                    	mov	rdx, rbp
140a299bb: 4d 89 e9                    	mov	r9, r13
140a299be: e8 dd 48 f4 ff              	call	0x14096e2a0 <.text+0x96d2a0>
140a299c3: 4c 8d 6c 24 38              	lea	r13, [rsp + 0x38]
140a299c8: 49 b8 0e 23 a2 3a 94 79 5e 9b       	movabs	r8, -0x64a1866bc55ddcf2
140a299d2: 4c 89 e9                    	mov	rcx, r13
140a299d5: 48 89 f2                    	mov	rdx, rsi
140a299d8: 4d 89 e1                    	mov	r9, r12
140a299db: e8 e0 40 f4 ff              	call	0x14096dac0 <.text+0x96cac0>
140a299e0: 48 8d b4 24 c8 01 00 00     	lea	rsi, [rsp + 0x1c8]
140a299e8: 49 b8 e3 c5 90 19 c5 9d fe 86       	movabs	r8, -0x7901623ae66f3a1d
140a299f2: 48 89 f1                    	mov	rcx, rsi
140a299f5: 4c 89 ea                    	mov	rdx, r13
140a299f8: e8 13 37 be 00              	call	0x14160d110 <.text+0x160c110>
140a299fd: 4c 8d 64 24 38              	lea	r12, [rsp + 0x38]
140a29a02: 49 b8 df 8b be ec 2e 03 24 d0       	movabs	r8, -0x2fdbfcd113417421
140a29a0c: 4c 89 e1                    	mov	rcx, r12
140a29a0f: 48 89 f2                    	mov	rdx, rsi
140a29a12: 41 b9 01 00 00 00           	mov	r9d, 0x1
140a29a18: e8 33 ea bd 00              	call	0x141608450 <.text+0x1607450>
140a29a1d: 48 8d b4 24 c8 01 00 00     	lea	rsi, [rsp + 0x1c8]
140a29a25: 49 b8 80 8c 9c 6a 6a c3 f9 cd       	movabs	r8, -0x32063c9595637380
140a29a2f: 48 89 f1                    	mov	rcx, rsi
140a29a32: 4c 89 e2                    	mov	rdx, r12
140a29a35: 41 b9 01 00 00 00           	mov	r9d, 0x1
140a29a3b: e8 20 e8 bd 00              	call	0x141608260 <.text+0x1607260>
140a29a40: 4c 8d 64 24 38              	lea	r12, [rsp + 0x38]
140a29a45: 49 b8 ba 2c 07 ed b8 e4 8e 2d       	movabs	r8, 0x2d8ee4b8ed072cba
140a29a4f: 4c 89 e1                    	mov	rcx, r12
140a29a52: 48 89 f2                    	mov	rdx, rsi
140a29a55: 41 b9 01 00 00 00           	mov	r9d, 0x1
140a29a5b: e8 90 f5 bd 00              	call	0x141608ff0 <.text+0x1607ff0>
140a29a60: 48 8d b4 24 c8 01 00 00     	lea	rsi, [rsp + 0x1c8]
140a29a68: 49 b8 ae 0c 04 2a 5e 89 66 40       	movabs	r8, 0x4066895e2a040cae
140a29a72: 48 89 f1                    	mov	rcx, rsi
140a29a75: 4c 89 e2                    	mov	rdx, r12
140a29a78: 4d 89 f9                    	mov	r9, r15
140a29a7b: e8 40 40 f4 ff              	call	0x14096dac0 <.text+0x96cac0>
140a29a80: 4c 8d 7c 24 38              	lea	r15, [rsp + 0x38]
140a29a85: 49 b8 f1 20 e5 f6 e7 17 ab df       	movabs	r8, -0x2054e818091adf0f
140a29a8f: 4c 89 f9                    	mov	rcx, r15
140a29a92: 48 89 f2                    	mov	rdx, rsi
140a29a95: 4d 89 f1                    	mov	r9, r14
140a29a98: e8 23 40 f4 ff              	call	0x14096dac0 <.text+0x96cac0>
140a29a9d: 48 8d b4 24 c8 01 00 00     	lea	rsi, [rsp + 0x1c8]
140a29aa5: 49 b8 84 ac 8a 23 de bb 37 53       	movabs	r8, 0x5337bbde238aac84
140a29aaf: 48 89 f1                    	mov	rcx, rsi
140a29ab2: 4c 89 fa                    	mov	rdx, r15
140a29ab5: 4c 8b 8c 24 d8 03 00 00     	mov	r9, qword ptr [rsp + 0x3d8]
140a29abd: e8 fe 3f f4 ff              	call	0x14096dac0 <.text+0x96cac0>
140a29ac2: 88 5c 24 40                 	mov	byte ptr [rsp + 0x40], bl
140a29ac6: 0f b6 84 24 c0 03 00 00     	movzx	eax, byte ptr [rsp + 0x3c0]
140a29ace: 88 44 24 41                 	mov	byte ptr [rsp + 0x41], al
140a29ad2: 48 89 7c 24 38              	mov	qword ptr [rsp + 0x38], rdi
140a29ad7: 4c 8d 44 24 38              	lea	r8, [rsp + 0x38]
140a29adc: 48 89 f1                    	mov	rcx, rsi
140a29adf: 48 8b 94 24 d0 03 00 00     	mov	rdx, qword ptr [rsp + 0x3d0]
140a29ae7: e8 e4 14 be 00              	call	0x14160afd0 <.text+0x1609fd0>
140a29aec: 90                          	nop
140a29aed: 48 81 c4 58 03 00 00        	add	rsp, 0x358
140a29af4: 5b                          	pop	rbx
140a29af5: 5d                          	pop	rbp
140a29af6: 5f                          	pop	rdi
140a29af7: 5e                          	pop	rsi
140a29af8: 41 5c                       	pop	r12
140a29afa: 41 5d                       	pop	r13
140a29afc: 41 5e                       	pop	r14
140a29afe: 41 5f                       	pop	r15
140a29b00: c3                          	ret
