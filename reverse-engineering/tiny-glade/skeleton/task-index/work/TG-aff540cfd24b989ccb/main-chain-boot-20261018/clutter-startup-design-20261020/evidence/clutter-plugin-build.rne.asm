140652050: 55 push rbp
140652051: 4156 push r14
140652053: 56 push rsi
140652054: 57 push rdi
140652055: 53 push rbx
140652056: 4881ec80020000 sub rsp, 0x280
14065205d: 488dac2480000000 lea rbp, [rsp + 0x80]
140652065: 0f29bdf0010000 movaps xmmword ptr [rbp + 0x1f0], xmm7
14065206c: 0f29b5e0010000 movaps xmmword ptr [rbp + 0x1e0], xmm6
140652073: 48c785d8010000feffffff mov qword ptr [rbp + 0x1d8], 0xfffffffffffffffe
14065207e: 4889d6 mov rsi, rdx
140652081: 488d7a38 lea rdi, [rdx + 0x38]
140652085: 48ba84a72b9651d21b8e movabs rdx, 0x8e1bd251962ba784
14065208f: 49b89b52e6c07423786a movabs r8, 0x6a782374c0e6529b
140652099: 4889f9 mov rcx, rdi
14065209c: e84fb6a8ff call 0x1400dd6f0
1406520a1: 4885c0 test rax, rax
1406520a4: 0f84f60a0000 je 0x140652ba0
1406520aa: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
1406520b1: 0f86e90a0000 jbe 0x140652ba0
1406520b7: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
1406520be: 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
1406520c2: 4885c9 test rcx, rcx
1406520c5: 0f84d50a0000 je 0x140652ba0
1406520cb: 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
1406520d2: 48f7d1 not rcx
1406520d5: 488d0c49 lea rcx, [rcx + rcx*2]
1406520d9: 48c1e105 shl rcx, 5
1406520dd: 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
1406520e3: 0f84b70a0000 je 0x140652ba0
1406520e9: 4801c8 add rax, rcx
1406520ec: 488b4810 mov rcx, qword ptr [rax + 0x10]
1406520f0: 8b96b0050000 mov edx, dword ptr [rsi + 0x5b0]
1406520f6: 89505c mov dword ptr [rax + 0x5c], edx
1406520f9: e882738800 call 0x140ed9480
1406520fe: 4c8d05f3f03b02 lea r8, [rip + 0x23bf0f3]
140652105: 488d9df0000000 lea rbx, [rbp + 0xf0]
14065210c: 4889d9 mov rcx, rbx
14065210f: b217 mov dl, 0x17
140652111: e8ea978800 call 0x140edb900
140652116: 4889f1 mov rcx, rsi
140652119: 4889da mov rdx, rbx
14065211c: e8df598800 call 0x140ed7b00
140652121: 4c8d05e8f03b02 lea r8, [rip + 0x23bf0e8]
140652128: 4889d9 mov rcx, rbx
14065212b: b217 mov dl, 0x17
14065212d: e85ecb8800 call 0x140edec90
140652132: 4889f1 mov rcx, rsi
140652135: 4889da mov rdx, rbx
140652138: e8c3598800 call 0x140ed7b00
14065213d: 488d4da0 lea rcx, [rbp - 0x60]
140652141: e8aad78800 call 0x140edf8f0
140652146: 488d8df0000000 lea rcx, [rbp + 0xf0]
14065214d: e8feec8800 call 0x140ee0e50
140652152: 0f108550010000 movups xmm0, xmmword ptr [rbp + 0x150]
140652159: 0f2985e0000000 movaps xmmword ptr [rbp + 0xe0], xmm0
140652160: 0f108540010000 movups xmm0, xmmword ptr [rbp + 0x140]
140652167: 0f2985d0000000 movaps xmmword ptr [rbp + 0xd0], xmm0
14065216e: 0f108530010000 movups xmm0, xmmword ptr [rbp + 0x130]
140652175: 0f2985c0000000 movaps xmmword ptr [rbp + 0xc0], xmm0
14065217c: 0f1085f0000000 movups xmm0, xmmword ptr [rbp + 0xf0]
140652183: 0f108d00010000 movups xmm1, xmmword ptr [rbp + 0x100]
14065218a: 0f109510010000 movups xmm2, xmmword ptr [rbp + 0x110]
140652191: 0f109d20010000 movups xmm3, xmmword ptr [rbp + 0x120]
140652198: 0f299db0000000 movaps xmmword ptr [rbp + 0xb0], xmm3
14065219f: 0f2995a0000000 movaps xmmword ptr [rbp + 0xa0], xmm2
1406521a6: 0f298d90000000 movaps xmmword ptr [rbp + 0x90], xmm1
1406521ad: 0f298580000000 movaps xmmword ptr [rbp + 0x80], xmm0
1406521b4: 488d4d10 lea rcx, [rbp + 0x10]
1406521b8: 488d9580000000 lea rdx, [rbp + 0x80]
1406521bf: e8bcf88800 call 0x140ee1a80
1406521c4: 0f1045a0 movups xmm0, xmmword ptr [rbp - 0x60]
1406521c8: 0f104db0 movups xmm1, xmmword ptr [rbp - 0x50]
1406521cc: 0f1055c0 movups xmm2, xmmword ptr [rbp - 0x40]
1406521d0: 0f105dd0 movups xmm3, xmmword ptr [rbp - 0x30]
1406521d4: 0f2985f0000000 movaps xmmword ptr [rbp + 0xf0], xmm0
1406521db: 0f298d00010000 movaps xmmword ptr [rbp + 0x100], xmm1
1406521e2: 0f299510010000 movaps xmmword ptr [rbp + 0x110], xmm2
1406521e9: 0f299d20010000 movaps xmmword ptr [rbp + 0x120], xmm3
1406521f0: 0f1045e0 movups xmm0, xmmword ptr [rbp - 0x20]
1406521f4: 0f298530010000 movaps xmmword ptr [rbp + 0x130], xmm0
1406521fb: 0f1045f0 movups xmm0, xmmword ptr [rbp - 0x10]
1406521ff: 0f298540010000 movaps xmmword ptr [rbp + 0x140], xmm0
140652206: 0f104500 movups xmm0, xmmword ptr [rbp]
14065220a: 0f298550010000 movaps xmmword ptr [rbp + 0x150], xmm0
140652211: 0f104510 movups xmm0, xmmword ptr [rbp + 0x10]
140652215: 0f104d20 movups xmm1, xmmword ptr [rbp + 0x20]
140652219: 0f105530 movups xmm2, xmmword ptr [rbp + 0x30]
14065221d: 0f105d40 movups xmm3, xmmword ptr [rbp + 0x40]
140652221: 0f298560010000 movaps xmmword ptr [rbp + 0x160], xmm0
140652228: 0f298d70010000 movaps xmmword ptr [rbp + 0x170], xmm1
14065222f: 0f299580010000 movaps xmmword ptr [rbp + 0x180], xmm2
140652236: 0f299d90010000 movaps xmmword ptr [rbp + 0x190], xmm3
14065223d: 0f104550 movups xmm0, xmmword ptr [rbp + 0x50]
140652241: 0f2985a0010000 movaps xmmword ptr [rbp + 0x1a0], xmm0
140652248: 0f104560 movups xmm0, xmmword ptr [rbp + 0x60]
14065224c: 0f2985b0010000 movaps xmmword ptr [rbp + 0x1b0], xmm0
140652253: 0f104570 movups xmm0, xmmword ptr [rbp + 0x70]
140652257: 0f2985c0010000 movaps xmmword ptr [rbp + 0x1c0], xmm0
14065225e: 488d8d80000000 lea rcx, [rbp + 0x80]
140652265: 488d95f0000000 lea rdx, [rbp + 0xf0]
14065226c: e8ef028900 call 0x140ee2560
140652271: c6451024 mov byte ptr [rbp + 0x10], 0x24
140652275: 488b0df4a0fd02 mov rcx, qword ptr [rip + 0x2fda0f4]
14065227c: 4c8d05bd595402 lea r8, [rip + 0x25459bd]
140652283: 488d5d10 lea rbx, [rbp + 0x10]
140652287: 4889da mov rdx, rbx
14065228a: e8e1158900 call 0x140ee3870
14065228f: 4989d0 mov r8, rdx
140652292: 4c8db580000000 lea r14, [rbp + 0x80]
140652299: 4c89f1 mov rcx, r14
14065229c: 4889c2 mov rdx, rax
14065229f: e8acfa8800 call 0x140ee1d50
1406522a4: 0f1085e0000000 movups xmm0, xmmword ptr [rbp + 0xe0]
1406522ab: 0f298550010000 movaps xmmword ptr [rbp + 0x150], xmm0
1406522b2: 0f1085d0000000 movups xmm0, xmmword ptr [rbp + 0xd0]
1406522b9: 0f298540010000 movaps xmmword ptr [rbp + 0x140], xmm0
1406522c0: 0f1085c0000000 movups xmm0, xmmword ptr [rbp + 0xc0]
1406522c7: 0f298530010000 movaps xmmword ptr [rbp + 0x130], xmm0
1406522ce: 0f108580000000 movups xmm0, xmmword ptr [rbp + 0x80]
1406522d5: 0f108d90000000 movups xmm1, xmmword ptr [rbp + 0x90]
1406522dc: 0f1095a0000000 movups xmm2, xmmword ptr [rbp + 0xa0]
1406522e3: 0f109db0000000 movups xmm3, xmmword ptr [rbp + 0xb0]
1406522ea: 0f299d20010000 movaps xmmword ptr [rbp + 0x120], xmm3
1406522f1: 0f299510010000 movaps xmmword ptr [rbp + 0x110], xmm2
1406522f8: 0f298d00010000 movaps xmmword ptr [rbp + 0x100], xmm1
1406522ff: 0f2985f0000000 movaps xmmword ptr [rbp + 0xf0], xmm0
140652306: 488d95f0000000 lea rdx, [rbp + 0xf0]
14065230d: 4889f1 mov rcx, rsi
140652310: e8eb578800 call 0x140ed7b00
140652315: 48ba84a72b9651d21b8e movabs rdx, 0x8e1bd251962ba784
14065231f: 49b89b52e6c07423786a movabs r8, 0x6a782374c0e6529b
140652329: 4889f9 mov rcx, rdi
14065232c: e8bfb3a8ff call 0x1400dd6f0
140652331: 4885c0 test rax, rax
140652334: 0f84e2080000 je 0x140652c1c
14065233a: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
140652341: 0f86d5080000 jbe 0x140652c1c
140652347: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
14065234e: 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
140652352: 4885c9 test rcx, rcx
140652355: 0f84c1080000 je 0x140652c1c
14065235b: 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
140652362: 48f7d1 not rcx
140652365: 488d0c49 lea rcx, [rcx + rcx*2]
140652369: 48c1e105 shl rcx, 5
14065236d: 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
140652373: 0f84a3080000 je 0x140652c1c
140652379: 4801c8 add rax, rcx
14065237c: 488b4810 mov rcx, qword ptr [rax + 0x10]
140652380: 8b96b0050000 mov edx, dword ptr [rsi + 0x5b0]
140652386: 89505c mov dword ptr [rax + 0x5c], edx
140652389: b201 mov dl, 1
14065238b: e8e07a8800 call 0x140ed9e70
140652390: 31c9 xor ecx, ecx
140652392: e88977a8ff call 0x1400d9b20
140652397: 4885c0 test rax, rax
14065239a: 0f84ed080000 je 0x140652c8d
1406523a0: 0f1000 movups xmm0, xmmword ptr [rax]
1406523a3: 48ff00 inc qword ptr [rax]
1406523a6: 0f100d835b5402 movups xmm1, xmmword ptr [rip + 0x2545b83]
1406523ad: 0f298d90000000 movaps xmmword ptr [rbp + 0x90], xmm1
1406523b4: 0f100d655b5402 movups xmm1, xmmword ptr [rip + 0x2545b65]
1406523bb: 0f298d80000000 movaps xmmword ptr [rbp + 0x80], xmm1
1406523c2: 0f2985a0000000 movaps xmmword ptr [rbp + 0xa0], xmm0
1406523c9: 48c785f000000000000000 mov qword ptr [rbp + 0xf0], 0
1406523d4: 48c785f800000008000000 mov qword ptr [rbp + 0xf8], 8
1406523df: 48c7850001000000000000 mov qword ptr [rbp + 0x100], 0
1406523ea: 4c8d0d0f5d5402 lea r9, [rip + 0x2545d0f]
1406523f1: 488d4d10 lea rcx, [rbp + 0x10]
1406523f5: 488d95f0000000 lea rdx, [rbp + 0xf0]
1406523fc: 41b800100000 mov r8d, 0x1000
140652402: e8493b8900 call 0x140ee5f50
140652407: 0f288580000000 movaps xmm0, xmmword ptr [rbp + 0x80]
14065240e: 0f288d90000000 movaps xmm1, xmmword ptr [rbp + 0x90]
140652415: 0f2895a0000000 movaps xmm2, xmmword ptr [rbp + 0xa0]
14065241c: 0f298550010000 movaps xmmword ptr [rbp + 0x150], xmm0
140652423: 0f298d60010000 movaps xmmword ptr [rbp + 0x160], xmm1
14065242a: 0f299570010000 movaps xmmword ptr [rbp + 0x170], xmm2
140652431: 0f104510 movups xmm0, xmmword ptr [rbp + 0x10]
140652435: 0f2985f0000000 movaps xmmword ptr [rbp + 0xf0], xmm0
14065243c: 488b4520 mov rax, qword ptr [rbp + 0x20]
140652440: 48898500010000 mov qword ptr [rbp + 0x100], rax
140652447: 48c7850801000000000000 mov qword ptr [rbp + 0x108], 0
140652452: 48c7851001000008000000 mov qword ptr [rbp + 0x110], 8
14065245d: 0f57c0 xorps xmm0, xmm0
140652460: 0f118518010000 movups xmmword ptr [rbp + 0x118], xmm0
140652467: 48c7852801000002000000 mov qword ptr [rbp + 0x128], 2
140652472: 0f298530010000 movaps xmmword ptr [rbp + 0x130], xmm0
140652479: 48c7854001000008000000 mov qword ptr [rbp + 0x140], 8
140652484: 48c7854801000000000000 mov qword ptr [rbp + 0x148], 0
14065248f: c6858001000000 mov byte ptr [rbp + 0x180], 0
140652496: 488d95f0000000 lea rdx, [rbp + 0xf0]
14065249d: 4889f1 mov rcx, rsi
1406524a0: e80b628900 call 0x140ee86b0
1406524a5: 48c785f000000000000000 mov qword ptr [rbp + 0xf0], 0
1406524b0: 48c785f800000010000000 mov qword ptr [rbp + 0xf8], 0x10
1406524bb: 48c7850001000000000000 mov qword ptr [rbp + 0x100], 0
1406524c6: 488d95f0000000 lea rdx, [rbp + 0xf0]
1406524cd: 4889c1 mov rcx, rax
1406524d0: e8eb5b8900 call 0x140ee80c0
1406524d5: 48c785f000000000000000 mov qword ptr [rbp + 0xf0], 0
1406524e0: 48c785f800000008000000 mov qword ptr [rbp + 0xf8], 8
1406524eb: 48c7850001000000000000 mov qword ptr [rbp + 0x100], 0
1406524f6: 488d95f0000000 lea rdx, [rbp + 0xf0]
1406524fd: 4889c1 mov rcx, rax
140652500: e8db4f8900 call 0x140ee74e0
140652505: 4889c6 mov rsi, rax
140652508: e833b26f00 call 0x140d4d740
14065250d: 4889c7 mov rdi, rax
140652510: f60001 test byte ptr [rax], 1
140652513: 7411 je 0x140652526
140652515: 4889f9 mov rcx, rdi
140652518: 4883c108 add rcx, 8
14065251c: 488b4708 mov rax, qword ptr [rdi + 8]
140652520: 488b5710 mov rdx, qword ptr [rdi + 0x10]
140652524: eb1b jmp 0x140652541
140652526: e8d5287100 call 0x140d64e00
14065252b: 4889f9 mov rcx, rdi
14065252e: 4883c108 add rcx, 8
140652532: 48c70701000000 mov qword ptr [rdi], 1
140652539: 48894708 mov qword ptr [rdi + 8], rax
14065253d: 48895710 mov qword ptr [rdi + 0x10], rdx
140652541: 4c8d4001 lea r8, [rax + 1]
140652545: 4c8901 mov qword ptr [rcx], r8
140652548: 0f103571ec3b02 movups xmm6, xmmword ptr [rip + 0x23bec71]
14065254f: 0f29b500010000 movaps xmmword ptr [rbp + 0x100], xmm6
140652556: 0f103d53ec3b02 movups xmm7, xmmword ptr [rip + 0x23bec53]
14065255d: 0f29bdf0000000 movaps xmmword ptr [rbp + 0xf0], xmm7
140652564: 48898510010000 mov qword ptr [rbp + 0x110], rax
14065256b: 48899518010000 mov qword ptr [rbp + 0x118], rdx
140652572: 488d95f0000000 lea rdx, [rbp + 0xf0]
140652579: 4889f1 mov rcx, rsi
14065257c: e80f788900 call 0x140ee9d90
140652581: 4889c1 mov rcx, rax
140652584: 31d2 xor edx, edx
140652586: e8f5668900 call 0x140ee8c80
14065258b: 48c785f000000000000000 mov qword ptr [rbp + 0xf0], 0
140652596: 48c785f800000008000000 mov qword ptr [rbp + 0xf8], 8
1406525a1: 48c7850001000000000000 mov qword ptr [rbp + 0x100], 0
1406525ac: 488d95f0000000 lea rdx, [rbp + 0xf0]
1406525b3: 4889c1 mov rcx, rax
1406525b6: e8d5838900 call 0x140eea990
1406525bb: 4889c1 mov rcx, rax
1406525be: 31d2 xor edx, edx
1406525c0: e8bb898900 call 0x140eeaf80
1406525c5: 4889c6 mov rsi, rax
1406525c8: 48c7858000000000000000 mov qword ptr [rbp + 0x80], 0
1406525d3: 48c7858800000004000000 mov qword ptr [rbp + 0x88], 4
1406525de: 48c7859000000000000000 mov qword ptr [rbp + 0x90], 0
1406525e9: c7859800000000000000 mov dword ptr [rbp + 0x98], 0
1406525f3: 4c8d0d7e635402 lea r9, [rip + 0x254637e]
1406525fa: 488dbdf0000000 lea rdi, [rbp + 0xf0]
140652601: 488d9580000000 lea rdx, [rbp + 0x80]
140652608: 41b800100000 mov r8d, 0x1000
14065260e: 4889f9 mov rcx, rdi
140652611: e8aafd8800 call 0x140ee23c0
140652616: 0f57c0 xorps xmm0, xmm0
140652619: 0f118518010000 movups xmmword ptr [rbp + 0x118], xmm0
140652620: 0f118530010000 movups xmmword ptr [rbp + 0x130], xmm0
140652627: 48c7850801000000000000 mov qword ptr [rbp + 0x108], 0
140652632: 48c7851001000008000000 mov qword ptr [rbp + 0x110], 8
14065263d: 48c7852801000008000000 mov qword ptr [rbp + 0x128], 8
140652648: 4889f1 mov rcx, rsi
14065264b: 4889fa mov rdx, rdi
14065264e: e83d7d8900 call 0x140eea390
140652653: 48c785f000000000000000 mov qword ptr [rbp + 0xf0], 0
14065265e: 48c785f800000008000000 mov qword ptr [rbp + 0xf8], 8
140652669: 48c7850001000000000000 mov qword ptr [rbp + 0x100], 0
140652674: 488d95f0000000 lea rdx, [rbp + 0xf0]
14065267b: 4889c1 mov rcx, rax
14065267e: e8ed6b8900 call 0x140ee9270
140652683: 48c785f000000000000000 mov qword ptr [rbp + 0xf0], 0
14065268e: 48c785f800000010000000 mov qword ptr [rbp + 0xf8], 0x10
140652699: 48c7850001000000000000 mov qword ptr [rbp + 0x100], 0
1406526a4: 488d95f0000000 lea rdx, [rbp + 0xf0]
1406526ab: 4889c1 mov rcx, rax
1406526ae: e82d488900 call 0x140ee6ee0
1406526b3: 48c785f000000000000000 mov qword ptr [rbp + 0xf0], 0
1406526be: 48c785f800000008000000 mov qword ptr [rbp + 0xf8], 8
1406526c9: 48c7850001000000000000 mov qword ptr [rbp + 0x100], 0
1406526d4: 488d95f0000000 lea rdx, [rbp + 0xf0]
1406526db: 4889c1 mov rcx, rax
1406526de: e8dd8d8900 call 0x140eeb4c0
1406526e3: 4889c6 mov rsi, rax
1406526e6: 31c9 xor ecx, ecx
1406526e8: e83374a8ff call 0x1400d9b20
1406526ed: 4885c0 test rax, rax
1406526f0: 0f8497050000 je 0x140652c8d
1406526f6: 0f1000 movups xmm0, xmmword ptr [rax]
1406526f9: 48ff00 inc qword ptr [rax]
1406526fc: 0f298510010000 movaps xmmword ptr [rbp + 0x110], xmm0
140652703: 0f1005c6625402 movups xmm0, xmmword ptr [rip + 0x25462c6]
14065270a: 0f2985f0000000 movaps xmmword ptr [rbp + 0xf0], xmm0
140652711: 0f1005c8625402 movups xmm0, xmmword ptr [rip + 0x25462c8]
140652718: 0f298500010000 movaps xmmword ptr [rbp + 0x100], xmm0
14065271f: 488d95f0000000 lea rdx, [rbp + 0xf0]
140652726: 4889f1 mov rcx, rsi
140652729: e8a2538900 call 0x140ee7ad0
14065272e: 4889c6 mov rsi, rax
140652731: e83afc9aff call 0x140002370
140652736: 888500010000 mov byte ptr [rbp + 0x100], al
14065273c: 0f28057db92e02 movaps xmm0, xmmword ptr [rip + 0x22eb97d]
140652743: 0f2985f0000000 movaps xmmword ptr [rbp + 0xf0], xmm0
14065274a: 488d95f0000000 lea rdx, [rbp + 0xf0]
140652751: 4889f1 mov rcx, rsi
140652754: e8f7708900 call 0x140ee9850
140652759: 4889c6 mov rsi, rax
14065275c: e8dfaf6f00 call 0x140d4d740
140652761: 4889c7 mov rdi, rax
140652764: f60001 test byte ptr [rax], 1
140652767: 7411 je 0x14065277a
140652769: 4889f9 mov rcx, rdi
14065276c: 4883c108 add rcx, 8
140652770: 488b4708 mov rax, qword ptr [rdi + 8]
140652774: 488b5710 mov rdx, qword ptr [rdi + 0x10]
140652778: eb1b jmp 0x140652795
14065277a: e881267100 call 0x140d64e00
14065277f: 4889f9 mov rcx, rdi
140652782: 4883c108 add rcx, 8
140652786: 48c70701000000 mov qword ptr [rdi], 1
14065278d: 48894708 mov qword ptr [rdi + 8], rax
140652791: 48895710 mov qword ptr [rdi + 0x10], rdx
140652795: 4c8d4001 lea r8, [rax + 1]
140652799: 4c8901 mov qword ptr [rcx], r8
14065279c: 0f29b500010000 movaps xmmword ptr [rbp + 0x100], xmm6
1406527a3: 0f29bdf0000000 movaps xmmword ptr [rbp + 0xf0], xmm7
1406527aa: 48898510010000 mov qword ptr [rbp + 0x110], rax
1406527b1: 48899518010000 mov qword ptr [rbp + 0x118], rdx
1406527b8: 488d95f0000000 lea rdx, [rbp + 0xf0]
1406527bf: 4889f1 mov rcx, rsi
1406527c2: e819418900 call 0x140ee68e0
1406527c7: 4889c6 mov rsi, rax
1406527ca: 4889c7 mov rdi, rax
1406527cd: 4883c738 add rdi, 0x38
1406527d1: 48badaaac2870ed31812 movabs rdx, 0x1218d30e87c2aada
1406527db: 49b85a7a1ba82d9b4802 movabs r8, 0x2489b2da81b7a5a
1406527e5: 4889f9 mov rcx, rdi
1406527e8: e803afa8ff call 0x1400dd6f0
1406527ed: 4885c0 test rax, rax
1406527f0: 7433 je 0x140652825
1406527f2: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
1406527f9: 762a jbe 0x140652825
1406527fb: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
140652802: 488b04d0 mov rax, qword ptr [rax + rdx*8]
140652806: 4885c0 test rax, rax
140652809: 741a je 0x140652825
14065280b: 488b8ee0010000 mov rcx, qword ptr [rsi + 0x1e0]
140652812: 48f7d0 not rax
140652815: 488d0440 lea rax, [rax + rax*2]
140652819: 48c1e005 shl rax, 5
14065281d: 48837c012000 cmp qword ptr [rcx + rax + 0x20], 0
140652823: 7508 jne 0x14065282d
140652825: 4889f1 mov rcx, rsi
140652828: e823b58900 call 0x140eedd50
14065282d: 48ba5cdfb20747f1c1dd movabs rdx, 0xddc1f14707b2df5c
140652837: 49b86c8d126de0144e71 movabs r8, 0x714e14e06d128d6c
140652841: 4889f9 mov rcx, rdi
140652844: e8a7aea8ff call 0x1400dd6f0
140652849: 4885c0 test rax, rax
14065284c: 7433 je 0x140652881
14065284e: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
140652855: 762a jbe 0x140652881
140652857: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
14065285e: 488b04d0 mov rax, qword ptr [rax + rdx*8]
140652862: 4885c0 test rax, rax
140652865: 741a je 0x140652881
140652867: 488b8ee0010000 mov rcx, qword ptr [rsi + 0x1e0]
14065286e: 48f7d0 not rax
140652871: 488d0440 lea rax, [rax + rax*2]
140652875: 48c1e005 shl rax, 5
140652879: 48837c012000 cmp qword ptr [rcx + rax + 0x20], 0
14065287f: 7508 jne 0x140652889
140652881: 4889f1 mov rcx, rsi
140652884: e887cd8900 call 0x140eef610
140652889: 48bacf522827cfab239f movabs rdx, 0x9f23abcf272852cf
140652893: 49b84e73bf509d63c354 movabs r8, 0x54c3639d50bf734e
14065289d: 4889f9 mov rcx, rdi
1406528a0: e84baea8ff call 0x1400dd6f0
1406528a5: 4885c0 test rax, rax
1406528a8: 7433 je 0x1406528dd
1406528aa: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
1406528b1: 762a jbe 0x1406528dd
1406528b3: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
1406528ba: 488b04d0 mov rax, qword ptr [rax + rdx*8]
1406528be: 4885c0 test rax, rax
1406528c1: 741a je 0x1406528dd
1406528c3: 488b8ee0010000 mov rcx, qword ptr [rsi + 0x1e0]
1406528ca: 48f7d0 not rax
1406528cd: 488d0440 lea rax, [rax + rax*2]
1406528d1: 48c1e005 shl rax, 5
1406528d5: 48837c012000 cmp qword ptr [rcx + rax + 0x20], 0
1406528db: 7508 jne 0x1406528e5
1406528dd: 4889f1 mov rcx, rsi
1406528e0: e85bd38900 call 0x140eefc40
1406528e5: 48ba981d9f4dec2663e1 movabs rdx, 0xe16326ec4d9f1d98
1406528ef: 49b81d36dc5277f04853 movabs r8, 0x5348f07752dc361d
1406528f9: 4889f9 mov rcx, rdi
1406528fc: e8efada8ff call 0x1400dd6f0
140652901: 4885c0 test rax, rax
140652904: 7433 je 0x140652939
140652906: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
14065290d: 762a jbe 0x140652939
14065290f: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
140652916: 488b04d0 mov rax, qword ptr [rax + rdx*8]
14065291a: 4885c0 test rax, rax
14065291d: 741a je 0x140652939
14065291f: 488b8ee0010000 mov rcx, qword ptr [rsi + 0x1e0]
140652926: 48f7d0 not rax
140652929: 488d0440 lea rax, [rax + rax*2]
14065292d: 48c1e005 shl rax, 5
140652931: 48837c012000 cmp qword ptr [rcx + rax + 0x20], 0
140652937: 7508 jne 0x140652941
140652939: 4889f1 mov rcx, rsi
14065293c: e86fc08900 call 0x140eee9b0
140652941: 48bac0238afa39759d69 movabs rdx, 0x699d7539fa8a23c0
14065294b: 49b831567f228f846d3d movabs r8, 0x3d6d848f227f5631
140652955: 4889f9 mov rcx, rdi
140652958: e893ada8ff call 0x1400dd6f0
14065295d: 4885c0 test rax, rax
140652960: 7433 je 0x140652995
140652962: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
140652969: 762a jbe 0x140652995
14065296b: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
140652972: 488b04d0 mov rax, qword ptr [rax + rdx*8]
140652976: 4885c0 test rax, rax
140652979: 741a je 0x140652995
14065297b: 488b8ee0010000 mov rcx, qword ptr [rsi + 0x1e0]
140652982: 48f7d0 not rax
140652985: 488d0440 lea rax, [rax + rax*2]
140652989: 48c1e005 shl rax, 5
14065298d: 48837c012000 cmp qword ptr [rcx + rax + 0x20], 0
140652993: 7508 jne 0x14065299d
140652995: 4889f1 mov rcx, rsi
140652998: e803df8900 call 0x140ef08a0
14065299d: 48ba0332a7d3bba6fe49 movabs rdx, 0x49fea6bbd3a73203
1406529a7: 49b8a43b70311b8f3810 movabs r8, 0x10388f1b31703ba4
1406529b1: 4889f9 mov rcx, rdi
1406529b4: e837ada8ff call 0x1400dd6f0
1406529b9: 4885c0 test rax, rax
1406529bc: 7433 je 0x1406529f1
1406529be: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
1406529c5: 762a jbe 0x1406529f1
1406529c7: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
1406529ce: 488b04d0 mov rax, qword ptr [rax + rdx*8]
1406529d2: 4885c0 test rax, rax
1406529d5: 741a je 0x1406529f1
1406529d7: 488b8ee0010000 mov rcx, qword ptr [rsi + 0x1e0]
1406529de: 48f7d0 not rax
1406529e1: 488d0440 lea rax, [rax + rax*2]
1406529e5: 48c1e005 shl rax, 5
1406529e9: 48837c012000 cmp qword ptr [rcx + rax + 0x20], 0
1406529ef: 7508 jne 0x1406529f9
1406529f1: 4889f1 mov rcx, rsi
1406529f4: e8e7c58900 call 0x140eeefe0
1406529f9: 48ba60c90c56971b737e movabs rdx, 0x7e731b97560cc960
140652a03: 49b8255baf1a0382b3c6 movabs r8, 0xc6b382031aaf5b25
140652a0d: 4889f9 mov rcx, rdi
140652a10: e8dbaca8ff call 0x1400dd6f0
140652a15: 4885c0 test rax, rax
140652a18: 7433 je 0x140652a4d
140652a1a: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
140652a21: 762a jbe 0x140652a4d
140652a23: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
140652a2a: 488b04d0 mov rax, qword ptr [rax + rdx*8]
140652a2e: 4885c0 test rax, rax
140652a31: 741a je 0x140652a4d
140652a33: 488b8ee0010000 mov rcx, qword ptr [rsi + 0x1e0]
140652a3a: 48f7d0 not rax
140652a3d: 488d0440 lea rax, [rax + rax*2]
140652a41: 48c1e005 shl rax, 5
140652a45: 48837c012000 cmp qword ptr [rcx + rax + 0x20], 0
140652a4b: 7508 jne 0x140652a55
140652a4d: 4889f1 mov rcx, rsi
140652a50: e89ba68900 call 0x140eed0f0
140652a55: 48baf6025782f31a4c58 movabs rdx, 0x584c1af3825702f6
140652a5f: 49b8fb557deaf703fe04 movabs r8, 0x4fe03f7ea7d55fb
140652a69: 4889f9 mov rcx, rdi
140652a6c: e87faca8ff call 0x1400dd6f0
140652a71: 4885c0 test rax, rax
140652a74: 7433 je 0x140652aa9
140652a76: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
140652a7d: 762a jbe 0x140652aa9
140652a7f: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
140652a86: 488b04d0 mov rax, qword ptr [rax + rdx*8]
140652a8a: 4885c0 test rax, rax
140652a8d: 741a je 0x140652aa9
140652a8f: 488b8ee0010000 mov rcx, qword ptr [rsi + 0x1e0]
140652a96: 48f7d0 not rax
140652a99: 488d0440 lea rax, [rax + rax*2]
140652a9d: 48c1e005 shl rax, 5
140652aa1: 48837c012000 cmp qword ptr [rcx + rax + 0x20], 0
140652aa7: 7508 jne 0x140652ab1
140652aa9: 4889f1 mov rcx, rsi
140652aac: e86fac8900 call 0x140eed720
140652ab1: 48bafc7abefa45ba4a11 movabs rdx, 0x114aba45fabe7afc
140652abb: 49b895f6dc280fcc8c19 movabs r8, 0x198ccc0f28dcf695
140652ac5: 4889f9 mov rcx, rdi
140652ac8: e823aca8ff call 0x1400dd6f0
140652acd: 4885c0 test rax, rax
140652ad0: 7433 je 0x140652b05
140652ad2: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
140652ad9: 762a jbe 0x140652b05
140652adb: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
140652ae2: 488b04d0 mov rax, qword ptr [rax + rdx*8]
140652ae6: 4885c0 test rax, rax
140652ae9: 741a je 0x140652b05
140652aeb: 488b8ee0010000 mov rcx, qword ptr [rsi + 0x1e0]
140652af2: 48f7d0 not rax
140652af5: 488d0440 lea rax, [rax + rax*2]
140652af9: 48c1e005 shl rax, 5
140652afd: 48837c012000 cmp qword ptr [rcx + rax + 0x20], 0
140652b03: 7508 jne 0x140652b0d
140652b05: 4889f1 mov rcx, rsi
140652b08: e863d78900 call 0x140ef0270
140652b0d: 48ba3bf17216f03f7a6d movabs rdx, 0x6d7a3ff01672f13b
140652b17: 49b8146dda3f1c18833d movabs r8, 0x3d83181c3fda6d14
140652b21: 4889f9 mov rcx, rdi
140652b24: e8c7aba8ff call 0x1400dd6f0
140652b29: 4885c0 test rax, rax
140652b2c: 744f je 0x140652b7d
140652b2e: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
140652b35: 7646 jbe 0x140652b7d
140652b37: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
140652b3e: 488b04d0 mov rax, qword ptr [rax + rdx*8]
140652b42: 4885c0 test rax, rax
140652b45: 7436 je 0x140652b7d
140652b47: 488b8ee0010000 mov rcx, qword ptr [rsi + 0x1e0]
140652b4e: 48f7d0 not rax
140652b51: 488d0440 lea rax, [rax + rax*2]
140652b55: 48c1e005 shl rax, 5
140652b59: 48837c012000 cmp qword ptr [rcx + rax + 0x20], 0
140652b5f: 741c je 0x140652b7d
140652b61: 0f28b5e0010000 movaps xmm6, xmmword ptr [rbp + 0x1e0]
140652b68: 0f28bdf0010000 movaps xmm7, xmmword ptr [rbp + 0x1f0]
140652b6f: 4881c480020000 add rsp, 0x280
140652b76: 5b pop rbx
140652b77: 5f pop rdi
140652b78: 5e pop rsi
140652b79: 415e pop r14
140652b7b: 5d pop rbp
140652b7c: c3 ret 
140652b7d: 4889f1 mov rcx, rsi
140652b80: 0f28b5e0010000 movaps xmm6, xmmword ptr [rbp + 0x1e0]
140652b87: 0f28bdf0010000 movaps xmm7, xmmword ptr [rbp + 0x1f0]
140652b8e: 4881c480020000 add rsp, 0x280
140652b95: 5b pop rbx
140652b96: 5f pop rdi
140652b97: 5e pop rsi
140652b98: 415e pop r14
140652b9a: 5d pop rbp
140652b9b: e9e0b78900 jmp 0x140eee380
140652ba0: 488d0549265402 lea rax, [rip + 0x2542649]
140652ba7: 48898580000000 mov qword ptr [rbp + 0x80], rax
140652bae: 48c7858800000027000000 mov qword ptr [rbp + 0x88], 0x27
140652bb9: 488d8580000000 lea rax, [rbp + 0x80]
140652bc0: 48894510 mov qword ptr [rbp + 0x10], rax
140652bc4: 488d05c572abff lea rax, [rip - 0x548d3b]
140652bcb: 48894518 mov qword ptr [rbp + 0x18], rax
140652bcf: 488d054a275402 lea rax, [rip + 0x254274a]
140652bd6: 488985f0000000 mov qword ptr [rbp + 0xf0], rax
140652bdd: 48c785f800000002000000 mov qword ptr [rbp + 0xf8], 2
140652be8: 48c7851001000000000000 mov qword ptr [rbp + 0x110], 0
140652bf3: 488d4510 lea rax, [rbp + 0x10]
140652bf7: 48898500010000 mov qword ptr [rbp + 0x100], rax
140652bfe: 48c7850801000001000000 mov qword ptr [rbp + 0x108], 1
140652c09: 488d15c8255402 lea rdx, [rip + 0x25425c8]
140652c10: 488d8df0000000 lea rcx, [rbp + 0xf0]
140652c17: e874672802 call 0x1428d9390
140652c1c: 488d05cd255402 lea rax, [rip + 0x25425cd]
140652c23: 48898580000000 mov qword ptr [rbp + 0x80], rax
140652c2a: 48c7858800000027000000 mov qword ptr [rbp + 0x88], 0x27
140652c35: 4c897510 mov qword ptr [rbp + 0x10], r14
140652c39: 488d055072abff lea rax, [rip - 0x548db0]
140652c40: 48894518 mov qword ptr [rbp + 0x18], rax
140652c44: 488d05d5265402 lea rax, [rip + 0x25426d5]
140652c4b: 488985f0000000 mov qword ptr [rbp + 0xf0], rax
140652c52: 48c785f800000002000000 mov qword ptr [rbp + 0xf8], 2
140652c5d: 48c7851001000000000000 mov qword ptr [rbp + 0x110], 0
140652c68: 48899d00010000 mov qword ptr [rbp + 0x100], rbx
140652c6f: 48c7850801000001000000 mov qword ptr [rbp + 0x108], 1
140652c7a: 488d1557255402 lea rdx, [rip + 0x2542557]
140652c81: 488d8df0000000 lea rcx, [rbp + 0xf0]
140652c88: e803672802 call 0x1428d9390
140652c8d: 488d0d1c5f5402 lea rcx, [rip + 0x2545f1c]
140652c94: e837542802 call 0x1428d80d0
140652c99: cc int3 
140652c9a: 660f1f440000 nop word ptr [rax + rax]
140652ca0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140652ca5: 55 push rbp
140652ca6: 4156 push r14
140652ca8: 56 push rsi
140652ca9: 57 push rdi
140652caa: 53 push rbx
140652cab: 4883ec40 sub rsp, 0x40
140652caf: 488daa80000000 lea rbp, [rdx + 0x80]
140652cb6: 0f297c2420 movaps xmmword ptr [rsp + 0x20], xmm7
140652cbb: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140652cc0: 488d4da0 lea rcx, [rbp - 0x60]
140652cc4: e8b76b9dff call 0x140029880
140652cc9: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140652cce: 0f287c2420 movaps xmm7, xmmword ptr [rsp + 0x20]
140652cd3: 4883c440 add rsp, 0x40
140652cd7: 5b pop rbx
140652cd8: 5f pop rdi
140652cd9: 5e pop rsi
140652cda: 415e pop r14
140652cdc: 5d pop rbp
140652cdd: c3 ret 
140652cde: 6690 nop 
140652ce0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140652ce5: 55 push rbp
140652ce6: 4156 push r14
140652ce8: 56 push rsi
140652ce9: 57 push rdi
140652cea: 53 push rbx
140652ceb: 4883ec40 sub rsp, 0x40
140652cef: 488daa80000000 lea rbp, [rdx + 0x80]
140652cf6: 0f297c2420 movaps xmmword ptr [rsp + 0x20], xmm7
140652cfb: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140652d00: 488d8d80000000 lea rcx, [rbp + 0x80]
140652d07: e8746b9dff call 0x140029880
140652d0c: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140652d11: 0f287c2420 movaps xmm7, xmmword ptr [rsp + 0x20]
140652d16: 4883c440 add rsp, 0x40
140652d1a: 5b pop rbx
140652d1b: 5f pop rdi
140652d1c: 5e pop rsi
140652d1d: 415e pop r14
140652d1f: 5d pop rbp
140652d20: c3 ret 
140652d21: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140652d30: 4889542410 mov qword ptr [rsp + 0x10], rdx
140652d35: 55 push rbp
140652d36: 4156 push r14
140652d38: 56 push rsi
140652d39: 57 push rdi
140652d3a: 53 push rbx
140652d3b: 4883ec40 sub rsp, 0x40
140652d3f: 488daa80000000 lea rbp, [rdx + 0x80]
140652d46: 0f297c2420 movaps xmmword ptr [rsp + 0x20], xmm7
140652d4b: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140652d50: 488d8d80000000 lea rcx, [rbp + 0x80]
140652d57: 41b848000000 mov r8d, 0x48
140652d5d: 41b910000000 mov r9d, 0x10
140652d63: 488d95a0000000 lea rdx, [rbp + 0xa0]
140652d6a: e8714c8800 call 0x140ed79e0
140652d6f: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140652d74: 0f287c2420 movaps xmm7, xmmword ptr [rsp + 0x20]
140652d79: 4883c440 add rsp, 0x40
140652d7d: 5b pop rbx
140652d7e: 5f pop rdi
140652d7f: 5e pop rsi
140652d80: 415e pop r14
140652d82: 5d pop rbp
140652d83: c3 ret 
