0000000140b0da50  55                       push       rbp
0000000140b0da51  4157                     push       r15
0000000140b0da53  4156                     push       r14
0000000140b0da55  4155                     push       r13
0000000140b0da57  4154                     push       r12
0000000140b0da59  56                       push       rsi
0000000140b0da5a  57                       push       rdi
0000000140b0da5b  53                       push       rbx
0000000140b0da5c  4881eca8010000           sub        rsp, 0x1a8
0000000140b0da63  488dac2480000000         lea        rbp, [rsp + 0x80]
0000000140b0da6b  48c78520010000feffffff   mov        qword ptr [rbp + 0x120], 0xfffffffffffffffe
0000000140b0da76  4889d3                   mov        rbx, rdx
0000000140b0da79  4889ce                   mov        rsi, rcx
0000000140b0da7c  4531e4                   xor        r12d, r12d
0000000140b0da7f  b930000000               mov        ecx, 0x30
0000000140b0da84  4c89c0                   mov        rax, r8
0000000140b0da87  48f7e1                   mul        rcx
0000000140b0da8a  4989c7                   mov        r15, rax
0000000140b0da8d  0f804d040000             jo         0x140b0dee0
0000000140b0da93  48b8f0ffffffffffff7f     movabs     rax, 0x7ffffffffffffff0
0000000140b0da9d  4939c7                   cmp        r15, rax
0000000140b0daa0  0f873a040000             ja         0x140b0dee0
0000000140b0daa6  4c89c7                   mov        rdi, r8
0000000140b0daa9  4d85ff                   test       r15, r15
0000000140b0daac  743b                     je         0x140b0dae9
0000000140b0daae  0fb6055471b202           movzx      eax, byte ptr [rip + 0x2b27154]
0000000140b0dab5  41bc10000000             mov        r12d, 0x10
0000000140b0dabb  ba10000000               mov        edx, 0x10
0000000140b0dac0  4c89f9                   mov        rcx, r15
0000000140b0dac3  e84861b0ff               call       0x140613c10
0000000140b0dac8  4885c0                   test       rax, rax
0000000140b0dacb  0f840f040000             je         0x140b0dee0
0000000140b0dad1  4989c6                   mov        r14, rax
0000000140b0dad4  4989fd                   mov        r13, rdi
0000000140b0dad7  4885ff                   test       rdi, rdi
0000000140b0dada  741b                     je         0x140b0daf7
0000000140b0dadc  4883ff01                 cmp        rdi, 1
0000000140b0dae0  756d                     jne        0x140b0db4f
0000000140b0dae2  31c0                     xor        eax, eax
0000000140b0dae4  e9d4000000               jmp        0x140b0dbbd
0000000140b0dae9  41be10000000             mov        r14d, 0x10
0000000140b0daef  4531ed                   xor        r13d, r13d
0000000140b0daf2  4885ff                   test       rdi, rdi
0000000140b0daf5  75e5                     jne        0x140b0dadc
0000000140b0daf7  48c70600000000           mov        qword ptr [rsi], 0
0000000140b0dafe  48c7460804000000         mov        qword ptr [rsi + 8], 4
0000000140b0db06  0f57c0                   xorps      xmm0, xmm0
0000000140b0db09  0f114610                 movups     xmmword ptr [rsi + 0x10], xmm0
0000000140b0db0d  48c7462004000000         mov        qword ptr [rsi + 0x20], 4
0000000140b0db15  0f114628                 movups     xmmword ptr [rsi + 0x28], xmm0
0000000140b0db19  48c7463804000000         mov        qword ptr [rsi + 0x38], 4
0000000140b0db21  48c7464000000000         mov        qword ptr [rsi + 0x40], 0
0000000140b0db29  4d85ed                   test       r13, r13
0000000140b0db2c  0f8478030000             je         0x140b0deaa
0000000140b0db32  49c1e504                 shl        r13, 4
0000000140b0db36  4a8d146d00000000         lea        rdx, [r13*2]
0000000140b0db3e  4c01ea                   add        rdx, r13
0000000140b0db41  41b810000000             mov        r8d, 0x10
0000000140b0db47  4c89f1                   mov        rcx, r14
0000000140b0db4a  e956030000               jmp        0x140b0dea5
0000000140b0db4f  4889f9                   mov        rcx, rdi
0000000140b0db52  4883e1fe                 and        rcx, 0xfffffffffffffffe
0000000140b0db56  488d5340                 lea        rdx, [rbx + 0x40]
0000000140b0db5a  4d8d4650                 lea        r8, [r14 + 0x50]
0000000140b0db5e  31c0                     xor        eax, eax
0000000140b0db60  0f1042c0                 movups     xmm0, xmmword ptr [rdx - 0x40]
0000000140b0db64  0f104acc                 movups     xmm1, xmmword ptr [rdx - 0x34]
0000000140b0db68  0f1052d8                 movups     xmm2, xmmword ptr [rdx - 0x28]
0000000140b0db6c  0fc6c0a4                 shufps     xmm0, xmm0, 0xa4
0000000140b0db70  0fc6c9a4                 shufps     xmm1, xmm1, 0xa4
0000000140b0db74  0fc6d2a4                 shufps     xmm2, xmm2, 0xa4
0000000140b0db78  410f2940b0               movaps     xmmword ptr [r8 - 0x50], xmm0
0000000140b0db7d  410f2948c0               movaps     xmmword ptr [r8 - 0x40], xmm1
0000000140b0db82  410f2950d0               movaps     xmmword ptr [r8 - 0x30], xmm2
0000000140b0db87  0f1042e8                 movups     xmm0, xmmword ptr [rdx - 0x18]
0000000140b0db8b  0f104af4                 movups     xmm1, xmmword ptr [rdx - 0xc]
0000000140b0db8f  0f1012                   movups     xmm2, xmmword ptr [rdx]
0000000140b0db92  0fc6c0a4                 shufps     xmm0, xmm0, 0xa4
0000000140b0db96  0fc6c9a4                 shufps     xmm1, xmm1, 0xa4
0000000140b0db9a  0fc6d2a4                 shufps     xmm2, xmm2, 0xa4
0000000140b0db9e  410f2940e0               movaps     xmmword ptr [r8 - 0x20], xmm0
0000000140b0dba3  410f2948f0               movaps     xmmword ptr [r8 - 0x10], xmm1
0000000140b0dba8  410f2910                 movaps     xmmword ptr [r8], xmm2
0000000140b0dbac  4883c002                 add        rax, 2
0000000140b0dbb0  4883c250                 add        rdx, 0x50
0000000140b0dbb4  4983c060                 add        r8, 0x60
0000000140b0dbb8  4839c1                   cmp        rcx, rax
0000000140b0dbbb  75a3                     jne        0x140b0db60
0000000140b0dbbd  40f6c701                 test       dil, 1
0000000140b0dbc1  7437                     je         0x140b0dbfa
0000000140b0dbc3  488d0c80                 lea        rcx, [rax + rax*4]
0000000140b0dbc7  0f1004cb                 movups     xmm0, xmmword ptr [rbx + rcx*8]
0000000140b0dbcb  0f104ccb0c               movups     xmm1, xmmword ptr [rbx + rcx*8 + 0xc]
0000000140b0dbd0  0f1054cb18               movups     xmm2, xmmword ptr [rbx + rcx*8 + 0x18]
0000000140b0dbd5  0fc6c0a4                 shufps     xmm0, xmm0, 0xa4
0000000140b0dbd9  0fc6c9a4                 shufps     xmm1, xmm1, 0xa4
0000000140b0dbdd  0fc6d2a4                 shufps     xmm2, xmm2, 0xa4
0000000140b0dbe1  488d0440                 lea        rax, [rax + rax*2]
0000000140b0dbe5  48c1e004                 shl        rax, 4
0000000140b0dbe9  410f290406               movaps     xmmword ptr [r14 + rax], xmm0
0000000140b0dbee  410f294c0610             movaps     xmmword ptr [r14 + rax + 0x10], xmm1
0000000140b0dbf4  410f29540620             movaps     xmmword ptr [r14 + rax + 0x20], xmm2
0000000140b0dbfa  4c89adf8000000           mov        qword ptr [rbp + 0xf8], r13
0000000140b0dc01  4c89b500010000           mov        qword ptr [rbp + 0x100], r14
0000000140b0dc08  4889bd08010000           mov        qword ptr [rbp + 0x108], rdi
0000000140b0dc0f  4c89ad10010000           mov        qword ptr [rbp + 0x110], r13
0000000140b0dc16  e8c5592500               call       0x140d635e0
0000000140b0dc1b  4989c7                   mov        r15, rax
0000000140b0dc1e  4189d4                   mov        r12d, edx
0000000140b0dc21  48c7454000000000         mov        qword ptr [rbp + 0x40], 0
0000000140b0dc29  c7454800000000           mov        dword ptr [rbp + 0x48], 0
0000000140b0dc30  66c745690001             mov        word ptr [rbp + 0x69], 0x100
0000000140b0dc36  c6456804                 mov        byte ptr [rbp + 0x68], 4
0000000140b0dc3a  48c7455001000000         mov        qword ptr [rbp + 0x50], 1
0000000140b0dc42  f20f1005769cfd01         movsd      xmm0, qword ptr [rip + 0x1fd9c76]
0000000140b0dc4a  f20f114558               movsd      qword ptr [rbp + 0x58], xmm0
0000000140b0dc4f  48b80300000000004040     movabs     rax, 0x4040000000000003
0000000140b0dc59  48894560                 mov        qword ptr [rbp + 0x60], rax
0000000140b0dc5d  4c89ad10010000           mov        qword ptr [rbp + 0x110], r13
0000000140b0dc64  488d4540                 lea        rax, [rbp + 0x40]
0000000140b0dc68  4889442420               mov        qword ptr [rsp + 0x20], rax
0000000140b0dc6d  488d4db0                 lea        rcx, [rbp - 0x50]
0000000140b0dc71  4c8d4d50                 lea        r9, [rbp + 0x50]
0000000140b0dc75  4c89f2                   mov        rdx, r14
0000000140b0dc78  4989f8                   mov        r8, rdi
0000000140b0dc7b  e850b7cb00               call       0x1417c93d0
0000000140b0dc80  e85b592500               call       0x140d635e0
0000000140b0dc85  48894530                 mov        qword ptr [rbp + 0x30], rax
0000000140b0dc89  895538                   mov        dword ptr [rbp + 0x38], edx
0000000140b0dc8c  488b0515e7b102           mov        rax, qword ptr [rip + 0x2b1e715]
0000000140b0dc93  488b00                   mov        rax, qword ptr [rax]
0000000140b0dc96  4883f805                 cmp        rax, 5
0000000140b0dc9a  0f82e2000000             jb         0x140b0dd82
0000000140b0dca0  488d4d30                 lea        rcx, [rbp + 0x30]
0000000140b0dca4  4c89fa                   mov        rdx, r15
0000000140b0dca7  4589e0                   mov        r8d, r12d
0000000140b0dcaa  e8d1592500               call       0x140d63680
0000000140b0dcaf  488985e0000000           mov        qword ptr [rbp + 0xe0], rax
0000000140b0dcb6  8995e8000000             mov        dword ptr [rbp + 0xe8], edx
0000000140b0dcbc  488d85e0000000           lea        rax, [rbp + 0xe0]
0000000140b0dcc3  48894520                 mov        qword ptr [rbp + 0x20], rax
0000000140b0dcc7  488d0582c82700           lea        rax, [rip + 0x27c882]
0000000140b0dcce  48894528                 mov        qword ptr [rbp + 0x28], rax
0000000140b0dcd2  488d0d779ffd01           lea        rcx, [rip + 0x1fd9f77]
0000000140b0dcd9  e8a200b7ff               call       0x14067dd80
0000000140b0dcde  0f1000                   movups     xmm0, xmmword ptr [rax]
0000000140b0dce1  8b4010                   mov        eax, dword ptr [rax + 0x10]
0000000140b0dce4  48c7858000000005000000   mov        qword ptr [rbp + 0x80], 5
0000000140b0dcef  488d0d729ffd01           lea        rcx, [rip + 0x1fd9f72]
0000000140b0dcf6  48898d88000000           mov        qword ptr [rbp + 0x88], rcx
0000000140b0dcfd  48c7859000000020000000   mov        qword ptr [rbp + 0x90], 0x20
0000000140b0dd08  488d15019ffd01           lea        rdx, [rip + 0x1fd9f01]
0000000140b0dd0f  488995a0000000           mov        qword ptr [rbp + 0xa0], rdx
0000000140b0dd16  48c785a800000001000000   mov        qword ptr [rbp + 0xa8], 1
0000000140b0dd21  488d5520                 lea        rdx, [rbp + 0x20]
0000000140b0dd25  488995b0000000           mov        qword ptr [rbp + 0xb0], rdx
0000000140b0dd2c  48c785b800000001000000   mov        qword ptr [rbp + 0xb8], 1
0000000140b0dd37  48c785c000000000000000   mov        qword ptr [rbp + 0xc0], 0
0000000140b0dd42  48c7455000000000         mov        qword ptr [rbp + 0x50], 0
0000000140b0dd4a  48894d58                 mov        qword ptr [rbp + 0x58], rcx
0000000140b0dd4e  48c7456020000000         mov        qword ptr [rbp + 0x60], 0x20
0000000140b0dd56  48c7456800000000         mov        qword ptr [rbp + 0x68], 0
0000000140b0dd5e  0f114570                 movups     xmmword ptr [rbp + 0x70], xmm0
0000000140b0dd62  c7859800000001000000     mov        dword ptr [rbp + 0x98], 1
0000000140b0dd6c  89859c000000             mov        dword ptr [rbp + 0x9c], eax
0000000140b0dd72  488d8d1f010000           lea        rcx, [rbp + 0x11f]
0000000140b0dd79  488d5550                 lea        rdx, [rbp + 0x50]
0000000140b0dd7d  e8aec52300               call       0x140d4a330
0000000140b0dd82  4c8b75d0                 mov        r14, qword ptr [rbp - 0x30]
0000000140b0dd86  4c8b7dd8                 mov        r15, qword ptr [rbp - 0x28]
0000000140b0dd8a  4b8d04be                 lea        rax, [r14 + r15*4]
0000000140b0dd8e  4c897550                 mov        qword ptr [rbp + 0x50], r14
0000000140b0dd92  48894558                 mov        qword ptr [rbp + 0x58], rax
0000000140b0dd96  488d85f8000000           lea        rax, [rbp + 0xf8]
0000000140b0dd9d  48894560                 mov        qword ptr [rbp + 0x60], rax
0000000140b0dda1  4c8d05309cfd01           lea        r8, [rip + 0x1fd9c30]
0000000140b0dda8  488d8de0000000           lea        rcx, [rbp + 0xe0]
0000000140b0ddaf  488d5550                 lea        rdx, [rbp + 0x50]
0000000140b0ddb3  e828bd7f00               call       0x141309ae0
0000000140b0ddb8  4d85ff                   test       r15, r15
0000000140b0ddbb  7455                     je         0x140b0de12
0000000140b0ddbd  4e8d24bd00000000         lea        r12, [r15*4]
0000000140b0ddc5  0fb6053d6eb202           movzx      eax, byte ptr [rip + 0x2b26e3d]
0000000140b0ddcc  ba04000000               mov        edx, 4
0000000140b0ddd1  4c89e1                   mov        rcx, r12
0000000140b0ddd4  e8375eb0ff               call       0x140613c10
0000000140b0ddd9  4885c0                   test       rax, rax
0000000140b0dddc  0f8410010000             je         0x140b0def2
0000000140b0dde2  31d2                     xor        edx, edx
0000000140b0dde4  6666662e0f1f840000000000 nop        word ptr cs:[rax + rax]
0000000140b0ddf0  418b0c96                 mov        ecx, dword ptr [r14 + rdx*4]
0000000140b0ddf4  4839f9                   cmp        rcx, rdi
0000000140b0ddf7  0f83c4000000             jae        0x140b0dec1
0000000140b0ddfd  488d0c89                 lea        rcx, [rcx + rcx*4]
0000000140b0de01  8b4ccb24                 mov        ecx, dword ptr [rbx + rcx*8 + 0x24]
0000000140b0de05  890c90                   mov        dword ptr [rax + rdx*4], ecx
0000000140b0de08  48ffc2                   inc        rdx
0000000140b0de0b  4939d7                   cmp        r15, rdx
0000000140b0de0e  75e0                     jne        0x140b0ddf0
0000000140b0de10  eb05                     jmp        0x140b0de17
0000000140b0de12  b804000000               mov        eax, 4
0000000140b0de17  488b4dc0                 mov        rcx, qword ptr [rbp - 0x40]
0000000140b0de1b  48894e10                 mov        qword ptr [rsi + 0x10], rcx
0000000140b0de1f  0f2845b0                 movaps     xmm0, xmmword ptr [rbp - 0x50]
0000000140b0de23  0f1106                   movups     xmmword ptr [rsi], xmm0
0000000140b0de26  0f1085e0000000           movups     xmm0, xmmword ptr [rbp + 0xe0]
0000000140b0de2d  0f114618                 movups     xmmword ptr [rsi + 0x18], xmm0
0000000140b0de31  488b8df0000000           mov        rcx, qword ptr [rbp + 0xf0]
0000000140b0de38  48894e28                 mov        qword ptr [rsi + 0x28], rcx
0000000140b0de3c  4c897e30                 mov        qword ptr [rsi + 0x30], r15
0000000140b0de40  48894638                 mov        qword ptr [rsi + 0x38], rax
0000000140b0de44  4c897e40                 mov        qword ptr [rsi + 0x40], r15
0000000140b0de48  488b55c8                 mov        rdx, qword ptr [rbp - 0x38]
0000000140b0de4c  4885d2                   test       rdx, rdx
0000000140b0de4f  7412                     je         0x140b0de63
0000000140b0de51  48c1e202                 shl        rdx, 2
0000000140b0de55  41b804000000             mov        r8d, 4
0000000140b0de5b  4c89f1                   mov        rcx, r14
0000000140b0de5e  e8bd5db0ff               call       0x140613c20
0000000140b0de63  488b5500                 mov        rdx, qword ptr [rbp]
0000000140b0de67  4889d0                   mov        rax, rdx
0000000140b0de6a  48f7d8                   neg        rax
0000000140b0de6d  7015                     jo         0x140b0de84
0000000140b0de6f  7313                     jae        0x140b0de84
0000000140b0de71  488b4d08                 mov        rcx, qword ptr [rbp + 8]
0000000140b0de75  48c1e205                 shl        rdx, 5
0000000140b0de79  41b810000000             mov        r8d, 0x10
0000000140b0de7f  e89c5db0ff               call       0x140613c20
0000000140b0de84  488b85f8000000           mov        rax, qword ptr [rbp + 0xf8]
0000000140b0de8b  4885c0                   test       rax, rax
0000000140b0de8e  741a                     je         0x140b0deaa
0000000140b0de90  488b8d00010000           mov        rcx, qword ptr [rbp + 0x100]
0000000140b0de97  48c1e004                 shl        rax, 4
0000000140b0de9b  488d1440                 lea        rdx, [rax + rax*2]
0000000140b0de9f  41b810000000             mov        r8d, 0x10
0000000140b0dea5  e8765db0ff               call       0x140613c20
0000000140b0deaa  4889f0                   mov        rax, rsi
0000000140b0dead  4881c4a8010000           add        rsp, 0x1a8
0000000140b0deb4  5b                       pop        rbx
0000000140b0deb5  5f                       pop        rdi
0000000140b0deb6  5e                       pop        rsi
0000000140b0deb7  415c                     pop        r12
0000000140b0deb9  415d                     pop        r13
0000000140b0debb  415e                     pop        r14
0000000140b0debd  415f                     pop        r15
0000000140b0debf  5d                       pop        rbp
0000000140b0dec0  c3                       ret        
0000000140b0dec1  4c89a5d8000000           mov        qword ptr [rbp + 0xd8], r12
0000000140b0dec8  488985d0000000           mov        qword ptr [rbp + 0xd0], rax
0000000140b0decf  4c8d0562771502           lea        r8, [rip + 0x2157762]
0000000140b0ded6  4889fa                   mov        rdx, rdi
0000000140b0ded9  e83ab6dc01               call       0x1428d9518
0000000140b0dede  eb26                     jmp        0x140b0df06
0000000140b0dee0  4c8d05f19afd01           lea        r8, [rip + 0x1fd9af1]
0000000140b0dee7  4c89e1                   mov        rcx, r12
0000000140b0deea  4c89fa                   mov        rdx, r15
0000000140b0deed  e8f1b0dc01               call       0x1428d8fe3
0000000140b0def2  4c8d05df9afd01           lea        r8, [rip + 0x1fd9adf]
0000000140b0def9  b904000000               mov        ecx, 4
0000000140b0defe  4c89e2                   mov        rdx, r12
0000000140b0df01  e8ddb0dc01               call       0x1428d8fe3
0000000140b0df06  0f0b                     ud2        
0000000140b0df08  0f1f840000000000         nop        dword ptr [rax + rax]
0000000140b0df10  4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140b0df15  55                       push       rbp
0000000140b0df16  4157                     push       r15
0000000140b0df18  4156                     push       r14
0000000140b0df1a  4155                     push       r13
0000000140b0df1c  4154                     push       r12
0000000140b0df1e  56                       push       rsi
0000000140b0df1f  57                       push       rdi
0000000140b0df20  53                       push       rbx
0000000140b0df21  4883ec28                 sub        rsp, 0x28
0000000140b0df25  488daa80000000           lea        rbp, [rdx + 0x80]
0000000140b0df2c  488b8510010000           mov        rax, qword ptr [rbp + 0x110]
0000000140b0df33  4885c0                   test       rax, rax
0000000140b0df36  741a                     je         0x140b0df52
0000000140b0df38  488b8d00010000           mov        rcx, qword ptr [rbp + 0x100]
0000000140b0df3f  48c1e004                 shl        rax, 4
0000000140b0df43  488d1440                 lea        rdx, [rax + rax*2]
0000000140b0df47  41b810000000             mov        r8d, 0x10
0000000140b0df4d  e8ce5cb0ff               call       0x140613c20
0000000140b0df52  90                       nop        
0000000140b0df53  4883c428                 add        rsp, 0x28
0000000140b0df57  5b                       pop        rbx
0000000140b0df58  5f                       pop        rdi
0000000140b0df59  5e                       pop        rsi
0000000140b0df5a  415c                     pop        r12
0000000140b0df5c  415d                     pop        r13
0000000140b0df5e  415e                     pop        r14
0000000140b0df60  415f                     pop        r15
0000000140b0df62  5d                       pop        rbp
0000000140b0df63  c3                       ret        
0000000140b0df64  6666662e0f1f840000000000 nop        word ptr cs:[rax + rax]
0000000140b0df70  4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140b0df75  55                       push       rbp
0000000140b0df76  4157                     push       r15
0000000140b0df78  4156                     push       r14
0000000140b0df7a  4155                     push       r13
0000000140b0df7c  4154                     push       r12
0000000140b0df7e  56                       push       rsi
0000000140b0df7f  57                       push       rdi
0000000140b0df80  53                       push       rbx
0000000140b0df81  4883ec28                 sub        rsp, 0x28
0000000140b0df85  488daa80000000           lea        rbp, [rdx + 0x80]
0000000140b0df8c  488d4db0                 lea        rcx, [rbp - 0x50]
0000000140b0df90  e85b8fd7ff               call       0x140886ef0
0000000140b0df95  488b85f8000000           mov        rax, qword ptr [rbp + 0xf8]
0000000140b0df9c  48898510010000           mov        qword ptr [rbp + 0x110], rax
0000000140b0dfa3  4883c428                 add        rsp, 0x28
0000000140b0dfa7  5b                       pop        rbx
0000000140b0dfa8  5f                       pop        rdi
0000000140b0dfa9  5e                       pop        rsi
0000000140b0dfaa  415c                     pop        r12
0000000140b0dfac  415d                     pop        r13
0000000140b0dfae  415e                     pop        r14
0000000140b0dfb0  415f                     pop        r15
0000000140b0dfb2  5d                       pop        rbp
0000000140b0dfb3  c3                       ret        
0000000140b0dfb4  6666662e0f1f840000000000 nop        word ptr cs:[rax + rax]
0000000140b0dfc0  4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140b0dfc5  55                       push       rbp
0000000140b0dfc6  4157                     push       r15
0000000140b0dfc8  4156                     push       r14
0000000140b0dfca  4155                     push       r13
0000000140b0dfcc  4154                     push       r12
0000000140b0dfce  56                       push       rsi
0000000140b0dfcf  57                       push       rdi
0000000140b0dfd0  53                       push       rbx
0000000140b0dfd1  4883ec28                 sub        rsp, 0x28
0000000140b0dfd5  488daa80000000           lea        rbp, [rdx + 0x80]
0000000140b0dfdc  488b85e0000000           mov        rax, qword ptr [rbp + 0xe0]
0000000140b0dfe3  4885c0                   test       rax, rax
0000000140b0dfe6  741a                     je         0x140b0e002
0000000140b0dfe8  488b8de8000000           mov        rcx, qword ptr [rbp + 0xe8]
0000000140b0dfef  48c1e003                 shl        rax, 3
0000000140b0dff3  488d1440                 lea        rdx, [rax + rax*2]
0000000140b0dff7  41b804000000             mov        r8d, 4
0000000140b0dffd  e81e5cb0ff               call       0x140613c20
0000000140b0e002  90                       nop        
0000000140b0e003  4883c428                 add        rsp, 0x28
0000000140b0e007  5b                       pop        rbx
0000000140b0e008  5f                       pop        rdi
0000000140b0e009  5e                       pop        rsi
0000000140b0e00a  415c                     pop        r12
0000000140b0e00c  415d                     pop        r13
0000000140b0e00e  415e                     pop        r14
0000000140b0e010  415f                     pop        r15
0000000140b0e012  5d                       pop        rbp
0000000140b0e013  c3                       ret        
0000000140b0e014  6666662e0f1f840000000000 nop        word ptr cs:[rax + rax]
0000000140b0e020  4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140b0e025  55                       push       rbp
0000000140b0e026  4157                     push       r15
0000000140b0e028  4156                     push       r14
0000000140b0e02a  4155                     push       r13
0000000140b0e02c  4154                     push       r12
0000000140b0e02e  56                       push       rsi
0000000140b0e02f  57                       push       rdi
0000000140b0e030  53                       push       rbx
0000000140b0e031  4883ec28                 sub        rsp, 0x28
0000000140b0e035  488daa80000000           lea        rbp, [rdx + 0x80]
0000000140b0e03c  41b804000000             mov        r8d, 4
0000000140b0e042  488b8dd0000000           mov        rcx, qword ptr [rbp + 0xd0]
0000000140b0e049  488b95d8000000           mov        rdx, qword ptr [rbp + 0xd8]
0000000140b0e050  e8cb5bb0ff               call       0x140613c20
0000000140b0e055  90                       nop        
0000000140b0e056  4883c428                 add        rsp, 0x28
0000000140b0e05a  5b                       pop        rbx
0000000140b0e05b  5f                       pop        rdi
0000000140b0e05c  5e                       pop        rsi
0000000140b0e05d  415c                     pop        r12
0000000140b0e05f  415d                     pop        r13
0000000140b0e061  415e                     pop        r14
0000000140b0e063  415f                     pop        r15
0000000140b0e065  5d                       pop        rbp
0000000140b0e066  c3                       ret        
