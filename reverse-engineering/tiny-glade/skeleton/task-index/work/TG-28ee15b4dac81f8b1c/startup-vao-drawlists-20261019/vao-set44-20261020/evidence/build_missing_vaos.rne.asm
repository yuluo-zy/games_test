00000001408646b0: 55                       push rbp
00000001408646b1: 4157                     push r15
00000001408646b3: 4156                     push r14
00000001408646b5: 4155                     push r13
00000001408646b7: 4154                     push r12
00000001408646b9: 56                       push rsi
00000001408646ba: 57                       push rdi
00000001408646bb: 53                       push rbx
00000001408646bc: 4881ec48010000           sub rsp, 0x148
00000001408646c3: 488dac2480000000         lea rbp, [rsp + 0x80]
00000001408646cb: 0f29b5b0000000           movaps xmmword ptr [rbp + 0xb0], xmm6
00000001408646d2: 48c785a8000000feffffff   mov qword ptr [rbp + 0xa8], 0xfffffffffffffffe
00000001408646dd: 4c8b9530010000           mov r10, qword ptr [rbp + 0x130]
00000001408646e4: 4c8b19                   mov r11, qword ptr [rcx]
00000001408646e7: 498b9bf8000000           mov rbx, qword ptr [r11 + 0xf8]
00000001408646ee: 498b8300010000           mov rax, qword ptr [r11 + 0x100]
00000001408646f5: 4c8b2a                   mov r13, qword ptr [rdx]
00000001408646f8: 498b5208                 mov rdx, qword ptr [r10 + 8]
00000001408646fc: 48899590000000           mov qword ptr [rbp + 0x90], rdx
0000000140864703: 4885d2                   test rdx, rdx
0000000140864706: 750b                     jne 0x140864713
0000000140864708: 498b5210                 mov rdx, qword ptr [r10 + 0x10]
000000014086470c: 48899590000000           mov qword ptr [rbp + 0x90], rdx
0000000140864713: 4c8b7108                 mov r14, qword ptr [rcx + 8]
0000000140864717: 488d3483                 lea rsi, [rbx + rax*4]
000000014086471b: 4c895d70                 mov qword ptr [rbp + 0x70], r11
000000014086471f: 410fb68308010000         movzx eax, byte ptr [r11 + 0x108]
0000000140864727: 8885a7000000             mov byte ptr [rbp + 0xa7], al
000000014086472d: 418b411c                 mov eax, dword ptr [r9 + 0x1c]
0000000140864731: 89457c                   mov dword ptr [rbp + 0x7c], eax
0000000140864734: 498b01                   mov rax, qword ptr [r9]
0000000140864737: 488945c0                 mov qword ptr [rbp - 0x40], rax
000000014086473b: 498b4110                 mov rax, qword ptr [r9 + 0x10]
000000014086473f: 488945c8                 mov qword ptr [rbp - 0x38], rax
0000000140864743: 498b00                   mov rax, qword ptr [r8]
0000000140864746: 488945d0                 mov qword ptr [rbp - 0x30], rax
000000014086474a: 498d4520                 lea rax, [r13 + 0x20]
000000014086474e: 488945b8                 mov qword ptr [rbp - 0x48], rax
0000000140864752: b808000000               mov eax, 8
0000000140864757: 48894560                 mov qword ptr [rbp + 0x60], rax
000000014086475b: 48c7859800000000000000   mov qword ptr [rbp + 0x98], 0
0000000140864766: 0f57f6                   xorps xmm6, xmm6
0000000140864769: 48894568                 mov qword ptr [rbp + 0x68], rax
000000014086476d: 4531ff                   xor r15d, r15d
0000000140864770: 4531c0                   xor r8d, r8d
0000000140864773: 4c896d58                 mov qword ptr [rbp + 0x58], r13
0000000140864777: eb1a                     jmp 0x140864793
0000000140864779: 0f1f8000000000           nop dword ptr [rax]
0000000140864780: 488d8d80000000           lea rcx, [rbp + 0x80]
0000000140864787: e8a4844e00               call 0x140d4cc30
000000014086478c: 4c8b8588000000           mov r8, qword ptr [rbp + 0x88]
0000000140864793: 80bda700000000           cmp byte ptr [rbp + 0xa7], 0
000000014086479a: 0f8480000000             je 0x140864820
00000001408647a0: 4d39f8                   cmp r8, r15
00000001408647a3: 0f8519010000             jne 0x1408648c2
00000001408647a9: 0f1f8000000000           nop dword ptr [rax]
00000001408647b0: 4839f3                   cmp rbx, rsi
00000001408647b3: 0f84c5030000             je 0x140864b7e
00000001408647b9: 8b0b                     mov ecx, dword ptr [rbx]
00000001408647bb: 4883c304                 add rbx, 4
00000001408647bf: 498b86a8010000           mov rax, qword ptr [r14 + 0x1a8]
00000001408647c6: 488d0cc9                 lea rcx, [rcx + rcx*8]
00000001408647ca: 4c8b7cc810               mov r15, qword ptr [rax + rcx*8 + 0x10]
00000001408647cf: 4d85ff                   test r15, r15
00000001408647d2: 74dc                     je 0x1408647b0
00000001408647d4: 488d04c8                 lea rax, [rax + rcx*8]
00000001408647d8: 488b4d70                 mov rcx, qword ptr [rbp + 0x70]
00000001408647dc: 488b8910010000           mov rcx, qword ptr [rcx + 0x110]
00000001408647e3: 48394840                 cmp qword ptr [rax + 0x40], rcx
00000001408647e7: 0f86bf000000             jbe 0x1408648ac
00000001408647ed: 488b5038                 mov rdx, qword ptr [rax + 0x38]
00000001408647f1: 488b0cca                 mov rcx, qword ptr [rdx + rcx*8]
00000001408647f5: 4885c9                   test rcx, rcx
00000001408647f8: 0f84ae000000             je 0x1408648ac
00000001408647fe: 488b5018                 mov rdx, qword ptr [rax + 0x18]
0000000140864802: 48f7d1                   not rcx
0000000140864805: 488d0c49                 lea rcx, [rcx + rcx*2]
0000000140864809: 48c1e104                 shl rcx, 4
000000014086480d: 488b4c0a10               mov rcx, qword ptr [rdx + rcx + 0x10]
0000000140864812: 48898d98000000           mov qword ptr [rbp + 0x98], rcx
0000000140864819: e999000000               jmp 0x1408648b7
000000014086481e: 6690                     nop 
0000000140864820: 4d39f8                   cmp r8, r15
0000000140864823: 0f85bc000000             jne 0x1408648e5
0000000140864829: 0f1f8000000000           nop dword ptr [rax]
0000000140864830: 4839f3                   cmp rbx, rsi
0000000140864833: 0f8445030000             je 0x140864b7e
0000000140864839: 8b0b                     mov ecx, dword ptr [rbx]
000000014086483b: 4883c304                 add rbx, 4
000000014086483f: 498b8600010000           mov rax, qword ptr [r14 + 0x100]
0000000140864846: 488d0c89                 lea rcx, [rcx + rcx*4]
000000014086484a: 48c1e105                 shl rcx, 5
000000014086484e: 4c8b7c0858               mov r15, qword ptr [rax + rcx + 0x58]
0000000140864853: 4d85ff                   test r15, r15
0000000140864856: 74d8                     je 0x140864830
0000000140864858: 4801c8                   add rax, rcx
000000014086485b: 8b8894000000             mov ecx, dword ptr [rax + 0x94]
0000000140864861: 498b96a8010000           mov rdx, qword ptr [r14 + 0x1a8]
0000000140864868: 4c8d04c9                 lea r8, [rcx + rcx*8]
000000014086486c: 488b4d70                 mov rcx, qword ptr [rbp + 0x70]
0000000140864870: 488b8910010000           mov rcx, qword ptr [rcx + 0x110]
0000000140864877: 4a394cc240               cmp qword ptr [rdx + r8*8 + 0x40], rcx
000000014086487c: 7651                     jbe 0x1408648cf
000000014086487e: 4a8d14c2                 lea rdx, [rdx + r8*8]
0000000140864882: 4c8b4238                 mov r8, qword ptr [rdx + 0x38]
0000000140864886: 498b0cc8                 mov rcx, qword ptr [r8 + rcx*8]
000000014086488a: 4885c9                   test rcx, rcx
000000014086488d: 7440                     je 0x1408648cf
000000014086488f: 488b5218                 mov rdx, qword ptr [rdx + 0x18]
0000000140864893: 48f7d1                   not rcx
0000000140864896: 488d0c49                 lea rcx, [rcx + rcx*2]
000000014086489a: 48c1e104                 shl rcx, 4
000000014086489e: 488b4c0a10               mov rcx, qword ptr [rdx + rcx + 0x10]
00000001408648a3: 48898d98000000           mov qword ptr [rbp + 0x98], rcx
00000001408648aa: eb2e                     jmp 0x1408648da
00000001408648ac: 48c7859800000000000000   mov qword ptr [rbp + 0x98], 0
00000001408648b7: 488b4008                 mov rax, qword ptr [rax + 8]
00000001408648bb: 48894560                 mov qword ptr [rbp + 0x60], rax
00000001408648bf: 4531c0                   xor r8d, r8d
00000001408648c2: 488b4560                 mov rax, qword ptr [rbp + 0x60]
00000001408648c6: 4a8d04c0                 lea rax, [rax + r8*8]
00000001408648ca: 4489c1                   mov ecx, r8d
00000001408648cd: eb29                     jmp 0x1408648f8
00000001408648cf: 48c7859800000000000000   mov qword ptr [rbp + 0x98], 0
00000001408648da: 488b4050                 mov rax, qword ptr [rax + 0x50]
00000001408648de: 48894568                 mov qword ptr [rbp + 0x68], rax
00000001408648e2: 4531c0                   xor r8d, r8d
00000001408648e5: 4c89c1                   mov rcx, r8
00000001408648e8: 48c1e104                 shl rcx, 4
00000001408648ec: 488b5568                 mov rdx, qword ptr [rbp + 0x68]
00000001408648f0: 488d040a                 lea rax, [rdx + rcx]
00000001408648f4: 8b4c0a08                 mov ecx, dword ptr [rdx + rcx + 8]
00000001408648f8: 488b9598000000           mov rdx, qword ptr [rbp + 0x98]
00000001408648ff: 4c8d24ca                 lea r12, [rdx + rcx*8]
0000000140864903: 488b38                   mov rdi, qword ptr [rax]
0000000140864906: 4889f8                   mov rax, rdi
0000000140864909: 48c1e820                 shr rax, 0x20
000000014086490d: 0f846b020000             je 0x140864b7e
0000000140864913: 49ffc0                   inc r8
0000000140864916: 4c898588000000           mov qword ptr [rbp + 0x88], r8
000000014086491d: 498b0424                 mov rax, qword ptr [r12]
0000000140864921: 488945d8                 mov qword ptr [rbp - 0x28], rax
0000000140864925: 488b4db8                 mov rcx, qword ptr [rbp - 0x48]
0000000140864929: 488d55d8                 lea rdx, [rbp - 0x28]
000000014086492d: e81e4c1000               call 0x140969550 ; AssetMeshLibrary::get
0000000140864932: 4885c0                   test rax, rax
0000000140864935: 740b                     je 0x140864942
0000000140864937: 4883781000               cmp qword ptr [rax + 0x10], 0
000000014086493c: 0f845c010000             je 0x140864a9e
0000000140864942: 8b457c                   mov eax, dword ptr [rbp + 0x7c]
0000000140864945: 488b4dc8                 mov rcx, qword ptr [rbp - 0x38]
0000000140864949: 8901                     mov dword ptr [rcx], eax
000000014086494b: 488b45d0                 mov rax, qword ptr [rbp - 0x30]
000000014086494f: 488b10                   mov rdx, qword ptr [rax]
0000000140864952: 4883c210                 add rdx, 0x10
0000000140864956: 4d8b0c24                 mov r9, qword ptr [r12]
000000014086495a: 488b4dc0                 mov rcx, qword ptr [rbp - 0x40]
000000014086495e: 4d89e8                   mov r8, r13
0000000140864961: e81af80b00               call 0x140924180 ; AssetVAOLibrary::add
0000000140864966: 4989d5                   mov r13, rdx
0000000140864969: 4885c0                   test rax, rax
000000014086496c: 0f84ce000000             je 0x140864a40
0000000140864972: 4c89ad80000000           mov qword ptr [rbp + 0x80], r13
0000000140864979: 488b05287adc02           mov rax, qword ptr [rip + 0x2dc7a28]
0000000140864980: 488b00                   mov rax, qword ptr [rax]
0000000140864983: 4883f802                 cmp rax, 2
0000000140864987: 4c8b6d58                 mov r13, qword ptr [rbp + 0x58]
000000014086498b: 0f82effdffff             jb 0x140864780
0000000140864991: 488d8580000000           lea rax, [rbp + 0x80]
0000000140864998: 488945a8                 mov qword ptr [rbp - 0x58], rax
000000014086499c: 488d056d824e00           lea rax, [rip + 0x4e826d]
00000001408649a3: 488945b0                 mov qword ptr [rbp - 0x50], rax
00000001408649a7: 488d0df21a2302           lea rcx, [rip + 0x2231af2]
00000001408649ae: e8cd93e1ff               call 0x14067dd80
00000001408649b3: 0f1000                   movups xmm0, xmmword ptr [rax]
00000001408649b6: 8b4010                   mov eax, dword ptr [rax + 0x10]
00000001408649b9: 48c7450802000000         mov qword ptr [rbp + 8], 2
00000001408649c1: 488d0df01a2302           lea rcx, [rip + 0x2231af0]
00000001408649c8: 48894d10                 mov qword ptr [rbp + 0x10], rcx
00000001408649cc: 48c745182a000000         mov qword ptr [rbp + 0x18], 0x2a
00000001408649d4: 488d15b51a2302           lea rdx, [rip + 0x2231ab5]
00000001408649db: 48895528                 mov qword ptr [rbp + 0x28], rdx
00000001408649df: 48c7453001000000         mov qword ptr [rbp + 0x30], 1
00000001408649e7: 488d55a8                 lea rdx, [rbp - 0x58]
00000001408649eb: 48895538                 mov qword ptr [rbp + 0x38], rdx
00000001408649ef: 48c7454001000000         mov qword ptr [rbp + 0x40], 1
00000001408649f7: 48c7454800000000         mov qword ptr [rbp + 0x48], 0
00000001408649ff: 48c745d800000000         mov qword ptr [rbp - 0x28], 0
0000000140864a07: 48894de0                 mov qword ptr [rbp - 0x20], rcx
0000000140864a0b: 48c745e82a000000         mov qword ptr [rbp - 0x18], 0x2a
0000000140864a13: 48c745f000000000         mov qword ptr [rbp - 0x10], 0
0000000140864a1b: 0f1145f8                 movups xmmword ptr [rbp - 8], xmm0
0000000140864a1f: c7452001000000           mov dword ptr [rbp + 0x20], 1
0000000140864a26: 894524                   mov dword ptr [rbp + 0x24], eax
0000000140864a29: 488d8da6000000           lea rcx, [rbp + 0xa6]
0000000140864a30: 488d55d8                 lea rdx, [rbp - 0x28]
0000000140864a34: e8f7584e00               call 0x140d4a330 ; log dispatch
0000000140864a39: e942fdffff               jmp 0x140864780
0000000140864a3e: 6690                     nop 
0000000140864a40: 488b8d90000000           mov rcx, qword ptr [rbp + 0x90]
0000000140864a47: 488b01                   mov rax, qword ptr [rcx]
0000000140864a4a: 4c8b6110                 mov r12, qword ptr [rcx + 0x10]
0000000140864a4e: 4c29e0                   sub rax, r12
0000000140864a51: 4883f827                 cmp rax, 0x27
0000000140864a55: 0f86f3000000             jbe 0x140864b4e
0000000140864a5b: 488b4108                 mov rax, qword ptr [rcx + 8]
0000000140864a5f: 488d154a45a700           lea rdx, [rip + 0xa7454a]
0000000140864a66: 4a891420                 mov qword ptr [rax + r12], rdx
0000000140864a6a: 488d15bf02d000           lea rdx, [rip + 0xd002bf]
0000000140864a71: 4a89542008               mov qword ptr [rax + r12 + 8], rdx
0000000140864a76: 4a897c2010               mov qword ptr [rax + r12 + 0x10], rdi
0000000140864a7b: 4e896c2018               mov qword ptr [rax + r12 + 0x18], r13
0000000140864a80: 42c644202000             mov byte ptr [rax + r12 + 0x20], 0
0000000140864a86: 4983c428                 add r12, 0x28
0000000140864a8a: 4c896110                 mov qword ptr [rcx + 0x10], r12
0000000140864a8e: 4c8b6d58                 mov r13, qword ptr [rbp + 0x58]
0000000140864a92: 4c8b8588000000           mov r8, qword ptr [rbp + 0x88]
0000000140864a99: e9f5fcffff               jmp 0x140864793
0000000140864a9e: 488b050379dc02           mov rax, qword ptr [rip + 0x2dc7903]
0000000140864aa5: 488b00                   mov rax, qword ptr [rax]
0000000140864aa8: 4883f805                 cmp rax, 5
0000000140864aac: 4c8b8588000000           mov r8, qword ptr [rbp + 0x88]
0000000140864ab3: 0f82dafcffff             jb 0x140864793
0000000140864ab9: 488d0d581a2302           lea rcx, [rip + 0x2231a58]
0000000140864ac0: e8bb92e1ff               call 0x14067dd80
0000000140864ac5: 0f1000                   movups xmm0, xmmword ptr [rax]
0000000140864ac8: 8b4010                   mov eax, dword ptr [rax + 0x10]
0000000140864acb: 48c7450805000000         mov qword ptr [rbp + 8], 5
0000000140864ad3: 488d15de192302           lea rdx, [rip + 0x22319de]
0000000140864ada: 48895510                 mov qword ptr [rbp + 0x10], rdx
0000000140864ade: 48c745182a000000         mov qword ptr [rbp + 0x18], 0x2a
0000000140864ae6: 488d0d1b1a2302           lea rcx, [rip + 0x2231a1b]
0000000140864aed: 48894d28                 mov qword ptr [rbp + 0x28], rcx
0000000140864af1: 48c7453001000000         mov qword ptr [rbp + 0x30], 1
0000000140864af9: 48c7453808000000         mov qword ptr [rbp + 0x38], 8
0000000140864b01: 48c745d800000000         mov qword ptr [rbp - 0x28], 0
0000000140864b09: 488d4d40                 lea rcx, [rbp + 0x40]
0000000140864b0d: 0f1131                   movups xmmword ptr [rcx], xmm6
0000000140864b10: 488955e0                 mov qword ptr [rbp - 0x20], rdx
0000000140864b14: 48c745e82a000000         mov qword ptr [rbp - 0x18], 0x2a
0000000140864b1c: 48c745f000000000         mov qword ptr [rbp - 0x10], 0
0000000140864b24: 0f1145f8                 movups xmmword ptr [rbp - 8], xmm0
0000000140864b28: c7452001000000           mov dword ptr [rbp + 0x20], 1
0000000140864b2f: 894524                   mov dword ptr [rbp + 0x24], eax
0000000140864b32: 488d8da6000000           lea rcx, [rbp + 0xa6]
0000000140864b39: 488d55d8                 lea rdx, [rbp - 0x28]
0000000140864b3d: e8ee574e00               call 0x140d4a330 ; log dispatch
0000000140864b42: 4c8b8588000000           mov r8, qword ptr [rbp + 0x88]
0000000140864b49: e945fcffff               jmp 0x140864793
0000000140864b4e: 48c744242001000000       mov qword ptr [rsp + 0x20], 1
0000000140864b57: 41b828000000             mov r8d, 0x28
0000000140864b5d: 41b901000000             mov r9d, 1
0000000140864b63: 488b8d90000000           mov rcx, qword ptr [rbp + 0x90]
0000000140864b6a: 4c89e2                   mov rdx, r12
0000000140864b6d: e8ee4e0402               call 0x1428a9a60
0000000140864b72: 488b8d90000000           mov rcx, qword ptr [rbp + 0x90]
0000000140864b79: e9ddfeffff               jmp 0x140864a5b
0000000140864b7e: 0f28b5b0000000           movaps xmm6, xmmword ptr [rbp + 0xb0]
0000000140864b85: 4881c448010000           add rsp, 0x148
0000000140864b8c: 5b                       pop rbx
0000000140864b8d: 5f                       pop rdi
0000000140864b8e: 5e                       pop rsi
0000000140864b8f: 415c                     pop r12
0000000140864b91: 415d                     pop r13
0000000140864b93: 415e                     pop r14
0000000140864b95: 415f                     pop r15
0000000140864b97: 5d                       pop rbp
0000000140864b98: c3                       ret 
0000000140864b99: 0f1f8000000000           nop dword ptr [rax]
0000000140864ba0: 4889542410               mov qword ptr [rsp + 0x10], rdx
0000000140864ba5: 55                       push rbp
0000000140864ba6: 4157                     push r15
0000000140864ba8: 4156                     push r14
0000000140864baa: 4155                     push r13
0000000140864bac: 4154                     push r12
0000000140864bae: 56                       push rsi
0000000140864baf: 57                       push rdi
0000000140864bb0: 53                       push rbx
0000000140864bb1: 4883ec38                 sub rsp, 0x38
0000000140864bb5: 488daa80000000           lea rbp, [rdx + 0x80]
0000000140864bbc: 0f29742420               movaps xmmword ptr [rsp + 0x20], xmm6
0000000140864bc1: 488d8d80000000           lea rcx, [rbp + 0x80]
0000000140864bc8: e863804e00               call 0x140d4cc30
0000000140864bcd: 0f28742420               movaps xmm6, xmmword ptr [rsp + 0x20]
0000000140864bd2: 4883c438                 add rsp, 0x38
0000000140864bd6: 5b                       pop rbx
0000000140864bd7: 5f                       pop rdi
0000000140864bd8: 5e                       pop rsi
0000000140864bd9: 415c                     pop r12
0000000140864bdb: 415d                     pop r13
0000000140864bdd: 415e                     pop r14
0000000140864bdf: 415f                     pop r15
0000000140864be1: 5d                       pop rbp
0000000140864be2: c3                       ret 
