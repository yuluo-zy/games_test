0x140b68e40: push rbp
0x140b68e41: push rsi
0x140b68e42: push rdi
0x140b68e43: push rbx
0x140b68e44: sub rsp, 0x3f8
0x140b68e4b: lea rbp, [rsp + 0x80]
0x140b68e53: mov qword ptr [rbp + 0x370], 0xfffffffffffffffe
0x140b68e5e: mov rsi, rdx
0x140b68e61: xorps xmm0, xmm0
0x140b68e64: movups xmmword ptr [rbp - 0x38], xmm0
0x140b68e68: movups xmmword ptr [rbp - 0x20], xmm0
0x140b68e6c: mov qword ptr [rbp - 0x60], 1
0x140b68e74: mov qword ptr [rbp - 0x58], 1
0x140b68e7c: mov byte ptr [rbp - 0x50], 0
0x140b68e80: mov qword ptr [rbp - 0x48], 0
0x140b68e88: mov qword ptr [rbp - 0x40], 8
0x140b68e90: mov qword ptr [rbp - 0x28], 8
0x140b68e98: movzx eax, byte ptr [rip + 0x2acbd6a]
0x140b68e9f: mov ecx, 0x50
0x140b68ea4: mov edx, 8
0x140b68ea9: call 0x140613c10
0x140b68eae: test rax, rax
0x140b68eb1: je 0x140b69455
0x140b68eb7: movups xmm0, xmmword ptr [rbp - 0x20]
0x140b68ebb: movups xmmword ptr [rax + 0x40], xmm0
0x140b68ebf: movups xmm0, xmmword ptr [rbp - 0x60]
0x140b68ec3: movups xmm1, xmmword ptr [rbp - 0x50]
0x140b68ec7: movups xmm2, xmmword ptr [rbp - 0x40]
0x140b68ecb: movups xmm3, xmmword ptr [rbp - 0x30]
0x140b68ecf: movups xmmword ptr [rax + 0x30], xmm3
0x140b68ed3: movups xmmword ptr [rax + 0x20], xmm2
0x140b68ed7: movups xmmword ptr [rax + 0x10], xmm1
0x140b68edb: movups xmmword ptr [rax], xmm0
0x140b68ede: mov qword ptr [rbp + 0x2c0], rax
0x140b68ee5: movabs rax, 0x8000000000000000
0x140b68eef: mov qword ptr [rbp + 0x2b8], rax
0x140b68ef6: mov byte ptr [rbp + 0x36f], 1
0x140b68efd: lea rcx, [rbp - 0x60]
0x140b68f01: call 0x140b4abb0
0x140b68f06: mov byte ptr [rbp + 0x2f0], 0x10
0x140b68f0d: mov rcx, qword ptr [rip + 0x2ac345c]
0x140b68f14: lea r8, [rip + 0x20f6e45]
0x140b68f1b: lea rdx, [rbp + 0x2f0]
0x140b68f22: call 0x140b218a0
0x140b68f27: mov r8, rdx
0x140b68f2a: lea rcx, [rbp - 0x60]
0x140b68f2e: mov rdx, rax
0x140b68f31: call 0x141453000
0x140b68f36: movups xmm0, xmmword ptr [rbp]
0x140b68f3a: movaps xmmword ptr [rbp + 0x2a0], xmm0
0x140b68f41: movups xmm0, xmmword ptr [rbp - 0x10]
0x140b68f45: movaps xmmword ptr [rbp + 0x290], xmm0
0x140b68f4c: movups xmm0, xmmword ptr [rbp - 0x20]
0x140b68f50: movaps xmmword ptr [rbp + 0x280], xmm0
0x140b68f57: movups xmm0, xmmword ptr [rbp - 0x60]
0x140b68f5b: movups xmm1, xmmword ptr [rbp - 0x50]
0x140b68f5f: movups xmm2, xmmword ptr [rbp - 0x40]
0x140b68f63: movups xmm3, xmmword ptr [rbp - 0x30]
0x140b68f67: movaps xmmword ptr [rbp + 0x270], xmm3
0x140b68f6e: movaps xmmword ptr [rbp + 0x260], xmm2
0x140b68f75: movaps xmmword ptr [rbp + 0x250], xmm1
0x140b68f7c: movaps xmmword ptr [rbp + 0x240], xmm0
0x140b68f83: lea r8, [rip + 0x1f8a04e]
0x140b68f8a: lea rcx, [rbp + 0x2f0]
0x140b68f91: mov dl, 0x22
0x140b68f93: call 0x1409cdc40
0x140b68f98: movaps xmm0, xmmword ptr [rbp + 0x2a0]
0x140b68f9f: movaps xmmword ptr [rbp], xmm0
0x140b68fa3: movaps xmm0, xmmword ptr [rbp + 0x290]
0x140b68faa: movaps xmmword ptr [rbp - 0x10], xmm0
0x140b68fae: movaps xmm0, xmmword ptr [rbp + 0x280]
0x140b68fb5: movaps xmmword ptr [rbp - 0x20], xmm0
0x140b68fb9: movaps xmm0, xmmword ptr [rbp + 0x240]
0x140b68fc0: movaps xmm1, xmmword ptr [rbp + 0x250]
0x140b68fc7: movaps xmm2, xmmword ptr [rbp + 0x260]
0x140b68fce: movaps xmm3, xmmword ptr [rbp + 0x270]
0x140b68fd5: movaps xmmword ptr [rbp - 0x30], xmm3
0x140b68fd9: movaps xmmword ptr [rbp - 0x40], xmm2
0x140b68fdd: movaps xmmword ptr [rbp - 0x50], xmm1
0x140b68fe1: movaps xmmword ptr [rbp - 0x60], xmm0
0x140b68fe5: movups xmm0, xmmword ptr [rbp + 0x2f0]
0x140b68fec: movups xmm1, xmmword ptr [rbp + 0x300]
0x140b68ff3: movups xmm2, xmmword ptr [rbp + 0x310]
0x140b68ffa: movups xmm3, xmmword ptr [rbp + 0x320]
0x140b69001: movaps xmmword ptr [rbp + 0x10], xmm0
0x140b69005: movaps xmmword ptr [rbp + 0x20], xmm1
0x140b69009: movaps xmmword ptr [rbp + 0x30], xmm2
0x140b6900d: movaps xmmword ptr [rbp + 0x40], xmm3
0x140b69011: movups xmm0, xmmword ptr [rbp + 0x330]
0x140b69018: movaps xmmword ptr [rbp + 0x50], xmm0
0x140b6901c: movups xmm0, xmmword ptr [rbp + 0x340]
0x140b69023: movaps xmmword ptr [rbp + 0x60], xmm0
0x140b69027: movups xmm0, xmmword ptr [rbp + 0x350]
0x140b6902e: movaps xmmword ptr [rbp + 0x70], xmm0
0x140b69032: mov byte ptr [rbp + 0x36f], 1
0x140b69039: lea rdx, [rbp - 0x60]
0x140b6903d: mov rcx, rsi
0x140b69040: call 0x140b679c0
0x140b69045: movabs rax, 0x8000000000000002
0x140b6904f: cmp qword ptr [rbp + 0x2b8], rax
0x140b69056: jge 0x140b69466
0x140b6905c: mov rdi, qword ptr [rbp + 0x2c0]
0x140b69063: lock inc qword ptr [rdi]
0x140b69067: jle 0x140b6949f
0x140b6906d: mov qword ptr [rbp + 0x2f8], rdi
0x140b69074: movabs rbx, 0x8000000000000001
0x140b6907e: mov qword ptr [rbp + 0x2f0], rbx
0x140b69085: mov byte ptr [rbp + 0x36f], 1
0x140b6908c: lea rcx, [rbp - 0x60]
0x140b69090: lea rdx, [rbp + 0x2f0]
0x140b69097: call 0x140a15530
0x140b6909c: mov byte ptr [rbp + 0x36f], 1
0x140b690a3: lea rdx, [rbp - 0x60]
0x140b690a7: mov rcx, rsi
0x140b690aa: call 0x14153f1f0
0x140b690af: lock inc qword ptr [rdi]
0x140b690b3: jle 0x140b6949f
0x140b690b9: mov rsi, rax
0x140b690bc: mov qword ptr [rbp + 0x2f8], rdi
0x140b690c3: mov qword ptr [rbp + 0x2f0], rbx
0x140b690ca: mov byte ptr [rbp + 0x36f], 1
0x140b690d1: lea rcx, [rbp - 0x60]
0x140b690d5: lea rdx, [rbp + 0x2f0]
0x140b690dc: call 0x140a15530
0x140b690e1: mov byte ptr [rbp + 0x36f], 1
0x140b690e8: lea rdx, [rbp - 0x60]
0x140b690ec: mov rcx, rsi
0x140b690ef: call 0x141536570
0x140b690f4: lock inc qword ptr [rdi]
0x140b690f8: jle 0x140b6949f
0x140b690fe: mov rsi, rax
0x140b69101: mov qword ptr [rbp + 0x2f8], rdi
0x140b69108: mov qword ptr [rbp + 0x2f0], rbx
0x140b6910f: mov byte ptr [rbp + 0x36f], 1
0x140b69116: lea rcx, [rbp - 0x60]
0x140b6911a: lea rdx, [rbp + 0x2f0]
0x140b69121: call 0x140a15530
0x140b69126: mov byte ptr [rbp + 0x36f], 1
0x140b6912d: lea rdx, [rbp - 0x60]
0x140b69131: mov rcx, rsi
0x140b69134: call 0x141532d50
0x140b69139: mov rsi, rax
0x140b6913c: mov rax, qword ptr [rbp + 0x2e8]
0x140b69143: mov qword ptr [rbp + 0x320], rax
0x140b6914a: movups xmm0, xmmword ptr [rbp + 0x2b8]
0x140b69151: movups xmm1, xmmword ptr [rbp + 0x2c8]
0x140b69158: movups xmm2, xmmword ptr [rbp + 0x2d8]
0x140b6915f: movaps xmmword ptr [rbp + 0x310], xmm2
0x140b69166: movaps xmmword ptr [rbp + 0x300], xmm1
0x140b6916d: movaps xmmword ptr [rbp + 0x2f0], xmm0
0x140b69174: mov byte ptr [rbp + 0x36f], 0
0x140b6917b: lea rcx, [rbp - 0x60]
0x140b6917f: lea rdx, [rbp + 0x2f0]
0x140b69186: call 0x140a15530
0x140b6918b: mov byte ptr [rbp + 0x36f], 0
0x140b69192: lea rdx, [rbp - 0x60]
0x140b69196: mov rcx, rsi
0x140b69199: call 0x1415381b0
0x140b6919e: movsd xmm0, qword ptr [rip + 0x1f5c52a]
0x140b691a6: movsd qword ptr [rbp - 0x60], xmm0
0x140b691ab: mov dword ptr [rbp - 0x58], 0x40400000
0x140b691b2: mov byte ptr [rbp + 0x36f], 0
0x140b691b9: lea rdx, [rbp - 0x60]
0x140b691bd: mov rcx, rax
0x140b691c0: call 0x141530010
0x140b691c5: mov rsi, rax
0x140b691c8: lea rdi, [rax + 0x38]
0x140b691cc: mov byte ptr [rbp + 0x36f], 0
0x140b691d3: movabs rdx, 0x7ace8413c35ae5af
0x140b691dd: movabs r8, 0x6c2ea24e70d9c7de
0x140b691e7: mov rcx, rdi
0x140b691ea: call 0x1400dd6f0
0x140b691ef: test rax, rax
0x140b691f2: je 0x140b69227
0x140b691f4: cmp qword ptr [rsi + 0x218], rdx
0x140b691fb: jbe 0x140b69227
0x140b691fd: mov rax, qword ptr [rsi + 0x210]
0x140b69204: mov rax, qword ptr [rax + rdx*8]
0x140b69208: test rax, rax
0x140b6920b: je 0x140b69227
0x140b6920d: mov rcx, qword ptr [rsi + 0x1e0]
0x140b69214: not rax
0x140b69217: lea rax, [rax + rax*2]
0x140b6921b: shl rax, 5
0x140b6921f: cmp qword ptr [rcx + rax + 0x20], 0
0x140b69225: jne 0x140b69236
0x140b69227: mov byte ptr [rbp + 0x36f], 0
0x140b6922e: mov rcx, rsi
0x140b69231: call 0x141519050
0x140b69236: mov byte ptr [rbp + 0x36f], 0
0x140b6923d: movabs rdx, 0xec41cbdf0a8d4b33
0x140b69247: movabs r8, 0x3ac066cc65ed9374
0x140b69251: mov rcx, rdi
0x140b69254: call 0x1400dd6f0
0x140b69259: test rax, rax
0x140b6925c: je 0x140b69291
0x140b6925e: cmp qword ptr [rsi + 0x218], rdx
0x140b69265: jbe 0x140b69291
0x140b69267: mov rax, qword ptr [rsi + 0x210]
0x140b6926e: mov rax, qword ptr [rax + rdx*8]
0x140b69272: test rax, rax
0x140b69275: je 0x140b69291
0x140b69277: mov rcx, qword ptr [rsi + 0x1e0]
0x140b6927e: not rax
0x140b69281: lea rax, [rax + rax*2]
0x140b69285: shl rax, 5
0x140b69289: cmp qword ptr [rcx + rax + 0x20], 0
0x140b6928f: jne 0x140b692a0
0x140b69291: mov byte ptr [rbp + 0x36f], 0
0x140b69298: mov rcx, rsi
0x140b6929b: call 0x14151af40
0x140b692a0: mov byte ptr [rbp + 0x36f], 0
0x140b692a7: movabs rdx, 0x5cb0103106a33509
0x140b692b1: movabs r8, 0xaca1b3b29b41a133
0x140b692bb: mov rcx, rdi
0x140b692be: call 0x1400dd6f0
0x140b692c3: test rax, rax
0x140b692c6: je 0x140b692fb
0x140b692c8: cmp qword ptr [rsi + 0x218], rdx
0x140b692cf: jbe 0x140b692fb
0x140b692d1: mov rax, qword ptr [rsi + 0x210]
0x140b692d8: mov rax, qword ptr [rax + rdx*8]
0x140b692dc: test rax, rax
0x140b692df: je 0x140b692fb
0x140b692e1: mov rcx, qword ptr [rsi + 0x1e0]
0x140b692e8: not rax
0x140b692eb: lea rax, [rax + rax*2]
0x140b692ef: shl rax, 5
0x140b692f3: cmp qword ptr [rcx + rax + 0x20], 0
0x140b692f9: jne 0x140b6930a
0x140b692fb: mov byte ptr [rbp + 0x36f], 0
0x140b69302: mov rcx, rsi
0x140b69305: call 0x14150b190
0x140b6930a: mov byte ptr [rbp + 0x36f], 0
0x140b69311: movabs rdx, 0x78dfd58345d30c24
0x140b6931b: movabs r8, 0xcd427e4a8b2594f6
0x140b69325: mov rcx, rdi
0x140b69328: call 0x1400dd6f0
0x140b6932d: test rax, rax
0x140b69330: je 0x140b69365
0x140b69332: cmp qword ptr [rsi + 0x218], rdx
0x140b69339: jbe 0x140b69365
0x140b6933b: mov rax, qword ptr [rsi + 0x210]
0x140b69342: mov rax, qword ptr [rax + rdx*8]
0x140b69346: test rax, rax
0x140b69349: je 0x140b69365
0x140b6934b: mov rcx, qword ptr [rsi + 0x1e0]
0x140b69352: not rax
0x140b69355: lea rax, [rax + rax*2]
0x140b69359: shl rax, 5
0x140b6935d: cmp qword ptr [rcx + rax + 0x20], 0
0x140b69363: jne 0x140b69374
0x140b69365: mov byte ptr [rbp + 0x36f], 0
0x140b6936c: mov rcx, rsi
0x140b6936f: call 0x141516500
0x140b69374: mov byte ptr [rbp + 0x36f], 0
0x140b6937b: movabs rdx, 0x9261d08d6231cff7
0x140b69385: movabs r8, 0xc4a6d1338c2e8f4c
0x140b6938f: mov rcx, rdi
0x140b69392: call 0x1400dd6f0
0x140b69397: test rax, rax
0x140b6939a: je 0x140b693cf
0x140b6939c: cmp qword ptr [rsi + 0x218], rdx
0x140b693a3: jbe 0x140b693cf
0x140b693a5: mov rax, qword ptr [rsi + 0x210]
0x140b693ac: mov rax, qword ptr [rax + rdx*8]
0x140b693b0: test rax, rax
0x140b693b3: je 0x140b693cf
0x140b693b5: mov rcx, qword ptr [rsi + 0x1e0]
0x140b693bc: not rax
0x140b693bf: lea rax, [rax + rax*2]
0x140b693c3: shl rax, 5
0x140b693c7: cmp qword ptr [rcx + rax + 0x20], 0
0x140b693cd: jne 0x140b693de
0x140b693cf: mov byte ptr [rbp + 0x36f], 0
0x140b693d6: mov rcx, rsi
0x140b693d9: call 0x1415183f0
0x140b693de: mov byte ptr [rbp + 0x36f], 0
0x140b693e5: movabs rdx, 0x215298b27cb44d6c
0x140b693ef: movabs r8, 0x42080376a960d87c
0x140b693f9: mov rcx, rdi
0x140b693fc: call 0x1400dd6f0
0x140b69401: test rax, rax
0x140b69404: je 0x140b69439
0x140b69406: cmp qword ptr [rsi + 0x218], rdx
0x140b6940d: jbe 0x140b69439
0x140b6940f: mov rax, qword ptr [rsi + 0x210]
0x140b69416: mov rax, qword ptr [rax + rdx*8]
0x140b6941a: test rax, rax
0x140b6941d: je 0x140b69439
0x140b6941f: mov rcx, qword ptr [rsi + 0x1e0]
0x140b69426: not rax
0x140b69429: lea rax, [rax + rax*2]
0x140b6942d: shl rax, 5
0x140b69431: cmp qword ptr [rcx + rax + 0x20], 0
0x140b69437: jne 0x140b69448
0x140b69439: mov byte ptr [rbp + 0x36f], 0
0x140b69440: mov rcx, rsi
0x140b69443: call 0x14151da90
0x140b69448: nop 
0x140b69449: add rsp, 0x3f8
0x140b69450: pop rbx
0x140b69451: pop rdi
0x140b69452: pop rsi
0x140b69453: pop rbp
0x140b69454: ret 
0x140b69455: mov ecx, 8
0x140b6945a: mov edx, 0x50
0x140b6945f: call 0x1428d9000
0x140b69464: jmp 0x140b6949f
0x140b69466: lea rax, [rip + 0x1f47e8b]
0x140b6946d: mov qword ptr [rbp - 0x60], rax
0x140b69471: mov qword ptr [rbp - 0x58], 1
0x140b69479: mov qword ptr [rbp - 0x50], 8
0x140b69481: xorps xmm0, xmm0
0x140b69484: movups xmmword ptr [rbp - 0x48], xmm0
0x140b69488: mov byte ptr [rbp + 0x36f], 1
0x140b6948f: lea rdx, [rip + 0x1f47eaa]
0x140b69496: lea rcx, [rbp - 0x60]
0x140b6949a: call 0x1428d9390
0x140b6949f: ud2 
0x140b694a1: nop word ptr cs:[rax + rax]
0x140b694b0: mov qword ptr [rsp + 0x10], rdx
0x140b694b5: push rbp
0x140b694b6: push rsi
0x140b694b7: push rdi
0x140b694b8: push rbx
0x140b694b9: sub rsp, 0x28
0x140b694bd: lea rbp, [rdx + 0x80]
0x140b694c4: lea rcx, [rbp - 0x60]
0x140b694c8: call 0x140881d20
0x140b694cd: nop 
0x140b694ce: add rsp, 0x28
0x140b694d2: pop rbx
0x140b694d3: pop rdi
0x140b694d4: pop rsi
0x140b694d5: pop rbp
0x140b694d6: ret 
0x140b694d7: nop word ptr [rax + rax]
0x140b694e0: mov qword ptr [rsp + 0x10], rdx
0x140b694e5: push rbp
0x140b694e6: push rsi
0x140b694e7: push rdi
0x140b694e8: push rbx
0x140b694e9: sub rsp, 0x28
0x140b694ed: lea rbp, [rdx + 0x80]
0x140b694f4: cmp byte ptr [rbp + 0x36f], 0
0x140b694fb: jne 0x140b69506
0x140b694fd: add rsp, 0x28
0x140b69501: pop rbx
0x140b69502: pop rdi
0x140b69503: pop rsi
0x140b69504: pop rbp
0x140b69505: ret 
0x140b69506: lea rcx, [rbp + 0x2b8]
0x140b6950d: call 0x140a14920
0x140b69512: jmp 0x140b694fd
0x140b69514: nop word ptr cs:[rax + rax]
0x140b69520: mov qword ptr [rsp + 0x10], rdx
0x140b69525: push rbp
0x140b69526: push rsi
0x140b69527: push rdi
0x140b69528: push rbx
0x140b69529: sub rsp, 0x28
0x140b6952d: lea rbp, [rdx + 0x80]
0x140b69534: mov byte ptr [rbp + 0x36f], 1
0x140b6953b: lea rcx, [rbp - 0x60]
0x140b6953f: call 0x140029880
0x140b69544: mov byte ptr [rbp + 0x36f], 1
0x140b6954b: add rsp, 0x28
0x140b6954f: pop rbx
0x140b69550: pop rdi
0x140b69551: pop rsi
0x140b69552: pop rbp
0x140b69553: ret 
0x140b69554: nop word ptr cs:[rax + rax]
0x140b69560: mov qword ptr [rsp + 0x10], rdx
0x140b69565: push rbp
0x140b69566: push rsi
0x140b69567: push rdi
0x140b69568: push rbx
0x140b69569: sub rsp, 0x28
0x140b6956d: lea rbp, [rdx + 0x80]
0x140b69574: lea rcx, [rbp + 0x240]
0x140b6957b: call 0x140029880
0x140b69580: mov byte ptr [rbp + 0x36f], 1
0x140b69587: add rsp, 0x28
0x140b6958b: pop rbx
0x140b6958c: pop rdi
0x140b6958d: pop rsi
0x140b6958e: pop rbp
0x140b6958f: ret 
