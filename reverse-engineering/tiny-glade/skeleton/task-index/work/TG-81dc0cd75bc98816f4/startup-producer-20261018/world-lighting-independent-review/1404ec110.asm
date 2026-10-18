1404ec110: 55 push rbp
1404ec111: 4157 push r15
1404ec113: 4156 push r14
1404ec115: 4155 push r13
1404ec117: 4154 push r12
1404ec119: 56 push rsi
1404ec11a: 57 push rdi
1404ec11b: 53 push rbx
1404ec11c: 4881ecf8040000 sub rsp, 0x4f8
1404ec123: 488dac2480000000 lea rbp, [rsp + 0x80]
1404ec12b: 48c78570040000feffffff mov qword ptr [rbp + 0x470], 0xfffffffffffffffe
1404ec136: 4c898d60040000 mov qword ptr [rbp + 0x460], r9
1404ec13d: 4c8bbde0040000 mov r15, qword ptr [rbp + 0x4e0]
1404ec144: 4c89f8 mov rax, r15
1404ec147: 48c1e820 shr rax, 0x20
1404ec14b: 0f84520f0000 je 0x1404ed0a3
1404ec151: 4489f8 mov eax, r15d
1404ec154: 4885c0 test rax, rax
1404ec157: 0f84460f0000 je 0x1404ed0a3
1404ec15d: 4c89c7 mov rdi, r8
1404ec160: 4889d3 mov rbx, rdx
1404ec163: 4889ce mov rsi, rcx
1404ec166: 498d4838 lea rcx, [r8 + 0x38]
1404ec16a: 48ba9b71d8c6fc0d4374 movabs rdx, 0x74430dfcc6d8719b
1404ec174: 49b89c5064e4f79155ca movabs r8, 0xca5591f7e464509c
1404ec17e: 48898dd8030000 mov qword ptr [rbp + 0x3d8], rcx
1404ec185: e86615bfff call 0x1400dd6f0
1404ec18a: 4885c0 test rax, rax
1404ec18d: 0f84280f0000 je 0x1404ed0bb
1404ec193: 48399718020000 cmp qword ptr [rdi + 0x218], rdx
1404ec19a: 0f861b0f0000 jbe 0x1404ed0bb
1404ec1a0: 488b8710020000 mov rax, qword ptr [rdi + 0x210]
1404ec1a7: 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
1404ec1ab: 4885c9 test rcx, rcx
1404ec1ae: 0f84070f0000 je 0x1404ed0bb
1404ec1b4: 488b87e0010000 mov rax, qword ptr [rdi + 0x1e0]
1404ec1bb: 48f7d1 not rcx
1404ec1be: 488d0c49 lea rcx, [rcx + rcx*2]
1404ec1c2: 48c1e105 shl rcx, 5
1404ec1c6: 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
1404ec1cc: 0f84e90e0000 je 0x1404ed0bb
1404ec1d2: 4889b5b8020000 mov qword ptr [rbp + 0x2b8], rsi
1404ec1d9: 4c8ba5f0040000 mov r12, qword ptr [rbp + 0x4f0]
1404ec1e0: 4801c8 add rax, rcx
1404ec1e3: 488b4810 mov rcx, qword ptr [rax + 0x10]
1404ec1e7: 4883c158 add rcx, 0x58
1404ec1eb: e830c15f00 call 0x140ae8320
1404ec1f0: 88856f040000 mov byte ptr [rbp + 0x46f], al
1404ec1f6: 410f10442404 movups xmm0, xmmword ptr [r12 + 4]
1404ec1fc: 410f104c2414 movups xmm1, xmmword ptr [r12 + 0x14]
1404ec202: 0f298d50030000 movaps xmmword ptr [rbp + 0x350], xmm1
1404ec209: 0f298540030000 movaps xmmword ptr [rbp + 0x340], xmm0
1404ec210: 488db540020000 lea rsi, [rbp + 0x240]
1404ec217: 488d9540030000 lea rdx, [rbp + 0x340]
1404ec21e: 4889f1 mov rcx, rsi
1404ec221: 41b864000000 mov r8d, 0x64
1404ec227: e804926e00 call 0x140bd5430
1404ec22c: 4c8db510040000 lea r14, [rbp + 0x410]
1404ec233: 4c89f1 mov rcx, r14
1404ec236: 4889f2 mov rdx, rsi
1404ec239: 41b80c000000 mov r8d, 0xc
1404ec23f: e8fc906e00 call 0x140bd5340
1404ec244: 4c89742420 mov qword ptr [rsp + 0x20], r14
1404ec249: 4c8d0548fd4f02 lea r8, [rip + 0x24ffd48]
1404ec250: 488d8d40030000 lea rcx, [rbp + 0x340]
1404ec257: 41b917000000 mov r9d, 0x17
1404ec25d: 4889da mov rdx, rbx
1404ec260: e8ebab0000 call 0x1404f6e50
1404ec265: 80bd6403000002 cmp byte ptr [rbp + 0x364], 2
1404ec26c: 0f84790e0000 je 0x1404ed0eb
1404ec272: 488b8560030000 mov rax, qword ptr [rbp + 0x360]
1404ec279: 488985d0000000 mov qword ptr [rbp + 0xd0], rax
1404ec280: 0f108540030000 movups xmm0, xmmword ptr [rbp + 0x340]
1404ec287: 0f108d50030000 movups xmm1, xmmword ptr [rbp + 0x350]
1404ec28e: 0f298dc0000000 movaps xmmword ptr [rbp + 0xc0], xmm1
1404ec295: 0f2985b0000000 movaps xmmword ptr [rbp + 0xb0], xmm0
1404ec29c: 4889d9 mov rcx, rbx
1404ec29f: 4889fa mov rdx, rdi
1404ec2a2: 41b001 mov r8b, 1
1404ec2a5: e826c9eeff call 0x1403d8bd0
1404ec2aa: 4889d9 mov rcx, rbx
1404ec2ad: 4889fa mov rdx, rdi
1404ec2b0: 41b002 mov r8b, 2
1404ec2b3: e898ddeeff call 0x1403da050
1404ec2b8: 4889d9 mov rcx, rbx
1404ec2bb: 4889fa mov rdx, rdi
1404ec2be: 41b003 mov r8b, 3
1404ec2c1: e8aae2eeff call 0x1403da570
1404ec2c6: 4889d9 mov rcx, rbx
1404ec2c9: 4889fa mov rdx, rdi
1404ec2cc: 41b004 mov r8b, 4
1404ec2cf: e81cceeeff call 0x1403d90f0
1404ec2d4: 4889d9 mov rcx, rbx
1404ec2d7: 4889fa mov rdx, rdi
1404ec2da: 41b013 mov r8b, 0x13
1404ec2dd: e82ed3eeff call 0x1403d9610
1404ec2e2: 4889d9 mov rcx, rbx
1404ec2e5: 4889fa mov rdx, rdi
1404ec2e8: 41b014 mov r8b, 0x14
1404ec2eb: e840d8eeff call 0x1403d9b30
1404ec2f0: 4889d9 mov rcx, rbx
1404ec2f3: 4889fa mov rdx, rdi
1404ec2f6: e885635200 call 0x140a12680
1404ec2fb: 4889d9 mov rcx, rbx
1404ec2fe: 4889fa mov rdx, rdi
1404ec301: e81aae3d00 call 0x1408c7120
1404ec306: 4889d9 mov rcx, rbx
1404ec309: 4889fa mov rdx, rdi
1404ec30c: e86f2b5500 call 0x140a3ee80
1404ec311: 4889d9 mov rcx, rbx
1404ec314: 4889fa mov rdx, rdi
1404ec317: e864034a00 call 0x14098c680
1404ec31c: 4889d9 mov rcx, rbx
1404ec31f: 4889fa mov rdx, rdi
1404ec322: e889a54800 call 0x1409768b0
1404ec327: 4889d9 mov rcx, rbx
1404ec32a: 4889fa mov rdx, rdi
1404ec32d: e87ef44d00 call 0x1409cb7b0
1404ec332: 4889d9 mov rcx, rbx
1404ec335: 4889fa mov rdx, rdi
1404ec338: e813c15600 call 0x140a58450
1404ec33d: 4889d9 mov rcx, rbx
1404ec340: 4889fa mov rdx, rdi
1404ec343: e858e74e00 call 0x1409daaa0
1404ec348: 4889d9 mov rcx, rbx
1404ec34b: 4889fa mov rdx, rdi
1404ec34e: e88d514100 call 0x1409014e0
1404ec353: 4889d9 mov rcx, rbx
1404ec356: 4889fa mov rdx, rdi
1404ec359: e812645400 call 0x140a32770
1404ec35e: 4889d9 mov rcx, rbx
1404ec361: 4889fa mov rdx, rdi
1404ec364: e8d75d5300 call 0x140a22140
1404ec369: 4889d9 mov rcx, rbx
1404ec36c: 4889fa mov rdx, rdi
1404ec36f: e8fc755200 call 0x140a13970
1404ec374: 4889d9 mov rcx, rbx
1404ec377: 4889fa mov rdx, rdi
1404ec37a: e841893f00 call 0x1408e4cc0
1404ec37f: 4889d9 mov rcx, rbx
1404ec382: 4889fa mov rdx, rdi
1404ec385: e8b6a13700 call 0x140866540
1404ec38a: 4889d9 mov rcx, rbx
1404ec38d: 4889fa mov rdx, rdi
1404ec390: e8eb675300 call 0x140a22b80
1404ec395: 4889d9 mov rcx, rbx
1404ec398: 4889fa mov rdx, rdi
1404ec39b: e8d0cb4600 call 0x140958f70
1404ec3a0: 4889d9 mov rcx, rbx
1404ec3a3: 4889fa mov rdx, rdi
1404ec3a6: e855894000 call 0x1408f4d00
1404ec3ab: 4889d9 mov rcx, rbx
1404ec3ae: 4889fa mov rdx, rdi
1404ec3b1: e85ad75300 call 0x140a29b10
1404ec3b6: 4889d9 mov rcx, rbx
1404ec3b9: 4889fa mov rdx, rdi
1404ec3bc: e88fc65700 call 0x140a68a50
1404ec3c1: 4889d9 mov rcx, rbx
1404ec3c4: 4889fa mov rdx, rdi
1404ec3c7: e824e74900 call 0x14098aaf0
1404ec3cc: 4889d9 mov rcx, rbx
1404ec3cf: 4889fa mov rdx, rdi
1404ec3d2: 4531c0 xor r8d, r8d
1404ec3d5: e8f6b94900 call 0x140987dd0
1404ec3da: 4889d9 mov rcx, rbx
1404ec3dd: 4889fa mov rdx, rdi
1404ec3e0: 4c8bad60040000 mov r13, qword ptr [rbp + 0x460]
1404ec3e7: 4d89e8 mov r8, r13
1404ec3ea: e881fe4d00 call 0x1409cc270
1404ec3ef: 4889d9 mov rcx, rbx
1404ec3f2: 4889fa mov rdx, rdi
1404ec3f5: e846505600 call 0x140a51440
1404ec3fa: 488d8d10040000 lea rcx, [rbp + 0x410]
1404ec401: 4c89fa mov rdx, r15
1404ec404: 41b865000000 mov r8d, 0x65
1404ec40a: e8918e6e00 call 0x140bd52a0
1404ec40f: 0f108510040000 movups xmm0, xmmword ptr [rbp + 0x410]
1404ec416: 0f108d20040000 movups xmm1, xmmword ptr [rbp + 0x420]
1404ec41d: 0f298d50020000 movaps xmmword ptr [rbp + 0x250], xmm1
1404ec424: 0f298540020000 movaps xmmword ptr [rbp + 0x240], xmm0
1404ec42b: 488d8d50030000 lea rcx, [rbp + 0x350]
1404ec432: 488d9540020000 lea rdx, [rbp + 0x240]
1404ec439: e8a2a86e00 call 0x140bd6ce0
1404ec43e: 488d0583fb4f02 lea rax, [rip + 0x24ffb83]
1404ec445: 48898540030000 mov qword ptr [rbp + 0x340], rax
1404ec44c: 48c7854803000007000000 mov qword ptr [rbp + 0x348], 7
1404ec457: 488d9540030000 lea rdx, [rbp + 0x340]
1404ec45e: 4889d9 mov rcx, rbx
1404ec461: e86ad06f00 call 0x140be94d0
1404ec466: 89c6 mov esi, eax
1404ec468: 0f108510040000 movups xmm0, xmmword ptr [rbp + 0x410]
1404ec46f: 0f108d20040000 movups xmm1, xmmword ptr [rbp + 0x420]
1404ec476: 0f2985e0000000 movaps xmmword ptr [rbp + 0xe0], xmm0
1404ec47d: 0f298df0000000 movaps xmmword ptr [rbp + 0xf0], xmm1
1404ec484: 488d8de0030000 lea rcx, [rbp + 0x3e0]
1404ec48b: 4c89fa mov rdx, r15
1404ec48e: 41b861000000 mov r8d, 0x61
1404ec494: e8078e6e00 call 0x140bd52a0
1404ec499: 4c8db588020000 lea r14, [rbp + 0x288]
1404ec4a0: 0f1085e0030000 movups xmm0, xmmword ptr [rbp + 0x3e0]
1404ec4a7: 0f108df0030000 movups xmm1, xmmword ptr [rbp + 0x3f0]
1404ec4ae: 0f298d20040000 movaps xmmword ptr [rbp + 0x420], xmm1
1404ec4b5: 0f298510040000 movaps xmmword ptr [rbp + 0x410], xmm0
1404ec4bc: 488d8d50030000 lea rcx, [rbp + 0x350]
1404ec4c3: 488d9510040000 lea rdx, [rbp + 0x410]
1404ec4ca: e811a86e00 call 0x140bd6ce0
1404ec4cf: 488d05f9fa4f02 lea rax, [rip + 0x24ffaf9]
1404ec4d6: 48898540030000 mov qword ptr [rbp + 0x340], rax
1404ec4dd: 48c7854803000008000000 mov qword ptr [rbp + 0x348], 8
1404ec4e8: 488d9540030000 lea rdx, [rbp + 0x340]
1404ec4ef: 4889d9 mov rcx, rbx
1404ec4f2: e8d9cf6f00 call 0x140be94d0
1404ec4f7: 0f1085e0030000 movups xmm0, xmmword ptr [rbp + 0x3e0]
1404ec4fe: 0f108df0030000 movups xmm1, xmmword ptr [rbp + 0x3f0]
1404ec505: 0f11858c020000 movups xmmword ptr [rbp + 0x28c], xmm0
1404ec50c: 0f118d9c020000 movups xmmword ptr [rbp + 0x29c], xmm1
1404ec513: 898588020000 mov dword ptr [rbp + 0x288], eax
1404ec519: 89b540020000 mov dword ptr [rbp + 0x240], esi
1404ec51f: 0f2885e0000000 movaps xmm0, xmmword ptr [rbp + 0xe0]
1404ec526: 0f288df0000000 movaps xmm1, xmmword ptr [rbp + 0xf0]
1404ec52d: 0f118544020000 movups xmmword ptr [rbp + 0x244], xmm0
1404ec534: 0f118d54020000 movups xmmword ptr [rbp + 0x254], xmm1
1404ec53b: 488db564020000 lea rsi, [rbp + 0x264]
1404ec542: 410f100424 movups xmm0, xmmword ptr [r12]
1404ec547: 410f104c2410 movups xmm1, xmmword ptr [r12 + 0x10]
1404ec54d: 0f118564020000 movups xmmword ptr [rbp + 0x264], xmm0
1404ec554: 0f118d74020000 movups xmmword ptr [rbp + 0x274], xmm1
1404ec55b: 418b442420 mov eax, dword ptr [r12 + 0x20]
1404ec560: 898584020000 mov dword ptr [rbp + 0x284], eax
1404ec566: 4c896c2420 mov qword ptr [rsp + 0x20], r13
1404ec56b: 4c8bbdf8040000 mov r15, qword ptr [rbp + 0x4f8]
1404ec572: 4c8da540020000 lea r12, [rbp + 0x240]
1404ec579: 4889d9 mov rcx, rbx
1404ec57c: 4889fa mov rdx, rdi
1404ec57f: 4d89e0 mov r8, r12
1404ec582: 4d89f9 mov r9, r15
1404ec585: e836763400 call 0x140833bc0
1404ec58a: 488d0546fa4f02 lea rax, [rip + 0x24ffa46]
1404ec591: 4889442420 mov qword ptr [rsp + 0x20], rax
1404ec596: 48c744242804000000 mov qword ptr [rsp + 0x28], 4
1404ec59f: 488d4d60 lea rcx, [rbp + 0x60]
1404ec5a3: 4889da mov rdx, rbx
1404ec5a6: 4989f8 mov r8, rdi
1404ec5a9: 4989f1 mov r9, rsi
1404ec5ac: e83fc14300 call 0x1409286f0
1404ec5b1: 488b0520fb1303 mov rax, qword ptr [rip + 0x313fb20]
1404ec5b8: 803800 cmp byte ptr [rax], 0
1404ec5bb: 7520 jne 0x1404ec5dd
1404ec5bd: 488d7560 lea rsi, [rbp + 0x60]
1404ec5c1: 4889d9 mov rcx, rbx
1404ec5c4: 4889fa mov rdx, rdi
1404ec5c7: 4989f0 mov r8, rsi
1404ec5ca: e8d1ec4900 call 0x14098b2a0
1404ec5cf: 4889d9 mov rcx, rbx
1404ec5d2: 4889fa mov rdx, rdi
1404ec5d5: 4989f0 mov r8, rsi
1404ec5d8: e8f3b74900 call 0x140987dd0
1404ec5dd: 4c8d4560 lea r8, [rbp + 0x60]
1404ec5e1: 4889d9 mov rcx, rbx
1404ec5e4: 4889fa mov rdx, rdi
1404ec5e7: e8943c5700 call 0x140a60280
1404ec5ec: 488bb560040000 mov rsi, qword ptr [rbp + 0x460]
1404ec5f3: 4889742420 mov qword ptr [rsp + 0x20], rsi
1404ec5f8: 4889d9 mov rcx, rbx
1404ec5fb: 4889fa mov rdx, rdi
1404ec5fe: 4d89e0 mov r8, r12
1404ec601: 4d89f9 mov r9, r15
1404ec604: e837943400 call 0x140835a40
1404ec609: 4889d9 mov rcx, rbx
1404ec60c: 4889fa mov rdx, rdi
1404ec60f: 4c8dad64020000 lea r13, [rbp + 0x264]
1404ec616: 4d89e8 mov r8, r13
1404ec619: e8c22c5400 call 0x140a2f2e0
1404ec61e: 4889d9 mov rcx, rbx
1404ec621: 4889fa mov rdx, rdi
1404ec624: 4d89e8 mov r8, r13
1404ec627: e854b25200 call 0x140a17880
1404ec62c: 4889d9 mov rcx, rbx
1404ec62f: 4889fa mov rdx, rdi
1404ec632: 4d89e8 mov r8, r13
1404ec635: e8e64f5100 call 0x140a01620
1404ec63a: 4889742420 mov qword ptr [rsp + 0x20], rsi
1404ec63f: 4889d9 mov rcx, rbx
1404ec642: 4889fa mov rdx, rdi
1404ec645: 4d89e0 mov r8, r12
1404ec648: 4d89f9 mov r9, r15
1404ec64b: e860ad3400 call 0x1408373b0
1404ec650: 488d8d40030000 lea rcx, [rbp + 0x340]
1404ec657: 4889da mov rdx, rbx
1404ec65a: 4989f8 mov r8, rdi
1404ec65d: 4d89e1 mov r9, r12
1404ec660: e81b9b4000 call 0x1408f6180
1404ec665: 8b8560030000 mov eax, dword ptr [rbp + 0x360]
1404ec66b: 898500020000 mov dword ptr [rbp + 0x200], eax
1404ec671: 0f108540030000 movups xmm0, xmmword ptr [rbp + 0x340]
1404ec678: 0f108d50030000 movups xmm1, xmmword ptr [rbp + 0x350]
1404ec67f: 0f298df0010000 movaps xmmword ptr [rbp + 0x1f0], xmm1
1404ec686: 0f2985e0010000 movaps xmmword ptr [rbp + 0x1e0], xmm0
1404ec68d: 8b8584030000 mov eax, dword ptr [rbp + 0x384]
1404ec693: 898580010000 mov dword ptr [rbp + 0x180], eax
1404ec699: 0f108574030000 movups xmm0, xmmword ptr [rbp + 0x374]
1404ec6a0: 0f298570010000 movaps xmmword ptr [rbp + 0x170], xmm0
1404ec6a7: 0f108564030000 movups xmm0, xmmword ptr [rbp + 0x364]
1404ec6ae: 0f298560010000 movaps xmmword ptr [rbp + 0x160], xmm0
1404ec6b5: 488d85b0000000 lea rax, [rbp + 0xb0]
1404ec6bc: 4889442428 mov qword ptr [rsp + 0x28], rax
1404ec6c1: 4c89742420 mov qword ptr [rsp + 0x20], r14
1404ec6c6: 488db58c010000 lea rsi, [rbp + 0x18c]
1404ec6cd: 4889f1 mov rcx, rsi
1404ec6d0: 4889da mov rdx, rbx
1404ec6d3: 4989f8 mov r8, rdi
1404ec6d6: 4d89e9 mov r9, r13
1404ec6d9: e8b20a5200 call 0x140a0d190
1404ec6de: 4889742430 mov qword ptr [rsp + 0x30], rsi
1404ec6e3: 4c89642428 mov qword ptr [rsp + 0x28], r12
1404ec6e8: 4c8dad60010000 lea r13, [rbp + 0x160]
1404ec6ef: 4c896c2420 mov qword ptr [rsp + 0x20], r13
1404ec6f4: 488d8d8c000000 lea rcx, [rbp + 0x8c]
1404ec6fb: 4c8d8de0010000 lea r9, [rbp + 0x1e0]
1404ec702: 4889da mov rdx, rbx
1404ec705: 4989f8 mov r8, rdi
1404ec708: e8a37c3800 call 0x1408743b0
1404ec70d: 488b85e8040000 mov rax, qword ptr [rbp + 0x4e8]
1404ec714: 440fb6a0ab000000 movzx r12d, byte ptr [rax + 0xab]
1404ec71c: 4584e4 test r12b, r12b
1404ec71f: 0f84c7000000 je 0x1404ec7ec
1404ec725: 48ba8b047be66126c0a4 movabs rdx, 0xa4c02661e67b048b
1404ec72f: 49b8ec672aaae661e7ea movabs r8, 0xeae761e6aa2a67ec
1404ec739: 488b8dd8030000 mov rcx, qword ptr [rbp + 0x3d8]
1404ec740: e8ab0fbfff call 0x1400dd6f0
1404ec745: 4885c0 test rax, rax
1404ec748: 0f8485090000 je 0x1404ed0d3
1404ec74e: 48399718020000 cmp qword ptr [rdi + 0x218], rdx
1404ec755: 0f8678090000 jbe 0x1404ed0d3
1404ec75b: 488b8710020000 mov rax, qword ptr [rdi + 0x210]
1404ec762: 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
1404ec766: 4885c9 test rcx, rcx
1404ec769: 0f8464090000 je 0x1404ed0d3
1404ec76f: 488b87e0010000 mov rax, qword ptr [rdi + 0x1e0]
1404ec776: 48f7d1 not rcx
1404ec779: 488d0c49 lea rcx, [rcx + rcx*2]
1404ec77d: 48c1e105 shl rcx, 5
1404ec781: 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
1404ec787: 0f8446090000 je 0x1404ed0d3
1404ec78d: 4801c8 add rax, rcx
1404ec790: 488b4010 mov rax, qword ptr [rax + 0x10]
1404ec794: 488b5008 mov rdx, qword ptr [rax + 8]
1404ec798: 488db540030000 lea rsi, [rbp + 0x340]
1404ec79f: 4889f1 mov rcx, rsi
1404ec7a2: 41b87a000000 mov r8d, 0x7a
1404ec7a8: e8f38a6e00 call 0x140bd52a0
1404ec7ad: 4c8db510040000 lea r14, [rbp + 0x410]
1404ec7b4: 4c89f1 mov rcx, r14
1404ec7b7: 4889f2 mov rdx, rsi
1404ec7ba: e8b18b6e00 call 0x140bd5370
1404ec7bf: 488db5e0030000 lea rsi, [rbp + 0x3e0]
1404ec7c6: 4889f1 mov rcx, rsi
1404ec7c9: 4c89f2 mov rdx, r14
1404ec7cc: e8bf8b6e00 call 0x140bd5390
1404ec7d1: 4889742420 mov qword ptr [rsp + 0x20], rsi
1404ec7d6: 4c8d0523f84f02 lea r8, [rip + 0x24ff823]
1404ec7dd: 488d8d40030000 lea rcx, [rbp + 0x340]
1404ec7e4: 41b90c000000 mov r9d, 0xc
1404ec7ea: eb5e jmp 0x1404ec84a
1404ec7ec: 498d4f04 lea rcx, [r15 + 4]
1404ec7f0: e89b8f6e00 call 0x140bd5790
1404ec7f5: 488db540030000 lea rsi, [rbp + 0x340]
1404ec7fc: 4889f1 mov rcx, rsi
1404ec7ff: 4889c2 mov rdx, rax
1404ec802: 41b87a000000 mov r8d, 0x7a
1404ec808: e8938a6e00 call 0x140bd52a0
1404ec80d: 4c8db510040000 lea r14, [rbp + 0x410]
1404ec814: 4c89f1 mov rcx, r14
1404ec817: 4889f2 mov rdx, rsi
1404ec81a: e8518b6e00 call 0x140bd5370
1404ec81f: 488db5e0030000 lea rsi, [rbp + 0x3e0]
1404ec826: 4889f1 mov rcx, rsi
1404ec829: 4c89f2 mov rdx, r14
1404ec82c: e85f8b6e00 call 0x140bd5390
1404ec831: 4889742420 mov qword ptr [rsp + 0x20], rsi
1404ec836: 4c8d059ef74f02 lea r8, [rip + 0x24ff79e]
1404ec83d: 488d8d40030000 lea rcx, [rbp + 0x340]
1404ec844: 41b90b000000 mov r9d, 0xb
1404ec84a: 4889da mov rdx, rbx
1404ec84d: e8feab0000 call 0x1404f7450
1404ec852: 4488a56e040000 mov byte ptr [rbp + 0x46e], r12b
1404ec859: 83bd4003000001 cmp dword ptr [rbp + 0x340], 1
1404ec860: 0f84c0080000 je 0x1404ed126
1404ec866: 488b8584030000 mov rax, qword ptr [rbp + 0x384]
1404ec86d: 48898550040000 mov qword ptr [rbp + 0x450], rax
1404ec874: 0f108544030000 movups xmm0, xmmword ptr [rbp + 0x344]
1404ec87b: 0f108d54030000 movups xmm1, xmmword ptr [rbp + 0x354]
1404ec882: 0f109564030000 movups xmm2, xmmword ptr [rbp + 0x364]
1404ec889: 0f109d74030000 movups xmm3, xmmword ptr [rbp + 0x374]
1404ec890: 0f299d40040000 movaps xmmword ptr [rbp + 0x440], xmm3
1404ec897: 0f299530040000 movaps xmmword ptr [rbp + 0x430], xmm2
1404ec89e: 0f298d20040000 movaps xmmword ptr [rbp + 0x420], xmm1
1404ec8a5: 0f298510040000 movaps xmmword ptr [rbp + 0x410], xmm0
1404ec8ac: 0fb6858c030000 movzx eax, byte ptr [rbp + 0x38c]
1404ec8b3: 0f108534040000 movups xmm0, xmmword ptr [rbp + 0x434]
1404ec8ba: 0f118543030000 movups xmmword ptr [rbp + 0x343], xmm0
1404ec8c1: 0f108544040000 movups xmm0, xmmword ptr [rbp + 0x444]
1404ec8c8: 0f118553030000 movups xmmword ptr [rbp + 0x353], xmm0
1404ec8cf: 8b8d54040000 mov ecx, dword ptr [rbp + 0x454]
1404ec8d5: 898d63030000 mov dword ptr [rbp + 0x363], ecx
1404ec8db: 8b8d30040000 mov ecx, dword ptr [rbp + 0x430]
1404ec8e1: 898dd0010000 mov dword ptr [rbp + 0x1d0], ecx
1404ec8e7: 0f288510040000 movaps xmm0, xmmword ptr [rbp + 0x410]
1404ec8ee: 0f288d20040000 movaps xmm1, xmmword ptr [rbp + 0x420]
1404ec8f5: 0f298dc0010000 movaps xmmword ptr [rbp + 0x1c0], xmm1
1404ec8fc: 0f2985b0010000 movaps xmmword ptr [rbp + 0x1b0], xmm0
1404ec903: 8b8d63030000 mov ecx, dword ptr [rbp + 0x363]
1404ec909: 898dd0030000 mov dword ptr [rbp + 0x3d0], ecx
1404ec90f: 0f108543030000 movups xmm0, xmmword ptr [rbp + 0x343]
1404ec916: 0f108d53030000 movups xmm1, xmmword ptr [rbp + 0x353]
1404ec91d: 0f298dc0030000 movaps xmmword ptr [rbp + 0x3c0], xmm1
1404ec924: 0f2985b0030000 movaps xmmword ptr [rbp + 0x3b0], xmm0
1404ec92b: 8885d4030000 mov byte ptr [rbp + 0x3d4], al
1404ec931: 488d8d8c000000 lea rcx, [rbp + 0x8c]
1404ec938: 48894c2448 mov qword ptr [rsp + 0x48], rcx
1404ec93d: 4989cc mov r12, rcx
1404ec940: 4c8dbd8c010000 lea r15, [rbp + 0x18c]
1404ec947: 4c897c2440 mov qword ptr [rsp + 0x40], r15
1404ec94c: 2401 and al, 1
1404ec94e: 88442438 mov byte ptr [rsp + 0x38], al
1404ec952: 488d85b0030000 lea rax, [rbp + 0x3b0]
1404ec959: 4889442430 mov qword ptr [rsp + 0x30], rax
1404ec95e: 4c896c2428 mov qword ptr [rsp + 0x28], r13
1404ec963: 488db5e0010000 lea rsi, [rbp + 0x1e0]
1404ec96a: 4889742420 mov qword ptr [rsp + 0x20], rsi
1404ec96f: 4c8d8d40020000 lea r9, [rbp + 0x240]
1404ec976: 4c8db540030000 lea r14, [rbp + 0x340]
1404ec97d: 4c89f1 mov rcx, r14
1404ec980: 4889da mov rdx, rbx
1404ec983: 4989f8 mov r8, rdi
1404ec986: e865535100 call 0x140a01cf0
1404ec98b: 4c897c2438 mov qword ptr [rsp + 0x38], r15
1404ec990: 4c89742430 mov qword ptr [rsp + 0x30], r14
1404ec995: 4c89642428 mov qword ptr [rsp + 0x28], r12
1404ec99a: 4c896c2420 mov qword ptr [rsp + 0x20], r13
1404ec99f: 488d4d3c lea rcx, [rbp + 0x3c]
1404ec9a3: 4889da mov rdx, rbx
1404ec9a6: 4989f8 mov r8, rdi
1404ec9a9: 4989f1 mov r9, rsi
1404ec9ac: e81f835500 call 0x140a44cd0
1404ec9b1: 4c8da5e0000000 lea r12, [rbp + 0xe0]
1404ec9b8: 4c89e1 mov rcx, r12
1404ec9bb: 4889da mov rdx, rbx
1404ec9be: 4989f8 mov r8, rdi
1404ec9c1: e8aaad4f00 call 0x1409e7770
1404ec9c6: 4c89642430 mov qword ptr [rsp + 0x30], r12
1404ec9cb: 4c897c2428 mov qword ptr [rsp + 0x28], r15
1404ec9d0: 4c896c2420 mov qword ptr [rsp + 0x20], r13
1404ec9d5: 4c8d6d18 lea r13, [rbp + 0x18]
1404ec9d9: 4c89e9 mov rcx, r13
1404ec9dc: 4889da mov rdx, rbx
1404ec9df: 4989f8 mov r8, rdi
1404ec9e2: 4989f1 mov r9, rsi
1404ec9e5: e8d6345800 call 0x140a6fec0
1404ec9ea: 4c897c2420 mov qword ptr [rsp + 0x20], r15
1404ec9ef: 4c8d7df4 lea r15, [rbp - 0xc]
1404ec9f3: 4c89f9 mov rcx, r15
1404ec9f6: 4889da mov rdx, rbx
1404ec9f9: 4989f8 mov r8, rdi
1404ec9fc: 4c8d8d40020000 lea r9, [rbp + 0x240]
1404eca03: e868d45400 call 0x140a39e70
1404eca08: 0fb685d4030000 movzx eax, byte ptr [rbp + 0x3d4]
1404eca0f: 4c89642460 mov qword ptr [rsp + 0x60], r12
1404eca14: 4989f4 mov r12, rsi
1404eca17: 488db5b0030000 lea rsi, [rbp + 0x3b0]
1404eca1e: 4c896c2458 mov qword ptr [rsp + 0x58], r13
1404eca23: 4c897c2450 mov qword ptr [rsp + 0x50], r15
1404eca28: 488d4d3c lea rcx, [rbp + 0x3c]
1404eca2c: 48894c2448 mov qword ptr [rsp + 0x48], rcx
1404eca31: 4c89742440 mov qword ptr [rsp + 0x40], r14
1404eca36: 4c89642438 mov qword ptr [rsp + 0x38], r12
1404eca3b: 488d8d8c000000 lea rcx, [rbp + 0x8c]
1404eca42: 48894c2430 mov qword ptr [rsp + 0x30], rcx
1404eca47: 88442428 mov byte ptr [rsp + 0x28], al
1404eca4b: 4889742420 mov qword ptr [rsp + 0x20], rsi
1404eca50: 4889d9 mov rcx, rbx
1404eca53: 4889fa mov rdx, rdi
1404eca56: 4c8d8540020000 lea r8, [rbp + 0x240]
1404eca5d: 4c8badf8040000 mov r13, qword ptr [rbp + 0x4f8]
1404eca64: 4d89e9 mov r9, r13
1404eca67: e844564e00 call 0x1409d20b0
1404eca6c: 488b0525f61303 mov rax, qword ptr [rip + 0x313f625]
1404eca73: 803800 cmp byte ptr [rax], 0
1404eca76: 0f851d010000 jne 0x1404ecb99
1404eca7c: 498d4d04 lea rcx, [r13 + 4]
1404eca80: e80b8d6e00 call 0x140bd5790
1404eca85: 488d8de0020000 lea rcx, [rbp + 0x2e0]
1404eca8c: 4889c2 mov rdx, rax
1404eca8f: 41b87a000000 mov r8d, 0x7a
1404eca95: e806886e00 call 0x140bd52a0
1404eca9a: 0f1085e0020000 movups xmm0, xmmword ptr [rbp + 0x2e0]
1404ecaa1: 0f108df0020000 movups xmm1, xmmword ptr [rbp + 0x2f0]
1404ecaa8: 0f298df0030000 movaps xmmword ptr [rbp + 0x3f0], xmm1
1404ecaaf: 0f2985e0030000 movaps xmmword ptr [rbp + 0x3e0], xmm0
1404ecab6: 488d8d20040000 lea rcx, [rbp + 0x420]
1404ecabd: 488d95e0030000 lea rdx, [rbp + 0x3e0]
1404ecac4: e817a26e00 call 0x140bd6ce0
1404ecac9: 488d0558f54f02 lea rax, [rip + 0x24ff558]
1404ecad0: 48898510040000 mov qword ptr [rbp + 0x410], rax
1404ecad7: 48c7851804000011000000 mov qword ptr [rbp + 0x418], 0x11
1404ecae2: 488db510040000 lea rsi, [rbp + 0x410]
1404ecae9: 4889d9 mov rcx, rbx
1404ecaec: 4889f2 mov rdx, rsi
1404ecaef: e8dcc96f00 call 0x140be94d0
1404ecaf4: 0f1085e0020000 movups xmm0, xmmword ptr [rbp + 0x2e0]
1404ecafb: 0f108df0020000 movups xmm1, xmmword ptr [rbp + 0x2f0]
1404ecb02: 0f118514040000 movups xmmword ptr [rbp + 0x414], xmm0
1404ecb09: 0f118d24040000 movups xmmword ptr [rbp + 0x424], xmm1
1404ecb10: 898510040000 mov dword ptr [rbp + 0x410], eax
1404ecb16: 4c8bb560040000 mov r14, qword ptr [rbp + 0x460]
1404ecb1d: 4c89742428 mov qword ptr [rsp + 0x28], r14
1404ecb22: 4c8dbd64020000 lea r15, [rbp + 0x264]
1404ecb29: 4c897c2420 mov qword ptr [rsp + 0x20], r15
1404ecb2e: 4889d9 mov rcx, rbx
1404ecb31: 4889fa mov rdx, rdi
1404ecb34: 4989f0 mov r8, rsi
1404ecb37: 4d89e9 mov r9, r13
1404ecb3a: e8e1274d00 call 0x1409bf320
1404ecb3f: 488d058a3d4d00 lea rax, [rip + 0x4d3d8a]
1404ecb46: 4c8d15b32c4d00 lea r10, [rip + 0x4d2cb3]
1404ecb4d: 80bd6f04000000 cmp byte ptr [rbp + 0x46f], 0
1404ecb54: 4c0f45d0 cmovne r10, rax
1404ecb58: 0fb685d4030000 movzx eax, byte ptr [rbp + 0x3d4]
1404ecb5f: 4c89742448 mov qword ptr [rsp + 0x48], r14
1404ecb64: 488d8d8c010000 lea rcx, [rbp + 0x18c]
1404ecb6b: 48894c2440 mov qword ptr [rsp + 0x40], rcx
1404ecb70: 4889742438 mov qword ptr [rsp + 0x38], rsi
1404ecb75: 488db5b0030000 lea rsi, [rbp + 0x3b0]
1404ecb7c: 88442430 mov byte ptr [rsp + 0x30], al
1404ecb80: 4889742428 mov qword ptr [rsp + 0x28], rsi
1404ecb85: 4c89642420 mov qword ptr [rsp + 0x20], r12
1404ecb8a: 4889d9 mov rcx, rbx
1404ecb8d: 4889fa mov rdx, rdi
1404ecb90: 4d89e8 mov r8, r13
1404ecb93: 4d89f9 mov r9, r15
1404ecb96: 41ffd2 call r10
1404ecb99: 0fb685d4030000 movzx eax, byte ptr [rbp + 0x3d4]
1404ecba0: 488b8d60040000 mov rcx, qword ptr [rbp + 0x460]
1404ecba7: 48894c2438 mov qword ptr [rsp + 0x38], rcx
1404ecbac: 88442430 mov byte ptr [rsp + 0x30], al
1404ecbb0: 4889742428 mov qword ptr [rsp + 0x28], rsi
1404ecbb5: 4c89642420 mov qword ptr [rsp + 0x20], r12
1404ecbba: 4889d9 mov rcx, rbx
1404ecbbd: 4889fa mov rdx, rdi
1404ecbc0: 4d89e8 mov r8, r13
1404ecbc3: 488db564020000 lea rsi, [rbp + 0x264]
1404ecbca: 4989f1 mov r9, rsi
1404ecbcd: e8ae6e4d00 call 0x1409c3a80
1404ecbd2: 4889d9 mov rcx, rbx
1404ecbd5: 4889fa mov rdx, rdi
1404ecbd8: e813db5400 call 0x140a3a6f0
1404ecbdd: 8b4620 mov eax, dword ptr [rsi + 0x20]
1404ecbe0: 898530020000 mov dword ptr [rbp + 0x230], eax
1404ecbe6: 0f1006 movups xmm0, xmmword ptr [rsi]
1404ecbe9: 0f104e10 movups xmm1, xmmword ptr [rsi + 0x10]
1404ecbed: 0f298d20020000 movaps xmmword ptr [rbp + 0x220], xmm1
1404ecbf4: 0f298510020000 movaps xmmword ptr [rbp + 0x210], xmm0
1404ecbfb: 4c8db510020000 lea r14, [rbp + 0x210]
1404ecc02: 4889d9 mov rcx, rbx
1404ecc05: 4889fa mov rdx, rdi
1404ecc08: 4d89e8 mov r8, r13
1404ecc0b: 4d89f1 mov r9, r14
1404ecc0e: e8ad783b00 call 0x1408a44c0
1404ecc13: 4c8dbd10040000 lea r15, [rbp + 0x410]
1404ecc1a: 4c89f9 mov rcx, r15
1404ecc1d: 4889da mov rdx, rbx
1404ecc20: 4989f8 mov r8, rdi
1404ecc23: 4d89f1 mov r9, r14
1404ecc26: e8f5d35600 call 0x140a5a020
1404ecc2b: 48ba28641ac941e9b8c2 movabs rdx, 0xc2b8e941c91a6428
1404ecc35: 49b8a043dad3d1c6f359 movabs r8, 0x59f3c6d1d3da43a0
1404ecc3f: 488b8dd8030000 mov rcx, qword ptr [rbp + 0x3d8]
1404ecc46: e8a50abfff call 0x1400dd6f0
1404ecc4b: 4885c0 test rax, rax
1404ecc4e: 4c8ba5b8020000 mov r12, qword ptr [rbp + 0x2b8]
1404ecc55: 0f846c040000 je 0x1404ed0c7
1404ecc5b: 48399718020000 cmp qword ptr [rdi + 0x218], rdx
1404ecc62: 0f865f040000 jbe 0x1404ed0c7
1404ecc68: 488b8710020000 mov rax, qword ptr [rdi + 0x210]
1404ecc6f: 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
1404ecc73: 4885c9 test rcx, rcx
1404ecc76: 0f844b040000 je 0x1404ed0c7
1404ecc7c: 488b87e0010000 mov rax, qword ptr [rdi + 0x1e0]
1404ecc83: 48f7d1 not rcx
1404ecc86: 488d0c49 lea rcx, [rcx + rcx*2]
1404ecc8a: 48c1e105 shl rcx, 5
1404ecc8e: 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
1404ecc94: 0f842d040000 je 0x1404ed0c7
1404ecc9a: 4801c8 add rax, rcx
1404ecc9d: 488b4010 mov rax, qword ptr [rax + 0x10]
1404ecca1: 8b30 mov esi, dword ptr [rax]
1404ecca3: 31c0 xor eax, eax
1404ecca5: 83fe13 cmp esi, 0x13
1404ecca8: 0f94c0 sete al
1404eccab: 488b8de8040000 mov rcx, qword ptr [rbp + 0x4e8]
1404eccb2: 80bc08a900000000 cmp byte ptr [rax + rcx + 0xa9], 0
1404eccba: 0f847e010000 je 0x1404ece3e
1404eccc0: 4d8d4504 lea r8, [r13 + 4]
1404eccc4: 488d058df34f02 lea rax, [rip + 0x24ff38d]
1404ecccb: 4889442420 mov qword ptr [rsp + 0x20], rax
1404eccd0: 48c744242808000000 mov qword ptr [rsp + 0x28], 8
1404eccd9: 4889d9 mov rcx, rbx
1404eccdc: 4889fa mov rdx, rdi
1404eccdf: 4d89e9 mov r9, r13
1404ecce2: e8a9ebc0ff call 0x1400fb890
1404ecce7: 83fe13 cmp esi, 0x13
1404eccea: 0f94442430 sete byte ptr [rsp + 0x30]
1404eccef: 4c897c2428 mov qword ptr [rsp + 0x28], r15
1404eccf4: 4c89742420 mov qword ptr [rsp + 0x20], r14
1404eccf9: 488d8de0020000 lea rcx, [rbp + 0x2e0]
1404ecd00: 4889da mov rdx, rbx
1404ecd03: 4989f8 mov r8, rdi
1404ecd06: 4d89e9 mov r9, r13
1404ecd09: e862d85600 call 0x140a5a570
1404ecd0e: 80bd6e04000000 cmp byte ptr [rbp + 0x46e], 0
1404ecd15: 0f8552010000 jne 0x1404ece6d
1404ecd1b: 488d8de4020000 lea rcx, [rbp + 0x2e4]
1404ecd22: e8698a6e00 call 0x140bd5790
1404ecd27: 488d8dc0020000 lea rcx, [rbp + 0x2c0]
1404ecd2e: 4889c2 mov rdx, rax
1404ecd31: 41b87a000000 mov r8d, 0x7a
1404ecd37: e864856e00 call 0x140bd52a0
1404ecd3c: 0f1085c0020000 movups xmm0, xmmword ptr [rbp + 0x2c0]
1404ecd43: 0f108dd0020000 movups xmm1, xmmword ptr [rbp + 0x2d0]
1404ecd4a: 0f298d20030000 movaps xmmword ptr [rbp + 0x320], xmm1
1404ecd51: 0f298510030000 movaps xmmword ptr [rbp + 0x310], xmm0
1404ecd58: 488d8df0030000 lea rcx, [rbp + 0x3f0]
1404ecd5f: 488d9510030000 lea rdx, [rbp + 0x310]
1404ecd66: e8759f6e00 call 0x140bd6ce0
1404ecd6b: 488d05eef24f02 lea rax, [rip + 0x24ff2ee]
1404ecd72: 488985e0030000 mov qword ptr [rbp + 0x3e0], rax
1404ecd79: 48c785e80300000a000000 mov qword ptr [rbp + 0x3e8], 0xa
1404ecd84: 488db5e0030000 lea rsi, [rbp + 0x3e0]
1404ecd8b: 4889d9 mov rcx, rbx
1404ecd8e: 4889f2 mov rdx, rsi
1404ecd91: e83ac76f00 call 0x140be94d0
1404ecd96: 0f1085c0020000 movups xmm0, xmmword ptr [rbp + 0x2c0]
1404ecd9d: 0f108dd0020000 movups xmm1, xmmword ptr [rbp + 0x2d0]
1404ecda4: 0f118540010000 movups xmmword ptr [rbp + 0x140], xmm0
1404ecdab: 0f118d50010000 movups xmmword ptr [rbp + 0x150], xmm1
1404ecdb2: 89853c010000 mov dword ptr [rbp + 0x13c], eax
1404ecdb8: 488b85e8040000 mov rax, qword ptr [rbp + 0x4e8]
1404ecdbf: f30f10809c000000 movss xmm0, dword ptr [rax + 0x9c]
1404ecdc7: 488b85d0030000 mov rax, qword ptr [rbp + 0x3d0]
1404ecdce: 48898500040000 mov qword ptr [rbp + 0x400], rax
1404ecdd5: 0f288db0030000 movaps xmm1, xmmword ptr [rbp + 0x3b0]
1404ecddc: 0f2895c0030000 movaps xmm2, xmmword ptr [rbp + 0x3c0]
1404ecde3: 0f2995f0030000 movaps xmmword ptr [rbp + 0x3f0], xmm2
1404ecdea: 0f298de0030000 movaps xmmword ptr [rbp + 0x3e0], xmm1
1404ecdf1: f30f11442440 movss dword ptr [rsp + 0x40], xmm0
1404ecdf7: 4889742438 mov qword ptr [rsp + 0x38], rsi
1404ecdfc: 488d85b0010000 lea rax, [rbp + 0x1b0]
1404ece03: 4889442430 mov qword ptr [rsp + 0x30], rax
1404ece08: 488d858c010000 lea rax, [rbp + 0x18c]
1404ece0f: 4889442428 mov qword ptr [rsp + 0x28], rax
1404ece14: 488d853c010000 lea rax, [rbp + 0x13c]
1404ece1b: 4889442420 mov qword ptr [rsp + 0x20], rax
1404ece20: 488d8d10030000 lea rcx, [rbp + 0x310]
1404ece27: 4c8d8de0020000 lea r9, [rbp + 0x2e0]
1404ece2e: 4889da mov rdx, rbx
1404ece31: 4989f8 mov r8, rdi
1404ece34: e8b7e45400 call 0x140a3b2f0
1404ece39: e9b6010000 jmp 0x1404ecff4
1404ece3e: 418b4520 mov eax, dword ptr [r13 + 0x20]
1404ece42: 898500030000 mov dword ptr [rbp + 0x300], eax
1404ece48: 410f104500 movups xmm0, xmmword ptr [r13]
1404ece4d: 410f104d10 movups xmm1, xmmword ptr [r13 + 0x10]
1404ece52: 0f298df0020000 movaps xmmword ptr [rbp + 0x2f0], xmm1
1404ece59: 0f2985e0020000 movaps xmmword ptr [rbp + 0x2e0], xmm0
1404ece60: 80bd6e04000000 cmp byte ptr [rbp + 0x46e], 0
1404ece67: 0f84aefeffff je 0x1404ecd1b
1404ece6d: 48ba8b047be66126c0a4 movabs rdx, 0xa4c02661e67b048b
1404ece77: 49b8ec672aaae661e7ea movabs r8, 0xeae761e6aa2a67ec
1404ece81: 488b8dd8030000 mov rcx, qword ptr [rbp + 0x3d8]
1404ece88: e86308bfff call 0x1400dd6f0
1404ece8d: 4885c0 test rax, rax
1404ece90: 0f8449020000 je 0x1404ed0df
1404ece96: 48399718020000 cmp qword ptr [rdi + 0x218], rdx
1404ece9d: 0f863c020000 jbe 0x1404ed0df
1404ecea3: 488b8710020000 mov rax, qword ptr [rdi + 0x210]
1404eceaa: 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
1404eceae: 4885c9 test rcx, rcx
1404eceb1: 0f8428020000 je 0x1404ed0df
1404eceb7: 488b87e0010000 mov rax, qword ptr [rdi + 0x1e0]
1404ecebe: 48f7d1 not rcx
1404ecec1: 488d0c49 lea rcx, [rcx + rcx*2]
1404ecec5: 48c1e105 shl rcx, 5
1404ecec9: 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
1404ececf: 0f840a020000 je 0x1404ed0df
1404eced5: 4801c8 add rax, rcx
1404eced8: 488b4010 mov rax, qword ptr [rax + 0x10]
1404ecedc: 488b5008 mov rdx, qword ptr [rax + 8]
1404ecee0: 488d8dc0020000 lea rcx, [rbp + 0x2c0]
1404ecee7: 41b87a000000 mov r8d, 0x7a
1404eceed: e8ae836e00 call 0x140bd52a0
1404ecef2: 0f1085c0020000 movups xmm0, xmmword ptr [rbp + 0x2c0]
1404ecef9: 0f108dd0020000 movups xmm1, xmmword ptr [rbp + 0x2d0]
1404ecf00: 0f298d20030000 movaps xmmword ptr [rbp + 0x320], xmm1
1404ecf07: 0f298510030000 movaps xmmword ptr [rbp + 0x310], xmm0
1404ecf0e: 488d8df0030000 lea rcx, [rbp + 0x3f0]
1404ecf15: 488d9510030000 lea rdx, [rbp + 0x310]
1404ecf1c: e8bf9d6e00 call 0x140bd6ce0
1404ecf21: 488d0538f14f02 lea rax, [rip + 0x24ff138]
1404ecf28: 488985e0030000 mov qword ptr [rbp + 0x3e0], rax
1404ecf2f: 48c785e80300000a000000 mov qword ptr [rbp + 0x3e8], 0xa
1404ecf3a: 488db5e0030000 lea rsi, [rbp + 0x3e0]
1404ecf41: 4889d9 mov rcx, rbx
1404ecf44: 4889f2 mov rdx, rsi
1404ecf47: e884c56f00 call 0x140be94d0
1404ecf4c: 0f1085c0020000 movups xmm0, xmmword ptr [rbp + 0x2c0]
1404ecf53: 0f108dd0020000 movups xmm1, xmmword ptr [rbp + 0x2d0]
1404ecf5a: 0f11851c010000 movups xmmword ptr [rbp + 0x11c], xmm0
1404ecf61: 0f118d2c010000 movups xmmword ptr [rbp + 0x12c], xmm1
1404ecf68: 898518010000 mov dword ptr [rbp + 0x118], eax
1404ecf6e: 488b85e8040000 mov rax, qword ptr [rbp + 0x4e8]
1404ecf75: f30f10809c000000 movss xmm0, dword ptr [rax + 0x9c]
1404ecf7d: 488b85d0030000 mov rax, qword ptr [rbp + 0x3d0]
1404ecf84: 48898500040000 mov qword ptr [rbp + 0x400], rax
1404ecf8b: 0f288db0030000 movaps xmm1, xmmword ptr [rbp + 0x3b0]
1404ecf92: 0f2895c0030000 movaps xmm2, xmmword ptr [rbp + 0x3c0]
1404ecf99: 0f2995f0030000 movaps xmmword ptr [rbp + 0x3f0], xmm2
1404ecfa0: 0f298de0030000 movaps xmmword ptr [rbp + 0x3e0], xmm1
1404ecfa7: f30f11442448 movss dword ptr [rsp + 0x48], xmm0
1404ecfad: 4889742440 mov qword ptr [rsp + 0x40], rsi
1404ecfb2: 488d85b0010000 lea rax, [rbp + 0x1b0]
1404ecfb9: 4889442438 mov qword ptr [rsp + 0x38], rax
1404ecfbe: 488d858c010000 lea rax, [rbp + 0x18c]
1404ecfc5: 4889442430 mov qword ptr [rsp + 0x30], rax
1404ecfca: 488d8518010000 lea rax, [rbp + 0x118]
1404ecfd1: 4889442428 mov qword ptr [rsp + 0x28], rax
1404ecfd6: 4c89742420 mov qword ptr [rsp + 0x20], r14
1404ecfdb: 488d8d10030000 lea rcx, [rbp + 0x310]
1404ecfe2: 4c8d8de0020000 lea r9, [rbp + 0x2e0]
1404ecfe9: 4889da mov rdx, rbx
1404ecfec: 4989f8 mov r8, rdi
1404ecfef: e86cea5400 call 0x140a3ba60
1404ecff4: 488db510030000 lea rsi, [rbp + 0x310]
1404ecffb: 4889d9 mov rcx, rbx
1404ecffe: 4889fa mov rdx, rdi
1404ed001: 4989f0 mov r8, rsi
1404ed004: 4c8b8d60040000 mov r9, qword ptr [rbp + 0x460]
1404ed00b: e8a0704d00 call 0x1409c40b0
1404ed010: 4889d9 mov rcx, rbx
1404ed013: 4889fa mov rdx, rdi
1404ed016: 4989f0 mov r8, rsi
1404ed019: e8d2294a00 call 0x14098f9f0
1404ed01e: 8b8530040000 mov eax, dword ptr [rbp + 0x430]
1404ed024: 4189442444 mov dword ptr [r12 + 0x44], eax
1404ed029: 0f108510040000 movups xmm0, xmmword ptr [rbp + 0x410]
1404ed030: 0f108d20040000 movups xmm1, xmmword ptr [rbp + 0x420]
1404ed037: 410f114c2434 movups xmmword ptr [r12 + 0x34], xmm1
1404ed03d: 410f11442424 movups xmmword ptr [r12 + 0x24], xmm0
1404ed043: 0f108510030000 movups xmm0, xmmword ptr [rbp + 0x310]
1404ed04a: 0f108d20030000 movups xmm1, xmmword ptr [rbp + 0x320]
1404ed051: 410f11442448 movups xmmword ptr [r12 + 0x48], xmm0
1404ed057: 410f114c2458 movups xmmword ptr [r12 + 0x58], xmm1
1404ed05d: 8b8530030000 mov eax, dword ptr [rbp + 0x330]
1404ed063: 4189442468 mov dword ptr [r12 + 0x68], eax
1404ed068: 0f288510020000 movaps xmm0, xmmword ptr [rbp + 0x210]
1404ed06f: 0f288d20020000 movaps xmm1, xmmword ptr [rbp + 0x220]
1404ed076: 410f110424 movups xmmword ptr [r12], xmm0
1404ed07b: 410f114c2410 movups xmmword ptr [r12 + 0x10], xmm1
1404ed081: 8b8530020000 mov eax, dword ptr [rbp + 0x230]
1404ed087: 4189442420 mov dword ptr [r12 + 0x20], eax
1404ed08c: 4c89e0 mov rax, r12
1404ed08f: 4881c4f8040000 add rsp, 0x4f8
1404ed096: 5b pop rbx
1404ed097: 5f pop rdi
1404ed098: 5e pop rsi
1404ed099: 415c pop r12
1404ed09b: 415d pop r13
1404ed09d: 415e pop r14
1404ed09f: 415f pop r15
1404ed0a1: 5d pop rbp
1404ed0a2: c3 ret 
1404ed0a3: 488d0d76ee4f02 lea rcx, [rip + 0x24fee76]
1404ed0aa: 4c8d05b7ee4f02 lea r8, [rip + 0x24feeb7]
1404ed0b1: ba46000000 mov edx, 0x46
1404ed0b6: e875c33e02 call 0x1428d9430
1404ed0bb: 488d0dbeee4f02 lea rcx, [rip + 0x24feebe]
1404ed0c2: e849c23e02 call 0x1428d9310
1404ed0c7: 488d0d72ef4f02 lea rcx, [rip + 0x24fef72]
1404ed0ce: e83dc23e02 call 0x1428d9310
1404ed0d3: 488d0d0eef4f02 lea rcx, [rip + 0x24fef0e]
1404ed0da: e831c23e02 call 0x1428d9310
1404ed0df: 488d0d8aef4f02 lea rcx, [rip + 0x24fef8a]
1404ed0e6: e825c23e02 call 0x1428d9310
1404ed0eb: 488b8540030000 mov rax, qword ptr [rbp + 0x340]
1404ed0f2: 48898540020000 mov qword ptr [rbp + 0x240], rax
1404ed0f9: 488d05b0ee4f02 lea rax, [rip + 0x24feeb0]
1404ed100: 4889442420 mov qword ptr [rsp + 0x20], rax
1404ed105: 488d0d94e74f02 lea rcx, [rip + 0x24fe794]
1404ed10c: 4c8d0d6de74f02 lea r9, [rip + 0x24fe76d]
1404ed113: 4c8d8540020000 lea r8, [rbp + 0x240]
1404ed11a: ba2b000000 mov edx, 0x2b
1404ed11f: e83cc63e02 call 0x1428d9760
1404ed124: eb39 jmp 0x1404ed15f
1404ed126: 488b8548030000 mov rax, qword ptr [rbp + 0x348]
1404ed12d: 48898510040000 mov qword ptr [rbp + 0x410], rax
1404ed134: 488d05d5ee4f02 lea rax, [rip + 0x24feed5]
1404ed13b: 4889442420 mov qword ptr [rsp + 0x20], rax
1404ed140: 488d0d59e74f02 lea rcx, [rip + 0x24fe759]
1404ed147: 4c8d0d32e74f02 lea r9, [rip + 0x24fe732]
1404ed14e: 4c8d8510040000 lea r8, [rbp + 0x410]
1404ed155: ba2b000000 mov edx, 0x2b
1404ed15a: e801c63e02 call 0x1428d9760
1404ed15f: 0f0b ud2 
1404ed161: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1404ed170: 4889542410 mov qword ptr [rsp + 0x10], rdx
1404ed175: 55 push rbp
1404ed176: 4157 push r15
1404ed178: 4156 push r14
1404ed17a: 4155 push r13
1404ed17c: 4154 push r12
1404ed17e: 56 push rsi
1404ed17f: 57 push rdi
1404ed180: 53 push rbx
1404ed181: 4883ec68 sub rsp, 0x68
1404ed185: 488daa80000000 lea rbp, [rdx + 0x80]
1404ed18c: 488d8d40020000 lea rcx, [rbp + 0x240]
1404ed193: e898fa8500 call 0x140d4cc30
1404ed198: 90 nop 
1404ed199: 4883c468 add rsp, 0x68
1404ed19d: 5b pop rbx
1404ed19e: 5f pop rdi
1404ed19f: 5e pop rsi
1404ed1a0: 415c pop r12
1404ed1a2: 415d pop r13
1404ed1a4: 415e pop r14
1404ed1a6: 415f pop r15
1404ed1a8: 5d pop rbp
1404ed1a9: c3 ret 
1404ed1aa: 660f1f440000 nop word ptr [rax + rax]
1404ed1b0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1404ed1b5: 55 push rbp
1404ed1b6: 4157 push r15
1404ed1b8: 4156 push r14
1404ed1ba: 4155 push r13
1404ed1bc: 4154 push r12
1404ed1be: 56 push rsi
1404ed1bf: 57 push rdi
1404ed1c0: 53 push rbx
1404ed1c1: 4883ec68 sub rsp, 0x68
1404ed1c5: 488daa80000000 lea rbp, [rdx + 0x80]
1404ed1cc: 488d8d10040000 lea rcx, [rbp + 0x410]
1404ed1d3: e858fa8500 call 0x140d4cc30
1404ed1d8: 90 nop 
1404ed1d9: 4883c468 add rsp, 0x68
1404ed1dd: 5b pop rbx
1404ed1de: 5f pop rdi
1404ed1df: 5e pop rsi
1404ed1e0: 415c pop r12
1404ed1e2: 415d pop r13
1404ed1e4: 415e pop r14
1404ed1e6: 415f pop r15
1404ed1e8: 5d pop rbp
1404ed1e9: c3 ret 
