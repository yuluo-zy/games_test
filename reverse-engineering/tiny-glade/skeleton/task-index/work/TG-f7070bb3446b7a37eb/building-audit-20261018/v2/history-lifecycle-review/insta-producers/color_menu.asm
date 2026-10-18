142291230: 55                           push rbp
142291231: 4157                         push r15
142291233: 4156                         push r14
142291235: 4155                         push r13
142291237: 4154                         push r12
142291239: 56                           push rsi
14229123a: 57                           push rdi
14229123b: 53                           push rbx
14229123c: 4881ec48080000               sub rsp, 0x848
142291243: 488dac2480000000             lea rbp, [rsp + 0x80]
14229124b: 440f29bdb0070000             movaps xmmword ptr [rbp + 0x7b0], xmm15
142291253: 440f29b5a0070000             movaps xmmword ptr [rbp + 0x7a0], xmm14
14229125b: 440f29ad90070000             movaps xmmword ptr [rbp + 0x790], xmm13
142291263: 440f29a580070000             movaps xmmword ptr [rbp + 0x780], xmm12
14229126b: 440f299d70070000             movaps xmmword ptr [rbp + 0x770], xmm11
142291273: 440f299560070000             movaps xmmword ptr [rbp + 0x760], xmm10
14229127b: 440f298d50070000             movaps xmmword ptr [rbp + 0x750], xmm9
142291283: 440f298540070000             movaps xmmword ptr [rbp + 0x740], xmm8
14229128b: 0f29bd30070000               movaps xmmword ptr [rbp + 0x730], xmm7
142291292: 0f29b520070000               movaps xmmword ptr [rbp + 0x720], xmm6
142291299: 48c78518070000feffffff       mov qword ptr [rbp + 0x718], 0xfffffffffffffffe
1422912a4: 4c898d00020000               mov qword ptr [rbp + 0x200], r9
1422912ab: 488b8540080000               mov rax, qword ptr [rbp + 0x840]
1422912b2: 4c8b9538080000               mov r10, qword ptr [rbp + 0x838]
1422912b9: 410f1002                     movups xmm0, xmmword ptr [r10]
1422912bd: 410f104a10                   movups xmm1, xmmword ptr [r10 + 0x10]
1422912c2: 0f294d20                     movaps xmmword ptr [rbp + 0x20], xmm1
1422912c6: 0f294510                     movaps xmmword ptr [rbp + 0x10], xmm0
1422912ca: 4d8b5a20                     mov r11, qword ptr [r10 + 0x20]
1422912ce: 4c899d20030000               mov qword ptr [rbp + 0x320], r11
1422912d5: 4d8b5a40                     mov r11, qword ptr [r10 + 0x40]
1422912d9: 4c899da0000000               mov qword ptr [rbp + 0xa0], r11
1422912e0: 4d8b5a60                     mov r11, qword ptr [r10 + 0x60]
1422912e4: 4c899d80010000               mov qword ptr [rbp + 0x180], r11
1422912eb: 4d8b9a80000000               mov r11, qword ptr [r10 + 0x80]
1422912f2: 4c899db0010000               mov qword ptr [rbp + 0x1b0], r11
1422912f9: 4d8b9aa0000000               mov r11, qword ptr [r10 + 0xa0]
142291300: 4c899d08020000               mov qword ptr [rbp + 0x208], r11
142291307: 4d8b9ac0000000               mov r11, qword ptr [r10 + 0xc0]
14229130e: 4c899dc0000000               mov qword ptr [rbp + 0xc0], r11
142291315: 4d8b9ac8000000               mov r11, qword ptr [r10 + 0xc8]
14229131c: 4c899dc8000000               mov qword ptr [rbp + 0xc8], r11
142291323: 4d8b92d8000000               mov r10, qword ptr [r10 + 0xd8]
14229132a: 4c899560060000               mov qword ptr [rbp + 0x660], r10
142291331: 4c8b10                       mov r10, qword ptr [rax]
142291334: 4c899570060000               mov qword ptr [rbp + 0x670], r10
14229133b: 4c8b5010                     mov r10, qword ptr [rax + 0x10]
14229133f: 4c899568060000               mov qword ptr [rbp + 0x668], r10
142291346: 448b501c                     mov r10d, dword ptr [rax + 0x1c]
14229134a: 448995d8060000               mov dword ptr [rbp + 0x6d8], r10d
142291351: 488b4020                     mov rax, qword ptr [rax + 0x20]
142291355: 48898550060000               mov qword ptr [rbp + 0x650], rax
14229135c: 4c8b12                       mov r10, qword ptr [rdx]
14229135f: 488b4208                     mov rax, qword ptr [rdx + 8]
142291363: 48898500050000               mov qword ptr [rbp + 0x500], rax
14229136a: 8b4214                       mov eax, dword ptr [rdx + 0x14]
14229136d: 8985ec050000                 mov dword ptr [rbp + 0x5ec], eax
142291373: 498b92f8000000               mov rdx, qword ptr [r10 + 0xf8]
14229137a: 498b8200010000               mov rax, qword ptr [r10 + 0x100]
142291381: 48899510030000               mov qword ptr [rbp + 0x310], rdx
142291388: 488d0482                     lea rax, [rdx + rax*4]
14229138c: 488985d0010000               mov qword ptr [rbp + 0x1d0], rax
142291393: 488b01                       mov rax, qword ptr [rcx]
142291396: 48898508060000               mov qword ptr [rbp + 0x608], rax
14229139d: 488b4108                     mov rax, qword ptr [rcx + 8]
1422913a1: 48898588000000               mov qword ptr [rbp + 0x88], rax
1422913a8: 8b4114                       mov eax, dword ptr [rcx + 0x14]
1422913ab: 8985f8060000                 mov dword ptr [rbp + 0x6f8], eax
1422913b1: 418b401c                     mov eax, dword ptr [r8 + 0x1c]
1422913b5: 8985f0060000                 mov dword ptr [rbp + 0x6f0], eax
1422913bb: 498b00                       mov rax, qword ptr [r8]
1422913be: 48898588060000               mov qword ptr [rbp + 0x688], rax
1422913c5: 498b4010                     mov rax, qword ptr [r8 + 0x10]
1422913c9: 48898590060000               mov qword ptr [rbp + 0x690], rax
1422913d0: 488b8550080000               mov rax, qword ptr [rbp + 0x850]
1422913d7: 8b481c                       mov ecx, dword ptr [rax + 0x1c]
1422913da: 898dcc020000                 mov dword ptr [rbp + 0x2cc], ecx
1422913e0: 4c8b30                       mov r14, qword ptr [rax]
1422913e3: 488b4010                     mov rax, qword ptr [rax + 0x10]
1422913e7: 48898520010000               mov qword ptr [rbp + 0x120], rax
1422913ee: 488b8560080000               mov rax, qword ptr [rbp + 0x860]
1422913f5: 8b501c                       mov edx, dword ptr [rax + 0x1c]
1422913f8: 8995b0040000                 mov dword ptr [rbp + 0x4b0], edx
1422913fe: 488b08                       mov rcx, qword ptr [rax]
142291401: 48898d10050000               mov qword ptr [rbp + 0x510], rcx
142291408: 488b4010                     mov rax, qword ptr [rax + 0x10]
14229140c: 48898520020000               mov qword ptr [rbp + 0x220], rax
142291413: 488b8578080000               mov rax, qword ptr [rbp + 0x878]
14229141a: 448b401c                     mov r8d, dword ptr [rax + 0x1c]
14229141e: 448985b4040000               mov dword ptr [rbp + 0x4b4], r8d
142291425: 488b08                       mov rcx, qword ptr [rax]
142291428: 48898d18050000               mov qword ptr [rbp + 0x518], rcx
14229142f: 488b4010                     mov rax, qword ptr [rax + 0x10]
142291433: 48898528020000               mov qword ptr [rbp + 0x228], rax
14229143a: 488b8580080000               mov rax, qword ptr [rbp + 0x880]
142291441: 448b581c                     mov r11d, dword ptr [rax + 0x1c]
142291445: 44899dcc040000               mov dword ptr [rbp + 0x4cc], r11d
14229144c: 488b08                       mov rcx, qword ptr [rax]
14229144f: 48898d20050000               mov qword ptr [rbp + 0x520], rcx
142291456: 488b4010                     mov rax, qword ptr [rax + 0x10]
14229145a: 48898588020000               mov qword ptr [rbp + 0x288], rax
142291461: 488b8568080000               mov rax, qword ptr [rbp + 0x868]
142291468: 8b701c                       mov esi, dword ptr [rax + 0x1c]
14229146b: 89b5bc040000                 mov dword ptr [rbp + 0x4bc], esi
142291471: 488b08                       mov rcx, qword ptr [rax]
142291474: 48898dd0030000               mov qword ptr [rbp + 0x3d0], rcx
14229147b: 488b4010                     mov rax, qword ptr [rax + 0x10]
14229147f: 48898540020000               mov qword ptr [rbp + 0x240], rax
142291486: 488b8570080000               mov rax, qword ptr [rbp + 0x870]
14229148d: 8b781c                       mov edi, dword ptr [rax + 0x1c]
142291490: 89bdd4020000                 mov dword ptr [rbp + 0x2d4], edi
142291496: 488b08                       mov rcx, qword ptr [rax]
142291499: 48898d80030000               mov qword ptr [rbp + 0x380], rcx
1422914a0: 488b4010                     mov rax, qword ptr [rax + 0x10]
1422914a4: 48898550010000               mov qword ptr [rbp + 0x150], rax
1422914ab: 488b8548080000               mov rax, qword ptr [rbp + 0x848]
1422914b2: 488b4820                     mov rcx, qword ptr [rax + 0x20]
1422914b6: 48898d88030000               mov qword ptr [rbp + 0x388], rcx
1422914bd: 4c8b7830                     mov r15, qword ptr [rax + 0x30]
1422914c1: 4c89bd58010000               mov qword ptr [rbp + 0x158], r15
1422914c8: 448b783c                     mov r15d, dword ptr [rax + 0x3c]
1422914cc: 4489bdd8020000               mov dword ptr [rbp + 0x2d8], r15d
1422914d3: 488b4840                     mov rcx, qword ptr [rax + 0x40]
1422914d7: 48898d58030000               mov qword ptr [rbp + 0x358], rcx
1422914de: 4c8b6050                     mov r12, qword ptr [rax + 0x50]
1422914e2: 4c89a5e0000000               mov qword ptr [rbp + 0xe0], r12
1422914e9: 448b605c                     mov r12d, dword ptr [rax + 0x5c]
1422914ed: 4489a5c0020000               mov dword ptr [rbp + 0x2c0], r12d
1422914f4: 488b4860                     mov rcx, qword ptr [rax + 0x60]
1422914f8: 48898d90030000               mov qword ptr [rbp + 0x390], rcx
1422914ff: 4c8b6870                     mov r13, qword ptr [rax + 0x70]
142291503: 4c89ad60010000               mov qword ptr [rbp + 0x160], r13
14229150a: 448b687c                     mov r13d, dword ptr [rax + 0x7c]
14229150e: 4489addc020000               mov dword ptr [rbp + 0x2dc], r13d
142291515: 488b8880000000               mov rcx, qword ptr [rax + 0x80]
14229151c: 48898d78030000               mov qword ptr [rbp + 0x378], rcx
142291523: 488b8890000000               mov rcx, qword ptr [rax + 0x90]
14229152a: 48898d30010000               mov qword ptr [rbp + 0x130], rcx
142291531: 8b889c000000                 mov ecx, dword ptr [rax + 0x9c]
142291537: 898dd0020000                 mov dword ptr [rbp + 0x2d0], ecx
14229153d: 488b88a0000000               mov rcx, qword ptr [rax + 0xa0]
142291544: 4c8b88b0000000               mov r9, qword ptr [rax + 0xb0]
14229154b: 4c898d38020000               mov qword ptr [rbp + 0x238], r9
142291552: 448b88bc000000               mov r9d, dword ptr [rax + 0xbc]
142291559: 44898db8040000               mov dword ptr [rbp + 0x4b8], r9d
142291560: 4c8b88c0000000               mov r9, qword ptr [rax + 0xc0]
142291567: 488b90d0000000               mov rdx, qword ptr [rax + 0xd0]
14229156e: 48899518020000               mov qword ptr [rbp + 0x218], rdx
142291575: 8b90dc000000                 mov edx, dword ptr [rax + 0xdc]
14229157b: 8995ac040000                 mov dword ptr [rbp + 0x4ac], edx
142291581: 488b90e0000000               mov rdx, qword ptr [rax + 0xe0]
142291588: 4c8b80f0000000               mov r8, qword ptr [rax + 0xf0]
14229158f: 4c898558020000               mov qword ptr [rbp + 0x258], r8
142291596: 448b80fc000000               mov r8d, dword ptr [rax + 0xfc]
14229159d: 448985c0040000               mov dword ptr [rbp + 0x4c0], r8d
1422915a4: 4c8b8000010000               mov r8, qword ptr [rax + 0x100]
1422915ab: 4c898598030000               mov qword ptr [rbp + 0x398], r8
1422915b2: 4c8b9810010000               mov r11, qword ptr [rax + 0x110]
1422915b9: 4c899d68010000               mov qword ptr [rbp + 0x168], r11
1422915c0: 448b981c010000               mov r11d, dword ptr [rax + 0x11c]
1422915c7: 44899de0020000               mov dword ptr [rbp + 0x2e0], r11d
1422915ce: 4c8b9820010000               mov r11, qword ptr [rax + 0x120]
1422915d5: 488bb030010000               mov rsi, qword ptr [rax + 0x130]
1422915dc: 4889b560020000               mov qword ptr [rbp + 0x260], rsi
1422915e3: 8bb03c010000                 mov esi, dword ptr [rax + 0x13c]
1422915e9: 89b5c4040000                 mov dword ptr [rbp + 0x4c4], esi
1422915ef: 488bb040010000               mov rsi, qword ptr [rax + 0x140]
1422915f6: 488b9850010000               mov rbx, qword ptr [rax + 0x150]
1422915fd: 48899df8000000               mov qword ptr [rbp + 0xf8], rbx
142291604: 8b985c010000                 mov ebx, dword ptr [rax + 0x15c]
14229160a: 899dc4020000                 mov dword ptr [rbp + 0x2c4], ebx
142291610: 488b9860010000               mov rbx, qword ptr [rax + 0x160]
142291617: 488bb870010000               mov rdi, qword ptr [rax + 0x170]
14229161e: 4889bd68020000               mov qword ptr [rbp + 0x268], rdi
142291625: 8bb87c010000                 mov edi, dword ptr [rax + 0x17c]
14229162b: 89bdc8040000                 mov dword ptr [rbp + 0x4c8], edi
142291631: 488bb880010000               mov rdi, qword ptr [rax + 0x180]
142291638: 4c8bb890010000               mov r15, qword ptr [rax + 0x190]
14229163f: 4c89bda8000000               mov qword ptr [rbp + 0xa8], r15
142291646: 448bb89c010000               mov r15d, dword ptr [rax + 0x19c]
14229164d: 4489bdb4020000               mov dword ptr [rbp + 0x2b4], r15d
142291654: 4c8bb8a0010000               mov r15, qword ptr [rax + 0x1a0]
14229165b: 4c8ba0b0010000               mov r12, qword ptr [rax + 0x1b0]
142291662: 4c89a590000000               mov qword ptr [rbp + 0x90], r12
142291669: 448ba0bc010000               mov r12d, dword ptr [rax + 0x1bc]
142291670: 4489a5b0020000               mov dword ptr [rbp + 0x2b0], r12d
142291677: 4c8ba0c0010000               mov r12, qword ptr [rax + 0x1c0]
14229167e: 4c8ba8d0010000               mov r13, qword ptr [rax + 0x1d0]
142291685: 4c89ad00010000               mov qword ptr [rbp + 0x100], r13
14229168c: 448ba8dc010000               mov r13d, dword ptr [rax + 0x1dc]
142291693: 4489adc8020000               mov dword ptr [rbp + 0x2c8], r13d
14229169a: 4c8ba8e0010000               mov r13, qword ptr [rax + 0x1e0]
1422916a1: 4c8b80f0010000               mov r8, qword ptr [rax + 0x1f0]
1422916a8: 4c8985b8000000               mov qword ptr [rbp + 0xb8], r8
1422916af: 448b80fc010000               mov r8d, dword ptr [rax + 0x1fc]
1422916b6: 448985bc020000               mov dword ptr [rbp + 0x2bc], r8d
1422916bd: 448b401c                     mov r8d, dword ptr [rax + 0x1c]
1422916c1: 448985e4020000               mov dword ptr [rbp + 0x2e4], r8d
1422916c8: 4c8b00                       mov r8, qword ptr [rax]
1422916cb: 488b4010                     mov rax, qword ptr [rax + 0x10]
1422916cf: 48898570010000               mov qword ptr [rbp + 0x170], rax
1422916d6: f30f103d962f6a00             movss xmm7, dword ptr [rip + 0x6a2f96]
1422916de: 450f57c0                     xorps xmm8, xmm8
1422916e2: f3440f100d85416900           movss xmm9, dword ptr [rip + 0x694185]
1422916eb: f3440f101594426900           movss xmm10, dword ptr [rip + 0x694294]
1422916f4: 440f281de49bda00             movaps xmm11, xmmword ptr [rip + 0xda9be4]
1422916fc: f3440f10252bb97300           movss xmm12, dword ptr [rip + 0x73b92b]
142291705: 440f282dd3416900             movaps xmm13, xmmword ptr [rip + 0x6941d3]
14229170d: 4c899528050000               mov qword ptr [rbp + 0x528], r10
142291714: 410fb68208010000             movzx eax, byte ptr [r10 + 0x108]
14229171c: 88857f060000                 mov byte ptr [rbp + 0x67f], al
142291722: 4c8b9558080000               mov r10, qword ptr [rbp + 0x858]
142291729: 488b8500020000               mov rax, qword ptr [rbp + 0x200]
142291730: 488b00                       mov rax, qword ptr [rax]
142291733: 48898538030000               mov qword ptr [rbp + 0x338], rax
14229173a: 4c89bdb8010000               mov qword ptr [rbp + 0x1b8], r15
142291741: 498d4720                     lea rax, [r15 + 0x20]
142291745: 48898598000000               mov qword ptr [rbp + 0x98], rax
14229174c: 4889b548020000               mov qword ptr [rbp + 0x248], rsi
142291753: 488d4620                     lea rax, [rsi + 0x20]
142291757: 48898538010000               mov qword ptr [rbp + 0x138], rax
14229175e: 4c89a550020000               mov qword ptr [rbp + 0x250], r12
142291765: 498d442420                   lea rax, [r12 + 0x20]
14229176a: 48898540010000               mov qword ptr [rbp + 0x140], rax
142291771: 418b421c                     mov eax, dword ptr [r10 + 0x1c]
142291775: 8985b8020000                 mov dword ptr [rbp + 0x2b8], eax
14229177b: 498b02                       mov rax, qword ptr [r10]
14229177e: 4d8b5210                     mov r10, qword ptr [r10 + 0x10]
142291782: 4c8995b0000000               mov qword ptr [rbp + 0xb0], r10
142291789: 488985e0010000               mov qword ptr [rbp + 0x1e0], rax
142291790: 488d4020                     lea rax, [rax + 0x20]
142291794: 488985d8000000               mov qword ptr [rbp + 0xd8], rax
14229179b: 4c89ade8010000               mov qword ptr [rbp + 0x1e8], r13
1422917a2: 498d4520                     lea rax, [r13 + 0x20]
1422917a6: 4c8bad00050000               mov r13, qword ptr [rbp + 0x500]
1422917ad: 488985e8000000               mov qword ptr [rbp + 0xe8], rax
1422917b4: 4c89b570020000               mov qword ptr [rbp + 0x270], r14
1422917bb: 498d4620                     lea rax, [r14 + 0x20]
1422917bf: 48898548010000               mov qword ptr [rbp + 0x148], rax
1422917c6: 48898de0030000               mov qword ptr [rbp + 0x3e0], rcx
1422917cd: 488d4120                     lea rax, [rcx + 0x20]
1422917d1: 48898598020000               mov qword ptr [rbp + 0x298], rax
1422917d8: 4c898dd8030000               mov qword ptr [rbp + 0x3d8], r9
1422917df: 498d4120                     lea rax, [r9 + 0x20]
1422917e3: 48898578020000               mov qword ptr [rbp + 0x278], rax
1422917ea: 488b8510050000               mov rax, qword ptr [rbp + 0x510]
1422917f1: 488d4020                     lea rax, [rax + 0x20]
1422917f5: 48898580020000               mov qword ptr [rbp + 0x280], rax
1422917fc: 488b8518050000               mov rax, qword ptr [rbp + 0x518]
142291803: 488d4020                     lea rax, [rax + 0x20]
142291807: 48898590020000               mov qword ptr [rbp + 0x290], rax
14229180e: 488b8520050000               mov rax, qword ptr [rbp + 0x520]
142291815: 488d4020                     lea rax, [rax + 0x20]
142291819: 488985a0020000               mov qword ptr [rbp + 0x2a0], rax
142291820: 48899d70030000               mov qword ptr [rbp + 0x370], rbx
142291827: 488d4320                     lea rax, [rbx + 0x20]
14229182b: 48898508010000               mov qword ptr [rbp + 0x108], rax
142291832: 488b85d0030000               mov rax, qword ptr [rbp + 0x3d0]
142291839: 488d4020                     lea rax, [rax + 0x20]
14229183d: 488985f0000000               mov qword ptr [rbp + 0xf0], rax
142291844: 48899560030000               mov qword ptr [rbp + 0x360], rdx
14229184b: 488d4220                     lea rax, [rdx + 0x20]
14229184f: 48898510010000               mov qword ptr [rbp + 0x110], rax
142291856: 4c899d68030000               mov qword ptr [rbp + 0x368], r11
14229185d: 498d4320                     lea rax, [r11 + 0x20]
142291861: 48898518010000               mov qword ptr [rbp + 0x118], rax
142291868: 488b8580030000               mov rax, qword ptr [rbp + 0x380]
14229186f: 488d4020                     lea rax, [rax + 0x20]
142291873: 48898588010000               mov qword ptr [rbp + 0x188], rax
14229187a: 488b8578030000               mov rax, qword ptr [rbp + 0x378]
142291881: 488d4020                     lea rax, [rax + 0x20]
142291885: 48898578010000               mov qword ptr [rbp + 0x178], rax
14229188c: 488b8590030000               mov rax, qword ptr [rbp + 0x390]
142291893: 488d4020                     lea rax, [rax + 0x20]
142291897: 48898590010000               mov qword ptr [rbp + 0x190], rax
14229189e: 488b8598030000               mov rax, qword ptr [rbp + 0x398]
1422918a5: 488d4020                     lea rax, [rax + 0x20]
1422918a9: 48898598010000               mov qword ptr [rbp + 0x198], rax
1422918b0: 488b8558030000               mov rax, qword ptr [rbp + 0x358]
1422918b7: 488d4020                     lea rax, [rax + 0x20]
1422918bb: 48898528010000               mov qword ptr [rbp + 0x128], rax
1422918c2: 488b8588030000               mov rax, qword ptr [rbp + 0x388]
1422918c9: 488d4020                     lea rax, [rax + 0x20]
1422918cd: 488985a0010000               mov qword ptr [rbp + 0x1a0], rax
1422918d4: 4c8985a8020000               mov qword ptr [rbp + 0x2a8], r8
1422918db: 498d4020                     lea rax, [r8 + 0x20]
1422918df: 488985a8010000               mov qword ptr [rbp + 0x1a8], rax
1422918e6: 4889bdd8010000               mov qword ptr [rbp + 0x1d8], rdi
1422918ed: 488d4720                     lea rax, [rdi + 0x20]
1422918f1: 488985d0000000               mov qword ptr [rbp + 0xd0], rax
1422918f8: b808000000                   mov eax, 8
1422918fd: 488985f0010000               mov qword ptr [rbp + 0x1f0], rax
142291904: 41be00000000                 mov r14d, 0
14229190a: 488985f8010000               mov qword ptr [rbp + 0x1f8], rax
142291911: 48c785c803000000000000       mov qword ptr [rbp + 0x3c8], 0
14229191c: 48c7851803000000000000       mov qword ptr [rbp + 0x318], 0
142291927: 48c785b803000000000000       mov qword ptr [rbp + 0x3b8], 0
142291932: 488985b0030000               mov qword ptr [rbp + 0x3b0], rax
142291939: 48c7858006000000000000       mov qword ptr [rbp + 0x680], 0
142291944: 48c7855003000000000000       mov qword ptr [rbp + 0x350], 0
14229194f: c785f805000000000000         mov dword ptr [rbp + 0x5f8], 0
142291959: 31c0                         xor eax, eax
14229195b: c785f405000000000000         mov dword ptr [rbp + 0x5f4], 0
142291965: 48c785f002000000000000       mov qword ptr [rbp + 0x2f0], 0
142291970: eb36                         jmp 0x1422919a8
142291972: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
142291980: 488b95d8010000               mov rdx, qword ptr [rbp + 0x1d8]
142291987: 488b4228                     mov rax, qword ptr [rdx + 0x28]
14229198b: 4889f1                       mov rcx, rsi
14229198e: 48c1e104                     shl rcx, 4
142291992: 48893c08                     mov qword ptr [rax + rcx], rdi
142291996: c644080800                   mov byte ptr [rax + rcx + 8], 0
14229199b: 48ffc6                       inc rsi
14229199e: 48897230                     mov qword ptr [rdx + 0x30], rsi
1422919a2: 48ff4240                     inc qword ptr [rdx + 0x40]
1422919a6: b001                         mov al, 1
1422919a8: 80bd7f06000000               cmp byte ptr [rbp + 0x67f], 0
1422919af: 0f84bb000000                 je 0x142291a70
1422919b5: 4c8ba518030000               mov r12, qword ptr [rbp + 0x318]
1422919bc: 4c8b8db8030000               mov r9, qword ptr [rbp + 0x3b8]
1422919c3: 4d39e1                       cmp r9, r12
1422919c6: 4c8bbd10030000               mov r15, qword ptr [rbp + 0x310]
1422919cd: 4c8b85d0010000               mov r8, qword ptr [rbp + 0x1d0]
1422919d4: 757e                         jne 0x142291a54
1422919d6: 662e0f1f840000000000         nop word ptr cs:[rax + rax]
1422919e0: 4d39c7                       cmp r15, r8
1422919e3: 0f845a4e0000                 je 0x142296843
1422919e9: 418b17                       mov edx, dword ptr [r15]
1422919ec: 4983c704                     add r15, 4
1422919f0: 498b8da8010000               mov rcx, qword ptr [r13 + 0x1a8]
1422919f7: 488d14d2                     lea rdx, [rdx + rdx*8]
1422919fb: 4c8b64d110                   mov r12, qword ptr [rcx + rdx*8 + 0x10]
142291a00: 4d85e4                       test r12, r12
142291a03: 74db                         je 0x1422919e0
142291a05: 488d0cd1                     lea rcx, [rcx + rdx*8]
142291a09: 488b9528050000               mov rdx, qword ptr [rbp + 0x528]
142291a10: 488b9210010000               mov rdx, qword ptr [rdx + 0x110]
142291a17: 4c8b4138                     mov r8, qword ptr [rcx + 0x38]
142291a1b: 498b14d0                     mov rdx, qword ptr [r8 + rdx*8]
142291a1f: 4c8b4108                     mov r8, qword ptr [rcx + 8]
142291a23: 4c8985f0010000               mov qword ptr [rbp + 0x1f0], r8
142291a2a: 488b4918                     mov rcx, qword ptr [rcx + 0x18]
142291a2e: 48f7d2                       not rdx
142291a31: 488d1452                     lea rdx, [rdx + rdx*2]
142291a35: 48c1e204                     shl rdx, 4
142291a39: 4c8b441110                   mov r8, qword ptr [rcx + rdx + 0x10]
142291a3e: 4c8985c8030000               mov qword ptr [rbp + 0x3c8], r8
142291a45: 488b4c1128                   mov rcx, qword ptr [rcx + rdx + 0x28]
142291a4a: 48898d48030000               mov qword ptr [rbp + 0x348], rcx
142291a51: 4531c9                       xor r9d, r9d
142291a54: 488b8df0010000               mov rcx, qword ptr [rbp + 0x1f0]
142291a5b: 4a8d0cc9                     lea rcx, [rcx + r9*8]
142291a5f: 4489ca                       mov edx, r9d
142291a62: e9da000000                   jmp 0x142291b41
142291a67: 660f1f840000000000           nop word ptr [rax + rax]
142291a70: 4c8ba518030000               mov r12, qword ptr [rbp + 0x318]
142291a77: 4c8b8db8030000               mov r9, qword ptr [rbp + 0x3b8]
142291a7e: 4d39e1                       cmp r9, r12
142291a81: 4c8bbd10030000               mov r15, qword ptr [rbp + 0x310]
142291a88: 4c8b85d0010000               mov r8, qword ptr [rbp + 0x1d0]
142291a8f: 0f8595000000                 jne 0x142291b2a
142291a95: 66662e0f1f840000000000       nop word ptr cs:[rax + rax]
142291aa0: 4d39c7                       cmp r15, r8
142291aa3: 0f849a4d0000                 je 0x142296843
142291aa9: 418b17                       mov edx, dword ptr [r15]
142291aac: 4983c704                     add r15, 4
142291ab0: 498b8d00010000               mov rcx, qword ptr [r13 + 0x100]
142291ab7: 488d1492                     lea rdx, [rdx + rdx*4]
142291abb: 48c1e205                     shl rdx, 5
142291abf: 4c8b641158                   mov r12, qword ptr [rcx + rdx + 0x58]
142291ac4: 4d85e4                       test r12, r12
142291ac7: 74d7                         je 0x142291aa0
142291ac9: 4801d1                       add rcx, rdx
142291acc: 8b9194000000                 mov edx, dword ptr [rcx + 0x94]
142291ad2: 4d8b85a8010000               mov r8, qword ptr [r13 + 0x1a8]
142291ad9: 488d14d2                     lea rdx, [rdx + rdx*8]
142291add: 4c8b8d28050000               mov r9, qword ptr [rbp + 0x528]
142291ae4: 4d8b8910010000               mov r9, qword ptr [r9 + 0x110]
142291aeb: 4d8b54d018                   mov r10, qword ptr [r8 + rdx*8 + 0x18]
142291af0: 498b54d038                   mov rdx, qword ptr [r8 + rdx*8 + 0x38]
142291af5: 4a8b14ca                     mov rdx, qword ptr [rdx + r9*8]
142291af9: 48f7d2                       not rdx
142291afc: 488d1452                     lea rdx, [rdx + rdx*2]
142291b00: 48c1e204                     shl rdx, 4
142291b04: 4d8b441210                   mov r8, qword ptr [r10 + rdx + 0x10]
142291b09: 4c8985c8030000               mov qword ptr [rbp + 0x3c8], r8
142291b10: 498b541228                   mov rdx, qword ptr [r10 + rdx + 0x28]
142291b15: 48899548030000               mov qword ptr [rbp + 0x348], rdx
142291b1c: 488b4950                     mov rcx, qword ptr [rcx + 0x50]
142291b20: 48898df8010000               mov qword ptr [rbp + 0x1f8], rcx
142291b27: 4531c9                       xor r9d, r9d
142291b2a: 4c89ca                       mov rdx, r9
142291b2d: 48c1e204                     shl rdx, 4
142291b31: 4c8b85f8010000               mov r8, qword ptr [rbp + 0x1f8]
142291b38: 498d0c10                     lea rcx, [r8 + rdx]
142291b3c: 418b541008                   mov edx, dword ptr [r8 + rdx + 8]
142291b41: 4c8d1492                     lea r10, [rdx + rdx*4]
142291b45: 49c1e205                     shl r10, 5
142291b49: 4c0395c8030000               add r10, qword ptr [rbp + 0x3c8]
142291b50: 4c8b8548030000               mov r8, qword ptr [rbp + 0x348]
142291b57: 498d1490                     lea rdx, [r8 + rdx*4]
142291b5b: 488b31                       mov rsi, qword ptr [rcx]
142291b5e: 4889f1                       mov rcx, rsi
142291b61: 48c1e920                     shr rcx, 0x20
142291b65: 0f84d84c0000                 je 0x142296843
142291b6b: 4c898db8030000               mov qword ptr [rbp + 0x3b8], r9
142291b72: 4c89b5a8060000               mov qword ptr [rbp + 0x6a8], r14
142291b79: 8b85ec050000                 mov eax, dword ptr [rbp + 0x5ec]
142291b7f: 8902                         mov dword ptr [rdx], eax
142291b81: 488b9d60060000               mov rbx, qword ptr [rbp + 0x660]
142291b88: 8b8ba0000000                 mov ecx, dword ptr [rbx + 0xa0]
142291b8e: f30f108ba4000000             movss xmm1, dword ptr [rbx + 0xa4]
142291b96: 4c899508070000               mov qword ptr [rbp + 0x708], r10
142291b9d: e8eede6afe                   call 0x14093fa90
142291ba2: 89c7                         mov edi, eax
142291ba4: 8b8ba8000000                 mov ecx, dword ptr [rbx + 0xa8]
142291baa: f30f108bac000000             movss xmm1, dword ptr [rbx + 0xac]
142291bb2: e8d9de6afe                   call 0x14093fa90
142291bb7: 4c8b9d08070000               mov r11, qword ptr [rbp + 0x708]
142291bbe: 89c1                         mov ecx, eax
142291bc0: 89f8                         mov eax, edi
142291bc2: 08c8                         or al, cl
142291bc4: 7415                         je 0x142291bdb
142291bc6: 41f60301                     test byte ptr [r11], 1
142291bca: 750f                         jne 0x142291bdb
142291bcc: 498b5360                     mov rdx, qword ptr [r11 + 0x60]
142291bd0: 49c70301000000               mov qword ptr [r11], 1
142291bd7: 49895308                     mov qword ptr [r11 + 8], rdx
142291bdb: 488b9538030000               mov rdx, qword ptr [rbp + 0x338]
142291be2: 837a1001                     cmp dword ptr [rdx + 0x10], 1
142291be6: 0f85a4000000                 jne 0x142291c90
142291bec: 48397218                     cmp qword ptr [rdx + 0x18], rsi
142291bf0: 0f94c2                       sete dl
142291bf3: 8995cc060000                 mov dword ptr [rbp + 0x6cc], edx
142291bf9: 84d0                         test al, dl
142291bfb: 0f8499000000                 je 0x142291c9a
142291c01: 410fb603                     movzx eax, byte ptr [r11]
142291c05: 4d8d4308                     lea r8, [r11 + 8]
142291c09: 84c0                         test al, al
142291c0b: ba00000000                   mov edx, 0
142291c10: 4c0f44c2                     cmove r8, rdx
142291c14: a801                         test al, 1
142291c16: 0f84a5500000                 je 0x142296cc1
142291c1c: 4084ff                       test dil, dil
142291c1f: 0f8483300000                 je 0x142294ca8
142291c25: 4d8b4b20                     mov r9, qword ptr [r11 + 0x20]
142291c29: 4d8d5101                     lea r10, [r9 + 1]
142291c2d: eb29                         jmp 0x142291c58
142291c2f: 90                           nop 
142291c30: 31d2                         xor edx, edx
142291c32: 41f7f1                       div r9d
142291c35: 498910                       mov qword ptr [r8], rdx
142291c38: 498b4348                     mov rax, qword ptr [r11 + 0x48]
142291c3c: 4839c2                       cmp rdx, rax
142291c3f: 0f83f94f0000                 jae 0x142296c3e
142291c45: 498b4340                     mov rax, qword ptr [r11 + 0x40]
142291c49: 488d1492                     lea rdx, [rdx + rdx*4]
142291c4d: 807cd01805                   cmp byte ptr [rax + rdx*8 + 0x18], 5
142291c52: 0f8550300000                 jne 0x142294ca8
142291c58: 49ffca                       dec r10
142291c5b: 0f8447300000                 je 0x142294ca8
142291c61: 498b00                       mov rax, qword ptr [r8]
142291c64: 48ffc0                       inc rax
142291c67: 4889c2                       mov rdx, rax
142291c6a: 4c09ca                       or rdx, r9
142291c6d: 48c1ea20                     shr rdx, 0x20
142291c71: 74bd                         je 0x142291c30
142291c73: 31d2                         xor edx, edx
142291c75: 49f7f1                       div r9
142291c78: 498910                       mov qword ptr [r8], rdx
142291c7b: 498b4348                     mov rax, qword ptr [r11 + 0x48]
142291c7f: 4839c2                       cmp rdx, rax
142291c82: 72c1                         jb 0x142291c45
142291c84: e9b54f0000                   jmp 0x142296c3e
142291c89: 0f1f8000000000               nop dword ptr [rax]
142291c90: c785cc06000000000000         mov dword ptr [rbp + 0x6cc], 0
142291c9a: 4c89a518030000               mov qword ptr [rbp + 0x318], r12
142291ca1: 4c89bd10030000               mov qword ptr [rbp + 0x310], r15
142291ca8: 48ff85b8030000               inc qword ptr [rbp + 0x3b8]
142291caf: 49ffc6                       inc r14
142291cb2: 4c89b508050000               mov qword ptr [rbp + 0x508], r14
142291cb9: 4d8b4318                     mov r8, qword ptr [r11 + 0x18]
142291cbd: 498b4320                     mov rax, qword ptr [r11 + 0x20]
142291cc1: 498d04c0                     lea rax, [r8 + rax*8]
142291cc5: 48898528060000               mov qword ptr [rbp + 0x628], rax
142291ccc: c685a606000002               mov byte ptr [rbp + 0x6a6], 2
142291cd3: c685f606000004               mov byte ptr [rbp + 0x6f6], 4
142291cda: 48c785c801000000000000       mov qword ptr [rbp + 0x1c8], 0
142291ce5: c685de06000003               mov byte ptr [rbp + 0x6de], 3
142291cec: c785f005000000000000         mov dword ptr [rbp + 0x5f0], 0
142291cf6: c685ff06000003               mov byte ptr [rbp + 0x6ff], 3
142291cfd: 48c7854003000000000000       mov qword ptr [rbp + 0x340], 0
142291d08: c685a706000002               mov byte ptr [rbp + 0x6a7], 2
142291d0f: c785ec06000000000000         mov dword ptr [rbp + 0x6ec], 0
142291d19: 48c7852803000000000000       mov qword ptr [rbp + 0x328], 0
142291d24: 48c7853003000000000000       mov qword ptr [rbp + 0x330], 0
142291d2f: c785a404000000000000         mov dword ptr [rbp + 0x4a4], 0
142291d39: c685f706000003               mov byte ptr [rbp + 0x6f7], 3
142291d40: 48c785c001000000000000       mov qword ptr [rbp + 0x1c0], 0
142291d4b: 31c9                         xor ecx, ecx
142291d4d: e9b2000000                   jmp 0x142291e04
142291d52: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
142291d60: 4531ed                       xor r13d, r13d
142291d63: 4d89e0                       mov r8, r12
142291d66: 4983c008                     add r8, 8
142291d6a: 488b9508060000               mov rdx, qword ptr [rbp + 0x608]
142291d71: 488b9278010000               mov rdx, qword ptr [rdx + 0x178]
142291d78: 488b0cd1                     mov rcx, qword ptr [rcx + rdx*8]
142291d7c: 48f7d1                       not rcx
142291d7f: 488d0c49                     lea rcx, [rcx + rcx*2]
142291d83: 48c1e104                     shl rcx, 4
142291d87: 4c8b640810                   mov r12, qword ptr [rax + rcx + 0x10]
142291d8c: 4c8b7c0828                   mov r15, qword ptr [rax + rcx + 0x28]
142291d91: 488b9500070000               mov rdx, qword ptr [rbp + 0x700]
142291d98: 488d04d2                     lea rax, [rdx + rdx*8]
142291d9c: 498d0c93                     lea rcx, [r11 + rdx*4]
142291da0: 48c1e920                     shr rcx, 0x20
142291da4: 48898db0030000               mov qword ptr [rbp + 0x3b0], rcx
142291dab: 8b8df8060000                 mov ecx, dword ptr [rbp + 0x6f8]
142291db1: 4c8b8dd0060000               mov r9, qword ptr [rbp + 0x6d0]
142291db8: 41890c91                     mov dword ptr [r9 + rdx*4], ecx
142291dbc: 41c644822000                 mov byte ptr [r10 + rax*4 + 0x20], 0
142291dc2: 4c8b9d08070000               mov r11, qword ptr [rbp + 0x708]
142291dc9: 498b4b08                     mov rcx, qword ptr [r11 + 8]
142291dcd: 48398de0060000               cmp qword ptr [rbp + 0x6e0], rcx
142291dd4: 498b0b                       mov rcx, qword ptr [r11]
142291dd7: 410f94c1                     sete r9b
142291ddb: 4120c9                       and r9b, cl
142291dde: 488b8df0020000               mov rcx, qword ptr [rbp + 0x2f0]
142291de5: 4408c9                       or cl, r9b
142291de8: 80e101                       and cl, 1
142291deb: 48898df0020000               mov qword ptr [rbp + 0x2f0], rcx
142291df2: 4c89f1                       mov rcx, r14
142291df5: 48ffc1                       inc rcx
142291df8: 450f2e648218                 ucomiss xmm12, dword ptr [r10 + rax*4 + 0x18]
142291dfe: 0f86f0060000                 jbe 0x1422924f4
142291e04: 4c3b8528060000               cmp r8, qword ptr [rbp + 0x628]
142291e0b: 0f84df290000                 je 0x1422947f0
142291e11: 4989ce                       mov r14, rcx
142291e14: 48898de0060000               mov qword ptr [rbp + 0x6e0], rcx
142291e1b: 41807b3009                   cmp byte ptr [r11 + 0x30], 9
142291e20: 4c8bbd88000000               mov r15, qword ptr [rbp + 0x88]
142291e27: 4c8bad08060000               mov r13, qword ptr [rbp + 0x608]
142291e2e: 7566                         jne 0x142291e96
142291e30: 418b7b34                     mov edi, dword ptr [r11 + 0x34]
142291e34: 488b8590000000               mov rax, qword ptr [rbp + 0x90]
142291e3b: 8b8db0020000                 mov ecx, dword ptr [rbp + 0x2b0]
142291e41: 8908                         mov dword ptr [rax], ecx
142291e43: 488b85b8010000               mov rax, qword ptr [rbp + 0x1b8]
142291e4a: 488b7030                     mov rsi, qword ptr [rax + 0x30]
142291e4e: 488b5840                     mov rbx, qword ptr [rax + 0x40]
142291e52: 483b7020                     cmp rsi, qword ptr [rax + 0x20]
142291e56: 7519                         jne 0x142291e71
142291e58: 488b8d98000000               mov rcx, qword ptr [rbp + 0x98]
142291e5f: 488d156294da00               lea rdx, [rip + 0xda9462]
142291e66: 4d89c4                       mov r12, r8
142291e69: e8c2072dfe                   call 0x140562630
142291e6e: 4d89e0                       mov r8, r12
142291e71: 488b95b8010000               mov rdx, qword ptr [rbp + 0x1b8]
142291e78: 488b4228                     mov rax, qword ptr [rdx + 0x28]
142291e7c: 4889f1                       mov rcx, rsi
142291e7f: 48c1e104                     shl rcx, 4
142291e83: 48891c08                     mov qword ptr [rax + rcx], rbx
142291e87: 897c0808                     mov dword ptr [rax + rcx + 8], edi
142291e8b: 48ffc6                       inc rsi
142291e8e: 48897230                     mov qword ptr [rdx + 0x30], rsi
142291e92: 48ff4240                     inc qword ptr [rdx + 0x40]
142291e96: 498b00                       mov rax, qword ptr [r8]
142291e99: 89c1                         mov ecx, eax
142291e9b: 493b4f10                     cmp rcx, qword ptr [r15 + 0x10]
142291e9f: 0f83434d0000                 jae 0x142296be8
142291ea5: 4d89c4                       mov r12, r8
142291ea8: 4d8b4708                     mov r8, qword ptr [r15 + 8]
142291eac: 48c1e820                     shr rax, 0x20
142291eb0: 4c8d0c89                     lea r9, [rcx + rcx*4]
142291eb4: ba01000000                   mov edx, 1
142291eb9: 43390488                     cmp dword ptr [r8 + r9*4], eax
142291ebd: 0f852e4d0000                 jne 0x142296bf1
142291ec3: 4f8d1488                     lea r10, [r8 + r9*4]
142291ec7: 458b4204                     mov r8d, dword ptr [r10 + 4]
142291ecb: 41b9ffffffff                 mov r9d, 0xffffffff
142291ed1: 4d39c8                       cmp r8, r9
142291ed4: 0f84174d0000                 je 0x142296bf1
142291eda: 31d2                         xor edx, edx
142291edc: 4d394538                     cmp qword ptr [r13 + 0x38], r8
142291ee0: 0f86124d0000                 jbe 0x142296bf8
142291ee6: 4d8b4d28                     mov r9, qword ptr [r13 + 0x28]
142291eea: 4589c3                       mov r11d, r8d
142291eed: 41c1eb06                     shr r11d, 6
142291ef1: 4f8b0cd9                     mov r9, qword ptr [r9 + r11*8]
142291ef5: 4d0fa3c1                     bt r9, r8
142291ef9: 0f83f94c0000                 jae 0x142296bf8
142291eff: 4d398708010000               cmp qword ptr [r15 + 0x108], r8
142291f06: 7618                         jbe 0x142291f20
142291f08: 4f8d0480                     lea r8, [r8 + r8*4]
142291f0c: 49c1e005                     shl r8, 5
142291f10: 4d038700010000               add r8, qword ptr [r15 + 0x100]
142291f17: eb0a                         jmp 0x142291f23
142291f19: 0f1f8000000000               nop dword ptr [rax]
142291f20: 4531c0                       xor r8d, r8d
142291f23: 418b420c                     mov eax, dword ptr [r10 + 0xc]
142291f27: 498b8fa8010000               mov rcx, qword ptr [r15 + 0x1a8]
142291f2e: 4c8d0cc0                     lea r9, [rax + rax*8]
142291f32: 4a8b54c940                   mov rdx, qword ptr [rcx + r9*8 + 0x40]
142291f37: 4a8b44c918                   mov rax, qword ptr [rcx + r9*8 + 0x18]
142291f3c: 4a8b4cc938                   mov rcx, qword ptr [rcx + r9*8 + 0x38]
142291f41: 4d8b9d18010000               mov r11, qword ptr [r13 + 0x118]
142291f48: 4d8b8888000000               mov r9, qword ptr [r8 + 0x88]
142291f4f: 4d39d9                       cmp r9, r11
142291f52: 764c                         jbe 0x142291fa0
142291f54: 498bb080000000               mov rsi, qword ptr [r8 + 0x80]
142291f5b: 4a833cde00                   cmp qword ptr [rsi + r11*8], 0
142291f60: 743e                         je 0x142291fa0
142291f62: 41b701                       mov r15b, 1
142291f65: 4c39da                       cmp rdx, r11
142291f68: 7656                         jbe 0x142291fc0
142291f6a: 4e8b1cd9                     mov r11, qword ptr [rcx + r11*8]
142291f6e: 4d85db                       test r11, r11
142291f71: 744d                         je 0x142291fc0
142291f73: 49f7d3                       not r11
142291f76: 4f8d1c5b                     lea r11, [r11 + r11*2]
142291f7a: 49c1e304                     shl r11, 4
142291f7e: 4e8b5c1810                   mov r11, qword ptr [rax + r11 + 0x10]
142291f83: 4c899dd0040000               mov qword ptr [rbp + 0x4d0], r11
142291f8a: 4d8b9d20010000               mov r11, qword ptr [r13 + 0x120]
142291f91: 4d39d9                       cmp r9, r11
142291f94: 0f8696000000                 jbe 0x142292030
142291f9a: eb3b                         jmp 0x142291fd7
142291f9c: 0f1f4000                     nop dword ptr [rax]
142291fa0: 48c785d004000000000000       mov qword ptr [rbp + 0x4d0], 0
142291fab: 4531ff                       xor r15d, r15d
142291fae: 4d8b9d20010000               mov r11, qword ptr [r13 + 0x120]
142291fb5: 4d39d9                       cmp r9, r11
142291fb8: 7676                         jbe 0x142292030
142291fba: eb1b                         jmp 0x142291fd7
142291fbc: 0f1f4000                     nop dword ptr [rax]
142291fc0: 48c785d004000000000000       mov qword ptr [rbp + 0x4d0], 0
142291fcb: 4d8b9d20010000               mov r11, qword ptr [r13 + 0x120]
142291fd2: 4d39d9                       cmp r9, r11
142291fd5: 7659                         jbe 0x142292030
142291fd7: 498bb080000000               mov rsi, qword ptr [r8 + 0x80]
142291fde: 4a833cde00                   cmp qword ptr [rsi + r11*8], 0
142291fe3: 744b                         je 0x142292030
142291fe5: 40b601                       mov sil, 1
142291fe8: 4889b5f8020000               mov qword ptr [rbp + 0x2f8], rsi
142291fef: 4c39da                       cmp rdx, r11
142291ff2: 0f86a8000000                 jbe 0x1422920a0
142291ff8: 4e8b1cd9                     mov r11, qword ptr [rcx + r11*8]
142291ffc: 4d85db                       test r11, r11
142291fff: 0f849b000000                 je 0x1422920a0
142292005: 49f7d3                       not r11
142292008: 4f8d1c5b                     lea r11, [r11 + r11*2]
14229200c: 49c1e304                     shl r11, 4
142292010: 4e8b5c1810                   mov r11, qword ptr [rax + r11 + 0x10]
142292015: 4c899dd8040000               mov qword ptr [rbp + 0x4d8], r11
14229201c: 4d8b9d28010000               mov r11, qword ptr [r13 + 0x128]
142292023: 4d39d9                       cmp r9, r11
142292026: 0f868b000000                 jbe 0x1422920b7
14229202c: eb24                         jmp 0x142292052
14229202e: 6690                         nop 
142292030: 48c785d804000000000000       mov qword ptr [rbp + 0x4d8], 0
14229203b: 48c785f802000000000000       mov qword ptr [rbp + 0x2f8], 0
142292046: 4d8b9d28010000               mov r11, qword ptr [r13 + 0x128]
14229204d: 4d39d9                       cmp r9, r11
142292050: 7665                         jbe 0x1422920b7
142292052: 498bb080000000               mov rsi, qword ptr [r8 + 0x80]
142292059: 4a833cde00                   cmp qword ptr [rsi + r11*8], 0
14229205e: 410f95c3                     setne r11b
142292062: 44899da0040000               mov dword ptr [rbp + 0x4a0], r11d
142292069: 4d8b9d30010000               mov r11, qword ptr [r13 + 0x130]
142292070: 4d39d9                       cmp r9, r11
142292073: 7758                         ja 0x1422920cd
142292075: c7859c04000000000000         mov dword ptr [rbp + 0x49c], 0
14229207f: 4d8b9d38010000               mov r11, qword ptr [r13 + 0x138]
142292086: 4d39d9                       cmp r9, r11
142292089: 7765                         ja 0x1422920f0
14229208b: c7859804000000000000         mov dword ptr [rbp + 0x498], 0
142292095: eb70                         jmp 0x142292107
142292097: 660f1f840000000000           nop word ptr [rax + rax]
1422920a0: 48c785d804000000000000       mov qword ptr [rbp + 0x4d8], 0
1422920ab: 4d8b9d28010000               mov r11, qword ptr [r13 + 0x128]
1422920b2: 4d39d9                       cmp r9, r11
1422920b5: 779b                         ja 0x142292052
1422920b7: c785a004000000000000         mov dword ptr [rbp + 0x4a0], 0
1422920c1: 4d8b9d30010000               mov r11, qword ptr [r13 + 0x130]
1422920c8: 4d39d9                       cmp r9, r11
1422920cb: 76a8                         jbe 0x142292075
1422920cd: 498bb080000000               mov rsi, qword ptr [r8 + 0x80]
1422920d4: 4a833cde00                   cmp qword ptr [rsi + r11*8], 0
1422920d9: 410f95c3                     setne r11b
1422920dd: 44899d9c040000               mov dword ptr [rbp + 0x49c], r11d
1422920e4: 4d8b9d38010000               mov r11, qword ptr [r13 + 0x138]
1422920eb: 4d39d9                       cmp r9, r11
1422920ee: 769b                         jbe 0x14229208b
1422920f0: 498bb080000000               mov rsi, qword ptr [r8 + 0x80]
1422920f7: 4a833cde00                   cmp qword ptr [rsi + r11*8], 0
1422920fc: 410f95c3                     setne r11b
142292100: 44899d98040000               mov dword ptr [rbp + 0x498], r11d
142292107: 458b5210                     mov r10d, dword ptr [r10 + 0x10]
14229210b: 4c899500070000               mov qword ptr [rbp + 0x700], r10
142292112: 4d8b9510010000               mov r10, qword ptr [r13 + 0x110]
142292119: 4e8b14d1                     mov r10, qword ptr [rcx + r10*8]
14229211d: 49f7d2                       not r10
142292120: 4b8d3452                     lea rsi, [r10 + r10*2]
142292124: 48c1e604                     shl rsi, 4
142292128: 4c8b543010                   mov r10, qword ptr [rax + rsi + 0x10]
14229212d: 4c8b5c3020                   mov r11, qword ptr [rax + rsi + 0x20]
142292132: 488b743028                   mov rsi, qword ptr [rax + rsi + 0x28]
142292137: 4889b5d0060000               mov qword ptr [rbp + 0x6d0], rsi
14229213e: 498bb540010000               mov rsi, qword ptr [r13 + 0x140]
142292145: 4939f1                       cmp r9, rsi
142292148: 7656                         jbe 0x1422921a0
14229214a: 498bb880000000               mov rdi, qword ptr [r8 + 0x80]
142292151: 48833cf700                   cmp qword ptr [rdi + rsi*8], 0
142292156: 7448                         je 0x1422921a0
142292158: 40b701                       mov dil, 1
14229215b: 4889bd08030000               mov qword ptr [rbp + 0x308], rdi
142292162: 4839f2                       cmp rdx, rsi
142292165: 0f8685000000                 jbe 0x1422921f0
14229216b: 488b34f1                     mov rsi, qword ptr [rcx + rsi*8]
14229216f: 4885f6                       test rsi, rsi
142292172: 0f8478000000                 je 0x1422921f0
142292178: 48f7d6                       not rsi
14229217b: 488d3476                     lea rsi, [rsi + rsi*2]
14229217f: 48c1e604                     shl rsi, 4
142292183: 488b743010                   mov rsi, qword ptr [rax + rsi + 0x10]
142292188: 4889b5e0050000               mov qword ptr [rbp + 0x5e0], rsi
14229218f: 498bb548010000               mov rsi, qword ptr [r13 + 0x148]
142292196: 4939f1                       cmp r9, rsi
142292199: 766c                         jbe 0x142292207
14229219b: eb25                         jmp 0x1422921c2
14229219d: 0f1f00                       nop dword ptr [rax]
1422921a0: 48c785e005000000000000       mov qword ptr [rbp + 0x5e0], 0
1422921ab: 48c7850803000000000000       mov qword ptr [rbp + 0x308], 0
1422921b6: 498bb548010000               mov rsi, qword ptr [r13 + 0x148]
1422921bd: 4939f1                       cmp r9, rsi
1422921c0: 7645                         jbe 0x142292207
1422921c2: 498bb880000000               mov rdi, qword ptr [r8 + 0x80]
1422921c9: 48833cf700                   cmp qword ptr [rdi + rsi*8], 0
1422921ce: 400f95c7                     setne dil
1422921d2: 498bb550010000               mov rsi, qword ptr [r13 + 0x150]
1422921d9: 4939f1                       cmp r9, rsi
1422921dc: 4c89bd80000000               mov qword ptr [rbp + 0x80], r15
1422921e3: 7737                         ja 0x14229221c
1422921e5: e986000000                   jmp 0x142292270
1422921ea: 660f1f440000                 nop word ptr [rax + rax]
1422921f0: 48c785e005000000000000       mov qword ptr [rbp + 0x5e0], 0
1422921fb: 498bb548010000               mov rsi, qword ptr [r13 + 0x148]
142292202: 4939f1                       cmp r9, rsi
142292205: 77bb                         ja 0x1422921c2
142292207: 31ff                         xor edi, edi
142292209: 498bb550010000               mov rsi, qword ptr [r13 + 0x150]
142292210: 4939f1                       cmp r9, rsi
142292213: 4c89bd80000000               mov qword ptr [rbp + 0x80], r15
14229221a: 7654                         jbe 0x142292270
14229221c: 498b9880000000               mov rbx, qword ptr [r8 + 0x80]
142292223: 48833cf300                   cmp qword ptr [rbx + rsi*8], 0
142292228: 7446                         je 0x142292270
14229222a: b301                         mov bl, 1
14229222c: 4839f2                       cmp rdx, rsi
14229222f: 0f86ab000000                 jbe 0x1422922e0
142292235: 488b34f1                     mov rsi, qword ptr [rcx + rsi*8]
142292239: 4885f6                       test rsi, rsi
14229223c: 0f849e000000                 je 0x1422922e0
142292242: 48f7d6                       not rsi
142292245: 488d3476                     lea rsi, [rsi + rsi*2]
142292249: 48c1e604                     shl rsi, 4
14229224d: 488b743010                   mov rsi, qword ptr [rax + rsi + 0x10]
142292252: 4d8bbd58010000               mov r15, qword ptr [r13 + 0x158]
142292259: 4d39f9                       cmp r9, r15
14229225c: 0f868c000000                 jbe 0x1422922ee
142292262: eb1c                         jmp 0x142292280
142292264: 6666662e0f1f840000000000     nop word ptr cs:[rax + rax]
142292270: 31f6                         xor esi, esi
142292272: 31db                         xor ebx, ebx
142292274: 4d8bbd58010000               mov r15, qword ptr [r13 + 0x158]
14229227b: 4d39f9                       cmp r9, r15
14229227e: 766e                         jbe 0x1422922ee
142292280: 4d8ba880000000               mov r13, qword ptr [r8 + 0x80]
142292287: 4b837cfd0000                 cmp qword ptr [r13 + r15*8], 0
14229228d: 0f84aa000000                 je 0x14229233d
142292293: 41b501                       mov r13b, 1
142292296: 4c89ad00030000               mov qword ptr [rbp + 0x300], r13
14229229d: 4c39fa                       cmp rdx, r15
1422922a0: 4c8bad08060000               mov r13, qword ptr [rbp + 0x608]
1422922a7: 7677                         jbe 0x142292320
1422922a9: 4e8b3cf9                     mov r15, qword ptr [rcx + r15*8]
1422922ad: 4d85ff                       test r15, r15
1422922b0: 746e                         je 0x142292320
1422922b2: 49f7d7                       not r15
1422922b5: 4f8d3c7f                     lea r15, [r15 + r15*2]
1422922b9: 49c1e704                     shl r15, 4
1422922bd: 4e8b7c3810                   mov r15, qword ptr [rax + r15 + 0x10]
1422922c2: 4c89bdf0040000               mov qword ptr [rbp + 0x4f0], r15
1422922c9: 4d8bbd60010000               mov r15, qword ptr [r13 + 0x160]
1422922d0: 4d39f9                       cmp r9, r15
1422922d3: 0f8797000000                 ja 0x142292370
1422922d9: e9f2000000                   jmp 0x1422923d0
1422922de: 6690                         nop 
1422922e0: 31f6                         xor esi, esi
1422922e2: 4d8bbd58010000               mov r15, qword ptr [r13 + 0x158]
1422922e9: 4d39f9                       cmp r9, r15
1422922ec: 7792                         ja 0x142292280
1422922ee: 48c785f004000000000000       mov qword ptr [rbp + 0x4f0], 0
1422922f9: 48c7850003000000000000       mov qword ptr [rbp + 0x300], 0
142292304: 4d8bbd60010000               mov r15, qword ptr [r13 + 0x160]
14229230b: 4d39f9                       cmp r9, r15
14229230e: 7760                         ja 0x142292370
142292310: e9bb000000                   jmp 0x1422923d0
142292315: 66662e0f1f840000000000       nop word ptr cs:[rax + rax]
142292320: 48c785f004000000000000       mov qword ptr [rbp + 0x4f0], 0
14229232b: 4d8bbd60010000               mov r15, qword ptr [r13 + 0x160]
142292332: 4d39f9                       cmp r9, r15
142292335: 0f8695000000                 jbe 0x1422923d0
14229233b: eb33                         jmp 0x142292370
14229233d: 48c785f004000000000000       mov qword ptr [rbp + 0x4f0], 0
142292348: 48c7850003000000000000       mov qword ptr [rbp + 0x300], 0
142292353: 4c8bad08060000               mov r13, qword ptr [rbp + 0x608]
14229235a: 4d8bbd60010000               mov r15, qword ptr [r13 + 0x160]
142292361: 4d39f9                       cmp r9, r15
142292364: 766a                         jbe 0x1422923d0
142292366: 662e0f1f840000000000         nop word ptr cs:[rax + rax]
142292370: 4d8ba880000000               mov r13, qword ptr [r8 + 0x80]
142292377: 4b837cfd0000                 cmp qword ptr [r13 + r15*8], 0
14229237d: 7451                         je 0x1422923d0
14229237f: 41b501                       mov r13b, 1
142292382: 4c89ada8030000               mov qword ptr [rbp + 0x3a8], r13
142292389: 4c39fa                       cmp rdx, r15
14229238c: 7672                         jbe 0x142292400
14229238e: 4e8b3cf9                     mov r15, qword ptr [rcx + r15*8]
142292392: 4d85ff                       test r15, r15
142292395: 7469                         je 0x142292400
142292397: 49f7d7                       not r15
14229239a: 4f8d3c7f                     lea r15, [r15 + r15*2]
14229239e: 49c1e704                     shl r15, 4
1422923a2: 4e8b7c3810                   mov r15, qword ptr [rax + r15 + 0x10]
1422923a7: 4c89bde8040000               mov qword ptr [rbp + 0x4e8], r15
1422923ae: 4c8bbd08060000               mov r15, qword ptr [rbp + 0x608]
1422923b5: 4d8bbf68010000               mov r15, qword ptr [r15 + 0x168]
1422923bc: 4d39f9                       cmp r9, r15
1422923bf: 0f86bb000000                 jbe 0x142292480
1422923c5: eb57                         jmp 0x14229241e
1422923c7: 660f1f840000000000           nop word ptr [rax + rax]
1422923d0: 48c785e804000000000000       mov qword ptr [rbp + 0x4e8], 0
1422923db: 48c785a803000000000000       mov qword ptr [rbp + 0x3a8], 0
1422923e6: 4c8bbd08060000               mov r15, qword ptr [rbp + 0x608]
1422923ed: 4d8bbf68010000               mov r15, qword ptr [r15 + 0x168]
1422923f4: 4d39f9                       cmp r9, r15
1422923f7: 0f8683000000                 jbe 0x142292480
1422923fd: eb1f                         jmp 0x14229241e
1422923ff: 90                           nop 
142292400: 48c785e804000000000000       mov qword ptr [rbp + 0x4e8], 0
14229240b: 4c8bbd08060000               mov r15, qword ptr [rbp + 0x608]
142292412: 4d8bbf68010000               mov r15, qword ptr [r15 + 0x168]
142292419: 4d39f9                       cmp r9, r15
14229241c: 7662                         jbe 0x142292480
14229241e: 4d8b8080000000               mov r8, qword ptr [r8 + 0x80]
142292425: 4b833cf800                   cmp qword ptr [r8 + r15*8], 0
14229242a: 7454                         je 0x142292480
14229242c: 41b001                       mov r8b, 1
14229242f: 4c8985a0030000               mov qword ptr [rbp + 0x3a0], r8
142292436: 4c39fa                       cmp rdx, r15
142292439: 7675                         jbe 0x1422924b0
14229243b: 4e8b04f9                     mov r8, qword ptr [rcx + r15*8]
14229243f: 4d85c0                       test r8, r8
142292442: 746c                         je 0x1422924b0
142292444: 49f7d0                       not r8
142292447: 4f8d0440                     lea r8, [r8 + r8*2]
14229244b: 49c1e004                     shl r8, 4
14229244f: 4e8b440010                   mov r8, qword ptr [rax + r8 + 0x10]
142292454: 4c8985e0040000               mov qword ptr [rbp + 0x4e0], r8
14229245b: 4c8b8508060000               mov r8, qword ptr [rbp + 0x608]
142292462: 4d8b8070010000               mov r8, qword ptr [r8 + 0x170]
142292469: 4c39c2                       cmp rdx, r8
14229246c: 0f86eef8ffff                 jbe 0x142291d60
142292472: eb5e                         jmp 0x1422924d2
142292474: 6666662e0f1f840000000000     nop word ptr cs:[rax + rax]
142292480: 48c785e004000000000000       mov qword ptr [rbp + 0x4e0], 0
14229248b: 48c785a003000000000000       mov qword ptr [rbp + 0x3a0], 0
142292496: 4c8b8508060000               mov r8, qword ptr [rbp + 0x608]
14229249d: 4d8b8070010000               mov r8, qword ptr [r8 + 0x170]
1422924a4: 4c39c2                       cmp rdx, r8
1422924a7: 0f86b3f8ffff                 jbe 0x142291d60
1422924ad: eb23                         jmp 0x1422924d2
1422924af: 90                           nop 
1422924b0: 48c785e004000000000000       mov qword ptr [rbp + 0x4e0], 0
1422924bb: 4c8b8508060000               mov r8, qword ptr [rbp + 0x608]
1422924c2: 4d8b8070010000               mov r8, qword ptr [r8 + 0x170]
1422924c9: 4c39c2                       cmp rdx, r8
1422924cc: 0f868ef8ffff                 jbe 0x142291d60
1422924d2: 4a8b14c1                     mov rdx, qword ptr [rcx + r8*8]
1422924d6: 4885d2                       test rdx, rdx
1422924d9: 0f8481f8ffff                 je 0x142291d60
1422924df: 48f7d2                       not rdx
1422924e2: 488d1452                     lea rdx, [rdx + rdx*2]
1422924e6: 48c1e204                     shl rdx, 4
1422924ea: 4c8b6c1010                   mov r13, qword ptr [rax + rdx + 0x10]
1422924ef: e96ff8ffff                   jmp 0x142291d63
1422924f4: 44888d17070000               mov byte ptr [rbp + 0x717], r9b
1422924fb: 4c89bdc0030000               mov qword ptr [rbp + 0x3c0], r15
142292502: 4c89a5f8040000               mov qword ptr [rbp + 0x4f8], r12
142292509: 48898dc0060000               mov qword ptr [rbp + 0x6c0], rcx
142292510: 4c8985b8060000               mov qword ptr [rbp + 0x6b8], r8
142292517: 498d0482                     lea rax, [r10 + rax*4]
14229251b: 488985b0060000               mov qword ptr [rbp + 0x6b0], rax
142292522: 4889d0                       mov rax, rdx
142292525: 48c1e004                     shl rax, 4
142292529: 4901c5                       add r13, rax
14229252c: 4c8db530050000               lea r14, [rbp + 0x530]
142292533: 4c89f1                       mov rcx, r14
142292536: e8756a71fe                   call 0x1409a8fb0
14229253b: c785380600000c000000         mov dword ptr [rbp + 0x638], 0xc
142292545: 8b455c                       mov eax, dword ptr [rbp + 0x5c]
142292548: 488d8d3c060000               lea rcx, [rbp + 0x63c]
14229254f: 894110                       mov dword ptr [rcx + 0x10], eax
142292552: 0f10454c                     movups xmm0, xmmword ptr [rbp + 0x4c]
142292556: 0f1101                       movups xmmword ptr [rcx], xmm0
142292559: 4c8da5e8030000               lea r12, [rbp + 0x3e8]
142292560: 4c89e1                       mov rcx, r12
142292563: 4c89f2                       mov rdx, r14
142292566: 4c8dbd38060000               lea r15, [rbp + 0x638]
14229256d: 4d89f8                       mov r8, r15
142292570: e85b6a71fe                   call 0x1409a8fd0
142292575: c785300500000d000000         mov dword ptr [rbp + 0x530], 0xd
14229257f: 8b4548                       mov eax, dword ptr [rbp + 0x48]
142292582: 488d8d34050000               lea rcx, [rbp + 0x534]
142292589: 894110                       mov dword ptr [rcx + 0x10], eax
14229258c: 0f104538                     movups xmm0, xmmword ptr [rbp + 0x38]
142292590: 0f1101                       movups xmmword ptr [rcx], xmm0
142292593: 488d4db0                     lea rcx, [rbp - 0x50]
142292597: 4c89e2                       mov rdx, r12
14229259a: 4d89f0                       mov r8, r14
14229259d: e87e6a71fe                   call 0x1409a9020
1422925a2: 84db                         test bl, bl
1422925a4: 4c89ad30060000               mov qword ptr [rbp + 0x630], r13
1422925ab: 0f8483010000                 je 0x142292734
1422925b1: 488d85e0060000               lea rax, [rbp + 0x6e0]
1422925b8: 488985e8030000               mov qword ptr [rbp + 0x3e8], rax
1422925bf: 488d05fa5ee3fd               lea rax, [rip - 0x21ca106]
1422925c6: 488985f0030000               mov qword ptr [rbp + 0x3f0], rax
1422925cd: 488d8da8060000               lea rcx, [rbp + 0x6a8]
1422925d4: 48898df8030000               mov qword ptr [rbp + 0x3f8], rcx
1422925db: 48898500040000               mov qword ptr [rbp + 0x400], rax
1422925e2: 488d053f91da00               lea rax, [rip + 0xda913f]
1422925e9: 48898530050000               mov qword ptr [rbp + 0x530], rax
1422925f0: 48c7853805000002000000       mov qword ptr [rbp + 0x538], 2
1422925fb: 4c89a540050000               mov qword ptr [rbp + 0x540], r12
142292602: 48c7854805000002000000       mov qword ptr [rbp + 0x548], 2
14229260d: 48c7855005000000000000       mov qword ptr [rbp + 0x550], 0
142292618: 4c89f9                       mov rcx, r15
14229261b: 4c89f2                       mov rdx, r14
14229261e: e86d79aefe                   call 0x140d79f90
142292623: f2410f104500                 movsd xmm0, qword ptr [r13]
142292629: f2410f104d08                 movsd xmm1, qword ptr [r13 + 8]
14229262f: f20f12d0                     movddup xmm2, xmm0
142292633: 410f59cd                     mulps xmm1, xmm13
142292637: f20f12d9                     movddup xmm3, xmm1
14229263b: 0f58da                       addps xmm3, xmm2
14229263e: 0f5cc1                       subps xmm0, xmm1
142292641: 0fc6c3e4                     shufps xmm0, xmm3, 0xe4
142292645: 0f1185f0030000               movups xmmword ptr [rbp + 0x3f0], xmm0
14229264c: c785ec03000000000000         mov dword ptr [rbp + 0x3ec], 0
142292656: c685e803000003               mov byte ptr [rbp + 0x3e8], 3
14229265d: c6851406000000               mov byte ptr [rbp + 0x614], 0
142292664: c685a406000001               mov byte ptr [rbp + 0x6a4], 1
14229266b: 4c89f1                       mov rcx, r14
14229266e: 4c89e2                       mov rdx, r12
142292671: 4c8d8514060000               lea r8, [rbp + 0x614]
142292678: e8936771fe                   call 0x1409a8e10
14229267d: 4c89e3                       mov rbx, r12
142292680: c685a406000000               mov byte ptr [rbp + 0x6a4], 0
142292687: 488b8d50060000               mov rcx, qword ptr [rbp + 0x650]
14229268e: 4c89fa                       mov rdx, r15
142292691: 4d89f0                       mov r8, r14
142292694: e887aa0200                   call 0x1422bd120
142292699: 488985e8030000               mov qword ptr [rbp + 0x3e8], rax
1422926a0: 488995f0030000               mov qword ptr [rbp + 0x3f0], rdx
1422926a7: 4889d9                       mov rcx, rbx
1422926aa: e8d1bf69fe                   call 0x14092e680
1422926af: 84c0                         test al, al
1422926b1: 448badec060000               mov r13d, dword ptr [rbp + 0x6ec]
1422926b8: 0f848b000000                 je 0x142292749
1422926be: 488b85a8060000               mov rax, qword ptr [rbp + 0x6a8]
1422926c5: 48898558060000               mov qword ptr [rbp + 0x658], rax
1422926cc: 8b85f8060000                 mov eax, dword ptr [rbp + 0x6f8]
1422926d2: 488b8d00070000               mov rcx, qword ptr [rbp + 0x700]
1422926d9: 488b95d0060000               mov rdx, qword ptr [rbp + 0x6d0]
1422926e0: 89048a                       mov dword ptr [rdx + rcx*4], eax
1422926e3: 488b85b0060000               mov rax, qword ptr [rbp + 0x6b0]
1422926ea: c6402001                     mov byte ptr [rax + 0x20], 1
1422926ee: b801000000                   mov eax, 1
1422926f3: 488b8d00070000               mov rcx, qword ptr [rbp + 0x700]
1422926fa: 488d0c49                     lea rcx, [rcx + rcx*2]
1422926fe: 488d0c8e                     lea rcx, [rsi + rcx*4]
142292702: 0fb611                       movzx edx, byte ptr [rcx]
142292705: 8895de060000                 mov byte ptr [rbp + 0x6de], dl
14229270b: 0fb65101                     movzx edx, byte ptr [rcx + 1]
14229270f: 8895df060000                 mov byte ptr [rbp + 0x6df], dl
142292715: 0fb75102                     movzx edx, word ptr [rcx + 2]
142292719: 66899526060000               mov word ptr [rbp + 0x626], dx
142292720: 8b5104                       mov edx, dword ptr [rcx + 4]
142292723: 8995fc050000                 mov dword ptr [rbp + 0x5fc], edx
142292729: 8b4908                       mov ecx, dword ptr [rcx + 8]
14229272c: 898d00060000                 mov dword ptr [rbp + 0x600], ecx
142292732: eb30                         jmp 0x142292764
142292734: 4531f6                       xor r14d, r14d
142292737: 4c89e3                       mov rbx, r12
14229273a: 4c89fe                       mov rsi, r15
14229273d: 448badec060000               mov r13d, dword ptr [rbp + 0x6ec]
142292744: e989000000                   jmp 0x1422927d2
142292749: 4883bd8006000000             cmp qword ptr [rbp + 0x680], 0
142292751: 440fb68517070000             movzx r8d, byte ptr [rbp + 0x717]
142292759: 0f84db060000                 je 0x142292e3a
14229275f: b801000000                   mov eax, 1
142292764: 48898580060000               mov qword ptr [rbp + 0x680], rax
14229276b: 4889d9                       mov rcx, rbx
14229276e: e87dbe69fe                   call 0x14092e5f0
142292773: 41b601                       mov r14b, 1
142292776: b101                         mov cl, 1
142292778: 84c0                         test al, al
14229277a: 488db538060000               lea rsi, [rbp + 0x638]
142292781: 7405                         je 0x142292788
142292783: 4189cd                       mov r13d, ecx
142292786: eb4a                         jmp 0x1422927d2
142292788: 488b8560060000               mov rax, qword ptr [rbp + 0x660]
14229278f: 8b8898000000                 mov ecx, dword ptr [rax + 0x98]
142292795: f30f10889c000000             movss xmm1, dword ptr [rax + 0x9c]
14229279d: e8eed26afe                   call 0x14093fa90
1422927a2: 84c0                         test al, al
1422927a4: 7429                         je 0x1422927cf
1422927a6: 488b8508070000               mov rax, qword ptr [rbp + 0x708]
1422927ad: 833801                       cmp dword ptr [rax], 1
1422927b0: 751d                         jne 0x1422927cf
1422927b2: 488b4008                     mov rax, qword ptr [rax + 8]
1422927b6: 483b85e0060000               cmp rax, qword ptr [rbp + 0x6e0]
1422927bd: 410f94c6                     sete r14b
1422927c1: 410fb6cd                     movzx ecx, r13b
1422927c5: b801000000                   mov eax, 1
1422927ca: 0f44c8                       cmove ecx, eax
1422927cd: ebb4                         jmp 0x142292783
1422927cf: 4531f6                       xor r14d, r14d
1422927d2: 488b8500070000               mov rax, qword ptr [rbp + 0x700]
1422927d9: 48c1e005                     shl rax, 5
1422927dd: 480185f8040000               add qword ptr [rbp + 0x4f8], rax
1422927e4: 4084ff                       test dil, dil
1422927e7: 0f8464010000                 je 0x142292951
1422927ed: 488d85e0060000               lea rax, [rbp + 0x6e0]
1422927f4: 488985e8030000               mov qword ptr [rbp + 0x3e8], rax
1422927fb: 488d05be5ce3fd               lea rax, [rip - 0x21ca342]
142292802: 488985f0030000               mov qword ptr [rbp + 0x3f0], rax
142292809: 488d8da8060000               lea rcx, [rbp + 0x6a8]
142292810: 48898df8030000               mov qword ptr [rbp + 0x3f8], rcx
142292817: 48898500040000               mov qword ptr [rbp + 0x400], rax
14229281e: 488d05338fda00               lea rax, [rip + 0xda8f33]
142292825: 48898530050000               mov qword ptr [rbp + 0x530], rax
14229282c: 48c7853805000002000000       mov qword ptr [rbp + 0x538], 2
142292837: 48899d40050000               mov qword ptr [rbp + 0x540], rbx
14229283e: 48c7854805000002000000       mov qword ptr [rbp + 0x548], 2
142292849: 48c7855005000000000000       mov qword ptr [rbp + 0x550], 0
142292854: 4889f1                       mov rcx, rsi
142292857: 4889f7                       mov rdi, rsi
14229285a: 488db530050000               lea rsi, [rbp + 0x530]
142292861: 4889f2                       mov rdx, rsi
142292864: e82777aefe                   call 0x140d79f90
142292869: 488b8530060000               mov rax, qword ptr [rbp + 0x630]
142292870: f20f1000                     movsd xmm0, qword ptr [rax]
142292874: f20f104808                   movsd xmm1, qword ptr [rax + 8]
142292879: f20f12d0                     movddup xmm2, xmm0
14229287d: 410f59cd                     mulps xmm1, xmm13
142292881: f20f12d9                     movddup xmm3, xmm1
142292885: 0f58da                       addps xmm3, xmm2
142292888: 0f5cc1                       subps xmm0, xmm1
14229288b: 0fc6c3e4                     shufps xmm0, xmm3, 0xe4
14229288f: 0f1185f0030000               movups xmmword ptr [rbp + 0x3f0], xmm0
142292896: c785ec03000000000000         mov dword ptr [rbp + 0x3ec], 0
1422928a0: c685e803000003               mov byte ptr [rbp + 0x3e8], 3
1422928a7: c6851406000000               mov byte ptr [rbp + 0x614], 0
1422928ae: c685a306000001               mov byte ptr [rbp + 0x6a3], 1
1422928b5: 4889f1                       mov rcx, rsi
1422928b8: 4889da                       mov rdx, rbx
1422928bb: 4c8d8514060000               lea r8, [rbp + 0x614]
1422928c2: e8496571fe                   call 0x1409a8e10
1422928c7: c685a306000000               mov byte ptr [rbp + 0x6a3], 0
1422928ce: 488b8d50060000               mov rcx, qword ptr [rbp + 0x650]
1422928d5: 4889fa                       mov rdx, rdi
1422928d8: 4989f0                       mov r8, rsi
1422928db: e840a80200                   call 0x1422bd120
1422928e0: 488985e8030000               mov qword ptr [rbp + 0x3e8], rax
1422928e7: 488995f0030000               mov qword ptr [rbp + 0x3f0], rdx
1422928ee: 488b8508070000               mov rax, qword ptr [rbp + 0x708]
1422928f5: 0fb69889000000               movzx ebx, byte ptr [rax + 0x89]
1422928fc: 488d8de8030000               lea rcx, [rbp + 0x3e8]
142292903: e878bd69fe                   call 0x14092e680
142292908: 84c0                         test al, al
14229290a: 0f84ce010000                 je 0x142292ade
142292910: 488b85a8060000               mov rax, qword ptr [rbp + 0x6a8]
142292917: 48898558060000               mov qword ptr [rbp + 0x658], rax
14229291e: 8b85f8060000                 mov eax, dword ptr [rbp + 0x6f8]
142292924: 488b8d00070000               mov rcx, qword ptr [rbp + 0x700]
14229292b: 488b95d0060000               mov rdx, qword ptr [rbp + 0x6d0]
142292932: 89048a                       mov dword ptr [rdx + rcx*4], eax
142292935: 488b85b0060000               mov rax, qword ptr [rbp + 0x6b0]
14229293c: c6402001                     mov byte ptr [rax + 0x20], 1
142292940: 80f301                       xor bl, 1
142292943: b001                         mov al, 1
142292945: 48898530030000               mov qword ptr [rbp + 0x330], rax
14229294c: e99b010000                   jmp 0x142292aec
142292951: 4589ec                       mov r12d, r13d
142292954: 488dbd30050000               lea rdi, [rbp + 0x530]
14229295b: 4c8b85b8060000               mov r8, qword ptr [rbp + 0x6b8]
142292962: 488b8dc0060000               mov rcx, qword ptr [rbp + 0x6c0]
142292969: 488b85a8030000               mov rax, qword ptr [rbp + 0x3a8]
142292970: 84c0                         test al, al
142292972: 0f842f030000                 je 0x142292ca7
142292978: 488d85e0060000               lea rax, [rbp + 0x6e0]
14229297f: 488985e8030000               mov qword ptr [rbp + 0x3e8], rax
142292986: 488d05335be3fd               lea rax, [rip - 0x21ca4cd]
14229298d: 488985f0030000               mov qword ptr [rbp + 0x3f0], rax
142292994: 488d8da8060000               lea rcx, [rbp + 0x6a8]
14229299b: 48898df8030000               mov qword ptr [rbp + 0x3f8], rcx
1422929a2: 48898500040000               mov qword ptr [rbp + 0x400], rax
1422929a9: 488d05188eda00               lea rax, [rip + 0xda8e18]
1422929b0: 48898530050000               mov qword ptr [rbp + 0x530], rax
1422929b7: 48c7853805000002000000       mov qword ptr [rbp + 0x538], 2
1422929c2: 48899d40050000               mov qword ptr [rbp + 0x540], rbx
1422929c9: 48c7854805000002000000       mov qword ptr [rbp + 0x548], 2
1422929d4: 48c7855005000000000000       mov qword ptr [rbp + 0x550], 0
1422929df: 4889f1                       mov rcx, rsi
1422929e2: 4889fa                       mov rdx, rdi
1422929e5: e8a675aefe                   call 0x140d79f90
1422929ea: 488b8530060000               mov rax, qword ptr [rbp + 0x630]
1422929f1: f20f1000                     movsd xmm0, qword ptr [rax]
1422929f5: f20f104808                   movsd xmm1, qword ptr [rax + 8]
1422929fa: f20f12d0                     movddup xmm2, xmm0
1422929fe: 410f59cd                     mulps xmm1, xmm13
142292a02: f20f12d9                     movddup xmm3, xmm1
142292a06: 0f58da                       addps xmm3, xmm2
142292a09: 0f5cc1                       subps xmm0, xmm1
142292a0c: 0fc6c3e4                     shufps xmm0, xmm3, 0xe4
142292a10: 0f1185f0030000               movups xmmword ptr [rbp + 0x3f0], xmm0
142292a17: c785ec03000000000000         mov dword ptr [rbp + 0x3ec], 0
142292a21: c685e803000003               mov byte ptr [rbp + 0x3e8], 3
142292a28: c6851406000000               mov byte ptr [rbp + 0x614], 0
142292a2f: c685a206000001               mov byte ptr [rbp + 0x6a2], 1
142292a36: 4889f9                       mov rcx, rdi
142292a39: 4889da                       mov rdx, rbx
142292a3c: 4c8d8514060000               lea r8, [rbp + 0x614]
142292a43: e8c86371fe                   call 0x1409a8e10
142292a48: 4c8bb550060000               mov r14, qword ptr [rbp + 0x650]
142292a4f: 4c8bad80060000               mov r13, qword ptr [rbp + 0x680]
142292a56: c685a206000000               mov byte ptr [rbp + 0x6a2], 0
142292a5d: 4c89f1                       mov rcx, r14
142292a60: 4889f2                       mov rdx, rsi
142292a63: 4989f8                       mov r8, rdi
142292a66: e8b5a60200                   call 0x1422bd120
142292a6b: 488985e8030000               mov qword ptr [rbp + 0x3e8], rax
142292a72: 488995f0030000               mov qword ptr [rbp + 0x3f0], rdx
142292a79: 4889d9                       mov rcx, rbx
142292a7c: e8ffbb69fe                   call 0x14092e680
142292a81: 84c0                         test al, al
142292a83: 0f842e010000                 je 0x142292bb7
142292a89: 488b85a8060000               mov rax, qword ptr [rbp + 0x6a8]
142292a90: 48898558060000               mov qword ptr [rbp + 0x658], rax
142292a97: 8b85f8060000                 mov eax, dword ptr [rbp + 0x6f8]
142292a9d: 488b8d00070000               mov rcx, qword ptr [rbp + 0x700]
142292aa4: 488b95d0060000               mov rdx, qword ptr [rbp + 0x6d0]
142292aab: 89048a                       mov dword ptr [rdx + rcx*4], eax
142292aae: 488b85b0060000               mov rax, qword ptr [rbp + 0x6b0]
142292ab5: c6402001                     mov byte ptr [rax + 0x20], 1
142292ab9: b801000000                   mov eax, 1
142292abe: 488b8d00070000               mov rcx, qword ptr [rbp + 0x700]
142292ac5: 488b95e8040000               mov rdx, qword ptr [rbp + 0x4e8]
142292acc: 0fb60c0a                     movzx ecx, byte ptr [rdx + rcx]
142292ad0: 888df6060000                 mov byte ptr [rbp + 0x6f6], cl
142292ad6: 4989c5                       mov r13, rax
142292ad9: e98e040000                   jmp 0x142292f6c
142292ade: 4883bd8006000000             cmp qword ptr [rbp + 0x680], 0
142292ae6: 0f84b8030000                 je 0x142292ea4
142292aec: b801000000                   mov eax, 1
142292af1: 48898580060000               mov qword ptr [rbp + 0x680], rax
142292af8: 488d8de8030000               lea rcx, [rbp + 0x3e8]
142292aff: e8ecba69fe                   call 0x14092e5f0
142292b04: 40b601                       mov sil, 1
142292b07: 41b401                       mov r12b, 1
142292b0a: 84c0                         test al, al
142292b0c: 7453                         je 0x142292b61
142292b0e: 0fb605f4203a01               movzx eax, byte ptr [rip + 0x13a20f4]
142292b15: 84db                         test bl, bl
142292b17: 0f84ca000000                 je 0x142292be7
142292b1d: b91f000000                   mov ecx, 0x1f
142292b22: ba01000000                   mov edx, 1
142292b27: e8e41038fe                   call 0x140613c10
142292b2c: 4885c0                       test rax, rax
142292b2f: 488d9de8030000               lea rbx, [rbp + 0x3e8]
142292b36: 0f8426410000                 je 0x142296c62
142292b3c: 4889c7                       mov rdi, rax
142292b3f: 4189f7                       mov r15d, esi
142292b42: 0f10055f8cda00               movups xmm0, xmmword ptr [rip + 0xda8c5f]
142292b49: 0f11400f                     movups xmmword ptr [rax + 0xf], xmm0
142292b4d: 0f1005458cda00               movups xmm0, xmmword ptr [rip + 0xda8c45]
142292b54: 0f1100                       movups xmmword ptr [rax], xmm0
142292b57: be1f000000                   mov esi, 0x1f
142292b5c: e9c9000000                   jmp 0x142292c2a
142292b61: 488b8560060000               mov rax, qword ptr [rbp + 0x660]
142292b68: 8b8898000000                 mov ecx, dword ptr [rax + 0x98]
142292b6e: f30f10889c000000             movss xmm1, dword ptr [rax + 0x9c]
142292b76: e815cf6afe                   call 0x14093fa90
142292b7b: 84c0                         test al, al
142292b7d: 7454                         je 0x142292bd3
142292b7f: 488b8508070000               mov rax, qword ptr [rbp + 0x708]
142292b86: 833801                       cmp dword ptr [rax], 1
142292b89: 7548                         jne 0x142292bd3
142292b8b: 488b4008                     mov rax, qword ptr [rax + 8]
142292b8f: 483b85e0060000               cmp rax, qword ptr [rbp + 0x6e0]
142292b96: 400f94c6                     sete sil
142292b9a: 450fb6e5                     movzx r12d, r13b
142292b9e: b801000000                   mov eax, 1
142292ba3: 440f44e0                     cmove r12d, eax
142292ba7: 0fb6055b203a01               movzx eax, byte ptr [rip + 0x13a205b]
142292bae: 84db                         test bl, bl
142292bb0: 7435                         je 0x142292be7
142292bb2: e966ffffff                   jmp 0x142292b1d
142292bb7: 4d85ed                       test r13, r13
142292bba: 440fb68517070000             movzx r8d, byte ptr [rbp + 0x717]
142292bc2: 0f8445030000                 je 0x142292f0d
142292bc8: 41bd01000000                 mov r13d, 1
142292bce: e999030000                   jmp 0x142292f6c
142292bd3: 31f6                         xor esi, esi
142292bd5: 4589ec                       mov r12d, r13d
142292bd8: 0fb6052a203a01               movzx eax, byte ptr [rip + 0x13a202a]
142292bdf: 84db                         test bl, bl
142292be1: 0f8536ffffff                 jne 0x142292b1d
142292be7: b921000000                   mov ecx, 0x21
142292bec: ba01000000                   mov edx, 1
142292bf1: e81a1038fe                   call 0x140613c10
142292bf6: 4885c0                       test rax, rax
142292bf9: 488d9de8030000               lea rbx, [rbp + 0x3e8]
142292c00: 0f84c7400000                 je 0x142296ccd
142292c06: 4889c7                       mov rdi, rax
142292c09: 4189f7                       mov r15d, esi
142292c0c: 0f1005758bda00               movups xmm0, xmmword ptr [rip + 0xda8b75]
142292c13: 0f114010                     movups xmmword ptr [rax + 0x10], xmm0
142292c17: 0f10055a8bda00               movups xmm0, xmmword ptr [rip + 0xda8b5a]
142292c1e: 0f1100                       movups xmmword ptr [rax], xmm0
142292c21: c6402065                     mov byte ptr [rax + 0x20], 0x65
142292c25: be21000000                   mov esi, 0x21
142292c2a: 4c8b85b8060000               mov r8, qword ptr [rbp + 0x6b8]
142292c31: 488b8dc0060000               mov rcx, qword ptr [rbp + 0x6c0]
142292c38: 8b85f8060000                 mov eax, dword ptr [rbp + 0x6f8]
142292c3e: 488b9500070000               mov rdx, qword ptr [rbp + 0x700]
142292c45: 4c8b8dc0030000               mov r9, qword ptr [rbp + 0x3c0]
142292c4c: 41890491                     mov dword ptr [r9 + rdx*4], eax
142292c50: 4c8bb5f8040000               mov r14, qword ptr [rbp + 0x4f8]
142292c57: 498b16                       mov rdx, qword ptr [r14]
142292c5a: 4885d2                       test rdx, rdx
142292c5d: 741d                         je 0x142292c7c
142292c5f: 498b4e08                     mov rcx, qword ptr [r14 + 8]
142292c63: 41b801000000                 mov r8d, 1
142292c69: e8b20f38fe                   call 0x140613c20
142292c6e: 488b8dc0060000               mov rcx, qword ptr [rbp + 0x6c0]
142292c75: 4c8b85b8060000               mov r8, qword ptr [rbp + 0x6b8]
142292c7c: 498936                       mov qword ptr [r14], rsi
142292c7f: 49897e08                     mov qword ptr [r14 + 8], rdi
142292c83: 49897610                     mov qword ptr [r14 + 0x10], rsi
142292c87: 488dbd30050000               lea rdi, [rbp + 0x530]
142292c8e: 488db538060000               lea rsi, [rbp + 0x638]
142292c95: 4589fe                       mov r14d, r15d
142292c98: 488b85a8030000               mov rax, qword ptr [rbp + 0x3a8]
142292c9f: 84c0                         test al, al
142292ca1: 0f85d1fcffff                 jne 0x142292978
142292ca7: 4c8bad80060000               mov r13, qword ptr [rbp + 0x680]
142292cae: 488b85a0030000               mov rax, qword ptr [rbp + 0x3a0]
142292cb5: 84c0                         test al, al
142292cb7: 0f842d030000                 je 0x142292fea
142292cbd: 488d85e0060000               lea rax, [rbp + 0x6e0]
142292cc4: 488985e8030000               mov qword ptr [rbp + 0x3e8], rax
142292ccb: 488d05ee57e3fd               lea rax, [rip - 0x21ca812]
142292cd2: 488985f0030000               mov qword ptr [rbp + 0x3f0], rax
142292cd9: 488d8da8060000               lea rcx, [rbp + 0x6a8]
142292ce0: 48898df8030000               mov qword ptr [rbp + 0x3f8], rcx
142292ce7: 48898500040000               mov qword ptr [rbp + 0x400], rax
142292cee: 488d050b8bda00               lea rax, [rip + 0xda8b0b]
142292cf5: 48898530050000               mov qword ptr [rbp + 0x530], rax
142292cfc: 48c7853805000002000000       mov qword ptr [rbp + 0x538], 2
142292d07: 48899d40050000               mov qword ptr [rbp + 0x540], rbx
142292d0e: 48c7854805000002000000       mov qword ptr [rbp + 0x548], 2
142292d19: 48c7855005000000000000       mov qword ptr [rbp + 0x550], 0
142292d24: 4889f1                       mov rcx, rsi
142292d27: 4889fa                       mov rdx, rdi
142292d2a: e86172aefe                   call 0x140d79f90
142292d2f: 488b8530060000               mov rax, qword ptr [rbp + 0x630]
142292d36: f20f1000                     movsd xmm0, qword ptr [rax]
142292d3a: f20f104808                   movsd xmm1, qword ptr [rax + 8]
142292d3f: f20f12d0                     movddup xmm2, xmm0
142292d43: 410f59cd                     mulps xmm1, xmm13
142292d47: f20f12d9                     movddup xmm3, xmm1
142292d4b: 0f58da                       addps xmm3, xmm2
142292d4e: 0f5cc1                       subps xmm0, xmm1
142292d51: 0fc6c3e4                     shufps xmm0, xmm3, 0xe4
142292d55: 0f1185f0030000               movups xmmword ptr [rbp + 0x3f0], xmm0
142292d5c: c785ec03000000000000         mov dword ptr [rbp + 0x3ec], 0
142292d66: c685e803000003               mov byte ptr [rbp + 0x3e8], 3
142292d6d: c6851406000000               mov byte ptr [rbp + 0x614], 0
142292d74: c685a106000001               mov byte ptr [rbp + 0x6a1], 1
142292d7b: 4889f9                       mov rcx, rdi
142292d7e: 4889da                       mov rdx, rbx
142292d81: 4c8d8514060000               lea r8, [rbp + 0x614]
142292d88: e8836071fe                   call 0x1409a8e10
142292d8d: c685a106000000               mov byte ptr [rbp + 0x6a1], 0
142292d94: 488b8d50060000               mov rcx, qword ptr [rbp + 0x650]
142292d9b: 4889f2                       mov rdx, rsi
142292d9e: 4989f8                       mov r8, rdi
142292da1: e87aa30200                   call 0x1422bd120
142292da6: 488985e8030000               mov qword ptr [rbp + 0x3e8], rax
142292dad: 488995f0030000               mov qword ptr [rbp + 0x3f0], rdx
142292db4: 4889d9                       mov rcx, rbx
142292db7: e8c4b869fe                   call 0x14092e680
142292dbc: 84c0                         test al, al
142292dbe: 745e                         je 0x142292e1e
142292dc0: 488b85a8060000               mov rax, qword ptr [rbp + 0x6a8]
142292dc7: 48898558060000               mov qword ptr [rbp + 0x658], rax
142292dce: 8b85f8060000                 mov eax, dword ptr [rbp + 0x6f8]
142292dd4: 488b8d00070000               mov rcx, qword ptr [rbp + 0x700]
142292ddb: 488b95d0060000               mov rdx, qword ptr [rbp + 0x6d0]
142292de2: 89048a                       mov dword ptr [rdx + rcx*4], eax
142292de5: 488b85b0060000               mov rax, qword ptr [rbp + 0x6b0]
142292dec: c6402001                     mov byte ptr [rax + 0x20], 1
142292df0: b801000000                   mov eax, 1
142292df5: 488b8d00070000               mov rcx, qword ptr [rbp + 0x700]
142292dfc: 488b95e0040000               mov rdx, qword ptr [rbp + 0x4e0]
142292e03: 8b0c8a                       mov ecx, dword ptr [rdx + rcx*4]
142292e06: 898d04060000                 mov dword ptr [rbp + 0x604], ecx
142292e0c: c785a404000001000000         mov dword ptr [rbp + 0x4a4], 1
142292e16: 4989c5                       mov r13, rax
142292e19: e9a5030000                   jmp 0x1422931c3
142292e1e: 4d85ed                       test r13, r13
142292e21: 440fb68517070000             movzx r8d, byte ptr [rbp + 0x717]
142292e29: 0f8435030000                 je 0x142293164
142292e2f: 41bd01000000                 mov r13d, 1
142292e35: e989030000                   jmp 0x1422931c3
142292e3a: 80bdcc06000000               cmp byte ptr [rbp + 0x6cc], 0
142292e41: 7442                         je 0x142292e85
142292e43: 488b8508070000               mov rax, qword ptr [rbp + 0x708]
142292e4a: f60001                       test byte ptr [rax], 1
142292e4d: 7436                         je 0x142292e85
142292e4f: 488b8508070000               mov rax, qword ptr [rbp + 0x708]
142292e56: 488b4008                     mov rax, qword ptr [rax + 8]
142292e5a: 483985e0060000               cmp qword ptr [rbp + 0x6e0], rax
142292e61: 7522                         jne 0x142292e85
142292e63: 8b85f8060000                 mov eax, dword ptr [rbp + 0x6f8]
142292e69: 488b8d00070000               mov rcx, qword ptr [rbp + 0x700]
142292e70: 488b95d0060000               mov rdx, qword ptr [rbp + 0x6d0]
142292e77: 89048a                       mov dword ptr [rdx + rcx*4], eax
142292e7a: 488b85b0060000               mov rax, qword ptr [rbp + 0x6b0]
142292e81: c6402001                     mov byte ptr [rax + 0x20], 1
142292e85: b800000000                   mov eax, 0
142292e8a: b900000000                   mov ecx, 0
142292e8f: 48898d80060000               mov qword ptr [rbp + 0x680], rcx
142292e96: 4584c0                       test r8b, r8b
142292e99: 0f8554f8ffff                 jne 0x1422926f3
142292e9f: e9c7f8ffff                   jmp 0x14229276b
142292ea4: 80bdcc06000000               cmp byte ptr [rbp + 0x6cc], 0
142292eab: 488b8508070000               mov rax, qword ptr [rbp + 0x708]
142292eb2: 7434                         je 0x142292ee8
142292eb4: f60001                       test byte ptr [rax], 1
142292eb7: 742f                         je 0x142292ee8
142292eb9: 488b4008                     mov rax, qword ptr [rax + 8]
142292ebd: 483985e0060000               cmp qword ptr [rbp + 0x6e0], rax
142292ec4: 7522                         jne 0x142292ee8
142292ec6: 8b85f8060000                 mov eax, dword ptr [rbp + 0x6f8]
142292ecc: 488b8d00070000               mov rcx, qword ptr [rbp + 0x700]
142292ed3: 488b95d0060000               mov rdx, qword ptr [rbp + 0x6d0]
142292eda: 89048a                       mov dword ptr [rdx + rcx*4], eax
142292edd: 488b85b0060000               mov rax, qword ptr [rbp + 0x6b0]
142292ee4: c6402001                     mov byte ptr [rax + 0x20], 1
142292ee8: 80bd1707000000               cmp byte ptr [rbp + 0x717], 0
142292eef: 740c                         je 0x142292efd
142292ef1: 80f301                       xor bl, 1
142292ef4: b001                         mov al, 1
142292ef6: 48898530030000               mov qword ptr [rbp + 0x330], rax
142292efd: 48c7858006000000000000       mov qword ptr [rbp + 0x680], 0
142292f08: e9ebfbffff                   jmp 0x142292af8
142292f0d: 80bdcc06000000               cmp byte ptr [rbp + 0x6cc], 0
142292f14: 7442                         je 0x142292f58
142292f16: 488b8508070000               mov rax, qword ptr [rbp + 0x708]
142292f1d: f60001                       test byte ptr [rax], 1
142292f20: 7436                         je 0x142292f58
142292f22: 488b8508070000               mov rax, qword ptr [rbp + 0x708]
142292f29: 488b4008                     mov rax, qword ptr [rax + 8]
142292f2d: 483985e0060000               cmp qword ptr [rbp + 0x6e0], rax
142292f34: 7522                         jne 0x142292f58
142292f36: 8b85f8060000                 mov eax, dword ptr [rbp + 0x6f8]
142292f3c: 488b8d00070000               mov rcx, qword ptr [rbp + 0x700]
142292f43: 488b95d0060000               mov rdx, qword ptr [rbp + 0x6d0]
142292f4a: 89048a                       mov dword ptr [rdx + rcx*4], eax
142292f4d: 488b85b0060000               mov rax, qword ptr [rbp + 0x6b0]
142292f54: c6402001                     mov byte ptr [rax + 0x20], 1
142292f58: b800000000                   mov eax, 0
142292f5d: 41bd00000000                 mov r13d, 0
142292f63: 4584c0                       test r8b, r8b
142292f66: 0f8552fbffff                 jne 0x142292abe
142292f6c: 4889d9                       mov rcx, rbx
142292f6f: e87cb669fe                   call 0x14092e5f0
142292f74: 41b601                       mov r14b, 1
142292f77: b101                         mov cl, 1
142292f79: 84c0                         test al, al
142292f7b: 754d                         jne 0x142292fca
142292f7d: 488b8560060000               mov rax, qword ptr [rbp + 0x660]
142292f84: 8b8898000000                 mov ecx, dword ptr [rax + 0x98]
142292f8a: f30f10889c000000             movss xmm1, dword ptr [rax + 0x9c]
142292f92: e8f9ca6afe                   call 0x14093fa90
142292f97: 84c0                         test al, al
142292f99: 7429                         je 0x142292fc4
142292f9b: 488b8508070000               mov rax, qword ptr [rbp + 0x708]
142292fa2: 833801                       cmp dword ptr [rax], 1
142292fa5: 751d                         jne 0x142292fc4
142292fa7: 488b4008                     mov rax, qword ptr [rax + 8]
142292fab: 483b85e0060000               cmp rax, qword ptr [rbp + 0x6e0]
142292fb2: 410f94c6                     sete r14b
142292fb6: 410fb6cc                     movzx ecx, r12b
142292fba: b801000000                   mov eax, 1
142292fbf: 0f44c8                       cmove ecx, eax
142292fc2: eb06                         jmp 0x142292fca
142292fc4: 4531f6                       xor r14d, r14d
142292fc7: 4489e1                       mov ecx, r12d
142292fca: 4189cc                       mov r12d, ecx
142292fcd: 4c8b85b8060000               mov r8, qword ptr [rbp + 0x6b8]
142292fd4: 488b8dc0060000               mov rcx, qword ptr [rbp + 0x6c0]
142292fdb: 488b85a0030000               mov rax, qword ptr [rbp + 0x3a0]
142292fe2: 84c0                         test al, al
142292fe4: 0f85d3fcffff                 jne 0x142292cbd
142292fea: 80bda004000000               cmp byte ptr [rbp + 0x4a0], 0
142292ff1: 0f8451020000                 je 0x142293248
142292ff7: 488d85e0060000               lea rax, [rbp + 0x6e0]
142292ffe: 488985e8030000               mov qword ptr [rbp + 0x3e8], rax
142293005: 488d05b454e3fd               lea rax, [rip - 0x21cab4c]
14229300c: 488985f0030000               mov qword ptr [rbp + 0x3f0], rax
142293013: 488d8da8060000               lea rcx, [rbp + 0x6a8]
14229301a: 48898df8030000               mov qword ptr [rbp + 0x3f8], rcx
142293021: 48898500040000               mov qword ptr [rbp + 0x400], rax
142293028: 488d050188da00               lea rax, [rip + 0xda8801]
14229302f: 48898530050000               mov qword ptr [rbp + 0x530], rax
142293036: 48c7853805000002000000       mov qword ptr [rbp + 0x538], 2
142293041: 48899d40050000               mov qword ptr [rbp + 0x540], rbx
142293048: 48c7854805000002000000       mov qword ptr [rbp + 0x548], 2
142293053: 48c7855005000000000000       mov qword ptr [rbp + 0x550], 0
14229305e: 4889f1                       mov rcx, rsi
142293061: 4889fa                       mov rdx, rdi
142293064: e8276faefe                   call 0x140d79f90
142293069: 488b8530060000               mov rax, qword ptr [rbp + 0x630]
142293070: f20f1000                     movsd xmm0, qword ptr [rax]
142293074: f20f104808                   movsd xmm1, qword ptr [rax + 8]
142293079: f20f12d0                     movddup xmm2, xmm0
14229307d: 410f59cd                     mulps xmm1, xmm13
142293081: f20f12d9                     movddup xmm3, xmm1
142293085: 0f58da                       addps xmm3, xmm2
142293088: 0f5cc1                       subps xmm0, xmm1
14229308b: 0fc6c3e4                     shufps xmm0, xmm3, 0xe4
14229308f: 0f1185f0030000               movups xmmword ptr [rbp + 0x3f0], xmm0
142293096: c785ec03000000000000         mov dword ptr [rbp + 0x3ec], 0
1422930a0: c685e803000003               mov byte ptr [rbp + 0x3e8], 3
1422930a7: c6851406000000               mov byte ptr [rbp + 0x614], 0
1422930ae: c685a006000001               mov byte ptr [rbp + 0x6a0], 1
1422930b5: 4889f9                       mov rcx, rdi
1422930b8: 4889da                       mov rdx, rbx
1422930bb: 4c8d8514060000               lea r8, [rbp + 0x614]
1422930c2: e8495d71fe                   call 0x1409a8e10
1422930c7: c685a006000000               mov byte ptr [rbp + 0x6a0], 0
1422930ce: 488b8d50060000               mov rcx, qword ptr [rbp + 0x650]
1422930d5: 4889f2                       mov rdx, rsi
1422930d8: 4989f8                       mov r8, rdi
1422930db: e840a00200                   call 0x1422bd120
1422930e0: 488985e8030000               mov qword ptr [rbp + 0x3e8], rax
1422930e7: 488995f0030000               mov qword ptr [rbp + 0x3f0], rdx
1422930ee: 488b8508070000               mov rax, qword ptr [rbp + 0x708]
1422930f5: 0fb6b086000000               movzx esi, byte ptr [rax + 0x86]
1422930fc: 4889d9                       mov rcx, rbx
1422930ff: e87cb569fe                   call 0x14092e680
142293104: 84c0                         test al, al
142293106: 7448                         je 0x142293150
142293108: 488b85a8060000               mov rax, qword ptr [rbp + 0x6a8]
14229310f: 48898558060000               mov qword ptr [rbp + 0x658], rax
142293116: 8b85f8060000                 mov eax, dword ptr [rbp + 0x6f8]
14229311c: 488b8d00070000               mov rcx, qword ptr [rbp + 0x700]
142293123: 488b95d0060000               mov rdx, qword ptr [rbp + 0x6d0]
14229312a: 89048a                       mov dword ptr [rdx + rcx*4], eax
14229312d: 488b85b0060000               mov rax, qword ptr [rbp + 0x6b0]
142293134: c6402001                     mov byte ptr [rax + 0x20], 1
142293138: 4080f601                     xor sil, 1
14229313c: b001                         mov al, 1
14229313e: 48898540030000               mov qword ptr [rbp + 0x340], rax
142293145: 41bd01000000                 mov r13d, 1
14229314b: e9f7020000                   jmp 0x142293447
142293150: 4d85ed                       test r13, r13
142293153: 0f8491020000                 je 0x1422933ea
142293159: 41bd01000000                 mov r13d, 1
14229315f: e9e3020000                   jmp 0x142293447
142293164: 80bdcc06000000               cmp byte ptr [rbp + 0x6cc], 0
14229316b: 7442                         je 0x1422931af
14229316d: 488b8508070000               mov rax, qword ptr [rbp + 0x708]
142293174: f60001                       test byte ptr [rax], 1
142293177: 7436                         je 0x1422931af
142293179: 488b8508070000               mov rax, qword ptr [rbp + 0x708]
142293180: 488b4008                     mov rax, qword ptr [rax + 8]
142293184: 483985e0060000               cmp qword ptr [rbp + 0x6e0], rax
14229318b: 7522                         jne 0x1422931af
14229318d: 8b85f8060000                 mov eax, dword ptr [rbp + 0x6f8]
142293193: 488b8d00070000               mov rcx, qword ptr [rbp + 0x700]
14229319a: 488b95d0060000               mov rdx, qword ptr [rbp + 0x6d0]
1422931a1: 89048a                       mov dword ptr [rdx + rcx*4], eax
1422931a4: 488b85b0060000               mov rax, qword ptr [rbp + 0x6b0]
1422931ab: c6402001                     mov byte ptr [rax + 0x20], 1
1422931af: b800000000                   mov eax, 0
1422931b4: 41bd00000000                 mov r13d, 0
1422931ba: 4584c0                       test r8b, r8b
1422931bd: 0f8532fcffff                 jne 0x142292df5
1422931c3: 4889d9                       mov rcx, rbx
1422931c6: e825b469fe                   call 0x14092e5f0
1422931cb: b101                         mov cl, 1
1422931cd: 48898dc0010000               mov qword ptr [rbp + 0x1c0], rcx
1422931d4: 41b601                       mov r14b, 1
1422931d7: b101                         mov cl, 1
1422931d9: 84c0                         test al, al
1422931db: 754d                         jne 0x14229322a
1422931dd: 488b8560060000               mov rax, qword ptr [rbp + 0x660]
1422931e4: 8b8898000000                 mov ecx, dword ptr [rax + 0x98]
1422931ea: f30f10889c000000             movss xmm1, dword ptr [rax + 0x9c]
1422931f2: e899c86afe                   call 0x14093fa90
1422931f7: 84c0                         test al, al
1422931f9: 7429                         je 0x142293224
1422931fb: 488b8508070000               mov rax, qword ptr [rbp + 0x708]
142293202: 833801                       cmp dword ptr [rax], 1
142293205: 751d                         jne 0x142293224
142293207: 488b4008                     mov rax, qword ptr [rax + 8]
14229320b: 483b85e0060000               cmp rax, qword ptr [rbp + 0x6e0]
142293212: 410f94c6                     sete r14b
142293216: 410fb6cc                     movzx ecx, r12b
14229321a: b801000000                   mov eax, 1
14229321f: 0f44c8                       cmove ecx, eax
142293222: eb06                         jmp 0x14229322a
142293224: 4531f6                       xor r14d, r14d
142293227: 4489e1                       mov ecx, r12d
14229322a: 4189cc                       mov r12d, ecx
14229322d: 4c8b85b8060000               mov r8, qword ptr [rbp + 0x6b8]
142293234: 488b8dc0060000               mov rcx, qword ptr [rbp + 0x6c0]
14229323b: 80bda004000000               cmp byte ptr [rbp + 0x4a0], 0
142293242: 0f85affdffff                 jne 0x142292ff7
142293248: 4589e7                       mov r15d, r12d
14229324b: 80bd9c04000000               cmp byte ptr [rbp + 0x49c], 0
142293252: 0f845d030000                 je 0x1422935b5
142293258: 4c89eb                       mov rbx, r13
14229325b: 488d85e0060000               lea rax, [rbp + 0x6e0]
142293262: 488985e8030000               mov qword ptr [rbp + 0x3e8], rax
142293269: 488d055052e3fd               lea rax, [rip - 0x21cadb0]
142293270: 488985f0030000               mov qword ptr [rbp + 0x3f0], rax
142293277: 488d8da8060000               lea rcx, [rbp + 0x6a8]
14229327e: 48898df8030000               mov qword ptr [rbp + 0x3f8], rcx
142293285: 48898500040000               mov qword ptr [rbp + 0x400], rax
14229328c: 488d050586da00               lea rax, [rip + 0xda8605]
142293293: 48898530050000               mov qword ptr [rbp + 0x530], rax
14229329a: 48c7853805000002000000       mov qword ptr [rbp + 0x538], 2
1422932a5: 488db5e8030000               lea rsi, [rbp + 0x3e8]
1422932ac: 4889b540050000               mov qword ptr [rbp + 0x540], rsi
1422932b3: 48c7854805000002000000       mov qword ptr [rbp + 0x548], 2
1422932be: 48c7855005000000000000       mov qword ptr [rbp + 0x550], 0
1422932c9: 4c8dad38060000               lea r13, [rbp + 0x638]
1422932d0: 4c89e9                       mov rcx, r13
1422932d3: 4889fa                       mov rdx, rdi
1422932d6: e8b56caefe                   call 0x140d79f90
1422932db: 488b8530060000               mov rax, qword ptr [rbp + 0x630]
1422932e2: f20f1000                     movsd xmm0, qword ptr [rax]
1422932e6: f20f104808                   movsd xmm1, qword ptr [rax + 8]
1422932eb: f20f12d0                     movddup xmm2, xmm0
1422932ef: 410f59cd                     mulps xmm1, xmm13
1422932f3: f20f12d9                     movddup xmm3, xmm1
1422932f7: 0f58da                       addps xmm3, xmm2
1422932fa: 0f5cc1                       subps xmm0, xmm1
1422932fd: 0fc6c3e4                     shufps xmm0, xmm3, 0xe4
142293301: 0f1185f0030000               movups xmmword ptr [rbp + 0x3f0], xmm0
142293308: c785ec03000000000000         mov dword ptr [rbp + 0x3ec], 0
142293312: c685e803000003               mov byte ptr [rbp + 0x3e8], 3
142293319: c6851406000000               mov byte ptr [rbp + 0x614], 0
142293320: c6859f06000001               mov byte ptr [rbp + 0x69f], 1
142293327: 4889f9                       mov rcx, rdi
14229332a: 4889f2                       mov rdx, rsi
14229332d: 4c8d8514060000               lea r8, [rbp + 0x614]
142293334: e8d75a71fe                   call 0x1409a8e10
142293339: 488b8d50060000               mov rcx, qword ptr [rbp + 0x650]
142293340: c6859f06000000               mov byte ptr [rbp + 0x69f], 0
142293347: 4c89ea                       mov rdx, r13
14229334a: 4989f8                       mov r8, rdi
14229334d: e8ce9d0200                   call 0x1422bd120
142293352: 488985e8030000               mov qword ptr [rbp + 0x3e8], rax
142293359: 488995f0030000               mov qword ptr [rbp + 0x3f0], rdx
142293360: 488b8508070000               mov rax, qword ptr [rbp + 0x708]
142293367: 0fb68087000000               movzx eax, byte ptr [rax + 0x87]
14229336e: 888530050000                 mov byte ptr [rbp + 0x530], al
142293374: 488d8de8030000               lea rcx, [rbp + 0x3e8]
14229337b: e800b369fe                   call 0x14092e680
142293380: 84c0                         test al, al
142293382: 7452                         je 0x1422933d6
142293384: 488b85a8060000               mov rax, qword ptr [rbp + 0x6a8]
14229338b: 48898558060000               mov qword ptr [rbp + 0x658], rax
142293392: 8b85f8060000                 mov eax, dword ptr [rbp + 0x6f8]
142293398: 488b8d00070000               mov rcx, qword ptr [rbp + 0x700]
14229339f: 488b95d0060000               mov rdx, qword ptr [rbp + 0x6d0]
1422933a6: 89048a                       mov dword ptr [rdx + rcx*4], eax
1422933a9: 488b85b0060000               mov rax, qword ptr [rbp + 0x6b0]
1422933b0: c6402001                     mov byte ptr [rax + 0x20], 1
1422933b4: be01000000                   mov esi, 1
1422933b9: 4889f9                       mov rcx, rdi
1422933bc: e89f2263fe                   call 0x1408c5660
1422933c1: 0fb68530050000               movzx eax, byte ptr [rbp + 0x530]
1422933c8: 8885a6060000                 mov byte ptr [rbp + 0x6a6], al
1422933ce: 4989f5                       mov r13, rsi
1422933d1: e9f7030000                   jmp 0x1422937cd
1422933d6: 4885db                       test rbx, rbx
1422933d9: 0f848b030000                 je 0x14229376a
1422933df: 41bd01000000                 mov r13d, 1
1422933e5: e9e3030000                   jmp 0x1422937cd
1422933ea: 80bdcc06000000               cmp byte ptr [rbp + 0x6cc], 0
1422933f1: 488b8508070000               mov rax, qword ptr [rbp + 0x708]
1422933f8: 7434                         je 0x14229342e
1422933fa: f60001                       test byte ptr [rax], 1
1422933fd: 742f                         je 0x14229342e
1422933ff: 488b4008                     mov rax, qword ptr [rax + 8]
142293403: 483985e0060000               cmp qword ptr [rbp + 0x6e0], rax
14229340a: 7522                         jne 0x14229342e
14229340c: 8b85f8060000                 mov eax, dword ptr [rbp + 0x6f8]
142293412: 488b8d00070000               mov rcx, qword ptr [rbp + 0x700]
142293419: 488b95d0060000               mov rdx, qword ptr [rbp + 0x6d0]
142293420: 89048a                       mov dword ptr [rdx + rcx*4], eax
142293423: 488b85b0060000               mov rax, qword ptr [rbp + 0x6b0]
14229342a: c6402001                     mov byte ptr [rax + 0x20], 1
14229342e: 80bd1707000000               cmp byte ptr [rbp + 0x717], 0
142293435: 740d                         je 0x142293444
142293437: 4080f601                     xor sil, 1
14229343b: b001                         mov al, 1
14229343d: 48898540030000               mov qword ptr [rbp + 0x340], rax
142293444: 4531ed                       xor r13d, r13d
142293447: 4889d9                       mov rcx, rbx
14229344a: e8a1b169fe                   call 0x14092e5f0
14229344f: 41b601                       mov r14b, 1
142293452: 41b701                       mov r15b, 1
142293455: 84c0                         test al, al
142293457: 488b9dc0030000               mov rbx, qword ptr [rbp + 0x3c0]
14229345e: 744a                         je 0x1422934aa
142293460: 0fb605a2173a01               movzx eax, byte ptr [rip + 0x13a17a2]
142293467: 4084f6                       test sil, sil
14229346a: 0f84a7000000                 je 0x142293517
142293470: b91a000000                   mov ecx, 0x1a
142293475: ba01000000                   mov edx, 1
14229347a: e8910738fe                   call 0x140613c10
14229347f: 4885c0                       test rax, rax
142293482: 0f8471380000                 je 0x142296cf9
142293488: 4889c7                       mov rdi, rax
14229348b: 0f1005e583da00               movups xmm0, xmmword ptr [rip + 0xda83e5]
142293492: 0f11400a                     movups xmmword ptr [rax + 0xa], xmm0
142293496: 0f1005d083da00               movups xmm0, xmmword ptr [rip + 0xda83d0]
14229349d: 0f1100                       movups xmmword ptr [rax], xmm0
1422934a0: be1a000000                   mov esi, 0x1a
1422934a5: e9a2000000                   jmp 0x14229354c
1422934aa: 488b8560060000               mov rax, qword ptr [rbp + 0x660]
1422934b1: 8b8898000000                 mov ecx, dword ptr [rax + 0x98]
1422934b7: f30f10889c000000             movss xmm1, dword ptr [rax + 0x9c]
1422934bf: e8ccc56afe                   call 0x14093fa90
1422934c4: 84c0                         test al, al
1422934c6: 7439                         je 0x142293501
1422934c8: 488b8508070000               mov rax, qword ptr [rbp + 0x708]
1422934cf: 833801                       cmp dword ptr [rax], 1
1422934d2: 752d                         jne 0x142293501
1422934d4: 488b4008                     mov rax, qword ptr [rax + 8]
1422934d8: 483b85e0060000               cmp rax, qword ptr [rbp + 0x6e0]
1422934df: 410f94c6                     sete r14b
1422934e3: 450fb6fc                     movzx r15d, r12b
1422934e7: b801000000                   mov eax, 1
1422934ec: 440f44f8                     cmove r15d, eax
1422934f0: 0fb60512173a01               movzx eax, byte ptr [rip + 0x13a1712]
1422934f7: 4084f6                       test sil, sil
1422934fa: 741b                         je 0x142293517
1422934fc: e96fffffff                   jmp 0x142293470
142293501: 4531f6                       xor r14d, r14d
142293504: 4589e7                       mov r15d, r12d
142293507: 0fb605fb163a01               movzx eax, byte ptr [rip + 0x13a16fb]
14229350e: 4084f6                       test sil, sil
142293511: 0f8559ffffff                 jne 0x142293470
142293517: b91d000000                   mov ecx, 0x1d
14229351c: ba01000000                   mov edx, 1
142293521: e8ea0638fe                   call 0x140613c10
142293526: 4885c0                       test rax, rax
142293529: 0f84b4370000                 je 0x142296ce3
14229352f: 4889c7                       mov rdi, rax
142293532: 0f10052483da00               movups xmm0, xmmword ptr [rip + 0xda8324]
142293539: 0f11400d                     movups xmmword ptr [rax + 0xd], xmm0
14229353d: 0f10050c83da00               movups xmm0, xmmword ptr [rip + 0xda830c]
142293544: 0f1100                       movups xmmword ptr [rax], xmm0
142293547: be1d000000                   mov esi, 0x1d
14229354c: 4c8b85b8060000               mov r8, qword ptr [rbp + 0x6b8]
142293553: 488b8dc0060000               mov rcx, qword ptr [rbp + 0x6c0]
14229355a: 8b85f8060000                 mov eax, dword ptr [rbp + 0x6f8]
142293560: 488b9500070000               mov rdx, qword ptr [rbp + 0x700]
142293567: 890493                       mov dword ptr [rbx + rdx*4], eax
14229356a: 488b9df8040000               mov rbx, qword ptr [rbp + 0x4f8]
142293571: 488b13                       mov rdx, qword ptr [rbx]
142293574: 4885d2                       test rdx, rdx
142293577: 741d                         je 0x142293596
142293579: 488b4b08                     mov rcx, qword ptr [rbx + 8]
14229357d: 41b801000000                 mov r8d, 1
142293583: e8980638fe                   call 0x140613c20
142293588: 488b8dc0060000               mov rcx, qword ptr [rbp + 0x6c0]
14229358f: 4c8b85b8060000               mov r8, qword ptr [rbp + 0x6b8]
142293596: 488933                       mov qword ptr [rbx], rsi
142293599: 48897b08                     mov qword ptr [rbx + 8], rdi
14229359d: 48897310                     mov qword ptr [rbx + 0x10], rsi
1422935a1: 488dbd30050000               lea rdi, [rbp + 0x530]
1422935a8: 80bd9c04000000               cmp byte ptr [rbp + 0x49c], 0
1422935af: 0f85a3fcffff                 jne 0x142293258
1422935b5: 4489fe                       mov esi, r15d
1422935b8: 80bd9804000000               cmp byte ptr [rbp + 0x498], 0
1422935bf: 4c8dbde8030000               lea r15, [rbp + 0x3e8]
1422935c6: 0f8461030000                 je 0x14229392d
1422935cc: 4c89eb                       mov rbx, r13
1422935cf: 488d85e0060000               lea rax, [rbp + 0x6e0]
1422935d6: 488985e8030000               mov qword ptr [rbp + 0x3e8], rax
1422935dd: 488d05dc4ee3fd               lea rax, [rip - 0x21cb124]
1422935e4: 488985f0030000               mov qword ptr [rbp + 0x3f0], rax
1422935eb: 488d8da8060000               lea rcx, [rbp + 0x6a8]
1422935f2: 48898df8030000               mov qword ptr [rbp + 0x3f8], rcx
1422935f9: 48898500040000               mov qword ptr [rbp + 0x400], rax
142293600: 488d050183da00               lea rax, [rip + 0xda8301]
142293607: 48898530050000               mov qword ptr [rbp + 0x530], rax
14229360e: 48c7853805000002000000       mov qword ptr [rbp + 0x538], 2
142293619: 4c89bd40050000               mov qword ptr [rbp + 0x540], r15
142293620: 48c7854805000002000000       mov qword ptr [rbp + 0x548], 2
14229362b: 48c7855005000000000000       mov qword ptr [rbp + 0x550], 0
142293636: 4c8dad38060000               lea r13, [rbp + 0x638]
14229363d: 4c89e9                       mov rcx, r13
142293640: 4889fa                       mov rdx, rdi
142293643: e84869aefe                   call 0x140d79f90
142293648: 488b8530060000               mov rax, qword ptr [rbp + 0x630]
14229364f: f20f1000                     movsd xmm0, qword ptr [rax]
142293653: f20f104808                   movsd xmm1, qword ptr [rax + 8]
142293658: f20f12d0                     movddup xmm2, xmm0
14229365c: 410f59cd                     mulps xmm1, xmm13
142293660: f20f12d9                     movddup xmm3, xmm1
142293664: 0f58da                       addps xmm3, xmm2
142293667: 0f5cc1                       subps xmm0, xmm1
14229366a: 0fc6c3e4                     shufps xmm0, xmm3, 0xe4
14229366e: 0f1185f0030000               movups xmmword ptr [rbp + 0x3f0], xmm0
142293675: c785ec03000000000000         mov dword ptr [rbp + 0x3ec], 0
14229367f: c685e803000003               mov byte ptr [rbp + 0x3e8], 3
142293686: c6851406000000               mov byte ptr [rbp + 0x614], 0
14229368d: c6859e06000001               mov byte ptr [rbp + 0x69e], 1
142293694: 4889f9                       mov rcx, rdi
142293697: 4c89fa                       mov rdx, r15
14229369a: 4c8d8514060000               lea r8, [rbp + 0x614]
1422936a1: e86a5771fe                   call 0x1409a8e10
1422936a6: c6859e06000000               mov byte ptr [rbp + 0x69e], 0
1422936ad: 488b8d50060000               mov rcx, qword ptr [rbp + 0x650]
1422936b4: 4c89ea                       mov rdx, r13
1422936b7: 4989f8                       mov r8, rdi
1422936ba: e8619a0200                   call 0x1422bd120
1422936bf: 488985e8030000               mov qword ptr [rbp + 0x3e8], rax
1422936c6: 488995f0030000               mov qword ptr [rbp + 0x3f0], rdx
1422936cd: 488b8508070000               mov rax, qword ptr [rbp + 0x708]
1422936d4: 0fb68088000000               movzx eax, byte ptr [rax + 0x88]
1422936db: 888530050000                 mov byte ptr [rbp + 0x530], al
1422936e1: 4c89f9                       mov rcx, r15
1422936e4: e897af69fe                   call 0x14092e680
1422936e9: 84c0                         test al, al
1422936eb: 745a                         je 0x142293747
1422936ed: 488b85a8060000               mov rax, qword ptr [rbp + 0x6a8]
1422936f4: 48898558060000               mov qword ptr [rbp + 0x658], rax
1422936fb: 8b85f8060000                 mov eax, dword ptr [rbp + 0x6f8]
142293701: 488b8d00070000               mov rcx, qword ptr [rbp + 0x700]
142293708: 488b95d0060000               mov rdx, qword ptr [rbp + 0x6d0]
14229370f: 89048a                       mov dword ptr [rdx + rcx*4], eax
142293712: 488b85b0060000               mov rax, qword ptr [rbp + 0x6b0]
142293719: c6402001                     mov byte ptr [rax + 0x20], 1
14229371d: b801000000                   mov eax, 1
142293722: 488b9d30060000               mov rbx, qword ptr [rbp + 0x630]
142293729: 0fb68d30050000               movzx ecx, byte ptr [rbp + 0x530]
142293730: 80f101                       xor cl, 1
142293733: 888da7060000                 mov byte ptr [rbp + 0x6a7], cl
142293739: 888d30050000                 mov byte ptr [rbp + 0x530], cl
14229373f: 4989c5                       mov r13, rax
142293742: e9da030000                   jmp 0x142293b21
142293747: 4885db                       test rbx, rbx
14229374a: 440fb68517070000             movzx r8d, byte ptr [rbp + 0x717]
142293752: 0f8463030000                 je 0x142293abb
142293758: 41bd01000000                 mov r13d, 1
14229375e: 488b9d30060000               mov rbx, qword ptr [rbp + 0x630]
142293765: e9b7030000                   jmp 0x142293b21
14229376a: 80bdcc06000000               cmp byte ptr [rbp + 0x6cc], 0
142293771: 7442                         je 0x1422937b5
142293773: 488b8508070000               mov rax, qword ptr [rbp + 0x708]
14229377a: f60001                       test byte ptr [rax], 1
14229377d: 7436                         je 0x1422937b5
14229377f: 488b8508070000               mov rax, qword ptr [rbp + 0x708]
142293786: 488b4008                     mov rax, qword ptr [rax + 8]
14229378a: 483985e0060000               cmp qword ptr [rbp + 0x6e0], rax
142293791: 7522                         jne 0x1422937b5
142293793: 8b85f8060000                 mov eax, dword ptr [rbp + 0x6f8]
142293799: 488b8d00070000               mov rcx, qword ptr [rbp + 0x700]
1422937a0: 488b95d0060000               mov rdx, qword ptr [rbp + 0x6d0]
1422937a7: 89048a                       mov dword ptr [rdx + rcx*4], eax
1422937aa: 488b85b0060000               mov rax, qword ptr [rbp + 0x6b0]
1422937b1: c6402001                     mov byte ptr [rax + 0x20], 1
1422937b5: be00000000                   mov esi, 0
1422937ba: 41bd00000000                 mov r13d, 0
1422937c0: 80bd1707000000               cmp byte ptr [rbp + 0x717], 0
1422937c7: 0f85ecfbffff                 jne 0x1422933b9
1422937cd: 488d8de8030000               lea rcx, [rbp + 0x3e8]
1422937d4: e817ae69fe                   call 0x14092e5f0
1422937d9: b301                         mov bl, 1
1422937db: 40b601                       mov sil, 1
1422937de: 84c0                         test al, al
1422937e0: 754b                         jne 0x14229382d
1422937e2: 488b8560060000               mov rax, qword ptr [rbp + 0x660]
1422937e9: 8b8898000000                 mov ecx, dword ptr [rax + 0x98]
1422937ef: f30f10889c000000             movss xmm1, dword ptr [rax + 0x9c]
1422937f7: e894c26afe                   call 0x14093fa90
1422937fc: 84c0                         test al, al
1422937fe: 7428                         je 0x142293828
142293800: 488b8508070000               mov rax, qword ptr [rbp + 0x708]
142293807: 833801                       cmp dword ptr [rax], 1
14229380a: 751c                         jne 0x142293828
14229380c: 488b4008                     mov rax, qword ptr [rax + 8]
142293810: 483b85e0060000               cmp rax, qword ptr [rbp + 0x6e0]
142293817: 0f94c3                       sete bl
14229381a: 410fb6f7                     movzx esi, r15b
14229381e: b801000000                   mov eax, 1
142293823: 0f44f0                       cmove esi, eax
142293826: eb05                         jmp 0x14229382d
142293828: 31db                         xor ebx, ebx
14229382a: 4489fe                       mov esi, r15d
14229382d: 4889f9                       mov rcx, rdi
142293830: e8fbf959fe                   call 0x140833230
142293835: 0fb60dcd133a01               movzx ecx, byte ptr [rip + 0x13a13cd]
14229383c: 84c0                         test al, al
14229383e: 743a                         je 0x14229387a
142293840: b91e000000                   mov ecx, 0x1e
142293845: ba01000000                   mov edx, 1
14229384a: e8c10338fe                   call 0x140613c10
14229384f: 4885c0                       test rax, rax
142293852: 0f842f340000                 je 0x142296c87
142293858: 4889c7                       mov rdi, rax
14229385b: 4189df                       mov r15d, ebx
14229385e: 0f10058080da00               movups xmm0, xmmword ptr [rip + 0xda8080]
142293865: 0f11400e                     movups xmmword ptr [rax + 0xe], xmm0
142293869: 0f10056780da00               movups xmm0, xmmword ptr [rip + 0xda8067]
142293870: 0f1100                       movups xmmword ptr [rax], xmm0
142293873: bb1e000000                   mov ebx, 0x1e
142293878: eb38                         jmp 0x1422938b2
14229387a: b91f000000                   mov ecx, 0x1f
14229387f: ba01000000                   mov edx, 1
142293884: e8870338fe                   call 0x140613c10
142293889: 4885c0                       test rax, rax
14229388c: 0f84d0330000                 je 0x142296c62
142293892: 4889c7                       mov rdi, rax
142293895: 4189df                       mov r15d, ebx
142293898: 0f10052880da00               movups xmm0, xmmword ptr [rip + 0xda8028]
14229389f: 0f11400f                     movups xmmword ptr [rax + 0xf], xmm0
1422938a3: 0f10050e80da00               movups xmm0, xmmword ptr [rip + 0xda800e]
1422938aa: 0f1100                       movups xmmword ptr [rax], xmm0
1422938ad: bb1f000000                   mov ebx, 0x1f
1422938b2: 4c8b85b8060000               mov r8, qword ptr [rbp + 0x6b8]
1422938b9: 488b8dc0060000               mov rcx, qword ptr [rbp + 0x6c0]
1422938c0: 8b85f8060000                 mov eax, dword ptr [rbp + 0x6f8]
1422938c6: 488b9500070000               mov rdx, qword ptr [rbp + 0x700]
1422938cd: 4c8b8dc0030000               mov r9, qword ptr [rbp + 0x3c0]
1422938d4: 41890491                     mov dword ptr [r9 + rdx*4], eax
1422938d8: 4c8bb5f8040000               mov r14, qword ptr [rbp + 0x4f8]
1422938df: 498b16                       mov rdx, qword ptr [r14]
1422938e2: 4885d2                       test rdx, rdx
1422938e5: 741d                         je 0x142293904
1422938e7: 498b4e08                     mov rcx, qword ptr [r14 + 8]
1422938eb: 41b801000000                 mov r8d, 1
1422938f1: e82a0338fe                   call 0x140613c20
1422938f6: 488b8dc0060000               mov rcx, qword ptr [rbp + 0x6c0]
1422938fd: 4c8b85b8060000               mov r8, qword ptr [rbp + 0x6b8]
142293904: 49891e                       mov qword ptr [r14], rbx
142293907: 49897e08                     mov qword ptr [r14 + 8], rdi
14229390b: 49895e10                     mov qword ptr [r14 + 0x10], rbx
14229390f: 488dbd30050000               lea rdi, [rbp + 0x530]
142293916: 4589fe                       mov r14d, r15d
142293919: 80bd9804000000               cmp byte ptr [rbp + 0x498], 0
142293920: 4c8dbde8030000               lea r15, [rbp + 0x3e8]
142293927: 0f859ffcffff                 jne 0x1422935cc
14229392d: 89b5ec060000                 mov dword ptr [rbp + 0x6ec], esi
142293933: 488b9d30060000               mov rbx, qword ptr [rbp + 0x630]
14229393a: 80bdf802000000               cmp byte ptr [rbp + 0x2f8], 0
142293941: 488db538060000               lea rsi, [rbp + 0x638]
142293948: 0f843a030000                 je 0x142293c88
14229394e: 488d85e0060000               lea rax, [rbp + 0x6e0]
142293955: 488985e8030000               mov qword ptr [rbp + 0x3e8], rax
14229395c: 488d055d4be3fd               lea rax, [rip - 0x21cb4a3]
142293963: 488985f0030000               mov qword ptr [rbp + 0x3f0], rax
14229396a: 488d8da8060000               lea rcx, [rbp + 0x6a8]
142293971: 48898df8030000               mov qword ptr [rbp + 0x3f8], rcx
142293978: 48898500040000               mov qword ptr [rbp + 0x400], rax
14229397f: 488d05b27fda00               lea rax, [rip + 0xda7fb2]
142293986: 48898530050000               mov qword ptr [rbp + 0x530], rax
14229398d: 48c7853805000002000000       mov qword ptr [rbp + 0x538], 2
142293998: 4c89bd40050000               mov qword ptr [rbp + 0x540], r15
14229399f: 48c7854805000002000000       mov qword ptr [rbp + 0x548], 2
1422939aa: 48c7855005000000000000       mov qword ptr [rbp + 0x550], 0
1422939b5: 4889f1                       mov rcx, rsi
1422939b8: 4889fa                       mov rdx, rdi
1422939bb: e8d065aefe                   call 0x140d79f90
1422939c0: f20f1003                     movsd xmm0, qword ptr [rbx]
1422939c4: f20f104b08                   movsd xmm1, qword ptr [rbx + 8]
1422939c9: f20f12d0                     movddup xmm2, xmm0
1422939cd: 410f59cd                     mulps xmm1, xmm13
1422939d1: f20f12d9                     movddup xmm3, xmm1
1422939d5: 0f58da                       addps xmm3, xmm2
1422939d8: 0f5cc1                       subps xmm0, xmm1
1422939db: 0fc6c3e4                     shufps xmm0, xmm3, 0xe4
1422939df: 0f1185f0030000               movups xmmword ptr [rbp + 0x3f0], xmm0
1422939e6: c785ec03000000000000         mov dword ptr [rbp + 0x3ec], 0
1422939f0: c685e803000003               mov byte ptr [rbp + 0x3e8], 3
1422939f7: c6851406000000               mov byte ptr [rbp + 0x614], 0
1422939fe: c6859d06000001               mov byte ptr [rbp + 0x69d], 1
142293a05: 4889f9                       mov rcx, rdi
142293a08: 4c89fa                       mov rdx, r15
142293a0b: 4c8d8514060000               lea r8, [rbp + 0x614]
142293a12: e8f95371fe                   call 0x1409a8e10
142293a17: c6859d06000000               mov byte ptr [rbp + 0x69d], 0
142293a1e: 488b8d50060000               mov rcx, qword ptr [rbp + 0x650]
142293a25: 4889f2                       mov rdx, rsi
142293a28: 4989f8                       mov r8, rdi
142293a2b: e8f0960200                   call 0x1422bd120
142293a30: 488985e8030000               mov qword ptr [rbp + 0x3e8], rax
142293a37: 488995f0030000               mov qword ptr [rbp + 0x3f0], rdx
142293a3e: 4c89f9                       mov rcx, r15
142293a41: e83aac69fe                   call 0x14092e680
142293a46: 84c0                         test al, al
142293a48: 7455                         je 0x142293a9f
142293a4a: 488b85a8060000               mov rax, qword ptr [rbp + 0x6a8]
142293a51: 48898558060000               mov qword ptr [rbp + 0x658], rax
142293a58: 8b85f8060000                 mov eax, dword ptr [rbp + 0x6f8]
142293a5e: 488b8d00070000               mov rcx, qword ptr [rbp + 0x700]
142293a65: 488b95d0060000               mov rdx, qword ptr [rbp + 0x6d0]
142293a6c: 89048a                       mov dword ptr [rdx + rcx*4], eax
142293a6f: 488b85b0060000               mov rax, qword ptr [rbp + 0x6b0]
142293a76: c6402001                     mov byte ptr [rax + 0x20], 1
142293a7a: b801000000                   mov eax, 1
142293a7f: 488b8d00070000               mov rcx, qword ptr [rbp + 0x700]
142293a86: 488b95d8040000               mov rdx, qword ptr [rbp + 0x4d8]
142293a8d: 0fb60c0a                     movzx ecx, byte ptr [rdx + rcx]
142293a91: 888df7060000                 mov byte ptr [rbp + 0x6f7], cl
142293a97: 4989c5                       mov r13, rax
142293a9a: e9dd030000                   jmp 0x142293e7c
142293a9f: 4d85ed                       test r13, r13
142293aa2: 440fb68517070000             movzx r8d, byte ptr [rbp + 0x717]
142293aaa: 0f846d030000                 je 0x142293e1d
142293ab0: 41bd01000000                 mov r13d, 1
142293ab6: e9c1030000                   jmp 0x142293e7c
142293abb: 80bdcc06000000               cmp byte ptr [rbp + 0x6cc], 0
142293ac2: 488b9d30060000               mov rbx, qword ptr [rbp + 0x630]
142293ac9: 7442                         je 0x142293b0d
142293acb: 488b8508070000               mov rax, qword ptr [rbp + 0x708]
142293ad2: f60001                       test byte ptr [rax], 1
142293ad5: 7436                         je 0x142293b0d
142293ad7: 488b8508070000               mov rax, qword ptr [rbp + 0x708]
142293ade: 488b4008                     mov rax, qword ptr [rax + 8]
142293ae2: 483985e0060000               cmp qword ptr [rbp + 0x6e0], rax
142293ae9: 7522                         jne 0x142293b0d
142293aeb: 8b85f8060000                 mov eax, dword ptr [rbp + 0x6f8]
142293af1: 488b8d00070000               mov rcx, qword ptr [rbp + 0x700]
142293af8: 488b95d0060000               mov rdx, qword ptr [rbp + 0x6d0]
142293aff: 89048a                       mov dword ptr [rdx + rcx*4], eax
142293b02: 488b85b0060000               mov rax, qword ptr [rbp + 0x6b0]
142293b09: c6402001                     mov byte ptr [rax + 0x20], 1
142293b0d: b800000000                   mov eax, 0
142293b12: 41bd00000000                 mov r13d, 0
142293b18: 4584c0                       test r8b, r8b
142293b1b: 0f8508fcffff                 jne 0x142293729
142293b21: 4c89f9                       mov rcx, r15
142293b24: e8c7aa69fe                   call 0x14092e5f0
142293b29: 41b401                       mov r12b, 1
142293b2c: b101                         mov cl, 1
142293b2e: 898dec060000                 mov dword ptr [rbp + 0x6ec], ecx
142293b34: 84c0                         test al, al
142293b36: 7556                         jne 0x142293b8e
142293b38: 488b8560060000               mov rax, qword ptr [rbp + 0x660]
142293b3f: 8b8898000000                 mov ecx, dword ptr [rax + 0x98]
142293b45: f30f10889c000000             movss xmm1, dword ptr [rax + 0x9c]
142293b4d: e83ebf6afe                   call 0x14093fa90
142293b52: 84c0                         test al, al
142293b54: 742f                         je 0x142293b85
142293b56: 488b8508070000               mov rax, qword ptr [rbp + 0x708]
142293b5d: 833801                       cmp dword ptr [rax], 1
142293b60: 7523                         jne 0x142293b85
142293b62: 488b4008                     mov rax, qword ptr [rax + 8]
142293b66: 483b85e0060000               cmp rax, qword ptr [rbp + 0x6e0]
142293b6d: 410f94c4                     sete r12b
142293b71: 400fb6ce                     movzx ecx, sil
142293b75: b801000000                   mov eax, 1
142293b7a: 0f44c8                       cmove ecx, eax
142293b7d: 898dec060000                 mov dword ptr [rbp + 0x6ec], ecx
142293b83: eb09                         jmp 0x142293b8e
142293b85: 4531e4                       xor r12d, r12d
142293b88: 89b5ec060000                 mov dword ptr [rbp + 0x6ec], esi
142293b8e: 4889f9                       mov rcx, rdi
142293b91: e89af659fe                   call 0x140833230
142293b96: 0fb60d6c103a01               movzx ecx, byte ptr [rip + 0x13a106c]
142293b9d: 84c0                         test al, al
142293b9f: 7437                         je 0x142293bd8
142293ba1: b91e000000                   mov ecx, 0x1e
142293ba6: ba01000000                   mov edx, 1
142293bab: e8600038fe                   call 0x140613c10
142293bb0: 4885c0                       test rax, rax
142293bb3: 0f84ce300000                 je 0x142296c87
142293bb9: 4889c7                       mov rdi, rax
142293bbc: 0f1005227dda00               movups xmm0, xmmword ptr [rip + 0xda7d22]
142293bc3: 0f11400e                     movups xmmword ptr [rax + 0xe], xmm0
142293bc7: 0f1005097dda00               movups xmm0, xmmword ptr [rip + 0xda7d09]
142293bce: 0f1100                       movups xmmword ptr [rax], xmm0
142293bd1: be1e000000                   mov esi, 0x1e
142293bd6: eb35                         jmp 0x142293c0d
142293bd8: b91f000000                   mov ecx, 0x1f
142293bdd: ba01000000                   mov edx, 1
142293be2: e8290038fe                   call 0x140613c10
142293be7: 4885c0                       test rax, rax
142293bea: 0f8472300000                 je 0x142296c62
142293bf0: 4889c7                       mov rdi, rax
142293bf3: 0f1005cd7cda00               movups xmm0, xmmword ptr [rip + 0xda7ccd]
142293bfa: 0f11400f                     movups xmmword ptr [rax + 0xf], xmm0
142293bfe: 0f1005b37cda00               movups xmm0, xmmword ptr [rip + 0xda7cb3]
142293c05: 0f1100                       movups xmmword ptr [rax], xmm0
142293c08: be1f000000                   mov esi, 0x1f
142293c0d: 4c8b85b8060000               mov r8, qword ptr [rbp + 0x6b8]
142293c14: 488b8dc0060000               mov rcx, qword ptr [rbp + 0x6c0]
142293c1b: 8b85f8060000                 mov eax, dword ptr [rbp + 0x6f8]
142293c21: 488b9500070000               mov rdx, qword ptr [rbp + 0x700]
142293c28: 4c8b8dc0030000               mov r9, qword ptr [rbp + 0x3c0]
142293c2f: 41890491                     mov dword ptr [r9 + rdx*4], eax
142293c33: 4c8bb5f8040000               mov r14, qword ptr [rbp + 0x4f8]
142293c3a: 498b16                       mov rdx, qword ptr [r14]
142293c3d: 4885d2                       test rdx, rdx
142293c40: 741d                         je 0x142293c5f
142293c42: 498b4e08                     mov rcx, qword ptr [r14 + 8]
142293c46: 41b801000000                 mov r8d, 1
142293c4c: e8cfff37fe                   call 0x140613c20
142293c51: 488b8dc0060000               mov rcx, qword ptr [rbp + 0x6c0]
142293c58: 4c8b85b8060000               mov r8, qword ptr [rbp + 0x6b8]
142293c5f: 498936                       mov qword ptr [r14], rsi
142293c62: 49897e08                     mov qword ptr [r14 + 8], rdi
142293c66: 49897610                     mov qword ptr [r14 + 0x10], rsi
142293c6a: 488dbd30050000               lea rdi, [rbp + 0x530]
142293c71: 4589e6                       mov r14d, r12d
142293c74: 80bdf802000000               cmp byte ptr [rbp + 0x2f8], 0
142293c7b: 488db538060000               lea rsi, [rbp + 0x638]
142293c82: 0f85c6fcffff                 jne 0x14229394e
142293c88: 80bd0003000000               cmp byte ptr [rbp + 0x300], 0
142293c8f: 0f846c020000                 je 0x142293f01
142293c95: 488d85e0060000               lea rax, [rbp + 0x6e0]
142293c9c: 488985e8030000               mov qword ptr [rbp + 0x3e8], rax
142293ca3: 488d051648e3fd               lea rax, [rip - 0x21cb7ea]
142293caa: 488985f0030000               mov qword ptr [rbp + 0x3f0], rax
142293cb1: 488d8da8060000               lea rcx, [rbp + 0x6a8]
142293cb8: 48898df8030000               mov qword ptr [rbp + 0x3f8], rcx
142293cbf: 48898500040000               mov qword ptr [rbp + 0x400], rax
142293cc6: 488d05a37cda00               lea rax, [rip + 0xda7ca3]
142293ccd: 48898530050000               mov qword ptr [rbp + 0x530], rax
142293cd4: 48c7853805000002000000       mov qword ptr [rbp + 0x538], 2
142293cdf: 4c89bd40050000               mov qword ptr [rbp + 0x540], r15
142293ce6: 48c7854805000002000000       mov qword ptr [rbp + 0x548], 2
142293cf1: 48c7855005000000000000       mov qword ptr [rbp + 0x550], 0
142293cfc: 4889f1                       mov rcx, rsi
142293cff: 4889fa                       mov rdx, rdi
142293d02: e88962aefe                   call 0x140d79f90
142293d07: f20f1003                     movsd xmm0, qword ptr [rbx]
142293d0b: f20f104b08                   movsd xmm1, qword ptr [rbx + 8]
142293d10: f20f12d0                     movddup xmm2, xmm0
142293d14: 410f59cd                     mulps xmm1, xmm13
142293d18: f20f12d9                     movddup xmm3, xmm1
142293d1c: 0f58da                       addps xmm3, xmm2
142293d1f: 0f5cc1                       subps xmm0, xmm1
142293d22: 0fc6c3e4                     shufps xmm0, xmm3, 0xe4
142293d26: 0f1185f0030000               movups xmmword ptr [rbp + 0x3f0], xmm0
142293d2d: c785ec03000000000000         mov dword ptr [rbp + 0x3ec], 0
142293d37: c685e803000003               mov byte ptr [rbp + 0x3e8], 3
142293d3e: c6851406000000               mov byte ptr [rbp + 0x614], 0
142293d45: c6859c06000001               mov byte ptr [rbp + 0x69c], 1
142293d4c: 4889f9                       mov rcx, rdi
142293d4f: 4c89fa                       mov rdx, r15
142293d52: 4c8d8514060000               lea r8, [rbp + 0x614]
142293d59: e8b25071fe                   call 0x1409a8e10
142293d5e: c6859c06000000               mov byte ptr [rbp + 0x69c], 0
142293d65: 488b8d50060000               mov rcx, qword ptr [rbp + 0x650]
142293d6c: 4889f2                       mov rdx, rsi
142293d6f: 4989f8                       mov r8, rdi
142293d72: e8a9930200                   call 0x1422bd120
142293d77: 488985e8030000               mov qword ptr [rbp + 0x3e8], rax
142293d7e: 488995f0030000               mov qword ptr [rbp + 0x3f0], rdx
142293d85: 4c89f9                       mov rcx, r15
142293d88: e8f3a869fe                   call 0x14092e680
142293d8d: 84c0                         test al, al
142293d8f: 7469                         je 0x142293dfa
142293d91: 488b85a8060000               mov rax, qword ptr [rbp + 0x6a8]
142293d98: 48898558060000               mov qword ptr [rbp + 0x658], rax
142293d9f: 8b85f8060000                 mov eax, dword ptr [rbp + 0x6f8]
142293da5: 488b8d00070000               mov rcx, qword ptr [rbp + 0x700]
142293dac: 488b95d0060000               mov rdx, qword ptr [rbp + 0x6d0]
142293db3: 89048a                       mov dword ptr [rdx + rcx*4], eax
142293db6: 488b85b0060000               mov rax, qword ptr [rbp + 0x6b0]
142293dbd: c6402001                     mov byte ptr [rax + 0x20], 1
142293dc1: b801000000                   mov eax, 1
142293dc6: 488b8d08070000               mov rcx, qword ptr [rbp + 0x708]
142293dcd: c7412800000000               mov dword ptr [rcx + 0x28], 0
142293dd4: 488b8d00070000               mov rcx, qword ptr [rbp + 0x700]
142293ddb: 488b95f0040000               mov rdx, qword ptr [rbp + 0x4f0]
142293de2: f3440f10348a                 movss xmm14, dword ptr [rdx + rcx*4]
142293de8: c785f005000001000000         mov dword ptr [rbp + 0x5f0], 1
142293df2: 4989c5                       mov r13, rax
142293df5: e9e1020000                   jmp 0x1422940db
142293dfa: 4d85ed                       test r13, r13
142293dfd: 488b8d08070000               mov rcx, qword ptr [rbp + 0x708]
142293e04: 440fb68d17070000             movzx r9d, byte ptr [rbp + 0x717]
142293e0c: 0f8477020000                 je 0x142294089
142293e12: 41bd01000000                 mov r13d, 1
142293e18: e9be020000                   jmp 0x1422940db
142293e1d: 80bdcc06000000               cmp byte ptr [rbp + 0x6cc], 0
142293e24: 7442                         je 0x142293e68
142293e26: 488b8508070000               mov rax, qword ptr [rbp + 0x708]
142293e2d: f60001                       test byte ptr [rax], 1
142293e30: 7436                         je 0x142293e68
142293e32: 488b8508070000               mov rax, qword ptr [rbp + 0x708]
142293e39: 488b4008                     mov rax, qword ptr [rax + 8]
142293e3d: 483985e0060000               cmp qword ptr [rbp + 0x6e0], rax
142293e44: 7522                         jne 0x142293e68
142293e46: 8b85f8060000                 mov eax, dword ptr [rbp + 0x6f8]
142293e4c: 488b8d00070000               mov rcx, qword ptr [rbp + 0x700]
142293e53: 488b95d0060000               mov rdx, qword ptr [rbp + 0x6d0]
142293e5a: 89048a                       mov dword ptr [rdx + rcx*4], eax
142293e5d: 488b85b0060000               mov rax, qword ptr [rbp + 0x6b0]
142293e64: c6402001                     mov byte ptr [rax + 0x20], 1
142293e68: b800000000                   mov eax, 0
142293e6d: 41bd00000000                 mov r13d, 0
142293e73: 4584c0                       test r8b, r8b
142293e76: 0f8503fcffff                 jne 0x142293a7f
142293e7c: 4c89f9                       mov rcx, r15
142293e7f: e86ca769fe                   call 0x14092e5f0
142293e84: 41b601                       mov r14b, 1
142293e87: b101                         mov cl, 1
142293e89: 84c0                         test al, al
142293e8b: 7553                         jne 0x142293ee0
142293e8d: 488b8560060000               mov rax, qword ptr [rbp + 0x660]
142293e94: 8b8898000000                 mov ecx, dword ptr [rax + 0x98]
142293e9a: f30f10889c000000             movss xmm1, dword ptr [rax + 0x9c]
142293ea2: e8e9bb6afe                   call 0x14093fa90
142293ea7: 84c0                         test al, al
142293ea9: 742c                         je 0x142293ed7
142293eab: 488b8508070000               mov rax, qword ptr [rbp + 0x708]
142293eb2: 833801                       cmp dword ptr [rax], 1
142293eb5: 7520                         jne 0x142293ed7
142293eb7: 488b4008                     mov rax, qword ptr [rax + 8]
142293ebb: 483b85e0060000               cmp rax, qword ptr [rbp + 0x6e0]
142293ec2: 410f94c6                     sete r14b
142293ec6: 0fb68dec060000               movzx ecx, byte ptr [rbp + 0x6ec]
142293ecd: b801000000                   mov eax, 1
142293ed2: 0f44c8                       cmove ecx, eax
142293ed5: eb09                         jmp 0x142293ee0
142293ed7: 4531f6                       xor r14d, r14d
142293eda: 8b8dec060000                 mov ecx, dword ptr [rbp + 0x6ec]
142293ee0: 898dec060000                 mov dword ptr [rbp + 0x6ec], ecx
142293ee6: 4c8b85b8060000               mov r8, qword ptr [rbp + 0x6b8]
142293eed: 488b8dc0060000               mov rcx, qword ptr [rbp + 0x6c0]
142293ef4: 80bd0003000000               cmp byte ptr [rbp + 0x300], 0
142293efb: 0f8594fdffff                 jne 0x142293c95
142293f01: 4c8b9d08070000               mov r11, qword ptr [rbp + 0x708]
142293f08: 80bd0803000000               cmp byte ptr [rbp + 0x308], 0
142293f0f: 0f8491040000                 je 0x1422943a6
142293f15: 488d85e0060000               lea rax, [rbp + 0x6e0]
142293f1c: 488985e8030000               mov qword ptr [rbp + 0x3e8], rax
142293f23: 488d059645e3fd               lea rax, [rip - 0x21cba6a]
142293f2a: 488985f0030000               mov qword ptr [rbp + 0x3f0], rax
142293f31: 488d8da8060000               lea rcx, [rbp + 0x6a8]
142293f38: 48898df8030000               mov qword ptr [rbp + 0x3f8], rcx
142293f3f: 48898500040000               mov qword ptr [rbp + 0x400], rax
142293f46: 488d05537ada00               lea rax, [rip + 0xda7a53]
142293f4d: 48898530050000               mov qword ptr [rbp + 0x530], rax
142293f54: 48c7853805000002000000       mov qword ptr [rbp + 0x538], 2
142293f5f: 4c89bd40050000               mov qword ptr [rbp + 0x540], r15
142293f66: 48c7854805000002000000       mov qword ptr [rbp + 0x548], 2
142293f71: 48c7855005000000000000       mov qword ptr [rbp + 0x550], 0
142293f7c: 4889f1                       mov rcx, rsi
142293f7f: 4889fa                       mov rdx, rdi
142293f82: e80960aefe                   call 0x140d79f90
142293f87: f20f1003                     movsd xmm0, qword ptr [rbx]
142293f8b: f20f104b08                   movsd xmm1, qword ptr [rbx + 8]
142293f90: f20f12d0                     movddup xmm2, xmm0
142293f94: 410f59cd                     mulps xmm1, xmm13
142293f98: f20f12d9                     movddup xmm3, xmm1
142293f9c: 0f58da                       addps xmm3, xmm2
142293f9f: 0f5cc1                       subps xmm0, xmm1
142293fa2: 0fc6c3e4                     shufps xmm0, xmm3, 0xe4
142293fa6: 0f1185f0030000               movups xmmword ptr [rbp + 0x3f0], xmm0
142293fad: c785ec03000000000000         mov dword ptr [rbp + 0x3ec], 0
142293fb7: c685e803000003               mov byte ptr [rbp + 0x3e8], 3
142293fbe: c6851406000000               mov byte ptr [rbp + 0x614], 0
142293fc5: c6859b06000001               mov byte ptr [rbp + 0x69b], 1
142293fcc: 4889f9                       mov rcx, rdi
142293fcf: 4c89fa                       mov rdx, r15
142293fd2: 4c8d8514060000               lea r8, [rbp + 0x614]
142293fd9: e8324e71fe                   call 0x1409a8e10
142293fde: c6859b06000000               mov byte ptr [rbp + 0x69b], 0
142293fe5: 488b8d50060000               mov rcx, qword ptr [rbp + 0x650]
142293fec: 4889f2                       mov rdx, rsi
142293fef: 4989f8                       mov r8, rdi
142293ff2: e829910200                   call 0x1422bd120
142293ff7: 488985e8030000               mov qword ptr [rbp + 0x3e8], rax
142293ffe: 488995f0030000               mov qword ptr [rbp + 0x3f0], rdx
142294005: 4c89f9                       mov rcx, r15
142294008: e873a669fe                   call 0x14092e680
14229400d: 84c0                         test al, al
14229400f: 7455                         je 0x142294066
142294011: 488b85a8060000               mov rax, qword ptr [rbp + 0x6a8]
142294018: 48898558060000               mov qword ptr [rbp + 0x658], rax
14229401f: 8b85f8060000                 mov eax, dword ptr [rbp + 0x6f8]
142294025: 488b8d00070000               mov rcx, qword ptr [rbp + 0x700]
14229402c: 488b95d0060000               mov rdx, qword ptr [rbp + 0x6d0]
142294033: 89048a                       mov dword ptr [rdx + rcx*4], eax
142294036: 488b85b0060000               mov rax, qword ptr [rbp + 0x6b0]
14229403d: c6402001                     mov byte ptr [rax + 0x20], 1
142294041: b801000000                   mov eax, 1
142294046: 488bb5e0050000               mov rsi, qword ptr [rbp + 0x5e0]
14229404d: 488b8d00070000               mov rcx, qword ptr [rbp + 0x700]
142294054: 0fb60c0e                     movzx ecx, byte ptr [rsi + rcx]
142294058: 888dff060000                 mov byte ptr [rbp + 0x6ff], cl
14229405e: 4989c5                       mov r13, rax
142294061: e980010000                   jmp 0x1422941e6
142294066: 4d85ed                       test r13, r13
142294069: 488bb5e0050000               mov rsi, qword ptr [rbp + 0x5e0]
142294070: 440fb68517070000             movzx r8d, byte ptr [rbp + 0x717]
142294078: 0f8409010000                 je 0x142294187
14229407e: 41bd01000000                 mov r13d, 1
142294084: e95d010000                   jmp 0x1422941e6
142294089: 80bdcc06000000               cmp byte ptr [rbp + 0x6cc], 0
142294090: 7435                         je 0x1422940c7
142294092: f60101                       test byte ptr [rcx], 1
142294095: 7430                         je 0x1422940c7
142294097: 488b4108                     mov rax, qword ptr [rcx + 8]
14229409b: 483985e0060000               cmp qword ptr [rbp + 0x6e0], rax
1422940a2: 7523                         jne 0x1422940c7
1422940a4: 8b85f8060000                 mov eax, dword ptr [rbp + 0x6f8]
1422940aa: 488b9500070000               mov rdx, qword ptr [rbp + 0x700]
1422940b1: 4c8b85d0060000               mov r8, qword ptr [rbp + 0x6d0]
1422940b8: 41890490                     mov dword ptr [r8 + rdx*4], eax
1422940bc: 488b85b0060000               mov rax, qword ptr [rbp + 0x6b0]
1422940c3: c6402001                     mov byte ptr [rax + 0x20], 1
1422940c7: b800000000                   mov eax, 0
1422940cc: 41bd00000000                 mov r13d, 0
1422940d2: 4584c9                       test r9b, r9b
1422940d5: 0f85f2fcffff                 jne 0x142293dcd
1422940db: 4c89f9                       mov rcx, r15
1422940de: e80da569fe                   call 0x14092e5f0
1422940e3: 84c0                         test al, al
1422940e5: 743a                         je 0x142294121
1422940e7: 4c8b9d08070000               mov r11, qword ptr [rbp + 0x708]
1422940ee: 41c7432800000000             mov dword ptr [r11 + 0x28], 0
1422940f6: 41b601                       mov r14b, 1
1422940f9: b001                         mov al, 1
1422940fb: 8985ec060000                 mov dword ptr [rbp + 0x6ec], eax
142294101: 4c8b85b8060000               mov r8, qword ptr [rbp + 0x6b8]
142294108: 488b8dc0060000               mov rcx, qword ptr [rbp + 0x6c0]
14229410f: 80bd0803000000               cmp byte ptr [rbp + 0x308], 0
142294116: 0f848a020000                 je 0x1422943a6
14229411c: e9f4fdffff                   jmp 0x142293f15
142294121: 488b8560060000               mov rax, qword ptr [rbp + 0x660]
142294128: 8b8898000000                 mov ecx, dword ptr [rax + 0x98]
14229412e: f30f10889c000000             movss xmm1, dword ptr [rax + 0x9c]
142294136: e855b96afe                   call 0x14093fa90
14229413b: 84c0                         test al, al
14229413d: 741e                         je 0x14229415d
14229413f: 4c8b9d08070000               mov r11, qword ptr [rbp + 0x708]
142294146: 41833b01                     cmp dword ptr [r11], 1
14229414a: 0f8538020000                 jne 0x142294388
142294150: 498b4308                     mov rax, qword ptr [r11 + 8]
142294154: 483b85e0060000               cmp rax, qword ptr [rbp + 0x6e0]
14229415b: 748a                         je 0x1422940e7
14229415d: 4531f6                       xor r14d, r14d
142294160: 4c8b9d08070000               mov r11, qword ptr [rbp + 0x708]
142294167: 4c8b85b8060000               mov r8, qword ptr [rbp + 0x6b8]
14229416e: 488b8dc0060000               mov rcx, qword ptr [rbp + 0x6c0]
142294175: 80bd0803000000               cmp byte ptr [rbp + 0x308], 0
14229417c: 0f8424020000                 je 0x1422943a6
142294182: e98efdffff                   jmp 0x142293f15
142294187: 80bdcc06000000               cmp byte ptr [rbp + 0x6cc], 0
14229418e: 7442                         je 0x1422941d2
142294190: 488b8508070000               mov rax, qword ptr [rbp + 0x708]
142294197: f60001                       test byte ptr [rax], 1
14229419a: 7436                         je 0x1422941d2
14229419c: 488b8508070000               mov rax, qword ptr [rbp + 0x708]
1422941a3: 488b4008                     mov rax, qword ptr [rax + 8]
1422941a7: 483985e0060000               cmp qword ptr [rbp + 0x6e0], rax
1422941ae: 7522                         jne 0x1422941d2
1422941b0: 8b85f8060000                 mov eax, dword ptr [rbp + 0x6f8]
1422941b6: 488b8d00070000               mov rcx, qword ptr [rbp + 0x700]
1422941bd: 488b95d0060000               mov rdx, qword ptr [rbp + 0x6d0]
1422941c4: 89048a                       mov dword ptr [rdx + rcx*4], eax
1422941c7: 488b85b0060000               mov rax, qword ptr [rbp + 0x6b0]
1422941ce: c6402001                     mov byte ptr [rax + 0x20], 1
1422941d2: b800000000                   mov eax, 0
1422941d7: 41bd00000000                 mov r13d, 0
1422941dd: 4584c0                       test r8b, r8b
1422941e0: 0f8567feffff                 jne 0x14229404d
1422941e6: 4c89f9                       mov rcx, r15
1422941e9: e802a469fe                   call 0x14092e5f0
1422941ee: 84c0                         test al, al
1422941f0: 7441                         je 0x142294233
1422941f2: 488b8500070000               mov rax, qword ptr [rbp + 0x700]
1422941f9: 0fb60c06                     movzx ecx, byte ptr [rsi + rax]
1422941fd: e86e2587fe                   call 0x140b06770
142294202: 8b85f0060000                 mov eax, dword ptr [rbp + 0x6f0]
142294208: 488b8d90060000               mov rcx, qword ptr [rbp + 0x690]
14229420f: 8901                         mov dword ptr [rcx], eax
142294211: 488b8588060000               mov rax, qword ptr [rbp + 0x688]
142294218: f30f11403c                   movss dword ptr [rax + 0x3c], xmm0
14229421d: 41b601                       mov r14b, 1
142294220: b001                         mov al, 1
142294222: 48898528030000               mov qword ptr [rbp + 0x328], rax
142294229: b001                         mov al, 1
14229422b: 8985ec060000                 mov dword ptr [rbp + 0x6ec], eax
142294231: eb3f                         jmp 0x142294272
142294233: 488b8560060000               mov rax, qword ptr [rbp + 0x660]
14229423a: 8b8898000000                 mov ecx, dword ptr [rax + 0x98]
142294240: f30f10889c000000             movss xmm1, dword ptr [rax + 0x9c]
142294248: e843b86afe                   call 0x14093fa90
14229424d: 84c0                         test al, al
14229424f: 741e                         je 0x14229426f
142294251: 4c8b9d08070000               mov r11, qword ptr [rbp + 0x708]
142294258: 41833b01                     cmp dword ptr [r11], 1
14229425c: 0f8561050000                 jne 0x1422947c3
142294262: 498b4308                     mov rax, qword ptr [r11 + 8]
142294266: 483b85e0060000               cmp rax, qword ptr [rbp + 0x6e0]
14229426d: 7483                         je 0x1422941f2
14229426f: 4531f6                       xor r14d, r14d
142294272: 4c8b9d08070000               mov r11, qword ptr [rbp + 0x708]
142294279: 4c8b85b8060000               mov r8, qword ptr [rbp + 0x6b8]
142294280: 488b8dc0060000               mov rcx, qword ptr [rbp + 0x6c0]
142294287: 80bd8000000000               cmp byte ptr [rbp + 0x80], 0
14229428e: 0f851f010000                 jne 0x1422943b3
142294294: 488b85b0060000               mov rax, qword ptr [rbp + 0x6b0]
14229429b: 80782000                     cmp byte ptr [rax + 0x20], 0
14229429f: 7442                         je 0x1422942e3
1422942a1: 488b85a8060000               mov rax, qword ptr [rbp + 0x6a8]
1422942a8: 48898510020000               mov qword ptr [rbp + 0x210], rax
1422942af: 488b85e0060000               mov rax, qword ptr [rbp + 0x6e0]
1422942b6: 48898530020000               mov qword ptr [rbp + 0x230], rax
1422942bd: 8b45b0                       mov eax, dword ptr [rbp - 0x50]
1422942c0: 8985f8050000                 mov dword ptr [rbp + 0x5f8], eax
1422942c6: 488d45b4                     lea rax, [rbp - 0x4c]
1422942ca: 0f1000                       movups xmm0, xmmword ptr [rax]
1422942cd: 0f294560                     movaps xmmword ptr [rbp + 0x60], xmm0
1422942d1: 8b4010                       mov eax, dword ptr [rax + 0x10]
1422942d4: 894570                       mov dword ptr [rbp + 0x70], eax
1422942d7: b801000000                   mov eax, 1
1422942dc: 48898550030000               mov qword ptr [rbp + 0x350], rax
1422942e3: 4584f6                       test r14b, r14b
1422942e6: 0f8490000000                 je 0x14229437c
1422942ec: 8b85b8020000                 mov eax, dword ptr [rbp + 0x2b8]
1422942f2: 488b8db0000000               mov rcx, qword ptr [rbp + 0xb0]
1422942f9: 8901                         mov dword ptr [rcx], eax
1422942fb: 488b85e0010000               mov rax, qword ptr [rbp + 0x1e0]
142294302: 488b7030                     mov rsi, qword ptr [rax + 0x30]
142294306: 488b7840                     mov rdi, qword ptr [rax + 0x40]
14229430a: 483b7020                     cmp rsi, qword ptr [rax + 0x20]
14229430e: 751a                         jne 0x14229432a
142294310: 488b8dd8000000               mov rcx, qword ptr [rbp + 0xd8]
142294317: 488d15aa6fda00               lea rdx, [rip + 0xda6faa]
14229431e: e80de92cfe                   call 0x140562c30
142294323: 4c8b9d08070000               mov r11, qword ptr [rbp + 0x708]
14229432a: 4c8b8de0010000               mov r9, qword ptr [rbp + 0x1e0]
142294331: 498b4128                     mov rax, qword ptr [r9 + 0x28]
142294335: 4889f1                       mov rcx, rsi
142294338: 48c1e105                     shl rcx, 5
14229433c: 4c8d45b4                     lea r8, [rbp - 0x4c]
142294340: 498b5054                     mov rdx, qword ptr [r8 + 0x54]
142294344: 4889540810                   mov qword ptr [rax + rcx + 0x10], rdx
142294349: 410f104044                   movups xmm0, xmmword ptr [r8 + 0x44]
14229434e: 0f110408                     movups xmmword ptr [rax + rcx], xmm0
142294352: 48897c0818                   mov qword ptr [rax + rcx + 0x18], rdi
142294357: 48ffc6                       inc rsi
14229435a: 49897130                     mov qword ptr [r9 + 0x30], rsi
14229435e: 49ff4140                     inc qword ptr [r9 + 0x40]
142294362: 4c8b85b8060000               mov r8, qword ptr [rbp + 0x6b8]
142294369: 488b8dc0060000               mov rcx, qword ptr [rbp + 0x6c0]
142294370: 4c89ad80060000               mov qword ptr [rbp + 0x680], r13
142294377: e988daffff                   jmp 0x142291e04
14229437c: 4c89ad80060000               mov qword ptr [rbp + 0x680], r13
142294383: e97cdaffff                   jmp 0x142291e04
142294388: 4531f6                       xor r14d, r14d
14229438b: 4c8b85b8060000               mov r8, qword ptr [rbp + 0x6b8]
142294392: 488b8dc0060000               mov rcx, qword ptr [rbp + 0x6c0]
142294399: 80bd0803000000               cmp byte ptr [rbp + 0x308], 0
1422943a0: 0f856ffbffff                 jne 0x142293f15
1422943a6: 80bd8000000000               cmp byte ptr [rbp + 0x80], 0
1422943ad: 0f84e1feffff                 je 0x142294294
1422943b3: 4c89eb                       mov rbx, r13
1422943b6: 488b8500070000               mov rax, qword ptr [rbp + 0x700]
1422943bd: 488b8dd0040000               mov rcx, qword ptr [rbp + 0x4d0]
1422943c4: 8b3481                       mov esi, dword ptr [rcx + rax*4]
1422943c7: 4889f9                       mov rcx, rdi
1422943ca: e8e14b71fe                   call 0x1409a8fb0
1422943cf: 81fe09030000                 cmp esi, 0x309
1422943d5: 7526                         jne 0x1422943fd
1422943d7: c785380600000a000000         mov dword ptr [rbp + 0x638], 0xa
1422943e1: 4c89f9                       mov rcx, r15
1422943e4: 4889fa                       mov rdx, rdi
1422943e7: 4c8dad38060000               lea r13, [rbp + 0x638]
1422943ee: 4d89e8                       mov r8, r13
1422943f1: e8da4b71fe                   call 0x1409a8fd0
1422943f6: b80b000000                   mov eax, 0xb
1422943fb: eb4c                         jmp 0x142294449
1422943fd: c785380600000c000000         mov dword ptr [rbp + 0x638], 0xc
142294407: 8b455c                       mov eax, dword ptr [rbp + 0x5c]
14229440a: 488d8d3c060000               lea rcx, [rbp + 0x63c]
142294411: 894110                       mov dword ptr [rcx + 0x10], eax
142294414: 0f10454c                     movups xmm0, xmmword ptr [rbp + 0x4c]
142294418: 0f1101                       movups xmmword ptr [rcx], xmm0
14229441b: 4c89f9                       mov rcx, r15
14229441e: 4889fa                       mov rdx, rdi
142294421: 4c8dad38060000               lea r13, [rbp + 0x638]
142294428: 4d89e8                       mov r8, r13
14229442b: e8a04b71fe                   call 0x1409a8fd0
142294430: 8b4548                       mov eax, dword ptr [rbp + 0x48]
142294433: 488d8d34050000               lea rcx, [rbp + 0x534]
14229443a: 894110                       mov dword ptr [rcx + 0x10], eax
14229443d: 0f104538                     movups xmm0, xmmword ptr [rbp + 0x38]
142294441: 0f1101                       movups xmmword ptr [rcx], xmm0
142294444: b80d000000                   mov eax, 0xd
142294449: 898530050000                 mov dword ptr [rbp + 0x530], eax
14229444f: 488d4db0                     lea rcx, [rbp - 0x50]
142294453: 4c89fa                       mov rdx, r15
142294456: 4989f8                       mov r8, rdi
142294459: e8c24b71fe                   call 0x1409a9020
14229445e: 488d85e0060000               lea rax, [rbp + 0x6e0]
142294465: 488985e8030000               mov qword ptr [rbp + 0x3e8], rax
14229446c: 488d054d40e3fd               lea rax, [rip - 0x21cbfb3]
142294473: 488985f0030000               mov qword ptr [rbp + 0x3f0], rax
14229447a: 488d8da8060000               lea rcx, [rbp + 0x6a8]
142294481: 48898df8030000               mov qword ptr [rbp + 0x3f8], rcx
142294488: 48898500040000               mov qword ptr [rbp + 0x400], rax
14229448f: 488d053a75da00               lea rax, [rip + 0xda753a]
142294496: 48898530050000               mov qword ptr [rbp + 0x530], rax
14229449d: 48c7853805000002000000       mov qword ptr [rbp + 0x538], 2
1422944a8: 4c89bd40050000               mov qword ptr [rbp + 0x540], r15
1422944af: 48c7854805000002000000       mov qword ptr [rbp + 0x548], 2
1422944ba: 48c7855005000000000000       mov qword ptr [rbp + 0x550], 0
1422944c5: 4c89e9                       mov rcx, r13
1422944c8: 4889fa                       mov rdx, rdi
1422944cb: e8c05aaefe                   call 0x140d79f90
1422944d0: 4c8bb530060000               mov r14, qword ptr [rbp + 0x630]
1422944d7: f2410f1006                   movsd xmm0, qword ptr [r14]
1422944dc: f2410f104e08                 movsd xmm1, qword ptr [r14 + 8]
1422944e2: f20f12d0                     movddup xmm2, xmm0
1422944e6: 410f59cd                     mulps xmm1, xmm13
1422944ea: f20f12d9                     movddup xmm3, xmm1
1422944ee: 0f58da                       addps xmm3, xmm2
1422944f1: 0f5cc1                       subps xmm0, xmm1
1422944f4: 0fc6c3e4                     shufps xmm0, xmm3, 0xe4
1422944f8: 0f1185f0030000               movups xmmword ptr [rbp + 0x3f0], xmm0
1422944ff: c785ec03000000000000         mov dword ptr [rbp + 0x3ec], 0
142294509: c685e803000003               mov byte ptr [rbp + 0x3e8], 3
142294510: c6851406000000               mov byte ptr [rbp + 0x614], 0
142294517: c685dd06000001               mov byte ptr [rbp + 0x6dd], 1
14229451e: 4889f9                       mov rcx, rdi
142294521: 4c89fa                       mov rdx, r15
142294524: 4c8d8514060000               lea r8, [rbp + 0x614]
14229452b: e8e04871fe                   call 0x1409a8e10
142294530: c685dd06000001               mov byte ptr [rbp + 0x6dd], 1
142294537: 4c89f9                       mov rcx, r15
14229453a: 4889fa                       mov rdx, rdi
14229453d: 4531c0                       xor r8d, r8d
142294540: e89b4971fe                   call 0x1409a8ee0
142294545: c685dd06000000               mov byte ptr [rbp + 0x6dd], 0
14229454c: 488b8d50060000               mov rcx, qword ptr [rbp + 0x650]
142294553: 488d9538060000               lea rdx, [rbp + 0x638]
14229455a: 4d89f8                       mov r8, r15
14229455d: e8be8b0200                   call 0x1422bd120
142294562: 48898530050000               mov qword ptr [rbp + 0x530], rax
142294569: 48899538050000               mov qword ptr [rbp + 0x538], rdx
142294570: 4889f9                       mov rcx, rdi
142294573: e808a169fe                   call 0x14092e680
142294578: 84c0                         test al, al
14229457a: 0f8441010000                 je 0x1422946c1
142294580: 488b85a8060000               mov rax, qword ptr [rbp + 0x6a8]
142294587: 48898558060000               mov qword ptr [rbp + 0x658], rax
14229458e: 8b85f8060000                 mov eax, dword ptr [rbp + 0x6f8]
142294594: 488b8d00070000               mov rcx, qword ptr [rbp + 0x700]
14229459b: 488b95d0060000               mov rdx, qword ptr [rbp + 0x6d0]
1422945a2: 89048a                       mov dword ptr [rdx + rcx*4], eax
1422945a5: 488b85b0060000               mov rax, qword ptr [rbp + 0x6b0]
1422945ac: c6402001                     mov byte ptr [rax + 0x20], 1
1422945b0: 41bd01000000                 mov r13d, 1
1422945b6: 4c8b8508070000               mov r8, qword ptr [rbp + 0x708]
1422945bd: 81fe09030000                 cmp esi, 0x309
1422945c3: 7576                         jne 0x14229463b
1422945c5: 488b8520030000               mov rax, qword ptr [rbp + 0x320]
1422945cc: f20f1030                     movsd xmm6, qword ptr [rax]
1422945d0: 488b85f8000000               mov rax, qword ptr [rbp + 0xf8]
1422945d7: 8b8dc4020000                 mov ecx, dword ptr [rbp + 0x2c4]
1422945dd: 8908                         mov dword ptr [rax], ecx
1422945df: 488b8548020000               mov rax, qword ptr [rbp + 0x248]
1422945e6: 488b7830                     mov rdi, qword ptr [rax + 0x30]
1422945ea: 488b5840                     mov rbx, qword ptr [rax + 0x40]
1422945ee: 483b7820                     cmp rdi, qword ptr [rax + 0x20]
1422945f2: 751a                         jne 0x14229460e
1422945f4: 488b8d38010000               mov rcx, qword ptr [rbp + 0x138]
1422945fb: 488d15c66cda00               lea rdx, [rip + 0xda6cc6]
142294602: e829e02cfe                   call 0x140562630
142294607: 4c8b8508070000               mov r8, qword ptr [rbp + 0x708]
14229460e: 488b9548020000               mov rdx, qword ptr [rbp + 0x248]
142294615: 488b4228                     mov rax, qword ptr [rdx + 0x28]
142294619: 4889f9                       mov rcx, rdi
14229461c: 48c1e104                     shl rcx, 4
142294620: 48891c08                     mov qword ptr [rax + rcx], rbx
142294624: 0f13740808                   movlps qword ptr [rax + rcx + 8], xmm6
142294629: 48ffc7                       inc rdi
14229462c: 48897a30                     mov qword ptr [rdx + 0x30], rdi
142294630: 48ff4240                     inc qword ptr [rdx + 0x40]
142294634: 488dbd30050000               lea rdi, [rbp + 0x530]
14229463b: b801000000                   mov eax, 1
142294640: 488985c8010000               mov qword ptr [rbp + 0x1c8], rax
142294647: 4180783004                   cmp byte ptr [r8 + 0x30], 4
14229464c: 0f8583000000                 jne 0x1422946d5
142294652: 418b5834                     mov ebx, dword ptr [r8 + 0x34]
142294656: 488b8500010000               mov rax, qword ptr [rbp + 0x100]
14229465d: 8b8dc8020000                 mov ecx, dword ptr [rbp + 0x2c8]
142294663: 8908                         mov dword ptr [rax], ecx
142294665: 488b8550020000               mov rax, qword ptr [rbp + 0x250]
14229466c: 488b7830                     mov rdi, qword ptr [rax + 0x30]
142294670: 4c8b7840                     mov r15, qword ptr [rax + 0x40]
142294674: 483b7820                     cmp rdi, qword ptr [rax + 0x20]
142294678: 7513                         jne 0x14229468d
14229467a: 488b8d40010000               mov rcx, qword ptr [rbp + 0x140]
142294681: 488d15406cda00               lea rdx, [rip + 0xda6c40]
142294688: e8a3df2cfe                   call 0x140562630
14229468d: 488b9550020000               mov rdx, qword ptr [rbp + 0x250]
142294694: 488b4228                     mov rax, qword ptr [rdx + 0x28]
142294698: 4889f9                       mov rcx, rdi
14229469b: 48c1e104                     shl rcx, 4
14229469f: 4c893c08                     mov qword ptr [rax + rcx], r15
1422946a3: 895c0808                     mov dword ptr [rax + rcx + 8], ebx
1422946a7: 48ffc7                       inc rdi
1422946aa: 48897a30                     mov qword ptr [rdx + 0x30], rdi
1422946ae: 48ff4240                     inc qword ptr [rdx + 0x40]
1422946b2: 89b5a8040000                 mov dword ptr [rbp + 0x4a8], esi
1422946b8: 488dbd30050000               lea rdi, [rbp + 0x530]
1422946bf: eb6d                         jmp 0x14229472e
1422946c1: 4885db                       test rbx, rbx
1422946c4: 4c8b8508070000               mov r8, qword ptr [rbp + 0x708]
1422946cb: 7410                         je 0x1422946dd
1422946cd: 41bd01000000                 mov r13d, 1
1422946d3: eb59                         jmp 0x14229472e
1422946d5: 89b5a8040000                 mov dword ptr [rbp + 0x4a8], esi
1422946db: eb51                         jmp 0x14229472e
1422946dd: 80bdcc06000000               cmp byte ptr [rbp + 0x6cc], 0
1422946e4: 7435                         je 0x14229471b
1422946e6: 41f60001                     test byte ptr [r8], 1
1422946ea: 742f                         je 0x14229471b
1422946ec: 498b4008                     mov rax, qword ptr [r8 + 8]
1422946f0: 483985e0060000               cmp qword ptr [rbp + 0x6e0], rax
1422946f7: 7522                         jne 0x14229471b
1422946f9: 8b85f8060000                 mov eax, dword ptr [rbp + 0x6f8]
1422946ff: 488b8d00070000               mov rcx, qword ptr [rbp + 0x700]
142294706: 488b95d0060000               mov rdx, qword ptr [rbp + 0x6d0]
14229470d: 89048a                       mov dword ptr [rdx + rcx*4], eax
142294710: 488b85b0060000               mov rax, qword ptr [rbp + 0x6b0]
142294717: c6402001                     mov byte ptr [rax + 0x20], 1
14229471b: 41bd00000000                 mov r13d, 0
142294721: 80bd1707000000               cmp byte ptr [rbp + 0x717], 0
142294728: 0f858ffeffff                 jne 0x1422945bd
14229472e: 4889f9                       mov rcx, rdi
142294731: e8ba9e69fe                   call 0x14092e5f0
142294736: 41b601                       mov r14b, 1
142294739: b101                         mov cl, 1
14229473b: 84c0                         test al, al
14229473d: 7553                         jne 0x142294792
14229473f: 488b8560060000               mov rax, qword ptr [rbp + 0x660]
142294746: 8b8898000000                 mov ecx, dword ptr [rax + 0x98]
14229474c: f30f10889c000000             movss xmm1, dword ptr [rax + 0x9c]
142294754: e837b36afe                   call 0x14093fa90
142294759: 84c0                         test al, al
14229475b: 742c                         je 0x142294789
14229475d: 488b8508070000               mov rax, qword ptr [rbp + 0x708]
142294764: 833801                       cmp dword ptr [rax], 1
142294767: 7520                         jne 0x142294789
142294769: 488b4008                     mov rax, qword ptr [rax + 8]
14229476d: 483b85e0060000               cmp rax, qword ptr [rbp + 0x6e0]
142294774: 410f94c6                     sete r14b
142294778: 0fb68dec060000               movzx ecx, byte ptr [rbp + 0x6ec]
14229477f: b801000000                   mov eax, 1
142294784: 0f44c8                       cmove ecx, eax
142294787: eb09                         jmp 0x142294792
142294789: 4531f6                       xor r14d, r14d
14229478c: 8b8dec060000                 mov ecx, dword ptr [rbp + 0x6ec]
142294792: 898dec060000                 mov dword ptr [rbp + 0x6ec], ecx
142294798: 4c8b9d08070000               mov r11, qword ptr [rbp + 0x708]
14229479f: 4c8b85b8060000               mov r8, qword ptr [rbp + 0x6b8]
1422947a6: 488b8dc0060000               mov rcx, qword ptr [rbp + 0x6c0]
1422947ad: 488b85b0060000               mov rax, qword ptr [rbp + 0x6b0]
1422947b4: 80782000                     cmp byte ptr [rax + 0x20], 0
1422947b8: 0f85e3faffff                 jne 0x1422942a1
1422947be: e920fbffff                   jmp 0x1422942e3
1422947c3: 4531f6                       xor r14d, r14d
1422947c6: 4c8b85b8060000               mov r8, qword ptr [rbp + 0x6b8]
1422947cd: 488b8dc0060000               mov rcx, qword ptr [rbp + 0x6c0]
1422947d4: 80bd8000000000               cmp byte ptr [rbp + 0x80], 0
1422947db: 0f84b3faffff                 je 0x142294294
1422947e1: e9cdfbffff                   jmp 0x1422943b3
1422947e6: 662e0f1f840000000000         nop word ptr cs:[rax + rax]
1422947f0: 488b8560060000               mov rax, qword ptr [rbp + 0x660]
1422947f7: 8b8898000000                 mov ecx, dword ptr [rax + 0x98]
1422947fd: f30f10889c000000             movss xmm1, dword ptr [rax + 0x9c]
142294805: e886b26afe                   call 0x14093fa90
14229480a: 84c0                         test al, al
14229480c: 488d9de8030000               lea rbx, [rbp + 0x3e8]
142294813: 0f84f7020000                 je 0x142294b10
142294819: f685f405000001               test byte ptr [rbp + 0x5f4], 1
142294820: 4c8b8d08070000               mov r9, qword ptr [rbp + 0x708]
142294827: 440fb6b5de060000             movzx r14d, byte ptr [rbp + 0x6de]
14229482f: 7540                         jne 0x142294871
142294831: 488b8568060000               mov rax, qword ptr [rbp + 0x668]
142294838: 8b8dd8060000                 mov ecx, dword ptr [rbp + 0x6d8]
14229483e: 8908                         mov dword ptr [rax], ecx
142294840: 488b8d70060000               mov rcx, qword ptr [rbp + 0x670]
142294847: 488d5510                     lea rdx, [rbp + 0x10]
14229484b: e8c01c7ffe                   call 0x140a86510
142294850: 4c8b8d08070000               mov r9, qword ptr [rbp + 0x708]
142294857: 31ff                         xor edi, edi
142294859: c7850007000000000000         mov dword ptr [rbp + 0x700], 0
142294863: c785f405000000000000         mov dword ptr [rbp + 0x5f4], 0
14229486d: 84c0                         test al, al
14229486f: 7475                         je 0x1422948e6
142294871: 488b85b8000000               mov rax, qword ptr [rbp + 0xb8]
142294878: 8b8dbc020000                 mov ecx, dword ptr [rbp + 0x2bc]
14229487e: 8908                         mov dword ptr [rax], ecx
142294880: 488b85e8010000               mov rax, qword ptr [rbp + 0x1e8]
142294887: 488b7030                     mov rsi, qword ptr [rax + 0x30]
14229488b: 488b7840                     mov rdi, qword ptr [rax + 0x40]
14229488f: 483b7020                     cmp rsi, qword ptr [rax + 0x20]
142294893: 751a                         jne 0x1422948af
142294895: 488b8de8000000               mov rcx, qword ptr [rbp + 0xe8]
14229489c: 488d15cd22ea00               lea rdx, [rip + 0xea22cd]
1422948a3: e8c8df2cfe                   call 0x140562870
1422948a8: 4c8b8d08070000               mov r9, qword ptr [rbp + 0x708]
1422948af: 488b8de8010000               mov rcx, qword ptr [rbp + 0x1e8]
1422948b6: 488b4128                     mov rax, qword ptr [rcx + 0x28]
1422948ba: 48893cf0                     mov qword ptr [rax + rsi*8], rdi
1422948be: 48ffc6                       inc rsi
1422948c1: 48897130                     mov qword ptr [rcx + 0x30], rsi
1422948c5: 48ff4140                     inc qword ptr [rcx + 0x40]
1422948c9: b001                         mov al, 1
1422948cb: 8985f4050000                 mov dword ptr [rbp + 0x5f4], eax
1422948d1: 488b8528030000               mov rax, qword ptr [rbp + 0x328]
1422948d8: 89c7                         mov edi, eax
1422948da: 8b85ec060000                 mov eax, dword ptr [rbp + 0x6ec]
1422948e0: 898500070000                 mov dword ptr [rbp + 0x700], eax
1422948e6: 410fb64130                   movzx eax, byte ptr [r9 + 0x30]
1422948eb: 41b501                       mov r13b, 1
1422948ee: 3c0a                         cmp al, 0xa
1422948f0: 0f854e020000                 jne 0x142294b44
1422948f6: 41b501                       mov r13b, 1
1422948f9: f6853003000001               test byte ptr [rbp + 0x330], 1
142294900: 0f84fa050000                 je 0x142294f00
142294906: 418b7134                     mov esi, dword ptr [r9 + 0x34]
14229490a: 41c7412800000000             mov dword ptr [r9 + 0x28], 0
142294912: f6850007000001               test byte ptr [rbp + 0x700], 1
142294919: 0f8405060000                 je 0x142294f24
14229491f: 450fb6b989000000             movzx r15d, byte ptr [r9 + 0x89]
142294927: 8b85cc020000                 mov eax, dword ptr [rbp + 0x2cc]
14229492d: 488b8d20010000               mov rcx, qword ptr [rbp + 0x120]
142294934: 8901                         mov dword ptr [rcx], eax
142294936: 488b8570020000               mov rax, qword ptr [rbp + 0x270]
14229493d: 488b5830                     mov rbx, qword ptr [rax + 0x30]
142294941: 4c8b6040                     mov r12, qword ptr [rax + 0x40]
142294945: 483b5820                     cmp rbx, qword ptr [rax + 0x20]
142294949: 751a                         jne 0x142294965
14229494b: 488b8d48010000               mov rcx, qword ptr [rbp + 0x148]
142294952: 488d156f69da00               lea rdx, [rip + 0xda696f]
142294959: e8d2dc2cfe                   call 0x140562630
14229495e: 4c8b8d08070000               mov r9, qword ptr [rbp + 0x708]
142294965: 4183f701                     xor r15d, 1
142294969: 49c1e720                     shl r15, 0x20
14229496d: 4909f7                       or r15, rsi
142294970: 48b80000000000010000         movabs rax, 0x10000000000
14229497a: 4909c7                       or r15, rax
14229497d: 488b9570020000               mov rdx, qword ptr [rbp + 0x270]
142294984: 488b4228                     mov rax, qword ptr [rdx + 0x28]
142294988: 4889d9                       mov rcx, rbx
14229498b: 48c1e104                     shl rcx, 4
14229498f: 4c892408                     mov qword ptr [rax + rcx], r12
142294993: 4c897c0808                   mov qword ptr [rax + rcx + 8], r15
142294998: 48ffc3                       inc rbx
14229499b: 48895a30                     mov qword ptr [rdx + 0x30], rbx
14229499f: 48ff4240                     inc qword ptr [rdx + 0x40]
1422949a3: 488b8568060000               mov rax, qword ptr [rbp + 0x668]
1422949aa: 8b8dd8060000                 mov ecx, dword ptr [rbp + 0x6d8]
1422949b0: 8908                         mov dword ptr [rax], ecx
1422949b2: 410fb69989000000             movzx ebx, byte ptr [r9 + 0x89]
1422949ba: 0fb60548023a01               movzx eax, byte ptr [rip + 0x13a0248]
1422949c1: b930000000                   mov ecx, 0x30
1422949c6: ba08000000                   mov edx, 8
1422949cb: 4d89cf                       mov r15, r9
1422949ce: e83df237fe                   call 0x140613c10
1422949d3: 4885c0                       test rax, rax
1422949d6: 0f849c220000                 je 0x142296c78
1422949dc: 48c70001000000               mov qword ptr [rax], 1
1422949e3: 48c7400801000000             mov qword ptr [rax + 8], 1
1422949eb: 897010                       mov dword ptr [rax + 0x10], esi
1422949ee: 885814                       mov byte ptr [rax + 0x14], bl
1422949f1: 80f301                       xor bl, 1
1422949f4: 885815                       mov byte ptr [rax + 0x15], bl
1422949f7: c640180c                     mov byte ptr [rax + 0x18], 0xc
1422949fb: 48898538050000               mov qword ptr [rbp + 0x538], rax
142294a02: 48c7853005000009000000       mov qword ptr [rbp + 0x530], 9
142294a0d: 488b8d70060000               mov rcx, qword ptr [rbp + 0x670]
142294a14: 488d9530050000               lea rdx, [rbp + 0x530]
142294a1b: 4c8d05ae6ada00               lea r8, [rip + 0xda6aae]
142294a22: e899047ffe                   call 0x140a84ec0
142294a27: 410fb64730                   movzx eax, byte ptr [r15 + 0x30]
142294a2c: 4531ed                       xor r13d, r13d
142294a2f: 488d9de8030000               lea rbx, [rbp + 0x3e8]
142294a36: 4d89f9                       mov r9, r15
142294a39: 3c06                         cmp al, 6
142294a3b: 0f840b010000                 je 0x142294b4c
142294a41: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
142294a50: 0fb68dff060000               movzx ecx, byte ptr [rbp + 0x6ff]
142294a57: 4180793009                   cmp byte ptr [r9 + 0x30], 9
142294a5c: 0f859e040000                 jne 0x142294f00
142294a62: 458b7934                     mov r15d, dword ptr [r9 + 0x34]
142294a66: 89fb                         mov ebx, edi
142294a68: 80e301                       and bl, 1
142294a6b: 80f903                       cmp cl, 3
142294a6e: 0f85b4020000                 jne 0x142294d28
142294a74: f3410f10716c                 movss xmm6, dword ptr [r9 + 0x6c]
142294a7a: 488b8518020000               mov rax, qword ptr [rbp + 0x218]
142294a81: 8b8dac040000                 mov ecx, dword ptr [rbp + 0x4ac]
142294a87: 8908                         mov dword ptr [rax], ecx
142294a89: 488b85d8030000               mov rax, qword ptr [rbp + 0x3d8]
142294a90: 488b7030                     mov rsi, qword ptr [rax + 0x30]
142294a94: 488b7840                     mov rdi, qword ptr [rax + 0x40]
142294a98: 483b7020                     cmp rsi, qword ptr [rax + 0x20]
142294a9c: 751a                         jne 0x142294ab8
142294a9e: 488b8d78020000               mov rcx, qword ptr [rbp + 0x278]
142294aa5: 488d151c68da00               lea rdx, [rip + 0xda681c]
142294aac: e8ffe22cfe                   call 0x140562db0
142294ab1: 4c8b8d08070000               mov r9, qword ptr [rbp + 0x708]
142294ab8: 488b95d8030000               mov rdx, qword ptr [rbp + 0x3d8]
142294abf: 488b4228                     mov rax, qword ptr [rdx + 0x28]
142294ac3: 488d0c76                     lea rcx, [rsi + rsi*2]
142294ac7: 48893cc8                     mov qword ptr [rax + rcx*8], rdi
142294acb: 44897cc808                   mov dword ptr [rax + rcx*8 + 8], r15d
142294ad0: f30f1174c80c                 movss dword ptr [rax + rcx*8 + 0xc], xmm6
142294ad6: 885cc810                     mov byte ptr [rax + rcx*8 + 0x10], bl
142294ada: 48ffc6                       inc rsi
142294add: 48897230                     mov qword ptr [rdx + 0x30], rsi
142294ae1: 48ff4240                     inc qword ptr [rdx + 0x40]
142294ae5: 488d9de8030000               lea rbx, [rbp + 0x3e8]
142294aec: 83bdc801000001               cmp dword ptr [rbp + 0x1c8], 1
142294af3: 4489add0060000               mov dword ptr [rbp + 0x6d0], r13d
142294afa: 0f8560040000                 jne 0x142294f60
142294b00: e90b040000                   jmp 0x142294f10
142294b05: 66662e0f1f840000000000       nop word ptr cs:[rax + rax]
142294b10: 488b8528030000               mov rax, qword ptr [rbp + 0x328]
142294b17: 89c7                         mov edi, eax
142294b19: 8b85ec060000                 mov eax, dword ptr [rbp + 0x6ec]
142294b1f: 898500070000                 mov dword ptr [rbp + 0x700], eax
142294b25: 4c8b8d08070000               mov r9, qword ptr [rbp + 0x708]
142294b2c: 440fb6b5de060000             movzx r14d, byte ptr [rbp + 0x6de]
142294b34: 410fb64130                   movzx eax, byte ptr [r9 + 0x30]
142294b39: 41b501                       mov r13b, 1
142294b3c: 3c0a                         cmp al, 0xa
142294b3e: 0f84b2fdffff                 je 0x1422948f6
142294b44: 3c06                         cmp al, 6
142294b46: 0f8504ffffff                 jne 0x142294a50
142294b4c: 458b7934                     mov r15d, dword ptr [r9 + 0x34]
142294b50: 410fb69986000000             movzx ebx, byte ptr [r9 + 0x86]
142294b58: f6854003000001               test byte ptr [rbp + 0x340], 1
142294b5f: 0f84fb020000                 je 0x142294e60
142294b65: 488b8538020000               mov rax, qword ptr [rbp + 0x238]
142294b6c: 8b8db8040000                 mov ecx, dword ptr [rbp + 0x4b8]
142294b72: 8908                         mov dword ptr [rax], ecx
142294b74: 488b85e0030000               mov rax, qword ptr [rbp + 0x3e0]
142294b7b: 488b7030                     mov rsi, qword ptr [rax + 0x30]
142294b7f: 4c8b6040                     mov r12, qword ptr [rax + 0x40]
142294b83: 483b7020                     cmp rsi, qword ptr [rax + 0x20]
142294b87: 751a                         jne 0x142294ba3
142294b89: 488b8d98020000               mov rcx, qword ptr [rbp + 0x298]
142294b90: 488d153167da00               lea rdx, [rip + 0xda6731]
142294b97: e894da2cfe                   call 0x140562630
142294b9c: 4c8b8d08070000               mov r9, qword ptr [rbp + 0x708]
142294ba3: 80f301                       xor bl, 1
142294ba6: 448b9500070000               mov r10d, dword ptr [rbp + 0x700]
142294bad: 4489d0                       mov eax, r10d
142294bb0: 2401                         and al, 1
142294bb2: 0fb6c0                       movzx eax, al
142294bb5: 48c1e028                     shl rax, 0x28
142294bb9: 0fb6cb                       movzx ecx, bl
142294bbc: 48c1e120                     shl rcx, 0x20
142294bc0: 4809c1                       or rcx, rax
142294bc3: 4c09f9                       or rcx, r15
142294bc6: 4c8b85e0030000               mov r8, qword ptr [rbp + 0x3e0]
142294bcd: 498b4028                     mov rax, qword ptr [r8 + 0x28]
142294bd1: 4889f2                       mov rdx, rsi
142294bd4: 48c1e204                     shl rdx, 4
142294bd8: 4c892410                     mov qword ptr [rax + rdx], r12
142294bdc: 48894c1008                   mov qword ptr [rax + rdx + 8], rcx
142294be1: 48ffc6                       inc rsi
142294be4: 49897030                     mov qword ptr [r8 + 0x30], rsi
142294be8: 49ff4040                     inc qword ptr [r8 + 0x40]
142294bec: 41c7412800000000             mov dword ptr [r9 + 0x28], 0
142294bf4: 41f6c201                     test r10b, 1
142294bf8: 0f84f5120000                 je 0x142295ef3
142294bfe: 488b8568060000               mov rax, qword ptr [rbp + 0x668]
142294c05: 8b8dd8060000                 mov ecx, dword ptr [rbp + 0x6d8]
142294c0b: 8908                         mov dword ptr [rax], ecx
142294c0d: 410fb69986000000             movzx ebx, byte ptr [r9 + 0x86]
142294c15: 0fb605edff3901               movzx eax, byte ptr [rip + 0x139ffed]
142294c1c: b930000000                   mov ecx, 0x30
142294c21: ba08000000                   mov edx, 8
142294c26: e8e5ef37fe                   call 0x140613c10
142294c2b: 4885c0                       test rax, rax
142294c2e: 0f8444200000                 je 0x142296c78
142294c34: 48c70001000000               mov qword ptr [rax], 1
142294c3b: 48c7400801000000             mov qword ptr [rax + 8], 1
142294c43: 44897810                     mov dword ptr [rax + 0x10], r15d
142294c47: 885814                       mov byte ptr [rax + 0x14], bl
142294c4a: 80f301                       xor bl, 1
142294c4d: 885815                       mov byte ptr [rax + 0x15], bl
142294c50: c640180a                     mov byte ptr [rax + 0x18], 0xa
142294c54: 48898538050000               mov qword ptr [rbp + 0x538], rax
142294c5b: 48c7853005000009000000       mov qword ptr [rbp + 0x530], 9
142294c66: 488b8d70060000               mov rcx, qword ptr [rbp + 0x670]
142294c6d: 488d9530050000               lea rdx, [rbp + 0x530]
142294c74: 4c8d056d68da00               lea r8, [rip + 0xda686d]
142294c7b: e840027ffe                   call 0x140a84ec0
142294c80: 4531ed                       xor r13d, r13d
142294c83: 488d9de8030000               lea rbx, [rbp + 0x3e8]
142294c8a: 4c8b8d08070000               mov r9, qword ptr [rbp + 0x708]
142294c91: 0fb68dff060000               movzx ecx, byte ptr [rbp + 0x6ff]
142294c98: 4180793009                   cmp byte ptr [r9 + 0x30], 9
142294c9d: 0f84bffdffff                 je 0x142294a62
142294ca3: e958020000                   jmp 0x142294f00
142294ca8: b001                         mov al, 1
142294caa: 8985cc060000                 mov dword ptr [rbp + 0x6cc], eax
142294cb0: 84c9                         test cl, cl
142294cb2: 0f84e2cfffff                 je 0x142291c9a
142294cb8: 498b4b20                     mov rcx, qword ptr [r11 + 0x20]
142294cbc: 4c8d49ff                     lea r9, [rcx - 1]
142294cc0: 4c8d5101                     lea r10, [rcx + 1]
142294cc4: eb31                         jmp 0x142294cf7
142294cc6: 662e0f1f840000000000         nop word ptr cs:[rax + rax]
142294cd0: 31d2                         xor edx, edx
142294cd2: f7f1                         div ecx
142294cd4: 498910                       mov qword ptr [r8], rdx
142294cd7: 498b4348                     mov rax, qword ptr [r11 + 0x48]
142294cdb: 4839c2                       cmp rdx, rax
142294cde: 0f836c1f0000                 jae 0x142296c50
142294ce4: 498b4340                     mov rax, qword ptr [r11 + 0x40]
142294ce8: 488d1492                     lea rdx, [rdx + rdx*4]
142294cec: 807cd01805                   cmp byte ptr [rax + rdx*8 + 0x18], 5
142294cf1: 0f85a3cfffff                 jne 0x142291c9a
142294cf7: 49ffca                       dec r10
142294cfa: 0f849acfffff                 je 0x142291c9a
142294d00: 498b00                       mov rax, qword ptr [r8]
142294d03: 4c01c8                       add rax, r9
142294d06: 4889c2                       mov rdx, rax
142294d09: 4809ca                       or rdx, rcx
142294d0c: 48c1ea20                     shr rdx, 0x20
142294d10: 74be                         je 0x142294cd0
142294d12: 31d2                         xor edx, edx
142294d14: 48f7f1                       div rcx
142294d17: 498910                       mov qword ptr [r8], rdx
142294d1a: 498b4348                     mov rax, qword ptr [r11 + 0x48]
142294d1e: 4839c2                       cmp rdx, rax
142294d21: 72c1                         jb 0x142294ce4
142294d23: e9281f0000                   jmp 0x142296c50
142294d28: e8431a87fe                   call 0x140b06770
142294d2d: 0f28f0                       movaps xmm6, xmm0
142294d30: 488b8518020000               mov rax, qword ptr [rbp + 0x218]
142294d37: 8b8dac040000                 mov ecx, dword ptr [rbp + 0x4ac]
142294d3d: 8908                         mov dword ptr [rax], ecx
142294d3f: 488b85d8030000               mov rax, qword ptr [rbp + 0x3d8]
142294d46: 488b7030                     mov rsi, qword ptr [rax + 0x30]
142294d4a: 4c8b6040                     mov r12, qword ptr [rax + 0x40]
142294d4e: 483b7020                     cmp rsi, qword ptr [rax + 0x20]
142294d52: 7513                         jne 0x142294d67
142294d54: 488b8d78020000               mov rcx, qword ptr [rbp + 0x278]
142294d5b: 488d156665da00               lea rdx, [rip + 0xda6566]
142294d62: e849e02cfe                   call 0x140562db0
142294d67: 488b95d8030000               mov rdx, qword ptr [rbp + 0x3d8]
142294d6e: 488b4228                     mov rax, qword ptr [rdx + 0x28]
142294d72: 488d0c76                     lea rcx, [rsi + rsi*2]
142294d76: 4c8924c8                     mov qword ptr [rax + rcx*8], r12
142294d7a: 44897cc808                   mov dword ptr [rax + rcx*8 + 8], r15d
142294d7f: f30f1174c80c                 movss dword ptr [rax + rcx*8 + 0xc], xmm6
142294d85: 885cc810                     mov byte ptr [rax + rcx*8 + 0x10], bl
142294d89: 48ffc6                       inc rsi
142294d8c: 48897230                     mov qword ptr [rdx + 0x30], rsi
142294d90: 48ff4240                     inc qword ptr [rdx + 0x40]
142294d94: 40f6c701                     test dil, 1
142294d98: 4c8b8d08070000               mov r9, qword ptr [rbp + 0x708]
142294d9f: 0f8495010000                 je 0x142294f3a
142294da5: 488b8568060000               mov rax, qword ptr [rbp + 0x668]
142294dac: 8b8dd8060000                 mov ecx, dword ptr [rbp + 0x6d8]
142294db2: 8908                         mov dword ptr [rax], ecx
142294db4: f3450f10796c                 movss xmm15, dword ptr [r9 + 0x6c]
142294dba: 0fb68dff060000               movzx ecx, byte ptr [rbp + 0x6ff]
142294dc1: e8aa1987fe                   call 0x140b06770
142294dc6: 0f28f0                       movaps xmm6, xmm0
142294dc9: 0fb60539fe3901               movzx eax, byte ptr [rip + 0x139fe39]
142294dd0: b930000000                   mov ecx, 0x30
142294dd5: ba08000000                   mov edx, 8
142294dda: e831ee37fe                   call 0x140613c10
142294ddf: 4885c0                       test rax, rax
142294de2: 488d9530050000               lea rdx, [rbp + 0x530]
142294de9: 488d9de8030000               lea rbx, [rbp + 0x3e8]
142294df0: 0f84821e0000                 je 0x142296c78
142294df6: 48c70001000000               mov qword ptr [rax], 1
142294dfd: 48c7400801000000             mov qword ptr [rax + 8], 1
142294e05: c640180b                     mov byte ptr [rax + 0x18], 0xb
142294e09: 4489781c                     mov dword ptr [rax + 0x1c], r15d
142294e0d: f3440f117820                 movss dword ptr [rax + 0x20], xmm15
142294e13: f30f117024                   movss dword ptr [rax + 0x24], xmm6
142294e18: 48898538050000               mov qword ptr [rbp + 0x538], rax
142294e1f: 48c7853005000009000000       mov qword ptr [rbp + 0x530], 9
142294e2a: 488b8d70060000               mov rcx, qword ptr [rbp + 0x670]
142294e31: 4c8d05c866da00               lea r8, [rip + 0xda66c8]
142294e38: e883007ffe                   call 0x140a84ec0
142294e3d: 4531ed                       xor r13d, r13d
142294e40: 4c8b8d08070000               mov r9, qword ptr [rbp + 0x708]
142294e47: 83bdc801000001               cmp dword ptr [rbp + 0x1c8], 1
142294e4e: 4489add0060000               mov dword ptr [rbp + 0x6d0], r13d
142294e55: 0f8505010000                 jne 0x142294f60
142294e5b: e9b0000000                   jmp 0x142294f10
142294e60: 488b8538020000               mov rax, qword ptr [rbp + 0x238]
142294e67: 8b8db8040000                 mov ecx, dword ptr [rbp + 0x4b8]
142294e6d: 8908                         mov dword ptr [rax], ecx
142294e6f: 488b85e0030000               mov rax, qword ptr [rbp + 0x3e0]
142294e76: 488b7030                     mov rsi, qword ptr [rax + 0x30]
142294e7a: 4c8b6040                     mov r12, qword ptr [rax + 0x40]
142294e7e: 483b7020                     cmp rsi, qword ptr [rax + 0x20]
142294e82: 751a                         jne 0x142294e9e
142294e84: 488b8d98020000               mov rcx, qword ptr [rbp + 0x298]
142294e8b: 488d153664da00               lea rdx, [rip + 0xda6436]
142294e92: e899d72cfe                   call 0x140562630
142294e97: 4c8b8d08070000               mov r9, qword ptr [rbp + 0x708]
142294e9e: 0fb6c3                       movzx eax, bl
142294ea1: 8b8d00070000                 mov ecx, dword ptr [rbp + 0x700]
142294ea7: 80e101                       and cl, 1
142294eaa: 0fb6c9                       movzx ecx, cl
142294ead: 48c1e128                     shl rcx, 0x28
142294eb1: 48c1e020                     shl rax, 0x20
142294eb5: 4809c8                       or rax, rcx
142294eb8: 4c09f8                       or rax, r15
142294ebb: 4c8b85e0030000               mov r8, qword ptr [rbp + 0x3e0]
142294ec2: 498b4828                     mov rcx, qword ptr [r8 + 0x28]
142294ec6: 4889f2                       mov rdx, rsi
142294ec9: 48c1e204                     shl rdx, 4
142294ecd: 4c892411                     mov qword ptr [rcx + rdx], r12
142294ed1: 4889441108                   mov qword ptr [rcx + rdx + 8], rax
142294ed6: 48ffc6                       inc rsi
142294ed9: 49897030                     mov qword ptr [r8 + 0x30], rsi
142294edd: 49ff4040                     inc qword ptr [r8 + 0x40]
142294ee1: 488d9de8030000               lea rbx, [rbp + 0x3e8]
142294ee8: 0fb68dff060000               movzx ecx, byte ptr [rbp + 0x6ff]
142294eef: 4180793009                   cmp byte ptr [r9 + 0x30], 9
142294ef4: 0f8468fbffff                 je 0x142294a62
142294efa: 660f1f440000                 nop word ptr [rax + rax]
142294f00: 83bdc801000001               cmp dword ptr [rbp + 0x1c8], 1
142294f07: 4489add0060000               mov dword ptr [rbp + 0x6d0], r13d
142294f0e: 7550                         jne 0x142294f60
142294f10: 41c7412801000000             mov dword ptr [r9 + 0x28], 1
142294f18: 8b85a8040000                 mov eax, dword ptr [rbp + 0x4a8]
142294f1e: 4189412c                     mov dword ptr [r9 + 0x2c], eax
142294f22: eb4d                         jmp 0x142294f71
142294f24: 4531ed                       xor r13d, r13d
142294f27: 0fb68dff060000               movzx ecx, byte ptr [rbp + 0x6ff]
142294f2e: 4180793009                   cmp byte ptr [r9 + 0x30], 9
142294f33: 75cb                         jne 0x142294f00
142294f35: e928fbffff                   jmp 0x142294a62
142294f3a: 4531ed                       xor r13d, r13d
142294f3d: 488d9de8030000               lea rbx, [rbp + 0x3e8]
142294f44: 83bdc801000001               cmp dword ptr [rbp + 0x1c8], 1
142294f4b: 4489add0060000               mov dword ptr [rbp + 0x6d0], r13d
142294f52: 74bc                         je 0x142294f10
142294f54: 6666662e0f1f840000000000     nop word ptr cs:[rax + rax]
142294f60: 80bdf706000003               cmp byte ptr [rbp + 0x6f7], 3
142294f67: 7408                         je 0x142294f71
142294f69: 41c7412800000000             mov dword ptr [r9 + 0x28], 0
142294f71: 488bb520030000               mov rsi, qword ptr [rbp + 0x320]
142294f78: f30f1006                     movss xmm0, dword ptr [rsi]
142294f7c: f30f104e04                   movss xmm1, dword ptr [rsi + 4]
142294f81: f3410f5c4150                 subss xmm0, dword ptr [r9 + 0x50]
142294f87: f3410f5c4954                 subss xmm1, dword ptr [r9 + 0x54]
142294f8d: e88ebe9ffe                   call 0x140c90e20
142294f92: 0f28c1                       movaps xmm0, xmm1
142294f95: f30f58c7                     addss xmm0, xmm7
142294f99: 0f28d1                       movaps xmm2, xmm1
142294f9c: f3410fc2d001                 cmpltss xmm2, xmm8
142294fa2: 0f54c2                       andps xmm0, xmm2
142294fa5: 0f55d1                       andnps xmm2, xmm1
142294fa8: 0f56d0                       orps xmm2, xmm0
142294fab: 488b8508070000               mov rax, qword ptr [rbp + 0x708]
142294fb2: f30f104058                   movss xmm0, dword ptr [rax + 0x58]
142294fb7: 488b8508070000               mov rax, qword ptr [rbp + 0x708]
142294fbe: f30f10485c                   movss xmm1, dword ptr [rax + 0x5c]
142294fc3: e808d53500                   call 0x1425f24d0
142294fc8: 0f28c8                       movaps xmm1, xmm0
142294fcb: 488b8508070000               mov rax, qword ptr [rbp + 0x708]
142294fd2: f30f10808c000000             movss xmm0, dword ptr [rax + 0x8c]
142294fda: e891be9ffe                   call 0x140c90e70
142294fdf: 4c8b9508070000               mov r10, qword ptr [rbp + 0x708]
142294fe6: f2410f105250                 movsd xmm2, qword ptr [r10 + 0x50]
142294fec: 0f14c1                       unpcklps xmm0, xmm1
142294fef: 0f58c2                       addps xmm0, xmm2
142294ff2: f20f100e                     movsd xmm1, qword ptr [rsi]
142294ff6: 0f5cc1                       subps xmm0, xmm1
142294ff9: 0f59c0                       mulps xmm0, xmm0
142294ffc: f30f16c8                     movshdup xmm1, xmm0
142295000: f30f58c8                     addss xmm1, xmm0
142295004: 0f57c0                       xorps xmm0, xmm0
142295007: f30f51c1                     sqrtss xmm0, xmm1
14229500b: f3410f108a90000000           movss xmm1, dword ptr [r10 + 0x90]
142295014: 488b85a0000000               mov rax, qword ptr [rbp + 0xa0]
14229501b: f30f5988e8000000             mulss xmm1, dword ptr [rax + 0xe8]
142295023: f3410f59c9                   mulss xmm1, xmm9
142295028: f3410f58ca                   addss xmm1, xmm10
14229502d: 0f2ec1                       ucomiss xmm0, xmm1
142295030: 488dbd30050000               lea rdi, [rbp + 0x530]
142295037: 760d                         jbe 0x142295046
142295039: f685f002000001               test byte ptr [rbp + 0x2f0], 1
142295040: 0f84b4010000                 je 0x1422951fa
142295046: 488bb520030000               mov rsi, qword ptr [rbp + 0x320]
14229504d: 4889b5e8030000               mov qword ptr [rbp + 0x3e8], rsi
142295054: 488d0585cbfefe               lea rax, [rip - 0x101347b]
14229505b: 488985f0030000               mov qword ptr [rbp + 0x3f0], rax
142295062: 488d05bf64da00               lea rax, [rip + 0xda64bf]
142295069: 48898530050000               mov qword ptr [rbp + 0x530], rax
142295070: 48c7853805000001000000       mov qword ptr [rbp + 0x538], 1
14229507b: 48899d40050000               mov qword ptr [rbp + 0x540], rbx
142295082: 48c7854805000001000000       mov qword ptr [rbp + 0x548], 1
14229508d: 48c7855005000000000000       mov qword ptr [rbp + 0x550], 0
142295098: 4c8dbd38060000               lea r15, [rbp + 0x638]
14229509f: 4c89f9                       mov rcx, r15
1422950a2: 4889fa                       mov rdx, rdi
1422950a5: e8e64eaefe                   call 0x140d79f90
1422950aa: f20f1206                     movddup xmm0, qword ptr [rsi]
1422950ae: 4c89fe                       mov rsi, r15
1422950b1: 410f58c3                     addps xmm0, xmm11
1422950b5: 0f1185f0030000               movups xmmword ptr [rbp + 0x3f0], xmm0
1422950bc: c785ec0300000ad7233c         mov dword ptr [rbp + 0x3ec], 0x3c23d70a
1422950c6: c685e803000003               mov byte ptr [rbp + 0x3e8], 3
1422950cd: c645b000                     mov byte ptr [rbp - 0x50], 0
1422950d1: c685eb06000001               mov byte ptr [rbp + 0x6eb], 1
1422950d8: 4889f9                       mov rcx, rdi
1422950db: 4889da                       mov rdx, rbx
1422950de: 4c8d7db0                     lea r15, [rbp - 0x50]
1422950e2: 4d89f8                       mov r8, r15
1422950e5: e8263d71fe                   call 0x1409a8e10
1422950ea: c685a506000001               mov byte ptr [rbp + 0x6a5], 1
1422950f1: 4c89f9                       mov rcx, r15
1422950f4: e8b73e71fe                   call 0x1409a8fb0
1422950f9: c685a506000000               mov byte ptr [rbp + 0x6a5], 0
142295100: 4889d9                       mov rcx, rbx
142295103: 4889fa                       mov rdx, rdi
142295106: 4d89f8                       mov r8, r15
142295109: e8723d71fe                   call 0x1409a8e80
14229510e: c685eb06000000               mov byte ptr [rbp + 0x6eb], 0
142295115: 488b8d50060000               mov rcx, qword ptr [rbp + 0x650]
14229511c: 4889f2                       mov rdx, rsi
14229511f: 4989d8                       mov r8, rbx
142295122: e8f97f0200                   call 0x1422bd120
142295127: 40b701                       mov dil, 1
14229512a: 4c8b9508070000               mov r10, qword ptr [rbp + 0x708]
142295131: 410fb64230                   movzx eax, byte ptr [r10 + 0x30]
142295136: 3c02                         cmp al, 2
142295138: 0f85e2000000                 jne 0x142295220
14229513e: 4d8b7a38                     mov r15, qword ptr [r10 + 0x38]
142295142: 4180fe03                     cmp r14b, 3
142295146: 0f85ef000000                 jne 0x14229523b
14229514c: 4084ff                       test dil, dil
14229514f: 0f85ab020000                 jne 0x142295400
142295155: 418b8280000000               mov eax, dword ptr [r10 + 0x80]
14229515c: 898538050000                 mov dword ptr [rbp + 0x538], eax
142295162: 498b4278                     mov rax, qword ptr [r10 + 0x78]
142295166: 48898530050000               mov qword ptr [rbp + 0x530], rax
14229516d: 8b85b0040000                 mov eax, dword ptr [rbp + 0x4b0]
142295173: 488b8d20020000               mov rcx, qword ptr [rbp + 0x220]
14229517a: 8901                         mov dword ptr [rcx], eax
14229517c: 488b8510050000               mov rax, qword ptr [rbp + 0x510]
142295183: 488b7030                     mov rsi, qword ptr [rax + 0x30]
142295187: 488b5840                     mov rbx, qword ptr [rax + 0x40]
14229518b: 483b7020                     cmp rsi, qword ptr [rax + 0x20]
14229518f: 751a                         jne 0x1422951ab
142295191: 488b8d80020000               mov rcx, qword ptr [rbp + 0x280]
142295198: 488d152961da00               lea rdx, [rip + 0xda6129]
14229519f: e88cda2cfe                   call 0x140562c30
1422951a4: 4c8b9508070000               mov r10, qword ptr [rbp + 0x708]
1422951ab: 8b8500070000                 mov eax, dword ptr [rbp + 0x700]
1422951b1: 2401                         and al, 1
1422951b3: 4c8b8d10050000               mov r9, qword ptr [rbp + 0x510]
1422951ba: 498b4928                     mov rcx, qword ptr [r9 + 0x28]
1422951be: 4889f2                       mov rdx, rsi
1422951c1: 48c1e205                     shl rdx, 5
1422951c5: 48891c11                     mov qword ptr [rcx + rdx], rbx
1422951c9: 4c897c1108                   mov qword ptr [rcx + rdx + 8], r15
1422951ce: 4c8b8530050000               mov r8, qword ptr [rbp + 0x530]
1422951d5: 4c89441110                   mov qword ptr [rcx + rdx + 0x10], r8
1422951da: 448b8538050000               mov r8d, dword ptr [rbp + 0x538]
1422951e1: 4489441118                   mov dword ptr [rcx + rdx + 0x18], r8d
1422951e6: 8844111c                     mov byte ptr [rcx + rdx + 0x1c], al
1422951ea: 48ffc6                       inc rsi
1422951ed: 49897130                     mov qword ptr [r9 + 0x30], rsi
1422951f1: 49ff4140                     inc qword ptr [r9 + 0x40]
1422951f5: e906020000                   jmp 0x142295400
1422951fa: 41c7422800000000             mov dword ptr [r10 + 0x28], 0
142295202: 31ff                         xor edi, edi
142295204: 410fb64230                   movzx eax, byte ptr [r10 + 0x30]
142295209: 3c02                         cmp al, 2
14229520b: 0f842dffffff                 je 0x14229513e
142295211: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
142295220: 3c0b                         cmp al, 0xb
142295222: 4c8bad00050000               mov r13, qword ptr [rbp + 0x500]
142295229: 4c8bb508050000               mov r14, qword ptr [rbp + 0x508]
142295230: 0f84e5010000                 je 0x14229541b
142295236: e9c5030000                   jmp 0x142295600
14229523b: 8b85b0040000                 mov eax, dword ptr [rbp + 0x4b0]
142295241: 488b8d20020000               mov rcx, qword ptr [rbp + 0x220]
142295248: 8901                         mov dword ptr [rcx], eax
14229524a: 488b8510050000               mov rax, qword ptr [rbp + 0x510]
142295251: 488b7030                     mov rsi, qword ptr [rax + 0x30]
142295255: 488b5840                     mov rbx, qword ptr [rax + 0x40]
142295259: 483b7020                     cmp rsi, qword ptr [rax + 0x20]
14229525d: 751a                         jne 0x142295279
14229525f: 488b8d80020000               mov rcx, qword ptr [rbp + 0x280]
142295266: 488d155b60da00               lea rdx, [rip + 0xda605b]
14229526d: e8bed92cfe                   call 0x140562c30
142295272: 4c8b9508070000               mov r10, qword ptr [rbp + 0x708]
142295279: 448b9d00070000               mov r11d, dword ptr [rbp + 0x700]
142295280: 4489d8                       mov eax, r11d
142295283: 2401                         and al, 1
142295285: 4c8b8510050000               mov r8, qword ptr [rbp + 0x510]
14229528c: 498b4828                     mov rcx, qword ptr [r8 + 0x28]
142295290: 4889f2                       mov rdx, rsi
142295293: 48c1e205                     shl rdx, 5
142295297: 48891c11                     mov qword ptr [rcx + rdx], rbx
14229529b: 4c897c1108                   mov qword ptr [rcx + rdx + 8], r15
1422952a0: 4488741110                   mov byte ptr [rcx + rdx + 0x10], r14b
1422952a5: 440fb68ddf060000             movzx r9d, byte ptr [rbp + 0x6df]
1422952ad: 44884c1111                   mov byte ptr [rcx + rdx + 0x11], r9b
1422952b2: 440fb78d26060000             movzx r9d, word ptr [rbp + 0x626]
1422952ba: 6644894c1112                 mov word ptr [rcx + rdx + 0x12], r9w
1422952c0: 448b8dfc050000               mov r9d, dword ptr [rbp + 0x5fc]
1422952c7: 44894c1114                   mov dword ptr [rcx + rdx + 0x14], r9d
1422952cc: 448b8d00060000               mov r9d, dword ptr [rbp + 0x600]
1422952d3: 44894c1118                   mov dword ptr [rcx + rdx + 0x18], r9d
1422952d8: 8844111c                     mov byte ptr [rcx + rdx + 0x1c], al
1422952dc: 48ffc6                       inc rsi
1422952df: 49897030                     mov qword ptr [r8 + 0x30], rsi
1422952e3: 49ff4040                     inc qword ptr [r8 + 0x40]
1422952e7: 41f6c301                     test r11b, 1
1422952eb: 0f840f010000                 je 0x142295400
1422952f1: 45387278                     cmp byte ptr [r10 + 0x78], r14b
1422952f5: 7536                         jne 0x14229532d
1422952f7: 4180fe02                     cmp r14b, 2
1422952fb: 0f85ff000000                 jne 0x142295400
142295301: 8b85fc050000                 mov eax, dword ptr [rbp + 0x5fc]
142295307: 4139427c                     cmp dword ptr [r10 + 0x7c], eax
14229530b: 7520                         jne 0x14229532d
14229530d: 0fb685df060000               movzx eax, byte ptr [rbp + 0x6df]
142295314: 41384279                     cmp byte ptr [r10 + 0x79], al
142295318: 7513                         jne 0x14229532d
14229531a: 8b8500060000                 mov eax, dword ptr [rbp + 0x600]
142295320: 41398280000000               cmp dword ptr [r10 + 0x80], eax
142295327: 0f84d3000000                 je 0x142295400
14229532d: 498d4278                     lea rax, [r10 + 0x78]
142295331: 488b8d68060000               mov rcx, qword ptr [rbp + 0x668]
142295338: 8b95d8060000                 mov edx, dword ptr [rbp + 0x6d8]
14229533e: 8911                         mov dword ptr [rcx], edx
142295340: 8b4808                       mov ecx, dword ptr [rax + 8]
142295343: 898d38050000                 mov dword ptr [rbp + 0x538], ecx
142295349: 488b00                       mov rax, qword ptr [rax]
14229534c: 48898530050000               mov qword ptr [rbp + 0x530], rax
142295353: 0fb605aff83901               movzx eax, byte ptr [rip + 0x139f8af]
14229535a: b930000000                   mov ecx, 0x30
14229535f: ba08000000                   mov edx, 8
142295364: e8a7e837fe                   call 0x140613c10
142295369: 4885c0                       test rax, rax
14229536c: 0f8406190000                 je 0x142296c78
142295372: 48c70001000000               mov qword ptr [rax], 1
142295379: 48c7400801000000             mov qword ptr [rax + 8], 1
142295381: 4c897810                     mov qword ptr [rax + 0x10], r15
142295385: 488b8d30050000               mov rcx, qword ptr [rbp + 0x530]
14229538c: 48894818                     mov qword ptr [rax + 0x18], rcx
142295390: 8b8d38050000                 mov ecx, dword ptr [rbp + 0x538]
142295396: 894820                       mov dword ptr [rax + 0x20], ecx
142295399: 44887024                     mov byte ptr [rax + 0x24], r14b
14229539d: 0fb68ddf060000               movzx ecx, byte ptr [rbp + 0x6df]
1422953a4: 884825                       mov byte ptr [rax + 0x25], cl
1422953a7: 0fb78d26060000               movzx ecx, word ptr [rbp + 0x626]
1422953ae: 66894826                     mov word ptr [rax + 0x26], cx
1422953b2: 8b8dfc050000                 mov ecx, dword ptr [rbp + 0x5fc]
1422953b8: 894828                       mov dword ptr [rax + 0x28], ecx
1422953bb: 8b8d00060000                 mov ecx, dword ptr [rbp + 0x600]
1422953c1: 89482c                       mov dword ptr [rax + 0x2c], ecx
1422953c4: 48898538050000               mov qword ptr [rbp + 0x538], rax
1422953cb: 48c7853005000009000000       mov qword ptr [rbp + 0x530], 9
1422953d6: 488b8d70060000               mov rcx, qword ptr [rbp + 0x670]
1422953dd: 488d9530050000               lea rdx, [rbp + 0x530]
1422953e4: 4c8d054d61da00               lea r8, [rip + 0xda614d]
1422953eb: e8d0fa7efe                   call 0x140a84ec0
1422953f0: 4c8b9508070000               mov r10, qword ptr [rbp + 0x708]
1422953f7: 660f1f840000000000           nop word ptr [rax + rax]
142295400: 410fb64230                   movzx eax, byte ptr [r10 + 0x30]
142295405: 3c0b                         cmp al, 0xb
142295407: 4c8bad00050000               mov r13, qword ptr [rbp + 0x500]
14229540e: 4c8bb508050000               mov r14, qword ptr [rbp + 0x508]
142295415: 0f85e5010000                 jne 0x142295600
14229541b: 458b7a34                     mov r15d, dword ptr [r10 + 0x34]
14229541f: 80bdf606000004               cmp byte ptr [rbp + 0x6f6], 4
142295426: 0f8581000000                 jne 0x1422954ad
14229542c: 4084ff                       test dil, dil
14229542f: 0f85cb010000                 jne 0x142295600
142295435: 410fb69a84000000             movzx ebx, byte ptr [r10 + 0x84]
14229543d: 8b85b4040000                 mov eax, dword ptr [rbp + 0x4b4]
142295443: 488b8d28020000               mov rcx, qword ptr [rbp + 0x228]
14229544a: 8901                         mov dword ptr [rcx], eax
14229544c: 488b8518050000               mov rax, qword ptr [rbp + 0x518]
142295453: 488b7030                     mov rsi, qword ptr [rax + 0x30]
142295457: 4c8b6040                     mov r12, qword ptr [rax + 0x40]
14229545b: 483b7020                     cmp rsi, qword ptr [rax + 0x20]
14229545f: 751a                         jne 0x14229547b
142295461: 488b8d90020000               mov rcx, qword ptr [rbp + 0x290]
142295468: 488d15595eda00               lea rdx, [rip + 0xda5e59]
14229546f: e8bcd12cfe                   call 0x140562630
142295474: 4c8b9508070000               mov r10, qword ptr [rbp + 0x708]
14229547b: 48c1e320                     shl rbx, 0x20
14229547f: 4c09fb                       or rbx, r15
142295482: 488b9518050000               mov rdx, qword ptr [rbp + 0x518]
142295489: 488b4228                     mov rax, qword ptr [rdx + 0x28]
14229548d: 4889f1                       mov rcx, rsi
142295490: 48c1e104                     shl rcx, 4
142295494: 4c892408                     mov qword ptr [rax + rcx], r12
142295498: 48895c0808                   mov qword ptr [rax + rcx + 8], rbx
14229549d: 48ffc6                       inc rsi
1422954a0: 48897230                     mov qword ptr [rdx + 0x30], rsi
1422954a4: 48ff4240                     inc qword ptr [rdx + 0x40]
1422954a8: e953010000                   jmp 0x142295600
1422954ad: 8b85b4040000                 mov eax, dword ptr [rbp + 0x4b4]
1422954b3: 488b8d28020000               mov rcx, qword ptr [rbp + 0x228]
1422954ba: 8901                         mov dword ptr [rcx], eax
1422954bc: 488b8518050000               mov rax, qword ptr [rbp + 0x518]
1422954c3: 488b7030                     mov rsi, qword ptr [rax + 0x30]
1422954c7: 488b5840                     mov rbx, qword ptr [rax + 0x40]
1422954cb: 483b7020                     cmp rsi, qword ptr [rax + 0x20]
1422954cf: 751a                         jne 0x1422954eb
1422954d1: 488b8d90020000               mov rcx, qword ptr [rbp + 0x290]
1422954d8: 488d15e95dda00               lea rdx, [rip + 0xda5de9]
1422954df: e84cd12cfe                   call 0x140562630
1422954e4: 4c8b9508070000               mov r10, qword ptr [rbp + 0x708]
1422954eb: 448b8d00070000               mov r9d, dword ptr [rbp + 0x700]
1422954f2: 4489c8                       mov eax, r9d
1422954f5: 2401                         and al, 1
1422954f7: 0fb6c0                       movzx eax, al
1422954fa: 48c1e028                     shl rax, 0x28
1422954fe: 0fb68df6060000               movzx ecx, byte ptr [rbp + 0x6f6]
142295505: 48c1e120                     shl rcx, 0x20
142295509: 4809c1                       or rcx, rax
14229550c: 4c09f9                       or rcx, r15
14229550f: 4c8b8518050000               mov r8, qword ptr [rbp + 0x518]
142295516: 498b4028                     mov rax, qword ptr [r8 + 0x28]
14229551a: 4889f2                       mov rdx, rsi
14229551d: 48c1e204                     shl rdx, 4
142295521: 48891c10                     mov qword ptr [rax + rdx], rbx
142295525: 48894c1008                   mov qword ptr [rax + rdx + 8], rcx
14229552a: 48ffc6                       inc rsi
14229552d: 49897030                     mov qword ptr [r8 + 0x30], rsi
142295531: 49ff4040                     inc qword ptr [r8 + 0x40]
142295535: 41f6c101                     test r9b, 1
142295539: 0f84c1000000                 je 0x142295600
14229553f: 0fb685f6060000               movzx eax, byte ptr [rbp + 0x6f6]
142295546: 41388284000000               cmp byte ptr [r10 + 0x84], al
14229554d: 0f84ad000000                 je 0x142295600
142295553: 488b8568060000               mov rax, qword ptr [rbp + 0x668]
14229555a: 8b8dd8060000                 mov ecx, dword ptr [rbp + 0x6d8]
142295560: 8908                         mov dword ptr [rax], ecx
142295562: 410fb69a84000000             movzx ebx, byte ptr [r10 + 0x84]
14229556a: 0fb60598f63901               movzx eax, byte ptr [rip + 0x139f698]
142295571: b930000000                   mov ecx, 0x30
142295576: ba08000000                   mov edx, 8
14229557b: e890e637fe                   call 0x140613c10
142295580: 4885c0                       test rax, rax
142295583: 0f84ef160000                 je 0x142296c78
142295589: 48c70001000000               mov qword ptr [rax], 1
142295590: 48c7400801000000             mov qword ptr [rax + 8], 1
142295598: 44897810                     mov dword ptr [rax + 0x10], r15d
14229559c: 885814                       mov byte ptr [rax + 0x14], bl
14229559f: 0fb69df6060000               movzx ebx, byte ptr [rbp + 0x6f6]
1422955a6: 885815                       mov byte ptr [rax + 0x15], bl
1422955a9: c640180e                     mov byte ptr [rax + 0x18], 0xe
1422955ad: 48898538050000               mov qword ptr [rbp + 0x538], rax
1422955b4: 48c7853005000009000000       mov qword ptr [rbp + 0x530], 9
1422955bf: 488b8d70060000               mov rcx, qword ptr [rbp + 0x670]
1422955c6: 488d9530050000               lea rdx, [rbp + 0x530]
1422955cd: 4c8d057c5fda00               lea r8, [rip + 0xda5f7c]
1422955d4: e8e7f87efe                   call 0x140a84ec0
1422955d9: 8b85f0060000                 mov eax, dword ptr [rbp + 0x6f0]
1422955df: 488b8d90060000               mov rcx, qword ptr [rbp + 0x690]
1422955e6: 8901                         mov dword ptr [rcx], eax
1422955e8: 488b8588060000               mov rax, qword ptr [rbp + 0x688]
1422955ef: 885814                       mov byte ptr [rax + 0x14], bl
1422955f2: 4c8b9508070000               mov r10, qword ptr [rbp + 0x708]
1422955f9: 0f1f8000000000               nop dword ptr [rax]
142295600: f685c001000001               test byte ptr [rbp + 0x1c0], 1
142295607: 0f84d3010000                 je 0x1422957e0
14229560d: 410fb64230                   movzx eax, byte ptr [r10 + 0x30]
142295612: 8d48f5                       lea ecx, [rax - 0xb]
142295615: 80f901                       cmp cl, 1
142295618: 0f87c7010000                 ja 0x1422957e5
14229561e: 458b7a34                     mov r15d, dword ptr [r10 + 0x34]
142295622: 83bda404000001               cmp dword ptr [rbp + 0x4a4], 1
142295629: 0f853a010000                 jne 0x142295769
14229562f: 8b85cc040000                 mov eax, dword ptr [rbp + 0x4cc]
142295635: 488b8d88020000               mov rcx, qword ptr [rbp + 0x288]
14229563c: 8901                         mov dword ptr [rcx], eax
14229563e: 488b8520050000               mov rax, qword ptr [rbp + 0x520]
142295645: 488b7030                     mov rsi, qword ptr [rax + 0x30]
142295649: 488b7840                     mov rdi, qword ptr [rax + 0x40]
14229564d: 483b7020                     cmp rsi, qword ptr [rax + 0x20]
142295651: 751a                         jne 0x14229566d
142295653: 488b8da0020000               mov rcx, qword ptr [rbp + 0x2a0]
14229565a: 488d15675cda00               lea rdx, [rip + 0xda5c67]
142295661: e84ad72cfe                   call 0x140562db0
142295666: 4c8b9508070000               mov r10, qword ptr [rbp + 0x708]
14229566d: 448b9d00070000               mov r11d, dword ptr [rbp + 0x700]
142295674: 4489d8                       mov eax, r11d
142295677: 2401                         and al, 1
142295679: 4c8b8520050000               mov r8, qword ptr [rbp + 0x520]
142295680: 498b4828                     mov rcx, qword ptr [r8 + 0x28]
142295684: 488d1476                     lea rdx, [rsi + rsi*2]
142295688: 48893cd1                     mov qword ptr [rcx + rdx*8], rdi
14229568c: 44897cd108                   mov dword ptr [rcx + rdx*8 + 8], r15d
142295691: 448b8d04060000               mov r9d, dword ptr [rbp + 0x604]
142295698: 44894cd10c                   mov dword ptr [rcx + rdx*8 + 0xc], r9d
14229569d: 8844d110                     mov byte ptr [rcx + rdx*8 + 0x10], al
1422956a1: 48ffc6                       inc rsi
1422956a4: 49897030                     mov qword ptr [r8 + 0x30], rsi
1422956a8: 49ff4040                     inc qword ptr [r8 + 0x40]
1422956ac: 41f6c301                     test r11b, 1
1422956b0: 0f842a010000                 je 0x1422957e0
1422956b6: 8b8504060000                 mov eax, dword ptr [rbp + 0x604]
1422956bc: 41394274                     cmp dword ptr [r10 + 0x74], eax
1422956c0: 0f841a010000                 je 0x1422957e0
1422956c6: 488b8568060000               mov rax, qword ptr [rbp + 0x668]
1422956cd: 8b8dd8060000                 mov ecx, dword ptr [rbp + 0x6d8]
1422956d3: 8908                         mov dword ptr [rax], ecx
1422956d5: 418b7274                     mov esi, dword ptr [r10 + 0x74]
1422956d9: 0fb60529f53901               movzx eax, byte ptr [rip + 0x139f529]
1422956e0: b930000000                   mov ecx, 0x30
1422956e5: ba08000000                   mov edx, 8
1422956ea: e821e537fe                   call 0x140613c10
1422956ef: 4885c0                       test rax, rax
1422956f2: 0f8480150000                 je 0x142296c78
1422956f8: 48c70001000000               mov qword ptr [rax], 1
1422956ff: 48c7400801000000             mov qword ptr [rax + 8], 1
142295707: c640180f                     mov byte ptr [rax + 0x18], 0xf
14229570b: 4489781c                     mov dword ptr [rax + 0x1c], r15d
14229570f: 897020                       mov dword ptr [rax + 0x20], esi
142295712: 8bb504060000                 mov esi, dword ptr [rbp + 0x604]
142295718: 897024                       mov dword ptr [rax + 0x24], esi
14229571b: 48898538050000               mov qword ptr [rbp + 0x538], rax
142295722: 48c7853005000009000000       mov qword ptr [rbp + 0x530], 9
14229572d: 488b8d70060000               mov rcx, qword ptr [rbp + 0x670]
142295734: 488d9530050000               lea rdx, [rbp + 0x530]
14229573b: 4c8d05265eda00               lea r8, [rip + 0xda5e26]
142295742: e879f77efe                   call 0x140a84ec0
142295747: 8b85f0060000                 mov eax, dword ptr [rbp + 0x6f0]
14229574d: 488b8d90060000               mov rcx, qword ptr [rbp + 0x690]
142295754: 8901                         mov dword ptr [rcx], eax
142295756: 488b8588060000               mov rax, qword ptr [rbp + 0x688]
14229575d: 897010                       mov dword ptr [rax + 0x10], esi
142295760: 4c8b9508070000               mov r10, qword ptr [rbp + 0x708]
142295767: eb77                         jmp 0x1422957e0
142295769: 4084ff                       test dil, dil
14229576c: 7572                         jne 0x1422957e0
14229576e: 418b7a74                     mov edi, dword ptr [r10 + 0x74]
142295772: 8b85cc040000                 mov eax, dword ptr [rbp + 0x4cc]
142295778: 488b8d88020000               mov rcx, qword ptr [rbp + 0x288]
14229577f: 8901                         mov dword ptr [rcx], eax
142295781: 488b8520050000               mov rax, qword ptr [rbp + 0x520]
142295788: 488b7030                     mov rsi, qword ptr [rax + 0x30]
14229578c: 488b5840                     mov rbx, qword ptr [rax + 0x40]
142295790: 483b7020                     cmp rsi, qword ptr [rax + 0x20]
142295794: 751a                         jne 0x1422957b0
142295796: 488b8da0020000               mov rcx, qword ptr [rbp + 0x2a0]
14229579d: 488d15245bda00               lea rdx, [rip + 0xda5b24]
1422957a4: e807d62cfe                   call 0x140562db0
1422957a9: 4c8b9508070000               mov r10, qword ptr [rbp + 0x708]
1422957b0: 488b9520050000               mov rdx, qword ptr [rbp + 0x520]
1422957b7: 488b4228                     mov rax, qword ptr [rdx + 0x28]
1422957bb: 488d0c76                     lea rcx, [rsi + rsi*2]
1422957bf: 48891cc8                     mov qword ptr [rax + rcx*8], rbx
1422957c3: 44897cc808                   mov dword ptr [rax + rcx*8 + 8], r15d
1422957c8: 897cc80c                     mov dword ptr [rax + rcx*8 + 0xc], edi
1422957cc: c644c81000                   mov byte ptr [rax + rcx*8 + 0x10], 0
1422957d1: 48ffc6                       inc rsi
1422957d4: 48897230                     mov qword ptr [rdx + 0x30], rsi
1422957d8: 48ff4240                     inc qword ptr [rdx + 0x40]
1422957dc: 0f1f4000                     nop dword ptr [rax]
1422957e0: 410fb64230                   movzx eax, byte ptr [r10 + 0x30]
1422957e5: 84c0                         test al, al
1422957e7: 0f8586050000                 jne 0x142295d73
1422957ed: 498b7a38                     mov rdi, qword ptr [r10 + 0x38]
1422957f1: 488bb508020000               mov rsi, qword ptr [rbp + 0x208]
1422957f8: 4889f1                       mov rcx, rsi
1422957fb: 4889fa                       mov rdx, rdi
1422957fe: 4c8d057b5dda00               lea r8, [rip + 0xda5d7b]
142295805: e8e6c67ffe                   call 0x140a91ef0
14229580a: 4885c0                       test rax, rax
14229580d: 0f84a2140000                 je 0x142296cb5
142295813: 440fb6a899020000             movzx r13d, byte ptr [rax + 0x299]
14229581b: 4889f1                       mov rcx, rsi
14229581e: 4889fa                       mov rdx, rdi
142295821: 4c8d05885dda00               lea r8, [rip + 0xda5d88]
142295828: e8c3c67ffe                   call 0x140a91ef0
14229582d: 4885c0                       test rax, rax
142295830: 488b9dc0000000               mov rbx, qword ptr [rbp + 0xc0]
142295837: 4c8bbdc8000000               mov r15, qword ptr [rbp + 0xc8]
14229583e: 0f8465140000                 je 0x142296ca9
142295844: 440fb6a095020000             movzx r12d, byte ptr [rax + 0x295]
14229584c: 4889f1                       mov rcx, rsi
14229584f: 4889fa                       mov rdx, rdi
142295852: 4c8d05875dda00               lea r8, [rip + 0xda5d87]
142295859: e892c67ffe                   call 0x140a91ef0
14229585e: 4885c0                       test rax, rax
142295861: 0f8436140000                 je 0x142296c9d
142295867: f30f10b090020000             movss xmm6, dword ptr [rax + 0x290]
14229586f: 4889f1                       mov rcx, rsi
142295872: 4889fa                       mov rdx, rdi
142295875: e836ca7ffe                   call 0x140a922b0
14229587a: 4889c1                       mov rcx, rax
14229587d: 48c1e920                     shr rcx, 0x20
142295881: 756b                         jne 0x1422958ee
142295883: c7852806000000000000         mov dword ptr [rbp + 0x628], 0
14229588d: 4c8b9508070000               mov r10, qword ptr [rbp + 0x708]
142295894: 80bdf706000003               cmp byte ptr [rbp + 0x6f7], 3
14229589b: 488b8d88060000               mov rcx, qword ptr [rbp + 0x688]
1422958a2: 0f850f010000                 jne 0x1422959b7
1422958a8: 410fb68285000000             movzx eax, byte ptr [r10 + 0x85]
1422958b0: 8885f7060000                 mov byte ptr [rbp + 0x6f7], al
1422958b6: 4138c5                       cmp r13b, al
1422958b9: 0f84b3010000                 je 0x142295a72
1422958bf: 488b8568020000               mov rax, qword ptr [rbp + 0x268]
1422958c6: 8b8dc8040000                 mov ecx, dword ptr [rbp + 0x4c8]
1422958cc: 8908                         mov dword ptr [rax], ecx
1422958ce: 488b8570030000               mov rax, qword ptr [rbp + 0x370]
1422958d5: 488b5830                     mov rbx, qword ptr [rax + 0x30]
1422958d9: 4c8b7840                     mov r15, qword ptr [rax + 0x40]
1422958dd: 31f6                         xor esi, esi
1422958df: 483b5820                     cmp rbx, qword ptr [rax + 0x20]
1422958e3: 0f854a010000                 jne 0x142295a33
1422958e9: e92b010000                   jmp 0x142295a19
1422958ee: 89c0                         mov eax, eax
1422958f0: 493b4710                     cmp rax, qword ptr [r15 + 0x10]
1422958f4: 4c8b9508070000               mov r10, qword ptr [rbp + 0x708]
1422958fb: 0f8398000000                 jae 0x142295999
142295901: 498b5708                     mov rdx, qword ptr [r15 + 8]
142295905: 488d0480                     lea rax, [rax + rax*4]
142295909: 390c82                       cmp dword ptr [rdx + rax*4], ecx
14229590c: 0f8587000000                 jne 0x142295999
142295912: 488d0482                     lea rax, [rdx + rax*4]
142295916: 8b4804                       mov ecx, dword ptr [rax + 4]
142295919: baffffffff                   mov edx, 0xffffffff
14229591e: 4839d1                       cmp rcx, rdx
142295921: 7476                         je 0x142295999
142295923: 48394b38                     cmp qword ptr [rbx + 0x38], rcx
142295927: 7670                         jbe 0x142295999
142295929: 488b5328                     mov rdx, qword ptr [rbx + 0x28]
14229592d: 4189c8                       mov r8d, ecx
142295930: 41c1e806                     shr r8d, 6
142295934: 4a8b14c2                     mov rdx, qword ptr [rdx + r8*8]
142295938: 480fa3ca                     bt rdx, rcx
14229593c: 735b                         jae 0x142295999
14229593e: 8b480c                       mov ecx, dword ptr [rax + 0xc]
142295941: 8b4010                       mov eax, dword ptr [rax + 0x10]
142295944: 498b97a8010000               mov rdx, qword ptr [r15 + 0x1a8]
14229594b: 488d0cc9                     lea rcx, [rcx + rcx*8]
14229594f: 4c8b8310010000               mov r8, qword ptr [rbx + 0x110]
142295956: 4c8b4cca18                   mov r9, qword ptr [rdx + rcx*8 + 0x18]
14229595b: 488b4cca38                   mov rcx, qword ptr [rdx + rcx*8 + 0x38]
142295960: 4a8b0cc1                     mov rcx, qword ptr [rcx + r8*8]
142295964: 48f7d1                       not rcx
142295967: 488d0c49                     lea rcx, [rcx + rcx*2]
14229596b: 48c1e104                     shl rcx, 4
14229596f: 498b4c0910                   mov rcx, qword ptr [r9 + rcx + 0x10]
142295974: 486bc058                     imul rax, rax, 0x58
142295978: 0fb644013d                   movzx eax, byte ptr [rcx + rax + 0x3d]
14229597d: 898528060000                 mov dword ptr [rbp + 0x628], eax
142295983: 80bdf706000003               cmp byte ptr [rbp + 0x6f7], 3
14229598a: 488b8d88060000               mov rcx, qword ptr [rbp + 0x688]
142295991: 0f8411ffffff                 je 0x1422958a8
142295997: eb1e                         jmp 0x1422959b7
142295999: c7852806000000000000         mov dword ptr [rbp + 0x628], 0
1422959a3: 80bdf706000003               cmp byte ptr [rbp + 0x6f7], 3
1422959aa: 488b8d88060000               mov rcx, qword ptr [rbp + 0x688]
1422959b1: 0f84f1feffff                 je 0x1422958a8
1422959b7: f6850007000001               test byte ptr [rbp + 0x700], 1
1422959be: 7419                         je 0x1422959d9
1422959c0: 8b85f0060000                 mov eax, dword ptr [rbp + 0x6f0]
1422959c6: 488b9590060000               mov rdx, qword ptr [rbp + 0x690]
1422959cd: 8902                         mov dword ptr [rdx], eax
1422959cf: 0fb685f7060000               movzx eax, byte ptr [rbp + 0x6f7]
1422959d6: 884142                       mov byte ptr [rcx + 0x42], al
1422959d9: 450fb6aa85000000             movzx r13d, byte ptr [r10 + 0x85]
1422959e1: 8bb500070000                 mov esi, dword ptr [rbp + 0x700]
1422959e7: 4080e601                     and sil, 1
1422959eb: 488b8568020000               mov rax, qword ptr [rbp + 0x268]
1422959f2: 8b8dc8040000                 mov ecx, dword ptr [rbp + 0x4c8]
1422959f8: 8908                         mov dword ptr [rax], ecx
1422959fa: 488b8570030000               mov rax, qword ptr [rbp + 0x370]
142295a01: 488b5830                     mov rbx, qword ptr [rax + 0x30]
142295a05: 4c8b7840                     mov r15, qword ptr [rax + 0x40]
142295a09: c785d006000000000000         mov dword ptr [rbp + 0x6d0], 0
142295a13: 483b5820                     cmp rbx, qword ptr [rax + 0x20]
142295a17: 751a                         jne 0x142295a33
142295a19: 488b8d08010000               mov rcx, qword ptr [rbp + 0x108]
142295a20: 488d15a158da00               lea rdx, [rip + 0xda58a1]
142295a27: e884d32cfe                   call 0x140562db0
142295a2c: 4c8b9508070000               mov r10, qword ptr [rbp + 0x708]
142295a33: 488b9570030000               mov rdx, qword ptr [rbp + 0x370]
142295a3a: 488b4228                     mov rax, qword ptr [rdx + 0x28]
142295a3e: 488d0c5b                     lea rcx, [rbx + rbx*2]
142295a42: 4c893cc8                     mov qword ptr [rax + rcx*8], r15
142295a46: 48897cc808                   mov qword ptr [rax + rcx*8 + 8], rdi
142295a4b: 44886cc810                   mov byte ptr [rax + rcx*8 + 0x10], r13b
142295a50: 440fb685f7060000             movzx r8d, byte ptr [rbp + 0x6f7]
142295a58: 448844c811                   mov byte ptr [rax + rcx*8 + 0x11], r8b
142295a5d: 408874c812                   mov byte ptr [rax + rcx*8 + 0x12], sil
142295a62: 408874c813                   mov byte ptr [rax + rcx*8 + 0x13], sil
142295a67: 48ffc3                       inc rbx
142295a6a: 48895a30                     mov qword ptr [rdx + 0x30], rbx
142295a6e: 48ff4240                     inc qword ptr [rdx + 0x40]
142295a72: 8b85f0050000                 mov eax, dword ptr [rbp + 0x5f0]
142295a78: 83f801                       cmp eax, 1
142295a7b: 4c8bb500050000               mov r14, qword ptr [rbp + 0x500]
142295a82: 0f85d2000000                 jne 0x142295b5a
142295a88: f3410f107270                 movss xmm6, dword ptr [r10 + 0x70]
142295a8e: 8b9d00070000                 mov ebx, dword ptr [rbp + 0x700]
142295a94: 80e301                       and bl, 1
142295a97: 8b85bc040000                 mov eax, dword ptr [rbp + 0x4bc]
142295a9d: 488b8d40020000               mov rcx, qword ptr [rbp + 0x240]
142295aa4: 8901                         mov dword ptr [rcx], eax
142295aa6: 488b85d0030000               mov rax, qword ptr [rbp + 0x3d0]
142295aad: 488b7030                     mov rsi, qword ptr [rax + 0x30]
142295ab1: 4c8b7840                     mov r15, qword ptr [rax + 0x40]
142295ab5: 483b7020                     cmp rsi, qword ptr [rax + 0x20]
142295ab9: 751a                         jne 0x142295ad5
142295abb: 488b8df0000000               mov rcx, qword ptr [rbp + 0xf0]
142295ac2: 488d15ff57da00               lea rdx, [rip + 0xda57ff]
142295ac9: e862d12cfe                   call 0x140562c30
142295ace: 4c8b9508070000               mov r10, qword ptr [rbp + 0x708]
142295ad5: 488b95d0030000               mov rdx, qword ptr [rbp + 0x3d0]
142295adc: 488b4228                     mov rax, qword ptr [rdx + 0x28]
142295ae0: 4889f1                       mov rcx, rsi
142295ae3: 48c1e105                     shl rcx, 5
142295ae7: 4c893c08                     mov qword ptr [rax + rcx], r15
142295aeb: 48897c0808                   mov qword ptr [rax + rcx + 8], rdi
142295af0: f30f11740810                 movss dword ptr [rax + rcx + 0x10], xmm6
142295af6: f3440f11740814               movss dword ptr [rax + rcx + 0x14], xmm14
142295afd: 885c0818                     mov byte ptr [rax + rcx + 0x18], bl
142295b01: 48ffc6                       inc rsi
142295b04: 48897230                     mov qword ptr [rdx + 0x30], rsi
142295b08: 48ff4240                     inc qword ptr [rdx + 0x40]
142295b0c: 440fb6ada6060000             movzx r13d, byte ptr [rbp + 0x6a6]
142295b14: 4180fd02                     cmp r13b, 2
142295b18: 757d                         jne 0x142295b97
142295b1a: 450fb6aa87000000             movzx r13d, byte ptr [r10 + 0x87]
142295b22: 4538ec                       cmp r12b, r13b
142295b25: 0f8427010000                 je 0x142295c52
142295b2b: 488b8558020000               mov rax, qword ptr [rbp + 0x258]
142295b32: 8b8dc0040000                 mov ecx, dword ptr [rbp + 0x4c0]
142295b38: 8908                         mov dword ptr [rax], ecx
142295b3a: 488b8560030000               mov rax, qword ptr [rbp + 0x360]
142295b41: 488b7030                     mov rsi, qword ptr [rax + 0x30]
142295b45: 4c8b7840                     mov r15, qword ptr [rax + 0x40]
142295b49: 31db                         xor ebx, ebx
142295b4b: 483b7020                     cmp rsi, qword ptr [rax + 0x20]
142295b4f: 0f84b2000000                 je 0x142295c07
142295b55: e9c7000000                   jmp 0x142295c21
142295b5a: f3450f107270                 movss xmm14, dword ptr [r10 + 0x70]
142295b60: 410f2ef6                     ucomiss xmm6, xmm14
142295b64: 7502                         jne 0x142295b68
142295b66: 7ba4                         jnp 0x142295b0c
142295b68: 8b85bc040000                 mov eax, dword ptr [rbp + 0x4bc]
142295b6e: 488b8d40020000               mov rcx, qword ptr [rbp + 0x240]
142295b75: 8901                         mov dword ptr [rcx], eax
142295b77: 488b85d0030000               mov rax, qword ptr [rbp + 0x3d0]
142295b7e: 488b7030                     mov rsi, qword ptr [rax + 0x30]
142295b82: 4c8b7840                     mov r15, qword ptr [rax + 0x40]
142295b86: 31db                         xor ebx, ebx
142295b88: 483b7020                     cmp rsi, qword ptr [rax + 0x20]
142295b8c: 0f8429ffffff                 je 0x142295abb
142295b92: e93effffff                   jmp 0x142295ad5
142295b97: f6850007000001               test byte ptr [rbp + 0x700], 1
142295b9e: 488b9588060000               mov rdx, qword ptr [rbp + 0x688]
142295ba5: 7427                         je 0x142295bce
142295ba7: 4489e8                       mov eax, r13d
142295baa: 8b8df0060000                 mov ecx, dword ptr [rbp + 0x6f0]
142295bb0: 4c8b8590060000               mov r8, qword ptr [rbp + 0x690]
142295bb7: 418908                       mov dword ptr [r8], ecx
142295bba: 2401                         and al, 1
142295bbc: 4189c5                       mov r13d, eax
142295bbf: 884240                       mov byte ptr [rdx + 0x40], al
142295bc2: 418b4268                     mov eax, dword ptr [r10 + 0x68]
142295bc6: 418908                       mov dword ptr [r8], ecx
142295bc9: 894218                       mov dword ptr [rdx + 0x18], eax
142295bcc: eb04                         jmp 0x142295bd2
142295bce: 4180e501                     and r13b, 1
142295bd2: 450fb6a287000000             movzx r12d, byte ptr [r10 + 0x87]
142295bda: 8b9d00070000                 mov ebx, dword ptr [rbp + 0x700]
142295be0: 80e301                       and bl, 1
142295be3: 488b8558020000               mov rax, qword ptr [rbp + 0x258]
142295bea: 8b8dc0040000                 mov ecx, dword ptr [rbp + 0x4c0]
142295bf0: 8908                         mov dword ptr [rax], ecx
142295bf2: 488b8560030000               mov rax, qword ptr [rbp + 0x360]
142295bf9: 488b7030                     mov rsi, qword ptr [rax + 0x30]
142295bfd: 4c8b7840                     mov r15, qword ptr [rax + 0x40]
142295c01: 483b7020                     cmp rsi, qword ptr [rax + 0x20]
142295c05: 751a                         jne 0x142295c21
142295c07: 488b8d10010000               mov rcx, qword ptr [rbp + 0x110]
142295c0e: 488d15b356da00               lea rdx, [rip + 0xda56b3]
142295c15: e896d12cfe                   call 0x140562db0
142295c1a: 4c8b9508070000               mov r10, qword ptr [rbp + 0x708]
142295c21: 488b9560030000               mov rdx, qword ptr [rbp + 0x360]
142295c28: 488b4228                     mov rax, qword ptr [rdx + 0x28]
142295c2c: 488d0c76                     lea rcx, [rsi + rsi*2]
142295c30: 4c893cc8                     mov qword ptr [rax + rcx*8], r15
142295c34: 48897cc808                   mov qword ptr [rax + rcx*8 + 8], rdi
142295c39: 448864c810                   mov byte ptr [rax + rcx*8 + 0x10], r12b
142295c3e: 44886cc811                   mov byte ptr [rax + rcx*8 + 0x11], r13b
142295c43: 885cc812                     mov byte ptr [rax + rcx*8 + 0x12], bl
142295c47: 48ffc6                       inc rsi
142295c4a: 48897230                     mov qword ptr [rdx + 0x30], rsi
142295c4e: 48ff4240                     inc qword ptr [rdx + 0x40]
142295c52: 440fb6a5a7060000             movzx r12d, byte ptr [rbp + 0x6a7]
142295c5a: 4180fc02                     cmp r12b, 2
142295c5e: 488b8d88060000               mov rcx, qword ptr [rbp + 0x688]
142295c65: 7520                         jne 0x142295c87
142295c67: 450fb6a288000000             movzx r12d, byte ptr [r10 + 0x88]
142295c6f: 4438a528060000               cmp byte ptr [rbp + 0x628], r12b
142295c76: 4d89f5                       mov r13, r14
142295c79: 7531                         jne 0x142295cac
142295c7b: 4c8bb508050000               mov r14, qword ptr [rbp + 0x508]
142295c82: e9ec000000                   jmp 0x142295d73
142295c87: f6850007000001               test byte ptr [rbp + 0x700], 1
142295c8e: 4d89f5                       mov r13, r14
142295c91: 7448                         je 0x142295cdb
142295c93: 8b85f0060000                 mov eax, dword ptr [rbp + 0x6f0]
142295c99: 488b9590060000               mov rdx, qword ptr [rbp + 0x690]
142295ca0: 8902                         mov dword ptr [rdx], eax
142295ca2: 4180e401                     and r12b, 1
142295ca6: 44886141                     mov byte ptr [rcx + 0x41], r12b
142295caa: eb33                         jmp 0x142295cdf
142295cac: 488b8560020000               mov rax, qword ptr [rbp + 0x260]
142295cb3: 8b8dc4040000                 mov ecx, dword ptr [rbp + 0x4c4]
142295cb9: 8908                         mov dword ptr [rax], ecx
142295cbb: 488b8568030000               mov rax, qword ptr [rbp + 0x368]
142295cc2: 488b7030                     mov rsi, qword ptr [rax + 0x30]
142295cc6: 4c8b7840                     mov r15, qword ptr [rax + 0x40]
142295cca: 31db                         xor ebx, ebx
142295ccc: 483b7020                     cmp rsi, qword ptr [rax + 0x20]
142295cd0: 4c8bb508050000               mov r14, qword ptr [rbp + 0x508]
142295cd7: 7448                         je 0x142295d21
142295cd9: eb60                         jmp 0x142295d3b
142295cdb: 4180e401                     and r12b, 1
142295cdf: 4c8bb508050000               mov r14, qword ptr [rbp + 0x508]
142295ce6: 410fb68288000000             movzx eax, byte ptr [r10 + 0x88]
142295cee: 898528060000                 mov dword ptr [rbp + 0x628], eax
142295cf4: 8b9d00070000                 mov ebx, dword ptr [rbp + 0x700]
142295cfa: 80e301                       and bl, 1
142295cfd: 488b8560020000               mov rax, qword ptr [rbp + 0x260]
142295d04: 8b8dc4040000                 mov ecx, dword ptr [rbp + 0x4c4]
142295d0a: 8908                         mov dword ptr [rax], ecx
142295d0c: 488b8568030000               mov rax, qword ptr [rbp + 0x368]
142295d13: 488b7030                     mov rsi, qword ptr [rax + 0x30]
142295d17: 4c8b7840                     mov r15, qword ptr [rax + 0x40]
142295d1b: 483b7020                     cmp rsi, qword ptr [rax + 0x20]
142295d1f: 751a                         jne 0x142295d3b
142295d21: 488b8d18010000               mov rcx, qword ptr [rbp + 0x118]
142295d28: 488d159955da00               lea rdx, [rip + 0xda5599]
142295d2f: e87cd02cfe                   call 0x140562db0
142295d34: 4c8b9508070000               mov r10, qword ptr [rbp + 0x708]
142295d3b: 488b9568030000               mov rdx, qword ptr [rbp + 0x368]
142295d42: 488b4228                     mov rax, qword ptr [rdx + 0x28]
142295d46: 488d0c76                     lea rcx, [rsi + rsi*2]
142295d4a: 4c893cc8                     mov qword ptr [rax + rcx*8], r15
142295d4e: 48897cc808                   mov qword ptr [rax + rcx*8 + 8], rdi
142295d53: 448b8528060000               mov r8d, dword ptr [rbp + 0x628]
142295d5a: 448844c810                   mov byte ptr [rax + rcx*8 + 0x10], r8b
142295d5f: 448864c811                   mov byte ptr [rax + rcx*8 + 0x11], r12b
142295d64: 885cc812                     mov byte ptr [rax + rcx*8 + 0x12], bl
142295d68: 48ffc6                       inc rsi
142295d6b: 48897230                     mov qword ptr [rdx + 0x30], rsi
142295d6f: 48ff4240                     inc qword ptr [rdx + 0x40]
142295d73: 418b4268                     mov eax, dword ptr [r10 + 0x68]
142295d77: 41807a2800                   cmp byte ptr [r10 + 0x28], 0
142295d7c: 89c7                         mov edi, eax
142295d7e: 7404                         je 0x142295d84
142295d80: 418b7a2c                     mov edi, dword ptr [r10 + 0x2c]
142295d84: 81ff09030000                 cmp edi, 0x309
142295d8a: 7554                         jne 0x142295de0
142295d8c: b001                         mov al, 1
142295d8e: f6850007000001               test byte ptr [rbp + 0x700], 1
142295d95: 0f840dbcffff                 je 0x1422919a8
142295d9b: 488b85a8000000               mov rax, qword ptr [rbp + 0xa8]
142295da2: 8b8db4020000                 mov ecx, dword ptr [rbp + 0x2b4]
142295da8: 8908                         mov dword ptr [rax], ecx
142295daa: 488b85d8010000               mov rax, qword ptr [rbp + 0x1d8]
142295db1: 488b7030                     mov rsi, qword ptr [rax + 0x30]
142295db5: 488b7840                     mov rdi, qword ptr [rax + 0x40]
142295db9: 483b7020                     cmp rsi, qword ptr [rax + 0x20]
142295dbd: 0f85bdbbffff                 jne 0x142291980
142295dc3: 488b8dd0000000               mov rcx, qword ptr [rbp + 0xd0]
142295dca: 488d15f754da00               lea rdx, [rip + 0xda54f7]
142295dd1: e85ac82cfe                   call 0x140562630
142295dd6: e9a5bbffff                   jmp 0x142291980
142295ddb: 0f1f440000                   nop dword ptr [rax + rax]
142295de0: 39f8                         cmp eax, edi
142295de2: 410f95c7                     setne r15b
142295de6: 4422bd00070000               and r15b, byte ptr [rbp + 0x700]
142295ded: 410fb64230                   movzx eax, byte ptr [r10 + 0x30]
142295df2: 488d0d1f5cda00               lea rcx, [rip + 0xda5c1f]
142295df9: 48630481                     movsxd rax, dword ptr [rcx + rax*4]
142295dfd: 4801c8                       add rax, rcx
142295e00: ffe0                         jmp rax
142295e02: 458b6234                     mov r12d, dword ptr [r10 + 0x34]
142295e06: 488b85e0000000               mov rax, qword ptr [rbp + 0xe0]
142295e0d: 8b8dc0020000                 mov ecx, dword ptr [rbp + 0x2c0]
142295e13: 8908                         mov dword ptr [rax], ecx
142295e15: 488b8558030000               mov rax, qword ptr [rbp + 0x358]
142295e1c: 488b7030                     mov rsi, qword ptr [rax + 0x30]
142295e20: 488b5840                     mov rbx, qword ptr [rax + 0x40]
142295e24: 483b7020                     cmp rsi, qword ptr [rax + 0x20]
142295e28: 751a                         jne 0x142295e44
142295e2a: 488b8d28010000               mov rcx, qword ptr [rbp + 0x128]
142295e31: 488d159054da00               lea rdx, [rip + 0xda5490]
142295e38: e873cf2cfe                   call 0x140562db0
142295e3d: 4c8b9508070000               mov r10, qword ptr [rbp + 0x708]
142295e44: 448b8500070000               mov r8d, dword ptr [rbp + 0x700]
142295e4b: 448b8dd0060000               mov r9d, dword ptr [rbp + 0x6d0]
142295e52: 4520c1                       and r9b, r8b
142295e55: 488b9558030000               mov rdx, qword ptr [rbp + 0x358]
142295e5c: 488b4228                     mov rax, qword ptr [rdx + 0x28]
142295e60: 488d0c76                     lea rcx, [rsi + rsi*2]
142295e64: 48891cc8                     mov qword ptr [rax + rcx*8], rbx
142295e68: 448964c808                   mov dword ptr [rax + rcx*8 + 8], r12d
142295e6d: 897cc80c                     mov dword ptr [rax + rcx*8 + 0xc], edi
142295e71: 44884cc810                   mov byte ptr [rax + rcx*8 + 0x10], r9b
142295e76: 48ffc6                       inc rsi
142295e79: 48897230                     mov qword ptr [rdx + 0x30], rsi
142295e7d: 48ff4240                     inc qword ptr [rdx + 0x40]
142295e81: b001                         mov al, 1
142295e83: 41f6c001                     test r8b, 1
142295e87: 0f841bbbffff                 je 0x1422919a8
142295e8d: 410fb64230                   movzx eax, byte ptr [r10 + 0x30]
142295e92: 83c0fd                       add eax, -3
142295e95: 83f808                       cmp eax, 8
142295e98: 0f8739080000                 ja 0x1422966d7
142295e9e: 488d0da75bda00               lea rcx, [rip + 0xda5ba7]
142295ea5: 48630481                     movsxd rax, dword ptr [rcx + rax*4]
142295ea9: 4801c8                       add rax, rcx
142295eac: ffe0                         jmp rax
142295eae: 81ffe7030000                 cmp edi, 0x3e7
142295eb4: 7413                         je 0x142295ec9
142295eb6: 85ff                         test edi, edi
142295eb8: 0f8558090000                 jne 0x142296816
142295ebe: 41807a3100                   cmp byte ptr [r10 + 0x31], 0
142295ec3: 0f854d090000                 jne 0x142296816
142295ec9: 8b85f0060000                 mov eax, dword ptr [rbp + 0x6f0]
142295ecf: 488b8d90060000               mov rcx, qword ptr [rbp + 0x690]
142295ed6: 8901                         mov dword ptr [rcx], eax
142295ed8: 488b8588060000               mov rax, qword ptr [rbp + 0x688]
142295edf: c70000000000                 mov dword ptr [rax], 0
142295ee5: 4584ff                       test r15b, r15b
142295ee8: 0f84adfeffff                 je 0x142295d9b
142295eee: e9ed070000                   jmp 0x1422966e0
142295ef3: 4531ed                       xor r13d, r13d
142295ef6: 488d9de8030000               lea rbx, [rbp + 0x3e8]
142295efd: 0fb68dff060000               movzx ecx, byte ptr [rbp + 0x6ff]
142295f04: 4180793009                   cmp byte ptr [r9 + 0x30], 9
142295f09: 0f8453ebffff                 je 0x142294a62
142295f0f: e9ecefffff                   jmp 0x142294f00
142295f14: 498b5a38                     mov rbx, qword ptr [r10 + 0x38]
142295f18: 48899de8030000               mov qword ptr [rbp + 0x3e8], rbx
142295f1f: 488b8530010000               mov rax, qword ptr [rbp + 0x130]
142295f26: 8b8dd0020000                 mov ecx, dword ptr [rbp + 0x2d0]
142295f2c: 8908                         mov dword ptr [rax], ecx
142295f2e: 488b8578030000               mov rax, qword ptr [rbp + 0x378]
142295f35: 488b7030                     mov rsi, qword ptr [rax + 0x30]
142295f39: 4c8b6040                     mov r12, qword ptr [rax + 0x40]
142295f3d: 483b7020                     cmp rsi, qword ptr [rax + 0x20]
142295f41: 751a                         jne 0x142295f5d
142295f43: 488b8d78010000               mov rcx, qword ptr [rbp + 0x178]
142295f4a: 488d157753da00               lea rdx, [rip + 0xda5377]
142295f51: e85ace2cfe                   call 0x140562db0
142295f56: 4c8b9508070000               mov r10, qword ptr [rbp + 0x708]
142295f5d: 448b8d00070000               mov r9d, dword ptr [rbp + 0x700]
142295f64: 4489c8                       mov eax, r9d
142295f67: 2401                         and al, 1
142295f69: 4c8b8578030000               mov r8, qword ptr [rbp + 0x378]
142295f70: 498b4828                     mov rcx, qword ptr [r8 + 0x28]
142295f74: 488d1476                     lea rdx, [rsi + rsi*2]
142295f78: 4c8924d1                     mov qword ptr [rcx + rdx*8], r12
142295f7c: 48895cd108                   mov qword ptr [rcx + rdx*8 + 8], rbx
142295f81: 897cd110                     mov dword ptr [rcx + rdx*8 + 0x10], edi
142295f85: 8844d114                     mov byte ptr [rcx + rdx*8 + 0x14], al
142295f89: 48ffc6                       inc rsi
142295f8c: 49897030                     mov qword ptr [r8 + 0x30], rsi
142295f90: 49ff4040                     inc qword ptr [r8 + 0x40]
142295f94: 41f6c101                     test r9b, 1
142295f98: 7476                         je 0x142296010
142295f9a: 41807a3007                   cmp byte ptr [r10 + 0x30], 7
142295f9f: 8b85f0060000                 mov eax, dword ptr [rbp + 0x6f0]
142295fa5: 488b8d90060000               mov rcx, qword ptr [rbp + 0x690]
142295fac: 8901                         mov dword ptr [rcx], eax
142295fae: 0f8528060000                 jne 0x1422965dc
142295fb4: 488bb588060000               mov rsi, qword ptr [rbp + 0x688]
142295fbb: 897e24                       mov dword ptr [rsi + 0x24], edi
142295fbe: 488d8de8030000               lea rcx, [rbp + 0x3e8]
142295fc5: e80626dffd                   call 0x1400885d0
142295fca: 488b8d80010000               mov rcx, qword ptr [rbp + 0x180]
142295fd1: 488b5110                     mov rdx, qword ptr [rcx + 0x10]
142295fd5: 4839d0                       cmp rax, rdx
142295fd8: 0f83310d0000                 jae 0x142296d0f
142295fde: 488b4908                     mov rcx, qword ptr [rcx + 8]
142295fe2: 8b0481                       mov eax, dword ptr [rcx + rax*4]
142295fe5: 89461c                       mov dword ptr [rsi + 0x1c], eax
142295fe8: 8b85f0060000                 mov eax, dword ptr [rbp + 0x6f0]
142295fee: 488b8d90060000               mov rcx, qword ptr [rbp + 0x690]
142295ff5: 8901                         mov dword ptr [rcx], eax
142295ff7: c6464202                     mov byte ptr [rsi + 0x42], 2
142295ffb: 4c8b9508070000               mov r10, qword ptr [rbp + 0x708]
142296002: 4584ff                       test r15b, r15b
142296005: 0f8490fdffff                 je 0x142295d9b
14229600b: e9df050000                   jmp 0x1422965ef
142296010: b001                         mov al, 1
142296012: e991b9ffff                   jmp 0x1422919a8
142296017: 498b5a38                     mov rbx, qword ptr [r10 + 0x38]
14229601b: 488b8558010000               mov rax, qword ptr [rbp + 0x158]
142296022: 8b8dd8020000                 mov ecx, dword ptr [rbp + 0x2d8]
142296028: 8908                         mov dword ptr [rax], ecx
14229602a: 488b8588030000               mov rax, qword ptr [rbp + 0x388]
142296031: 488b7030                     mov rsi, qword ptr [rax + 0x30]
142296035: 4c8b6040                     mov r12, qword ptr [rax + 0x40]
142296039: 483b7020                     cmp rsi, qword ptr [rax + 0x20]
14229603d: 751a                         jne 0x142296059
14229603f: 488b8da0010000               mov rcx, qword ptr [rbp + 0x1a0]
142296046: 488d157b52da00               lea rdx, [rip + 0xda527b]
14229604d: e85ecd2cfe                   call 0x140562db0
142296052: 4c8b9508070000               mov r10, qword ptr [rbp + 0x708]
142296059: 448b8d00070000               mov r9d, dword ptr [rbp + 0x700]
142296060: 4489c8                       mov eax, r9d
142296063: 2401                         and al, 1
142296065: 4c8b8588030000               mov r8, qword ptr [rbp + 0x388]
14229606c: 498b4828                     mov rcx, qword ptr [r8 + 0x28]
142296070: 488d1476                     lea rdx, [rsi + rsi*2]
142296074: 4c8924d1                     mov qword ptr [rcx + rdx*8], r12
142296078: 48895cd108                   mov qword ptr [rcx + rdx*8 + 8], rbx
14229607d: 897cd110                     mov dword ptr [rcx + rdx*8 + 0x10], edi
142296081: 8844d114                     mov byte ptr [rcx + rdx*8 + 0x14], al
142296085: 48ffc6                       inc rsi
142296088: 49897030                     mov qword ptr [r8 + 0x30], rsi
14229608c: 49ff4040                     inc qword ptr [r8 + 0x40]
142296090: b001                         mov al, 1
142296092: 41f6c101                     test r9b, 1
142296096: 0f840cb9ffff                 je 0x1422919a8
14229609c: 8b85f0060000                 mov eax, dword ptr [rbp + 0x6f0]
1422960a2: 488b8d90060000               mov rcx, qword ptr [rbp + 0x690]
1422960a9: 8901                         mov dword ptr [rcx], eax
1422960ab: 488b8588060000               mov rax, qword ptr [rbp + 0x688]
1422960b2: 897820                       mov dword ptr [rax + 0x20], edi
1422960b5: 4584ff                       test r15b, r15b
1422960b8: 0f84ddfcffff                 je 0x142295d9b
1422960be: 488b8568060000               mov rax, qword ptr [rbp + 0x668]
1422960c5: 8b8dd8060000                 mov ecx, dword ptr [rbp + 0x6d8]
1422960cb: 8908                         mov dword ptr [rax], ecx
1422960cd: 418b7268                     mov esi, dword ptr [r10 + 0x68]
1422960d1: 0fb60531eb3901               movzx eax, byte ptr [rip + 0x139eb31]
1422960d8: b930000000                   mov ecx, 0x30
1422960dd: ba08000000                   mov edx, 8
1422960e2: e829db37fe                   call 0x140613c10
1422960e7: 4885c0                       test rax, rax
1422960ea: 0f84880b0000                 je 0x142296c78
1422960f0: 48c70001000000               mov qword ptr [rax], 1
1422960f7: 48c7400801000000             mov qword ptr [rax + 8], 1
1422960ff: c6401805                     mov byte ptr [rax + 0x18], 5
142296103: 48895820                     mov qword ptr [rax + 0x20], rbx
142296107: 897028                       mov dword ptr [rax + 0x28], esi
14229610a: 89782c                       mov dword ptr [rax + 0x2c], edi
14229610d: 48898538050000               mov qword ptr [rbp + 0x538], rax
142296114: 48c7853005000009000000       mov qword ptr [rbp + 0x530], 9
14229611f: 488b8d70060000               mov rcx, qword ptr [rbp + 0x670]
142296126: 488d9530050000               lea rdx, [rbp + 0x530]
14229612d: 4c8d052455da00               lea r8, [rip + 0xda5524]
142296134: e887ed7efe                   call 0x140a84ec0
142296139: e95dfcffff                   jmp 0x142295d9b
14229613e: 418b7234                     mov esi, dword ptr [r10 + 0x34]
142296142: 8b85d4020000                 mov eax, dword ptr [rbp + 0x2d4]
142296148: 488b8d50010000               mov rcx, qword ptr [rbp + 0x150]
14229614f: 8901                         mov dword ptr [rcx], eax
142296151: 488b8580030000               mov rax, qword ptr [rbp + 0x380]
142296158: 488b5830                     mov rbx, qword ptr [rax + 0x30]
14229615c: 4c8b6040                     mov r12, qword ptr [rax + 0x40]
142296160: 483b5820                     cmp rbx, qword ptr [rax + 0x20]
142296164: 751a                         jne 0x142296180
142296166: 488b8d88010000               mov rcx, qword ptr [rbp + 0x188]
14229616d: 488d155451da00               lea rdx, [rip + 0xda5154]
142296174: e837cc2cfe                   call 0x140562db0
142296179: 4c8b9508070000               mov r10, qword ptr [rbp + 0x708]
142296180: 8b8500070000                 mov eax, dword ptr [rbp + 0x700]
142296186: 2401                         and al, 1
142296188: 4c8b8580030000               mov r8, qword ptr [rbp + 0x380]
14229618f: 498b4828                     mov rcx, qword ptr [r8 + 0x28]
142296193: 488d145b                     lea rdx, [rbx + rbx*2]
142296197: 4c8924d1                     mov qword ptr [rcx + rdx*8], r12
14229619b: 8974d108                     mov dword ptr [rcx + rdx*8 + 8], esi
14229619f: 897cd10c                     mov dword ptr [rcx + rdx*8 + 0xc], edi
1422961a3: 8844d110                     mov byte ptr [rcx + rdx*8 + 0x10], al
1422961a7: 48ffc3                       inc rbx
1422961aa: 49895830                     mov qword ptr [r8 + 0x30], rbx
1422961ae: 49ff4040                     inc qword ptr [r8 + 0x40]
1422961b2: 4584ff                       test r15b, r15b
1422961b5: 0f84d1fbffff                 je 0x142295d8c
1422961bb: 488b8568060000               mov rax, qword ptr [rbp + 0x668]
1422961c2: 8b8dd8060000                 mov ecx, dword ptr [rbp + 0x6d8]
1422961c8: 8908                         mov dword ptr [rax], ecx
1422961ca: 418b5a68                     mov ebx, dword ptr [r10 + 0x68]
1422961ce: 0fb60534ea3901               movzx eax, byte ptr [rip + 0x139ea34]
1422961d5: b930000000                   mov ecx, 0x30
1422961da: ba08000000                   mov edx, 8
1422961df: e82cda37fe                   call 0x140613c10
1422961e4: 4885c0                       test rax, rax
1422961e7: 0f848b0a0000                 je 0x142296c78
1422961ed: 48c70001000000               mov qword ptr [rax], 1
1422961f4: 48c7400801000000             mov qword ptr [rax + 8], 1
1422961fc: c640180d                     mov byte ptr [rax + 0x18], 0xd
142296200: 89701c                       mov dword ptr [rax + 0x1c], esi
142296203: 895820                       mov dword ptr [rax + 0x20], ebx
142296206: 897824                       mov dword ptr [rax + 0x24], edi
142296209: 48898538050000               mov qword ptr [rbp + 0x538], rax
142296210: 48c7853005000009000000       mov qword ptr [rbp + 0x530], 9
14229621b: 488b8d70060000               mov rcx, qword ptr [rbp + 0x670]
142296222: 488d9530050000               lea rdx, [rbp + 0x530]
142296229: 4c8d05b854da00               lea r8, [rip + 0xda54b8]
142296230: e88bec7efe                   call 0x140a84ec0
142296235: 8b85f0060000                 mov eax, dword ptr [rbp + 0x6f0]
14229623b: 488b8d90060000               mov rcx, qword ptr [rbp + 0x690]
142296242: 8901                         mov dword ptr [rcx], eax
142296244: 488b8588060000               mov rax, qword ptr [rbp + 0x688]
14229624b: 89780c                       mov dword ptr [rax + 0xc], edi
14229624e: e948fbffff                   jmp 0x142295d9b
142296253: 498b5a38                     mov rbx, qword ptr [r10 + 0x38]
142296257: 48899de8020000               mov qword ptr [rbp + 0x2e8], rbx
14229625e: 8b85e4020000                 mov eax, dword ptr [rbp + 0x2e4]
142296264: 488b8d70010000               mov rcx, qword ptr [rbp + 0x170]
14229626b: 8901                         mov dword ptr [rcx], eax
14229626d: 488b85a8020000               mov rax, qword ptr [rbp + 0x2a8]
142296274: 488b7030                     mov rsi, qword ptr [rax + 0x30]
142296278: 4c8b6040                     mov r12, qword ptr [rax + 0x40]
14229627c: 483b7020                     cmp rsi, qword ptr [rax + 0x20]
142296280: 7513                         jne 0x142296295
142296282: 488b8da8010000               mov rcx, qword ptr [rbp + 0x1a8]
142296289: 488d153850da00               lea rdx, [rip + 0xda5038]
142296290: e81bcb2cfe                   call 0x140562db0
142296295: 448b8500070000               mov r8d, dword ptr [rbp + 0x700]
14229629c: 448b8dd0060000               mov r9d, dword ptr [rbp + 0x6d0]
1422962a3: 4520c1                       and r9b, r8b
1422962a6: 488b95a8020000               mov rdx, qword ptr [rbp + 0x2a8]
1422962ad: 488b4228                     mov rax, qword ptr [rdx + 0x28]
1422962b1: 488d0c76                     lea rcx, [rsi + rsi*2]
1422962b5: 4c8924c8                     mov qword ptr [rax + rcx*8], r12
1422962b9: 48895cc808                   mov qword ptr [rax + rcx*8 + 8], rbx
1422962be: 897cc810                     mov dword ptr [rax + rcx*8 + 0x10], edi
1422962c2: 44884cc814                   mov byte ptr [rax + rcx*8 + 0x14], r9b
1422962c7: 48ffc6                       inc rsi
1422962ca: 48897230                     mov qword ptr [rdx + 0x30], rsi
1422962ce: 48ff4240                     inc qword ptr [rdx + 0x40]
1422962d2: b001                         mov al, 1
1422962d4: 41f6c001                     test r8b, 1
1422962d8: 0f84cab6ffff                 je 0x1422919a8
1422962de: 488b8d88060000               mov rcx, qword ptr [rbp + 0x688]
1422962e5: 89791c                       mov dword ptr [rcx + 0x1c], edi
1422962e8: 8b85f0060000                 mov eax, dword ptr [rbp + 0x6f0]
1422962ee: 488b9590060000               mov rdx, qword ptr [rbp + 0x690]
1422962f5: 8902                         mov dword ptr [rdx], eax
1422962f7: 897918                       mov dword ptr [rcx + 0x18], edi
1422962fa: 488b95e8020000               mov rdx, qword ptr [rbp + 0x2e8]
142296301: 488b8d08020000               mov rcx, qword ptr [rbp + 0x208]
142296308: 4c8d050153da00               lea r8, [rip + 0xda5301]
14229630f: e8dcbb7ffe                   call 0x140a91ef0
142296314: 4885c0                       test rax, rax
142296317: 0f8463040000                 je 0x142296780
14229631d: 83b85802000002               cmp dword ptr [rax + 0x258], 2
142296324: 0f8336040000                 jae 0x142296760
14229632a: 480599020000                 add rax, 0x299
142296330: 4889c6                       mov rsi, rax
142296333: 4889c1                       mov rcx, rax
142296336: e835f362fe                   call 0x1408c5670
14229633b: 84c0                         test al, al
14229633d: 7440                         je 0x14229637f
14229633f: 488d8de8020000               lea rcx, [rbp + 0x2e8]
142296346: e88522dffd                   call 0x1400885d0
14229634b: 488b8db0010000               mov rcx, qword ptr [rbp + 0x1b0]
142296352: 488b5110                     mov rdx, qword ptr [rcx + 0x10]
142296356: 4839d0                       cmp rax, rdx
142296359: 0f83bf090000                 jae 0x142296d1e
14229635f: 488b4908                     mov rcx, qword ptr [rcx + 8]
142296363: 8b0481                       mov eax, dword ptr [rcx + rax*4]
142296366: 8b8df0060000                 mov ecx, dword ptr [rbp + 0x6f0]
14229636c: 488b9590060000               mov rdx, qword ptr [rbp + 0x690]
142296373: 890a                         mov dword ptr [rdx], ecx
142296375: 488b8d88060000               mov rcx, qword ptr [rbp + 0x688]
14229637c: 894124                       mov dword ptr [rcx + 0x24], eax
14229637f: 0fb606                       movzx eax, byte ptr [rsi]
142296382: 8b8df0060000                 mov ecx, dword ptr [rbp + 0x6f0]
142296388: 488b9590060000               mov rdx, qword ptr [rbp + 0x690]
14229638f: 890a                         mov dword ptr [rdx], ecx
142296391: 488b8d88060000               mov rcx, qword ptr [rbp + 0x688]
142296398: 884142                       mov byte ptr [rcx + 0x42], al
14229639b: e9e0030000                   jmp 0x142296780
1422963a0: 498b5a38                     mov rbx, qword ptr [r10 + 0x38]
1422963a4: 488b8560010000               mov rax, qword ptr [rbp + 0x160]
1422963ab: 8b8ddc020000                 mov ecx, dword ptr [rbp + 0x2dc]
1422963b1: 8908                         mov dword ptr [rax], ecx
1422963b3: 488b8590030000               mov rax, qword ptr [rbp + 0x390]
1422963ba: 488b7030                     mov rsi, qword ptr [rax + 0x30]
1422963be: 4c8b6040                     mov r12, qword ptr [rax + 0x40]
1422963c2: 483b7020                     cmp rsi, qword ptr [rax + 0x20]
1422963c6: 751a                         jne 0x1422963e2
1422963c8: 488b8d90010000               mov rcx, qword ptr [rbp + 0x190]
1422963cf: 488d15f24eda00               lea rdx, [rip + 0xda4ef2]
1422963d6: e8d5c92cfe                   call 0x140562db0
1422963db: 4c8b9508070000               mov r10, qword ptr [rbp + 0x708]
1422963e2: 448b8d00070000               mov r9d, dword ptr [rbp + 0x700]
1422963e9: 4489c8                       mov eax, r9d
1422963ec: 2401                         and al, 1
1422963ee: 4c8b8590030000               mov r8, qword ptr [rbp + 0x390]
1422963f5: 498b4828                     mov rcx, qword ptr [r8 + 0x28]
1422963f9: 488d1476                     lea rdx, [rsi + rsi*2]
1422963fd: 4c8924d1                     mov qword ptr [rcx + rdx*8], r12
142296401: 48895cd108                   mov qword ptr [rcx + rdx*8 + 8], rbx
142296406: 897cd110                     mov dword ptr [rcx + rdx*8 + 0x10], edi
14229640a: 8844d114                     mov byte ptr [rcx + rdx*8 + 0x14], al
14229640e: 48ffc6                       inc rsi
142296411: 49897030                     mov qword ptr [r8 + 0x30], rsi
142296415: 49ff4040                     inc qword ptr [r8 + 0x40]
142296419: b001                         mov al, 1
14229641b: 41f6c101                     test r9b, 1
14229641f: 0f8483b5ffff                 je 0x1422919a8
142296425: 8b85f0060000                 mov eax, dword ptr [rbp + 0x6f0]
14229642b: 488b8d90060000               mov rcx, qword ptr [rbp + 0x690]
142296432: 8901                         mov dword ptr [rcx], eax
142296434: 488b8588060000               mov rax, qword ptr [rbp + 0x688]
14229643b: 897838                       mov dword ptr [rax + 0x38], edi
14229643e: 4584ff                       test r15b, r15b
142296441: 0f8454f9ffff                 je 0x142295d9b
142296447: 488b8568060000               mov rax, qword ptr [rbp + 0x668]
14229644e: 8b8dd8060000                 mov ecx, dword ptr [rbp + 0x6d8]
142296454: 8908                         mov dword ptr [rax], ecx
142296456: 418b7268                     mov esi, dword ptr [r10 + 0x68]
14229645a: 0fb605a8e73901               movzx eax, byte ptr [rip + 0x139e7a8]
142296461: b930000000                   mov ecx, 0x30
142296466: ba08000000                   mov edx, 8
14229646b: e8a0d737fe                   call 0x140613c10
142296470: 4885c0                       test rax, rax
142296473: 0f84ff070000                 je 0x142296c78
142296479: 48c70001000000               mov qword ptr [rax], 1
142296480: 48c7400801000000             mov qword ptr [rax + 8], 1
142296488: c6401804                     mov byte ptr [rax + 0x18], 4
14229648c: 48895820                     mov qword ptr [rax + 0x20], rbx
142296490: 897028                       mov dword ptr [rax + 0x28], esi
142296493: 89782c                       mov dword ptr [rax + 0x2c], edi
142296496: 48898538050000               mov qword ptr [rbp + 0x538], rax
14229649d: 48c7853005000009000000       mov qword ptr [rbp + 0x530], 9
1422964a8: 488b8d70060000               mov rcx, qword ptr [rbp + 0x670]
1422964af: 488d9530050000               lea rdx, [rbp + 0x530]
1422964b6: 4c8d05cb51da00               lea r8, [rip + 0xda51cb]
1422964bd: e8fee97efe                   call 0x140a84ec0
1422964c2: e9d4f8ffff                   jmp 0x142295d9b
1422964c7: 418b7234                     mov esi, dword ptr [r10 + 0x34]
1422964cb: 488b8568010000               mov rax, qword ptr [rbp + 0x168]
1422964d2: 8b8de0020000                 mov ecx, dword ptr [rbp + 0x2e0]
1422964d8: 8908                         mov dword ptr [rax], ecx
1422964da: 488b8598030000               mov rax, qword ptr [rbp + 0x398]
1422964e1: 488b5830                     mov rbx, qword ptr [rax + 0x30]
1422964e5: 4c8b6040                     mov r12, qword ptr [rax + 0x40]
1422964e9: 483b5820                     cmp rbx, qword ptr [rax + 0x20]
1422964ed: 751a                         jne 0x142296509
1422964ef: 488b8d98010000               mov rcx, qword ptr [rbp + 0x198]
1422964f6: 488d15cb4dda00               lea rdx, [rip + 0xda4dcb]
1422964fd: e8aec82cfe                   call 0x140562db0
142296502: 4c8b9508070000               mov r10, qword ptr [rbp + 0x708]
142296509: 8b8500070000                 mov eax, dword ptr [rbp + 0x700]
14229650f: 2401                         and al, 1
142296511: 4c8b8598030000               mov r8, qword ptr [rbp + 0x398]
142296518: 498b4828                     mov rcx, qword ptr [r8 + 0x28]
14229651c: 488d145b                     lea rdx, [rbx + rbx*2]
142296520: 4c8924d1                     mov qword ptr [rcx + rdx*8], r12
142296524: 8974d108                     mov dword ptr [rcx + rdx*8 + 8], esi
142296528: 897cd10c                     mov dword ptr [rcx + rdx*8 + 0xc], edi
14229652c: 8844d110                     mov byte ptr [rcx + rdx*8 + 0x10], al
142296530: 48ffc3                       inc rbx
142296533: 49895830                     mov qword ptr [r8 + 0x30], rbx
142296537: 49ff4040                     inc qword ptr [r8 + 0x40]
14229653b: 4584ff                       test r15b, r15b
14229653e: 0f8448f8ffff                 je 0x142295d8c
142296544: 8b85f0060000                 mov eax, dword ptr [rbp + 0x6f0]
14229654a: 488b8d90060000               mov rcx, qword ptr [rbp + 0x690]
142296551: 8901                         mov dword ptr [rcx], eax
142296553: 488b8588060000               mov rax, qword ptr [rbp + 0x688]
14229655a: 897830                       mov dword ptr [rax + 0x30], edi
14229655d: 488b8568060000               mov rax, qword ptr [rbp + 0x668]
142296564: 8b8dd8060000                 mov ecx, dword ptr [rbp + 0x6d8]
14229656a: 8908                         mov dword ptr [rax], ecx
14229656c: 418b5a68                     mov ebx, dword ptr [r10 + 0x68]
142296570: 0fb60592e63901               movzx eax, byte ptr [rip + 0x139e692]
142296577: b930000000                   mov ecx, 0x30
14229657c: ba08000000                   mov edx, 8
142296581: e88ad637fe                   call 0x140613c10
142296586: 4885c0                       test rax, rax
142296589: 0f84e9060000                 je 0x142296c78
14229658f: 48c70001000000               mov qword ptr [rax], 1
142296596: 48c7400801000000             mov qword ptr [rax + 8], 1
14229659e: c6401808                     mov byte ptr [rax + 0x18], 8
1422965a2: 89701c                       mov dword ptr [rax + 0x1c], esi
1422965a5: 895820                       mov dword ptr [rax + 0x20], ebx
1422965a8: 897824                       mov dword ptr [rax + 0x24], edi
1422965ab: 48898538050000               mov qword ptr [rbp + 0x538], rax
1422965b2: 48c7853005000009000000       mov qword ptr [rbp + 0x530], 9
1422965bd: 488b8d70060000               mov rcx, qword ptr [rbp + 0x670]
1422965c4: 488d9530050000               lea rdx, [rbp + 0x530]
1422965cb: 4c8d059e50da00               lea r8, [rip + 0xda509e]
1422965d2: e8e9e87efe                   call 0x140a84ec0
1422965d7: e9bff7ffff                   jmp 0x142295d9b
1422965dc: 488b8588060000               mov rax, qword ptr [rbp + 0x688]
1422965e3: 89782c                       mov dword ptr [rax + 0x2c], edi
1422965e6: 4584ff                       test r15b, r15b
1422965e9: 0f84acf7ffff                 je 0x142295d9b
1422965ef: 488b8568060000               mov rax, qword ptr [rbp + 0x668]
1422965f6: 8b8dd8060000                 mov ecx, dword ptr [rbp + 0x6d8]
1422965fc: 8908                         mov dword ptr [rax], ecx
1422965fe: 488bb5e8030000               mov rsi, qword ptr [rbp + 0x3e8]
142296605: 418b5a68                     mov ebx, dword ptr [r10 + 0x68]
142296609: 0fb605f9e53901               movzx eax, byte ptr [rip + 0x139e5f9]
142296610: b930000000                   mov ecx, 0x30
142296615: ba08000000                   mov edx, 8
14229661a: e8f1d537fe                   call 0x140613c10
14229661f: 4885c0                       test rax, rax
142296622: 0f8450060000                 je 0x142296c78
142296628: 48c70001000000               mov qword ptr [rax], 1
14229662f: 48c7400801000000             mov qword ptr [rax + 8], 1
142296637: c6401809                     mov byte ptr [rax + 0x18], 9
14229663b: 48897020                     mov qword ptr [rax + 0x20], rsi
14229663f: 895828                       mov dword ptr [rax + 0x28], ebx
142296642: 89782c                       mov dword ptr [rax + 0x2c], edi
142296645: 48898538050000               mov qword ptr [rbp + 0x538], rax
14229664c: 48c7853005000009000000       mov qword ptr [rbp + 0x530], 9
142296657: 488b8d70060000               mov rcx, qword ptr [rbp + 0x670]
14229665e: 488d9530050000               lea rdx, [rbp + 0x530]
142296665: 4c8d054c50da00               lea r8, [rip + 0xda504c]
14229666c: e84fe87efe                   call 0x140a84ec0
142296671: e925f7ffff                   jmp 0x142295d9b
142296676: 8b85f0060000                 mov eax, dword ptr [rbp + 0x6f0]
14229667c: 488b8d90060000               mov rcx, qword ptr [rbp + 0x690]
142296683: 8901                         mov dword ptr [rcx], eax
142296685: 488b8588060000               mov rax, qword ptr [rbp + 0x688]
14229668c: 897828                       mov dword ptr [rax + 0x28], edi
14229668f: 4584ff                       test r15b, r15b
142296692: 0f8403f7ffff                 je 0x142295d9b
142296698: eb46                         jmp 0x1422966e0
14229669a: 8b85f0060000                 mov eax, dword ptr [rbp + 0x6f0]
1422966a0: 488b8d90060000               mov rcx, qword ptr [rbp + 0x690]
1422966a7: 8901                         mov dword ptr [rcx], eax
1422966a9: 488b8588060000               mov rax, qword ptr [rbp + 0x688]
1422966b0: 897834                       mov dword ptr [rax + 0x34], edi
1422966b3: 4584ff                       test r15b, r15b
1422966b6: 0f84dff6ffff                 je 0x142295d9b
1422966bc: eb22                         jmp 0x1422966e0
1422966be: 8b85f0060000                 mov eax, dword ptr [rbp + 0x6f0]
1422966c4: 488b8d90060000               mov rcx, qword ptr [rbp + 0x690]
1422966cb: 8901                         mov dword ptr [rcx], eax
1422966cd: 488b8588060000               mov rax, qword ptr [rbp + 0x688]
1422966d4: 897808                       mov dword ptr [rax + 8], edi
1422966d7: 4584ff                       test r15b, r15b
1422966da: 0f84bbf6ffff                 je 0x142295d9b
1422966e0: 488b8568060000               mov rax, qword ptr [rbp + 0x668]
1422966e7: 8b8dd8060000                 mov ecx, dword ptr [rbp + 0x6d8]
1422966ed: 8908                         mov dword ptr [rax], ecx
1422966ef: 418b7268                     mov esi, dword ptr [r10 + 0x68]
1422966f3: 0fb6050fe53901               movzx eax, byte ptr [rip + 0x139e50f]
1422966fa: b930000000                   mov ecx, 0x30
1422966ff: ba08000000                   mov edx, 8
142296704: e807d537fe                   call 0x140613c10
142296709: 4885c0                       test rax, rax
14229670c: 0f8466050000                 je 0x142296c78
142296712: 48c70001000000               mov qword ptr [rax], 1
142296719: 48c7400801000000             mov qword ptr [rax + 8], 1
142296721: c6401807                     mov byte ptr [rax + 0x18], 7
142296725: 4489601c                     mov dword ptr [rax + 0x1c], r12d
142296729: 897020                       mov dword ptr [rax + 0x20], esi
14229672c: 897824                       mov dword ptr [rax + 0x24], edi
14229672f: 48898538050000               mov qword ptr [rbp + 0x538], rax
142296736: 48c7853005000009000000       mov qword ptr [rbp + 0x530], 9
142296741: 488b8d70060000               mov rcx, qword ptr [rbp + 0x670]
142296748: 488d9530050000               lea rdx, [rbp + 0x530]
14229674f: 4c8d057a4fda00               lea r8, [rip + 0xda4f7a]
142296756: e865e77efe                   call 0x140a84ec0
14229675b: e93bf6ffff                   jmp 0x142295d9b
142296760: 0fb68095020000               movzx eax, byte ptr [rax + 0x295]
142296767: 8b8df0060000                 mov ecx, dword ptr [rbp + 0x6f0]
14229676d: 488b9590060000               mov rdx, qword ptr [rbp + 0x690]
142296774: 890a                         mov dword ptr [rdx], ecx
142296776: 488b8d88060000               mov rcx, qword ptr [rbp + 0x688]
14229677d: 884140                       mov byte ptr [rcx + 0x40], al
142296780: 4584ff                       test r15b, r15b
142296783: 488b9508070000               mov rdx, qword ptr [rbp + 0x708]
14229678a: 0f840bf6ffff                 je 0x142295d9b
142296790: 488b8568060000               mov rax, qword ptr [rbp + 0x668]
142296797: 8b8dd8060000                 mov ecx, dword ptr [rbp + 0x6d8]
14229679d: 8908                         mov dword ptr [rax], ecx
14229679f: 8b7268                       mov esi, dword ptr [rdx + 0x68]
1422967a2: 488b9de8020000               mov rbx, qword ptr [rbp + 0x2e8]
1422967a9: 0fb60559e43901               movzx eax, byte ptr [rip + 0x139e459]
1422967b0: b930000000                   mov ecx, 0x30
1422967b5: ba08000000                   mov edx, 8
1422967ba: e851d437fe                   call 0x140613c10
1422967bf: 4885c0                       test rax, rax
1422967c2: 0f84b0040000                 je 0x142296c78
1422967c8: 48c70001000000               mov qword ptr [rax], 1
1422967cf: 48c7400801000000             mov qword ptr [rax + 8], 1
1422967d7: c6401803                     mov byte ptr [rax + 0x18], 3
1422967db: 48895820                     mov qword ptr [rax + 0x20], rbx
1422967df: 897028                       mov dword ptr [rax + 0x28], esi
1422967e2: 89782c                       mov dword ptr [rax + 0x2c], edi
1422967e5: 48898538050000               mov qword ptr [rbp + 0x538], rax
1422967ec: 48c7853005000009000000       mov qword ptr [rbp + 0x530], 9
1422967f7: 488b8d70060000               mov rcx, qword ptr [rbp + 0x670]
1422967fe: 488d9530050000               lea rdx, [rbp + 0x530]
142296805: 4c8d05344eda00               lea r8, [rip + 0xda4e34]
14229680c: e8afe67efe                   call 0x140a84ec0
142296811: e985f5ffff                   jmp 0x142295d9b
142296816: 8b85f0060000                 mov eax, dword ptr [rbp + 0x6f0]
14229681c: 488b8d90060000               mov rcx, qword ptr [rbp + 0x690]
142296823: 8901                         mov dword ptr [rcx], eax
142296825: 488b8588060000               mov rax, qword ptr [rbp + 0x688]
14229682c: c70001000000                 mov dword ptr [rax], 1
142296832: 897804                       mov dword ptr [rax + 4], edi
142296835: 4584ff                       test r15b, r15b
142296838: 0f845df5ffff                 je 0x142295d9b
14229683e: e99dfeffff                   jmp 0x1422966e0
142296843: 4c8b8530080000               mov r8, qword ptr [rbp + 0x830]
14229684a: a801                         test al, 1
14229684c: 743f                         je 0x14229688d
14229684e: 410fb600                     movzx eax, byte ptr [r8]
142296852: 89c2                         mov edx, eax
142296854: 4c8b8d50030000               mov r9, qword ptr [rbp + 0x350]
14229685b: 4420ca                       and dl, r9b
14229685e: 4489c9                       mov ecx, r9d
142296861: f6c201                       test dl, 1
142296864: 4c8bb580060000               mov r14, qword ptr [rbp + 0x680]
14229686b: 743a                         je 0x1422968a7
14229686d: 488b8510020000               mov rax, qword ptr [rbp + 0x210]
142296874: 49394008                     cmp qword ptr [r8 + 8], rax
142296878: 7538                         jne 0x1422968b2
14229687a: 488b8530020000               mov rax, qword ptr [rbp + 0x230]
142296881: 49394010                     cmp qword ptr [r8 + 0x10], rax
142296885: 0f84ae000000                 je 0x142296939
14229688b: eb25                         jmp 0x1422968b2
14229688d: 49c70000000000               mov qword ptr [r8], 0
142296894: 31c0                         xor eax, eax
142296896: 4c8b8d50030000               mov r9, qword ptr [rbp + 0x350]
14229689d: 4489c9                       mov ecx, r9d
1422968a0: 4c8bb580060000               mov r14, qword ptr [rbp + 0x680]
1422968a7: 08c1                         or cl, al
1422968a9: f6c101                       test cl, 1
1422968ac: 0f8487000000                 je 0x142296939
1422968b2: 4d8908                       mov qword ptr [r8], r9
1422968b5: 488b8510020000               mov rax, qword ptr [rbp + 0x210]
1422968bc: 49894008                     mov qword ptr [r8 + 8], rax
1422968c0: 488b8530020000               mov rax, qword ptr [rbp + 0x230]
1422968c7: 49894010                     mov qword ptr [r8 + 0x10], rax
1422968cb: 83bdf805000000               cmp dword ptr [rbp + 0x5f8], 0
1422968d2: 7465                         je 0x142296939
1422968d4: 488b9558080000               mov rdx, qword ptr [rbp + 0x858]
1422968db: 8b421c                       mov eax, dword ptr [rdx + 0x1c]
1422968de: 488b4a10                     mov rcx, qword ptr [rdx + 0x10]
1422968e2: 8901                         mov dword ptr [rcx], eax
1422968e4: 488b32                       mov rsi, qword ptr [rdx]
1422968e7: 488b7e30                     mov rdi, qword ptr [rsi + 0x30]
1422968eb: 488b5e40                     mov rbx, qword ptr [rsi + 0x40]
1422968ef: 483b7e20                     cmp rdi, qword ptr [rsi + 0x20]
1422968f3: 7510                         jne 0x142296905
1422968f5: 488d4e20                     lea rcx, [rsi + 0x20]
1422968f9: 488d15c849da00               lea rdx, [rip + 0xda49c8]
142296900: e82bc32cfe                   call 0x140562c30
142296905: 488b4628                     mov rax, qword ptr [rsi + 0x28]
142296909: 4889f9                       mov rcx, rdi
14229690c: 48c1e105                     shl rcx, 5
142296910: 8b95f8050000                 mov edx, dword ptr [rbp + 0x5f8]
142296916: 891408                       mov dword ptr [rax + rcx], edx
142296919: 0f284560                     movaps xmm0, xmmword ptr [rbp + 0x60]
14229691d: 0f11440804                   movups xmmword ptr [rax + rcx + 4], xmm0
142296922: 8b5570                       mov edx, dword ptr [rbp + 0x70]
142296925: 89540814                     mov dword ptr [rax + rcx + 0x14], edx
142296929: 48895c0818                   mov qword ptr [rax + rcx + 0x18], rbx
14229692e: 48ffc7                       inc rdi
142296931: 48897e30                     mov qword ptr [rsi + 0x30], rdi
142296935: 48ff4640                     inc qword ptr [rsi + 0x40]
142296939: 41f6c601                     test r14b, 1
14229693d: 0f8443020000                 je 0x142296b86
142296943: 4c8b8d28050000               mov r9, qword ptr [rbp + 0x528]
14229694a: 498b81f8000000               mov rax, qword ptr [r9 + 0xf8]
142296951: 498b8900010000               mov rcx, qword ptr [r9 + 0x100]
142296958: 488d0c88                     lea rcx, [rax + rcx*4]
14229695c: 488b9500020000               mov rdx, qword ptr [rbp + 0x200]
142296963: 448b421c                     mov r8d, dword ptr [rdx + 0x1c]
142296967: 44898508060000               mov dword ptr [rbp + 0x608], r8d
14229696e: 4c8b4210                     mov r8, qword ptr [rdx + 0x10]
142296972: 4180b90801000000             cmp byte ptr [r9 + 0x108], 0
14229697a: 0f84fe000000                 je 0x142296a7e
142296980: 41bb08000000                 mov r11d, 8
142296986: 4531c9                       xor r9d, r9d
142296989: 31f6                         xor esi, esi
14229698b: 4531d2                       xor r10d, r10d
14229698e: 31db                         xor ebx, ebx
142296990: 4c39d3                       cmp rbx, r10
142296993: 757b                         jne 0x142296a10
142296995: 488bbd00050000               mov rdi, qword ptr [rbp + 0x500]
14229699c: 0f1f4000                     nop dword ptr [rax]
1422969a0: 4839c8                       cmp rax, rcx
1422969a3: 0f84dd010000                 je 0x142296b86
1422969a9: 448b10                       mov r10d, dword ptr [rax]
1422969ac: 4883c004                     add rax, 4
1422969b0: 4c8b9fa8010000               mov r11, qword ptr [rdi + 0x1a8]
1422969b7: 4b8d34d2                     lea rsi, [r10 + r10*8]
1422969bb: 4d8b54f310                   mov r10, qword ptr [r11 + rsi*8 + 0x10]
1422969c0: 4d85d2                       test r10, r10
1422969c3: 74db                         je 0x1422969a0
1422969c5: 498d34f3                     lea rsi, [r11 + rsi*8]
1422969c9: 4c8b9d28050000               mov r11, qword ptr [rbp + 0x528]
1422969d0: 4d8b9b10010000               mov r11, qword ptr [r11 + 0x110]
1422969d7: 488b7e38                     mov rdi, qword ptr [rsi + 0x38]
1422969db: 4a8b3cdf                     mov rdi, qword ptr [rdi + r11*8]
1422969df: 4c8b5e08                     mov r11, qword ptr [rsi + 8]
1422969e3: 488b5e18                     mov rbx, qword ptr [rsi + 0x18]
1422969e7: 48f7d7                       not rdi
1422969ea: 488d3c7f                     lea rdi, [rdi + rdi*2]
1422969ee: 48c1e704                     shl rdi, 4
1422969f2: 488b743b10                   mov rsi, qword ptr [rbx + rdi + 0x10]
1422969f7: 488b7c3b28                   mov rdi, qword ptr [rbx + rdi + 0x28]
1422969fc: 4531e4                       xor r12d, r12d
1422969ff: eb12                         jmp 0x142296a13
142296a01: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
142296a10: 4989dc                       mov r12, rbx
142296a13: 4f8b34e3                     mov r14, qword ptr [r11 + r12*8]
142296a17: 4c89f3                       mov rbx, r14
142296a1a: 48c1eb20                     shr rbx, 0x20
142296a1e: 0f8462010000                 je 0x142296b86
142296a24: 498d5c2401                   lea rbx, [r12 + 1]
142296a29: 4d8d7901                     lea r15, [r9 + 1]
142296a2d: 4589e4                       mov r12d, r12d
142296a30: 4f8d2ca4                     lea r13, [r12 + r12*4]
142296a34: 49c1e505                     shl r13, 5
142296a38: 8b95ec050000                 mov edx, dword ptr [rbp + 0x5ec]
142296a3e: 428914a7                     mov dword ptr [rdi + r12*4], edx
142296a42: 4ac7042e00000000             mov qword ptr [rsi + r13], 0
142296a4a: 4c3b8d58060000               cmp r9, qword ptr [rbp + 0x658]
142296a51: 4d89f9                       mov r9, r15
142296a54: 0f8536ffffff                 jne 0x142296990
142296a5a: 8b9508060000                 mov edx, dword ptr [rbp + 0x608]
142296a60: 418910                       mov dword ptr [r8], edx
142296a63: 488b9538030000               mov rdx, qword ptr [rbp + 0x338]
142296a6a: 48c7421001000000             mov qword ptr [rdx + 0x10], 1
142296a72: 4c897218                     mov qword ptr [rdx + 0x18], r14
142296a76: 4d89f9                       mov r9, r15
142296a79: e912ffffff                   jmp 0x142296990
142296a7e: 41bb08000000                 mov r11d, 8
142296a84: 4531c9                       xor r9d, r9d
142296a87: 31f6                         xor esi, esi
142296a89: 4531d2                       xor r10d, r10d
142296a8c: 31db                         xor ebx, ebx
142296a8e: 4c39d3                       cmp rbx, r10
142296a91: 0f857e000000                 jne 0x142296b15
142296a97: 488bbd00050000               mov rdi, qword ptr [rbp + 0x500]
142296a9e: 6690                         nop 
142296aa0: 4839c8                       cmp rax, rcx
142296aa3: 0f84dd000000                 je 0x142296b86
142296aa9: 448b10                       mov r10d, dword ptr [rax]
142296aac: 4883c004                     add rax, 4
142296ab0: 4c8b9f00010000               mov r11, qword ptr [rdi + 0x100]
142296ab7: 4b8d3492                     lea rsi, [r10 + r10*4]
142296abb: 48c1e605                     shl rsi, 5
142296abf: 4d8b543358                   mov r10, qword ptr [r11 + rsi + 0x58]
142296ac4: 4d85d2                       test r10, r10
142296ac7: 74d7                         je 0x142296aa0
142296ac9: 4901f3                       add r11, rsi
142296acc: 418bb394000000               mov esi, dword ptr [r11 + 0x94]
142296ad3: 488bbfa8010000               mov rdi, qword ptr [rdi + 0x1a8]
142296ada: 488d34f6                     lea rsi, [rsi + rsi*8]
142296ade: 488b9d28050000               mov rbx, qword ptr [rbp + 0x528]
142296ae5: 488b9b10010000               mov rbx, qword ptr [rbx + 0x110]
142296aec: 4c8b74f718                   mov r14, qword ptr [rdi + rsi*8 + 0x18]
142296af1: 488b74f738                   mov rsi, qword ptr [rdi + rsi*8 + 0x38]
142296af6: 488b34de                     mov rsi, qword ptr [rsi + rbx*8]
142296afa: 48f7d6                       not rsi
142296afd: 488d3c76                     lea rdi, [rsi + rsi*2]
142296b01: 48c1e704                     shl rdi, 4
142296b05: 498b743e10                   mov rsi, qword ptr [r14 + rdi + 0x10]
142296b0a: 498b7c3e28                   mov rdi, qword ptr [r14 + rdi + 0x28]
142296b0f: 4d8b5b50                     mov r11, qword ptr [r11 + 0x50]
142296b13: 31db                         xor ebx, ebx
142296b15: 4989dc                       mov r12, rbx
142296b18: 49c1e404                     shl r12, 4
142296b1c: 4f8b3423                     mov r14, qword ptr [r11 + r12]
142296b20: 4d89f7                       mov r15, r14
142296b23: 49c1ef20                     shr r15, 0x20
142296b27: 745d                         je 0x142296b86
142296b29: 4d01dc                       add r12, r11
142296b2c: 48ffc3                       inc rbx
142296b2f: 4d8d7901                     lea r15, [r9 + 1]
142296b33: 458b642408                   mov r12d, dword ptr [r12 + 8]
142296b38: 4f8d2ca4                     lea r13, [r12 + r12*4]
142296b3c: 49c1e505                     shl r13, 5
142296b40: 8b95ec050000                 mov edx, dword ptr [rbp + 0x5ec]
142296b46: 428914a7                     mov dword ptr [rdi + r12*4], edx
142296b4a: 4ac7042e00000000             mov qword ptr [rsi + r13], 0
142296b52: 4c3b8d58060000               cmp r9, qword ptr [rbp + 0x658]
142296b59: 4d89f9                       mov r9, r15
142296b5c: 0f852cffffff                 jne 0x142296a8e
142296b62: 8b9508060000                 mov edx, dword ptr [rbp + 0x608]
142296b68: 418910                       mov dword ptr [r8], edx
142296b6b: 488b9538030000               mov rdx, qword ptr [rbp + 0x338]
142296b72: 48c7421001000000             mov qword ptr [rdx + 0x10], 1
142296b7a: 4c897218                     mov qword ptr [rdx + 0x18], r14
142296b7e: 4d89f9                       mov r9, r15
142296b81: e908ffffff                   jmp 0x142296a8e
142296b86: 0f28b520070000               movaps xmm6, xmmword ptr [rbp + 0x720]
142296b8d: 0f28bd30070000               movaps xmm7, xmmword ptr [rbp + 0x730]
142296b94: 440f288540070000             movaps xmm8, xmmword ptr [rbp + 0x740]
142296b9c: 440f288d50070000             movaps xmm9, xmmword ptr [rbp + 0x750]
142296ba4: 440f289560070000             movaps xmm10, xmmword ptr [rbp + 0x760]
142296bac: 440f289d70070000             movaps xmm11, xmmword ptr [rbp + 0x770]
142296bb4: 440f28a580070000             movaps xmm12, xmmword ptr [rbp + 0x780]
142296bbc: 440f28ad90070000             movaps xmm13, xmmword ptr [rbp + 0x790]
142296bc4: 440f28b5a0070000             movaps xmm14, xmmword ptr [rbp + 0x7a0]
142296bcc: 440f28bdb0070000             movaps xmm15, xmmword ptr [rbp + 0x7b0]
142296bd4: 4881c448080000               add rsp, 0x848
142296bdb: 5b                           pop rbx
142296bdc: 5f                           pop rdi
142296bdd: 5e                           pop rsi
142296bde: 415c                         pop r12
142296be0: 415d                         pop r13
142296be2: 415e                         pop r14
142296be4: 415f                         pop r15
142296be6: 5d                           pop rbp
142296be7: c3                           ret 
142296be8: 48c1e820                     shr rax, 0x20
142296bec: ba01000000                   mov edx, 1
142296bf1: 4c8b85b0030000               mov r8, qword ptr [rbp + 0x3b0]
142296bf8: 899530050000                 mov dword ptr [rbp + 0x530], edx
142296bfe: 44898534050000               mov dword ptr [rbp + 0x534], r8d
142296c05: 48c1e020                     shl rax, 0x20
142296c09: 4809c8                       or rax, rcx
142296c0c: 48898538050000               mov qword ptr [rbp + 0x538], rax
142296c13: 488d05e64ada00               lea rax, [rip + 0xda4ae6]
142296c1a: 4889442420                   mov qword ptr [rsp + 0x20], rax
142296c1f: 488d0dea46da00               lea rcx, [rip + 0xda46ea]
142296c26: 4c8d0dc346da00               lea r9, [rip + 0xda46c3]
142296c2d: 4c8d8530050000               lea r8, [rbp + 0x530]
142296c34: ba2b000000                   mov edx, 0x2b
142296c39: e8222b6400                   call 0x1428d9760
142296c3e: 4c8d055b48da00               lea r8, [rip + 0xda485b]
142296c45: 4889d1                       mov rcx, rdx
142296c48: 4889c2                       mov rdx, rax
142296c4b: e8c8286400                   call 0x1428d9518
142296c50: 4c8d056148da00               lea r8, [rip + 0xda4861]
142296c57: 4889d1                       mov rcx, rdx
142296c5a: 4889c2                       mov rdx, rax
142296c5d: e8b6286400                   call 0x1428d9518
142296c62: 4c8d051f47da00               lea r8, [rip + 0xda471f]
142296c69: b901000000                   mov ecx, 1
142296c6e: ba1f000000                   mov edx, 0x1f
142296c73: e86b236400                   call 0x1428d8fe3
142296c78: b908000000                   mov ecx, 8
142296c7d: ba30000000                   mov edx, 0x30
142296c82: e879236400                   call 0x1428d9000
142296c87: 4c8d05fa46da00               lea r8, [rip + 0xda46fa]
142296c8e: b901000000                   mov ecx, 1
142296c93: ba1e000000                   mov edx, 0x1e
142296c98: e846236400                   call 0x1428d8fe3
142296c9d: 488d0d5449da00               lea rcx, [rip + 0xda4954]
142296ca4: e867266400                   call 0x1428d9310
142296ca9: 488d0d1849da00               lea rcx, [rip + 0xda4918]
142296cb0: e85b266400                   call 0x1428d9310
142296cb5: 488d0ddc48da00               lea rcx, [rip + 0xda48dc]
142296cbc: e84f266400                   call 0x1428d9310
142296cc1: 488d0dc047da00               lea rcx, [rip + 0xda47c0]
142296cc8: e843266400                   call 0x1428d9310
142296ccd: 4c8d05b446da00               lea r8, [rip + 0xda46b4]
142296cd4: b901000000                   mov ecx, 1
142296cd9: ba21000000                   mov edx, 0x21
142296cde: e800236400                   call 0x1428d8fe3
142296ce3: 4c8d059e46da00               lea r8, [rip + 0xda469e]
142296cea: b901000000                   mov ecx, 1
142296cef: ba1d000000                   mov edx, 0x1d
142296cf4: e8ea226400                   call 0x1428d8fe3
142296cf9: 4c8d058846da00               lea r8, [rip + 0xda4688]
142296d00: b901000000                   mov ecx, 1
142296d05: ba1a000000                   mov edx, 0x1a
142296d0a: e8d4226400                   call 0x1428d8fe3
142296d0f: 4c8d058a49da00               lea r8, [rip + 0xda498a]
142296d16: 4889c1                       mov rcx, rax
142296d19: e8fa276400                   call 0x1428d9518
142296d1e: 4c8d050349da00               lea r8, [rip + 0xda4903]
142296d25: 4889c1                       mov rcx, rax
142296d28: e8eb276400                   call 0x1428d9518
142296d2d: cc                           int3 
142296d2e: 6690                         nop 
142296d30: 4889542410                   mov qword ptr [rsp + 0x10], rdx
142296d35: 55                           push rbp
142296d36: 4157                         push r15
142296d38: 4156                         push r14
142296d3a: 4155                         push r13
142296d3c: 4154                         push r12
142296d3e: 56                           push rsi
142296d3f: 57                           push rdi
142296d40: 53                           push rbx
142296d41: 4881ecc8000000               sub rsp, 0xc8
142296d48: 488daa80000000               lea rbp, [rdx + 0x80]
142296d4f: 440f297c2420                 movaps xmmword ptr [rsp + 0x20], xmm15
142296d55: 440f29742430                 movaps xmmword ptr [rsp + 0x30], xmm14
142296d5b: 440f296c2440                 movaps xmmword ptr [rsp + 0x40], xmm13
142296d61: 440f29642450                 movaps xmmword ptr [rsp + 0x50], xmm12
142296d67: 440f295c2460                 movaps xmmword ptr [rsp + 0x60], xmm11
142296d6d: 440f29542470                 movaps xmmword ptr [rsp + 0x70], xmm10
142296d73: 440f298c2480000000           movaps xmmword ptr [rsp + 0x80], xmm9
142296d7c: 440f29842490000000           movaps xmmword ptr [rsp + 0x90], xmm8
142296d85: 0f29bc24a0000000             movaps xmmword ptr [rsp + 0xa0], xmm7
142296d8d: 0f29b424b0000000             movaps xmmword ptr [rsp + 0xb0], xmm6
142296d95: 80bd9b06000000               cmp byte ptr [rbp + 0x69b], 0
142296d9c: 741e                         je 0x142296dbc
142296d9e: 488b9538060000               mov rdx, qword ptr [rbp + 0x638]
142296da5: 4885d2                       test rdx, rdx
142296da8: 7412                         je 0x142296dbc
142296daa: 488b8d40060000               mov rcx, qword ptr [rbp + 0x640]
142296db1: 41b801000000                 mov r8d, 1
142296db7: e864ce37fe                   call 0x140613c20
142296dbc: 0f28b424b0000000             movaps xmm6, xmmword ptr [rsp + 0xb0]
142296dc4: 0f28bc24a0000000             movaps xmm7, xmmword ptr [rsp + 0xa0]
142296dcc: 440f28842490000000           movaps xmm8, xmmword ptr [rsp + 0x90]
142296dd5: 440f288c2480000000           movaps xmm9, xmmword ptr [rsp + 0x80]
142296dde: 440f28542470                 movaps xmm10, xmmword ptr [rsp + 0x70]
142296de4: 440f285c2460                 movaps xmm11, xmmword ptr [rsp + 0x60]
142296dea: 440f28642450                 movaps xmm12, xmmword ptr [rsp + 0x50]
142296df0: 440f286c2440                 movaps xmm13, xmmword ptr [rsp + 0x40]
142296df6: 440f28742430                 movaps xmm14, xmmword ptr [rsp + 0x30]
142296dfc: 440f287c2420                 movaps xmm15, xmmword ptr [rsp + 0x20]
142296e02: 4881c4c8000000               add rsp, 0xc8
142296e09: 5b                           pop rbx
142296e0a: 5f                           pop rdi
142296e0b: 5e                           pop rsi
142296e0c: 415c                         pop r12
142296e0e: 415d                         pop r13
142296e10: 415e                         pop r14
142296e12: 415f                         pop r15
142296e14: 5d                           pop rbp
142296e15: c3                           ret 
142296e16: 662e0f1f840000000000         nop word ptr cs:[rax + rax]
142296e20: 4889542410                   mov qword ptr [rsp + 0x10], rdx
142296e25: 55                           push rbp
142296e26: 4157                         push r15
142296e28: 4156                         push r14
142296e2a: 4155                         push r13
142296e2c: 4154                         push r12
142296e2e: 56                           push rsi
142296e2f: 57                           push rdi
142296e30: 53                           push rbx
142296e31: 4881ecc8000000               sub rsp, 0xc8
142296e38: 488daa80000000               lea rbp, [rdx + 0x80]
142296e3f: 440f297c2420                 movaps xmmword ptr [rsp + 0x20], xmm15
142296e45: 440f29742430                 movaps xmmword ptr [rsp + 0x30], xmm14
142296e4b: 440f296c2440                 movaps xmmword ptr [rsp + 0x40], xmm13
142296e51: 440f29642450                 movaps xmmword ptr [rsp + 0x50], xmm12
142296e57: 440f295c2460                 movaps xmmword ptr [rsp + 0x60], xmm11
142296e5d: 440f29542470                 movaps xmmword ptr [rsp + 0x70], xmm10
142296e63: 440f298c2480000000           movaps xmmword ptr [rsp + 0x80], xmm9
142296e6c: 440f29842490000000           movaps xmmword ptr [rsp + 0x90], xmm8
142296e75: 0f29bc24a0000000             movaps xmmword ptr [rsp + 0xa0], xmm7
142296e7d: 0f29b424b0000000             movaps xmmword ptr [rsp + 0xb0], xmm6
142296e85: 80bd9c06000000               cmp byte ptr [rbp + 0x69c], 0
142296e8c: 741e                         je 0x142296eac
142296e8e: 488b9538060000               mov rdx, qword ptr [rbp + 0x638]
142296e95: 4885d2                       test rdx, rdx
142296e98: 7412                         je 0x142296eac
142296e9a: 488b8d40060000               mov rcx, qword ptr [rbp + 0x640]
142296ea1: 41b801000000                 mov r8d, 1
142296ea7: e874cd37fe                   call 0x140613c20
142296eac: 0f28b424b0000000             movaps xmm6, xmmword ptr [rsp + 0xb0]
142296eb4: 0f28bc24a0000000             movaps xmm7, xmmword ptr [rsp + 0xa0]
142296ebc: 440f28842490000000           movaps xmm8, xmmword ptr [rsp + 0x90]
142296ec5: 440f288c2480000000           movaps xmm9, xmmword ptr [rsp + 0x80]
142296ece: 440f28542470                 movaps xmm10, xmmword ptr [rsp + 0x70]
142296ed4: 440f285c2460                 movaps xmm11, xmmword ptr [rsp + 0x60]
142296eda: 440f28642450                 movaps xmm12, xmmword ptr [rsp + 0x50]
142296ee0: 440f286c2440                 movaps xmm13, xmmword ptr [rsp + 0x40]
142296ee6: 440f28742430                 movaps xmm14, xmmword ptr [rsp + 0x30]
142296eec: 440f287c2420                 movaps xmm15, xmmword ptr [rsp + 0x20]
142296ef2: 4881c4c8000000               add rsp, 0xc8
142296ef9: 5b                           pop rbx
142296efa: 5f                           pop rdi
142296efb: 5e                           pop rsi
142296efc: 415c                         pop r12
142296efe: 415d                         pop r13
142296f00: 415e                         pop r14
142296f02: 415f                         pop r15
142296f04: 5d                           pop rbp
142296f05: c3                           ret 
142296f06: 662e0f1f840000000000         nop word ptr cs:[rax + rax]
142296f10: 4889542410                   mov qword ptr [rsp + 0x10], rdx
142296f15: 55                           push rbp
142296f16: 4157                         push r15
142296f18: 4156                         push r14
142296f1a: 4155                         push r13
142296f1c: 4154                         push r12
142296f1e: 56                           push rsi
142296f1f: 57                           push rdi
142296f20: 53                           push rbx
142296f21: 4881ecc8000000               sub rsp, 0xc8
142296f28: 488daa80000000               lea rbp, [rdx + 0x80]
142296f2f: 440f297c2420                 movaps xmmword ptr [rsp + 0x20], xmm15
142296f35: 440f29742430                 movaps xmmword ptr [rsp + 0x30], xmm14
142296f3b: 440f296c2440                 movaps xmmword ptr [rsp + 0x40], xmm13
142296f41: 440f29642450                 movaps xmmword ptr [rsp + 0x50], xmm12
142296f47: 440f295c2460                 movaps xmmword ptr [rsp + 0x60], xmm11
142296f4d: 440f29542470                 movaps xmmword ptr [rsp + 0x70], xmm10
142296f53: 440f298c2480000000           movaps xmmword ptr [rsp + 0x80], xmm9
142296f5c: 440f29842490000000           movaps xmmword ptr [rsp + 0x90], xmm8
142296f65: 0f29bc24a0000000             movaps xmmword ptr [rsp + 0xa0], xmm7
142296f6d: 0f29b424b0000000             movaps xmmword ptr [rsp + 0xb0], xmm6
142296f75: 80bd9d06000000               cmp byte ptr [rbp + 0x69d], 0
142296f7c: 741e                         je 0x142296f9c
142296f7e: 488b9538060000               mov rdx, qword ptr [rbp + 0x638]
142296f85: 4885d2                       test rdx, rdx
142296f88: 7412                         je 0x142296f9c
142296f8a: 488b8d40060000               mov rcx, qword ptr [rbp + 0x640]
142296f91: 41b801000000                 mov r8d, 1
142296f97: e884cc37fe                   call 0x140613c20
142296f9c: 0f28b424b0000000             movaps xmm6, xmmword ptr [rsp + 0xb0]
142296fa4: 0f28bc24a0000000             movaps xmm7, xmmword ptr [rsp + 0xa0]
142296fac: 440f28842490000000           movaps xmm8, xmmword ptr [rsp + 0x90]
142296fb5: 440f288c2480000000           movaps xmm9, xmmword ptr [rsp + 0x80]
142296fbe: 440f28542470                 movaps xmm10, xmmword ptr [rsp + 0x70]
142296fc4: 440f285c2460                 movaps xmm11, xmmword ptr [rsp + 0x60]
142296fca: 440f28642450                 movaps xmm12, xmmword ptr [rsp + 0x50]
142296fd0: 440f286c2440                 movaps xmm13, xmmword ptr [rsp + 0x40]
142296fd6: 440f28742430                 movaps xmm14, xmmword ptr [rsp + 0x30]
142296fdc: 440f287c2420                 movaps xmm15, xmmword ptr [rsp + 0x20]
142296fe2: 4881c4c8000000               add rsp, 0xc8
142296fe9: 5b                           pop rbx
142296fea: 5f                           pop rdi
142296feb: 5e                           pop rsi
142296fec: 415c                         pop r12
142296fee: 415d                         pop r13
142296ff0: 415e                         pop r14
142296ff2: 415f                         pop r15
142296ff4: 5d                           pop rbp
142296ff5: c3                           ret 
142296ff6: 662e0f1f840000000000         nop word ptr cs:[rax + rax]
142297000: 4889542410                   mov qword ptr [rsp + 0x10], rdx
142297005: 55                           push rbp
142297006: 4157                         push r15
142297008: 4156                         push r14
14229700a: 4155                         push r13
14229700c: 4154                         push r12
14229700e: 56                           push rsi
14229700f: 57                           push rdi
142297010: 53                           push rbx
142297011: 4881ecc8000000               sub rsp, 0xc8
142297018: 488daa80000000               lea rbp, [rdx + 0x80]
14229701f: 440f297c2420                 movaps xmmword ptr [rsp + 0x20], xmm15
142297025: 440f29742430                 movaps xmmword ptr [rsp + 0x30], xmm14
14229702b: 440f296c2440                 movaps xmmword ptr [rsp + 0x40], xmm13
142297031: 440f29642450                 movaps xmmword ptr [rsp + 0x50], xmm12
142297037: 440f295c2460                 movaps xmmword ptr [rsp + 0x60], xmm11
14229703d: 440f29542470                 movaps xmmword ptr [rsp + 0x70], xmm10
142297043: 440f298c2480000000           movaps xmmword ptr [rsp + 0x80], xmm9
14229704c: 440f29842490000000           movaps xmmword ptr [rsp + 0x90], xmm8
142297055: 0f29bc24a0000000             movaps xmmword ptr [rsp + 0xa0], xmm7
14229705d: 0f29b424b0000000             movaps xmmword ptr [rsp + 0xb0], xmm6
142297065: 80bd9e06000000               cmp byte ptr [rbp + 0x69e], 0
14229706c: 741e                         je 0x14229708c
14229706e: 488b9538060000               mov rdx, qword ptr [rbp + 0x638]
142297075: 4885d2                       test rdx, rdx
142297078: 7412                         je 0x14229708c
14229707a: 488b8d40060000               mov rcx, qword ptr [rbp + 0x640]
142297081: 41b801000000                 mov r8d, 1
142297087: e894cb37fe                   call 0x140613c20
14229708c: 0f28b424b0000000             movaps xmm6, xmmword ptr [rsp + 0xb0]
142297094: 0f28bc24a0000000             movaps xmm7, xmmword ptr [rsp + 0xa0]
14229709c: 440f28842490000000           movaps xmm8, xmmword ptr [rsp + 0x90]
1422970a5: 440f288c2480000000           movaps xmm9, xmmword ptr [rsp + 0x80]
1422970ae: 440f28542470                 movaps xmm10, xmmword ptr [rsp + 0x70]
1422970b4: 440f285c2460                 movaps xmm11, xmmword ptr [rsp + 0x60]
1422970ba: 440f28642450                 movaps xmm12, xmmword ptr [rsp + 0x50]
1422970c0: 440f286c2440                 movaps xmm13, xmmword ptr [rsp + 0x40]
1422970c6: 440f28742430                 movaps xmm14, xmmword ptr [rsp + 0x30]
1422970cc: 440f287c2420                 movaps xmm15, xmmword ptr [rsp + 0x20]
1422970d2: 4881c4c8000000               add rsp, 0xc8
1422970d9: 5b                           pop rbx
1422970da: 5f                           pop rdi
1422970db: 5e                           pop rsi
1422970dc: 415c                         pop r12
1422970de: 415d                         pop r13
1422970e0: 415e                         pop r14
1422970e2: 415f                         pop r15
1422970e4: 5d                           pop rbp
1422970e5: c3                           ret 
1422970e6: 662e0f1f840000000000         nop word ptr cs:[rax + rax]
1422970f0: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1422970f5: 55                           push rbp
1422970f6: 4157                         push r15
1422970f8: 4156                         push r14
1422970fa: 4155                         push r13
1422970fc: 4154                         push r12
1422970fe: 56                           push rsi
1422970ff: 57                           push rdi
142297100: 53                           push rbx
142297101: 4881ecc8000000               sub rsp, 0xc8
142297108: 488daa80000000               lea rbp, [rdx + 0x80]
14229710f: 440f297c2420                 movaps xmmword ptr [rsp + 0x20], xmm15
142297115: 440f29742430                 movaps xmmword ptr [rsp + 0x30], xmm14
14229711b: 440f296c2440                 movaps xmmword ptr [rsp + 0x40], xmm13
142297121: 440f29642450                 movaps xmmword ptr [rsp + 0x50], xmm12
142297127: 440f295c2460                 movaps xmmword ptr [rsp + 0x60], xmm11
14229712d: 440f29542470                 movaps xmmword ptr [rsp + 0x70], xmm10
142297133: 440f298c2480000000           movaps xmmword ptr [rsp + 0x80], xmm9
14229713c: 440f29842490000000           movaps xmmword ptr [rsp + 0x90], xmm8
142297145: 0f29bc24a0000000             movaps xmmword ptr [rsp + 0xa0], xmm7
14229714d: 0f29b424b0000000             movaps xmmword ptr [rsp + 0xb0], xmm6
142297155: 80bd9f06000000               cmp byte ptr [rbp + 0x69f], 0
14229715c: 741e                         je 0x14229717c
14229715e: 488b9538060000               mov rdx, qword ptr [rbp + 0x638]
142297165: 4885d2                       test rdx, rdx
142297168: 7412                         je 0x14229717c
14229716a: 488b8d40060000               mov rcx, qword ptr [rbp + 0x640]
142297171: 41b801000000                 mov r8d, 1
142297177: e8a4ca37fe                   call 0x140613c20
14229717c: 0f28b424b0000000             movaps xmm6, xmmword ptr [rsp + 0xb0]
142297184: 0f28bc24a0000000             movaps xmm7, xmmword ptr [rsp + 0xa0]
14229718c: 440f28842490000000           movaps xmm8, xmmword ptr [rsp + 0x90]
142297195: 440f288c2480000000           movaps xmm9, xmmword ptr [rsp + 0x80]
14229719e: 440f28542470                 movaps xmm10, xmmword ptr [rsp + 0x70]
1422971a4: 440f285c2460                 movaps xmm11, xmmword ptr [rsp + 0x60]
1422971aa: 440f28642450                 movaps xmm12, xmmword ptr [rsp + 0x50]
1422971b0: 440f286c2440                 movaps xmm13, xmmword ptr [rsp + 0x40]
1422971b6: 440f28742430                 movaps xmm14, xmmword ptr [rsp + 0x30]
1422971bc: 440f287c2420                 movaps xmm15, xmmword ptr [rsp + 0x20]
1422971c2: 4881c4c8000000               add rsp, 0xc8
1422971c9: 5b                           pop rbx
1422971ca: 5f                           pop rdi
1422971cb: 5e                           pop rsi
1422971cc: 415c                         pop r12
1422971ce: 415d                         pop r13
1422971d0: 415e                         pop r14
1422971d2: 415f                         pop r15
1422971d4: 5d                           pop rbp
1422971d5: c3                           ret 
1422971d6: 662e0f1f840000000000         nop word ptr cs:[rax + rax]
1422971e0: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1422971e5: 55                           push rbp
1422971e6: 4157                         push r15
1422971e8: 4156                         push r14
1422971ea: 4155                         push r13
1422971ec: 4154                         push r12
1422971ee: 56                           push rsi
1422971ef: 57                           push rdi
1422971f0: 53                           push rbx
1422971f1: 4881ecc8000000               sub rsp, 0xc8
1422971f8: 488daa80000000               lea rbp, [rdx + 0x80]
1422971ff: 440f297c2420                 movaps xmmword ptr [rsp + 0x20], xmm15
142297205: 440f29742430                 movaps xmmword ptr [rsp + 0x30], xmm14
14229720b: 440f296c2440                 movaps xmmword ptr [rsp + 0x40], xmm13
142297211: 440f29642450                 movaps xmmword ptr [rsp + 0x50], xmm12
142297217: 440f295c2460                 movaps xmmword ptr [rsp + 0x60], xmm11
14229721d: 440f29542470                 movaps xmmword ptr [rsp + 0x70], xmm10
142297223: 440f298c2480000000           movaps xmmword ptr [rsp + 0x80], xmm9
14229722c: 440f29842490000000           movaps xmmword ptr [rsp + 0x90], xmm8
142297235: 0f29bc24a0000000             movaps xmmword ptr [rsp + 0xa0], xmm7
14229723d: 0f29b424b0000000             movaps xmmword ptr [rsp + 0xb0], xmm6
142297245: 80bda006000000               cmp byte ptr [rbp + 0x6a0], 0
14229724c: 741e                         je 0x14229726c
14229724e: 488b9538060000               mov rdx, qword ptr [rbp + 0x638]
142297255: 4885d2                       test rdx, rdx
142297258: 7412                         je 0x14229726c
14229725a: 488b8d40060000               mov rcx, qword ptr [rbp + 0x640]
142297261: 41b801000000                 mov r8d, 1
142297267: e8b4c937fe                   call 0x140613c20
14229726c: 0f28b424b0000000             movaps xmm6, xmmword ptr [rsp + 0xb0]
142297274: 0f28bc24a0000000             movaps xmm7, xmmword ptr [rsp + 0xa0]
14229727c: 440f28842490000000           movaps xmm8, xmmword ptr [rsp + 0x90]
142297285: 440f288c2480000000           movaps xmm9, xmmword ptr [rsp + 0x80]
14229728e: 440f28542470                 movaps xmm10, xmmword ptr [rsp + 0x70]
142297294: 440f285c2460                 movaps xmm11, xmmword ptr [rsp + 0x60]
14229729a: 440f28642450                 movaps xmm12, xmmword ptr [rsp + 0x50]
1422972a0: 440f286c2440                 movaps xmm13, xmmword ptr [rsp + 0x40]
1422972a6: 440f28742430                 movaps xmm14, xmmword ptr [rsp + 0x30]
1422972ac: 440f287c2420                 movaps xmm15, xmmword ptr [rsp + 0x20]
1422972b2: 4881c4c8000000               add rsp, 0xc8
1422972b9: 5b                           pop rbx
1422972ba: 5f                           pop rdi
1422972bb: 5e                           pop rsi
1422972bc: 415c                         pop r12
1422972be: 415d                         pop r13
1422972c0: 415e                         pop r14
1422972c2: 415f                         pop r15
1422972c4: 5d                           pop rbp
1422972c5: c3                           ret 
1422972c6: 662e0f1f840000000000         nop word ptr cs:[rax + rax]
1422972d0: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1422972d5: 55                           push rbp
1422972d6: 4157                         push r15
1422972d8: 4156                         push r14
1422972da: 4155                         push r13
1422972dc: 4154                         push r12
1422972de: 56                           push rsi
1422972df: 57                           push rdi
1422972e0: 53                           push rbx
1422972e1: 4881ecc8000000               sub rsp, 0xc8
1422972e8: 488daa80000000               lea rbp, [rdx + 0x80]
1422972ef: 440f297c2420                 movaps xmmword ptr [rsp + 0x20], xmm15
1422972f5: 440f29742430                 movaps xmmword ptr [rsp + 0x30], xmm14
1422972fb: 440f296c2440                 movaps xmmword ptr [rsp + 0x40], xmm13
142297301: 440f29642450                 movaps xmmword ptr [rsp + 0x50], xmm12
142297307: 440f295c2460                 movaps xmmword ptr [rsp + 0x60], xmm11
14229730d: 440f29542470                 movaps xmmword ptr [rsp + 0x70], xmm10
142297313: 440f298c2480000000           movaps xmmword ptr [rsp + 0x80], xmm9
14229731c: 440f29842490000000           movaps xmmword ptr [rsp + 0x90], xmm8
142297325: 0f29bc24a0000000             movaps xmmword ptr [rsp + 0xa0], xmm7
14229732d: 0f29b424b0000000             movaps xmmword ptr [rsp + 0xb0], xmm6
142297335: 80bda106000000               cmp byte ptr [rbp + 0x6a1], 0
14229733c: 741e                         je 0x14229735c
14229733e: 488b9538060000               mov rdx, qword ptr [rbp + 0x638]
142297345: 4885d2                       test rdx, rdx
142297348: 7412                         je 0x14229735c
14229734a: 488b8d40060000               mov rcx, qword ptr [rbp + 0x640]
142297351: 41b801000000                 mov r8d, 1
142297357: e8c4c837fe                   call 0x140613c20
14229735c: 0f28b424b0000000             movaps xmm6, xmmword ptr [rsp + 0xb0]
142297364: 0f28bc24a0000000             movaps xmm7, xmmword ptr [rsp + 0xa0]
14229736c: 440f28842490000000           movaps xmm8, xmmword ptr [rsp + 0x90]
142297375: 440f288c2480000000           movaps xmm9, xmmword ptr [rsp + 0x80]
14229737e: 440f28542470                 movaps xmm10, xmmword ptr [rsp + 0x70]
142297384: 440f285c2460                 movaps xmm11, xmmword ptr [rsp + 0x60]
14229738a: 440f28642450                 movaps xmm12, xmmword ptr [rsp + 0x50]
142297390: 440f286c2440                 movaps xmm13, xmmword ptr [rsp + 0x40]
142297396: 440f28742430                 movaps xmm14, xmmword ptr [rsp + 0x30]
14229739c: 440f287c2420                 movaps xmm15, xmmword ptr [rsp + 0x20]
1422973a2: 4881c4c8000000               add rsp, 0xc8
1422973a9: 5b                           pop rbx
1422973aa: 5f                           pop rdi
1422973ab: 5e                           pop rsi
1422973ac: 415c                         pop r12
1422973ae: 415d                         pop r13
1422973b0: 415e                         pop r14
1422973b2: 415f                         pop r15
1422973b4: 5d                           pop rbp
1422973b5: c3                           ret 
1422973b6: 662e0f1f840000000000         nop word ptr cs:[rax + rax]
1422973c0: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1422973c5: 55                           push rbp
1422973c6: 4157                         push r15
1422973c8: 4156                         push r14
1422973ca: 4155                         push r13
1422973cc: 4154                         push r12
1422973ce: 56                           push rsi
1422973cf: 57                           push rdi
1422973d0: 53                           push rbx
1422973d1: 4881ecc8000000               sub rsp, 0xc8
1422973d8: 488daa80000000               lea rbp, [rdx + 0x80]
1422973df: 440f297c2420                 movaps xmmword ptr [rsp + 0x20], xmm15
1422973e5: 440f29742430                 movaps xmmword ptr [rsp + 0x30], xmm14
1422973eb: 440f296c2440                 movaps xmmword ptr [rsp + 0x40], xmm13
1422973f1: 440f29642450                 movaps xmmword ptr [rsp + 0x50], xmm12
1422973f7: 440f295c2460                 movaps xmmword ptr [rsp + 0x60], xmm11
1422973fd: 440f29542470                 movaps xmmword ptr [rsp + 0x70], xmm10
142297403: 440f298c2480000000           movaps xmmword ptr [rsp + 0x80], xmm9
14229740c: 440f29842490000000           movaps xmmword ptr [rsp + 0x90], xmm8
142297415: 0f29bc24a0000000             movaps xmmword ptr [rsp + 0xa0], xmm7
14229741d: 0f29b424b0000000             movaps xmmword ptr [rsp + 0xb0], xmm6
142297425: 80bda206000000               cmp byte ptr [rbp + 0x6a2], 0
14229742c: 741e                         je 0x14229744c
14229742e: 488b9538060000               mov rdx, qword ptr [rbp + 0x638]
142297435: 4885d2                       test rdx, rdx
142297438: 7412                         je 0x14229744c
14229743a: 488b8d40060000               mov rcx, qword ptr [rbp + 0x640]
142297441: 41b801000000                 mov r8d, 1
142297447: e8d4c737fe                   call 0x140613c20
14229744c: 0f28b424b0000000             movaps xmm6, xmmword ptr [rsp + 0xb0]
142297454: 0f28bc24a0000000             movaps xmm7, xmmword ptr [rsp + 0xa0]
14229745c: 440f28842490000000           movaps xmm8, xmmword ptr [rsp + 0x90]
142297465: 440f288c2480000000           movaps xmm9, xmmword ptr [rsp + 0x80]
14229746e: 440f28542470                 movaps xmm10, xmmword ptr [rsp + 0x70]
142297474: 440f285c2460                 movaps xmm11, xmmword ptr [rsp + 0x60]
14229747a: 440f28642450                 movaps xmm12, xmmword ptr [rsp + 0x50]
142297480: 440f286c2440                 movaps xmm13, xmmword ptr [rsp + 0x40]
142297486: 440f28742430                 movaps xmm14, xmmword ptr [rsp + 0x30]
14229748c: 440f287c2420                 movaps xmm15, xmmword ptr [rsp + 0x20]
142297492: 4881c4c8000000               add rsp, 0xc8
142297499: 5b                           pop rbx
14229749a: 5f                           pop rdi
14229749b: 5e                           pop rsi
14229749c: 415c                         pop r12
14229749e: 415d                         pop r13
1422974a0: 415e                         pop r14
1422974a2: 415f                         pop r15
1422974a4: 5d                           pop rbp
1422974a5: c3                           ret 
1422974a6: 662e0f1f840000000000         nop word ptr cs:[rax + rax]
1422974b0: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1422974b5: 55                           push rbp
1422974b6: 4157                         push r15
1422974b8: 4156                         push r14
1422974ba: 4155                         push r13
1422974bc: 4154                         push r12
1422974be: 56                           push rsi
1422974bf: 57                           push rdi
1422974c0: 53                           push rbx
1422974c1: 4881ecc8000000               sub rsp, 0xc8
1422974c8: 488daa80000000               lea rbp, [rdx + 0x80]
1422974cf: 440f297c2420                 movaps xmmword ptr [rsp + 0x20], xmm15
1422974d5: 440f29742430                 movaps xmmword ptr [rsp + 0x30], xmm14
1422974db: 440f296c2440                 movaps xmmword ptr [rsp + 0x40], xmm13
1422974e1: 440f29642450                 movaps xmmword ptr [rsp + 0x50], xmm12
1422974e7: 440f295c2460                 movaps xmmword ptr [rsp + 0x60], xmm11
1422974ed: 440f29542470                 movaps xmmword ptr [rsp + 0x70], xmm10
1422974f3: 440f298c2480000000           movaps xmmword ptr [rsp + 0x80], xmm9
1422974fc: 440f29842490000000           movaps xmmword ptr [rsp + 0x90], xmm8
142297505: 0f29bc24a0000000             movaps xmmword ptr [rsp + 0xa0], xmm7
14229750d: 0f29b424b0000000             movaps xmmword ptr [rsp + 0xb0], xmm6
142297515: 80bda306000000               cmp byte ptr [rbp + 0x6a3], 0
14229751c: 741e                         je 0x14229753c
14229751e: 488b9538060000               mov rdx, qword ptr [rbp + 0x638]
142297525: 4885d2                       test rdx, rdx
142297528: 7412                         je 0x14229753c
14229752a: 488b8d40060000               mov rcx, qword ptr [rbp + 0x640]
142297531: 41b801000000                 mov r8d, 1
142297537: e8e4c637fe                   call 0x140613c20
14229753c: 0f28b424b0000000             movaps xmm6, xmmword ptr [rsp + 0xb0]
142297544: 0f28bc24a0000000             movaps xmm7, xmmword ptr [rsp + 0xa0]
14229754c: 440f28842490000000           movaps xmm8, xmmword ptr [rsp + 0x90]
142297555: 440f288c2480000000           movaps xmm9, xmmword ptr [rsp + 0x80]
14229755e: 440f28542470                 movaps xmm10, xmmword ptr [rsp + 0x70]
142297564: 440f285c2460                 movaps xmm11, xmmword ptr [rsp + 0x60]
14229756a: 440f28642450                 movaps xmm12, xmmword ptr [rsp + 0x50]
142297570: 440f286c2440                 movaps xmm13, xmmword ptr [rsp + 0x40]
142297576: 440f28742430                 movaps xmm14, xmmword ptr [rsp + 0x30]
14229757c: 440f287c2420                 movaps xmm15, xmmword ptr [rsp + 0x20]
142297582: 4881c4c8000000               add rsp, 0xc8
142297589: 5b                           pop rbx
14229758a: 5f                           pop rdi
14229758b: 5e                           pop rsi
14229758c: 415c                         pop r12
14229758e: 415d                         pop r13
142297590: 415e                         pop r14
142297592: 415f                         pop r15
142297594: 5d                           pop rbp
142297595: c3                           ret 
142297596: 662e0f1f840000000000         nop word ptr cs:[rax + rax]
1422975a0: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1422975a5: 55                           push rbp
1422975a6: 4157                         push r15
1422975a8: 4156                         push r14
1422975aa: 4155                         push r13
1422975ac: 4154                         push r12
1422975ae: 56                           push rsi
1422975af: 57                           push rdi
1422975b0: 53                           push rbx
1422975b1: 4881ecc8000000               sub rsp, 0xc8
1422975b8: 488daa80000000               lea rbp, [rdx + 0x80]
1422975bf: 440f297c2420                 movaps xmmword ptr [rsp + 0x20], xmm15
1422975c5: 440f29742430                 movaps xmmword ptr [rsp + 0x30], xmm14
1422975cb: 440f296c2440                 movaps xmmword ptr [rsp + 0x40], xmm13
1422975d1: 440f29642450                 movaps xmmword ptr [rsp + 0x50], xmm12
1422975d7: 440f295c2460                 movaps xmmword ptr [rsp + 0x60], xmm11
1422975dd: 440f29542470                 movaps xmmword ptr [rsp + 0x70], xmm10
1422975e3: 440f298c2480000000           movaps xmmword ptr [rsp + 0x80], xmm9
1422975ec: 440f29842490000000           movaps xmmword ptr [rsp + 0x90], xmm8
1422975f5: 0f29bc24a0000000             movaps xmmword ptr [rsp + 0xa0], xmm7
1422975fd: 0f29b424b0000000             movaps xmmword ptr [rsp + 0xb0], xmm6
142297605: 80bda406000000               cmp byte ptr [rbp + 0x6a4], 0
14229760c: 741e                         je 0x14229762c
14229760e: 488b9538060000               mov rdx, qword ptr [rbp + 0x638]
142297615: 4885d2                       test rdx, rdx
142297618: 7412                         je 0x14229762c
14229761a: 488b8d40060000               mov rcx, qword ptr [rbp + 0x640]
142297621: 41b801000000                 mov r8d, 1
142297627: e8f4c537fe                   call 0x140613c20
14229762c: 0f28b424b0000000             movaps xmm6, xmmword ptr [rsp + 0xb0]
142297634: 0f28bc24a0000000             movaps xmm7, xmmword ptr [rsp + 0xa0]
14229763c: 440f28842490000000           movaps xmm8, xmmword ptr [rsp + 0x90]
142297645: 440f288c2480000000           movaps xmm9, xmmword ptr [rsp + 0x80]
14229764e: 440f28542470                 movaps xmm10, xmmword ptr [rsp + 0x70]
142297654: 440f285c2460                 movaps xmm11, xmmword ptr [rsp + 0x60]
14229765a: 440f28642450                 movaps xmm12, xmmword ptr [rsp + 0x50]
142297660: 440f286c2440                 movaps xmm13, xmmword ptr [rsp + 0x40]
142297666: 440f28742430                 movaps xmm14, xmmword ptr [rsp + 0x30]
14229766c: 440f287c2420                 movaps xmm15, xmmword ptr [rsp + 0x20]
142297672: 4881c4c8000000               add rsp, 0xc8
142297679: 5b                           pop rbx
14229767a: 5f                           pop rdi
14229767b: 5e                           pop rsi
14229767c: 415c                         pop r12
14229767e: 415d                         pop r13
142297680: 415e                         pop r14
142297682: 415f                         pop r15
142297684: 5d                           pop rbp
142297685: c3                           ret 
142297686: 662e0f1f840000000000         nop word ptr cs:[rax + rax]
142297690: 4889542410                   mov qword ptr [rsp + 0x10], rdx
142297695: 55                           push rbp
142297696: 4157                         push r15
142297698: 4156                         push r14
14229769a: 4155                         push r13
14229769c: 4154                         push r12
14229769e: 56                           push rsi
14229769f: 57                           push rdi
1422976a0: 53                           push rbx
1422976a1: 4881ecc8000000               sub rsp, 0xc8
1422976a8: 488daa80000000               lea rbp, [rdx + 0x80]
1422976af: 440f297c2420                 movaps xmmword ptr [rsp + 0x20], xmm15
1422976b5: 440f29742430                 movaps xmmword ptr [rsp + 0x30], xmm14
1422976bb: 440f296c2440                 movaps xmmword ptr [rsp + 0x40], xmm13
1422976c1: 440f29642450                 movaps xmmword ptr [rsp + 0x50], xmm12
1422976c7: 440f295c2460                 movaps xmmword ptr [rsp + 0x60], xmm11
1422976cd: 440f29542470                 movaps xmmword ptr [rsp + 0x70], xmm10
1422976d3: 440f298c2480000000           movaps xmmword ptr [rsp + 0x80], xmm9
1422976dc: 440f29842490000000           movaps xmmword ptr [rsp + 0x90], xmm8
1422976e5: 0f29bc24a0000000             movaps xmmword ptr [rsp + 0xa0], xmm7
1422976ed: 0f29b424b0000000             movaps xmmword ptr [rsp + 0xb0], xmm6
1422976f5: 80bddd06000000               cmp byte ptr [rbp + 0x6dd], 0
1422976fc: 741e                         je 0x14229771c
1422976fe: 488b9538060000               mov rdx, qword ptr [rbp + 0x638]
142297705: 4885d2                       test rdx, rdx
142297708: 7412                         je 0x14229771c
14229770a: 488b8d40060000               mov rcx, qword ptr [rbp + 0x640]
142297711: 41b801000000                 mov r8d, 1
142297717: e804c537fe                   call 0x140613c20
14229771c: 0f28b424b0000000             movaps xmm6, xmmword ptr [rsp + 0xb0]
142297724: 0f28bc24a0000000             movaps xmm7, xmmword ptr [rsp + 0xa0]
14229772c: 440f28842490000000           movaps xmm8, xmmword ptr [rsp + 0x90]
142297735: 440f288c2480000000           movaps xmm9, xmmword ptr [rsp + 0x80]
14229773e: 440f28542470                 movaps xmm10, xmmword ptr [rsp + 0x70]
142297744: 440f285c2460                 movaps xmm11, xmmword ptr [rsp + 0x60]
14229774a: 440f28642450                 movaps xmm12, xmmword ptr [rsp + 0x50]
142297750: 440f286c2440                 movaps xmm13, xmmword ptr [rsp + 0x40]
142297756: 440f28742430                 movaps xmm14, xmmword ptr [rsp + 0x30]
14229775c: 440f287c2420                 movaps xmm15, xmmword ptr [rsp + 0x20]
142297762: 4881c4c8000000               add rsp, 0xc8
142297769: 5b                           pop rbx
14229776a: 5f                           pop rdi
14229776b: 5e                           pop rsi
14229776c: 415c                         pop r12
14229776e: 415d                         pop r13
142297770: 415e                         pop r14
142297772: 415f                         pop r15
142297774: 5d                           pop rbp
142297775: c3                           ret 
142297776: 662e0f1f840000000000         nop word ptr cs:[rax + rax]
142297780: 4889542410                   mov qword ptr [rsp + 0x10], rdx
142297785: 55                           push rbp
142297786: 4157                         push r15
142297788: 4156                         push r14
14229778a: 4155                         push r13
14229778c: 4154                         push r12
14229778e: 56                           push rsi
14229778f: 57                           push rdi
142297790: 53                           push rbx
142297791: 4881ecc8000000               sub rsp, 0xc8
142297798: 488daa80000000               lea rbp, [rdx + 0x80]
14229779f: 440f297c2420                 movaps xmmword ptr [rsp + 0x20], xmm15
1422977a5: 440f29742430                 movaps xmmword ptr [rsp + 0x30], xmm14
1422977ab: 440f296c2440                 movaps xmmword ptr [rsp + 0x40], xmm13
1422977b1: 440f29642450                 movaps xmmword ptr [rsp + 0x50], xmm12
1422977b7: 440f295c2460                 movaps xmmword ptr [rsp + 0x60], xmm11
1422977bd: 440f29542470                 movaps xmmword ptr [rsp + 0x70], xmm10
1422977c3: 440f298c2480000000           movaps xmmword ptr [rsp + 0x80], xmm9
1422977cc: 440f29842490000000           movaps xmmword ptr [rsp + 0x90], xmm8
1422977d5: 0f29bc24a0000000             movaps xmmword ptr [rsp + 0xa0], xmm7
1422977dd: 0f29b424b0000000             movaps xmmword ptr [rsp + 0xb0], xmm6
1422977e5: 80bda506000000               cmp byte ptr [rbp + 0x6a5], 0
1422977ec: 746d                         je 0x14229785b
1422977ee: 488d8d30050000               lea rcx, [rbp + 0x530]
1422977f5: e876dc5ffe                   call 0x140895470
1422977fa: c685eb06000001               mov byte ptr [rbp + 0x6eb], 1
142297801: 0f28b424b0000000             movaps xmm6, xmmword ptr [rsp + 0xb0]
142297809: 0f28bc24a0000000             movaps xmm7, xmmword ptr [rsp + 0xa0]
142297811: 440f28842490000000           movaps xmm8, xmmword ptr [rsp + 0x90]
14229781a: 440f288c2480000000           movaps xmm9, xmmword ptr [rsp + 0x80]
142297823: 440f28542470                 movaps xmm10, xmmword ptr [rsp + 0x70]
142297829: 440f285c2460                 movaps xmm11, xmmword ptr [rsp + 0x60]
14229782f: 440f28642450                 movaps xmm12, xmmword ptr [rsp + 0x50]
142297835: 440f286c2440                 movaps xmm13, xmmword ptr [rsp + 0x40]
14229783b: 440f28742430                 movaps xmm14, xmmword ptr [rsp + 0x30]
142297841: 440f287c2420                 movaps xmm15, xmmword ptr [rsp + 0x20]
142297847: 4881c4c8000000               add rsp, 0xc8
14229784e: 5b                           pop rbx
14229784f: 5f                           pop rdi
142297850: 5e                           pop rsi
142297851: 415c                         pop r12
142297853: 415d                         pop r13
142297855: 415e                         pop r14
142297857: 415f                         pop r15
142297859: 5d                           pop rbp
14229785a: c3                           ret 
14229785b: c685eb06000001               mov byte ptr [rbp + 0x6eb], 1
142297862: 0f28b424b0000000             movaps xmm6, xmmword ptr [rsp + 0xb0]
14229786a: 0f28bc24a0000000             movaps xmm7, xmmword ptr [rsp + 0xa0]
142297872: 440f28842490000000           movaps xmm8, xmmword ptr [rsp + 0x90]
14229787b: 440f288c2480000000           movaps xmm9, xmmword ptr [rsp + 0x80]
142297884: 440f28542470                 movaps xmm10, xmmword ptr [rsp + 0x70]
14229788a: 440f285c2460                 movaps xmm11, xmmword ptr [rsp + 0x60]
142297890: 440f28642450                 movaps xmm12, xmmword ptr [rsp + 0x50]
142297896: 440f286c2440                 movaps xmm13, xmmword ptr [rsp + 0x40]
14229789c: 440f28742430                 movaps xmm14, xmmword ptr [rsp + 0x30]
1422978a2: 440f287c2420                 movaps xmm15, xmmword ptr [rsp + 0x20]
1422978a8: 4881c4c8000000               add rsp, 0xc8
1422978af: 5b                           pop rbx
1422978b0: 5f                           pop rdi
1422978b1: 5e                           pop rsi
1422978b2: 415c                         pop r12
1422978b4: 415d                         pop r13
1422978b6: 415e                         pop r14
1422978b8: 415f                         pop r15
1422978ba: 5d                           pop rbp
1422978bb: c3                           ret 
1422978bc: 0f1f4000                     nop dword ptr [rax]
1422978c0: 4889542410                   mov qword ptr [rsp + 0x10], rdx
1422978c5: 55                           push rbp
1422978c6: 4157                         push r15
1422978c8: 4156                         push r14
1422978ca: 4155                         push r13
1422978cc: 4154                         push r12
1422978ce: 56                           push rsi
1422978cf: 57                           push rdi
1422978d0: 53                           push rbx
1422978d1: 4881ecc8000000               sub rsp, 0xc8
1422978d8: 488daa80000000               lea rbp, [rdx + 0x80]
1422978df: 440f297c2420                 movaps xmmword ptr [rsp + 0x20], xmm15
1422978e5: 440f29742430                 movaps xmmword ptr [rsp + 0x30], xmm14
1422978eb: 440f296c2440                 movaps xmmword ptr [rsp + 0x40], xmm13
1422978f1: 440f29642450                 movaps xmmword ptr [rsp + 0x50], xmm12
1422978f7: 440f295c2460                 movaps xmmword ptr [rsp + 0x60], xmm11
1422978fd: 440f29542470                 movaps xmmword ptr [rsp + 0x70], xmm10
142297903: 440f298c2480000000           movaps xmmword ptr [rsp + 0x80], xmm9
14229790c: 440f29842490000000           movaps xmmword ptr [rsp + 0x90], xmm8
142297915: 0f29bc24a0000000             movaps xmmword ptr [rsp + 0xa0], xmm7
14229791d: 0f29b424b0000000             movaps xmmword ptr [rsp + 0xb0], xmm6
142297925: 80bdeb06000000               cmp byte ptr [rbp + 0x6eb], 0
14229792c: 741e                         je 0x14229794c
14229792e: 488b9538060000               mov rdx, qword ptr [rbp + 0x638]
142297935: 4885d2                       test rdx, rdx
142297938: 7412                         je 0x14229794c
14229793a: 488b8d40060000               mov rcx, qword ptr [rbp + 0x640]
142297941: 41b801000000                 mov r8d, 1
142297947: e8d4c237fe                   call 0x140613c20
14229794c: 0f28b424b0000000             movaps xmm6, xmmword ptr [rsp + 0xb0]
142297954: 0f28bc24a0000000             movaps xmm7, xmmword ptr [rsp + 0xa0]
14229795c: 440f28842490000000           movaps xmm8, xmmword ptr [rsp + 0x90]
142297965: 440f288c2480000000           movaps xmm9, xmmword ptr [rsp + 0x80]
14229796e: 440f28542470                 movaps xmm10, xmmword ptr [rsp + 0x70]
142297974: 440f285c2460                 movaps xmm11, xmmword ptr [rsp + 0x60]
14229797a: 440f28642450                 movaps xmm12, xmmword ptr [rsp + 0x50]
142297980: 440f286c2440                 movaps xmm13, xmmword ptr [rsp + 0x40]
142297986: 440f28742430                 movaps xmm14, xmmword ptr [rsp + 0x30]
14229798c: 440f287c2420                 movaps xmm15, xmmword ptr [rsp + 0x20]
142297992: 4881c4c8000000               add rsp, 0xc8
142297999: 5b                           pop rbx
14229799a: 5f                           pop rdi
14229799b: 5e                           pop rsi
14229799c: 415c                         pop r12
14229799e: 415d                         pop r13
1422979a0: 415e                         pop r14
1422979a2: 415f                         pop r15
1422979a4: 5d                           pop rbp
1422979a5: c3                           ret 
