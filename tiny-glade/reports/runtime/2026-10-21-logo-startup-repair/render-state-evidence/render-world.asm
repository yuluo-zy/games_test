0x1404ec110: push rbp
0x1404ec111: push r15
0x1404ec113: push r14
0x1404ec115: push r13
0x1404ec117: push r12
0x1404ec119: push rsi
0x1404ec11a: push rdi
0x1404ec11b: push rbx
0x1404ec11c: sub rsp, 0x4f8
0x1404ec123: lea rbp, [rsp + 0x80]
0x1404ec12b: mov qword ptr [rbp + 0x470], 0xfffffffffffffffe
0x1404ec136: mov qword ptr [rbp + 0x460], r9
0x1404ec13d: mov r15, qword ptr [rbp + 0x4e0]
0x1404ec144: mov rax, r15
0x1404ec147: shr rax, 0x20
0x1404ec14b: je 0x1404ed0a3
0x1404ec151: mov eax, r15d
0x1404ec154: test rax, rax
0x1404ec157: je 0x1404ed0a3
0x1404ec15d: mov rdi, r8
0x1404ec160: mov rbx, rdx
0x1404ec163: mov rsi, rcx
0x1404ec166: lea rcx, [r8 + 0x38]
0x1404ec16a: movabs rdx, 0x74430dfcc6d8719b
0x1404ec174: movabs r8, 0xca5591f7e464509c
0x1404ec17e: mov qword ptr [rbp + 0x3d8], rcx
0x1404ec185: call 0x1400dd6f0
0x1404ec18a: test rax, rax
0x1404ec18d: je 0x1404ed0bb
0x1404ec193: cmp qword ptr [rdi + 0x218], rdx
0x1404ec19a: jbe 0x1404ed0bb
0x1404ec1a0: mov rax, qword ptr [rdi + 0x210]
0x1404ec1a7: mov rcx, qword ptr [rax + rdx*8]
0x1404ec1ab: test rcx, rcx
0x1404ec1ae: je 0x1404ed0bb
0x1404ec1b4: mov rax, qword ptr [rdi + 0x1e0]
0x1404ec1bb: not rcx
0x1404ec1be: lea rcx, [rcx + rcx*2]
0x1404ec1c2: shl rcx, 5
0x1404ec1c6: cmp qword ptr [rax + rcx + 0x20], 0
0x1404ec1cc: je 0x1404ed0bb
0x1404ec1d2: mov qword ptr [rbp + 0x2b8], rsi
0x1404ec1d9: mov r12, qword ptr [rbp + 0x4f0]
0x1404ec1e0: add rax, rcx
0x1404ec1e3: mov rcx, qword ptr [rax + 0x10]
0x1404ec1e7: add rcx, 0x58
0x1404ec1eb: call 0x140ae8320
0x1404ec1f0: mov byte ptr [rbp + 0x46f], al
0x1404ec1f6: movups xmm0, xmmword ptr [r12 + 4]
0x1404ec1fc: movups xmm1, xmmword ptr [r12 + 0x14]
0x1404ec202: movaps xmmword ptr [rbp + 0x350], xmm1
0x1404ec209: movaps xmmword ptr [rbp + 0x340], xmm0
0x1404ec210: lea rsi, [rbp + 0x240]
0x1404ec217: lea rdx, [rbp + 0x340]
0x1404ec21e: mov rcx, rsi
0x1404ec221: mov r8d, 0x64
0x1404ec227: call 0x140bd5430
0x1404ec22c: lea r14, [rbp + 0x410]
0x1404ec233: mov rcx, r14
0x1404ec236: mov rdx, rsi
0x1404ec239: mov r8d, 0xc
0x1404ec23f: call 0x140bd5340
0x1404ec244: mov qword ptr [rsp + 0x20], r14
0x1404ec249: lea r8, [rip + 0x24ffd48]
0x1404ec250: lea rcx, [rbp + 0x340]
0x1404ec257: mov r9d, 0x17
0x1404ec25d: mov rdx, rbx
0x1404ec260: call 0x1404f6e50
0x1404ec265: cmp byte ptr [rbp + 0x364], 2
0x1404ec26c: je 0x1404ed0eb
0x1404ec272: mov rax, qword ptr [rbp + 0x360]
0x1404ec279: mov qword ptr [rbp + 0xd0], rax
0x1404ec280: movups xmm0, xmmword ptr [rbp + 0x340]
0x1404ec287: movups xmm1, xmmword ptr [rbp + 0x350]
0x1404ec28e: movaps xmmword ptr [rbp + 0xc0], xmm1
0x1404ec295: movaps xmmword ptr [rbp + 0xb0], xmm0
0x1404ec29c: mov rcx, rbx
0x1404ec29f: mov rdx, rdi
0x1404ec2a2: mov r8b, 1
0x1404ec2a5: call 0x1403d8bd0
0x1404ec2aa: mov rcx, rbx
0x1404ec2ad: mov rdx, rdi
0x1404ec2b0: mov r8b, 2
0x1404ec2b3: call 0x1403da050
0x1404ec2b8: mov rcx, rbx
0x1404ec2bb: mov rdx, rdi
0x1404ec2be: mov r8b, 3
0x1404ec2c1: call 0x1403da570
0x1404ec2c6: mov rcx, rbx
0x1404ec2c9: mov rdx, rdi
0x1404ec2cc: mov r8b, 4
0x1404ec2cf: call 0x1403d90f0
0x1404ec2d4: mov rcx, rbx
0x1404ec2d7: mov rdx, rdi
0x1404ec2da: mov r8b, 0x13
0x1404ec2dd: call 0x1403d9610
0x1404ec2e2: mov rcx, rbx
0x1404ec2e5: mov rdx, rdi
0x1404ec2e8: mov r8b, 0x14
0x1404ec2eb: call 0x1403d9b30
0x1404ec2f0: mov rcx, rbx
0x1404ec2f3: mov rdx, rdi
0x1404ec2f6: call 0x140a12680
0x1404ec2fb: mov rcx, rbx
0x1404ec2fe: mov rdx, rdi
0x1404ec301: call 0x1408c7120
0x1404ec306: mov rcx, rbx
0x1404ec309: mov rdx, rdi
0x1404ec30c: call 0x140a3ee80
0x1404ec311: mov rcx, rbx
0x1404ec314: mov rdx, rdi
0x1404ec317: call 0x14098c680
0x1404ec31c: mov rcx, rbx
0x1404ec31f: mov rdx, rdi
0x1404ec322: call 0x1409768b0
0x1404ec327: mov rcx, rbx
0x1404ec32a: mov rdx, rdi
0x1404ec32d: call 0x1409cb7b0
0x1404ec332: mov rcx, rbx
0x1404ec335: mov rdx, rdi
0x1404ec338: call 0x140a58450
0x1404ec33d: mov rcx, rbx
0x1404ec340: mov rdx, rdi
0x1404ec343: call 0x1409daaa0
0x1404ec348: mov rcx, rbx
0x1404ec34b: mov rdx, rdi
0x1404ec34e: call 0x1409014e0
0x1404ec353: mov rcx, rbx
0x1404ec356: mov rdx, rdi
0x1404ec359: call 0x140a32770
0x1404ec35e: mov rcx, rbx
0x1404ec361: mov rdx, rdi
0x1404ec364: call 0x140a22140
0x1404ec369: mov rcx, rbx
0x1404ec36c: mov rdx, rdi
0x1404ec36f: call 0x140a13970
0x1404ec374: mov rcx, rbx
0x1404ec377: mov rdx, rdi
0x1404ec37a: call 0x1408e4cc0
0x1404ec37f: mov rcx, rbx
0x1404ec382: mov rdx, rdi
0x1404ec385: call 0x140866540
0x1404ec38a: mov rcx, rbx
0x1404ec38d: mov rdx, rdi
0x1404ec390: call 0x140a22b80
0x1404ec395: mov rcx, rbx
0x1404ec398: mov rdx, rdi
0x1404ec39b: call 0x140958f70
0x1404ec3a0: mov rcx, rbx
0x1404ec3a3: mov rdx, rdi
0x1404ec3a6: call 0x1408f4d00
0x1404ec3ab: mov rcx, rbx
0x1404ec3ae: mov rdx, rdi
0x1404ec3b1: call 0x140a29b10
0x1404ec3b6: mov rcx, rbx
0x1404ec3b9: mov rdx, rdi
0x1404ec3bc: call 0x140a68a50
0x1404ec3c1: mov rcx, rbx
0x1404ec3c4: mov rdx, rdi
0x1404ec3c7: call 0x14098aaf0
0x1404ec3cc: mov rcx, rbx
0x1404ec3cf: mov rdx, rdi
0x1404ec3d2: xor r8d, r8d
0x1404ec3d5: call 0x140987dd0
0x1404ec3da: mov rcx, rbx
0x1404ec3dd: mov rdx, rdi
0x1404ec3e0: mov r13, qword ptr [rbp + 0x460]
0x1404ec3e7: mov r8, r13
0x1404ec3ea: call 0x1409cc270
0x1404ec3ef: mov rcx, rbx
0x1404ec3f2: mov rdx, rdi
0x1404ec3f5: call 0x140a51440
0x1404ec3fa: lea rcx, [rbp + 0x410]
0x1404ec401: mov rdx, r15
0x1404ec404: mov r8d, 0x65
0x1404ec40a: call 0x140bd52a0
0x1404ec40f: movups xmm0, xmmword ptr [rbp + 0x410]
0x1404ec416: movups xmm1, xmmword ptr [rbp + 0x420]
0x1404ec41d: movaps xmmword ptr [rbp + 0x250], xmm1
0x1404ec424: movaps xmmword ptr [rbp + 0x240], xmm0
0x1404ec42b: lea rcx, [rbp + 0x350]
0x1404ec432: lea rdx, [rbp + 0x240]
0x1404ec439: call 0x140bd6ce0
0x1404ec43e: lea rax, [rip + 0x24ffb83]
0x1404ec445: mov qword ptr [rbp + 0x340], rax
0x1404ec44c: mov qword ptr [rbp + 0x348], 7
0x1404ec457: lea rdx, [rbp + 0x340]
0x1404ec45e: mov rcx, rbx
0x1404ec461: call 0x140be94d0
0x1404ec466: mov esi, eax
0x1404ec468: movups xmm0, xmmword ptr [rbp + 0x410]
0x1404ec46f: movups xmm1, xmmword ptr [rbp + 0x420]
0x1404ec476: movaps xmmword ptr [rbp + 0xe0], xmm0
0x1404ec47d: movaps xmmword ptr [rbp + 0xf0], xmm1
0x1404ec484: lea rcx, [rbp + 0x3e0]
0x1404ec48b: mov rdx, r15
0x1404ec48e: mov r8d, 0x61
0x1404ec494: call 0x140bd52a0
0x1404ec499: lea r14, [rbp + 0x288]
0x1404ec4a0: movups xmm0, xmmword ptr [rbp + 0x3e0]
0x1404ec4a7: movups xmm1, xmmword ptr [rbp + 0x3f0]
0x1404ec4ae: movaps xmmword ptr [rbp + 0x420], xmm1
0x1404ec4b5: movaps xmmword ptr [rbp + 0x410], xmm0
0x1404ec4bc: lea rcx, [rbp + 0x350]
0x1404ec4c3: lea rdx, [rbp + 0x410]
0x1404ec4ca: call 0x140bd6ce0
0x1404ec4cf: lea rax, [rip + 0x24ffaf9]
0x1404ec4d6: mov qword ptr [rbp + 0x340], rax
0x1404ec4dd: mov qword ptr [rbp + 0x348], 8
0x1404ec4e8: lea rdx, [rbp + 0x340]
0x1404ec4ef: mov rcx, rbx
0x1404ec4f2: call 0x140be94d0
0x1404ec4f7: movups xmm0, xmmword ptr [rbp + 0x3e0]
0x1404ec4fe: movups xmm1, xmmword ptr [rbp + 0x3f0]
0x1404ec505: movups xmmword ptr [rbp + 0x28c], xmm0
0x1404ec50c: movups xmmword ptr [rbp + 0x29c], xmm1
0x1404ec513: mov dword ptr [rbp + 0x288], eax
0x1404ec519: mov dword ptr [rbp + 0x240], esi
0x1404ec51f: movaps xmm0, xmmword ptr [rbp + 0xe0]
0x1404ec526: movaps xmm1, xmmword ptr [rbp + 0xf0]
0x1404ec52d: movups xmmword ptr [rbp + 0x244], xmm0
0x1404ec534: movups xmmword ptr [rbp + 0x254], xmm1
0x1404ec53b: lea rsi, [rbp + 0x264]
0x1404ec542: movups xmm0, xmmword ptr [r12]
0x1404ec547: movups xmm1, xmmword ptr [r12 + 0x10]
0x1404ec54d: movups xmmword ptr [rbp + 0x264], xmm0
0x1404ec554: movups xmmword ptr [rbp + 0x274], xmm1
0x1404ec55b: mov eax, dword ptr [r12 + 0x20]
0x1404ec560: mov dword ptr [rbp + 0x284], eax
0x1404ec566: mov qword ptr [rsp + 0x20], r13
0x1404ec56b: mov r15, qword ptr [rbp + 0x4f8]
0x1404ec572: lea r12, [rbp + 0x240]
0x1404ec579: mov rcx, rbx
0x1404ec57c: mov rdx, rdi
0x1404ec57f: mov r8, r12
0x1404ec582: mov r9, r15
0x1404ec585: call 0x140833bc0
0x1404ec58a: lea rax, [rip + 0x24ffa46]
0x1404ec591: mov qword ptr [rsp + 0x20], rax
0x1404ec596: mov qword ptr [rsp + 0x28], 4
0x1404ec59f: lea rcx, [rbp + 0x60]
0x1404ec5a3: mov rdx, rbx
0x1404ec5a6: mov r8, rdi
0x1404ec5a9: mov r9, rsi
0x1404ec5ac: call 0x1409286f0
0x1404ec5b1: mov rax, qword ptr [rip + 0x313fb20]
0x1404ec5b8: cmp byte ptr [rax], 0
0x1404ec5bb: jne 0x1404ec5dd
0x1404ec5bd: lea rsi, [rbp + 0x60]
0x1404ec5c1: mov rcx, rbx
0x1404ec5c4: mov rdx, rdi
0x1404ec5c7: mov r8, rsi
0x1404ec5ca: call 0x14098b2a0
0x1404ec5cf: mov rcx, rbx
0x1404ec5d2: mov rdx, rdi
0x1404ec5d5: mov r8, rsi
0x1404ec5d8: call 0x140987dd0
0x1404ec5dd: lea r8, [rbp + 0x60]
0x1404ec5e1: mov rcx, rbx
0x1404ec5e4: mov rdx, rdi
0x1404ec5e7: call 0x140a60280
0x1404ec5ec: mov rsi, qword ptr [rbp + 0x460]
0x1404ec5f3: mov qword ptr [rsp + 0x20], rsi
0x1404ec5f8: mov rcx, rbx
0x1404ec5fb: mov rdx, rdi
0x1404ec5fe: mov r8, r12
0x1404ec601: mov r9, r15
0x1404ec604: call 0x140835a40
0x1404ec609: mov rcx, rbx
0x1404ec60c: mov rdx, rdi
0x1404ec60f: lea r13, [rbp + 0x264]
0x1404ec616: mov r8, r13
0x1404ec619: call 0x140a2f2e0
0x1404ec61e: mov rcx, rbx
0x1404ec621: mov rdx, rdi
0x1404ec624: mov r8, r13
0x1404ec627: call 0x140a17880
0x1404ec62c: mov rcx, rbx
0x1404ec62f: mov rdx, rdi
0x1404ec632: mov r8, r13
0x1404ec635: call 0x140a01620
0x1404ec63a: mov qword ptr [rsp + 0x20], rsi
0x1404ec63f: mov rcx, rbx
0x1404ec642: mov rdx, rdi
0x1404ec645: mov r8, r12
0x1404ec648: mov r9, r15
0x1404ec64b: call 0x1408373b0
0x1404ec650: lea rcx, [rbp + 0x340]
0x1404ec657: mov rdx, rbx
0x1404ec65a: mov r8, rdi
0x1404ec65d: mov r9, r12
0x1404ec660: call 0x1408f6180
0x1404ec665: mov eax, dword ptr [rbp + 0x360]
0x1404ec66b: mov dword ptr [rbp + 0x200], eax
0x1404ec671: movups xmm0, xmmword ptr [rbp + 0x340]
0x1404ec678: movups xmm1, xmmword ptr [rbp + 0x350]
0x1404ec67f: movaps xmmword ptr [rbp + 0x1f0], xmm1
0x1404ec686: movaps xmmword ptr [rbp + 0x1e0], xmm0
0x1404ec68d: mov eax, dword ptr [rbp + 0x384]
0x1404ec693: mov dword ptr [rbp + 0x180], eax
0x1404ec699: movups xmm0, xmmword ptr [rbp + 0x374]
0x1404ec6a0: movaps xmmword ptr [rbp + 0x170], xmm0
0x1404ec6a7: movups xmm0, xmmword ptr [rbp + 0x364]
0x1404ec6ae: movaps xmmword ptr [rbp + 0x160], xmm0
0x1404ec6b5: lea rax, [rbp + 0xb0]
0x1404ec6bc: mov qword ptr [rsp + 0x28], rax
0x1404ec6c1: mov qword ptr [rsp + 0x20], r14
0x1404ec6c6: lea rsi, [rbp + 0x18c]
0x1404ec6cd: mov rcx, rsi
0x1404ec6d0: mov rdx, rbx
0x1404ec6d3: mov r8, rdi
0x1404ec6d6: mov r9, r13
0x1404ec6d9: call 0x140a0d190
0x1404ec6de: mov qword ptr [rsp + 0x30], rsi
0x1404ec6e3: mov qword ptr [rsp + 0x28], r12
0x1404ec6e8: lea r13, [rbp + 0x160]
0x1404ec6ef: mov qword ptr [rsp + 0x20], r13
0x1404ec6f4: lea rcx, [rbp + 0x8c]
0x1404ec6fb: lea r9, [rbp + 0x1e0]
0x1404ec702: mov rdx, rbx
0x1404ec705: mov r8, rdi
0x1404ec708: call 0x1408743b0
0x1404ec70d: mov rax, qword ptr [rbp + 0x4e8]
0x1404ec714: movzx r12d, byte ptr [rax + 0xab]
0x1404ec71c: test r12b, r12b
0x1404ec71f: je 0x1404ec7ec
0x1404ec725: movabs rdx, 0xa4c02661e67b048b
0x1404ec72f: movabs r8, 0xeae761e6aa2a67ec
0x1404ec739: mov rcx, qword ptr [rbp + 0x3d8]
0x1404ec740: call 0x1400dd6f0
0x1404ec745: test rax, rax
0x1404ec748: je 0x1404ed0d3
0x1404ec74e: cmp qword ptr [rdi + 0x218], rdx
0x1404ec755: jbe 0x1404ed0d3
0x1404ec75b: mov rax, qword ptr [rdi + 0x210]
0x1404ec762: mov rcx, qword ptr [rax + rdx*8]
0x1404ec766: test rcx, rcx
0x1404ec769: je 0x1404ed0d3
0x1404ec76f: mov rax, qword ptr [rdi + 0x1e0]
0x1404ec776: not rcx
0x1404ec779: lea rcx, [rcx + rcx*2]
0x1404ec77d: shl rcx, 5
0x1404ec781: cmp qword ptr [rax + rcx + 0x20], 0
0x1404ec787: je 0x1404ed0d3
0x1404ec78d: add rax, rcx
0x1404ec790: mov rax, qword ptr [rax + 0x10]
0x1404ec794: mov rdx, qword ptr [rax + 8]
0x1404ec798: lea rsi, [rbp + 0x340]
0x1404ec79f: mov rcx, rsi
0x1404ec7a2: mov r8d, 0x7a
0x1404ec7a8: call 0x140bd52a0
0x1404ec7ad: lea r14, [rbp + 0x410]
0x1404ec7b4: mov rcx, r14
0x1404ec7b7: mov rdx, rsi
0x1404ec7ba: call 0x140bd5370
0x1404ec7bf: lea rsi, [rbp + 0x3e0]
0x1404ec7c6: mov rcx, rsi
0x1404ec7c9: mov rdx, r14
0x1404ec7cc: call 0x140bd5390
0x1404ec7d1: mov qword ptr [rsp + 0x20], rsi
0x1404ec7d6: lea r8, [rip + 0x24ff823]
0x1404ec7dd: lea rcx, [rbp + 0x340]
0x1404ec7e4: mov r9d, 0xc
0x1404ec7ea: jmp 0x1404ec84a
0x1404ec7ec: lea rcx, [r15 + 4]
0x1404ec7f0: call 0x140bd5790
0x1404ec7f5: lea rsi, [rbp + 0x340]
0x1404ec7fc: mov rcx, rsi
0x1404ec7ff: mov rdx, rax
0x1404ec802: mov r8d, 0x7a
0x1404ec808: call 0x140bd52a0
0x1404ec80d: lea r14, [rbp + 0x410]
0x1404ec814: mov rcx, r14
0x1404ec817: mov rdx, rsi
0x1404ec81a: call 0x140bd5370
0x1404ec81f: lea rsi, [rbp + 0x3e0]
0x1404ec826: mov rcx, rsi
0x1404ec829: mov rdx, r14
0x1404ec82c: call 0x140bd5390
0x1404ec831: mov qword ptr [rsp + 0x20], rsi
0x1404ec836: lea r8, [rip + 0x24ff79e]
0x1404ec83d: lea rcx, [rbp + 0x340]
0x1404ec844: mov r9d, 0xb
0x1404ec84a: mov rdx, rbx
0x1404ec84d: call 0x1404f7450
0x1404ec852: mov byte ptr [rbp + 0x46e], r12b
0x1404ec859: cmp dword ptr [rbp + 0x340], 1
0x1404ec860: je 0x1404ed126
0x1404ec866: mov rax, qword ptr [rbp + 0x384]
0x1404ec86d: mov qword ptr [rbp + 0x450], rax
0x1404ec874: movups xmm0, xmmword ptr [rbp + 0x344]
0x1404ec87b: movups xmm1, xmmword ptr [rbp + 0x354]
0x1404ec882: movups xmm2, xmmword ptr [rbp + 0x364]
0x1404ec889: movups xmm3, xmmword ptr [rbp + 0x374]
0x1404ec890: movaps xmmword ptr [rbp + 0x440], xmm3
0x1404ec897: movaps xmmword ptr [rbp + 0x430], xmm2
0x1404ec89e: movaps xmmword ptr [rbp + 0x420], xmm1
0x1404ec8a5: movaps xmmword ptr [rbp + 0x410], xmm0
0x1404ec8ac: movzx eax, byte ptr [rbp + 0x38c]
0x1404ec8b3: movups xmm0, xmmword ptr [rbp + 0x434]
0x1404ec8ba: movups xmmword ptr [rbp + 0x343], xmm0
0x1404ec8c1: movups xmm0, xmmword ptr [rbp + 0x444]
0x1404ec8c8: movups xmmword ptr [rbp + 0x353], xmm0
0x1404ec8cf: mov ecx, dword ptr [rbp + 0x454]
0x1404ec8d5: mov dword ptr [rbp + 0x363], ecx
0x1404ec8db: mov ecx, dword ptr [rbp + 0x430]
0x1404ec8e1: mov dword ptr [rbp + 0x1d0], ecx
0x1404ec8e7: movaps xmm0, xmmword ptr [rbp + 0x410]
0x1404ec8ee: movaps xmm1, xmmword ptr [rbp + 0x420]
0x1404ec8f5: movaps xmmword ptr [rbp + 0x1c0], xmm1
0x1404ec8fc: movaps xmmword ptr [rbp + 0x1b0], xmm0
0x1404ec903: mov ecx, dword ptr [rbp + 0x363]
0x1404ec909: mov dword ptr [rbp + 0x3d0], ecx
0x1404ec90f: movups xmm0, xmmword ptr [rbp + 0x343]
0x1404ec916: movups xmm1, xmmword ptr [rbp + 0x353]
0x1404ec91d: movaps xmmword ptr [rbp + 0x3c0], xmm1
0x1404ec924: movaps xmmword ptr [rbp + 0x3b0], xmm0
0x1404ec92b: mov byte ptr [rbp + 0x3d4], al
0x1404ec931: lea rcx, [rbp + 0x8c]
0x1404ec938: mov qword ptr [rsp + 0x48], rcx
0x1404ec93d: mov r12, rcx
0x1404ec940: lea r15, [rbp + 0x18c]
0x1404ec947: mov qword ptr [rsp + 0x40], r15
0x1404ec94c: and al, 1
0x1404ec94e: mov byte ptr [rsp + 0x38], al
0x1404ec952: lea rax, [rbp + 0x3b0]
0x1404ec959: mov qword ptr [rsp + 0x30], rax
0x1404ec95e: mov qword ptr [rsp + 0x28], r13
0x1404ec963: lea rsi, [rbp + 0x1e0]
0x1404ec96a: mov qword ptr [rsp + 0x20], rsi
0x1404ec96f: lea r9, [rbp + 0x240]
0x1404ec976: lea r14, [rbp + 0x340]
0x1404ec97d: mov rcx, r14
0x1404ec980: mov rdx, rbx
0x1404ec983: mov r8, rdi
0x1404ec986: call 0x140a01cf0
0x1404ec98b: mov qword ptr [rsp + 0x38], r15
0x1404ec990: mov qword ptr [rsp + 0x30], r14
0x1404ec995: mov qword ptr [rsp + 0x28], r12
0x1404ec99a: mov qword ptr [rsp + 0x20], r13
0x1404ec99f: lea rcx, [rbp + 0x3c]
0x1404ec9a3: mov rdx, rbx
0x1404ec9a6: mov r8, rdi
0x1404ec9a9: mov r9, rsi
0x1404ec9ac: call 0x140a44cd0
0x1404ec9b1: lea r12, [rbp + 0xe0]
0x1404ec9b8: mov rcx, r12
0x1404ec9bb: mov rdx, rbx
0x1404ec9be: mov r8, rdi
0x1404ec9c1: call 0x1409e7770
0x1404ec9c6: mov qword ptr [rsp + 0x30], r12
0x1404ec9cb: mov qword ptr [rsp + 0x28], r15
0x1404ec9d0: mov qword ptr [rsp + 0x20], r13
0x1404ec9d5: lea r13, [rbp + 0x18]
0x1404ec9d9: mov rcx, r13
0x1404ec9dc: mov rdx, rbx
0x1404ec9df: mov r8, rdi
0x1404ec9e2: mov r9, rsi
0x1404ec9e5: call 0x140a6fec0
0x1404ec9ea: mov qword ptr [rsp + 0x20], r15
0x1404ec9ef: lea r15, [rbp - 0xc]
0x1404ec9f3: mov rcx, r15
0x1404ec9f6: mov rdx, rbx
0x1404ec9f9: mov r8, rdi
0x1404ec9fc: lea r9, [rbp + 0x240]
0x1404eca03: call 0x140a39e70
0x1404eca08: movzx eax, byte ptr [rbp + 0x3d4]
0x1404eca0f: mov qword ptr [rsp + 0x60], r12
0x1404eca14: mov r12, rsi
0x1404eca17: lea rsi, [rbp + 0x3b0]
0x1404eca1e: mov qword ptr [rsp + 0x58], r13
0x1404eca23: mov qword ptr [rsp + 0x50], r15
0x1404eca28: lea rcx, [rbp + 0x3c]
0x1404eca2c: mov qword ptr [rsp + 0x48], rcx
0x1404eca31: mov qword ptr [rsp + 0x40], r14
0x1404eca36: mov qword ptr [rsp + 0x38], r12
0x1404eca3b: lea rcx, [rbp + 0x8c]
0x1404eca42: mov qword ptr [rsp + 0x30], rcx
0x1404eca47: mov byte ptr [rsp + 0x28], al
0x1404eca4b: mov qword ptr [rsp + 0x20], rsi
0x1404eca50: mov rcx, rbx
0x1404eca53: mov rdx, rdi
0x1404eca56: lea r8, [rbp + 0x240]
0x1404eca5d: mov r13, qword ptr [rbp + 0x4f8]
0x1404eca64: mov r9, r13
0x1404eca67: call 0x1409d20b0
0x1404eca6c: mov rax, qword ptr [rip + 0x313f625]
0x1404eca73: cmp byte ptr [rax], 0
0x1404eca76: jne 0x1404ecb99
0x1404eca7c: lea rcx, [r13 + 4]
0x1404eca80: call 0x140bd5790
0x1404eca85: lea rcx, [rbp + 0x2e0]
0x1404eca8c: mov rdx, rax
0x1404eca8f: mov r8d, 0x7a
0x1404eca95: call 0x140bd52a0
0x1404eca9a: movups xmm0, xmmword ptr [rbp + 0x2e0]
0x1404ecaa1: movups xmm1, xmmword ptr [rbp + 0x2f0]
0x1404ecaa8: movaps xmmword ptr [rbp + 0x3f0], xmm1
0x1404ecaaf: movaps xmmword ptr [rbp + 0x3e0], xmm0
0x1404ecab6: lea rcx, [rbp + 0x420]
0x1404ecabd: lea rdx, [rbp + 0x3e0]
0x1404ecac4: call 0x140bd6ce0
0x1404ecac9: lea rax, [rip + 0x24ff558]
0x1404ecad0: mov qword ptr [rbp + 0x410], rax
0x1404ecad7: mov qword ptr [rbp + 0x418], 0x11
0x1404ecae2: lea rsi, [rbp + 0x410]
0x1404ecae9: mov rcx, rbx
0x1404ecaec: mov rdx, rsi
0x1404ecaef: call 0x140be94d0
0x1404ecaf4: movups xmm0, xmmword ptr [rbp + 0x2e0]
0x1404ecafb: movups xmm1, xmmword ptr [rbp + 0x2f0]
0x1404ecb02: movups xmmword ptr [rbp + 0x414], xmm0
0x1404ecb09: movups xmmword ptr [rbp + 0x424], xmm1
0x1404ecb10: mov dword ptr [rbp + 0x410], eax
0x1404ecb16: mov r14, qword ptr [rbp + 0x460]
0x1404ecb1d: mov qword ptr [rsp + 0x28], r14
0x1404ecb22: lea r15, [rbp + 0x264]
0x1404ecb29: mov qword ptr [rsp + 0x20], r15
0x1404ecb2e: mov rcx, rbx
0x1404ecb31: mov rdx, rdi
0x1404ecb34: mov r8, rsi
0x1404ecb37: mov r9, r13
0x1404ecb3a: call 0x1409bf320
0x1404ecb3f: lea rax, [rip + 0x4d3d8a]
0x1404ecb46: lea r10, [rip + 0x4d2cb3]
0x1404ecb4d: cmp byte ptr [rbp + 0x46f], 0
0x1404ecb54: cmovne r10, rax
0x1404ecb58: movzx eax, byte ptr [rbp + 0x3d4]
0x1404ecb5f: mov qword ptr [rsp + 0x48], r14
0x1404ecb64: lea rcx, [rbp + 0x18c]
0x1404ecb6b: mov qword ptr [rsp + 0x40], rcx
0x1404ecb70: mov qword ptr [rsp + 0x38], rsi
0x1404ecb75: lea rsi, [rbp + 0x3b0]
0x1404ecb7c: mov byte ptr [rsp + 0x30], al
0x1404ecb80: mov qword ptr [rsp + 0x28], rsi
0x1404ecb85: mov qword ptr [rsp + 0x20], r12
0x1404ecb8a: mov rcx, rbx
0x1404ecb8d: mov rdx, rdi
0x1404ecb90: mov r8, r13
0x1404ecb93: mov r9, r15
0x1404ecb96: call r10
0x1404ecb99: movzx eax, byte ptr [rbp + 0x3d4]
0x1404ecba0: mov rcx, qword ptr [rbp + 0x460]
0x1404ecba7: mov qword ptr [rsp + 0x38], rcx
0x1404ecbac: mov byte ptr [rsp + 0x30], al
0x1404ecbb0: mov qword ptr [rsp + 0x28], rsi
0x1404ecbb5: mov qword ptr [rsp + 0x20], r12
0x1404ecbba: mov rcx, rbx
0x1404ecbbd: mov rdx, rdi
0x1404ecbc0: mov r8, r13
0x1404ecbc3: lea rsi, [rbp + 0x264]
0x1404ecbca: mov r9, rsi
0x1404ecbcd: call 0x1409c3a80
0x1404ecbd2: mov rcx, rbx
0x1404ecbd5: mov rdx, rdi
0x1404ecbd8: call 0x140a3a6f0
0x1404ecbdd: mov eax, dword ptr [rsi + 0x20]
0x1404ecbe0: mov dword ptr [rbp + 0x230], eax
0x1404ecbe6: movups xmm0, xmmword ptr [rsi]
0x1404ecbe9: movups xmm1, xmmword ptr [rsi + 0x10]
0x1404ecbed: movaps xmmword ptr [rbp + 0x220], xmm1
0x1404ecbf4: movaps xmmword ptr [rbp + 0x210], xmm0
0x1404ecbfb: lea r14, [rbp + 0x210]
0x1404ecc02: mov rcx, rbx
0x1404ecc05: mov rdx, rdi
0x1404ecc08: mov r8, r13
0x1404ecc0b: mov r9, r14
0x1404ecc0e: call 0x1408a44c0
0x1404ecc13: lea r15, [rbp + 0x410]
0x1404ecc1a: mov rcx, r15
0x1404ecc1d: mov rdx, rbx
0x1404ecc20: mov r8, rdi
0x1404ecc23: mov r9, r14
0x1404ecc26: call 0x140a5a020
0x1404ecc2b: movabs rdx, 0xc2b8e941c91a6428
0x1404ecc35: movabs r8, 0x59f3c6d1d3da43a0
0x1404ecc3f: mov rcx, qword ptr [rbp + 0x3d8]
0x1404ecc46: call 0x1400dd6f0
0x1404ecc4b: test rax, rax
0x1404ecc4e: mov r12, qword ptr [rbp + 0x2b8]
0x1404ecc55: je 0x1404ed0c7
0x1404ecc5b: cmp qword ptr [rdi + 0x218], rdx
0x1404ecc62: jbe 0x1404ed0c7
0x1404ecc68: mov rax, qword ptr [rdi + 0x210]
0x1404ecc6f: mov rcx, qword ptr [rax + rdx*8]
0x1404ecc73: test rcx, rcx
0x1404ecc76: je 0x1404ed0c7
0x1404ecc7c: mov rax, qword ptr [rdi + 0x1e0]
0x1404ecc83: not rcx
0x1404ecc86: lea rcx, [rcx + rcx*2]
0x1404ecc8a: shl rcx, 5
0x1404ecc8e: cmp qword ptr [rax + rcx + 0x20], 0
0x1404ecc94: je 0x1404ed0c7
0x1404ecc9a: add rax, rcx
0x1404ecc9d: mov rax, qword ptr [rax + 0x10]
0x1404ecca1: mov esi, dword ptr [rax]
0x1404ecca3: xor eax, eax
0x1404ecca5: cmp esi, 0x13
0x1404ecca8: sete al
0x1404eccab: mov rcx, qword ptr [rbp + 0x4e8]
0x1404eccb2: cmp byte ptr [rax + rcx + 0xa9], 0
0x1404eccba: je 0x1404ece3e
0x1404eccc0: lea r8, [r13 + 4]
0x1404eccc4: lea rax, [rip + 0x24ff38d]
0x1404ecccb: mov qword ptr [rsp + 0x20], rax
0x1404eccd0: mov qword ptr [rsp + 0x28], 8
0x1404eccd9: mov rcx, rbx
0x1404eccdc: mov rdx, rdi
0x1404eccdf: mov r9, r13
0x1404ecce2: call 0x1400fb890
0x1404ecce7: cmp esi, 0x13
0x1404eccea: sete byte ptr [rsp + 0x30]
0x1404eccef: mov qword ptr [rsp + 0x28], r15
0x1404eccf4: mov qword ptr [rsp + 0x20], r14
0x1404eccf9: lea rcx, [rbp + 0x2e0]
0x1404ecd00: mov rdx, rbx
0x1404ecd03: mov r8, rdi
0x1404ecd06: mov r9, r13
0x1404ecd09: call 0x140a5a570
0x1404ecd0e: cmp byte ptr [rbp + 0x46e], 0
0x1404ecd15: jne 0x1404ece6d
0x1404ecd1b: lea rcx, [rbp + 0x2e4]
0x1404ecd22: call 0x140bd5790
0x1404ecd27: lea rcx, [rbp + 0x2c0]
0x1404ecd2e: mov rdx, rax
0x1404ecd31: mov r8d, 0x7a
0x1404ecd37: call 0x140bd52a0
0x1404ecd3c: movups xmm0, xmmword ptr [rbp + 0x2c0]
0x1404ecd43: movups xmm1, xmmword ptr [rbp + 0x2d0]
0x1404ecd4a: movaps xmmword ptr [rbp + 0x320], xmm1
0x1404ecd51: movaps xmmword ptr [rbp + 0x310], xmm0
0x1404ecd58: lea rcx, [rbp + 0x3f0]
0x1404ecd5f: lea rdx, [rbp + 0x310]
0x1404ecd66: call 0x140bd6ce0
0x1404ecd6b: lea rax, [rip + 0x24ff2ee]
0x1404ecd72: mov qword ptr [rbp + 0x3e0], rax
0x1404ecd79: mov qword ptr [rbp + 0x3e8], 0xa
0x1404ecd84: lea rsi, [rbp + 0x3e0]
0x1404ecd8b: mov rcx, rbx
0x1404ecd8e: mov rdx, rsi
0x1404ecd91: call 0x140be94d0
0x1404ecd96: movups xmm0, xmmword ptr [rbp + 0x2c0]
0x1404ecd9d: movups xmm1, xmmword ptr [rbp + 0x2d0]
0x1404ecda4: movups xmmword ptr [rbp + 0x140], xmm0
0x1404ecdab: movups xmmword ptr [rbp + 0x150], xmm1
0x1404ecdb2: mov dword ptr [rbp + 0x13c], eax
0x1404ecdb8: mov rax, qword ptr [rbp + 0x4e8]
0x1404ecdbf: movss xmm0, dword ptr [rax + 0x9c]
0x1404ecdc7: mov rax, qword ptr [rbp + 0x3d0]
0x1404ecdce: mov qword ptr [rbp + 0x400], rax
0x1404ecdd5: movaps xmm1, xmmword ptr [rbp + 0x3b0]
0x1404ecddc: movaps xmm2, xmmword ptr [rbp + 0x3c0]
0x1404ecde3: movaps xmmword ptr [rbp + 0x3f0], xmm2
0x1404ecdea: movaps xmmword ptr [rbp + 0x3e0], xmm1
0x1404ecdf1: movss dword ptr [rsp + 0x40], xmm0
0x1404ecdf7: mov qword ptr [rsp + 0x38], rsi
0x1404ecdfc: lea rax, [rbp + 0x1b0]
0x1404ece03: mov qword ptr [rsp + 0x30], rax
0x1404ece08: lea rax, [rbp + 0x18c]
0x1404ece0f: mov qword ptr [rsp + 0x28], rax
0x1404ece14: lea rax, [rbp + 0x13c]
0x1404ece1b: mov qword ptr [rsp + 0x20], rax
0x1404ece20: lea rcx, [rbp + 0x310]
0x1404ece27: lea r9, [rbp + 0x2e0]
0x1404ece2e: mov rdx, rbx
0x1404ece31: mov r8, rdi
0x1404ece34: call 0x140a3b2f0
0x1404ece39: jmp 0x1404ecff4
0x1404ece3e: mov eax, dword ptr [r13 + 0x20]
0x1404ece42: mov dword ptr [rbp + 0x300], eax
0x1404ece48: movups xmm0, xmmword ptr [r13]
0x1404ece4d: movups xmm1, xmmword ptr [r13 + 0x10]
0x1404ece52: movaps xmmword ptr [rbp + 0x2f0], xmm1
0x1404ece59: movaps xmmword ptr [rbp + 0x2e0], xmm0
0x1404ece60: cmp byte ptr [rbp + 0x46e], 0
0x1404ece67: je 0x1404ecd1b
0x1404ece6d: movabs rdx, 0xa4c02661e67b048b
0x1404ece77: movabs r8, 0xeae761e6aa2a67ec
0x1404ece81: mov rcx, qword ptr [rbp + 0x3d8]
0x1404ece88: call 0x1400dd6f0
0x1404ece8d: test rax, rax
0x1404ece90: je 0x1404ed0df
0x1404ece96: cmp qword ptr [rdi + 0x218], rdx
0x1404ece9d: jbe 0x1404ed0df
0x1404ecea3: mov rax, qword ptr [rdi + 0x210]
0x1404eceaa: mov rcx, qword ptr [rax + rdx*8]
0x1404eceae: test rcx, rcx
0x1404eceb1: je 0x1404ed0df
0x1404eceb7: mov rax, qword ptr [rdi + 0x1e0]
0x1404ecebe: not rcx
0x1404ecec1: lea rcx, [rcx + rcx*2]
0x1404ecec5: shl rcx, 5
0x1404ecec9: cmp qword ptr [rax + rcx + 0x20], 0
0x1404ececf: je 0x1404ed0df
0x1404eced5: add rax, rcx
0x1404eced8: mov rax, qword ptr [rax + 0x10]
0x1404ecedc: mov rdx, qword ptr [rax + 8]
0x1404ecee0: lea rcx, [rbp + 0x2c0]
0x1404ecee7: mov r8d, 0x7a
0x1404eceed: call 0x140bd52a0
0x1404ecef2: movups xmm0, xmmword ptr [rbp + 0x2c0]
0x1404ecef9: movups xmm1, xmmword ptr [rbp + 0x2d0]
0x1404ecf00: movaps xmmword ptr [rbp + 0x320], xmm1
0x1404ecf07: movaps xmmword ptr [rbp + 0x310], xmm0
0x1404ecf0e: lea rcx, [rbp + 0x3f0]
0x1404ecf15: lea rdx, [rbp + 0x310]
0x1404ecf1c: call 0x140bd6ce0
0x1404ecf21: lea rax, [rip + 0x24ff138]
0x1404ecf28: mov qword ptr [rbp + 0x3e0], rax
0x1404ecf2f: mov qword ptr [rbp + 0x3e8], 0xa
0x1404ecf3a: lea rsi, [rbp + 0x3e0]
0x1404ecf41: mov rcx, rbx
0x1404ecf44: mov rdx, rsi
0x1404ecf47: call 0x140be94d0
0x1404ecf4c: movups xmm0, xmmword ptr [rbp + 0x2c0]
0x1404ecf53: movups xmm1, xmmword ptr [rbp + 0x2d0]
0x1404ecf5a: movups xmmword ptr [rbp + 0x11c], xmm0
0x1404ecf61: movups xmmword ptr [rbp + 0x12c], xmm1
0x1404ecf68: mov dword ptr [rbp + 0x118], eax
0x1404ecf6e: mov rax, qword ptr [rbp + 0x4e8]
0x1404ecf75: movss xmm0, dword ptr [rax + 0x9c]
0x1404ecf7d: mov rax, qword ptr [rbp + 0x3d0]
0x1404ecf84: mov qword ptr [rbp + 0x400], rax
0x1404ecf8b: movaps xmm1, xmmword ptr [rbp + 0x3b0]
0x1404ecf92: movaps xmm2, xmmword ptr [rbp + 0x3c0]
0x1404ecf99: movaps xmmword ptr [rbp + 0x3f0], xmm2
0x1404ecfa0: movaps xmmword ptr [rbp + 0x3e0], xmm1
0x1404ecfa7: movss dword ptr [rsp + 0x48], xmm0
0x1404ecfad: mov qword ptr [rsp + 0x40], rsi
0x1404ecfb2: lea rax, [rbp + 0x1b0]
0x1404ecfb9: mov qword ptr [rsp + 0x38], rax
0x1404ecfbe: lea rax, [rbp + 0x18c]
0x1404ecfc5: mov qword ptr [rsp + 0x30], rax
0x1404ecfca: lea rax, [rbp + 0x118]
0x1404ecfd1: mov qword ptr [rsp + 0x28], rax
0x1404ecfd6: mov qword ptr [rsp + 0x20], r14
0x1404ecfdb: lea rcx, [rbp + 0x310]
0x1404ecfe2: lea r9, [rbp + 0x2e0]
0x1404ecfe9: mov rdx, rbx
0x1404ecfec: mov r8, rdi
0x1404ecfef: call 0x140a3ba60
0x1404ecff4: lea rsi, [rbp + 0x310]
0x1404ecffb: mov rcx, rbx
0x1404ecffe: mov rdx, rdi
0x1404ed001: mov r8, rsi
0x1404ed004: mov r9, qword ptr [rbp + 0x460]
0x1404ed00b: call 0x1409c40b0
0x1404ed010: mov rcx, rbx
0x1404ed013: mov rdx, rdi
0x1404ed016: mov r8, rsi
0x1404ed019: call 0x14098f9f0
0x1404ed01e: mov eax, dword ptr [rbp + 0x430]
0x1404ed024: mov dword ptr [r12 + 0x44], eax
0x1404ed029: movups xmm0, xmmword ptr [rbp + 0x410]
0x1404ed030: movups xmm1, xmmword ptr [rbp + 0x420]
0x1404ed037: movups xmmword ptr [r12 + 0x34], xmm1
0x1404ed03d: movups xmmword ptr [r12 + 0x24], xmm0
0x1404ed043: movups xmm0, xmmword ptr [rbp + 0x310]
0x1404ed04a: movups xmm1, xmmword ptr [rbp + 0x320]
0x1404ed051: movups xmmword ptr [r12 + 0x48], xmm0
0x1404ed057: movups xmmword ptr [r12 + 0x58], xmm1
0x1404ed05d: mov eax, dword ptr [rbp + 0x330]
0x1404ed063: mov dword ptr [r12 + 0x68], eax
0x1404ed068: movaps xmm0, xmmword ptr [rbp + 0x210]
0x1404ed06f: movaps xmm1, xmmword ptr [rbp + 0x220]
0x1404ed076: movups xmmword ptr [r12], xmm0
0x1404ed07b: movups xmmword ptr [r12 + 0x10], xmm1
0x1404ed081: mov eax, dword ptr [rbp + 0x230]
0x1404ed087: mov dword ptr [r12 + 0x20], eax
0x1404ed08c: mov rax, r12
0x1404ed08f: add rsp, 0x4f8
0x1404ed096: pop rbx
0x1404ed097: pop rdi
0x1404ed098: pop rsi
0x1404ed099: pop r12
0x1404ed09b: pop r13
0x1404ed09d: pop r14
0x1404ed09f: pop r15
0x1404ed0a1: pop rbp
0x1404ed0a2: ret 
0x1404ed0a3: lea rcx, [rip + 0x24fee76]
0x1404ed0aa: lea r8, [rip + 0x24feeb7]
0x1404ed0b1: mov edx, 0x46
0x1404ed0b6: call 0x1428d9430
0x1404ed0bb: lea rcx, [rip + 0x24feebe]
0x1404ed0c2: call 0x1428d9310
0x1404ed0c7: lea rcx, [rip + 0x24fef72]
0x1404ed0ce: call 0x1428d9310
0x1404ed0d3: lea rcx, [rip + 0x24fef0e]
0x1404ed0da: call 0x1428d9310
0x1404ed0df: lea rcx, [rip + 0x24fef8a]
0x1404ed0e6: call 0x1428d9310
0x1404ed0eb: mov rax, qword ptr [rbp + 0x340]
0x1404ed0f2: mov qword ptr [rbp + 0x240], rax
0x1404ed0f9: lea rax, [rip + 0x24feeb0]
0x1404ed100: mov qword ptr [rsp + 0x20], rax
0x1404ed105: lea rcx, [rip + 0x24fe794]
0x1404ed10c: lea r9, [rip + 0x24fe76d]
0x1404ed113: lea r8, [rbp + 0x240]
0x1404ed11a: mov edx, 0x2b
0x1404ed11f: call 0x1428d9760
0x1404ed124: jmp 0x1404ed15f
0x1404ed126: mov rax, qword ptr [rbp + 0x348]
0x1404ed12d: mov qword ptr [rbp + 0x410], rax
0x1404ed134: lea rax, [rip + 0x24feed5]
0x1404ed13b: mov qword ptr [rsp + 0x20], rax
0x1404ed140: lea rcx, [rip + 0x24fe759]
0x1404ed147: lea r9, [rip + 0x24fe732]
0x1404ed14e: lea r8, [rbp + 0x410]
0x1404ed155: mov edx, 0x2b
0x1404ed15a: call 0x1428d9760
0x1404ed15f: ud2 
0x1404ed161: nop word ptr cs:[rax + rax]
0x1404ed170: mov qword ptr [rsp + 0x10], rdx
0x1404ed175: push rbp
0x1404ed176: push r15
0x1404ed178: push r14
0x1404ed17a: push r13
0x1404ed17c: push r12
0x1404ed17e: push rsi
0x1404ed17f: push rdi
0x1404ed180: push rbx
0x1404ed181: sub rsp, 0x68
0x1404ed185: lea rbp, [rdx + 0x80]
0x1404ed18c: lea rcx, [rbp + 0x240]
0x1404ed193: call 0x140d4cc30
0x1404ed198: nop 
0x1404ed199: add rsp, 0x68
0x1404ed19d: pop rbx
0x1404ed19e: pop rdi
0x1404ed19f: pop rsi
0x1404ed1a0: pop r12
0x1404ed1a2: pop r13
0x1404ed1a4: pop r14
0x1404ed1a6: pop r15
0x1404ed1a8: pop rbp
0x1404ed1a9: ret 
0x1404ed1aa: nop word ptr [rax + rax]
0x1404ed1b0: mov qword ptr [rsp + 0x10], rdx
0x1404ed1b5: push rbp
0x1404ed1b6: push r15
0x1404ed1b8: push r14
0x1404ed1ba: push r13
0x1404ed1bc: push r12
0x1404ed1be: push rsi
0x1404ed1bf: push rdi
0x1404ed1c0: push rbx
0x1404ed1c1: sub rsp, 0x68
0x1404ed1c5: lea rbp, [rdx + 0x80]
0x1404ed1cc: lea rcx, [rbp + 0x410]
0x1404ed1d3: call 0x140d4cc30
0x1404ed1d8: nop 
0x1404ed1d9: add rsp, 0x68
0x1404ed1dd: pop rbx
0x1404ed1de: pop rdi
0x1404ed1df: pop rsi
0x1404ed1e0: pop r12
0x1404ed1e2: pop r13
0x1404ed1e4: pop r14
0x1404ed1e6: pop r15
0x1404ed1e8: pop rbp
0x1404ed1e9: ret 
