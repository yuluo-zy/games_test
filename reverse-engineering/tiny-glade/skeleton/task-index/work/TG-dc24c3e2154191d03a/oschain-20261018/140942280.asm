140942280: push rsi
140942281: sub rsp, 0xb0
140942288: mov eax, dword ptr [rcx + 0x1c]
14094228b: mov rdx, qword ptr [rcx + 0x10]
14094228f: mov dword ptr [rdx], eax
140942291: mov rsi, qword ptr [rcx]
140942294: movabs rax, 0x8000000000000002
14094229e: cmp qword ptr [rsi + 0x40], rax
1409422a2: jne 0x1409423bd
1409422a8: mov rax, qword ptr [rsi + 0x48]
1409422ac: inc rax
1409422af: mov qword ptr [rsi + 0x48], rax
1409422b3: cmp rax, qword ptr [rsi + 0x38]
1409422b7: jb 0x1409423bd
1409422bd: mov rax, qword ptr [rip + 0x2cea0e4]
1409422c4: mov rax, qword ptr [rax]
1409422c7: cmp rax, 3
1409422cb: jb 0x1409423af
1409422d1: lea rcx, [rip + 0x2170548]
1409422d8: call 0x14067dd80
1409422dd: movups xmm0, xmmword ptr [rax]
1409422e0: mov eax, dword ptr [rax + 0x10]
1409422e3: mov qword ptr [rsp + 0x60], 3
1409422ec: lea rcx, [rip + 0x2170385]
1409422f3: mov qword ptr [rsp + 0x68], rcx
1409422f8: mov qword ptr [rsp + 0x70], 0x3a
140942301: lea rdx, [rip + 0x2170508]
140942308: mov qword ptr [rsp + 0x80], rdx
140942310: mov qword ptr [rsp + 0x88], 1
14094231c: mov qword ptr [rsp + 0x90], 8
140942328: mov qword ptr [rsp + 0x30], 0
140942331: xorps xmm1, xmm1
140942334: movups xmmword ptr [rsp + 0x98], xmm1
14094233c: mov qword ptr [rsp + 0x38], rcx
140942341: mov qword ptr [rsp + 0x40], 0x3a
14094234a: mov qword ptr [rsp + 0x48], 0
140942353: movups xmmword ptr [rsp + 0x50], xmm0
140942358: mov dword ptr [rsp + 0x78], 1
140942360: mov dword ptr [rsp + 0x7c], eax
140942364: lea rcx, [rsp + 0x2f]
140942369: lea rdx, [rsp + 0x30]
14094236e: call 0x140d4a330
140942373: mov rdx, qword ptr [rsi + 0x40]
140942377: movabs rax, 0x8000000000000005
140942381: cmp rdx, rax
140942384: setl al
140942387: movabs rcx, 0x8000000000000001
140942391: cmp rdx, rcx
140942394: setne cl
140942397: test al, cl
140942399: jne 0x1409423af
14094239b: test rdx, rdx
14094239e: je 0x1409423af
1409423a0: mov rcx, qword ptr [rsi + 0x48]
1409423a4: mov r8d, 1
1409423aa: call 0x140613c20
1409423af: movabs rax, 0x8000000000000004
1409423b9: mov qword ptr [rsi + 0x40], rax
1409423bd: add rsp, 0xb0
1409423c4: pop rsi
1409423c5: ret 
