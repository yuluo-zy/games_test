00000001408f6180  55                       push       rbp
00000001408f6181  4157                     push       r15
00000001408f6183  4156                     push       r14
00000001408f6185  4155                     push       r13
00000001408f6187  4154                     push       r12
00000001408f6189  56                       push       rsi
00000001408f618a  57                       push       rdi
00000001408f618b  53                       push       rbx
00000001408f618c  4881ec18040000           sub        rsp, 0x418
00000001408f6193  488dac2480000000         lea        rbp, [rsp + 0x80]
00000001408f619b  48c78590030000feffffff   mov        qword ptr [rbp + 0x390], 0xfffffffffffffffe
00000001408f61a6  4c89cb                   mov        rbx, r9
00000001408f61a9  4d89c6                   mov        r14, r8
00000001408f61ac  4989d7                   mov        r15, rdx
00000001408f61af  4889ce                   mov        rsi, rcx
00000001408f61b2  4d8d6924                 lea        r13, [r9 + 0x24]
00000001408f61b6  410f104128               movups     xmm0, xmmword ptr [r9 + 0x28]
00000001408f61bb  410f104938               movups     xmm1, xmmword ptr [r9 + 0x38]
00000001408f61c0  0f298d60010000           movaps     xmmword ptr [rbp + 0x160], xmm1
00000001408f61c7  0f298550010000           movaps     xmmword ptr [rbp + 0x150], xmm0
00000001408f61ce  48b80200000002000000     movabs     rax, 0x200000002
00000001408f61d8  48898530030000           mov        qword ptr [rbp + 0x330], rax
00000001408f61df  c7853803000001000000     mov        dword ptr [rbp + 0x338], 1
00000001408f61e9  488d7dc0                 lea        rdi, [rbp - 0x40]
00000001408f61ed  488d9550010000           lea        rdx, [rbp + 0x150]
00000001408f61f4  4c8d8530030000           lea        r8, [rbp + 0x330]
00000001408f61fb  4889f9                   mov        rcx, rdi
00000001408f61fe  e8ddf32d00               call       0x140bd55e0
00000001408f6203  4889f9                   mov        rcx, rdi
00000001408f6206  e885f52d00               call       0x140bd5790
00000001408f620b  4889c7                   mov        rdi, rax
00000001408f620e  488d8d30030000           lea        rcx, [rbp + 0x330]
00000001408f6215  4889c2                   mov        rdx, rax
00000001408f6218  41b864000000             mov        r8d, 0x64
00000001408f621e  e87df02d00               call       0x140bd52a0
00000001408f6223  0f108530030000           movups     xmm0, xmmword ptr [rbp + 0x330]
00000001408f622a  0f108d40030000           movups     xmm1, xmmword ptr [rbp + 0x340]
00000001408f6231  0f294dd0                 movaps     xmmword ptr [rbp - 0x30], xmm1
00000001408f6235  0f2945c0                 movaps     xmmword ptr [rbp - 0x40], xmm0
00000001408f6239  488d8d60010000           lea        rcx, [rbp + 0x160]
00000001408f6240  488d55c0                 lea        rdx, [rbp - 0x40]
00000001408f6244  e8970a2e00               call       0x140bd6ce0
00000001408f6249  488d05c0281b02           lea        rax, [rip + 0x21b28c0]
00000001408f6250  48898550010000           mov        qword ptr [rbp + 0x150], rax
00000001408f6257  48c785580100000e000000   mov        qword ptr [rbp + 0x158], 0xe
00000001408f6262  488d9550010000           lea        rdx, [rbp + 0x150]
00000001408f6269  4c89f9                   mov        rcx, r15
00000001408f626c  e85f322f00               call       0x140be94d0
00000001408f6271  0f108530030000           movups     xmm0, xmmword ptr [rbp + 0x330]
00000001408f6278  0f108d40030000           movups     xmm1, xmmword ptr [rbp + 0x340]
00000001408f627f  0f118508030000           movups     xmmword ptr [rbp + 0x308], xmm0
00000001408f6286  0f118d18030000           movups     xmmword ptr [rbp + 0x318], xmm1
00000001408f628d  898504030000             mov        dword ptr [rbp + 0x304], eax
00000001408f6293  488d8d30030000           lea        rcx, [rbp + 0x330]
00000001408f629a  4889fa                   mov        rdx, rdi
00000001408f629d  41b826000000             mov        r8d, 0x26
00000001408f62a3  e8f8ef2d00               call       0x140bd52a0
00000001408f62a8  0f108530030000           movups     xmm0, xmmword ptr [rbp + 0x330]
00000001408f62af  0f108d40030000           movups     xmm1, xmmword ptr [rbp + 0x340]
00000001408f62b6  0f294dd0                 movaps     xmmword ptr [rbp - 0x30], xmm1
00000001408f62ba  0f2945c0                 movaps     xmmword ptr [rbp - 0x40], xmm0
00000001408f62be  488d8d60010000           lea        rcx, [rbp + 0x160]
00000001408f62c5  4c8d65c0                 lea        r12, [rbp - 0x40]
00000001408f62c9  4c89e2                   mov        rdx, r12
00000001408f62cc  e80f0a2e00               call       0x140bd6ce0
00000001408f62d1  488d0546281b02           lea        rax, [rip + 0x21b2846]
00000001408f62d8  48898550010000           mov        qword ptr [rbp + 0x150], rax
00000001408f62df  48c7855801000010000000   mov        qword ptr [rbp + 0x158], 0x10
00000001408f62ea  488d9550010000           lea        rdx, [rbp + 0x150]
00000001408f62f1  4c89f9                   mov        rcx, r15
00000001408f62f4  e8d7312f00               call       0x140be94d0
00000001408f62f9  0f108530030000           movups     xmm0, xmmword ptr [rbp + 0x330]
00000001408f6300  0f108d40030000           movups     xmm1, xmmword ptr [rbp + 0x340]
00000001408f6307  0f1185e4020000           movups     xmmword ptr [rbp + 0x2e4], xmm0
00000001408f630e  0f118df4020000           movups     xmmword ptr [rbp + 0x2f4], xmm1
00000001408f6315  8985e0020000             mov        dword ptr [rbp + 0x2e0], eax
00000001408f631b  4c897c2430               mov        qword ptr [rsp + 0x30], r15
00000001408f6320  4c89742428               mov        qword ptr [rsp + 0x28], r14
00000001408f6325  48c744242027000000       mov        qword ptr [rsp + 0x20], 0x27
00000001408f632e  488d15f9271b02           lea        rdx, [rip + 0x21b27f9]
00000001408f6335  4c8d0d06281b02           lea        r9, [rip + 0x21b2806]
00000001408f633c  4c8db550010000           lea        r14, [rbp + 0x150]
00000001408f6343  41b814000000             mov        r8d, 0x14
00000001408f6349  4c89f1                   mov        rcx, r14
00000001408f634c  e84fcfd000               call       0x1416032a0
00000001408f6351  4c89ad30030000           mov        qword ptr [rbp + 0x330], r13
00000001408f6358  c745c002000000           mov        dword ptr [rbp - 0x40], 2
00000001408f635f  4c89642420               mov        qword ptr [rsp + 0x20], r12
00000001408f6364  488d8d30030000           lea        rcx, [rbp + 0x330]
00000001408f636b  49b8e35e28f33f8611bd     movabs     r8, 0xbd11863ff3285ee3
00000001408f6375  4c89f2                   mov        rdx, r14
00000001408f6378  41b119                   mov        r9b, 0x19
00000001408f637b  e8c07effff               call       0x1408ee240
00000001408f6380  488d4dc0                 lea        rcx, [rbp - 0x40]
00000001408f6384  488d9550010000           lea        rdx, [rbp + 0x150]
00000001408f638b  41b890010000             mov        r8d, 0x190
00000001408f6391  e86ad80202               call       0x142923c00
00000001408f6396  c6858f03000001           mov        byte ptr [rbp + 0x38f], 1
00000001408f639d  488d8d50010000           lea        rcx, [rbp + 0x150]
00000001408f63a4  e8f7f32d00               call       0x140bd57a0
00000001408f63a9  c6858f03000001           mov        byte ptr [rbp + 0x38f], 1
00000001408f63b0  488d8d30030000           lea        rcx, [rbp + 0x330]
00000001408f63b7  4c89f2                   mov        rdx, r14
00000001408f63ba  41b804000000             mov        r8d, 4
00000001408f63c0  e8dbf42d00               call       0x140bd58a0
00000001408f63c5  4c89ad28030000           mov        qword ptr [rbp + 0x328], r13
00000001408f63cc  0f108530030000           movups     xmm0, xmmword ptr [rbp + 0x330]
00000001408f63d3  0f108d40030000           movups     xmm1, xmmword ptr [rbp + 0x340]
00000001408f63da  0f109550030000           movups     xmm2, xmmword ptr [rbp + 0x350]
00000001408f63e1  0f109d5c030000           movups     xmm3, xmmword ptr [rbp + 0x35c]
00000001408f63e8  0f119d7c010000           movups     xmmword ptr [rbp + 0x17c], xmm3
00000001408f63ef  0f299570010000           movaps     xmmword ptr [rbp + 0x170], xmm2
00000001408f63f6  0f298d60010000           movaps     xmmword ptr [rbp + 0x160], xmm1
00000001408f63fd  0f298550010000           movaps     xmmword ptr [rbp + 0x150], xmm0
00000001408f6404  4c89742420               mov        qword ptr [rsp + 0x20], r14
00000001408f6409  488d8d28030000           lea        rcx, [rbp + 0x328]
00000001408f6410  488d55c0                 lea        rdx, [rbp - 0x40]
00000001408f6414  49b855449adc0cb1e431     movabs     r8, 0x31e4b10cdc9a4455
00000001408f641e  41b119                   mov        r9b, 0x19
00000001408f6421  e81a7effff               call       0x1408ee240
00000001408f6426  4c8d7dc0                 lea        r15, [rbp - 0x40]
00000001408f642a  41b890010000             mov        r8d, 0x190
00000001408f6430  4c89f1                   mov        rcx, r14
00000001408f6433  4c89fa                   mov        rdx, r15
00000001408f6436  e8c5d70202               call       0x142923c00
00000001408f643b  488d8504030000           lea        rax, [rbp + 0x304]
00000001408f6442  48898530030000           mov        qword ptr [rbp + 0x330], rax
00000001408f6449  c745c002000000           mov        dword ptr [rbp - 0x40], 2
00000001408f6450  4c897c2420               mov        qword ptr [rsp + 0x20], r15
00000001408f6455  488d8d30030000           lea        rcx, [rbp + 0x330]
00000001408f645c  49b834ace0e67afc2170     movabs     r8, 0x7021fc7ae6e0ac34
00000001408f6466  4c89f2                   mov        rdx, r14
00000001408f6469  41b132                   mov        r9b, 0x32
00000001408f646c  e88f78ffff               call       0x1408edd00
00000001408f6471  4c8d75c0                 lea        r14, [rbp - 0x40]
00000001408f6475  4c8dbd50010000           lea        r15, [rbp + 0x150]
00000001408f647c  41b890010000             mov        r8d, 0x190
00000001408f6482  4c89f1                   mov        rcx, r14
00000001408f6485  4c89fa                   mov        rdx, r15
00000001408f6488  e873d70202               call       0x142923c00
00000001408f648d  48899d30030000           mov        qword ptr [rbp + 0x330], rbx
00000001408f6494  c7855001000002000000     mov        dword ptr [rbp + 0x150], 2
00000001408f649e  4c897c2420               mov        qword ptr [rsp + 0x20], r15
00000001408f64a3  488d8d30030000           lea        rcx, [rbp + 0x330]
00000001408f64aa  49b8c5564f19ad49826e     movabs     r8, 0x6e8249ad194f56c5
00000001408f64b4  4c89f2                   mov        rdx, r14
00000001408f64b7  41b119                   mov        r9b, 0x19
00000001408f64ba  e8817dffff               call       0x1408ee240
00000001408f64bf  488d9d50010000           lea        rbx, [rbp + 0x150]
00000001408f64c6  41b890010000             mov        r8d, 0x190
00000001408f64cc  4889d9                   mov        rcx, rbx
00000001408f64cf  4c89f2                   mov        rdx, r14
00000001408f64d2  e829d70202               call       0x142923c00
00000001408f64d7  488d85e0020000           lea        rax, [rbp + 0x2e0]
00000001408f64de  48898530030000           mov        qword ptr [rbp + 0x330], rax
00000001408f64e5  c745c002000000           mov        dword ptr [rbp - 0x40], 2
00000001408f64ec  4c89742420               mov        qword ptr [rsp + 0x20], r14
00000001408f64f1  488d8d30030000           lea        rcx, [rbp + 0x330]
00000001408f64f8  49b867cc343bb86c1327     movabs     r8, 0x27136cb83b34cc67
00000001408f6502  4889da                   mov        rdx, rbx
00000001408f6505  41b132                   mov        r9b, 0x32
00000001408f6508  e8f377ffff               call       0x1408edd00
00000001408f650d  488d4dc0                 lea        rcx, [rbp - 0x40]
00000001408f6511  488d9550010000           lea        rdx, [rbp + 0x150]
00000001408f6518  41b890010000             mov        r8d, 0x190
00000001408f651e  e8ddd60202               call       0x142923c00
00000001408f6523  4889f8                   mov        rax, rdi
00000001408f6526  48c1e820                 shr        rax, 0x20
00000001408f652a  0f104560                 movups     xmm0, xmmword ptr [rbp + 0x60]
00000001408f652e  0f298570030000           movaps     xmmword ptr [rbp + 0x370], xmm0
00000001408f6535  0f104550                 movups     xmm0, xmmword ptr [rbp + 0x50]
00000001408f6539  0f298560030000           movaps     xmmword ptr [rbp + 0x360], xmm0
00000001408f6540  0f104540                 movups     xmm0, xmmword ptr [rbp + 0x40]
00000001408f6544  0f298550030000           movaps     xmmword ptr [rbp + 0x350], xmm0
00000001408f654b  0f104530                 movups     xmm0, xmmword ptr [rbp + 0x30]
00000001408f654f  0f298540030000           movaps     xmmword ptr [rbp + 0x340], xmm0
00000001408f6556  0f104520                 movups     xmm0, xmmword ptr [rbp + 0x20]
00000001408f655a  0f298530030000           movaps     xmmword ptr [rbp + 0x330], xmm0
00000001408f6561  488b8df8000000           mov        rcx, qword ptr [rbp + 0xf8]
00000001408f6568  488b9540010000           mov        rdx, qword ptr [rbp + 0x140]
00000001408f656f  4c8b45d0                 mov        r8, qword ptr [rbp - 0x30]
00000001408f6573  4c898560010000           mov        qword ptr [rbp + 0x160], r8
00000001408f657a  0f1045c0                 movups     xmm0, xmmword ptr [rbp - 0x40]
00000001408f657e  0f298550010000           movaps     xmmword ptr [rbp + 0x150], xmm0
00000001408f6585  0f1045d8                 movups     xmm0, xmmword ptr [rbp - 0x28]
00000001408f6589  0f298580010000           movaps     xmmword ptr [rbp + 0x180], xmm0
00000001408f6590  4c8b45e8                 mov        r8, qword ptr [rbp - 0x18]
00000001408f6594  4c898590010000           mov        qword ptr [rbp + 0x190], r8
00000001408f659b  0f1045f0                 movups     xmm0, xmmword ptr [rbp - 0x10]
00000001408f659f  0f118568010000           movups     xmmword ptr [rbp + 0x168], xmm0
00000001408f65a6  4c8b4500                 mov        r8, qword ptr [rbp]
00000001408f65aa  4c898578010000           mov        qword ptr [rbp + 0x178], r8
00000001408f65b1  0f104570                 movups     xmm0, xmmword ptr [rbp + 0x70]
00000001408f65b5  0f108d80000000           movups     xmm1, xmmword ptr [rbp + 0x80]
00000001408f65bc  0f109590000000           movups     xmm2, xmmword ptr [rbp + 0x90]
00000001408f65c3  0f109da0000000           movups     xmm3, xmmword ptr [rbp + 0xa0]
00000001408f65ca  0f1185c8010000           movups     xmmword ptr [rbp + 0x1c8], xmm0
00000001408f65d1  0f118dd8010000           movups     xmmword ptr [rbp + 0x1d8], xmm1
00000001408f65d8  0f1195e8010000           movups     xmmword ptr [rbp + 0x1e8], xmm2
00000001408f65df  0f119df8010000           movups     xmmword ptr [rbp + 0x1f8], xmm3
00000001408f65e6  0f1085b0000000           movups     xmm0, xmmword ptr [rbp + 0xb0]
00000001408f65ed  0f118508020000           movups     xmmword ptr [rbp + 0x208], xmm0
00000001408f65f4  0f1085c0000000           movups     xmm0, xmmword ptr [rbp + 0xc0]
00000001408f65fb  0f118518020000           movups     xmmword ptr [rbp + 0x218], xmm0
00000001408f6602  4c8b4518                 mov        r8, qword ptr [rbp + 0x18]
00000001408f6606  4c8985a8010000           mov        qword ptr [rbp + 0x1a8], r8
00000001408f660d  0f104508                 movups     xmm0, xmmword ptr [rbp + 8]
00000001408f6611  0f118598010000           movups     xmmword ptr [rbp + 0x198], xmm0
00000001408f6618  48899530020000           mov        qword ptr [rbp + 0x230], rdx
00000001408f661f  48898d28020000           mov        qword ptr [rbp + 0x228], rcx
00000001408f6626  c785b001000001000000     mov        dword ptr [rbp + 0x1b0], 1
00000001408f6630  89bdb4010000             mov        dword ptr [rbp + 0x1b4], edi
00000001408f6636  8985b8010000             mov        dword ptr [rbp + 0x1b8], eax
00000001408f663c  c785bc01000001000000     mov        dword ptr [rbp + 0x1bc], 1
00000001408f6646  488d8d30030000           lea        rcx, [rbp + 0x330]
00000001408f664d  488d9550010000           lea        rdx, [rbp + 0x150]
00000001408f6654  e8776e0e00               call       0x1409dd4d0
00000001408f6659  80bdf600000008           cmp        byte ptr [rbp + 0xf6], 8
00000001408f6660  740c                     je         0x1408f666e
00000001408f6662  488d8de0000000           lea        rcx, [rbp + 0xe0]
00000001408f6669  e842be2e00               call       0x140be24b0
00000001408f666e  8b8524030000             mov        eax, dword ptr [rbp + 0x324]
00000001408f6674  894620                   mov        dword ptr [rsi + 0x20], eax
00000001408f6677  0f108504030000           movups     xmm0, xmmword ptr [rbp + 0x304]
00000001408f667e  0f108d14030000           movups     xmm1, xmmword ptr [rbp + 0x314]
00000001408f6685  0f114e10                 movups     xmmword ptr [rsi + 0x10], xmm1
00000001408f6689  0f1106                   movups     xmmword ptr [rsi], xmm0
00000001408f668c  0f1085e0020000           movups     xmm0, xmmword ptr [rbp + 0x2e0]
00000001408f6693  0f108df0020000           movups     xmm1, xmmword ptr [rbp + 0x2f0]
00000001408f669a  0f114624                 movups     xmmword ptr [rsi + 0x24], xmm0
00000001408f669e  0f114e34                 movups     xmmword ptr [rsi + 0x34], xmm1
00000001408f66a2  8b8500030000             mov        eax, dword ptr [rbp + 0x300]
00000001408f66a8  894644                   mov        dword ptr [rsi + 0x44], eax
00000001408f66ab  4889f0                   mov        rax, rsi
00000001408f66ae  4881c418040000           add        rsp, 0x418
00000001408f66b5  5b                       pop        rbx
00000001408f66b6  5f                       pop        rdi
00000001408f66b7  5e                       pop        rsi
00000001408f66b8  415c                     pop        r12
00000001408f66ba  415d                     pop        r13
00000001408f66bc  415e                     pop        r14
00000001408f66be  415f                     pop        r15
00000001408f66c0  5d                       pop        rbp
00000001408f66c1  c3                       ret        
00000001408f66c2  66666666662e0f1f840000000000 nop        word ptr cs:[rax + rax]
00000001408f66d0  4889542410               mov        qword ptr [rsp + 0x10], rdx
00000001408f66d5  55                       push       rbp
00000001408f66d6  4157                     push       r15
00000001408f66d8  4156                     push       r14
00000001408f66da  4155                     push       r13
00000001408f66dc  4154                     push       r12
00000001408f66de  56                       push       rsi
00000001408f66df  57                       push       rdi
00000001408f66e0  53                       push       rbx
00000001408f66e1  4883ec38                 sub        rsp, 0x38
00000001408f66e5  488daa80000000           lea        rbp, [rdx + 0x80]
00000001408f66ec  80bdf600000008           cmp        byte ptr [rbp + 0xf6], 8
00000001408f66f3  740c                     je         0x1408f6701
00000001408f66f5  488d8de0000000           lea        rcx, [rbp + 0xe0]
00000001408f66fc  e8afbd2e00               call       0x140be24b0
00000001408f6701  90                       nop        
00000001408f6702  4883c438                 add        rsp, 0x38
00000001408f6706  5b                       pop        rbx
00000001408f6707  5f                       pop        rdi
00000001408f6708  5e                       pop        rsi
00000001408f6709  415c                     pop        r12
00000001408f670b  415d                     pop        r13
00000001408f670d  415e                     pop        r14
00000001408f670f  415f                     pop        r15
00000001408f6711  5d                       pop        rbp
00000001408f6712  c3                       ret        
00000001408f6713  666666662e0f1f840000000000 nop        word ptr cs:[rax + rax]
00000001408f6720  4889542410               mov        qword ptr [rsp + 0x10], rdx
00000001408f6725  55                       push       rbp
00000001408f6726  4157                     push       r15
00000001408f6728  4156                     push       r14
00000001408f672a  4155                     push       r13
00000001408f672c  4154                     push       r12
00000001408f672e  56                       push       rsi
00000001408f672f  57                       push       rdi
00000001408f6730  53                       push       rbx
00000001408f6731  4883ec38                 sub        rsp, 0x38
00000001408f6735  488daa80000000           lea        rbp, [rdx + 0x80]
00000001408f673c  80bd8f03000000           cmp        byte ptr [rbp + 0x38f], 0
00000001408f6743  7511                     jne        0x1408f6756
00000001408f6745  4883c438                 add        rsp, 0x38
00000001408f6749  5b                       pop        rbx
00000001408f674a  5f                       pop        rdi
00000001408f674b  5e                       pop        rsi
00000001408f674c  415c                     pop        r12
00000001408f674e  415d                     pop        r13
00000001408f6750  415e                     pop        r14
00000001408f6752  415f                     pop        r15
00000001408f6754  5d                       pop        rbp
00000001408f6755  c3                       ret        
00000001408f6756  488d4dc0                 lea        rcx, [rbp - 0x40]
00000001408f675a  e85108faff               call       0x140896fb0
00000001408f675f  ebe4                     jmp        0x1408f6745
00000001408f6761  6666666666662e0f1f840000000000 nop        word ptr cs:[rax + rax]
00000001408f6770  4889542410               mov        qword ptr [rsp + 0x10], rdx
00000001408f6775  55                       push       rbp
00000001408f6776  4157                     push       r15
00000001408f6778  4156                     push       r14
00000001408f677a  4155                     push       r13
00000001408f677c  4154                     push       r12
00000001408f677e  56                       push       rsi
00000001408f677f  57                       push       rdi
00000001408f6780  53                       push       rbx
00000001408f6781  4883ec38                 sub        rsp, 0x38
00000001408f6785  488daa80000000           lea        rbp, [rdx + 0x80]
00000001408f678c  488d8d50010000           lea        rcx, [rbp + 0x150]
00000001408f6793  e81808faff               call       0x140896fb0
00000001408f6798  90                       nop        
00000001408f6799  4883c438                 add        rsp, 0x38
00000001408f679d  5b                       pop        rbx
00000001408f679e  5f                       pop        rdi
00000001408f679f  5e                       pop        rsi
00000001408f67a0  415c                     pop        r12
00000001408f67a2  415d                     pop        r13
00000001408f67a4  415e                     pop        r14
00000001408f67a6  415f                     pop        r15
00000001408f67a8  5d                       pop        rbp
00000001408f67a9  c3                       ret        
00000001408f67aa  660f1f440000             nop        word ptr [rax + rax]
00000001408f67b0  4889542410               mov        qword ptr [rsp + 0x10], rdx
00000001408f67b5  55                       push       rbp
00000001408f67b6  4157                     push       r15
00000001408f67b8  4156                     push       r14
00000001408f67ba  4155                     push       r13
00000001408f67bc  4154                     push       r12
00000001408f67be  56                       push       rsi
00000001408f67bf  57                       push       rdi
00000001408f67c0  53                       push       rbx
00000001408f67c1  4883ec38                 sub        rsp, 0x38
00000001408f67c5  488daa80000000           lea        rbp, [rdx + 0x80]
00000001408f67cc  c6858f03000000           mov        byte ptr [rbp + 0x38f], 0
00000001408f67d3  488d4dc0                 lea        rcx, [rbp - 0x40]
00000001408f67d7  e8d407faff               call       0x140896fb0
00000001408f67dc  c6858f03000000           mov        byte ptr [rbp + 0x38f], 0
00000001408f67e3  4883c438                 add        rsp, 0x38
00000001408f67e7  5b                       pop        rbx
00000001408f67e8  5f                       pop        rdi
00000001408f67e9  5e                       pop        rsi
00000001408f67ea  415c                     pop        r12
00000001408f67ec  415d                     pop        r13
00000001408f67ee  415e                     pop        r14
00000001408f67f0  415f                     pop        r15
00000001408f67f2  5d                       pop        rbp
00000001408f67f3  c3                       ret        
00000001408f67f4  6666662e0f1f840000000000 nop        word ptr cs:[rax + rax]
00000001408f6800  4889542410               mov        qword ptr [rsp + 0x10], rdx
00000001408f6805  55                       push       rbp
00000001408f6806  4157                     push       r15
00000001408f6808  4156                     push       r14
00000001408f680a  4155                     push       r13
00000001408f680c  4154                     push       r12
00000001408f680e  56                       push       rsi
00000001408f680f  57                       push       rdi
00000001408f6810  53                       push       rbx
00000001408f6811  4883ec38                 sub        rsp, 0x38
00000001408f6815  488daa80000000           lea        rbp, [rdx + 0x80]
00000001408f681c  488d8d50010000           lea        rcx, [rbp + 0x150]
00000001408f6823  e88807faff               call       0x140896fb0
00000001408f6828  90                       nop        
00000001408f6829  4883c438                 add        rsp, 0x38
00000001408f682d  5b                       pop        rbx
00000001408f682e  5f                       pop        rdi
00000001408f682f  5e                       pop        rsi
00000001408f6830  415c                     pop        r12
00000001408f6832  415d                     pop        r13
00000001408f6834  415e                     pop        r14
00000001408f6836  415f                     pop        r15
00000001408f6838  5d                       pop        rbp
00000001408f6839  c3                       ret        
00000001408f683a  660f1f440000             nop        word ptr [rax + rax]
00000001408f6840  4889542410               mov        qword ptr [rsp + 0x10], rdx
00000001408f6845  55                       push       rbp
00000001408f6846  4157                     push       r15
00000001408f6848  4156                     push       r14
00000001408f684a  4155                     push       r13
00000001408f684c  4154                     push       r12
00000001408f684e  56                       push       rsi
00000001408f684f  57                       push       rdi
00000001408f6850  53                       push       rbx
00000001408f6851  4883ec38                 sub        rsp, 0x38
00000001408f6855  488daa80000000           lea        rbp, [rdx + 0x80]
00000001408f685c  488d4dc0                 lea        rcx, [rbp - 0x40]
00000001408f6860  e84b07faff               call       0x140896fb0
00000001408f6865  90                       nop        
00000001408f6866  4883c438                 add        rsp, 0x38
00000001408f686a  5b                       pop        rbx
00000001408f686b  5f                       pop        rdi
00000001408f686c  5e                       pop        rsi
00000001408f686d  415c                     pop        r12
00000001408f686f  415d                     pop        r13
00000001408f6871  415e                     pop        r14
00000001408f6873  415f                     pop        r15
00000001408f6875  5d                       pop        rbp
00000001408f6876  c3                       ret        
00000001408f6877  660f1f840000000000       nop        word ptr [rax + rax]
00000001408f6880  4889542410               mov        qword ptr [rsp + 0x10], rdx
00000001408f6885  55                       push       rbp
00000001408f6886  4157                     push       r15
00000001408f6888  4156                     push       r14
00000001408f688a  4155                     push       r13
00000001408f688c  4154                     push       r12
00000001408f688e  56                       push       rsi
00000001408f688f  57                       push       rdi
00000001408f6890  53                       push       rbx
00000001408f6891  4883ec38                 sub        rsp, 0x38
00000001408f6895  488daa80000000           lea        rbp, [rdx + 0x80]
00000001408f689c  488d8d50010000           lea        rcx, [rbp + 0x150]
00000001408f68a3  e80807faff               call       0x140896fb0
00000001408f68a8  90                       nop        
00000001408f68a9  4883c438                 add        rsp, 0x38
00000001408f68ad  5b                       pop        rbx
00000001408f68ae  5f                       pop        rdi
00000001408f68af  5e                       pop        rsi
00000001408f68b0  415c                     pop        r12
00000001408f68b2  415d                     pop        r13
00000001408f68b4  415e                     pop        r14
00000001408f68b6  415f                     pop        r15
00000001408f68b8  5d                       pop        rbp
00000001408f68b9  c3                       ret        
