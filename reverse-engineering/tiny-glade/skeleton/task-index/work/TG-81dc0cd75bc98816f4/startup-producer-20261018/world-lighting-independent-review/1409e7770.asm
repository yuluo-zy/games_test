1409e7770: 55 push rbp
1409e7771: 4157 push r15
1409e7773: 4156 push r14
1409e7775: 4155 push r13
1409e7777: 4154 push r12
1409e7779: 56 push rsi
1409e777a: 57 push rdi
1409e777b: 53 push rbx
1409e777c: 4881ec18040000 sub rsp, 0x418
1409e7783: 488dac2480000000 lea rbp, [rsp + 0x80]
1409e778b: 48c78590030000feffffff mov qword ptr [rbp + 0x390], 0xfffffffffffffffe
1409e7796: 4d89c6 mov r14, r8
1409e7799: 4889d7 mov rdi, rdx
1409e779c: 4889ce mov rsi, rcx
1409e779f: 498d4838 lea rcx, [r8 + 0x38]
1409e77a3: 48ba5593676e78cc2682 movabs rdx, 0x8226cc786e679355
1409e77ad: 49b8ac41a8556adacf65 movabs r8, 0x65cfda6a55a841ac
1409e77b7: e8345f6fff call 0x1400dd6f0
1409e77bc: 4885c0 test rax, rax
1409e77bf: 0f84680c0000 je 0x1409e842d
1409e77c5: 49399618020000 cmp qword ptr [r14 + 0x218], rdx
1409e77cc: 0f865b0c0000 jbe 0x1409e842d
1409e77d2: 498b8610020000 mov rax, qword ptr [r14 + 0x210]
1409e77d9: 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
1409e77dd: 4885c9 test rcx, rcx
1409e77e0: 0f84470c0000 je 0x1409e842d
1409e77e6: 498b86e0010000 mov rax, qword ptr [r14 + 0x1e0]
1409e77ed: 48f7d1 not rcx
1409e77f0: 488d0c49 lea rcx, [rcx + rcx*2]
1409e77f4: 48c1e105 shl rcx, 5
1409e77f8: 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
1409e77fe: 0f84290c0000 je 0x1409e842d
1409e7804: 4801c8 add rax, rcx
1409e7807: 488b4810 mov rcx, qword ptr [rax + 0x10]
1409e780b: 488d151e6a2402 lea rdx, [rip + 0x2246a1e]
1409e7812: e8091ff8ff call 0x140969720
1409e7817: 4885c0 test rax, rax
1409e781a: 0f84190c0000 je 0x1409e8439
1409e7820: 4889c3 mov rbx, rax
1409e7823: 488d05d45a0b02 lea rax, [rip + 0x20b5ad4]
1409e782a: 48894500 mov qword ptr [rbp], rax
1409e782e: 48c7450835000000 mov qword ptr [rbp + 8], 0x35
1409e7836: 48837b1835 cmp qword ptr [rbx + 0x18], 0x35
1409e783b: 0f85bd0b0000 jne 0x1409e83fe
1409e7841: 488b5310 mov rdx, qword ptr [rbx + 0x10]
1409e7845: 488d0db25a0b02 lea rcx, [rip + 0x20b5ab2]
1409e784c: 41b835000000 mov r8d, 0x35
1409e7852: e899c3f301 call 0x142923bf0
1409e7857: 85c0 test eax, eax
1409e7859: 0f859f0b0000 jne 0x1409e83fe
1409e785f: 488b0b mov rcx, qword ptr [rbx]
1409e7862: 488b4308 mov rax, qword ptr [rbx + 8]
1409e7866: ff5018 call qword ptr [rax + 0x18]
1409e7869: 4885c0 test rax, rax
1409e786c: 0f84c70b0000 je 0x1409e8439
1409e7872: 488b18 mov rbx, qword ptr [rax]
1409e7875: 488d8d40030000 lea rcx, [rbp + 0x340]
1409e787c: 85db test ebx, ebx
1409e787e: 0f849d0a0000 je 0x1409e8321
1409e7884: ba00000100 mov edx, 0x10000
1409e7889: 41b820000000 mov r8d, 0x20
1409e788f: e88cd51e00 call 0x140bd4e20
1409e7894: 0f108540030000 movups xmm0, xmmword ptr [rbp + 0x340]
1409e789b: 0f294500 movaps xmmword ptr [rbp], xmm0
1409e789f: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1409e78a6: 4889ea mov rdx, rbp
1409e78a9: e852f41e00 call 0x140bd6d00
1409e78ae: 488d05e6d00d02 lea rax, [rip + 0x20dd0e6]
1409e78b5: 488985b0010000 mov qword ptr [rbp + 0x1b0], rax
1409e78bc: 48c785b801000016000000 mov qword ptr [rbp + 0x1b8], 0x16
1409e78c7: 488d95b0010000 lea rdx, [rbp + 0x1b0]
1409e78ce: 4889f9 mov rcx, rdi
1409e78d1: e8fa1b2000 call 0x140be94d0
1409e78d6: 0f108540030000 movups xmm0, xmmword ptr [rbp + 0x340]
1409e78dd: 0f298590010000 movaps xmmword ptr [rbp + 0x190], xmm0
1409e78e4: 8985a0010000 mov dword ptr [rbp + 0x1a0], eax
1409e78ea: 4c8dbd90010000 lea r15, [rbp + 0x190]
1409e78f1: 4889f9 mov rcx, rdi
1409e78f4: 4c89f2 mov rdx, r14
1409e78f7: 4d89f8 mov r8, r15
1409e78fa: e8b162ffff call 0x1409ddbb0
1409e78ff: 48897c2428 mov qword ptr [rsp + 0x28], rdi
1409e7904: 4c89742420 mov qword ptr [rsp + 0x20], r14
1409e7909: 488d15a1d00d02 lea rdx, [rip + 0x20dd0a1]
1409e7910: 4c8da5b0010000 lea r12, [rbp + 0x1b0]
1409e7917: 41b811000000 mov r8d, 0x11
1409e791d: 4c89e1 mov rcx, r12
1409e7920: 4531c9 xor r9d, r9d
1409e7923: e80868c100 call 0x1415fe130
1409e7928: 4989ed mov r13, rbp
1409e792b: 49b8ad2a9920eb336b0c movabs r8, 0xc6b33eb20992aad
1409e7935: 4c89e9 mov rcx, r13
1409e7938: 4c89e2 mov rdx, r12
1409e793b: e8405dc200 call 0x14160d680
1409e7940: 488d8db0010000 lea rcx, [rbp + 0x1b0]
1409e7947: 49b8949d869af187e563 movabs r8, 0x63e587f19a869d94
1409e7951: 4c89ea mov rdx, r13
1409e7954: 4d89f9 mov r9, r15
1409e7957: e80464f8ff call 0x14096dd60
1409e795c: 80bde602000008 cmp byte ptr [rbp + 0x2e6], 8
1409e7963: 7425 je 0x1409e798a
1409e7965: 488d8dc0020000 lea rcx, [rbp + 0x2c0]
1409e796c: bae8000000 mov edx, 0xe8
1409e7971: 480395e8020000 add rdx, qword ptr [rbp + 0x2e8]
1409e7978: 49b89ff68f409e32889b movabs r8, 0x9b88329e408ff69f
1409e7982: 4189d9 mov r9d, ebx
1409e7985: e8e63bf8ff call 0x14096b570
1409e798a: 4889e9 mov rcx, rbp
1409e798d: 488d95b0010000 lea rdx, [rbp + 0x1b0]
1409e7994: 41b890010000 mov r8d, 0x190
1409e799a: e861c2f301 call 0x142923c00
1409e799f: 0f1085a0000000 movups xmm0, xmmword ptr [rbp + 0xa0]
1409e79a6: 0f298580030000 movaps xmmword ptr [rbp + 0x380], xmm0
1409e79ad: 0f108590000000 movups xmm0, xmmword ptr [rbp + 0x90]
1409e79b4: 0f298570030000 movaps xmmword ptr [rbp + 0x370], xmm0
1409e79bb: 0f108580000000 movups xmm0, xmmword ptr [rbp + 0x80]
1409e79c2: 0f298560030000 movaps xmmword ptr [rbp + 0x360], xmm0
1409e79c9: 0f104570 movups xmm0, xmmword ptr [rbp + 0x70]
1409e79cd: 0f298550030000 movaps xmmword ptr [rbp + 0x350], xmm0
1409e79d4: 0f104560 movups xmm0, xmmword ptr [rbp + 0x60]
1409e79d8: 0f298540030000 movaps xmmword ptr [rbp + 0x340], xmm0
1409e79df: 488b8538010000 mov rax, qword ptr [rbp + 0x138]
1409e79e6: 488b8d80010000 mov rcx, qword ptr [rbp + 0x180]
1409e79ed: 488b5510 mov rdx, qword ptr [rbp + 0x10]
1409e79f1: 488995c0010000 mov qword ptr [rbp + 0x1c0], rdx
1409e79f8: 0f104500 movups xmm0, xmmword ptr [rbp]
1409e79fc: 0f2985b0010000 movaps xmmword ptr [rbp + 0x1b0], xmm0
1409e7a03: 0f104518 movups xmm0, xmmword ptr [rbp + 0x18]
1409e7a07: 0f2985e0010000 movaps xmmword ptr [rbp + 0x1e0], xmm0
1409e7a0e: 488b5528 mov rdx, qword ptr [rbp + 0x28]
1409e7a12: 488995f0010000 mov qword ptr [rbp + 0x1f0], rdx
1409e7a19: 0f104530 movups xmm0, xmmword ptr [rbp + 0x30]
1409e7a1d: 0f1185c8010000 movups xmmword ptr [rbp + 0x1c8], xmm0
1409e7a24: 488b5540 mov rdx, qword ptr [rbp + 0x40]
1409e7a28: 488995d8010000 mov qword ptr [rbp + 0x1d8], rdx
1409e7a2f: 0f1085b0000000 movups xmm0, xmmword ptr [rbp + 0xb0]
1409e7a36: 0f108dc0000000 movups xmm1, xmmword ptr [rbp + 0xc0]
1409e7a3d: 0f1095d0000000 movups xmm2, xmmword ptr [rbp + 0xd0]
1409e7a44: 0f109de0000000 movups xmm3, xmmword ptr [rbp + 0xe0]
1409e7a4b: 0f118528020000 movups xmmword ptr [rbp + 0x228], xmm0
1409e7a52: 0f118d38020000 movups xmmword ptr [rbp + 0x238], xmm1
1409e7a59: 0f119548020000 movups xmmword ptr [rbp + 0x248], xmm2
1409e7a60: 0f119d58020000 movups xmmword ptr [rbp + 0x258], xmm3
1409e7a67: 0f1085f0000000 movups xmm0, xmmword ptr [rbp + 0xf0]
1409e7a6e: 0f118568020000 movups xmmword ptr [rbp + 0x268], xmm0
1409e7a75: 0f108500010000 movups xmm0, xmmword ptr [rbp + 0x100]
1409e7a7c: 0f118578020000 movups xmmword ptr [rbp + 0x278], xmm0
1409e7a83: 488b5558 mov rdx, qword ptr [rbp + 0x58]
1409e7a87: 48899508020000 mov qword ptr [rbp + 0x208], rdx
1409e7a8e: 0f104548 movups xmm0, xmmword ptr [rbp + 0x48]
1409e7a92: 0f1185f8010000 movups xmmword ptr [rbp + 0x1f8], xmm0
1409e7a99: 48898d90020000 mov qword ptr [rbp + 0x290], rcx
1409e7aa0: 48898588020000 mov qword ptr [rbp + 0x288], rax
1409e7aa7: c7851002000001000000 mov dword ptr [rbp + 0x210], 1
1409e7ab1: 899d14020000 mov dword ptr [rbp + 0x214], ebx
1409e7ab7: 49bd0100000001000000 movabs r13, 0x100000001
1409e7ac1: 4c89ad18020000 mov qword ptr [rbp + 0x218], r13
1409e7ac8: 488d8d40030000 lea rcx, [rbp + 0x340]
1409e7acf: 488d95b0010000 lea rdx, [rbp + 0x1b0]
1409e7ad6: e8f559ffff call 0x1409dd4d0
1409e7adb: 80bd3601000008 cmp byte ptr [rbp + 0x136], 8
1409e7ae2: 740c je 0x1409e7af0
1409e7ae4: 488d8d20010000 lea rcx, [rbp + 0x120]
1409e7aeb: e8c0a91f00 call 0x140be24b0
1409e7af0: 488d8d40030000 lea rcx, [rbp + 0x340]
1409e7af7: ba44000000 mov edx, 0x44
1409e7afc: 41b820000000 mov r8d, 0x20
1409e7b02: e819d31e00 call 0x140bd4e20
1409e7b07: 0f108540030000 movups xmm0, xmmword ptr [rbp + 0x340]
1409e7b0e: 0f294500 movaps xmmword ptr [rbp], xmm0
1409e7b12: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1409e7b19: 4889ea mov rdx, rbp
1409e7b1c: e8dff11e00 call 0x140bd6d00
1409e7b21: 488d059ace0d02 lea rax, [rip + 0x20dce9a]
1409e7b28: 488985b0010000 mov qword ptr [rbp + 0x1b0], rax
1409e7b2f: 48c785b80100001b000000 mov qword ptr [rbp + 0x1b8], 0x1b
1409e7b3a: 488d95b0010000 lea rdx, [rbp + 0x1b0]
1409e7b41: 4889f9 mov rcx, rdi
1409e7b44: e887192000 call 0x140be94d0
1409e7b49: 0f108540030000 movups xmm0, xmmword ptr [rbp + 0x340]
1409e7b50: 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
1409e7b54: 8945f0 mov dword ptr [rbp - 0x10], eax
1409e7b57: 48897c2430 mov qword ptr [rsp + 0x30], rdi
1409e7b5c: 4c89742428 mov qword ptr [rsp + 0x28], r14
1409e7b61: 48c744242026000000 mov qword ptr [rsp + 0x20], 0x26
1409e7b6a: 488d156cce0d02 lea rdx, [rip + 0x20dce6c]
1409e7b71: 4c8d0d77ce0d02 lea r9, [rip + 0x20dce77]
1409e7b78: 4c8dbdb0010000 lea r15, [rbp + 0x1b0]
1409e7b7f: 41b812000000 mov r8d, 0x12
1409e7b85: 4c89f9 mov rcx, r15
1409e7b88: e813b7c100 call 0x1416032a0
1409e7b8d: 4989ec mov r12, rbp
1409e7b90: 49b84872ad18d361b8aa movabs r8, 0xaab861d318ad7248
1409e7b9a: 4c8d8d90010000 lea r9, [rbp + 0x190]
1409e7ba1: 4c89e1 mov rcx, r12
1409e7ba4: 4c89fa mov rdx, r15
1409e7ba7: e8b461f8ff call 0x14096dd60
1409e7bac: 488d8db0010000 lea rcx, [rbp + 0x1b0]
1409e7bb3: 49b890691f97404467c3 movabs r8, 0xc3674440971f6990
1409e7bbd: 4c8d4de0 lea r9, [rbp - 0x20]
1409e7bc1: 4c89e2 mov rdx, r12
1409e7bc4: e89761f8ff call 0x14096dd60
1409e7bc9: 0f108550020000 movups xmm0, xmmword ptr [rbp + 0x250]
1409e7bd0: 0f298580030000 movaps xmmword ptr [rbp + 0x380], xmm0
1409e7bd7: 0f108540020000 movups xmm0, xmmword ptr [rbp + 0x240]
1409e7bde: 0f298570030000 movaps xmmword ptr [rbp + 0x370], xmm0
1409e7be5: 0f108530020000 movups xmm0, xmmword ptr [rbp + 0x230]
1409e7bec: 0f298560030000 movaps xmmword ptr [rbp + 0x360], xmm0
1409e7bf3: 0f108520020000 movups xmm0, xmmword ptr [rbp + 0x220]
1409e7bfa: 0f298550030000 movaps xmmword ptr [rbp + 0x350], xmm0
1409e7c01: 0f108510020000 movups xmm0, xmmword ptr [rbp + 0x210]
1409e7c08: 0f298540030000 movaps xmmword ptr [rbp + 0x340], xmm0
1409e7c0f: 488b85e8020000 mov rax, qword ptr [rbp + 0x2e8]
1409e7c16: 488b8d30030000 mov rcx, qword ptr [rbp + 0x330]
1409e7c1d: 488b95c0010000 mov rdx, qword ptr [rbp + 0x1c0]
1409e7c24: 48895510 mov qword ptr [rbp + 0x10], rdx
1409e7c28: 0f1085b0010000 movups xmm0, xmmword ptr [rbp + 0x1b0]
1409e7c2f: 0f294500 movaps xmmword ptr [rbp], xmm0
1409e7c33: 0f1085c8010000 movups xmm0, xmmword ptr [rbp + 0x1c8]
1409e7c3a: 0f294530 movaps xmmword ptr [rbp + 0x30], xmm0
1409e7c3e: 488b95d8010000 mov rdx, qword ptr [rbp + 0x1d8]
1409e7c45: 48895540 mov qword ptr [rbp + 0x40], rdx
1409e7c49: 0f1085e0010000 movups xmm0, xmmword ptr [rbp + 0x1e0]
1409e7c50: 0f114518 movups xmmword ptr [rbp + 0x18], xmm0
1409e7c54: 488b95f0010000 mov rdx, qword ptr [rbp + 0x1f0]
1409e7c5b: 48895528 mov qword ptr [rbp + 0x28], rdx
1409e7c5f: 0f108560020000 movups xmm0, xmmword ptr [rbp + 0x260]
1409e7c66: 0f108d70020000 movups xmm1, xmmword ptr [rbp + 0x270]
1409e7c6d: 0f109580020000 movups xmm2, xmmword ptr [rbp + 0x280]
1409e7c74: 0f109d90020000 movups xmm3, xmmword ptr [rbp + 0x290]
1409e7c7b: 0f114578 movups xmmword ptr [rbp + 0x78], xmm0
1409e7c7f: 0f118d88000000 movups xmmword ptr [rbp + 0x88], xmm1
1409e7c86: 0f119598000000 movups xmmword ptr [rbp + 0x98], xmm2
1409e7c8d: 0f119da8000000 movups xmmword ptr [rbp + 0xa8], xmm3
1409e7c94: 0f1085a0020000 movups xmm0, xmmword ptr [rbp + 0x2a0]
1409e7c9b: 0f1185b8000000 movups xmmword ptr [rbp + 0xb8], xmm0
1409e7ca2: 0f1085b0020000 movups xmm0, xmmword ptr [rbp + 0x2b0]
1409e7ca9: 0f1185c8000000 movups xmmword ptr [rbp + 0xc8], xmm0
1409e7cb0: 488b9508020000 mov rdx, qword ptr [rbp + 0x208]
1409e7cb7: 48895558 mov qword ptr [rbp + 0x58], rdx
1409e7cbb: 0f1085f8010000 movups xmm0, xmmword ptr [rbp + 0x1f8]
1409e7cc2: 0f114548 movups xmmword ptr [rbp + 0x48], xmm0
1409e7cc6: 48898de0000000 mov qword ptr [rbp + 0xe0], rcx
1409e7ccd: 488985d8000000 mov qword ptr [rbp + 0xd8], rax
1409e7cd4: 0f280545070d02 movaps xmm0, xmmword ptr [rip + 0x20d0745]
1409e7cdb: 0f294560 movaps xmmword ptr [rbp + 0x60], xmm0
1409e7cdf: 488d8d40030000 lea rcx, [rbp + 0x340]
1409e7ce6: 4889ea mov rdx, rbp
1409e7ce9: e8e257ffff call 0x1409dd4d0
1409e7cee: 80bde602000008 cmp byte ptr [rbp + 0x2e6], 8
1409e7cf5: 740c je 0x1409e7d03
1409e7cf7: 488d8dd0020000 lea rcx, [rbp + 0x2d0]
1409e7cfe: e8ada71f00 call 0x140be24b0
1409e7d03: 48897c2430 mov qword ptr [rsp + 0x30], rdi
1409e7d08: 4c89742428 mov qword ptr [rsp + 0x28], r14
1409e7d0d: 48c74424202f000000 mov qword ptr [rsp + 0x20], 0x2f
1409e7d16: 488d15f8cc0d02 lea rdx, [rip + 0x20dccf8]
1409e7d1d: 4c8d0d0ccd0d02 lea r9, [rip + 0x20dcd0c]
1409e7d24: 41b81b000000 mov r8d, 0x1b
1409e7d2a: 4c89f9 mov rcx, r15
1409e7d2d: e86eb5c100 call 0x1416032a0
1409e7d32: 4889e9 mov rcx, rbp
1409e7d35: 49b84872ad18d361b8aa movabs r8, 0xaab861d318ad7248
1409e7d3f: 4c8d4de0 lea r9, [rbp - 0x20]
1409e7d43: 4c89fa mov rdx, r15
1409e7d46: e81560f8ff call 0x14096dd60
1409e7d4b: 0fb605b7cec402 movzx eax, byte ptr [rip + 0x2c4ceb7]
1409e7d52: b904000000 mov ecx, 4
1409e7d57: ba01000000 mov edx, 1
1409e7d5c: e8afbec2ff call 0x140613c10
1409e7d61: 4885c0 test rax, rax
1409e7d64: 0f84db060000 je 0x1409e8445
1409e7d6a: 4989c7 mov r15, rax
1409e7d6d: c70010000000 mov dword ptr [rax], 0x10
1409e7d73: 488b5548 mov rdx, qword ptr [rbp + 0x48]
1409e7d77: 4885d2 test rdx, rdx
1409e7d7a: 740f je 0x1409e7d8b
1409e7d7c: 488b4d50 mov rcx, qword ptr [rbp + 0x50]
1409e7d80: 41b801000000 mov r8d, 1
1409e7d86: e895bec2ff call 0x140613c20
1409e7d8b: 48c7454804000000 mov qword ptr [rbp + 0x48], 4
1409e7d93: 4c897d50 mov qword ptr [rbp + 0x50], r15
1409e7d97: 48c7455804000000 mov qword ptr [rbp + 0x58], 4
1409e7d9f: 488d8db0010000 lea rcx, [rbp + 0x1b0]
1409e7da6: 4889ea mov rdx, rbp
1409e7da9: 41b890010000 mov r8d, 0x190
1409e7daf: e84cbef301 call 0x142923c00
1409e7db4: 0f108550020000 movups xmm0, xmmword ptr [rbp + 0x250]
1409e7dbb: 0f298580030000 movaps xmmword ptr [rbp + 0x380], xmm0
1409e7dc2: 0f108540020000 movups xmm0, xmmword ptr [rbp + 0x240]
1409e7dc9: 0f298570030000 movaps xmmword ptr [rbp + 0x370], xmm0
1409e7dd0: 0f108530020000 movups xmm0, xmmword ptr [rbp + 0x230]
1409e7dd7: 0f298560030000 movaps xmmword ptr [rbp + 0x360], xmm0
1409e7dde: 0f108520020000 movups xmm0, xmmword ptr [rbp + 0x220]
1409e7de5: 0f298550030000 movaps xmmword ptr [rbp + 0x350], xmm0
1409e7dec: 0f108510020000 movups xmm0, xmmword ptr [rbp + 0x210]
1409e7df3: 0f298540030000 movaps xmmword ptr [rbp + 0x340], xmm0
1409e7dfa: 488b85e8020000 mov rax, qword ptr [rbp + 0x2e8]
1409e7e01: 488b8d30030000 mov rcx, qword ptr [rbp + 0x330]
1409e7e08: 488b95c0010000 mov rdx, qword ptr [rbp + 0x1c0]
1409e7e0f: 48895510 mov qword ptr [rbp + 0x10], rdx
1409e7e13: 0f1085b0010000 movups xmm0, xmmword ptr [rbp + 0x1b0]
1409e7e1a: 0f294500 movaps xmmword ptr [rbp], xmm0
1409e7e1e: 0f1085c8010000 movups xmm0, xmmword ptr [rbp + 0x1c8]
1409e7e25: 0f294530 movaps xmmword ptr [rbp + 0x30], xmm0
1409e7e29: 488b95d8010000 mov rdx, qword ptr [rbp + 0x1d8]
1409e7e30: 48895540 mov qword ptr [rbp + 0x40], rdx
1409e7e34: 0f1085e0010000 movups xmm0, xmmword ptr [rbp + 0x1e0]
1409e7e3b: 0f114518 movups xmmword ptr [rbp + 0x18], xmm0
1409e7e3f: 488b95f0010000 mov rdx, qword ptr [rbp + 0x1f0]
1409e7e46: 48895528 mov qword ptr [rbp + 0x28], rdx
1409e7e4a: 0f108560020000 movups xmm0, xmmword ptr [rbp + 0x260]
1409e7e51: 0f108d70020000 movups xmm1, xmmword ptr [rbp + 0x270]
1409e7e58: 0f109580020000 movups xmm2, xmmword ptr [rbp + 0x280]
1409e7e5f: 0f109d90020000 movups xmm3, xmmword ptr [rbp + 0x290]
1409e7e66: 0f114578 movups xmmword ptr [rbp + 0x78], xmm0
1409e7e6a: 0f118d88000000 movups xmmword ptr [rbp + 0x88], xmm1
1409e7e71: 0f119598000000 movups xmmword ptr [rbp + 0x98], xmm2
1409e7e78: 0f119da8000000 movups xmmword ptr [rbp + 0xa8], xmm3
1409e7e7f: 0f1085a0020000 movups xmm0, xmmword ptr [rbp + 0x2a0]
1409e7e86: 0f1185b8000000 movups xmmword ptr [rbp + 0xb8], xmm0
1409e7e8d: 0f1085b0020000 movups xmm0, xmmword ptr [rbp + 0x2b0]
1409e7e94: 0f1185c8000000 movups xmmword ptr [rbp + 0xc8], xmm0
1409e7e9b: 488b9508020000 mov rdx, qword ptr [rbp + 0x208]
1409e7ea2: 48895558 mov qword ptr [rbp + 0x58], rdx
1409e7ea6: 0f1085f8010000 movups xmm0, xmmword ptr [rbp + 0x1f8]
1409e7ead: 0f114548 movups xmmword ptr [rbp + 0x48], xmm0
1409e7eb1: 48898de0000000 mov qword ptr [rbp + 0xe0], rcx
1409e7eb8: 488985d8000000 mov qword ptr [rbp + 0xd8], rax
1409e7ebf: 0f28059ac60d02 movaps xmm0, xmmword ptr [rip + 0x20dc69a]
1409e7ec6: 0f294560 movaps xmmword ptr [rbp + 0x60], xmm0
1409e7eca: 488d8d40030000 lea rcx, [rbp + 0x340]
1409e7ed1: 4889ea mov rdx, rbp
1409e7ed4: e8f755ffff call 0x1409dd4d0
1409e7ed9: 80bde602000008 cmp byte ptr [rbp + 0x2e6], 8
1409e7ee0: 740c je 0x1409e7eee
1409e7ee2: 488d8dd0020000 lea rcx, [rbp + 0x2d0]
1409e7ee9: e8c2a51f00 call 0x140be24b0
1409e7eee: 48897c2430 mov qword ptr [rsp + 0x30], rdi
1409e7ef3: 4c89742428 mov qword ptr [rsp + 0x28], r14
1409e7ef8: 48c744242028000000 mov qword ptr [rsp + 0x20], 0x28
1409e7f01: 488d1557cb0d02 lea rdx, [rip + 0x20dcb57]
1409e7f08: 4c8d0d68cb0d02 lea r9, [rip + 0x20dcb68]
1409e7f0f: 4c8dbdb0010000 lea r15, [rbp + 0x1b0]
1409e7f16: 41b818000000 mov r8d, 0x18
1409e7f1c: 4c89f9 mov rcx, r15
1409e7f1f: e87cb3c100 call 0x1416032a0
1409e7f24: 49b84872ad18d361b8aa movabs r8, 0xaab861d318ad7248
1409e7f2e: 4c8d8d90010000 lea r9, [rbp + 0x190]
1409e7f35: 4c89e1 mov rcx, r12
1409e7f38: 4c89fa mov rdx, r15
1409e7f3b: e8205ef8ff call 0x14096dd60
1409e7f40: 488d8db0010000 lea rcx, [rbp + 0x1b0]
1409e7f47: 49b890691f97404467c3 movabs r8, 0xc3674440971f6990
1409e7f51: 4c8d4de0 lea r9, [rbp - 0x20]
1409e7f55: 4c89e2 mov rdx, r12
1409e7f58: e8635bf8ff call 0x14096dac0
1409e7f5d: 0f108550020000 movups xmm0, xmmword ptr [rbp + 0x250]
1409e7f64: 0f298580030000 movaps xmmword ptr [rbp + 0x380], xmm0
1409e7f6b: 0f108540020000 movups xmm0, xmmword ptr [rbp + 0x240]
1409e7f72: 0f298570030000 movaps xmmword ptr [rbp + 0x370], xmm0
1409e7f79: 0f108530020000 movups xmm0, xmmword ptr [rbp + 0x230]
1409e7f80: 0f298560030000 movaps xmmword ptr [rbp + 0x360], xmm0
1409e7f87: 0f108520020000 movups xmm0, xmmword ptr [rbp + 0x220]
1409e7f8e: 0f298550030000 movaps xmmword ptr [rbp + 0x350], xmm0
1409e7f95: 0f108510020000 movups xmm0, xmmword ptr [rbp + 0x210]
1409e7f9c: 0f298540030000 movaps xmmword ptr [rbp + 0x340], xmm0
1409e7fa3: 488b85e8020000 mov rax, qword ptr [rbp + 0x2e8]
1409e7faa: 488b8d30030000 mov rcx, qword ptr [rbp + 0x330]
1409e7fb1: 488b95c0010000 mov rdx, qword ptr [rbp + 0x1c0]
1409e7fb8: 48895510 mov qword ptr [rbp + 0x10], rdx
1409e7fbc: 0f1085b0010000 movups xmm0, xmmword ptr [rbp + 0x1b0]
1409e7fc3: 0f294500 movaps xmmword ptr [rbp], xmm0
1409e7fc7: 0f1085c8010000 movups xmm0, xmmword ptr [rbp + 0x1c8]
1409e7fce: 0f294530 movaps xmmword ptr [rbp + 0x30], xmm0
1409e7fd2: 488b95d8010000 mov rdx, qword ptr [rbp + 0x1d8]
1409e7fd9: 48895540 mov qword ptr [rbp + 0x40], rdx
1409e7fdd: 0f1085e0010000 movups xmm0, xmmword ptr [rbp + 0x1e0]
1409e7fe4: 0f114518 movups xmmword ptr [rbp + 0x18], xmm0
1409e7fe8: 488b95f0010000 mov rdx, qword ptr [rbp + 0x1f0]
1409e7fef: 48895528 mov qword ptr [rbp + 0x28], rdx
1409e7ff3: 0f108560020000 movups xmm0, xmmword ptr [rbp + 0x260]
1409e7ffa: 0f108d70020000 movups xmm1, xmmword ptr [rbp + 0x270]
1409e8001: 0f109580020000 movups xmm2, xmmword ptr [rbp + 0x280]
1409e8008: 0f109d90020000 movups xmm3, xmmword ptr [rbp + 0x290]
1409e800f: 0f114578 movups xmmword ptr [rbp + 0x78], xmm0
1409e8013: 0f118d88000000 movups xmmword ptr [rbp + 0x88], xmm1
1409e801a: 0f119598000000 movups xmmword ptr [rbp + 0x98], xmm2
1409e8021: 0f119da8000000 movups xmmword ptr [rbp + 0xa8], xmm3
1409e8028: 0f1085a0020000 movups xmm0, xmmword ptr [rbp + 0x2a0]
1409e802f: 0f1185b8000000 movups xmmword ptr [rbp + 0xb8], xmm0
1409e8036: 0f1085b0020000 movups xmm0, xmmword ptr [rbp + 0x2b0]
1409e803d: 0f1185c8000000 movups xmmword ptr [rbp + 0xc8], xmm0
1409e8044: 488b9508020000 mov rdx, qword ptr [rbp + 0x208]
1409e804b: 48895558 mov qword ptr [rbp + 0x58], rdx
1409e804f: 0f1085f8010000 movups xmm0, xmmword ptr [rbp + 0x1f8]
1409e8056: 0f114548 movups xmmword ptr [rbp + 0x48], xmm0
1409e805a: 48898de0000000 mov qword ptr [rbp + 0xe0], rcx
1409e8061: 488985d8000000 mov qword ptr [rbp + 0xd8], rax
1409e8068: 0f280501c50d02 movaps xmm0, xmmword ptr [rip + 0x20dc501]
1409e806f: 0f294560 movaps xmmword ptr [rbp + 0x60], xmm0
1409e8073: 488d8d40030000 lea rcx, [rbp + 0x340]
1409e807a: 4889ea mov rdx, rbp
1409e807d: e84e54ffff call 0x1409dd4d0
1409e8082: 80bde602000008 cmp byte ptr [rbp + 0x2e6], 8
1409e8089: 740c je 0x1409e8097
1409e808b: 488d8dd0020000 lea rcx, [rbp + 0x2d0]
1409e8092: e819a41f00 call 0x140be24b0
1409e8097: 488d8d40030000 lea rcx, [rbp + 0x340]
1409e809e: ba00001000 mov edx, 0x100000
1409e80a3: 41b820000000 mov r8d, 0x20
1409e80a9: e872cd1e00 call 0x140bd4e20
1409e80ae: 0f108540030000 movups xmm0, xmmword ptr [rbp + 0x340]
1409e80b5: 0f294500 movaps xmmword ptr [rbp], xmm0
1409e80b9: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1409e80c0: 4889ea mov rdx, rbp
1409e80c3: e838ec1e00 call 0x140bd6d00
1409e80c8: 488d05d0c90d02 lea rax, [rip + 0x20dc9d0]
1409e80cf: 488985b0010000 mov qword ptr [rbp + 0x1b0], rax
1409e80d6: 48c785b80100000d000000 mov qword ptr [rbp + 0x1b8], 0xd
1409e80e1: 488d95b0010000 lea rdx, [rbp + 0x1b0]
1409e80e8: 4889f9 mov rcx, rdi
1409e80eb: e8e0132000 call 0x140be94d0
1409e80f0: 0f108540030000 movups xmm0, xmmword ptr [rbp + 0x340]
1409e80f7: 0f2945c0 movaps xmmword ptr [rbp - 0x40], xmm0
1409e80fb: 8945d0 mov dword ptr [rbp - 0x30], eax
1409e80fe: 48897c2428 mov qword ptr [rsp + 0x28], rdi
1409e8103: 4c89742420 mov qword ptr [rsp + 0x20], r14
1409e8108: 488d159dc90d02 lea rdx, [rip + 0x20dc99d]
1409e810f: 488dbdb0010000 lea rdi, [rbp + 0x1b0]
1409e8116: 41b813000000 mov r8d, 0x13
1409e811c: 4889f9 mov rcx, rdi
1409e811f: 41b901000000 mov r9d, 1
1409e8125: e80660c100 call 0x1415fe130
1409e812a: 4989ee mov r14, rbp
1409e812d: 49b8ad2a9920eb336b0c movabs r8, 0xc6b33eb20992aad
1409e8137: 4c89f1 mov rcx, r14
1409e813a: 4889fa mov rdx, rdi
1409e813d: e83e55c200 call 0x14160d680
1409e8142: 49b8949d869af187e563 movabs r8, 0x63e587f19a869d94
1409e814c: 4c8d8d90010000 lea r9, [rbp + 0x190]
1409e8153: 4889f9 mov rcx, rdi
1409e8156: 4c89f2 mov rdx, r14
1409e8159: e8025cf8ff call 0x14096dd60
1409e815e: 4889e9 mov rcx, rbp
1409e8161: 49b81da36ad98bc7b8e3 movabs r8, 0xe3b8c78bd96aa31d
1409e816b: 4c8d4dc0 lea r9, [rbp - 0x40]
1409e816f: 4889fa mov rdx, rdi
1409e8172: e8e95bf8ff call 0x14096dd60
1409e8177: 80bd3601000008 cmp byte ptr [rbp + 0x136], 8
1409e817e: 7425 je 0x1409e81a5
1409e8180: 488d8d10010000 lea rcx, [rbp + 0x110]
1409e8187: bae8000000 mov edx, 0xe8
1409e818c: 48039538010000 add rdx, qword ptr [rbp + 0x138]
1409e8193: 49b89ff68f409e32889b movabs r8, 0x9b88329e408ff69f
1409e819d: 4189d9 mov r9d, ebx
1409e81a0: e8cb33f8ff call 0x14096b570
1409e81a5: 488d8db0010000 lea rcx, [rbp + 0x1b0]
1409e81ac: 4889ea mov rdx, rbp
1409e81af: 41b890010000 mov r8d, 0x190
1409e81b5: e846baf301 call 0x142923c00
1409e81ba: 0f108550020000 movups xmm0, xmmword ptr [rbp + 0x250]
1409e81c1: 0f298580030000 movaps xmmword ptr [rbp + 0x380], xmm0
1409e81c8: 0f108540020000 movups xmm0, xmmword ptr [rbp + 0x240]
1409e81cf: 0f298570030000 movaps xmmword ptr [rbp + 0x370], xmm0
1409e81d6: 0f108530020000 movups xmm0, xmmword ptr [rbp + 0x230]
1409e81dd: 0f298560030000 movaps xmmword ptr [rbp + 0x360], xmm0
1409e81e4: 0f108520020000 movups xmm0, xmmword ptr [rbp + 0x220]
1409e81eb: 0f298550030000 movaps xmmword ptr [rbp + 0x350], xmm0
1409e81f2: 0f108510020000 movups xmm0, xmmword ptr [rbp + 0x210]
1409e81f9: 0f298540030000 movaps xmmword ptr [rbp + 0x340], xmm0
1409e8200: 488b85e8020000 mov rax, qword ptr [rbp + 0x2e8]
1409e8207: 488b8d30030000 mov rcx, qword ptr [rbp + 0x330]
1409e820e: 488b95c0010000 mov rdx, qword ptr [rbp + 0x1c0]
1409e8215: 48895510 mov qword ptr [rbp + 0x10], rdx
1409e8219: 0f1085b0010000 movups xmm0, xmmword ptr [rbp + 0x1b0]
1409e8220: 0f294500 movaps xmmword ptr [rbp], xmm0
1409e8224: 0f1085c8010000 movups xmm0, xmmword ptr [rbp + 0x1c8]
1409e822b: 0f294530 movaps xmmword ptr [rbp + 0x30], xmm0
1409e822f: 488b95d8010000 mov rdx, qword ptr [rbp + 0x1d8]
1409e8236: 48895540 mov qword ptr [rbp + 0x40], rdx
1409e823a: 0f1085e0010000 movups xmm0, xmmword ptr [rbp + 0x1e0]
1409e8241: 0f114518 movups xmmword ptr [rbp + 0x18], xmm0
1409e8245: 488b95f0010000 mov rdx, qword ptr [rbp + 0x1f0]
1409e824c: 48895528 mov qword ptr [rbp + 0x28], rdx
1409e8250: 0f108560020000 movups xmm0, xmmword ptr [rbp + 0x260]
1409e8257: 0f108d70020000 movups xmm1, xmmword ptr [rbp + 0x270]
1409e825e: 0f109580020000 movups xmm2, xmmword ptr [rbp + 0x280]
1409e8265: 0f109d90020000 movups xmm3, xmmword ptr [rbp + 0x290]
1409e826c: 0f114578 movups xmmword ptr [rbp + 0x78], xmm0
1409e8270: 0f118d88000000 movups xmmword ptr [rbp + 0x88], xmm1
1409e8277: 0f119598000000 movups xmmword ptr [rbp + 0x98], xmm2
1409e827e: 0f119da8000000 movups xmmword ptr [rbp + 0xa8], xmm3
1409e8285: 0f1085a0020000 movups xmm0, xmmword ptr [rbp + 0x2a0]
1409e828c: 0f1185b8000000 movups xmmword ptr [rbp + 0xb8], xmm0
1409e8293: 0f1085b0020000 movups xmm0, xmmword ptr [rbp + 0x2b0]
1409e829a: 0f1185c8000000 movups xmmword ptr [rbp + 0xc8], xmm0
1409e82a1: 488b9508020000 mov rdx, qword ptr [rbp + 0x208]
1409e82a8: 48895558 mov qword ptr [rbp + 0x58], rdx
1409e82ac: 0f1085f8010000 movups xmm0, xmmword ptr [rbp + 0x1f8]
1409e82b3: 0f114548 movups xmmword ptr [rbp + 0x48], xmm0
1409e82b7: 48898de0000000 mov qword ptr [rbp + 0xe0], rcx
1409e82be: 488985d8000000 mov qword ptr [rbp + 0xd8], rax
1409e82c5: c7456001000000 mov dword ptr [rbp + 0x60], 1
1409e82cc: 895d64 mov dword ptr [rbp + 0x64], ebx
1409e82cf: 4c896d68 mov qword ptr [rbp + 0x68], r13
1409e82d3: 488d8d40030000 lea rcx, [rbp + 0x340]
1409e82da: 4889ea mov rdx, rbp
1409e82dd: e8ee51ffff call 0x1409dd4d0
1409e82e2: 80bde602000008 cmp byte ptr [rbp + 0x2e6], 8
1409e82e9: 740c je 0x1409e82f7
1409e82eb: 488d8dd0020000 lea rcx, [rbp + 0x2d0]
1409e82f2: e8b9a11f00 call 0x140be24b0
1409e82f7: 488b85a0010000 mov rax, qword ptr [rbp + 0x1a0]
1409e82fe: 48894610 mov qword ptr [rsi + 0x10], rax
1409e8302: 0f288590010000 movaps xmm0, xmmword ptr [rbp + 0x190]
1409e8309: 0f1106 movups xmmword ptr [rsi], xmm0
1409e830c: 0f2845c0 movaps xmm0, xmmword ptr [rbp - 0x40]
1409e8310: 0f114618 movups xmmword ptr [rsi + 0x18], xmm0
1409e8314: 488b45d0 mov rax, qword ptr [rbp - 0x30]
1409e8318: 48894628 mov qword ptr [rsi + 0x28], rax
1409e831c: e9c6000000 jmp 0x1409e83e7
1409e8321: ba04000000 mov edx, 4
1409e8326: 41b820000000 mov r8d, 0x20
1409e832c: e8efca1e00 call 0x140bd4e20
1409e8331: 0f108540030000 movups xmm0, xmmword ptr [rbp + 0x340]
1409e8338: 0f294500 movaps xmmword ptr [rbp], xmm0
1409e833c: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1409e8343: 4889ea mov rdx, rbp
1409e8346: e8b5e91e00 call 0x140bd6d00
1409e834b: 488d050ec60d02 lea rax, [rip + 0x20dc60e]
1409e8352: 488985b0010000 mov qword ptr [rbp + 0x1b0], rax
1409e8359: 48c785b801000022000000 mov qword ptr [rbp + 0x1b8], 0x22
1409e8364: 488d95b0010000 lea rdx, [rbp + 0x1b0]
1409e836b: 4889f9 mov rcx, rdi
1409e836e: e85d112000 call 0x140be94d0
1409e8373: 0f108540030000 movups xmm0, xmmword ptr [rbp + 0x340]
1409e837a: 0f1106 movups xmmword ptr [rsi], xmm0
1409e837d: 894610 mov dword ptr [rsi + 0x10], eax
1409e8380: 488d8d40030000 lea rcx, [rbp + 0x340]
1409e8387: ba04000000 mov edx, 4
1409e838c: 41b820000000 mov r8d, 0x20
1409e8392: e889ca1e00 call 0x140bd4e20
1409e8397: 0f108540030000 movups xmm0, xmmword ptr [rbp + 0x340]
1409e839e: 0f294500 movaps xmmword ptr [rbp], xmm0
1409e83a2: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1409e83a9: 4889ea mov rdx, rbp
1409e83ac: e84fe91e00 call 0x140bd6d00
1409e83b1: 488d05cac50d02 lea rax, [rip + 0x20dc5ca]
1409e83b8: 488985b0010000 mov qword ptr [rbp + 0x1b0], rax
1409e83bf: 48c785b801000019000000 mov qword ptr [rbp + 0x1b8], 0x19
1409e83ca: 488d95b0010000 lea rdx, [rbp + 0x1b0]
1409e83d1: 4889f9 mov rcx, rdi
1409e83d4: e8f7102000 call 0x140be94d0
1409e83d9: 0f108540030000 movups xmm0, xmmword ptr [rbp + 0x340]
1409e83e0: 0f114618 movups xmmword ptr [rsi + 0x18], xmm0
1409e83e4: 894628 mov dword ptr [rsi + 0x28], eax
1409e83e7: 4889f0 mov rax, rsi
1409e83ea: 4881c418040000 add rsp, 0x418
1409e83f1: 5b pop rbx
1409e83f2: 5f pop rdi
1409e83f3: 5e pop rsi
1409e83f4: 415c pop r12
1409e83f6: 415d pop r13
1409e83f8: 415e pop r14
1409e83fa: 415f pop r15
1409e83fc: 5d pop rbp
1409e83fd: c3 ret 
1409e83fe: 48c785b001000000000000 mov qword ptr [rbp + 0x1b0], 0
1409e8409: 4883c310 add rbx, 0x10
1409e840d: 488d056c5d2402 lea rax, [rip + 0x2245d6c]
1409e8414: 4889442420 mov qword ptr [rsp + 0x20], rax
1409e8419: 4889ea mov rdx, rbp
1409e841c: 4c8d8db0010000 lea r9, [rbp + 0x1b0]
1409e8423: 31c9 xor ecx, ecx
1409e8425: 4989d8 mov r8, rbx
1409e8428: e81427ee01 call 0x1428cab41
1409e842d: 488d0dfcc40d02 lea rcx, [rip + 0x20dc4fc]
1409e8434: e8d70eef01 call 0x1428d9310
1409e8439: 488d0d08c50d02 lea rcx, [rip + 0x20dc508]
1409e8440: e8cb0eef01 call 0x1428d9310
1409e8445: 4c8d0584542902 lea r8, [rip + 0x2295484]
1409e844c: b901000000 mov ecx, 1
1409e8451: ba04000000 mov edx, 4
1409e8456: e8880bef01 call 0x1428d8fe3
1409e845b: 0f0b ud2 
1409e845d: 0f1f00 nop dword ptr [rax]
1409e8460: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409e8465: 55 push rbp
1409e8466: 4157 push r15
1409e8468: 4156 push r14
1409e846a: 4155 push r13
1409e846c: 4154 push r12
1409e846e: 56 push rsi
1409e846f: 57 push rdi
1409e8470: 53 push rbx
1409e8471: 4883ec38 sub rsp, 0x38
1409e8475: 488daa80000000 lea rbp, [rdx + 0x80]
1409e847c: 488d8db0010000 lea rcx, [rbp + 0x1b0]
1409e8483: e828ebeaff call 0x140896fb0
1409e8488: 90 nop 
1409e8489: 4883c438 add rsp, 0x38
1409e848d: 5b pop rbx
1409e848e: 5f pop rdi
1409e848f: 5e pop rsi
1409e8490: 415c pop r12
1409e8492: 415d pop r13
1409e8494: 415e pop r14
1409e8496: 415f pop r15
1409e8498: 5d pop rbp
1409e8499: c3 ret 
1409e849a: 660f1f440000 nop word ptr [rax + rax]
1409e84a0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409e84a5: 55 push rbp
1409e84a6: 4157 push r15
1409e84a8: 4156 push r14
1409e84aa: 4155 push r13
1409e84ac: 4154 push r12
1409e84ae: 56 push rsi
1409e84af: 57 push rdi
1409e84b0: 53 push rbx
1409e84b1: 4883ec38 sub rsp, 0x38
1409e84b5: 488daa80000000 lea rbp, [rdx + 0x80]
1409e84bc: 80bd3601000008 cmp byte ptr [rbp + 0x136], 8
1409e84c3: 740c je 0x1409e84d1
1409e84c5: 488d8d20010000 lea rcx, [rbp + 0x120]
1409e84cc: e8df9f1f00 call 0x140be24b0
1409e84d1: 90 nop 
1409e84d2: 4883c438 add rsp, 0x38
1409e84d6: 5b pop rbx
1409e84d7: 5f pop rdi
1409e84d8: 5e pop rsi
1409e84d9: 415c pop r12
1409e84db: 415d pop r13
1409e84dd: 415e pop r14
1409e84df: 415f pop r15
1409e84e1: 5d pop rbp
1409e84e2: c3 ret 
1409e84e3: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409e84f0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409e84f5: 55 push rbp
1409e84f6: 4157 push r15
1409e84f8: 4156 push r14
1409e84fa: 4155 push r13
1409e84fc: 4154 push r12
1409e84fe: 56 push rsi
1409e84ff: 57 push rdi
1409e8500: 53 push rbx
1409e8501: 4883ec38 sub rsp, 0x38
1409e8505: 488daa80000000 lea rbp, [rdx + 0x80]
1409e850c: 80bde602000008 cmp byte ptr [rbp + 0x2e6], 8
1409e8513: 740c je 0x1409e8521
1409e8515: 488d8dd0020000 lea rcx, [rbp + 0x2d0]
1409e851c: e88f9f1f00 call 0x140be24b0
1409e8521: 90 nop 
1409e8522: 4883c438 add rsp, 0x38
1409e8526: 5b pop rbx
1409e8527: 5f pop rdi
1409e8528: 5e pop rsi
1409e8529: 415c pop r12
1409e852b: 415d pop r13
1409e852d: 415e pop r14
1409e852f: 415f pop r15
1409e8531: 5d pop rbp
1409e8532: c3 ret 
1409e8533: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409e8540: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409e8545: 55 push rbp
1409e8546: 4157 push r15
1409e8548: 4156 push r14
1409e854a: 4155 push r13
1409e854c: 4154 push r12
1409e854e: 56 push rsi
1409e854f: 57 push rdi
1409e8550: 53 push rbx
1409e8551: 4883ec38 sub rsp, 0x38
1409e8555: 488daa80000000 lea rbp, [rdx + 0x80]
1409e855c: 4889e9 mov rcx, rbp
1409e855f: e84ceaeaff call 0x140896fb0
1409e8564: 90 nop 
1409e8565: 4883c438 add rsp, 0x38
1409e8569: 5b pop rbx
1409e856a: 5f pop rdi
1409e856b: 5e pop rsi
1409e856c: 415c pop r12
1409e856e: 415d pop r13
1409e8570: 415e pop r14
1409e8572: 415f pop r15
1409e8574: 5d pop rbp
1409e8575: c3 ret 
1409e8576: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409e8580: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409e8585: 55 push rbp
1409e8586: 4157 push r15
1409e8588: 4156 push r14
1409e858a: 4155 push r13
1409e858c: 4154 push r12
1409e858e: 56 push rsi
1409e858f: 57 push rdi
1409e8590: 53 push rbx
1409e8591: 4883ec38 sub rsp, 0x38
1409e8595: 488daa80000000 lea rbp, [rdx + 0x80]
1409e859c: 80bde602000008 cmp byte ptr [rbp + 0x2e6], 8
1409e85a3: 740c je 0x1409e85b1
1409e85a5: 488d8dd0020000 lea rcx, [rbp + 0x2d0]
1409e85ac: e8ff9e1f00 call 0x140be24b0
1409e85b1: 90 nop 
1409e85b2: 4883c438 add rsp, 0x38
1409e85b6: 5b pop rbx
1409e85b7: 5f pop rdi
1409e85b8: 5e pop rsi
1409e85b9: 415c pop r12
1409e85bb: 415d pop r13
1409e85bd: 415e pop r14
1409e85bf: 415f pop r15
1409e85c1: 5d pop rbp
1409e85c2: c3 ret 
1409e85c3: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409e85d0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409e85d5: 55 push rbp
1409e85d6: 4157 push r15
1409e85d8: 4156 push r14
1409e85da: 4155 push r13
1409e85dc: 4154 push r12
1409e85de: 56 push rsi
1409e85df: 57 push rdi
1409e85e0: 53 push rbx
1409e85e1: 4883ec38 sub rsp, 0x38
1409e85e5: 488daa80000000 lea rbp, [rdx + 0x80]
1409e85ec: 80bde602000008 cmp byte ptr [rbp + 0x2e6], 8
1409e85f3: 740c je 0x1409e8601
1409e85f5: 488d8dd0020000 lea rcx, [rbp + 0x2d0]
1409e85fc: e8af9e1f00 call 0x140be24b0
1409e8601: 90 nop 
1409e8602: 4883c438 add rsp, 0x38
1409e8606: 5b pop rbx
1409e8607: 5f pop rdi
1409e8608: 5e pop rsi
1409e8609: 415c pop r12
1409e860b: 415d pop r13
1409e860d: 415e pop r14
1409e860f: 415f pop r15
1409e8611: 5d pop rbp
1409e8612: c3 ret 
1409e8613: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409e8620: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409e8625: 55 push rbp
1409e8626: 4157 push r15
1409e8628: 4156 push r14
1409e862a: 4155 push r13
1409e862c: 4154 push r12
1409e862e: 56 push rsi
1409e862f: 57 push rdi
1409e8630: 53 push rbx
1409e8631: 4883ec38 sub rsp, 0x38
1409e8635: 488daa80000000 lea rbp, [rdx + 0x80]
1409e863c: 4889e9 mov rcx, rbp
1409e863f: e86ce9eaff call 0x140896fb0
1409e8644: 90 nop 
1409e8645: 4883c438 add rsp, 0x38
1409e8649: 5b pop rbx
1409e864a: 5f pop rdi
1409e864b: 5e pop rsi
1409e864c: 415c pop r12
1409e864e: 415d pop r13
1409e8650: 415e pop r14
1409e8652: 415f pop r15
1409e8654: 5d pop rbp
1409e8655: c3 ret 
1409e8656: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409e8660: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409e8665: 55 push rbp
1409e8666: 4157 push r15
1409e8668: 4156 push r14
1409e866a: 4155 push r13
1409e866c: 4154 push r12
1409e866e: 56 push rsi
1409e866f: 57 push rdi
1409e8670: 53 push rbx
1409e8671: 4883ec38 sub rsp, 0x38
1409e8675: 488daa80000000 lea rbp, [rdx + 0x80]
1409e867c: 80bde602000008 cmp byte ptr [rbp + 0x2e6], 8
1409e8683: 740c je 0x1409e8691
1409e8685: 488d8dd0020000 lea rcx, [rbp + 0x2d0]
1409e868c: e81f9e1f00 call 0x140be24b0
1409e8691: 90 nop 
1409e8692: 4883c438 add rsp, 0x38
1409e8696: 5b pop rbx
1409e8697: 5f pop rdi
1409e8698: 5e pop rsi
1409e8699: 415c pop r12
1409e869b: 415d pop r13
1409e869d: 415e pop r14
1409e869f: 415f pop r15
1409e86a1: 5d pop rbp
1409e86a2: c3 ret 
