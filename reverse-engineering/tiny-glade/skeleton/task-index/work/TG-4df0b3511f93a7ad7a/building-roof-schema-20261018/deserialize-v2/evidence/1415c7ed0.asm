0x1415c7ed0 55 push rbp
0x1415c7ed1 4157 push r15
0x1415c7ed3 4156 push r14
0x1415c7ed5 4155 push r13
0x1415c7ed7 4154 push r12
0x1415c7ed9 56 push rsi
0x1415c7eda 57 push rdi
0x1415c7edb 53 push rbx
0x1415c7edc 4881ec88010000 sub rsp, 0x188
0x1415c7ee3 488dac2480000000 lea rbp, [rsp + 0x80]
0x1415c7eeb 440f299df0000000 movaps xmmword ptr [rbp + 0xf0], xmm11
0x1415c7ef3 440f2995e0000000 movaps xmmword ptr [rbp + 0xe0], xmm10
0x1415c7efb 440f298dd0000000 movaps xmmword ptr [rbp + 0xd0], xmm9
0x1415c7f03 440f2985c0000000 movaps xmmword ptr [rbp + 0xc0], xmm8
0x1415c7f0b 0f29bdb0000000 movaps xmmword ptr [rbp + 0xb0], xmm7
0x1415c7f12 0f29b5a0000000 movaps xmmword ptr [rbp + 0xa0], xmm6
0x1415c7f19 48c78598000000feffffff mov qword ptr [rbp + 0x98], 0xfffffffffffffffe
0x1415c7f24 4989cc mov r12, rcx
0x1415c7f27 488b4210 mov rax, qword ptr [rdx + 0x10]
0x1415c7f2b 488945a8 mov qword ptr [rbp - 0x58], rax
0x1415c7f2f 488d5db0 lea rbx, [rbp - 0x50]
0x1415c7f33 4889d9 mov rcx, rbx
0x1415c7f36 e895015cff call 0x140b880d0
0x1415c7f3b 488d7531 lea rsi, [rbp + 0x31]
0x1415c7f3f c7457000000000 mov dword ptr [rbp + 0x70], 0
0x1415c7f46 c6859600000002 mov byte ptr [rbp + 0x96], 2
0x1415c7f4d 4c8d7d78 lea r15, [rbp + 0x78]
0x1415c7f51 488d3d10386b01 lea rdi, [rip + 0x16b3810]
0x1415c7f58 4c8d35ed376b01 lea r14, [rip + 0x16b37ed]
0x1415c7f5f c6859700000002 mov byte ptr [rbp + 0x97], 2
0x1415c7f66 c7457400000000 mov dword ptr [rbp + 0x74], 0
0x1415c7f6d c7456c00000000 mov dword ptr [rbp + 0x6c], 0
0x1415c7f74 c7456800000000 mov dword ptr [rbp + 0x68], 0
0x1415c7f7b c7456400000000 mov dword ptr [rbp + 0x64], 0
0x1415c7f82 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1415c7f90 4c89f9 mov rcx, r15
0x1415c7f93 4889da mov rdx, rbx
0x1415c7f96 e8b544e7ff call 0x14143c450
0x1415c7f9b 488b4578 mov rax, qword ptr [rbp + 0x78]
0x1415c7f9f 4885c0 test rax, rax
0x1415c7fa2 0f84be030000 je 0x1415c8366
0x1415c7fa8 488b8d88000000 mov rcx, qword ptr [rbp + 0x88]
0x1415c7faf 488d1449 lea rdx, [rcx + rcx*2]
0x1415c7fb3 4c8b84d068010000 mov r8, qword ptr [rax + rdx*8 + 0x168]
0x1415c7fbb 4c8b8cd070010000 mov r9, qword ptr [rax + rdx*8 + 0x170]
0x1415c7fc3 4c894d28 mov qword ptr [rbp + 0x28], r9
0x1415c7fc7 4c8bacd078010000 mov r13, qword ptr [rax + rdx*8 + 0x178]
0x1415c7fcf 48c1e105 shl rcx, 5
0x1415c7fd3 0f100408 movups xmm0, xmmword ptr [rax + rcx]
0x1415c7fd7 0f104c0810 movups xmm1, xmmword ptr [rax + rcx + 0x10]
0x1415c7fdc 0f294530 movaps xmmword ptr [rbp + 0x30], xmm0
0x1415c7fe0 0f294d40 movaps xmmword ptr [rbp + 0x40], xmm1
0x1415c7fe4 4c894558 mov qword ptr [rbp + 0x58], r8
0x1415c7fe8 4c89c0 mov rax, r8
0x1415c7feb 48f7d8 neg rax
0x1415c7fee 0f8072030000 jo 0x1415c8366
0x1415c7ff4 0fb645f8 movzx eax, byte ptr [rbp - 8]
0x1415c7ff8 4883f806 cmp rax, 6
0x1415c7ffc 0f8708010000 ja 0x1415c810a
0x1415c8002 49630486 movsxd rax, dword ptr [r14 + rax*4]
0x1415c8006 4c01f0 add rax, r14
0x1415c8009 ffe0 jmp rax
0x1415c800b 0f284530 movaps xmm0, xmmword ptr [rbp + 0x30]
0x1415c800f 0f284d40 movaps xmm1, xmmword ptr [rbp + 0x40]
0x1415c8013 488d45f8 lea rax, [rbp - 8]
0x1415c8017 0f114810 movups xmmword ptr [rax + 0x10], xmm1
0x1415c801b 0f1100 movups xmmword ptr [rax], xmm0
0x1415c801e 4c89f9 mov rcx, r15
0x1415c8021 488b5528 mov rdx, qword ptr [rbp + 0x28]
0x1415c8025 4d89e8 mov r8, r13
0x1415c8028 e8c32d1200 call 0x1416eadf0
0x1415c802d 488b5558 mov rdx, qword ptr [rbp + 0x58]
0x1415c8031 4885d2 test rdx, rdx
0x1415c8034 4d89e5 mov r13, r12
0x1415c8037 740f je 0x1415c8048
0x1415c8039 41b801000000 mov r8d, 1
0x1415c803f 488b4d28 mov rcx, qword ptr [rbp + 0x28]
0x1415c8043 e8d8bb04ff call 0x140613c20
0x1415c8048 807d7800 cmp byte ptr [rbp + 0x78], 0
0x1415c804c 0f85b8030000 jne 0x1415c840a
0x1415c8052 0fb64579 movzx eax, byte ptr [rbp + 0x79]
0x1415c8056 48630487 movsxd rax, dword ptr [rdi + rax*4]
0x1415c805a 4801f8 add rax, rdi
0x1415c805d ffe0 jmp rax
0x1415c805f 837d6400 cmp dword ptr [rbp + 0x64], 0
0x1415c8063 0f852d050000 jne 0x1415c8596
0x1415c8069 0fb645f8 movzx eax, byte ptr [rbp - 8]
0x1415c806d c645f806 mov byte ptr [rbp - 8], 6
0x1415c8071 3c06 cmp al, 6
0x1415c8073 0f8444040000 je 0x1415c84bd
0x1415c8079 884530 mov byte ptr [rbp + 0x30], al
0x1415c807c 0f104349 movups xmm0, xmmword ptr [rbx + 0x49]
0x1415c8080 0f104b58 movups xmm1, xmmword ptr [rbx + 0x58]
0x1415c8084 0f114e0f movups xmmword ptr [rsi + 0xf], xmm1
0x1415c8088 0f1106 movups xmmword ptr [rsi], xmm0
0x1415c808b 4c89f9 mov rcx, r15
0x1415c808e 488d5530 lea rdx, [rbp + 0x30]
0x1415c8092 e8d922fdff call 0x14159a370
0x1415c8097 f6457801 test byte ptr [rbp + 0x78], 1
0x1415c809b 0f8569030000 jne 0x1415c840a
0x1415c80a1 f3440f10557c movss xmm10, dword ptr [rbp + 0x7c]
0x1415c80a7 c7456401000000 mov dword ptr [rbp + 0x64], 1
0x1415c80ae e9ddfeffff jmp 0x1415c7f90
0x1415c80b3 488b5500 mov rdx, qword ptr [rbp]
0x1415c80b7 4885d2 test rdx, rdx
0x1415c80ba 0f844bffffff je 0x1415c800b
0x1415c80c0 488b4d08 mov rcx, qword ptr [rbp + 8]
0x1415c80c4 41b801000000 mov r8d, 1
0x1415c80ca e851bb04ff call 0x140613c20
0x1415c80cf e937ffffff jmp 0x1415c800b
0x1415c80d4 488b4d08 mov rcx, qword ptr [rbp + 8]
0x1415c80d8 488b5510 mov rdx, qword ptr [rbp + 0x10]
0x1415c80dc 48894d20 mov qword ptr [rbp + 0x20], rcx
0x1415c80e0 e88b0c2cff call 0x140888d70
0x1415c80e5 488b5500 mov rdx, qword ptr [rbp]
0x1415c80e9 4885d2 test rdx, rdx
0x1415c80ec 0f8419ffffff je 0x1415c800b
0x1415c80f2 48c1e205 shl rdx, 5
0x1415c80f6 41b808000000 mov r8d, 8
0x1415c80fc 488b4d20 mov rcx, qword ptr [rbp + 0x20]
0x1415c8100 e81bbb04ff call 0x140613c20
0x1415c8105 e901ffffff jmp 0x1415c800b
0x1415c810a 488d4d00 lea rcx, [rbp]
0x1415c810e e82d0e2dff call 0x140898f40
0x1415c8113 e9f3feffff jmp 0x1415c800b
0x1415c8118 80bd9700000002 cmp byte ptr [rbp + 0x97], 2
0x1415c811f 0f8545040000 jne 0x1415c856a
0x1415c8125 0fb645f8 movzx eax, byte ptr [rbp - 8]
0x1415c8129 c645f806 mov byte ptr [rbp - 8], 6
0x1415c812d 3c06 cmp al, 6
0x1415c812f 0f8475030000 je 0x1415c84aa
0x1415c8135 884530 mov byte ptr [rbp + 0x30], al
0x1415c8138 0f104349 movups xmm0, xmmword ptr [rbx + 0x49]
0x1415c813c 0f104b58 movups xmm1, xmmword ptr [rbx + 0x58]
0x1415c8140 0f114e0f movups xmmword ptr [rsi + 0xf], xmm1
0x1415c8144 0f1106 movups xmmword ptr [rsi], xmm0
0x1415c8147 4c89f9 mov rcx, r15
0x1415c814a 488d5530 lea rdx, [rbp + 0x30]
0x1415c814e e84d2e1200 call 0x1416eafa0
0x1415c8153 f6457801 test byte ptr [rbp + 0x78], 1
0x1415c8157 0f85ad020000 jne 0x1415c840a
0x1415c815d 0fb64579 movzx eax, byte ptr [rbp + 0x79]
0x1415c8161 888597000000 mov byte ptr [rbp + 0x97], al
0x1415c8167 e924feffff jmp 0x1415c7f90
0x1415c816c 837d6c00 cmp dword ptr [rbp + 0x6c], 0
0x1415c8170 0f8562040000 jne 0x1415c85d8
0x1415c8176 0fb645f8 movzx eax, byte ptr [rbp - 8]
0x1415c817a c645f806 mov byte ptr [rbp - 8], 6
0x1415c817e 3c06 cmp al, 6
0x1415c8180 0f84fe020000 je 0x1415c8484
0x1415c8186 884530 mov byte ptr [rbp + 0x30], al
0x1415c8189 0f104349 movups xmm0, xmmword ptr [rbx + 0x49]
0x1415c818d 0f104b58 movups xmm1, xmmword ptr [rbx + 0x58]
0x1415c8191 0f114e0f movups xmmword ptr [rsi + 0xf], xmm1
0x1415c8195 0f1106 movups xmmword ptr [rsi], xmm0
0x1415c8198 48c744242002000000 mov qword ptr [rsp + 0x20], 2
0x1415c81a1 41b904000000 mov r9d, 4
0x1415c81a7 4c89f9 mov rcx, r15
0x1415c81aa 488d5530 lea rdx, [rbp + 0x30]
0x1415c81ae 4c8d0593f26a01 lea r8, [rip + 0x16af293]
0x1415c81b5 e856b8fdff call 0x1415a3a10
0x1415c81ba f6457801 test byte ptr [rbp + 0x78], 1
0x1415c81be 0f8546020000 jne 0x1415c840a
0x1415c81c4 f3440f104d7c movss xmm9, dword ptr [rbp + 0x7c]
0x1415c81ca f3440f109d80000000 movss xmm11, dword ptr [rbp + 0x80]
0x1415c81d3 c7456c01000000 mov dword ptr [rbp + 0x6c], 1
0x1415c81da e9b1fdffff jmp 0x1415c7f90
0x1415c81df 837d7400 cmp dword ptr [rbp + 0x74], 0
0x1415c81e3 0f856e030000 jne 0x1415c8557
0x1415c81e9 0fb645f8 movzx eax, byte ptr [rbp - 8]
0x1415c81ed c645f806 mov byte ptr [rbp - 8], 6
0x1415c81f1 3c06 cmp al, 6
0x1415c81f3 0f8478020000 je 0x1415c8471
0x1415c81f9 884530 mov byte ptr [rbp + 0x30], al
0x1415c81fc 0f104349 movups xmm0, xmmword ptr [rbx + 0x49]
0x1415c8200 0f104b58 movups xmm1, xmmword ptr [rbx + 0x58]
0x1415c8204 0f114e0f movups xmmword ptr [rsi + 0xf], xmm1
0x1415c8208 0f1106 movups xmmword ptr [rsi], xmm0
0x1415c820b 4c89f9 mov rcx, r15
0x1415c820e 488d5530 lea rdx, [rbp + 0x30]
0x1415c8212 e85921fdff call 0x14159a370
0x1415c8217 f6457801 test byte ptr [rbp + 0x78], 1
0x1415c821b 0f85e9010000 jne 0x1415c840a
0x1415c8221 f30f107d7c movss xmm7, dword ptr [rbp + 0x7c]
0x1415c8226 c7457401000000 mov dword ptr [rbp + 0x74], 1
0x1415c822d e95efdffff jmp 0x1415c7f90
0x1415c8232 0fb645f8 movzx eax, byte ptr [rbp - 8]
0x1415c8236 c645f806 mov byte ptr [rbp - 8], 6
0x1415c823a 3c06 cmp al, 6
0x1415c823c 0f8419020000 je 0x1415c845b
0x1415c8242 884530 mov byte ptr [rbp + 0x30], al
0x1415c8245 0f104349 movups xmm0, xmmword ptr [rbx + 0x49]
0x1415c8249 0f104b58 movups xmm1, xmmword ptr [rbx + 0x58]
0x1415c824d 0f114e0f movups xmmword ptr [rsi + 0xf], xmm1
0x1415c8251 0f1106 movups xmmword ptr [rsi], xmm0
0x1415c8254 488d4d30 lea rcx, [rbp + 0x30]
0x1415c8258 e8e3b6fdff call 0x1415a3940
0x1415c825d 4885c0 test rax, rax
0x1415c8260 0f842afdffff je 0x1415c7f90
0x1415c8266 e989020000 jmp 0x1415c84f4
0x1415c826b 837d6800 cmp dword ptr [rbp + 0x68], 0
0x1415c826f 0f854d030000 jne 0x1415c85c2
0x1415c8275 0fb645f8 movzx eax, byte ptr [rbp - 8]
0x1415c8279 c645f806 mov byte ptr [rbp - 8], 6
0x1415c827d 3c06 cmp al, 6
0x1415c827f 0f8412020000 je 0x1415c8497
0x1415c8285 884530 mov byte ptr [rbp + 0x30], al
0x1415c8288 0f104349 movups xmm0, xmmword ptr [rbx + 0x49]
0x1415c828c 0f104b58 movups xmm1, xmmword ptr [rbx + 0x58]
0x1415c8290 0f114e0f movups xmmword ptr [rsi + 0xf], xmm1
0x1415c8294 0f1106 movups xmmword ptr [rsi], xmm0
0x1415c8297 4c89f9 mov rcx, r15
0x1415c829a 488d5530 lea rdx, [rbp + 0x30]
0x1415c829e e8cd20fdff call 0x14159a370
0x1415c82a3 f6457801 test byte ptr [rbp + 0x78], 1
0x1415c82a7 0f855d010000 jne 0x1415c840a
0x1415c82ad f3440f10457c movss xmm8, dword ptr [rbp + 0x7c]
0x1415c82b3 c7456801000000 mov dword ptr [rbp + 0x68], 1
0x1415c82ba e9d1fcffff jmp 0x1415c7f90
0x1415c82bf 837d7000 cmp dword ptr [rbp + 0x70], 0
0x1415c82c3 0f85b7020000 jne 0x1415c8580
0x1415c82c9 0fb645f8 movzx eax, byte ptr [rbp - 8]
0x1415c82cd c645f806 mov byte ptr [rbp - 8], 6
0x1415c82d1 3c06 cmp al, 6
0x1415c82d3 0f84f7010000 je 0x1415c84d0
0x1415c82d9 884530 mov byte ptr [rbp + 0x30], al
0x1415c82dc 0f104349 movups xmm0, xmmword ptr [rbx + 0x49]
0x1415c82e0 0f104b58 movups xmm1, xmmword ptr [rbx + 0x58]
0x1415c82e4 0f114e0f movups xmmword ptr [rsi + 0xf], xmm1
0x1415c82e8 0f1106 movups xmmword ptr [rsi], xmm0
0x1415c82eb 4c89f9 mov rcx, r15
0x1415c82ee 488d5530 lea rdx, [rbp + 0x30]
0x1415c82f2 e87920fdff call 0x14159a370
0x1415c82f7 f6457801 test byte ptr [rbp + 0x78], 1
0x1415c82fb 0f8509010000 jne 0x1415c840a
0x1415c8301 f30f10757c movss xmm6, dword ptr [rbp + 0x7c]
0x1415c8306 c7457001000000 mov dword ptr [rbp + 0x70], 1
0x1415c830d e97efcffff jmp 0x1415c7f90
0x1415c8312 80bd9600000002 cmp byte ptr [rbp + 0x96], 2
0x1415c8319 0f858d020000 jne 0x1415c85ac
0x1415c831f 0fb645f8 movzx eax, byte ptr [rbp - 8]
0x1415c8323 c645f806 mov byte ptr [rbp - 8], 6
0x1415c8327 3c06 cmp al, 6
0x1415c8329 0f84b4010000 je 0x1415c84e3
0x1415c832f 884530 mov byte ptr [rbp + 0x30], al
0x1415c8332 0f104349 movups xmm0, xmmword ptr [rbx + 0x49]
0x1415c8336 0f104b58 movups xmm1, xmmword ptr [rbx + 0x58]
0x1415c833a 0f114e0f movups xmmword ptr [rsi + 0xf], xmm1
0x1415c833e 0f1106 movups xmmword ptr [rsi], xmm0
0x1415c8341 4c89f9 mov rcx, r15
0x1415c8344 488d5530 lea rdx, [rbp + 0x30]
0x1415c8348 e8131d1200 call 0x1416ea060
0x1415c834d f6457801 test byte ptr [rbp + 0x78], 1
0x1415c8351 0f85b3000000 jne 0x1415c840a
0x1415c8357 0fb64579 movzx eax, byte ptr [rbp + 0x79]
0x1415c835b 888596000000 mov byte ptr [rbp + 0x96], al
0x1415c8361 e92afcffff jmp 0x1415c7f90
0x1415c8366 f6456401 test byte ptr [rbp + 0x64], 1
0x1415c836a 0f84a6000000 je 0x1415c8416
0x1415c8370 f6456801 test byte ptr [rbp + 0x68], 1
0x1415c8374 4d89e5 mov r13, r12
0x1415c8377 0f84b2000000 je 0x1415c842f
0x1415c837d f6456c01 test byte ptr [rbp + 0x6c], 1
0x1415c8381 0f84be000000 je 0x1415c8445
0x1415c8387 f3440f115d18 movss dword ptr [rbp + 0x18], xmm11
0x1415c838d f3440f11551c movss dword ptr [rbp + 0x1c], xmm10
0x1415c8393 48837df000 cmp qword ptr [rbp - 0x10], 0
0x1415c8398 0f8550020000 jne 0x1415c85ee
0x1415c839e f6457001 test byte ptr [rbp + 0x70], 1
0x1415c83a2 7508 jne 0x1415c83ac
0x1415c83a4 f30f1035bc4c4001 movss xmm6, dword ptr [rip + 0x1404cbc]
0x1415c83ac 488b4518 mov rax, qword ptr [rbp + 0x18]
0x1415c83b0 f6457401 test byte ptr [rbp + 0x74], 1
0x1415c83b4 7508 jne 0x1415c83be
0x1415c83b6 f30f103db2d53501 movss xmm7, dword ptr [rip + 0x135d5b2]
0x1415c83be 0fb69597000000 movzx edx, byte ptr [rbp + 0x97]
0x1415c83c5 80e201 and dl, 1
0x1415c83c8 0fb68d96000000 movzx ecx, byte ptr [rbp + 0x96]
0x1415c83cf 80e101 and cl, 1
0x1415c83d2 f3450f114d04 movss dword ptr [r13 + 4], xmm9
0x1415c83d8 49894508 mov qword ptr [r13 + 8], rax
0x1415c83dc f3450f114510 movss dword ptr [r13 + 0x10], xmm8
0x1415c83e2 f3410f117d14 movss dword ptr [r13 + 0x14], xmm7
0x1415c83e8 f3410f117518 movss dword ptr [r13 + 0x18], xmm6
0x1415c83ee 4188551c mov byte ptr [r13 + 0x1c], dl
0x1415c83f2 41884d1d mov byte ptr [r13 + 0x1d], cl
0x1415c83f6 31c0 xor eax, eax
0x1415c83f8 41894500 mov dword ptr [r13], eax
0x1415c83fc 488d4db0 lea rcx, [rbp - 0x50]
0x1415c8400 e8ebf92bff call 0x140887df0
0x1415c8405 e9ff000000 jmp 0x1415c8509
0x1415c840a 488b8580000000 mov rax, qword ptr [rbp + 0x80]
0x1415c8411 e9de000000 jmp 0x1415c84f4
0x1415c8416 488d0d53286b01 lea rcx, [rip + 0x16b2853]
0x1415c841d ba0a000000 mov edx, 0xa
0x1415c8422 e899173301 call 0x1428f9bc0
0x1415c8427 4d89e5 mov r13, r12
0x1415c842a e9c5000000 jmp 0x1415c84f4
0x1415c842f 488d0d44286b01 lea rcx, [rip + 0x16b2844]
0x1415c8436 ba09000000 mov edx, 9
0x1415c843b e880173301 call 0x1428f9bc0
0x1415c8440 e9af000000 jmp 0x1415c84f4
0x1415c8445 488d0d37286b01 lea rcx, [rip + 0x16b2837]
0x1415c844c ba0d000000 mov edx, 0xd
0x1415c8451 e86a173301 call 0x1428f9bc0
0x1415c8456 e999000000 jmp 0x1415c84f4
0x1415c845b 488d0d061e6b01 lea rcx, [rip + 0x16b1e06]
0x1415c8462 ba10000000 mov edx, 0x10
0x1415c8467 e8941b3301 call 0x1428fa000
0x1415c846c e983000000 jmp 0x1415c84f4
0x1415c8471 488d0df01d6b01 lea rcx, [rip + 0x16b1df0]
0x1415c8478 ba10000000 mov edx, 0x10
0x1415c847d e87e1b3301 call 0x1428fa000
0x1415c8482 eb70 jmp 0x1415c84f4
0x1415c8484 488d0ddd1d6b01 lea rcx, [rip + 0x16b1ddd]
0x1415c848b ba10000000 mov edx, 0x10
0x1415c8490 e86b1b3301 call 0x1428fa000
0x1415c8495 eb5d jmp 0x1415c84f4
0x1415c8497 488d0dca1d6b01 lea rcx, [rip + 0x16b1dca]
0x1415c849e ba10000000 mov edx, 0x10
0x1415c84a3 e8581b3301 call 0x1428fa000
0x1415c84a8 eb4a jmp 0x1415c84f4
0x1415c84aa 488d0db71d6b01 lea rcx, [rip + 0x16b1db7]
0x1415c84b1 ba10000000 mov edx, 0x10
0x1415c84b6 e8451b3301 call 0x1428fa000
0x1415c84bb eb37 jmp 0x1415c84f4
0x1415c84bd 488d0da41d6b01 lea rcx, [rip + 0x16b1da4]
0x1415c84c4 ba10000000 mov edx, 0x10
0x1415c84c9 e8321b3301 call 0x1428fa000
0x1415c84ce eb24 jmp 0x1415c84f4
0x1415c84d0 488d0d911d6b01 lea rcx, [rip + 0x16b1d91]
0x1415c84d7 ba10000000 mov edx, 0x10
0x1415c84dc e81f1b3301 call 0x1428fa000
0x1415c84e1 eb11 jmp 0x1415c84f4
0x1415c84e3 488d0d7e1d6b01 lea rcx, [rip + 0x16b1d7e]
0x1415c84ea ba10000000 mov edx, 0x10
0x1415c84ef e80c1b3301 call 0x1428fa000
0x1415c84f4 49894508 mov qword ptr [r13 + 8], rax
0x1415c84f8 41c7450001000000 mov dword ptr [r13], 1
0x1415c8500 488d4db0 lea rcx, [rbp - 0x50]
0x1415c8504 e8e7f82bff call 0x140887df0
0x1415c8509 488d4df8 lea rcx, [rbp - 8]
0x1415c850d e81e5f2cff call 0x14088e430
0x1415c8512 4c89e8 mov rax, r13
0x1415c8515 0f28b5a0000000 movaps xmm6, xmmword ptr [rbp + 0xa0]
0x1415c851c 0f28bdb0000000 movaps xmm7, xmmword ptr [rbp + 0xb0]
0x1415c8523 440f2885c0000000 movaps xmm8, xmmword ptr [rbp + 0xc0]
0x1415c852b 440f288dd0000000 movaps xmm9, xmmword ptr [rbp + 0xd0]
0x1415c8533 440f2895e0000000 movaps xmm10, xmmword ptr [rbp + 0xe0]
0x1415c853b 440f289df0000000 movaps xmm11, xmmword ptr [rbp + 0xf0]
0x1415c8543 4881c488010000 add rsp, 0x188
0x1415c854a 5b pop rbx
0x1415c854b 5f pop rdi
0x1415c854c 5e pop rsi
0x1415c854d 415c pop r12
0x1415c854f 415d pop r13
0x1415c8551 415e pop r14
0x1415c8553 415f pop r15
0x1415c8555 5d pop rbp
0x1415c8556 c3 ret 
0x1415c8557 488d0d32276b01 lea rcx, [rip + 0x16b2732]
0x1415c855e ba0f000000 mov edx, 0xf
0x1415c8563 e848183301 call 0x1428f9db0
0x1415c8568 eb8a jmp 0x1415c84f4
0x1415c856a 488d0d2e276b01 lea rcx, [rip + 0x16b272e]
0x1415c8571 ba09000000 mov edx, 9
0x1415c8576 e835183301 call 0x1428f9db0
0x1415c857b e974ffffff jmp 0x1415c84f4
0x1415c8580 488d0d21276b01 lea rcx, [rip + 0x16b2721]
0x1415c8587 ba0e000000 mov edx, 0xe
0x1415c858c e81f183301 call 0x1428f9db0
0x1415c8591 e95effffff jmp 0x1415c84f4
0x1415c8596 488d0dd3266b01 lea rcx, [rip + 0x16b26d3]
0x1415c859d ba0a000000 mov edx, 0xa
0x1415c85a2 e809183301 call 0x1428f9db0
0x1415c85a7 e948ffffff jmp 0x1415c84f4
0x1415c85ac 488d0dad1e6b01 lea rcx, [rip + 0x16b1ead]
0x1415c85b3 ba0e000000 mov edx, 0xe
0x1415c85b8 e8f3173301 call 0x1428f9db0
0x1415c85bd e932ffffff jmp 0x1415c84f4
0x1415c85c2 488d0db1266b01 lea rcx, [rip + 0x16b26b1]
0x1415c85c9 ba09000000 mov edx, 9
0x1415c85ce e8dd173301 call 0x1428f9db0
0x1415c85d3 e91cffffff jmp 0x1415c84f4
0x1415c85d8 488d0da4266b01 lea rcx, [rip + 0x16b26a4]
0x1415c85df ba0d000000 mov edx, 0xd
0x1415c85e4 e8c7173301 call 0x1428f9db0
0x1415c85e9 e906ffffff jmp 0x1415c84f4
0x1415c85ee 488d15a31a6b01 lea rdx, [rip + 0x16b1aa3]
0x1415c85f5 4c8d05641a6b01 lea r8, [rip + 0x16b1a64]
0x1415c85fc 488b4da8 mov rcx, qword ptr [rbp - 0x58]
0x1415c8600 e81b173301 call 0x1428f9d20
0x1415c8605 49894508 mov qword ptr [r13 + 8], rax
0x1415c8609 b801000000 mov eax, 1
0x1415c860e e9e5fdffff jmp 0x1415c83f8
0x1415c8613 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1415c8620 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415c8625 55 push rbp
0x1415c8626 4157 push r15
0x1415c8628 4156 push r14
0x1415c862a 4155 push r13
0x1415c862c 4154 push r12
0x1415c862e 56 push rsi
0x1415c862f 57 push rdi
0x1415c8630 53 push rbx
0x1415c8631 4881ec88000000 sub rsp, 0x88
0x1415c8638 488daa80000000 lea rbp, [rdx + 0x80]
0x1415c863f 440f295c2420 movaps xmmword ptr [rsp + 0x20], xmm11
0x1415c8645 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
0x1415c864b 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
0x1415c8651 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
0x1415c8657 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
0x1415c865c 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
0x1415c8661 488b5500 mov rdx, qword ptr [rbp]
0x1415c8665 4885d2 test rdx, rdx
0x1415c8668 7413 je 0x1415c867d
0x1415c866a 48c1e205 shl rdx, 5
0x1415c866e 41b808000000 mov r8d, 8
0x1415c8674 488b4d20 mov rcx, qword ptr [rbp + 0x20]
0x1415c8678 e8a3b504ff call 0x140613c20
0x1415c867d 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
0x1415c8682 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
0x1415c8687 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
0x1415c868d 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
0x1415c8693 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
0x1415c8699 440f285c2420 movaps xmm11, xmmword ptr [rsp + 0x20]
0x1415c869f 4881c488000000 add rsp, 0x88
0x1415c86a6 5b pop rbx
0x1415c86a7 5f pop rdi
0x1415c86a8 5e pop rsi
0x1415c86a9 415c pop r12
0x1415c86ab 415d pop r13
0x1415c86ad 415e pop r14
0x1415c86af 415f pop r15
0x1415c86b1 5d pop rbp
0x1415c86b2 c3 ret 
0x1415c86b3 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1415c86c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415c86c5 55 push rbp
0x1415c86c6 4157 push r15
0x1415c86c8 4156 push r14
0x1415c86ca 4155 push r13
0x1415c86cc 4154 push r12
0x1415c86ce 56 push rsi
0x1415c86cf 57 push rdi
0x1415c86d0 53 push rbx
0x1415c86d1 4881ec88000000 sub rsp, 0x88
0x1415c86d8 488daa80000000 lea rbp, [rdx + 0x80]
0x1415c86df 440f295c2420 movaps xmmword ptr [rsp + 0x20], xmm11
0x1415c86e5 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
0x1415c86eb 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
0x1415c86f1 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
0x1415c86f7 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
0x1415c86fc 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
0x1415c8701 0f284530 movaps xmm0, xmmword ptr [rbp + 0x30]
0x1415c8705 0f284d40 movaps xmm1, xmmword ptr [rbp + 0x40]
0x1415c8709 488d45f8 lea rax, [rbp - 8]
0x1415c870d 0f114810 movups xmmword ptr [rax + 0x10], xmm1
0x1415c8711 0f1100 movups xmmword ptr [rax], xmm0
0x1415c8714 c6859500000001 mov byte ptr [rbp + 0x95], 1
0x1415c871b 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
0x1415c8720 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
0x1415c8725 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
0x1415c872b 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
0x1415c8731 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
0x1415c8737 440f285c2420 movaps xmm11, xmmword ptr [rsp + 0x20]
0x1415c873d 4881c488000000 add rsp, 0x88
0x1415c8744 5b pop rbx
0x1415c8745 5f pop rdi
0x1415c8746 5e pop rsi
0x1415c8747 415c pop r12
0x1415c8749 415d pop r13
0x1415c874b 415e pop r14
0x1415c874d 415f pop r15
0x1415c874f 5d pop rbp
0x1415c8750 c3 ret 
0x1415c8751 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1415c8760 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415c8765 55 push rbp
0x1415c8766 4157 push r15
0x1415c8768 4156 push r14
0x1415c876a 4155 push r13
0x1415c876c 4154 push r12
0x1415c876e 56 push rsi
0x1415c876f 57 push rdi
0x1415c8770 53 push rbx
0x1415c8771 4881ec88000000 sub rsp, 0x88
0x1415c8778 488daa80000000 lea rbp, [rdx + 0x80]
0x1415c877f 440f295c2420 movaps xmmword ptr [rsp + 0x20], xmm11
0x1415c8785 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
0x1415c878b 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
0x1415c8791 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
0x1415c8797 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
0x1415c879c 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
0x1415c87a1 48837d5800 cmp qword ptr [rbp + 0x58], 0
0x1415c87a6 7413 je 0x1415c87bb
0x1415c87a8 41b801000000 mov r8d, 1
0x1415c87ae 488b4d28 mov rcx, qword ptr [rbp + 0x28]
0x1415c87b2 488b5558 mov rdx, qword ptr [rbp + 0x58]
0x1415c87b6 e865b404ff call 0x140613c20
0x1415c87bb c6859500000000 mov byte ptr [rbp + 0x95], 0
0x1415c87c2 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
0x1415c87c7 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
0x1415c87cc 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
0x1415c87d2 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
0x1415c87d8 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
0x1415c87de 440f285c2420 movaps xmm11, xmmword ptr [rsp + 0x20]
0x1415c87e4 4881c488000000 add rsp, 0x88
0x1415c87eb 5b pop rbx
0x1415c87ec 5f pop rdi
0x1415c87ed 5e pop rsi
0x1415c87ee 415c pop r12
0x1415c87f0 415d pop r13
0x1415c87f2 415e pop r14
0x1415c87f4 415f pop r15
0x1415c87f6 5d pop rbp
0x1415c87f7 c3 ret 
0x1415c87f8 0f1f840000000000 nop dword ptr [rax + rax]
0x1415c8800 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415c8805 55 push rbp
0x1415c8806 4157 push r15
0x1415c8808 4156 push r14
0x1415c880a 4155 push r13
0x1415c880c 4154 push r12
0x1415c880e 56 push rsi
0x1415c880f 57 push rdi
0x1415c8810 53 push rbx
0x1415c8811 4881ec88000000 sub rsp, 0x88
0x1415c8818 488daa80000000 lea rbp, [rdx + 0x80]
0x1415c881f 440f295c2420 movaps xmmword ptr [rsp + 0x20], xmm11
0x1415c8825 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
0x1415c882b 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
0x1415c8831 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
0x1415c8837 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
0x1415c883c 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
0x1415c8841 80bd9500000000 cmp byte ptr [rbp + 0x95], 0
0x1415c8848 7450 je 0x1415c889a
0x1415c884a 48837d5800 cmp qword ptr [rbp + 0x58], 0
0x1415c884f 7413 je 0x1415c8864
0x1415c8851 41b801000000 mov r8d, 1
0x1415c8857 488b4d28 mov rcx, qword ptr [rbp + 0x28]
0x1415c885b 488b5558 mov rdx, qword ptr [rbp + 0x58]
0x1415c885f e8bcb304ff call 0x140613c20
0x1415c8864 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
0x1415c8869 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
0x1415c886e 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
0x1415c8874 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
0x1415c887a 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
0x1415c8880 440f285c2420 movaps xmm11, xmmword ptr [rsp + 0x20]
0x1415c8886 4881c488000000 add rsp, 0x88
0x1415c888d 5b pop rbx
0x1415c888e 5f pop rdi
0x1415c888f 5e pop rsi
0x1415c8890 415c pop r12
0x1415c8892 415d pop r13
0x1415c8894 415e pop r14
0x1415c8896 415f pop r15
0x1415c8898 5d pop rbp
0x1415c8899 c3 ret 
0x1415c889a 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
0x1415c889f 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
0x1415c88a4 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
0x1415c88aa 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
0x1415c88b0 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
0x1415c88b6 440f285c2420 movaps xmm11, xmmword ptr [rsp + 0x20]
0x1415c88bc 4881c488000000 add rsp, 0x88
0x1415c88c3 5b pop rbx
0x1415c88c4 5f pop rdi
0x1415c88c5 5e pop rsi
0x1415c88c6 415c pop r12
0x1415c88c8 415d pop r13
0x1415c88ca 415e pop r14
0x1415c88cc 415f pop r15
0x1415c88ce 5d pop rbp
0x1415c88cf c3 ret 
0x1415c88d0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415c88d5 55 push rbp
0x1415c88d6 4157 push r15
0x1415c88d8 4156 push r14
0x1415c88da 4155 push r13
0x1415c88dc 4154 push r12
0x1415c88de 56 push rsi
0x1415c88df 57 push rdi
0x1415c88e0 53 push rbx
0x1415c88e1 4881ec88000000 sub rsp, 0x88
0x1415c88e8 488daa80000000 lea rbp, [rdx + 0x80]
0x1415c88ef 440f295c2420 movaps xmmword ptr [rsp + 0x20], xmm11
0x1415c88f5 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
0x1415c88fb 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
0x1415c8901 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
0x1415c8907 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
0x1415c890c 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
0x1415c8911 488d4db0 lea rcx, [rbp - 0x50]
0x1415c8915 e8e61b2cff call 0x14088a500
0x1415c891a 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
0x1415c891f 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
0x1415c8924 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
0x1415c892a 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
0x1415c8930 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
0x1415c8936 440f285c2420 movaps xmm11, xmmword ptr [rsp + 0x20]
0x1415c893c 4881c488000000 add rsp, 0x88
0x1415c8943 5b pop rbx
0x1415c8944 5f pop rdi
0x1415c8945 5e pop rsi
0x1415c8946 415c pop r12
0x1415c8948 415d pop r13
0x1415c894a 415e pop r14
0x1415c894c 415f pop r15
0x1415c894e 5d pop rbp
0x1415c894f c3 ret 
0x1415c8950 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415c8955 55 push rbp
0x1415c8956 4157 push r15
0x1415c8958 4156 push r14
0x1415c895a 4155 push r13
0x1415c895c 4154 push r12
0x1415c895e 56 push rsi
0x1415c895f 57 push rdi
0x1415c8960 53 push rbx
0x1415c8961 4881ec88000000 sub rsp, 0x88
0x1415c8968 488daa80000000 lea rbp, [rdx + 0x80]
0x1415c896f 440f295c2420 movaps xmmword ptr [rsp + 0x20], xmm11
0x1415c8975 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
0x1415c897b 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
0x1415c8981 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
0x1415c8987 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
0x1415c898c 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
0x1415c8991 488d4df8 lea rcx, [rbp - 8]
0x1415c8995 e8965a2cff call 0x14088e430
0x1415c899a 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
0x1415c899f 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
0x1415c89a4 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
0x1415c89aa 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
0x1415c89b0 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
0x1415c89b6 440f285c2420 movaps xmm11, xmmword ptr [rsp + 0x20]
0x1415c89bc 4881c488000000 add rsp, 0x88
0x1415c89c3 5b pop rbx
0x1415c89c4 5f pop rdi
0x1415c89c5 5e pop rsi
0x1415c89c6 415c pop r12
0x1415c89c8 415d pop r13
0x1415c89ca 415e pop r14
0x1415c89cc 415f pop r15
0x1415c89ce 5d pop rbp
0x1415c89cf c3 ret 
0x1415c89d0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415c89d5 55 push rbp
0x1415c89d6 4157 push r15
0x1415c89d8 4156 push r14
0x1415c89da 4155 push r13
0x1415c89dc 4154 push r12
0x1415c89de 56 push rsi
0x1415c89df 57 push rdi
0x1415c89e0 53 push rbx
0x1415c89e1 4881ec88000000 sub rsp, 0x88
0x1415c89e8 488daa80000000 lea rbp, [rdx + 0x80]
0x1415c89ef 440f295c2420 movaps xmmword ptr [rsp + 0x20], xmm11
0x1415c89f5 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
0x1415c89fb 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
0x1415c8a01 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
0x1415c8a07 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
0x1415c8a0c 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
0x1415c8a11 488d4df8 lea rcx, [rbp - 8]
0x1415c8a15 e8165a2cff call 0x14088e430
0x1415c8a1a 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
0x1415c8a1f 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
0x1415c8a24 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
0x1415c8a2a 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
0x1415c8a30 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
0x1415c8a36 440f285c2420 movaps xmm11, xmmword ptr [rsp + 0x20]
0x1415c8a3c 4881c488000000 add rsp, 0x88
0x1415c8a43 5b pop rbx
0x1415c8a44 5f pop rdi
0x1415c8a45 5e pop rsi
0x1415c8a46 415c pop r12
0x1415c8a48 415d pop r13
0x1415c8a4a 415e pop r14
0x1415c8a4c 415f pop r15
0x1415c8a4e 5d pop rbp
0x1415c8a4f c3 ret 
