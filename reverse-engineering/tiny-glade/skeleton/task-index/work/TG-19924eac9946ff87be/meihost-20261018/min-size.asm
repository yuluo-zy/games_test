0x140fe0e10 push r15
0x140fe0e12 push r14
0x140fe0e14 push r13
0x140fe0e16 push r12
0x140fe0e18 push rsi
0x140fe0e19 push rdi
0x140fe0e1a push rbp
0x140fe0e1b push rbx
0x140fe0e1c sub rsp, 0x78
0x140fe0e20 movaps xmmword ptr [rsp + 0x60], xmm9
0x140fe0e26 movdqa xmmword ptr [rsp + 0x50], xmm8
0x140fe0e2d movaps xmmword ptr [rsp + 0x40], xmm7
0x140fe0e32 movaps xmmword ptr [rsp + 0x30], xmm6
0x140fe0e37 mov rdi, rdx
0x140fe0e3a mov rsi, rcx
0x140fe0e3d cmp byte ptr [rcx + 0x20], 0
0x140fe0e41 xorps xmm0, xmm0
0x140fe0e44 jne 0x140fe0e4b
0x140fe0e46 movss xmm0, dword ptr [rsi + 0x24]
0x140fe0e4b mov r15, qword ptr [r8]
0x140fe0e4e mov r12, qword ptr [r8 + 8]
0x140fe0e52 mov rax, r12
0x140fe0e55 sub rax, r15
0x140fe0e58 shr rax, 3
0x140fe0e5c xor ecx, ecx
0x140fe0e5e sub rax, 1
0x140fe0e62 cmovae rcx, rax
0x140fe0e66 xorps xmm6, xmm6
0x140fe0e69 cvtsi2ss xmm6, rcx
0x140fe0e6e mulss xmm6, xmm0
0x140fe0e72 mov r13, qword ptr [r8 + 0x10]
0x140fe0e76 cmp byte ptr [rsi + 0x44], 0
0x140fe0e7a je 0x140fe0fbb
0x140fe0e80 cmp r15, r12
0x140fe0e83 je 0x140fe10fb
0x140fe0e89 lea rbx, [r13 + 0x20]
0x140fe0e8d xorps xmm9, xmm9
0x140fe0e91 lea r14, [rsp + 0x28]
0x140fe0e96 pcmpeqd xmm8, xmm8
0x140fe0e9b movaps xmm7, xmm6
0x140fe0e9e jmp 0x140fe0f66
0x140fe0ea3 nop word ptr cs:[rax + rax]
0x140fe0eb0 movdqu xmm1, xmmword ptr [rcx + r8]
0x140fe0eb6 movdqa xmm2, xmm1
0x140fe0eba pcmpeqb xmm2, xmm0
0x140fe0ebe pmovmskb r10d, xmm2
0x140fe0ec3 test r10d, r10d
0x140fe0ec6 je 0x140fe0f00
0x140fe0ec8 tzcnt r11d, r10d
0x140fe0ecd add r11, r8
0x140fe0ed0 and r11, rdx
0x140fe0ed3 neg r11
0x140fe0ed6 lea r11, [r11 + r11*2]
0x140fe0eda shl r11, 6
0x140fe0ede cmp qword ptr [rax + r11], rbp
0x140fe0ee2 je 0x140fe0f30
0x140fe0ee4 lea r11d, [r10 - 1]
0x140fe0ee8 and r11w, r10w
0x140fe0eec mov r10d, r11d
0x140fe0eef jne 0x140fe0ec8
0x140fe0ef1 nop word ptr cs:[rax + rax]
0x140fe0f00 pcmpeqb xmm1, xmm8
0x140fe0f05 pmovmskb r10d, xmm1
0x140fe0f0a test r10d, r10d
0x140fe0f0d jne 0x140fe10e3
0x140fe0f13 add r8, r9
0x140fe0f16 add r8, 0x10
0x140fe0f1a add r9, 0x10
0x140fe0f1e and r8, rdx
0x140fe0f21 jmp 0x140fe0eb0
0x140fe0f23 nop word ptr cs:[rax + rax]
0x140fe0f30 movss xmm0, dword ptr [rcx + r11 - 0x48]
0x140fe0f37 addss xmm7, dword ptr [rcx + r11 - 0x44]
0x140fe0f3e movaps xmm6, xmm9
0x140fe0f42 cmpunordss xmm6, xmm9
0x140fe0f48 movaps xmm1, xmm6
0x140fe0f4b andps xmm1, xmm0
0x140fe0f4e maxss xmm0, xmm9
0x140fe0f53 andnps xmm6, xmm0
0x140fe0f56 orps xmm6, xmm1
0x140fe0f59 movaps xmm9, xmm6
0x140fe0f5d cmp r15, r12
0x140fe0f60 je 0x140fe1101
0x140fe0f66 mov rbp, qword ptr [r15]
0x140fe0f69 mov qword ptr [rsp + 0x28], rbp
0x140fe0f6e cmp qword ptr [r13 + 0x18], 0
0x140fe0f73 je 0x140fe10e3
0x140fe0f79 add r15, 8
0x140fe0f7d mov rcx, rbx
0x140fe0f80 mov rdx, r14
0x140fe0f83 call 0x1405b0d70
0x140fe0f88 mov rcx, qword ptr [r13]
0x140fe0f8c mov rdx, qword ptr [r13 + 8]
0x140fe0f90 mov r8, rdx
0x140fe0f93 and r8, rax
0x140fe0f96 shr rax, 0x39
0x140fe0f9a movd xmm0, eax
0x140fe0f9e punpcklbw xmm0, xmm0
0x140fe0fa2 pshuflw xmm0, xmm0, 0
0x140fe0fa7 pshufd xmm0, xmm0, 0
0x140fe0fac lea rax, [rcx - 0xc0]
0x140fe0fb3 xor r9d, r9d
0x140fe0fb6 jmp 0x140fe0eb0
0x140fe0fbb cmp r15, r12
0x140fe0fbe je 0x140fe1208
0x140fe0fc4 lea rbx, [r13 + 0x20]
0x140fe0fc8 xorps xmm9, xmm9
0x140fe0fcc lea r14, [rsp + 0x28]
0x140fe0fd1 pcmpeqd xmm8, xmm8
0x140fe0fd6 jmp 0x140fe1092
0x140fe0fdb nop dword ptr [rax + rax]
0x140fe0fe0 movdqu xmm1, xmmword ptr [rcx + r8]
0x140fe0fe6 movdqa xmm2, xmm1
0x140fe0fea pcmpeqb xmm2, xmm0
0x140fe0fee pmovmskb r10d, xmm2
0x140fe0ff3 test r10d, r10d
0x140fe0ff6 je 0x140fe1030
0x140fe0ff8 tzcnt r11d, r10d
0x140fe0ffd add r11, r8
0x140fe1000 and r11, rdx
0x140fe1003 neg r11
0x140fe1006 lea r11, [r11 + r11*2]
0x140fe100a shl r11, 6
0x140fe100e cmp qword ptr [rax + r11], rbp
0x140fe1012 je 0x140fe1060
0x140fe1014 lea r11d, [r10 - 1]
0x140fe1018 and r11w, r10w
0x140fe101c mov r10d, r11d
0x140fe101f jne 0x140fe0ff8
0x140fe1021 nop word ptr cs:[rax + rax]
0x140fe1030 pcmpeqb xmm1, xmm8
0x140fe1035 pmovmskb r10d, xmm1
0x140fe103a test r10d, r10d
0x140fe103d jne 0x140fe10e3
0x140fe1043 add r8, r9
0x140fe1046 add r8, 0x10
0x140fe104a add r9, 0x10
0x140fe104e and r8, rdx
0x140fe1051 jmp 0x140fe0fe0
0x140fe1053 nop word ptr cs:[rax + rax]
0x140fe1060 movss xmm0, dword ptr [rcx + r11 - 0x44]
0x140fe1067 addss xmm6, dword ptr [rcx + r11 - 0x48]
0x140fe106e movaps xmm7, xmm9
0x140fe1072 cmpunordss xmm7, xmm9
0x140fe1078 movaps xmm1, xmm7
0x140fe107b andps xmm1, xmm0
0x140fe107e maxss xmm0, xmm9
0x140fe1083 andnps xmm7, xmm0
0x140fe1086 orps xmm7, xmm1
0x140fe1089 movaps xmm9, xmm7
0x140fe108d cmp r15, r12
0x140fe1090 je 0x140fe1101
0x140fe1092 mov rbp, qword ptr [r15]
0x140fe1095 mov qword ptr [rsp + 0x28], rbp
0x140fe109a cmp qword ptr [r13 + 0x18], 0
0x140fe109f je 0x140fe10e3
0x140fe10a1 add r15, 8
0x140fe10a5 mov rcx, rbx
0x140fe10a8 mov rdx, r14
0x140fe10ab call 0x1405b0d70
0x140fe10b0 mov rcx, qword ptr [r13]
0x140fe10b4 mov rdx, qword ptr [r13 + 8]
0x140fe10b8 mov r8, rdx
0x140fe10bb and r8, rax
0x140fe10be shr rax, 0x39
0x140fe10c2 movd xmm0, eax
0x140fe10c6 punpcklbw xmm0, xmm0
0x140fe10ca pshuflw xmm0, xmm0, 0
0x140fe10cf pshufd xmm0, xmm0, 0
0x140fe10d4 lea rax, [rcx - 0xc0]
0x140fe10db xor r9d, r9d
0x140fe10de jmp 0x140fe0fe0
0x140fe10e3 lea rcx, [rip + 0x1bcd824]
0x140fe10ea lea r8, [rip + 0x1bcdeef]
0x140fe10f1 mov edx, 0x16
0x140fe10f6 call 0x1428d9330
0x140fe10fb movaps xmm7, xmm6
0x140fe10fe xorps xmm6, xmm6
0x140fe1101 cmp byte ptr [rdi], 0
0x140fe1104 pxor xmm2, xmm2
0x140fe1108 pxor xmm0, xmm0
0x140fe110c je 0x140fe121b
0x140fe1112 cmp byte ptr [rdi + 8], 0
0x140fe1116 pxor xmm1, xmm1
0x140fe111a je 0x140fe122e
0x140fe1120 cmp byte ptr [rsi + 0x28], 0
0x140fe1124 xorps xmm4, xmm4
0x140fe1127 je 0x140fe1240
0x140fe112d cmp byte ptr [rsi + 0x30], 0
0x140fe1131 xorps xmm3, xmm3
0x140fe1134 je 0x140fe1252
0x140fe113a cmp byte ptr [rsi], 0
0x140fe113d xorps xmm5, xmm5
0x140fe1140 je 0x140fe1263
0x140fe1146 cmp byte ptr [rsi + 8], 0
0x140fe114a pxor xmm8, xmm8
0x140fe114f je 0x140fe1277
0x140fe1155 cmp byte ptr [rsi + 0x10], 0
0x140fe1159 xorps xmm9, xmm9
0x140fe115d je 0x140fe128b
0x140fe1163 cmp byte ptr [rsi + 0x18], 0
0x140fe1167 jne 0x140fe116e
0x140fe1169 movd xmm2, dword ptr [rsi + 0x1c]
0x140fe116e addss xmm8, xmm2
0x140fe1173 addss xmm5, xmm9
0x140fe1178 addss xmm6, xmm5
0x140fe117c addss xmm7, xmm8
0x140fe1181 movaps xmm2, xmm4
0x140fe1184 cmpunordss xmm2, xmm4
0x140fe1189 movaps xmm5, xmm2
0x140fe118c andps xmm5, xmm6
0x140fe118f maxss xmm6, xmm4
0x140fe1193 andnps xmm2, xmm6
0x140fe1196 orps xmm2, xmm5
0x140fe1199 movaps xmm4, xmm3
0x140fe119c cmpunordss xmm4, xmm3
0x140fe11a1 movaps xmm5, xmm4
0x140fe11a4 andps xmm5, xmm7
0x140fe11a7 maxss xmm7, xmm3
0x140fe11ab andnps xmm4, xmm7
0x140fe11ae orps xmm4, xmm5
0x140fe11b1 movaps xmm3, xmm2
0x140fe11b4 maxss xmm3, xmm0
0x140fe11b8 cmpunordss xmm0, xmm0
0x140fe11bd movaps xmm5, xmm0
0x140fe11c0 andps xmm5, xmm2
0x140fe11c3 andnps xmm0, xmm3
0x140fe11c6 orps xmm0, xmm5
0x140fe11c9 movaps xmm2, xmm4
0x140fe11cc maxss xmm2, xmm1
0x140fe11d0 cmpunordss xmm1, xmm1
0x140fe11d5 movaps xmm3, xmm1
0x140fe11d8 andps xmm3, xmm4
0x140fe11db andnps xmm1, xmm2
0x140fe11de orps xmm1, xmm3
0x140fe11e1 movaps xmm6, xmmword ptr [rsp + 0x30]
0x140fe11e6 movaps xmm7, xmmword ptr [rsp + 0x40]
0x140fe11eb movaps xmm8, xmmword ptr [rsp + 0x50]
0x140fe11f1 movaps xmm9, xmmword ptr [rsp + 0x60]
0x140fe11f7 add rsp, 0x78
0x140fe11fb pop rbx
0x140fe11fc pop rbp
0x140fe11fd pop rdi
0x140fe11fe pop rsi
0x140fe11ff pop r12
0x140fe1201 pop r13
0x140fe1203 pop r14
0x140fe1205 pop r15
0x140fe1207 ret 
0x140fe1208 xorps xmm7, xmm7
0x140fe120b cmp byte ptr [rdi], 0
0x140fe120e pxor xmm2, xmm2
0x140fe1212 xorps xmm0, xmm0
0x140fe1215 jne 0x140fe1112
0x140fe121b movd xmm0, dword ptr [rdi + 4]
0x140fe1220 cmp byte ptr [rdi + 8], 0
0x140fe1224 pxor xmm1, xmm1
0x140fe1228 jne 0x140fe1120
0x140fe122e movd xmm1, dword ptr [rdi + 0xc]
0x140fe1233 cmp byte ptr [rsi + 0x28], 0
0x140fe1237 xorps xmm4, xmm4
0x140fe123a jne 0x140fe112d
0x140fe1240 movss xmm4, dword ptr [rsi + 0x2c]
0x140fe1245 cmp byte ptr [rsi + 0x30], 0
0x140fe1249 xorps xmm3, xmm3
0x140fe124c jne 0x140fe113a
0x140fe1252 movss xmm3, dword ptr [rsi + 0x34]
0x140fe1257 cmp byte ptr [rsi], 0
0x140fe125a xorps xmm5, xmm5
0x140fe125d jne 0x140fe1146
0x140fe1263 movss xmm5, dword ptr [rsi + 4]
0x140fe1268 cmp byte ptr [rsi + 8], 0
0x140fe126c pxor xmm8, xmm8
0x140fe1271 jne 0x140fe1155
0x140fe1277 movd xmm8, dword ptr [rsi + 0xc]
0x140fe127d cmp byte ptr [rsi + 0x10], 0
0x140fe1281 xorps xmm9, xmm9
0x140fe1285 jne 0x140fe1163
0x140fe128b movss xmm9, dword ptr [rsi + 0x14]
0x140fe1291 cmp byte ptr [rsi + 0x18], 0
0x140fe1295 je 0x140fe1169
0x140fe129b jmp 0x140fe116e
