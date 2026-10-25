
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140ad5020: 56                          	push	rsi
140ad5021: 57                          	push	rdi
140ad5022: 48 81 ec b8 00 00 00        	sub	rsp, 0xb8
140ad5029: 44 0f 29 a4 24 a0 00 00 00  	movaps	xmmword ptr [rsp + 0xa0], xmm12
140ad5032: 44 0f 29 9c 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm11
140ad503b: 44 0f 29 94 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm10
140ad5044: 44 0f 29 4c 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm9
140ad504a: 44 0f 29 44 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm8
140ad5050: 0f 29 7c 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm7
140ad5055: 0f 29 74 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm6
140ad505a: 48 89 d7                    	mov	rdi, rdx
140ad505d: 48 89 ce                    	mov	rsi, rcx
140ad5060: f3 0f 10 7a 10              	movss	xmm7, dword ptr [rdx + 0x10]
140ad5065: f3 0f 59 3d 03 08 e5 01     	mulss	xmm7, dword ptr [rip + 0x1e50803] # 0x142925870
140ad506d: 0f 28 c7                    	movaps	xmm0, xmm7
140ad5070: e8 5b ef e4 01              	call	0x142923fd0 <.text+0x2922fd0>
140ad5075: 0f 28 f0                    	movaps	xmm6, xmm0
140ad5078: 0f 28 c7                    	movaps	xmm0, xmm7
140ad507b: e8 00 ee e4 01              	call	0x142923e80 <.text+0x2922e80>
140ad5080: 0f 57 d2                    	xorps	xmm2, xmm2
140ad5083: f3 0f 59 d6                 	mulss	xmm2, xmm6
140ad5087: 0f 10 0f                    	movups	xmm1, xmmword ptr [rdi]
140ad508a: 0f 29 4c 24 20              	movaps	xmmword ptr [rsp + 0x20], xmm1
140ad508f: 0f 28 d9                    	movaps	xmm3, xmm1
140ad5092: 0f c6 d9 ff                 	shufps	xmm3, xmm1, 0xff        # xmm3 = xmm3[3,3],xmm1[3,3]
140ad5096: 0f 13 5c 24 30              	movlps	qword ptr [rsp + 0x30], xmm3
140ad509b: 0f 10 64 24 2c              	movups	xmm4, xmmword ptr [rsp + 0x2c]
140ad50a0: 0f 28 5c 24 30              	movaps	xmm3, xmmword ptr [rsp + 0x30]
140ad50a5: 44 0f 28 ce                 	movaps	xmm9, xmm6
140ad50a9: 44 0f 14 ca                 	unpcklps	xmm9, xmm2              # xmm9 = xmm9[0],xmm2[0],xmm9[1],xmm2[1]
140ad50ad: 41 0f 28 e9                 	movaps	xmm5, xmm9
140ad50b1: 41 0f 58 e9                 	addps	xmm5, xmm9
140ad50b5: f3 0f 59 d5                 	mulss	xmm2, xmm5
140ad50b9: 44 0f 59 cd                 	mulps	xmm9, xmm5
140ad50bd: f3 0f 16 fd                 	movshdup	xmm7, xmm5              # xmm7 = xmm5[1,1,3,3]
140ad50c1: f3 0f 59 f7                 	mulss	xmm6, xmm7
140ad50c5: f3 0f 59 f8                 	mulss	xmm7, xmm0
140ad50c9: f3 0f 59 e8                 	mulss	xmm5, xmm0
140ad50cd: f3 41 0f 16 c1              	movshdup	xmm0, xmm9              # xmm0 = xmm9[1,1,3,3]
140ad50d2: f3 44 0f 58 c8              	addss	xmm9, xmm0
140ad50d7: f3 44 0f 10 05 24 08 e5 01  	movss	xmm8, dword ptr [rip + 0x1e50824] # 0x142925904
140ad50e0: 45 0f 28 d0                 	movaps	xmm10, xmm8
140ad50e4: f3 45 0f 5c d1              	subss	xmm10, xmm9
140ad50e9: 44 0f 28 ca                 	movaps	xmm9, xmm2
140ad50ed: f3 44 0f 58 cf              	addss	xmm9, xmm7
140ad50f2: 44 0f 28 d8                 	movaps	xmm11, xmm0
140ad50f6: f3 44 0f 5c dd              	subss	xmm11, xmm5
140ad50fb: f3 0f 58 e8                 	addss	xmm5, xmm0
140ad50ff: 44 0f 28 e6                 	movaps	xmm12, xmm6
140ad5103: f3 0f 5c f7                 	subss	xmm6, xmm7
140ad5107: 0f 14 ee                    	unpcklps	xmm5, xmm6              # xmm5 = xmm5[0],xmm6[0],xmm5[1],xmm6[1]
140ad510a: 41 0f c6 ea 04              	shufps	xmm5, xmm10, 0x4        # xmm5 = xmm5[0,1],xmm10[0,0]
140ad510f: 45 0f 14 d1                 	unpcklps	xmm10, xmm9             # xmm10 = xmm10[0],xmm9[0],xmm10[1],xmm9[1]
140ad5113: 45 0f c6 d3 04              	shufps	xmm10, xmm11, 0x4       # xmm10 = xmm10[0,1],xmm11[0,0]
140ad5118: f3 0f 5c d7                 	subss	xmm2, xmm7
140ad511c: f3 0f 58 c0                 	addss	xmm0, xmm0
140ad5120: f3 44 0f 5c c0              	subss	xmm8, xmm0
140ad5125: 41 0f 14 d0                 	unpcklps	xmm2, xmm8              # xmm2 = xmm2[0],xmm8[0],xmm2[1],xmm8[1]
140ad5129: f3 44 0f 58 e7              	addss	xmm12, xmm7
140ad512e: 41 0f c6 d4 04              	shufps	xmm2, xmm12, 0x4        # xmm2 = xmm2[0,1],xmm12[0,0]
140ad5133: 0f c6 e4 00                 	shufps	xmm4, xmm4, 0x0         # xmm4 = xmm4[0,0,0,0]
140ad5137: 41 0f 59 e2                 	mulps	xmm4, xmm10
140ad513b: 0f 28 c3                    	movaps	xmm0, xmm3
140ad513e: 0f c6 c3 00                 	shufps	xmm0, xmm3, 0x0         # xmm0 = xmm0[0,0],xmm3[0,0]
140ad5142: 0f 59 c2                    	mulps	xmm0, xmm2
140ad5145: 0f c6 db 55                 	shufps	xmm3, xmm3, 0x55        # xmm3 = xmm3[1,1,1,1]
140ad5149: 0f 59 dd                    	mulps	xmm3, xmm5
140ad514c: 0f c6 c9 a4                 	shufps	xmm1, xmm1, 0xa4        # xmm1 = xmm1[0,1,2,2]
140ad5150: 0f 29 26                    	movaps	xmmword ptr [rsi], xmm4
140ad5153: 0f 29 46 10                 	movaps	xmmword ptr [rsi + 0x10], xmm0
140ad5157: 0f 29 5e 20                 	movaps	xmmword ptr [rsi + 0x20], xmm3
140ad515b: 0f 29 4e 30                 	movaps	xmmword ptr [rsi + 0x30], xmm1
140ad515f: 48 89 f0                    	mov	rax, rsi
140ad5162: 0f 28 74 24 40              	movaps	xmm6, xmmword ptr [rsp + 0x40]
140ad5167: 0f 28 7c 24 50              	movaps	xmm7, xmmword ptr [rsp + 0x50]
140ad516c: 44 0f 28 44 24 60           	movaps	xmm8, xmmword ptr [rsp + 0x60]
140ad5172: 44 0f 28 4c 24 70           	movaps	xmm9, xmmword ptr [rsp + 0x70]
140ad5178: 44 0f 28 94 24 80 00 00 00  	movaps	xmm10, xmmword ptr [rsp + 0x80]
140ad5181: 44 0f 28 9c 24 90 00 00 00  	movaps	xmm11, xmmword ptr [rsp + 0x90]
140ad518a: 44 0f 28 a4 24 a0 00 00 00  	movaps	xmm12, xmmword ptr [rsp + 0xa0]
140ad5193: 48 81 c4 b8 00 00 00        	add	rsp, 0xb8
140ad519a: 5f                          	pop	rdi
140ad519b: 5e                          	pop	rsi
140ad519c: c3                          	ret
140ad519d: cc                          	int3
140ad519e: cc                          	int3
140ad519f: cc                          	int3
140ad51a0: 48 89 c8                    	mov	rax, rcx
140ad51a3: f3 0f 10 42 08              	movss	xmm0, dword ptr [rdx + 0x8]
140ad51a8: f2 0f 10 0a                 	movsd	xmm1, qword ptr [rdx]
140ad51ac: f2 0f 11 09                 	movsd	qword ptr [rcx], xmm1
140ad51b0: f3 0f 11 41 08              	movss	dword ptr [rcx + 0x8], xmm0
140ad51b5: c3                          	ret
