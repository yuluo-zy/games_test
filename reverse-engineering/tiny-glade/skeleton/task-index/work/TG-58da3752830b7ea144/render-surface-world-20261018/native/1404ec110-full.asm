
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1404ec110: 55                          	push	rbp
1404ec111: 41 57                       	push	r15
1404ec113: 41 56                       	push	r14
1404ec115: 41 55                       	push	r13
1404ec117: 41 54                       	push	r12
1404ec119: 56                          	push	rsi
1404ec11a: 57                          	push	rdi
1404ec11b: 53                          	push	rbx
1404ec11c: 48 81 ec f8 04 00 00        	sub	rsp, 0x4f8
1404ec123: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
1404ec12b: 48 c7 85 70 04 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x470], -0x2
1404ec136: 4c 89 8d 60 04 00 00        	mov	qword ptr [rbp + 0x460], r9
1404ec13d: 4c 8b bd e0 04 00 00        	mov	r15, qword ptr [rbp + 0x4e0]
1404ec144: 4c 89 f8                    	mov	rax, r15
1404ec147: 48 c1 e8 20                 	shr	rax, 0x20
1404ec14b: 0f 84 52 0f 00 00           	je	0x1404ed0a3 <.text+0x4ec0a3>
1404ec151: 44 89 f8                    	mov	eax, r15d
1404ec154: 48 85 c0                    	test	rax, rax
1404ec157: 0f 84 46 0f 00 00           	je	0x1404ed0a3 <.text+0x4ec0a3>
1404ec15d: 4c 89 c7                    	mov	rdi, r8
1404ec160: 48 89 d3                    	mov	rbx, rdx
1404ec163: 48 89 ce                    	mov	rsi, rcx
1404ec166: 49 8d 48 38                 	lea	rcx, [r8 + 0x38]
1404ec16a: 48 ba 9b 71 d8 c6 fc 0d 43 74       	movabs	rdx, 0x74430dfcc6d8719b
1404ec174: 49 b8 9c 50 64 e4 f7 91 55 ca       	movabs	r8, -0x35aa6e081b9baf64
1404ec17e: 48 89 8d d8 03 00 00        	mov	qword ptr [rbp + 0x3d8], rcx
1404ec185: e8 66 15 bf ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1404ec18a: 48 85 c0                    	test	rax, rax
1404ec18d: 0f 84 28 0f 00 00           	je	0x1404ed0bb <.text+0x4ec0bb>
1404ec193: 48 39 97 18 02 00 00        	cmp	qword ptr [rdi + 0x218], rdx
1404ec19a: 0f 86 1b 0f 00 00           	jbe	0x1404ed0bb <.text+0x4ec0bb>
1404ec1a0: 48 8b 87 10 02 00 00        	mov	rax, qword ptr [rdi + 0x210]
1404ec1a7: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
1404ec1ab: 48 85 c9                    	test	rcx, rcx
1404ec1ae: 0f 84 07 0f 00 00           	je	0x1404ed0bb <.text+0x4ec0bb>
1404ec1b4: 48 8b 87 e0 01 00 00        	mov	rax, qword ptr [rdi + 0x1e0]
1404ec1bb: 48 f7 d1                    	not	rcx
1404ec1be: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1404ec1c2: 48 c1 e1 05                 	shl	rcx, 0x5
1404ec1c6: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
1404ec1cc: 0f 84 e9 0e 00 00           	je	0x1404ed0bb <.text+0x4ec0bb>
1404ec1d2: 48 89 b5 b8 02 00 00        	mov	qword ptr [rbp + 0x2b8], rsi
1404ec1d9: 4c 8b a5 f0 04 00 00        	mov	r12, qword ptr [rbp + 0x4f0]
1404ec1e0: 48 01 c8                    	add	rax, rcx
1404ec1e3: 48 8b 48 10                 	mov	rcx, qword ptr [rax + 0x10]
1404ec1e7: 48 83 c1 58                 	add	rcx, 0x58
1404ec1eb: e8 30 c1 5f 00              	call	0x140ae8320 <.text+0xae7320>
1404ec1f0: 88 85 6f 04 00 00           	mov	byte ptr [rbp + 0x46f], al
1404ec1f6: 41 0f 10 44 24 04           	movups	xmm0, xmmword ptr [r12 + 0x4]
1404ec1fc: 41 0f 10 4c 24 14           	movups	xmm1, xmmword ptr [r12 + 0x14]
1404ec202: 0f 29 8d 50 03 00 00        	movaps	xmmword ptr [rbp + 0x350], xmm1
1404ec209: 0f 29 85 40 03 00 00        	movaps	xmmword ptr [rbp + 0x340], xmm0
1404ec210: 48 8d b5 40 02 00 00        	lea	rsi, [rbp + 0x240]
1404ec217: 48 8d 95 40 03 00 00        	lea	rdx, [rbp + 0x340]
1404ec21e: 48 89 f1                    	mov	rcx, rsi
1404ec221: 41 b8 64 00 00 00           	mov	r8d, 0x64
1404ec227: e8 04 92 6e 00              	call	0x140bd5430 <.text+0xbd4430>
1404ec22c: 4c 8d b5 10 04 00 00        	lea	r14, [rbp + 0x410]
1404ec233: 4c 89 f1                    	mov	rcx, r14
1404ec236: 48 89 f2                    	mov	rdx, rsi
1404ec239: 41 b8 0c 00 00 00           	mov	r8d, 0xc
1404ec23f: e8 fc 90 6e 00              	call	0x140bd5340 <.text+0xbd4340>
1404ec244: 4c 89 74 24 20              	mov	qword ptr [rsp + 0x20], r14
1404ec249: 4c 8d 05 48 fd 4f 02        	lea	r8, [rip + 0x24ffd48]   # 0x1429ebf98
1404ec250: 48 8d 8d 40 03 00 00        	lea	rcx, [rbp + 0x340]
1404ec257: 41 b9 17 00 00 00           	mov	r9d, 0x17
1404ec25d: 48 89 da                    	mov	rdx, rbx
1404ec260: e8 eb ab 00 00              	call	0x1404f6e50 <.text+0x4f5e50>
1404ec265: 80 bd 64 03 00 00 02        	cmp	byte ptr [rbp + 0x364], 0x2
1404ec26c: 0f 84 79 0e 00 00           	je	0x1404ed0eb <.text+0x4ec0eb>
1404ec272: 48 8b 85 60 03 00 00        	mov	rax, qword ptr [rbp + 0x360]
1404ec279: 48 89 85 d0 00 00 00        	mov	qword ptr [rbp + 0xd0], rax
1404ec280: 0f 10 85 40 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x340]
1404ec287: 0f 10 8d 50 03 00 00        	movups	xmm1, xmmword ptr [rbp + 0x350]
1404ec28e: 0f 29 8d c0 00 00 00        	movaps	xmmword ptr [rbp + 0xc0], xmm1
1404ec295: 0f 29 85 b0 00 00 00        	movaps	xmmword ptr [rbp + 0xb0], xmm0
1404ec29c: 48 89 d9                    	mov	rcx, rbx
1404ec29f: 48 89 fa                    	mov	rdx, rdi
1404ec2a2: 41 b0 01                    	mov	r8b, 0x1
1404ec2a5: e8 26 c9 ee ff              	call	0x1403d8bd0 <.text+0x3d7bd0>
1404ec2aa: 48 89 d9                    	mov	rcx, rbx
1404ec2ad: 48 89 fa                    	mov	rdx, rdi
1404ec2b0: 41 b0 02                    	mov	r8b, 0x2
1404ec2b3: e8 98 dd ee ff              	call	0x1403da050 <.text+0x3d9050>
1404ec2b8: 48 89 d9                    	mov	rcx, rbx
1404ec2bb: 48 89 fa                    	mov	rdx, rdi
1404ec2be: 41 b0 03                    	mov	r8b, 0x3
1404ec2c1: e8 aa e2 ee ff              	call	0x1403da570 <.text+0x3d9570>
1404ec2c6: 48 89 d9                    	mov	rcx, rbx
1404ec2c9: 48 89 fa                    	mov	rdx, rdi
1404ec2cc: 41 b0 04                    	mov	r8b, 0x4
1404ec2cf: e8 1c ce ee ff              	call	0x1403d90f0 <.text+0x3d80f0>
1404ec2d4: 48 89 d9                    	mov	rcx, rbx
1404ec2d7: 48 89 fa                    	mov	rdx, rdi
1404ec2da: 41 b0 13                    	mov	r8b, 0x13
1404ec2dd: e8 2e d3 ee ff              	call	0x1403d9610 <.text+0x3d8610>
1404ec2e2: 48 89 d9                    	mov	rcx, rbx
1404ec2e5: 48 89 fa                    	mov	rdx, rdi
1404ec2e8: 41 b0 14                    	mov	r8b, 0x14
1404ec2eb: e8 40 d8 ee ff              	call	0x1403d9b30 <.text+0x3d8b30>
1404ec2f0: 48 89 d9                    	mov	rcx, rbx
1404ec2f3: 48 89 fa                    	mov	rdx, rdi
1404ec2f6: e8 85 63 52 00              	call	0x140a12680 <.text+0xa11680>
1404ec2fb: 48 89 d9                    	mov	rcx, rbx
1404ec2fe: 48 89 fa                    	mov	rdx, rdi
1404ec301: e8 1a ae 3d 00              	call	0x1408c7120 <.text+0x8c6120>
1404ec306: 48 89 d9                    	mov	rcx, rbx
1404ec309: 48 89 fa                    	mov	rdx, rdi
1404ec30c: e8 6f 2b 55 00              	call	0x140a3ee80 <.text+0xa3de80>
1404ec311: 48 89 d9                    	mov	rcx, rbx
1404ec314: 48 89 fa                    	mov	rdx, rdi
1404ec317: e8 64 03 4a 00              	call	0x14098c680 <.text+0x98b680>
1404ec31c: 48 89 d9                    	mov	rcx, rbx
1404ec31f: 48 89 fa                    	mov	rdx, rdi
1404ec322: e8 89 a5 48 00              	call	0x1409768b0 <.text+0x9758b0>
1404ec327: 48 89 d9                    	mov	rcx, rbx
1404ec32a: 48 89 fa                    	mov	rdx, rdi
1404ec32d: e8 7e f4 4d 00              	call	0x1409cb7b0 <.text+0x9ca7b0>
1404ec332: 48 89 d9                    	mov	rcx, rbx
1404ec335: 48 89 fa                    	mov	rdx, rdi
1404ec338: e8 13 c1 56 00              	call	0x140a58450 <.text+0xa57450>
1404ec33d: 48 89 d9                    	mov	rcx, rbx
1404ec340: 48 89 fa                    	mov	rdx, rdi
1404ec343: e8 58 e7 4e 00              	call	0x1409daaa0 <.text+0x9d9aa0>
1404ec348: 48 89 d9                    	mov	rcx, rbx
1404ec34b: 48 89 fa                    	mov	rdx, rdi
1404ec34e: e8 8d 51 41 00              	call	0x1409014e0 <.text+0x9004e0>
1404ec353: 48 89 d9                    	mov	rcx, rbx
1404ec356: 48 89 fa                    	mov	rdx, rdi
1404ec359: e8 12 64 54 00              	call	0x140a32770 <.text+0xa31770>
1404ec35e: 48 89 d9                    	mov	rcx, rbx
1404ec361: 48 89 fa                    	mov	rdx, rdi
1404ec364: e8 d7 5d 53 00              	call	0x140a22140 <.text+0xa21140>
1404ec369: 48 89 d9                    	mov	rcx, rbx
1404ec36c: 48 89 fa                    	mov	rdx, rdi
1404ec36f: e8 fc 75 52 00              	call	0x140a13970 <.text+0xa12970>
1404ec374: 48 89 d9                    	mov	rcx, rbx
1404ec377: 48 89 fa                    	mov	rdx, rdi
1404ec37a: e8 41 89 3f 00              	call	0x1408e4cc0 <.text+0x8e3cc0>
1404ec37f: 48 89 d9                    	mov	rcx, rbx
1404ec382: 48 89 fa                    	mov	rdx, rdi
1404ec385: e8 b6 a1 37 00              	call	0x140866540 <.text+0x865540>
1404ec38a: 48 89 d9                    	mov	rcx, rbx
1404ec38d: 48 89 fa                    	mov	rdx, rdi
1404ec390: e8 eb 67 53 00              	call	0x140a22b80 <.text+0xa21b80>
1404ec395: 48 89 d9                    	mov	rcx, rbx
1404ec398: 48 89 fa                    	mov	rdx, rdi
1404ec39b: e8 d0 cb 46 00              	call	0x140958f70 <.text+0x957f70>
1404ec3a0: 48 89 d9                    	mov	rcx, rbx
1404ec3a3: 48 89 fa                    	mov	rdx, rdi
1404ec3a6: e8 55 89 40 00              	call	0x1408f4d00 <.text+0x8f3d00>
1404ec3ab: 48 89 d9                    	mov	rcx, rbx
1404ec3ae: 48 89 fa                    	mov	rdx, rdi
1404ec3b1: e8 5a d7 53 00              	call	0x140a29b10 <.text+0xa28b10>
1404ec3b6: 48 89 d9                    	mov	rcx, rbx
1404ec3b9: 48 89 fa                    	mov	rdx, rdi
1404ec3bc: e8 8f c6 57 00              	call	0x140a68a50 <.text+0xa67a50>
1404ec3c1: 48 89 d9                    	mov	rcx, rbx
1404ec3c4: 48 89 fa                    	mov	rdx, rdi
1404ec3c7: e8 24 e7 49 00              	call	0x14098aaf0 <.text+0x989af0>
1404ec3cc: 48 89 d9                    	mov	rcx, rbx
1404ec3cf: 48 89 fa                    	mov	rdx, rdi
1404ec3d2: 45 31 c0                    	xor	r8d, r8d
1404ec3d5: e8 f6 b9 49 00              	call	0x140987dd0 <.text+0x986dd0>
1404ec3da: 48 89 d9                    	mov	rcx, rbx
1404ec3dd: 48 89 fa                    	mov	rdx, rdi
1404ec3e0: 4c 8b ad 60 04 00 00        	mov	r13, qword ptr [rbp + 0x460]
1404ec3e7: 4d 89 e8                    	mov	r8, r13
1404ec3ea: e8 81 fe 4d 00              	call	0x1409cc270 <.text+0x9cb270>
1404ec3ef: 48 89 d9                    	mov	rcx, rbx
1404ec3f2: 48 89 fa                    	mov	rdx, rdi
1404ec3f5: e8 46 50 56 00              	call	0x140a51440 <.text+0xa50440>
1404ec3fa: 48 8d 8d 10 04 00 00        	lea	rcx, [rbp + 0x410]
1404ec401: 4c 89 fa                    	mov	rdx, r15
1404ec404: 41 b8 65 00 00 00           	mov	r8d, 0x65
1404ec40a: e8 91 8e 6e 00              	call	0x140bd52a0 <.text+0xbd42a0>
1404ec40f: 0f 10 85 10 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x410]
1404ec416: 0f 10 8d 20 04 00 00        	movups	xmm1, xmmword ptr [rbp + 0x420]
1404ec41d: 0f 29 8d 50 02 00 00        	movaps	xmmword ptr [rbp + 0x250], xmm1
1404ec424: 0f 29 85 40 02 00 00        	movaps	xmmword ptr [rbp + 0x240], xmm0
1404ec42b: 48 8d 8d 50 03 00 00        	lea	rcx, [rbp + 0x350]
1404ec432: 48 8d 95 40 02 00 00        	lea	rdx, [rbp + 0x240]
1404ec439: e8 a2 a8 6e 00              	call	0x140bd6ce0 <.text+0xbd5ce0>
1404ec43e: 48 8d 05 83 fb 4f 02        	lea	rax, [rip + 0x24ffb83]  # 0x1429ebfc8
1404ec445: 48 89 85 40 03 00 00        	mov	qword ptr [rbp + 0x340], rax
1404ec44c: 48 c7 85 48 03 00 00 07 00 00 00    	mov	qword ptr [rbp + 0x348], 0x7
1404ec457: 48 8d 95 40 03 00 00        	lea	rdx, [rbp + 0x340]
1404ec45e: 48 89 d9                    	mov	rcx, rbx
1404ec461: e8 6a d0 6f 00              	call	0x140be94d0 <.text+0xbe84d0>
1404ec466: 89 c6                       	mov	esi, eax
1404ec468: 0f 10 85 10 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x410]
1404ec46f: 0f 10 8d 20 04 00 00        	movups	xmm1, xmmword ptr [rbp + 0x420]
1404ec476: 0f 29 85 e0 00 00 00        	movaps	xmmword ptr [rbp + 0xe0], xmm0
1404ec47d: 0f 29 8d f0 00 00 00        	movaps	xmmword ptr [rbp + 0xf0], xmm1
1404ec484: 48 8d 8d e0 03 00 00        	lea	rcx, [rbp + 0x3e0]
1404ec48b: 4c 89 fa                    	mov	rdx, r15
1404ec48e: 41 b8 61 00 00 00           	mov	r8d, 0x61
1404ec494: e8 07 8e 6e 00              	call	0x140bd52a0 <.text+0xbd42a0>
1404ec499: 4c 8d b5 88 02 00 00        	lea	r14, [rbp + 0x288]
1404ec4a0: 0f 10 85 e0 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x3e0]
1404ec4a7: 0f 10 8d f0 03 00 00        	movups	xmm1, xmmword ptr [rbp + 0x3f0]
1404ec4ae: 0f 29 8d 20 04 00 00        	movaps	xmmword ptr [rbp + 0x420], xmm1
1404ec4b5: 0f 29 85 10 04 00 00        	movaps	xmmword ptr [rbp + 0x410], xmm0
1404ec4bc: 48 8d 8d 50 03 00 00        	lea	rcx, [rbp + 0x350]
1404ec4c3: 48 8d 95 10 04 00 00        	lea	rdx, [rbp + 0x410]
1404ec4ca: e8 11 a8 6e 00              	call	0x140bd6ce0 <.text+0xbd5ce0>
1404ec4cf: 48 8d 05 f9 fa 4f 02        	lea	rax, [rip + 0x24ffaf9]  # 0x1429ebfcf
1404ec4d6: 48 89 85 40 03 00 00        	mov	qword ptr [rbp + 0x340], rax
1404ec4dd: 48 c7 85 48 03 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x348], 0x8
1404ec4e8: 48 8d 95 40 03 00 00        	lea	rdx, [rbp + 0x340]
1404ec4ef: 48 89 d9                    	mov	rcx, rbx
1404ec4f2: e8 d9 cf 6f 00              	call	0x140be94d0 <.text+0xbe84d0>
1404ec4f7: 0f 10 85 e0 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x3e0]
1404ec4fe: 0f 10 8d f0 03 00 00        	movups	xmm1, xmmword ptr [rbp + 0x3f0]
1404ec505: 0f 11 85 8c 02 00 00        	movups	xmmword ptr [rbp + 0x28c], xmm0
1404ec50c: 0f 11 8d 9c 02 00 00        	movups	xmmword ptr [rbp + 0x29c], xmm1
1404ec513: 89 85 88 02 00 00           	mov	dword ptr [rbp + 0x288], eax
1404ec519: 89 b5 40 02 00 00           	mov	dword ptr [rbp + 0x240], esi
1404ec51f: 0f 28 85 e0 00 00 00        	movaps	xmm0, xmmword ptr [rbp + 0xe0]
1404ec526: 0f 28 8d f0 00 00 00        	movaps	xmm1, xmmword ptr [rbp + 0xf0]
1404ec52d: 0f 11 85 44 02 00 00        	movups	xmmword ptr [rbp + 0x244], xmm0
1404ec534: 0f 11 8d 54 02 00 00        	movups	xmmword ptr [rbp + 0x254], xmm1
1404ec53b: 48 8d b5 64 02 00 00        	lea	rsi, [rbp + 0x264]
1404ec542: 41 0f 10 04 24              	movups	xmm0, xmmword ptr [r12]
1404ec547: 41 0f 10 4c 24 10           	movups	xmm1, xmmword ptr [r12 + 0x10]
1404ec54d: 0f 11 85 64 02 00 00        	movups	xmmword ptr [rbp + 0x264], xmm0
1404ec554: 0f 11 8d 74 02 00 00        	movups	xmmword ptr [rbp + 0x274], xmm1
1404ec55b: 41 8b 44 24 20              	mov	eax, dword ptr [r12 + 0x20]
1404ec560: 89 85 84 02 00 00           	mov	dword ptr [rbp + 0x284], eax
1404ec566: 4c 89 6c 24 20              	mov	qword ptr [rsp + 0x20], r13
1404ec56b: 4c 8b bd f8 04 00 00        	mov	r15, qword ptr [rbp + 0x4f8]
1404ec572: 4c 8d a5 40 02 00 00        	lea	r12, [rbp + 0x240]
1404ec579: 48 89 d9                    	mov	rcx, rbx
1404ec57c: 48 89 fa                    	mov	rdx, rdi
1404ec57f: 4d 89 e0                    	mov	r8, r12
1404ec582: 4d 89 f9                    	mov	r9, r15
1404ec585: e8 36 76 34 00              	call	0x140833bc0 <.text+0x832bc0>
1404ec58a: 48 8d 05 46 fa 4f 02        	lea	rax, [rip + 0x24ffa46]  # 0x1429ebfd7
1404ec591: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1404ec596: 48 c7 44 24 28 04 00 00 00  	mov	qword ptr [rsp + 0x28], 0x4
1404ec59f: 48 8d 4d 60                 	lea	rcx, [rbp + 0x60]
1404ec5a3: 48 89 da                    	mov	rdx, rbx
1404ec5a6: 49 89 f8                    	mov	r8, rdi
1404ec5a9: 49 89 f1                    	mov	r9, rsi
1404ec5ac: e8 3f c1 43 00              	call	0x1409286f0 <.text+0x9276f0>
1404ec5b1: 48 8b 05 20 fb 13 03        	mov	rax, qword ptr [rip + 0x313fb20] # 0x14362c0d8
1404ec5b8: 80 38 00                    	cmp	byte ptr [rax], 0x0
1404ec5bb: 75 20                       	jne	0x1404ec5dd <.text+0x4eb5dd>
1404ec5bd: 48 8d 75 60                 	lea	rsi, [rbp + 0x60]
1404ec5c1: 48 89 d9                    	mov	rcx, rbx
1404ec5c4: 48 89 fa                    	mov	rdx, rdi
1404ec5c7: 49 89 f0                    	mov	r8, rsi
1404ec5ca: e8 d1 ec 49 00              	call	0x14098b2a0 <.text+0x98a2a0>
1404ec5cf: 48 89 d9                    	mov	rcx, rbx
1404ec5d2: 48 89 fa                    	mov	rdx, rdi
1404ec5d5: 49 89 f0                    	mov	r8, rsi
1404ec5d8: e8 f3 b7 49 00              	call	0x140987dd0 <.text+0x986dd0>
1404ec5dd: 4c 8d 45 60                 	lea	r8, [rbp + 0x60]
1404ec5e1: 48 89 d9                    	mov	rcx, rbx
1404ec5e4: 48 89 fa                    	mov	rdx, rdi
1404ec5e7: e8 94 3c 57 00              	call	0x140a60280 <.text+0xa5f280>
1404ec5ec: 48 8b b5 60 04 00 00        	mov	rsi, qword ptr [rbp + 0x460]
1404ec5f3: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
1404ec5f8: 48 89 d9                    	mov	rcx, rbx
1404ec5fb: 48 89 fa                    	mov	rdx, rdi
1404ec5fe: 4d 89 e0                    	mov	r8, r12
1404ec601: 4d 89 f9                    	mov	r9, r15
1404ec604: e8 37 94 34 00              	call	0x140835a40 <.text+0x834a40>
1404ec609: 48 89 d9                    	mov	rcx, rbx
1404ec60c: 48 89 fa                    	mov	rdx, rdi
1404ec60f: 4c 8d ad 64 02 00 00        	lea	r13, [rbp + 0x264]
1404ec616: 4d 89 e8                    	mov	r8, r13
1404ec619: e8 c2 2c 54 00              	call	0x140a2f2e0 <.text+0xa2e2e0>
1404ec61e: 48 89 d9                    	mov	rcx, rbx
1404ec621: 48 89 fa                    	mov	rdx, rdi
1404ec624: 4d 89 e8                    	mov	r8, r13
1404ec627: e8 54 b2 52 00              	call	0x140a17880 <.text+0xa16880>
1404ec62c: 48 89 d9                    	mov	rcx, rbx
1404ec62f: 48 89 fa                    	mov	rdx, rdi
1404ec632: 4d 89 e8                    	mov	r8, r13
1404ec635: e8 e6 4f 51 00              	call	0x140a01620 <.text+0xa00620>
1404ec63a: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
1404ec63f: 48 89 d9                    	mov	rcx, rbx
1404ec642: 48 89 fa                    	mov	rdx, rdi
1404ec645: 4d 89 e0                    	mov	r8, r12
1404ec648: 4d 89 f9                    	mov	r9, r15
1404ec64b: e8 60 ad 34 00              	call	0x1408373b0 <.text+0x8363b0>
1404ec650: 48 8d 8d 40 03 00 00        	lea	rcx, [rbp + 0x340]
1404ec657: 48 89 da                    	mov	rdx, rbx
1404ec65a: 49 89 f8                    	mov	r8, rdi
1404ec65d: 4d 89 e1                    	mov	r9, r12
1404ec660: e8 1b 9b 40 00              	call	0x1408f6180 <.text+0x8f5180>
1404ec665: 8b 85 60 03 00 00           	mov	eax, dword ptr [rbp + 0x360]
1404ec66b: 89 85 00 02 00 00           	mov	dword ptr [rbp + 0x200], eax
1404ec671: 0f 10 85 40 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x340]
1404ec678: 0f 10 8d 50 03 00 00        	movups	xmm1, xmmword ptr [rbp + 0x350]
1404ec67f: 0f 29 8d f0 01 00 00        	movaps	xmmword ptr [rbp + 0x1f0], xmm1
1404ec686: 0f 29 85 e0 01 00 00        	movaps	xmmword ptr [rbp + 0x1e0], xmm0
1404ec68d: 8b 85 84 03 00 00           	mov	eax, dword ptr [rbp + 0x384]
1404ec693: 89 85 80 01 00 00           	mov	dword ptr [rbp + 0x180], eax
1404ec699: 0f 10 85 74 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x374]
1404ec6a0: 0f 29 85 70 01 00 00        	movaps	xmmword ptr [rbp + 0x170], xmm0
1404ec6a7: 0f 10 85 64 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x364]
1404ec6ae: 0f 29 85 60 01 00 00        	movaps	xmmword ptr [rbp + 0x160], xmm0
1404ec6b5: 48 8d 85 b0 00 00 00        	lea	rax, [rbp + 0xb0]
1404ec6bc: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
1404ec6c1: 4c 89 74 24 20              	mov	qword ptr [rsp + 0x20], r14
1404ec6c6: 48 8d b5 8c 01 00 00        	lea	rsi, [rbp + 0x18c]
1404ec6cd: 48 89 f1                    	mov	rcx, rsi
1404ec6d0: 48 89 da                    	mov	rdx, rbx
1404ec6d3: 49 89 f8                    	mov	r8, rdi
1404ec6d6: 4d 89 e9                    	mov	r9, r13
1404ec6d9: e8 b2 0a 52 00              	call	0x140a0d190 <.text+0xa0c190>
1404ec6de: 48 89 74 24 30              	mov	qword ptr [rsp + 0x30], rsi
1404ec6e3: 4c 89 64 24 28              	mov	qword ptr [rsp + 0x28], r12
1404ec6e8: 4c 8d ad 60 01 00 00        	lea	r13, [rbp + 0x160]
1404ec6ef: 4c 89 6c 24 20              	mov	qword ptr [rsp + 0x20], r13
1404ec6f4: 48 8d 8d 8c 00 00 00        	lea	rcx, [rbp + 0x8c]
1404ec6fb: 4c 8d 8d e0 01 00 00        	lea	r9, [rbp + 0x1e0]
1404ec702: 48 89 da                    	mov	rdx, rbx
1404ec705: 49 89 f8                    	mov	r8, rdi
1404ec708: e8 a3 7c 38 00              	call	0x1408743b0 <.text+0x8733b0>
1404ec70d: 48 8b 85 e8 04 00 00        	mov	rax, qword ptr [rbp + 0x4e8]
1404ec714: 44 0f b6 a0 ab 00 00 00     	movzx	r12d, byte ptr [rax + 0xab]
1404ec71c: 45 84 e4                    	test	r12b, r12b
1404ec71f: 0f 84 c7 00 00 00           	je	0x1404ec7ec <.text+0x4eb7ec>
1404ec725: 48 ba 8b 04 7b e6 61 26 c0 a4       	movabs	rdx, -0x5b3fd99e1984fb75
1404ec72f: 49 b8 ec 67 2a aa e6 61 e7 ea       	movabs	r8, -0x15189e1955d59814
1404ec739: 48 8b 8d d8 03 00 00        	mov	rcx, qword ptr [rbp + 0x3d8]
1404ec740: e8 ab 0f bf ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1404ec745: 48 85 c0                    	test	rax, rax
1404ec748: 0f 84 85 09 00 00           	je	0x1404ed0d3 <.text+0x4ec0d3>
1404ec74e: 48 39 97 18 02 00 00        	cmp	qword ptr [rdi + 0x218], rdx
1404ec755: 0f 86 78 09 00 00           	jbe	0x1404ed0d3 <.text+0x4ec0d3>
1404ec75b: 48 8b 87 10 02 00 00        	mov	rax, qword ptr [rdi + 0x210]
1404ec762: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
1404ec766: 48 85 c9                    	test	rcx, rcx
1404ec769: 0f 84 64 09 00 00           	je	0x1404ed0d3 <.text+0x4ec0d3>
1404ec76f: 48 8b 87 e0 01 00 00        	mov	rax, qword ptr [rdi + 0x1e0]
1404ec776: 48 f7 d1                    	not	rcx
1404ec779: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1404ec77d: 48 c1 e1 05                 	shl	rcx, 0x5
1404ec781: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
1404ec787: 0f 84 46 09 00 00           	je	0x1404ed0d3 <.text+0x4ec0d3>
1404ec78d: 48 01 c8                    	add	rax, rcx
1404ec790: 48 8b 40 10                 	mov	rax, qword ptr [rax + 0x10]
1404ec794: 48 8b 50 08                 	mov	rdx, qword ptr [rax + 0x8]
1404ec798: 48 8d b5 40 03 00 00        	lea	rsi, [rbp + 0x340]
1404ec79f: 48 89 f1                    	mov	rcx, rsi
1404ec7a2: 41 b8 7a 00 00 00           	mov	r8d, 0x7a
1404ec7a8: e8 f3 8a 6e 00              	call	0x140bd52a0 <.text+0xbd42a0>
1404ec7ad: 4c 8d b5 10 04 00 00        	lea	r14, [rbp + 0x410]
1404ec7b4: 4c 89 f1                    	mov	rcx, r14
1404ec7b7: 48 89 f2                    	mov	rdx, rsi
1404ec7ba: e8 b1 8b 6e 00              	call	0x140bd5370 <.text+0xbd4370>
1404ec7bf: 48 8d b5 e0 03 00 00        	lea	rsi, [rbp + 0x3e0]
1404ec7c6: 48 89 f1                    	mov	rcx, rsi
1404ec7c9: 4c 89 f2                    	mov	rdx, r14
1404ec7cc: e8 bf 8b 6e 00              	call	0x140bd5390 <.text+0xbd4390>
1404ec7d1: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
1404ec7d6: 4c 8d 05 23 f8 4f 02        	lea	r8, [rip + 0x24ff823]   # 0x1429ec000
1404ec7dd: 48 8d 8d 40 03 00 00        	lea	rcx, [rbp + 0x340]
1404ec7e4: 41 b9 0c 00 00 00           	mov	r9d, 0xc
1404ec7ea: eb 5e                       	jmp	0x1404ec84a <.text+0x4eb84a>
1404ec7ec: 49 8d 4f 04                 	lea	rcx, [r15 + 0x4]
1404ec7f0: e8 9b 8f 6e 00              	call	0x140bd5790 <.text+0xbd4790>
1404ec7f5: 48 8d b5 40 03 00 00        	lea	rsi, [rbp + 0x340]
1404ec7fc: 48 89 f1                    	mov	rcx, rsi
1404ec7ff: 48 89 c2                    	mov	rdx, rax
1404ec802: 41 b8 7a 00 00 00           	mov	r8d, 0x7a
1404ec808: e8 93 8a 6e 00              	call	0x140bd52a0 <.text+0xbd42a0>
1404ec80d: 4c 8d b5 10 04 00 00        	lea	r14, [rbp + 0x410]
1404ec814: 4c 89 f1                    	mov	rcx, r14
1404ec817: 48 89 f2                    	mov	rdx, rsi
1404ec81a: e8 51 8b 6e 00              	call	0x140bd5370 <.text+0xbd4370>
1404ec81f: 48 8d b5 e0 03 00 00        	lea	rsi, [rbp + 0x3e0]
1404ec826: 48 89 f1                    	mov	rcx, rsi
1404ec829: 4c 89 f2                    	mov	rdx, r14
1404ec82c: e8 5f 8b 6e 00              	call	0x140bd5390 <.text+0xbd4390>
1404ec831: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
1404ec836: 4c 8d 05 9e f7 4f 02        	lea	r8, [rip + 0x24ff79e]   # 0x1429ebfdb
1404ec83d: 48 8d 8d 40 03 00 00        	lea	rcx, [rbp + 0x340]
1404ec844: 41 b9 0b 00 00 00           	mov	r9d, 0xb
1404ec84a: 48 89 da                    	mov	rdx, rbx
1404ec84d: e8 fe ab 00 00              	call	0x1404f7450 <.text+0x4f6450>
1404ec852: 44 88 a5 6e 04 00 00        	mov	byte ptr [rbp + 0x46e], r12b
1404ec859: 83 bd 40 03 00 00 01        	cmp	dword ptr [rbp + 0x340], 0x1
1404ec860: 0f 84 c0 08 00 00           	je	0x1404ed126 <.text+0x4ec126>
1404ec866: 48 8b 85 84 03 00 00        	mov	rax, qword ptr [rbp + 0x384]
1404ec86d: 48 89 85 50 04 00 00        	mov	qword ptr [rbp + 0x450], rax
1404ec874: 0f 10 85 44 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x344]
1404ec87b: 0f 10 8d 54 03 00 00        	movups	xmm1, xmmword ptr [rbp + 0x354]
1404ec882: 0f 10 95 64 03 00 00        	movups	xmm2, xmmword ptr [rbp + 0x364]
1404ec889: 0f 10 9d 74 03 00 00        	movups	xmm3, xmmword ptr [rbp + 0x374]
1404ec890: 0f 29 9d 40 04 00 00        	movaps	xmmword ptr [rbp + 0x440], xmm3
1404ec897: 0f 29 95 30 04 00 00        	movaps	xmmword ptr [rbp + 0x430], xmm2
1404ec89e: 0f 29 8d 20 04 00 00        	movaps	xmmword ptr [rbp + 0x420], xmm1
1404ec8a5: 0f 29 85 10 04 00 00        	movaps	xmmword ptr [rbp + 0x410], xmm0
1404ec8ac: 0f b6 85 8c 03 00 00        	movzx	eax, byte ptr [rbp + 0x38c]
1404ec8b3: 0f 10 85 34 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x434]
1404ec8ba: 0f 11 85 43 03 00 00        	movups	xmmword ptr [rbp + 0x343], xmm0
1404ec8c1: 0f 10 85 44 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x444]
1404ec8c8: 0f 11 85 53 03 00 00        	movups	xmmword ptr [rbp + 0x353], xmm0
1404ec8cf: 8b 8d 54 04 00 00           	mov	ecx, dword ptr [rbp + 0x454]
1404ec8d5: 89 8d 63 03 00 00           	mov	dword ptr [rbp + 0x363], ecx
1404ec8db: 8b 8d 30 04 00 00           	mov	ecx, dword ptr [rbp + 0x430]
1404ec8e1: 89 8d d0 01 00 00           	mov	dword ptr [rbp + 0x1d0], ecx
1404ec8e7: 0f 28 85 10 04 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x410]
1404ec8ee: 0f 28 8d 20 04 00 00        	movaps	xmm1, xmmword ptr [rbp + 0x420]
1404ec8f5: 0f 29 8d c0 01 00 00        	movaps	xmmword ptr [rbp + 0x1c0], xmm1
1404ec8fc: 0f 29 85 b0 01 00 00        	movaps	xmmword ptr [rbp + 0x1b0], xmm0
1404ec903: 8b 8d 63 03 00 00           	mov	ecx, dword ptr [rbp + 0x363]
1404ec909: 89 8d d0 03 00 00           	mov	dword ptr [rbp + 0x3d0], ecx
1404ec90f: 0f 10 85 43 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x343]
1404ec916: 0f 10 8d 53 03 00 00        	movups	xmm1, xmmword ptr [rbp + 0x353]
1404ec91d: 0f 29 8d c0 03 00 00        	movaps	xmmword ptr [rbp + 0x3c0], xmm1
1404ec924: 0f 29 85 b0 03 00 00        	movaps	xmmword ptr [rbp + 0x3b0], xmm0
1404ec92b: 88 85 d4 03 00 00           	mov	byte ptr [rbp + 0x3d4], al
1404ec931: 48 8d 8d 8c 00 00 00        	lea	rcx, [rbp + 0x8c]
1404ec938: 48 89 4c 24 48              	mov	qword ptr [rsp + 0x48], rcx
1404ec93d: 49 89 cc                    	mov	r12, rcx
1404ec940: 4c 8d bd 8c 01 00 00        	lea	r15, [rbp + 0x18c]
1404ec947: 4c 89 7c 24 40              	mov	qword ptr [rsp + 0x40], r15
1404ec94c: 24 01                       	and	al, 0x1
1404ec94e: 88 44 24 38                 	mov	byte ptr [rsp + 0x38], al
1404ec952: 48 8d 85 b0 03 00 00        	lea	rax, [rbp + 0x3b0]
1404ec959: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
1404ec95e: 4c 89 6c 24 28              	mov	qword ptr [rsp + 0x28], r13
1404ec963: 48 8d b5 e0 01 00 00        	lea	rsi, [rbp + 0x1e0]
1404ec96a: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
1404ec96f: 4c 8d 8d 40 02 00 00        	lea	r9, [rbp + 0x240]
1404ec976: 4c 8d b5 40 03 00 00        	lea	r14, [rbp + 0x340]
1404ec97d: 4c 89 f1                    	mov	rcx, r14
1404ec980: 48 89 da                    	mov	rdx, rbx
1404ec983: 49 89 f8                    	mov	r8, rdi
1404ec986: e8 65 53 51 00              	call	0x140a01cf0 <.text+0xa00cf0>
1404ec98b: 4c 89 7c 24 38              	mov	qword ptr [rsp + 0x38], r15
1404ec990: 4c 89 74 24 30              	mov	qword ptr [rsp + 0x30], r14
1404ec995: 4c 89 64 24 28              	mov	qword ptr [rsp + 0x28], r12
1404ec99a: 4c 89 6c 24 20              	mov	qword ptr [rsp + 0x20], r13
1404ec99f: 48 8d 4d 3c                 	lea	rcx, [rbp + 0x3c]
1404ec9a3: 48 89 da                    	mov	rdx, rbx
1404ec9a6: 49 89 f8                    	mov	r8, rdi
1404ec9a9: 49 89 f1                    	mov	r9, rsi
1404ec9ac: e8 1f 83 55 00              	call	0x140a44cd0 <.text+0xa43cd0>
1404ec9b1: 4c 8d a5 e0 00 00 00        	lea	r12, [rbp + 0xe0]
1404ec9b8: 4c 89 e1                    	mov	rcx, r12
1404ec9bb: 48 89 da                    	mov	rdx, rbx
1404ec9be: 49 89 f8                    	mov	r8, rdi
1404ec9c1: e8 aa ad 4f 00              	call	0x1409e7770 <.text+0x9e6770>
1404ec9c6: 4c 89 64 24 30              	mov	qword ptr [rsp + 0x30], r12
1404ec9cb: 4c 89 7c 24 28              	mov	qword ptr [rsp + 0x28], r15
1404ec9d0: 4c 89 6c 24 20              	mov	qword ptr [rsp + 0x20], r13
1404ec9d5: 4c 8d 6d 18                 	lea	r13, [rbp + 0x18]
1404ec9d9: 4c 89 e9                    	mov	rcx, r13
1404ec9dc: 48 89 da                    	mov	rdx, rbx
1404ec9df: 49 89 f8                    	mov	r8, rdi
1404ec9e2: 49 89 f1                    	mov	r9, rsi
1404ec9e5: e8 d6 34 58 00              	call	0x140a6fec0 <.text+0xa6eec0>
1404ec9ea: 4c 89 7c 24 20              	mov	qword ptr [rsp + 0x20], r15
1404ec9ef: 4c 8d 7d f4                 	lea	r15, [rbp - 0xc]
1404ec9f3: 4c 89 f9                    	mov	rcx, r15
1404ec9f6: 48 89 da                    	mov	rdx, rbx
1404ec9f9: 49 89 f8                    	mov	r8, rdi
1404ec9fc: 4c 8d 8d 40 02 00 00        	lea	r9, [rbp + 0x240]
1404eca03: e8 68 d4 54 00              	call	0x140a39e70 <.text+0xa38e70>
1404eca08: 0f b6 85 d4 03 00 00        	movzx	eax, byte ptr [rbp + 0x3d4]
1404eca0f: 4c 89 64 24 60              	mov	qword ptr [rsp + 0x60], r12
1404eca14: 49 89 f4                    	mov	r12, rsi
1404eca17: 48 8d b5 b0 03 00 00        	lea	rsi, [rbp + 0x3b0]
1404eca1e: 4c 89 6c 24 58              	mov	qword ptr [rsp + 0x58], r13
1404eca23: 4c 89 7c 24 50              	mov	qword ptr [rsp + 0x50], r15
1404eca28: 48 8d 4d 3c                 	lea	rcx, [rbp + 0x3c]
1404eca2c: 48 89 4c 24 48              	mov	qword ptr [rsp + 0x48], rcx
1404eca31: 4c 89 74 24 40              	mov	qword ptr [rsp + 0x40], r14
1404eca36: 4c 89 64 24 38              	mov	qword ptr [rsp + 0x38], r12
1404eca3b: 48 8d 8d 8c 00 00 00        	lea	rcx, [rbp + 0x8c]
1404eca42: 48 89 4c 24 30              	mov	qword ptr [rsp + 0x30], rcx
1404eca47: 88 44 24 28                 	mov	byte ptr [rsp + 0x28], al
1404eca4b: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
1404eca50: 48 89 d9                    	mov	rcx, rbx
1404eca53: 48 89 fa                    	mov	rdx, rdi
1404eca56: 4c 8d 85 40 02 00 00        	lea	r8, [rbp + 0x240]
1404eca5d: 4c 8b ad f8 04 00 00        	mov	r13, qword ptr [rbp + 0x4f8]
1404eca64: 4d 89 e9                    	mov	r9, r13
1404eca67: e8 44 56 4e 00              	call	0x1409d20b0 <.text+0x9d10b0>
1404eca6c: 48 8b 05 25 f6 13 03        	mov	rax, qword ptr [rip + 0x313f625] # 0x14362c098
1404eca73: 80 38 00                    	cmp	byte ptr [rax], 0x0
1404eca76: 0f 85 1d 01 00 00           	jne	0x1404ecb99 <.text+0x4ebb99>
1404eca7c: 49 8d 4d 04                 	lea	rcx, [r13 + 0x4]
1404eca80: e8 0b 8d 6e 00              	call	0x140bd5790 <.text+0xbd4790>
1404eca85: 48 8d 8d e0 02 00 00        	lea	rcx, [rbp + 0x2e0]
1404eca8c: 48 89 c2                    	mov	rdx, rax
1404eca8f: 41 b8 7a 00 00 00           	mov	r8d, 0x7a
1404eca95: e8 06 88 6e 00              	call	0x140bd52a0 <.text+0xbd42a0>
1404eca9a: 0f 10 85 e0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2e0]
1404ecaa1: 0f 10 8d f0 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x2f0]
1404ecaa8: 0f 29 8d f0 03 00 00        	movaps	xmmword ptr [rbp + 0x3f0], xmm1
1404ecaaf: 0f 29 85 e0 03 00 00        	movaps	xmmword ptr [rbp + 0x3e0], xmm0
1404ecab6: 48 8d 8d 20 04 00 00        	lea	rcx, [rbp + 0x420]
1404ecabd: 48 8d 95 e0 03 00 00        	lea	rdx, [rbp + 0x3e0]
1404ecac4: e8 17 a2 6e 00              	call	0x140bd6ce0 <.text+0xbd5ce0>
1404ecac9: 48 8d 05 58 f5 4f 02        	lea	rax, [rip + 0x24ff558]  # 0x1429ec028
1404ecad0: 48 89 85 10 04 00 00        	mov	qword ptr [rbp + 0x410], rax
1404ecad7: 48 c7 85 18 04 00 00 11 00 00 00    	mov	qword ptr [rbp + 0x418], 0x11
1404ecae2: 48 8d b5 10 04 00 00        	lea	rsi, [rbp + 0x410]
1404ecae9: 48 89 d9                    	mov	rcx, rbx
1404ecaec: 48 89 f2                    	mov	rdx, rsi
1404ecaef: e8 dc c9 6f 00              	call	0x140be94d0 <.text+0xbe84d0>
1404ecaf4: 0f 10 85 e0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2e0]
1404ecafb: 0f 10 8d f0 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x2f0]
1404ecb02: 0f 11 85 14 04 00 00        	movups	xmmword ptr [rbp + 0x414], xmm0
1404ecb09: 0f 11 8d 24 04 00 00        	movups	xmmword ptr [rbp + 0x424], xmm1
1404ecb10: 89 85 10 04 00 00           	mov	dword ptr [rbp + 0x410], eax
1404ecb16: 4c 8b b5 60 04 00 00        	mov	r14, qword ptr [rbp + 0x460]
1404ecb1d: 4c 89 74 24 28              	mov	qword ptr [rsp + 0x28], r14
1404ecb22: 4c 8d bd 64 02 00 00        	lea	r15, [rbp + 0x264]
1404ecb29: 4c 89 7c 24 20              	mov	qword ptr [rsp + 0x20], r15
1404ecb2e: 48 89 d9                    	mov	rcx, rbx
1404ecb31: 48 89 fa                    	mov	rdx, rdi
1404ecb34: 49 89 f0                    	mov	r8, rsi
1404ecb37: 4d 89 e9                    	mov	r9, r13
1404ecb3a: e8 e1 27 4d 00              	call	0x1409bf320 <.text+0x9be320>
1404ecb3f: 48 8d 05 8a 3d 4d 00        	lea	rax, [rip + 0x4d3d8a]   # 0x1409c08d0 <.text+0x9bf8d0>
1404ecb46: 4c 8d 15 b3 2c 4d 00        	lea	r10, [rip + 0x4d2cb3]   # 0x1409bf800 <.text+0x9be800>
1404ecb4d: 80 bd 6f 04 00 00 00        	cmp	byte ptr [rbp + 0x46f], 0x0
1404ecb54: 4c 0f 45 d0                 	cmovne	r10, rax
1404ecb58: 0f b6 85 d4 03 00 00        	movzx	eax, byte ptr [rbp + 0x3d4]
1404ecb5f: 4c 89 74 24 48              	mov	qword ptr [rsp + 0x48], r14
1404ecb64: 48 8d 8d 8c 01 00 00        	lea	rcx, [rbp + 0x18c]
1404ecb6b: 48 89 4c 24 40              	mov	qword ptr [rsp + 0x40], rcx
1404ecb70: 48 89 74 24 38              	mov	qword ptr [rsp + 0x38], rsi
1404ecb75: 48 8d b5 b0 03 00 00        	lea	rsi, [rbp + 0x3b0]
1404ecb7c: 88 44 24 30                 	mov	byte ptr [rsp + 0x30], al
1404ecb80: 48 89 74 24 28              	mov	qword ptr [rsp + 0x28], rsi
1404ecb85: 4c 89 64 24 20              	mov	qword ptr [rsp + 0x20], r12
1404ecb8a: 48 89 d9                    	mov	rcx, rbx
1404ecb8d: 48 89 fa                    	mov	rdx, rdi
1404ecb90: 4d 89 e8                    	mov	r8, r13
1404ecb93: 4d 89 f9                    	mov	r9, r15
1404ecb96: 41 ff d2                    	call	r10
1404ecb99: 0f b6 85 d4 03 00 00        	movzx	eax, byte ptr [rbp + 0x3d4]
1404ecba0: 48 8b 8d 60 04 00 00        	mov	rcx, qword ptr [rbp + 0x460]
1404ecba7: 48 89 4c 24 38              	mov	qword ptr [rsp + 0x38], rcx
1404ecbac: 88 44 24 30                 	mov	byte ptr [rsp + 0x30], al
1404ecbb0: 48 89 74 24 28              	mov	qword ptr [rsp + 0x28], rsi
1404ecbb5: 4c 89 64 24 20              	mov	qword ptr [rsp + 0x20], r12
1404ecbba: 48 89 d9                    	mov	rcx, rbx
1404ecbbd: 48 89 fa                    	mov	rdx, rdi
1404ecbc0: 4d 89 e8                    	mov	r8, r13
1404ecbc3: 48 8d b5 64 02 00 00        	lea	rsi, [rbp + 0x264]
1404ecbca: 49 89 f1                    	mov	r9, rsi
1404ecbcd: e8 ae 6e 4d 00              	call	0x1409c3a80 <.text+0x9c2a80>
1404ecbd2: 48 89 d9                    	mov	rcx, rbx
1404ecbd5: 48 89 fa                    	mov	rdx, rdi
1404ecbd8: e8 13 db 54 00              	call	0x140a3a6f0 <.text+0xa396f0>
1404ecbdd: 8b 46 20                    	mov	eax, dword ptr [rsi + 0x20]
1404ecbe0: 89 85 30 02 00 00           	mov	dword ptr [rbp + 0x230], eax
1404ecbe6: 0f 10 06                    	movups	xmm0, xmmword ptr [rsi]
1404ecbe9: 0f 10 4e 10                 	movups	xmm1, xmmword ptr [rsi + 0x10]
1404ecbed: 0f 29 8d 20 02 00 00        	movaps	xmmword ptr [rbp + 0x220], xmm1
1404ecbf4: 0f 29 85 10 02 00 00        	movaps	xmmword ptr [rbp + 0x210], xmm0
1404ecbfb: 4c 8d b5 10 02 00 00        	lea	r14, [rbp + 0x210]
1404ecc02: 48 89 d9                    	mov	rcx, rbx
1404ecc05: 48 89 fa                    	mov	rdx, rdi
1404ecc08: 4d 89 e8                    	mov	r8, r13
1404ecc0b: 4d 89 f1                    	mov	r9, r14
1404ecc0e: e8 ad 78 3b 00              	call	0x1408a44c0 <.text+0x8a34c0>
1404ecc13: 4c 8d bd 10 04 00 00        	lea	r15, [rbp + 0x410]
1404ecc1a: 4c 89 f9                    	mov	rcx, r15
1404ecc1d: 48 89 da                    	mov	rdx, rbx
1404ecc20: 49 89 f8                    	mov	r8, rdi
1404ecc23: 4d 89 f1                    	mov	r9, r14
1404ecc26: e8 f5 d3 56 00              	call	0x140a5a020 <.text+0xa59020>
1404ecc2b: 48 ba 28 64 1a c9 41 e9 b8 c2       	movabs	rdx, -0x3d4716be36e59bd8
1404ecc35: 49 b8 a0 43 da d3 d1 c6 f3 59       	movabs	r8, 0x59f3c6d1d3da43a0
1404ecc3f: 48 8b 8d d8 03 00 00        	mov	rcx, qword ptr [rbp + 0x3d8]
1404ecc46: e8 a5 0a bf ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1404ecc4b: 48 85 c0                    	test	rax, rax
1404ecc4e: 4c 8b a5 b8 02 00 00        	mov	r12, qword ptr [rbp + 0x2b8]
1404ecc55: 0f 84 6c 04 00 00           	je	0x1404ed0c7 <.text+0x4ec0c7>
1404ecc5b: 48 39 97 18 02 00 00        	cmp	qword ptr [rdi + 0x218], rdx
1404ecc62: 0f 86 5f 04 00 00           	jbe	0x1404ed0c7 <.text+0x4ec0c7>
1404ecc68: 48 8b 87 10 02 00 00        	mov	rax, qword ptr [rdi + 0x210]
1404ecc6f: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
1404ecc73: 48 85 c9                    	test	rcx, rcx
1404ecc76: 0f 84 4b 04 00 00           	je	0x1404ed0c7 <.text+0x4ec0c7>
1404ecc7c: 48 8b 87 e0 01 00 00        	mov	rax, qword ptr [rdi + 0x1e0]
1404ecc83: 48 f7 d1                    	not	rcx
1404ecc86: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1404ecc8a: 48 c1 e1 05                 	shl	rcx, 0x5
1404ecc8e: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
1404ecc94: 0f 84 2d 04 00 00           	je	0x1404ed0c7 <.text+0x4ec0c7>
1404ecc9a: 48 01 c8                    	add	rax, rcx
1404ecc9d: 48 8b 40 10                 	mov	rax, qword ptr [rax + 0x10]
1404ecca1: 8b 30                       	mov	esi, dword ptr [rax]
1404ecca3: 31 c0                       	xor	eax, eax
1404ecca5: 83 fe 13                    	cmp	esi, 0x13
1404ecca8: 0f 94 c0                    	sete	al
1404eccab: 48 8b 8d e8 04 00 00        	mov	rcx, qword ptr [rbp + 0x4e8]
1404eccb2: 80 bc 08 a9 00 00 00 00     	cmp	byte ptr [rax + rcx + 0xa9], 0x0
1404eccba: 0f 84 7e 01 00 00           	je	0x1404ece3e <.text+0x4ebe3e>
1404eccc0: 4d 8d 45 04                 	lea	r8, [r13 + 0x4]
1404eccc4: 48 8d 05 8d f3 4f 02        	lea	rax, [rip + 0x24ff38d]  # 0x1429ec058
1404ecccb: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1404eccd0: 48 c7 44 24 28 08 00 00 00  	mov	qword ptr [rsp + 0x28], 0x8
1404eccd9: 48 89 d9                    	mov	rcx, rbx
1404eccdc: 48 89 fa                    	mov	rdx, rdi
1404eccdf: 4d 89 e9                    	mov	r9, r13
1404ecce2: e8 a9 eb c0 ff              	call	0x1400fb890 <.text+0xfa890>
1404ecce7: 83 fe 13                    	cmp	esi, 0x13
1404eccea: 0f 94 44 24 30              	sete	byte ptr [rsp + 0x30]
1404eccef: 4c 89 7c 24 28              	mov	qword ptr [rsp + 0x28], r15
1404eccf4: 4c 89 74 24 20              	mov	qword ptr [rsp + 0x20], r14
1404eccf9: 48 8d 8d e0 02 00 00        	lea	rcx, [rbp + 0x2e0]
1404ecd00: 48 89 da                    	mov	rdx, rbx
1404ecd03: 49 89 f8                    	mov	r8, rdi
1404ecd06: 4d 89 e9                    	mov	r9, r13
1404ecd09: e8 62 d8 56 00              	call	0x140a5a570 <.text+0xa59570>
1404ecd0e: 80 bd 6e 04 00 00 00        	cmp	byte ptr [rbp + 0x46e], 0x0
1404ecd15: 0f 85 52 01 00 00           	jne	0x1404ece6d <.text+0x4ebe6d>
1404ecd1b: 48 8d 8d e4 02 00 00        	lea	rcx, [rbp + 0x2e4]
1404ecd22: e8 69 8a 6e 00              	call	0x140bd5790 <.text+0xbd4790>
1404ecd27: 48 8d 8d c0 02 00 00        	lea	rcx, [rbp + 0x2c0]
1404ecd2e: 48 89 c2                    	mov	rdx, rax
1404ecd31: 41 b8 7a 00 00 00           	mov	r8d, 0x7a
1404ecd37: e8 64 85 6e 00              	call	0x140bd52a0 <.text+0xbd42a0>
1404ecd3c: 0f 10 85 c0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2c0]
1404ecd43: 0f 10 8d d0 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x2d0]
1404ecd4a: 0f 29 8d 20 03 00 00        	movaps	xmmword ptr [rbp + 0x320], xmm1
1404ecd51: 0f 29 85 10 03 00 00        	movaps	xmmword ptr [rbp + 0x310], xmm0
1404ecd58: 48 8d 8d f0 03 00 00        	lea	rcx, [rbp + 0x3f0]
1404ecd5f: 48 8d 95 10 03 00 00        	lea	rdx, [rbp + 0x310]
1404ecd66: e8 75 9f 6e 00              	call	0x140bd6ce0 <.text+0xbd5ce0>
1404ecd6b: 48 8d 05 ee f2 4f 02        	lea	rax, [rip + 0x24ff2ee]  # 0x1429ec060
1404ecd72: 48 89 85 e0 03 00 00        	mov	qword ptr [rbp + 0x3e0], rax
1404ecd79: 48 c7 85 e8 03 00 00 0a 00 00 00    	mov	qword ptr [rbp + 0x3e8], 0xa
1404ecd84: 48 8d b5 e0 03 00 00        	lea	rsi, [rbp + 0x3e0]
1404ecd8b: 48 89 d9                    	mov	rcx, rbx
1404ecd8e: 48 89 f2                    	mov	rdx, rsi
1404ecd91: e8 3a c7 6f 00              	call	0x140be94d0 <.text+0xbe84d0>
1404ecd96: 0f 10 85 c0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2c0]
1404ecd9d: 0f 10 8d d0 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x2d0]
1404ecda4: 0f 11 85 40 01 00 00        	movups	xmmword ptr [rbp + 0x140], xmm0
1404ecdab: 0f 11 8d 50 01 00 00        	movups	xmmword ptr [rbp + 0x150], xmm1
1404ecdb2: 89 85 3c 01 00 00           	mov	dword ptr [rbp + 0x13c], eax
1404ecdb8: 48 8b 85 e8 04 00 00        	mov	rax, qword ptr [rbp + 0x4e8]
1404ecdbf: f3 0f 10 80 9c 00 00 00     	movss	xmm0, dword ptr [rax + 0x9c]
1404ecdc7: 48 8b 85 d0 03 00 00        	mov	rax, qword ptr [rbp + 0x3d0]
1404ecdce: 48 89 85 00 04 00 00        	mov	qword ptr [rbp + 0x400], rax
1404ecdd5: 0f 28 8d b0 03 00 00        	movaps	xmm1, xmmword ptr [rbp + 0x3b0]
1404ecddc: 0f 28 95 c0 03 00 00        	movaps	xmm2, xmmword ptr [rbp + 0x3c0]
1404ecde3: 0f 29 95 f0 03 00 00        	movaps	xmmword ptr [rbp + 0x3f0], xmm2
1404ecdea: 0f 29 8d e0 03 00 00        	movaps	xmmword ptr [rbp + 0x3e0], xmm1
1404ecdf1: f3 0f 11 44 24 40           	movss	dword ptr [rsp + 0x40], xmm0
1404ecdf7: 48 89 74 24 38              	mov	qword ptr [rsp + 0x38], rsi
1404ecdfc: 48 8d 85 b0 01 00 00        	lea	rax, [rbp + 0x1b0]
1404ece03: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
1404ece08: 48 8d 85 8c 01 00 00        	lea	rax, [rbp + 0x18c]
1404ece0f: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
1404ece14: 48 8d 85 3c 01 00 00        	lea	rax, [rbp + 0x13c]
1404ece1b: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1404ece20: 48 8d 8d 10 03 00 00        	lea	rcx, [rbp + 0x310]
1404ece27: 4c 8d 8d e0 02 00 00        	lea	r9, [rbp + 0x2e0]
1404ece2e: 48 89 da                    	mov	rdx, rbx
1404ece31: 49 89 f8                    	mov	r8, rdi
1404ece34: e8 b7 e4 54 00              	call	0x140a3b2f0 <.text+0xa3a2f0>
1404ece39: e9 b6 01 00 00              	jmp	0x1404ecff4 <.text+0x4ebff4>
1404ece3e: 41 8b 45 20                 	mov	eax, dword ptr [r13 + 0x20]
1404ece42: 89 85 00 03 00 00           	mov	dword ptr [rbp + 0x300], eax
1404ece48: 41 0f 10 45 00              	movups	xmm0, xmmword ptr [r13]
1404ece4d: 41 0f 10 4d 10              	movups	xmm1, xmmword ptr [r13 + 0x10]
1404ece52: 0f 29 8d f0 02 00 00        	movaps	xmmword ptr [rbp + 0x2f0], xmm1
1404ece59: 0f 29 85 e0 02 00 00        	movaps	xmmword ptr [rbp + 0x2e0], xmm0
1404ece60: 80 bd 6e 04 00 00 00        	cmp	byte ptr [rbp + 0x46e], 0x0
1404ece67: 0f 84 ae fe ff ff           	je	0x1404ecd1b <.text+0x4ebd1b>
1404ece6d: 48 ba 8b 04 7b e6 61 26 c0 a4       	movabs	rdx, -0x5b3fd99e1984fb75
1404ece77: 49 b8 ec 67 2a aa e6 61 e7 ea       	movabs	r8, -0x15189e1955d59814
1404ece81: 48 8b 8d d8 03 00 00        	mov	rcx, qword ptr [rbp + 0x3d8]
1404ece88: e8 63 08 bf ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1404ece8d: 48 85 c0                    	test	rax, rax
1404ece90: 0f 84 49 02 00 00           	je	0x1404ed0df <.text+0x4ec0df>
1404ece96: 48 39 97 18 02 00 00        	cmp	qword ptr [rdi + 0x218], rdx
1404ece9d: 0f 86 3c 02 00 00           	jbe	0x1404ed0df <.text+0x4ec0df>
1404ecea3: 48 8b 87 10 02 00 00        	mov	rax, qword ptr [rdi + 0x210]
1404eceaa: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
1404eceae: 48 85 c9                    	test	rcx, rcx
1404eceb1: 0f 84 28 02 00 00           	je	0x1404ed0df <.text+0x4ec0df>
1404eceb7: 48 8b 87 e0 01 00 00        	mov	rax, qword ptr [rdi + 0x1e0]
1404ecebe: 48 f7 d1                    	not	rcx
1404ecec1: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1404ecec5: 48 c1 e1 05                 	shl	rcx, 0x5
1404ecec9: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
1404ececf: 0f 84 0a 02 00 00           	je	0x1404ed0df <.text+0x4ec0df>
1404eced5: 48 01 c8                    	add	rax, rcx
1404eced8: 48 8b 40 10                 	mov	rax, qword ptr [rax + 0x10]
1404ecedc: 48 8b 50 08                 	mov	rdx, qword ptr [rax + 0x8]
1404ecee0: 48 8d 8d c0 02 00 00        	lea	rcx, [rbp + 0x2c0]
1404ecee7: 41 b8 7a 00 00 00           	mov	r8d, 0x7a
1404eceed: e8 ae 83 6e 00              	call	0x140bd52a0 <.text+0xbd42a0>
1404ecef2: 0f 10 85 c0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2c0]
1404ecef9: 0f 10 8d d0 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x2d0]
1404ecf00: 0f 29 8d 20 03 00 00        	movaps	xmmword ptr [rbp + 0x320], xmm1
1404ecf07: 0f 29 85 10 03 00 00        	movaps	xmmword ptr [rbp + 0x310], xmm0
1404ecf0e: 48 8d 8d f0 03 00 00        	lea	rcx, [rbp + 0x3f0]
1404ecf15: 48 8d 95 10 03 00 00        	lea	rdx, [rbp + 0x310]
1404ecf1c: e8 bf 9d 6e 00              	call	0x140bd6ce0 <.text+0xbd5ce0>
1404ecf21: 48 8d 05 38 f1 4f 02        	lea	rax, [rip + 0x24ff138]  # 0x1429ec060
1404ecf28: 48 89 85 e0 03 00 00        	mov	qword ptr [rbp + 0x3e0], rax
1404ecf2f: 48 c7 85 e8 03 00 00 0a 00 00 00    	mov	qword ptr [rbp + 0x3e8], 0xa
1404ecf3a: 48 8d b5 e0 03 00 00        	lea	rsi, [rbp + 0x3e0]
1404ecf41: 48 89 d9                    	mov	rcx, rbx
1404ecf44: 48 89 f2                    	mov	rdx, rsi
1404ecf47: e8 84 c5 6f 00              	call	0x140be94d0 <.text+0xbe84d0>
1404ecf4c: 0f 10 85 c0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2c0]
1404ecf53: 0f 10 8d d0 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x2d0]
1404ecf5a: 0f 11 85 1c 01 00 00        	movups	xmmword ptr [rbp + 0x11c], xmm0
1404ecf61: 0f 11 8d 2c 01 00 00        	movups	xmmword ptr [rbp + 0x12c], xmm1
1404ecf68: 89 85 18 01 00 00           	mov	dword ptr [rbp + 0x118], eax
1404ecf6e: 48 8b 85 e8 04 00 00        	mov	rax, qword ptr [rbp + 0x4e8]
1404ecf75: f3 0f 10 80 9c 00 00 00     	movss	xmm0, dword ptr [rax + 0x9c]
1404ecf7d: 48 8b 85 d0 03 00 00        	mov	rax, qword ptr [rbp + 0x3d0]
1404ecf84: 48 89 85 00 04 00 00        	mov	qword ptr [rbp + 0x400], rax
1404ecf8b: 0f 28 8d b0 03 00 00        	movaps	xmm1, xmmword ptr [rbp + 0x3b0]
1404ecf92: 0f 28 95 c0 03 00 00        	movaps	xmm2, xmmword ptr [rbp + 0x3c0]
1404ecf99: 0f 29 95 f0 03 00 00        	movaps	xmmword ptr [rbp + 0x3f0], xmm2
1404ecfa0: 0f 29 8d e0 03 00 00        	movaps	xmmword ptr [rbp + 0x3e0], xmm1
1404ecfa7: f3 0f 11 44 24 48           	movss	dword ptr [rsp + 0x48], xmm0
1404ecfad: 48 89 74 24 40              	mov	qword ptr [rsp + 0x40], rsi
1404ecfb2: 48 8d 85 b0 01 00 00        	lea	rax, [rbp + 0x1b0]
1404ecfb9: 48 89 44 24 38              	mov	qword ptr [rsp + 0x38], rax
1404ecfbe: 48 8d 85 8c 01 00 00        	lea	rax, [rbp + 0x18c]
1404ecfc5: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
1404ecfca: 48 8d 85 18 01 00 00        	lea	rax, [rbp + 0x118]
1404ecfd1: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
1404ecfd6: 4c 89 74 24 20              	mov	qword ptr [rsp + 0x20], r14
1404ecfdb: 48 8d 8d 10 03 00 00        	lea	rcx, [rbp + 0x310]
1404ecfe2: 4c 8d 8d e0 02 00 00        	lea	r9, [rbp + 0x2e0]
1404ecfe9: 48 89 da                    	mov	rdx, rbx
1404ecfec: 49 89 f8                    	mov	r8, rdi
1404ecfef: e8 6c ea 54 00              	call	0x140a3ba60 <.text+0xa3aa60>
1404ecff4: 48 8d b5 10 03 00 00        	lea	rsi, [rbp + 0x310]
1404ecffb: 48 89 d9                    	mov	rcx, rbx
1404ecffe: 48 89 fa                    	mov	rdx, rdi
1404ed001: 49 89 f0                    	mov	r8, rsi
1404ed004: 4c 8b 8d 60 04 00 00        	mov	r9, qword ptr [rbp + 0x460]
1404ed00b: e8 a0 70 4d 00              	call	0x1409c40b0 <.text+0x9c30b0>
1404ed010: 48 89 d9                    	mov	rcx, rbx
1404ed013: 48 89 fa                    	mov	rdx, rdi
1404ed016: 49 89 f0                    	mov	r8, rsi
1404ed019: e8 d2 29 4a 00              	call	0x14098f9f0 <.text+0x98e9f0>
1404ed01e: 8b 85 30 04 00 00           	mov	eax, dword ptr [rbp + 0x430]
1404ed024: 41 89 44 24 44              	mov	dword ptr [r12 + 0x44], eax
1404ed029: 0f 10 85 10 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x410]
1404ed030: 0f 10 8d 20 04 00 00        	movups	xmm1, xmmword ptr [rbp + 0x420]
1404ed037: 41 0f 11 4c 24 34           	movups	xmmword ptr [r12 + 0x34], xmm1
1404ed03d: 41 0f 11 44 24 24           	movups	xmmword ptr [r12 + 0x24], xmm0
1404ed043: 0f 10 85 10 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x310]
1404ed04a: 0f 10 8d 20 03 00 00        	movups	xmm1, xmmword ptr [rbp + 0x320]
1404ed051: 41 0f 11 44 24 48           	movups	xmmword ptr [r12 + 0x48], xmm0
1404ed057: 41 0f 11 4c 24 58           	movups	xmmword ptr [r12 + 0x58], xmm1
1404ed05d: 8b 85 30 03 00 00           	mov	eax, dword ptr [rbp + 0x330]
1404ed063: 41 89 44 24 68              	mov	dword ptr [r12 + 0x68], eax
1404ed068: 0f 28 85 10 02 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x210]
1404ed06f: 0f 28 8d 20 02 00 00        	movaps	xmm1, xmmword ptr [rbp + 0x220]
1404ed076: 41 0f 11 04 24              	movups	xmmword ptr [r12], xmm0
1404ed07b: 41 0f 11 4c 24 10           	movups	xmmword ptr [r12 + 0x10], xmm1
1404ed081: 8b 85 30 02 00 00           	mov	eax, dword ptr [rbp + 0x230]
1404ed087: 41 89 44 24 20              	mov	dword ptr [r12 + 0x20], eax
1404ed08c: 4c 89 e0                    	mov	rax, r12
1404ed08f: 48 81 c4 f8 04 00 00        	add	rsp, 0x4f8
1404ed096: 5b                          	pop	rbx
1404ed097: 5f                          	pop	rdi
1404ed098: 5e                          	pop	rsi
1404ed099: 41 5c                       	pop	r12
1404ed09b: 41 5d                       	pop	r13
1404ed09d: 41 5e                       	pop	r14
1404ed09f: 41 5f                       	pop	r15
1404ed0a1: 5d                          	pop	rbp
1404ed0a2: c3                          	ret
1404ed0a3: 48 8d 0d 76 ee 4f 02        	lea	rcx, [rip + 0x24fee76]  # 0x1429ebf20
1404ed0aa: 4c 8d 05 b7 ee 4f 02        	lea	r8, [rip + 0x24feeb7]   # 0x1429ebf68
1404ed0b1: ba 46 00 00 00              	mov	edx, 0x46
1404ed0b6: e8 75 c3 3e 02              	call	0x1428d9430 <.text+0x28d8430>
1404ed0bb: 48 8d 0d be ee 4f 02        	lea	rcx, [rip + 0x24feebe]  # 0x1429ebf80
1404ed0c2: e8 49 c2 3e 02              	call	0x1428d9310 <.text+0x28d8310>
1404ed0c7: 48 8d 0d 72 ef 4f 02        	lea	rcx, [rip + 0x24fef72]  # 0x1429ec040
1404ed0ce: e8 3d c2 3e 02              	call	0x1428d9310 <.text+0x28d8310>
1404ed0d3: 48 8d 0d 0e ef 4f 02        	lea	rcx, [rip + 0x24fef0e]  # 0x1429ebfe8
1404ed0da: e8 31 c2 3e 02              	call	0x1428d9310 <.text+0x28d8310>
1404ed0df: 48 8d 0d 8a ef 4f 02        	lea	rcx, [rip + 0x24fef8a]  # 0x1429ec070
1404ed0e6: e8 25 c2 3e 02              	call	0x1428d9310 <.text+0x28d8310>
1404ed0eb: 48 8b 85 40 03 00 00        	mov	rax, qword ptr [rbp + 0x340]
1404ed0f2: 48 89 85 40 02 00 00        	mov	qword ptr [rbp + 0x240], rax
1404ed0f9: 48 8d 05 b0 ee 4f 02        	lea	rax, [rip + 0x24feeb0]  # 0x1429ebfb0
1404ed100: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1404ed105: 48 8d 0d 94 e7 4f 02        	lea	rcx, [rip + 0x24fe794]  # 0x1429eb8a0
1404ed10c: 4c 8d 0d 6d e7 4f 02        	lea	r9, [rip + 0x24fe76d]   # 0x1429eb880
1404ed113: 4c 8d 85 40 02 00 00        	lea	r8, [rbp + 0x240]
1404ed11a: ba 2b 00 00 00              	mov	edx, 0x2b
1404ed11f: e8 3c c6 3e 02              	call	0x1428d9760 <.text+0x28d8760>
1404ed124: eb 39                       	jmp	0x1404ed15f <.text+0x4ec15f>
1404ed126: 48 8b 85 48 03 00 00        	mov	rax, qword ptr [rbp + 0x348]
1404ed12d: 48 89 85 10 04 00 00        	mov	qword ptr [rbp + 0x410], rax
1404ed134: 48 8d 05 d5 ee 4f 02        	lea	rax, [rip + 0x24feed5]  # 0x1429ec010
1404ed13b: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1404ed140: 48 8d 0d 59 e7 4f 02        	lea	rcx, [rip + 0x24fe759]  # 0x1429eb8a0
1404ed147: 4c 8d 0d 32 e7 4f 02        	lea	r9, [rip + 0x24fe732]   # 0x1429eb880
1404ed14e: 4c 8d 85 10 04 00 00        	lea	r8, [rbp + 0x410]
1404ed155: ba 2b 00 00 00              	mov	edx, 0x2b
1404ed15a: e8 01 c6 3e 02              	call	0x1428d9760 <.text+0x28d8760>
1404ed15f: 0f 0b                       	ud2
1404ed161: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
1404ed170: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1404ed175: 55                          	push	rbp
1404ed176: 41 57                       	push	r15
1404ed178: 41 56                       	push	r14
1404ed17a: 41 55                       	push	r13
1404ed17c: 41 54                       	push	r12
1404ed17e: 56                          	push	rsi
1404ed17f: 57                          	push	rdi
1404ed180: 53                          	push	rbx
1404ed181: 48 83 ec 68                 	sub	rsp, 0x68
1404ed185: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1404ed18c: 48 8d 8d 40 02 00 00        	lea	rcx, [rbp + 0x240]
1404ed193: e8 98 fa 85 00              	call	0x140d4cc30 <.text+0xd4bc30>
1404ed198: 90                          	nop
1404ed199: 48 83 c4 68                 	add	rsp, 0x68
1404ed19d: 5b                          	pop	rbx
1404ed19e: 5f                          	pop	rdi
1404ed19f: 5e                          	pop	rsi
1404ed1a0: 41 5c                       	pop	r12
1404ed1a2: 41 5d                       	pop	r13
1404ed1a4: 41 5e                       	pop	r14
1404ed1a6: 41 5f                       	pop	r15
1404ed1a8: 5d                          	pop	rbp
1404ed1a9: c3                          	ret
1404ed1aa: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1404ed1b0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1404ed1b5: 55                          	push	rbp
1404ed1b6: 41 57                       	push	r15
1404ed1b8: 41 56                       	push	r14
1404ed1ba: 41 55                       	push	r13
1404ed1bc: 41 54                       	push	r12
1404ed1be: 56                          	push	rsi
1404ed1bf: 57                          	push	rdi
1404ed1c0: 53                          	push	rbx
1404ed1c1: 48 83 ec 68                 	sub	rsp, 0x68
1404ed1c5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1404ed1cc: 48 8d 8d 10 04 00 00        	lea	rcx, [rbp + 0x410]
1404ed1d3: e8 58 fa 85 00              	call	0x140d4cc30 <.text+0xd4bc30>
1404ed1d8: 90                          	nop
1404ed1d9: 48 83 c4 68                 	add	rsp, 0x68
1404ed1dd: 5b                          	pop	rbx
1404ed1de: 5f                          	pop	rdi
1404ed1df: 5e                          	pop	rsi
1404ed1e0: 41 5c                       	pop	r12
1404ed1e2: 41 5d                       	pop	r13
1404ed1e4: 41 5e                       	pop	r14
1404ed1e6: 41 5f                       	pop	r15
1404ed1e8: 5d                          	pop	rbp
1404ed1e9: c3                          	ret
