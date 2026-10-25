1400c7150: 4157 push r15
1400c7152: 4156 push r14
1400c7154: 4155 push r13
1400c7156: 4154 push r12
1400c7158: 56 push rsi
1400c7159: 57 push rdi
1400c715a: 55 push rbp
1400c715b: 53 push rbx
1400c715c: 4881ecd8010000 sub rsp, 0x1d8
1400c7163: bf01000000 mov edi, 1
1400c7168: f00fc1bab0050000 lock xadd dword ptr [rdx + 0x5b0], edi
1400c7170: 31c0 xor eax, eax
1400c7172: 483b8138010000 cmp rax, qword ptr [rcx + 0x138]
1400c7179: 0f80a2060000 jo 0x1400c7821
1400c717f: 4889ce mov rsi, rcx
1400c7182: 488b81e8010000 mov rax, qword ptr [rcx + 0x1e8]
1400c7189: 48398218020000 cmp qword ptr [rdx + 0x218], rax
1400c7190: 0f86a8040000 jbe 0x1400c763e
1400c7196: 488b8a10020000 mov rcx, qword ptr [rdx + 0x210]
1400c719d: 488b0cc1 mov rcx, qword ptr [rcx + rax*8]
1400c71a1: 4885c9 test rcx, rcx
1400c71a4: 0f8494040000 je 0x1400c763e
1400c71aa: 488b82e0010000 mov rax, qword ptr [rdx + 0x1e0]
1400c71b1: 48f7d1 not rcx
1400c71b4: 488d0c49 lea rcx, [rcx + rcx*2]
1400c71b8: 48c1e105 shl rcx, 5
1400c71bc: 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
1400c71c2: 0f8476040000 je 0x1400c763e
1400c71c8: 4801c8 add rax, rcx
1400c71cb: 4c8b7810 mov r15, qword ptr [rax + 0x10]
1400c71cf: 8b8ab0050000 mov ecx, dword ptr [rdx + 0x5b0]
1400c71d5: 4c8b86f0010000 mov r8, qword ptr [rsi + 0x1f0]
1400c71dc: 488b8a18020000 mov rcx, qword ptr [rdx + 0x218]
1400c71e3: 4c39c1 cmp rcx, r8
1400c71e6: 0f8673040000 jbe 0x1400c765f
1400c71ec: 4c8b9210020000 mov r10, qword ptr [rdx + 0x210]
1400c71f3: 4f8b04c2 mov r8, qword ptr [r10 + r8*8]
1400c71f7: 4d85c0 test r8, r8
1400c71fa: 0f845f040000 je 0x1400c765f
1400c7200: 4c8ba2e0010000 mov r12, qword ptr [rdx + 0x1e0]
1400c7207: 49f7d0 not r8
1400c720a: 4f8d0440 lea r8, [r8 + r8*2]
1400c720e: 49c1e005 shl r8, 5
1400c7212: 4b837c042000 cmp qword ptr [r12 + r8 + 0x20], 0
1400c7218: 0f8441040000 je 0x1400c765f
1400c721e: 4c8b8ef8010000 mov r9, qword ptr [rsi + 0x1f8]
1400c7225: 4c39c9 cmp rcx, r9
1400c7228: 0f8652040000 jbe 0x1400c7680
1400c722e: 4f8b0cca mov r9, qword ptr [r10 + r9*8]
1400c7232: 4d85c9 test r9, r9
1400c7235: 0f8445040000 je 0x1400c7680
1400c723b: 49f7d1 not r9
1400c723e: 4f8d0c49 lea r9, [r9 + r9*2]
1400c7242: 49c1e105 shl r9, 5
1400c7246: 4b837c0c2000 cmp qword ptr [r12 + r9 + 0x20], 0
1400c724c: 0f842e040000 je 0x1400c7680
1400c7252: 4c8b9e00020000 mov r11, qword ptr [rsi + 0x200]
1400c7259: 4c39d9 cmp rcx, r11
1400c725c: 0f863f040000 jbe 0x1400c76a1
1400c7262: 4f8b1cda mov r11, qword ptr [r10 + r11*8]
1400c7266: 4d85db test r11, r11
1400c7269: 0f8432040000 je 0x1400c76a1
1400c726f: 49f7d3 not r11
1400c7272: 4f8d1c5b lea r11, [r11 + r11*2]
1400c7276: 49c1e305 shl r11, 5
1400c727a: 4b837c1c2000 cmp qword ptr [r12 + r11 + 0x20], 0
1400c7280: 0f841b040000 je 0x1400c76a1
1400c7286: 488b9e08020000 mov rbx, qword ptr [rsi + 0x208]
1400c728d: 4839d9 cmp rcx, rbx
1400c7290: 0f861e040000 jbe 0x1400c76b4
1400c7296: 498b1cda mov rbx, qword ptr [r10 + rbx*8]
1400c729a: 4885db test rbx, rbx
1400c729d: 0f8411040000 je 0x1400c76b4
1400c72a3: 48f7d3 not rbx
1400c72a6: 488d1c5b lea rbx, [rbx + rbx*2]
1400c72aa: 48c1e305 shl rbx, 5
1400c72ae: 49837c1c2000 cmp qword ptr [r12 + rbx + 0x20], 0
1400c72b4: 0f84fa030000 je 0x1400c76b4
1400c72ba: 4c8bb610020000 mov r14, qword ptr [rsi + 0x210]
1400c72c1: 4c39f1 cmp rcx, r14
1400c72c4: 0f860b040000 jbe 0x1400c76d5
1400c72ca: 4b8b0cf2 mov rcx, qword ptr [r10 + r14*8]
1400c72ce: 4885c9 test rcx, rcx
1400c72d1: 0f84fe030000 je 0x1400c76d5
1400c72d7: 48f7d1 not rcx
1400c72da: 4c8d3449 lea r14, [rcx + rcx*2]
1400c72de: 49c1e605 shl r14, 5
1400c72e2: 4b837c342000 cmp qword ptr [r12 + r14 + 0x20], 0
1400c72e8: 0f84e7030000 je 0x1400c76d5
1400c72ee: 4c89bc24d0010000 mov qword ptr [rsp + 0x1d0], r15
1400c72f6: 448b9630010000 mov r10d, dword ptr [rsi + 0x130]
1400c72fd: 4d01e0 add r8, r12
1400c7300: 498b4810 mov rcx, qword ptr [r8 + 0x10]
1400c7304: 48898c24a8010000 mov qword ptr [rsp + 0x1a8], rcx
1400c730c: 4d01e1 add r9, r12
1400c730f: 498b4910 mov rcx, qword ptr [r9 + 0x10]
1400c7313: 48898c24b0010000 mov qword ptr [rsp + 0x1b0], rcx
1400c731b: 4d01e3 add r11, r12
1400c731e: 498b4b10 mov rcx, qword ptr [r11 + 0x10]
1400c7322: 48898c24b8010000 mov qword ptr [rsp + 0x1b8], rcx
1400c732a: 4c01e3 add rbx, r12
1400c732d: 488b4b10 mov rcx, qword ptr [rbx + 0x10]
1400c7331: 48898c24c0010000 mov qword ptr [rsp + 0x1c0], rcx
1400c7339: 4d01e6 add r14, r12
1400c733c: 498b4e10 mov rcx, qword ptr [r14 + 0x10]
1400c7340: 48898c24c8010000 mov qword ptr [rsp + 0x1c8], rcx
1400c7348: 8b8ab0050000 mov ecx, dword ptr [rdx + 0x5b0]
1400c734e: 488b8e18020000 mov rcx, qword ptr [rsi + 0x218]
1400c7355: 4c8bba18020000 mov r15, qword ptr [rdx + 0x218]
1400c735c: 4939cf cmp r15, rcx
1400c735f: 0f8607040000 jbe 0x1400c776c
1400c7365: 4c8baa10020000 mov r13, qword ptr [rdx + 0x210]
1400c736c: 4d8b64cd00 mov r12, qword ptr [r13 + rcx*8]
1400c7371: 4d85e4 test r12, r12
1400c7374: 0f84f2030000 je 0x1400c776c
1400c737a: 488b8ae0010000 mov rcx, qword ptr [rdx + 0x1e0]
1400c7381: 49f7d4 not r12
1400c7384: 4f8d2464 lea r12, [r12 + r12*2]
1400c7388: 49c1e405 shl r12, 5
1400c738c: 4a837c212000 cmp qword ptr [rcx + r12 + 0x20], 0
1400c7392: 0f84d4030000 je 0x1400c776c
1400c7398: 488bae20020000 mov rbp, qword ptr [rsi + 0x220]
1400c739f: 4939ef cmp r15, rbp
1400c73a2: 0f86e2030000 jbe 0x1400c778a
1400c73a8: 4d8b7ced00 mov r15, qword ptr [r13 + rbp*8]
1400c73ad: 4d85ff test r15, r15
1400c73b0: 0f84d4030000 je 0x1400c778a
1400c73b6: 49f7d7 not r15
1400c73b9: 4f8d2c7f lea r13, [r15 + r15*2]
1400c73bd: 49c1e505 shl r13, 5
1400c73c1: 4a837c292000 cmp qword ptr [rcx + r13 + 0x20], 0
1400c73c7: 0f84bd030000 je 0x1400c778a
1400c73cd: 4901cc add r12, rcx
1400c73d0: 4901cd add r13, rcx
1400c73d3: 498b4c2410 mov rcx, qword ptr [r12 + 0x10]
1400c73d8: 4d8b7d10 mov r15, qword ptr [r13 + 0x10]
1400c73dc: 488bac24d0010000 mov rbp, qword ptr [rsp + 0x1d0]
1400c73e4: 4889ac2480000000 mov qword ptr [rsp + 0x80], rbp
1400c73ec: 488d685c lea rbp, [rax + 0x5c]
1400c73f0: 4883c058 add rax, 0x58
1400c73f4: 4889842488000000 mov qword ptr [rsp + 0x88], rax
1400c73fc: 4889ac2490000000 mov qword ptr [rsp + 0x90], rbp
1400c7404: 4489942498000000 mov dword ptr [rsp + 0x98], r10d
1400c740c: 89bc249c000000 mov dword ptr [rsp + 0x9c], edi
1400c7413: 488b8424a8010000 mov rax, qword ptr [rsp + 0x1a8]
1400c741b: 48898424a0000000 mov qword ptr [rsp + 0xa0], rax
1400c7423: 498d405c lea rax, [r8 + 0x5c]
1400c7427: 4983c058 add r8, 0x58
1400c742b: 4c898424a8000000 mov qword ptr [rsp + 0xa8], r8
1400c7433: 48898424b0000000 mov qword ptr [rsp + 0xb0], rax
1400c743b: 44899424b8000000 mov dword ptr [rsp + 0xb8], r10d
1400c7443: 89bc24bc000000 mov dword ptr [rsp + 0xbc], edi
1400c744a: 488b8424b0010000 mov rax, qword ptr [rsp + 0x1b0]
1400c7452: 48898424c0000000 mov qword ptr [rsp + 0xc0], rax
1400c745a: 498d415c lea rax, [r9 + 0x5c]
1400c745e: 4983c158 add r9, 0x58
1400c7462: 4c898c24c8000000 mov qword ptr [rsp + 0xc8], r9
1400c746a: 48898424d0000000 mov qword ptr [rsp + 0xd0], rax
1400c7472: 44899424d8000000 mov dword ptr [rsp + 0xd8], r10d
1400c747a: 89bc24dc000000 mov dword ptr [rsp + 0xdc], edi
1400c7481: 488b8424b8010000 mov rax, qword ptr [rsp + 0x1b8]
1400c7489: 48898424e0000000 mov qword ptr [rsp + 0xe0], rax
1400c7491: 498d435c lea rax, [r11 + 0x5c]
1400c7495: 4983c358 add r11, 0x58
1400c7499: 4c899c24e8000000 mov qword ptr [rsp + 0xe8], r11
1400c74a1: 48898424f0000000 mov qword ptr [rsp + 0xf0], rax
1400c74a9: 44899424f8000000 mov dword ptr [rsp + 0xf8], r10d
1400c74b1: 89bc24fc000000 mov dword ptr [rsp + 0xfc], edi
1400c74b8: 488b8424c0010000 mov rax, qword ptr [rsp + 0x1c0]
1400c74c0: 4889842400010000 mov qword ptr [rsp + 0x100], rax
1400c74c8: 488d435c lea rax, [rbx + 0x5c]
1400c74cc: 4883c358 add rbx, 0x58
1400c74d0: 48899c2408010000 mov qword ptr [rsp + 0x108], rbx
1400c74d8: 4889842410010000 mov qword ptr [rsp + 0x110], rax
1400c74e0: 488d8638010000 lea rax, [rsi + 0x138]
1400c74e7: 4489942418010000 mov dword ptr [rsp + 0x118], r10d
1400c74ef: 89bc241c010000 mov dword ptr [rsp + 0x11c], edi
1400c74f6: 4889942420010000 mov qword ptr [rsp + 0x120], rdx
1400c74fe: 4d8d5c245c lea r11, [r12 + 0x5c]
1400c7503: 4983c458 add r12, 0x58
1400c7507: 48c784242801000000000000 mov qword ptr [rsp + 0x128], 0
1400c7513: 4889842430010000 mov qword ptr [rsp + 0x130], rax
1400c751b: 488d8670010000 lea rax, [rsi + 0x170]
1400c7522: 4889842440010000 mov qword ptr [rsp + 0x140], rax
1400c752a: 488b9424c8010000 mov rdx, qword ptr [rsp + 0x1c8]
1400c7532: 4889942448010000 mov qword ptr [rsp + 0x148], rdx
1400c753a: 498d565c lea rdx, [r14 + 0x5c]
1400c753e: 4983c658 add r14, 0x58
1400c7542: 4c89b42450010000 mov qword ptr [rsp + 0x150], r14
1400c754a: 498d5d58 lea rbx, [r13 + 0x58]
1400c754e: 4983c55c add r13, 0x5c
1400c7552: 4889942458010000 mov qword ptr [rsp + 0x158], rdx
1400c755a: 488d9424a0000000 lea rdx, [rsp + 0xa0]
1400c7562: 4489942460010000 mov dword ptr [rsp + 0x160], r10d
1400c756a: 89bc2464010000 mov dword ptr [rsp + 0x164], edi
1400c7571: 48898c2468010000 mov qword ptr [rsp + 0x168], rcx
1400c7579: 4c8d8424c0000000 lea r8, [rsp + 0xc0]
1400c7581: 4c89a42470010000 mov qword ptr [rsp + 0x170], r12
1400c7589: 4c8d8c24e0000000 lea r9, [rsp + 0xe0]
1400c7591: 4c899c2478010000 mov qword ptr [rsp + 0x178], r11
1400c7599: 488d8c2400010000 lea rcx, [rsp + 0x100]
1400c75a1: 4489942480010000 mov dword ptr [rsp + 0x180], r10d
1400c75a9: 89bc2484010000 mov dword ptr [rsp + 0x184], edi
1400c75b0: 4c89bc2488010000 mov qword ptr [rsp + 0x188], r15
1400c75b8: 4c8d9c2420010000 lea r11, [rsp + 0x120]
1400c75c0: 48899c2490010000 mov qword ptr [rsp + 0x190], rbx
1400c75c8: 488d9c2448010000 lea rbx, [rsp + 0x148]
1400c75d0: 4c89ac2498010000 mov qword ptr [rsp + 0x198], r13
1400c75d8: 4c8db42468010000 lea r14, [rsp + 0x168]
1400c75e0: 44899424a0010000 mov dword ptr [rsp + 0x1a0], r10d
1400c75e8: 4c8d942488010000 lea r10, [rsp + 0x188]
1400c75f0: 89bc24a4010000 mov dword ptr [rsp + 0x1a4], edi
1400c75f7: 4c89542448 mov qword ptr [rsp + 0x48], r10
1400c75fc: 4c89742440 mov qword ptr [rsp + 0x40], r14
1400c7601: 48895c2438 mov qword ptr [rsp + 0x38], rbx
1400c7606: 4889442430 mov qword ptr [rsp + 0x30], rax
1400c760b: 4c895c2428 mov qword ptr [rsp + 0x28], r11
1400c7610: 48894c2420 mov qword ptr [rsp + 0x20], rcx
1400c7615: 488d8c2480000000 lea rcx, [rsp + 0x80]
1400c761d: e86edb7100 call 0x1407e5190
1400c7622: 89be30010000 mov dword ptr [rsi + 0x130], edi
1400c7628: 31c0 xor eax, eax
1400c762a: 4881c4d8010000 add rsp, 0x1d8
1400c7631: 5b pop rbx
1400c7632: 5d pop rbp
1400c7633: 5f pop rdi
1400c7634: 5e pop rsi
1400c7635: 415c pop r12
1400c7637: 415d pop r13
1400c7639: 415e pop r14
1400c763b: 415f pop r15
1400c763d: c3 ret 
1400c763e: 4881c698000000 add rsi, 0x98
1400c7645: 488d05f5ed8602 lea rax, [rip + 0x286edf5]
1400c764c: 4889442450 mov qword ptr [rsp + 0x50], rax
1400c7651: 48c74424583e000000 mov qword ptr [rsp + 0x58], 0x3e
1400c765a: e992000000 jmp 0x1400c76f1
1400c765f: 4881c698000000 add rsi, 0x98
1400c7666: 488d059bec8602 lea rax, [rip + 0x286ec9b]
1400c766d: 4889442450 mov qword ptr [rsp + 0x50], rax
1400c7672: 48c74424583d000000 mov qword ptr [rsp + 0x58], 0x3d
1400c767b: e926010000 jmp 0x1400c77a6
1400c7680: 4881c698000000 add rsi, 0x98
1400c7687: 488d052fed8602 lea rax, [rip + 0x286ed2f]
1400c768e: 4889442450 mov qword ptr [rsp + 0x50], rax
1400c7693: 48c744245841000000 mov qword ptr [rsp + 0x58], 0x41
1400c769c: e905010000 jmp 0x1400c77a6
1400c76a1: 4881c698000000 add rsi, 0x98
1400c76a8: 488d0596ec8602 lea rax, [rip + 0x286ec96]
1400c76af: e9c6000000 jmp 0x1400c777a
1400c76b4: 4881c698000000 add rsi, 0x98
1400c76bb: 488d0535eb8602 lea rax, [rip + 0x286eb35]
1400c76c2: 4889442450 mov qword ptr [rsp + 0x50], rax
1400c76c7: 48c74424584a000000 mov qword ptr [rsp + 0x58], 0x4a
1400c76d0: e9d1000000 jmp 0x1400c77a6
1400c76d5: 4881c698000000 add rsi, 0x98
1400c76dc: 488d05b5ed8602 lea rax, [rip + 0x286edb5]
1400c76e3: 4889442450 mov qword ptr [rsp + 0x50], rax
1400c76e8: 48c744245866000000 mov qword ptr [rsp + 0x58], 0x66
1400c76f1: 4889742460 mov qword ptr [rsp + 0x60], rsi
1400c76f6: 488d05d343f7ff lea rax, [rip - 0x8bc2d]
1400c76fd: 4889442468 mov qword ptr [rsp + 0x68], rax
1400c7702: 488d442450 lea rax, [rsp + 0x50]
1400c7707: 4889442470 mov qword ptr [rsp + 0x70], rax
1400c770c: 488d057d270400 lea rax, [rip + 0x4277d]
1400c7713: 4889442478 mov qword ptr [rsp + 0x78], rax
1400c7718: 488d0549eb8602 lea rax, [rip + 0x286eb49]
1400c771f: 4889842480000000 mov qword ptr [rsp + 0x80], rax
1400c7727: 48c784248800000002000000 mov qword ptr [rsp + 0x88], 2
1400c7733: 48c78424a000000000000000 mov qword ptr [rsp + 0xa0], 0
1400c773f: 488d442460 lea rax, [rsp + 0x60]
1400c7744: 4889842490000000 mov qword ptr [rsp + 0x90], rax
1400c774c: 48c784249800000002000000 mov qword ptr [rsp + 0x98], 2
1400c7758: 488d1521ed8602 lea rdx, [rip + 0x286ed21]
1400c775f: 488d8c2480000000 lea rcx, [rsp + 0x80]
1400c7767: e8241c8102 call 0x1428d9390
1400c776c: 4881c698000000 add rsi, 0x98
1400c7773: 488d0584ec8602 lea rax, [rip + 0x286ec84]
1400c777a: 4889442450 mov qword ptr [rsp + 0x50], rax
1400c777f: 48c744245843000000 mov qword ptr [rsp + 0x58], 0x43
1400c7788: eb1c jmp 0x1400c77a6
1400c778a: 4881c698000000 add rsi, 0x98
1400c7791: 488d05f0eb8602 lea rax, [rip + 0x286ebf0]
1400c7798: 4889442450 mov qword ptr [rsp + 0x50], rax
1400c779d: 48c744245835000000 mov qword ptr [rsp + 0x58], 0x35
1400c77a6: 4889742460 mov qword ptr [rsp + 0x60], rsi
1400c77ab: 488d051e43f7ff lea rax, [rip - 0x8bce2]
1400c77b2: 4889442468 mov qword ptr [rsp + 0x68], rax
1400c77b7: 488d442450 lea rax, [rsp + 0x50]
1400c77bc: 4889442470 mov qword ptr [rsp + 0x70], rax
1400c77c1: 488d05c8260400 lea rax, [rip + 0x426c8]
1400c77c8: 4889442478 mov qword ptr [rsp + 0x78], rax
1400c77cd: 488d0594ea8602 lea rax, [rip + 0x286ea94]
1400c77d4: 4889842480000000 mov qword ptr [rsp + 0x80], rax
1400c77dc: 48c784248800000002000000 mov qword ptr [rsp + 0x88], 2
1400c77e8: 48c78424a000000000000000 mov qword ptr [rsp + 0xa0], 0
1400c77f4: 488d442460 lea rax, [rsp + 0x60]
1400c77f9: 4889842490000000 mov qword ptr [rsp + 0x90], rax
1400c7801: 48c784249800000002000000 mov qword ptr [rsp + 0x98], 2
1400c780d: 488d15dcea8602 lea rdx, [rip + 0x286eadc]
1400c7814: 488d8c2480000000 lea rcx, [rsp + 0x80]
1400c781c: e86f1b8102 call 0x1428d9390
1400c7821: 488d0d28ee8602 lea rcx, [rip + 0x286ee28]
1400c7828: 4c8d0581ee8602 lea r8, [rip + 0x286ee81]
1400c782f: ba59000000 mov edx, 0x59
1400c7834: e8f71a8102 call 0x1428d9330
1400c7839: cc int3 
