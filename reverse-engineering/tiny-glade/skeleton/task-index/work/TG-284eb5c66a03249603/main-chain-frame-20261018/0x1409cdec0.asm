0x1409cdec0 55 push rbp
0x1409cdec1 4157 push r15
0x1409cdec3 4156 push r14
0x1409cdec5 4155 push r13
0x1409cdec7 4154 push r12
0x1409cdec9 56 push rsi
0x1409cdeca 57 push rdi
0x1409cdecb 53 push rbx
0x1409cdecc 4881ec980b0000 sub rsp, 0xb98
0x1409cded3 488dac2480000000 lea rbp, [rsp + 0x80]
0x1409cdedb 0f29b5000b0000 movaps xmmword ptr [rbp + 0xb00], xmm6
0x1409cdee2 48c785f80a0000feffffff mov qword ptr [rbp + 0xaf8], 0xfffffffffffffffe
0x1409cdeed 4c89ce mov rsi, r9
0x1409cdef0 4c89c7 mov rdi, r8
0x1409cdef3 4889d3 mov rbx, rdx
0x1409cdef6 4989ce mov r14, rcx
0x1409cdef9 e8028b2400 call 0x140c16a00
0x1409cdefe 84c0 test al, al
0x1409cdf00 8885e80a0000 mov byte ptr [rbp + 0xae8], al
0x1409cdf06 7460 je 0x1409cdf68
0x1409cdf08 4c8d25bc340f02 lea r12, [rip + 0x20f34bc]
0x1409cdf0f 41bd39000000 mov r13d, 0x39
0x1409cdf15 ba39000000 mov edx, 0x39
0x1409cdf1a 4c89e1 mov rcx, r12
0x1409cdf1d e8def8ffff call 0x1409cd800
0x1409cdf22 4883f801 cmp rax, 1
0x1409cdf26 0f85b2000000 jne 0x1409cdfde
0x1409cdf2c 4989d1 mov r9, rdx
0x1409cdf2f 4885d2 test rdx, rdx
0x1409cdf32 743d je 0x1409cdf71
0x1409cdf34 4983f939 cmp r9, 0x39
0x1409cdf38 7335 jae 0x1409cdf6f
0x1409cdf3a 488d058a340f02 lea rax, [rip + 0x20f348a]
0x1409cdf41 41803c01bf cmp byte ptr [r9 + rax], 0xbf
0x1409cdf46 7f29 jg 0x1409cdf71
0x1409cdf48 488d05d1350f02 lea rax, [rip + 0x20f35d1]
0x1409cdf4f 4889442420 mov qword ptr [rsp + 0x20], rax
0x1409cdf54 488d0d70340f02 lea rcx, [rip + 0x20f3470]
0x1409cdf5b ba39000000 mov edx, 0x39
0x1409cdf60 4531c0 xor r8d, r8d
0x1409cdf63 e8d8b9f001 call 0x1428d9940
0x1409cdf68 31c9 xor ecx, ecx
0x1409cdf6a e9e7010000 jmp 0x1409ce156
0x1409cdf6f 75d7 jne 0x1409cdf48
0x1409cdf71 4c8d2553340f02 lea r12, [rip + 0x20f3453]
0x1409cdf78 4c89e1 mov rcx, r12
0x1409cdf7b 4c89ca mov rdx, r9
0x1409cdf7e e87df8ffff call 0x1409cd800
0x1409cdf83 4883f801 cmp rax, 1
0x1409cdf87 7555 jne 0x1409cdfde
0x1409cdf89 4989d0 mov r8, rdx
0x1409cdf8c 4983c002 add r8, 2
0x1409cdf90 7439 je 0x1409cdfcb
0x1409cdf92 4983f839 cmp r8, 0x39
0x1409cdf96 7331 jae 0x1409cdfc9
0x1409cdf98 488d052c340f02 lea rax, [rip + 0x20f342c]
0x1409cdf9f 41803c00bf cmp byte ptr [r8 + rax], 0xbf
0x1409cdfa4 7f25 jg 0x1409cdfcb
0x1409cdfa6 488d058b350f02 lea rax, [rip + 0x20f358b]
0x1409cdfad 4889442420 mov qword ptr [rsp + 0x20], rax
0x1409cdfb2 488d0d12340f02 lea rcx, [rip + 0x20f3412]
0x1409cdfb9 ba39000000 mov edx, 0x39
0x1409cdfbe 41b939000000 mov r9d, 0x39
0x1409cdfc4 e877b9f001 call 0x1428d9940
0x1409cdfc9 75db jne 0x1409cdfa6
0x1409cdfcb 41bd37000000 mov r13d, 0x37
0x1409cdfd1 4929d5 sub r13, rdx
0x1409cdfd4 4c8d25f0330f02 lea r12, [rip + 0x20f33f0]
0x1409cdfdb 4d01c4 add r12, r8
0x1409cdfde 4c89add00a0000 mov qword ptr [rbp + 0xad0], r13
0x1409cdfe5 488d050e360f02 lea rax, [rip + 0x20f360e]
0x1409cdfec 4c8d2dd8350f02 lea r13, [rip + 0x20f35d8]
0x1409cdff3 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409ce000 0fbe48ff movsx ecx, byte ptr [rax - 1]
0x1409ce004 85c9 test ecx, ecx
0x1409ce006 7818 js 0x1409ce020
0x1409ce008 48ffc8 dec rax
0x1409ce00b 83f95c cmp ecx, 0x5c
0x1409ce00e 7569 jne 0x1409ce079
0x1409ce010 eb7d jmp 0x1409ce08f
0x1409ce012 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409ce020 0fb650fe movzx edx, byte ptr [rax - 2]
0x1409ce024 80fac0 cmp dl, 0xc0
0x1409ce027 7d25 jge 0x1409ce04e
0x1409ce029 440fb640fd movzx r8d, byte ptr [rax - 3]
0x1409ce02e 4180f8c0 cmp r8b, 0xc0
0x1409ce032 7d23 jge 0x1409ce057
0x1409ce034 440fb648fc movzx r9d, byte ptr [rax - 4]
0x1409ce039 4883c0fc add rax, -4
0x1409ce03d 4183e107 and r9d, 7
0x1409ce041 41c1e106 shl r9d, 6
0x1409ce045 4183e03f and r8d, 0x3f
0x1409ce049 4509c8 or r8d, r9d
0x1409ce04c eb11 jmp 0x1409ce05f
0x1409ce04e 4883c0fe add rax, -2
0x1409ce052 83e21f and edx, 0x1f
0x1409ce055 eb12 jmp 0x1409ce069
0x1409ce057 4883c0fd add rax, -3
0x1409ce05b 4183e00f and r8d, 0xf
0x1409ce05f 41c1e006 shl r8d, 6
0x1409ce063 83e23f and edx, 0x3f
0x1409ce066 4409c2 or edx, r8d
0x1409ce069 c1e206 shl edx, 6
0x1409ce06c 80e13f and cl, 0x3f
0x1409ce06f 0fb6c9 movzx ecx, cl
0x1409ce072 09d1 or ecx, edx
0x1409ce074 83f95c cmp ecx, 0x5c
0x1409ce077 7416 je 0x1409ce08f
0x1409ce079 83f92f cmp ecx, 0x2f
0x1409ce07c 7411 je 0x1409ce08f
0x1409ce07e 4c39e8 cmp rax, r13
0x1409ce081 0f8579ffffff jne 0x1409ce000
0x1409ce087 41bf2f000000 mov r15d, 0x2f
0x1409ce08d eb50 jmp 0x1409ce0df
0x1409ce08f 4c8d2d35350f02 lea r13, [rip + 0x20f3535]
0x1409ce096 4c29e8 sub rax, r13
0x1409ce099 4989c0 mov r8, rax
0x1409ce09c 49ffc0 inc r8
0x1409ce09f 7432 je 0x1409ce0d3
0x1409ce0a1 4983f82f cmp r8, 0x2f
0x1409ce0a5 732a jae 0x1409ce0d1
0x1409ce0a7 43803c28bf cmp byte ptr [r8 + r13], 0xbf
0x1409ce0ac 7f25 jg 0x1409ce0d3
0x1409ce0ae 488d054b340f02 lea rax, [rip + 0x20f344b]
0x1409ce0b5 4889442420 mov qword ptr [rsp + 0x20], rax
0x1409ce0ba 488d0d0a350f02 lea rcx, [rip + 0x20f350a]
0x1409ce0c1 ba2f000000 mov edx, 0x2f
0x1409ce0c6 41b92f000000 mov r9d, 0x2f
0x1409ce0cc e86fb8f001 call 0x1428d9940
0x1409ce0d1 75db jne 0x1409ce0ae
0x1409ce0d3 41bf2e000000 mov r15d, 0x2e
0x1409ce0d9 4929c7 sub r15, rax
0x1409ce0dc 4d01c5 add r13, r8
0x1409ce0df e84ca02400 call 0x140c18130
0x1409ce0e4 4889c1 mov rcx, rax
0x1409ce0e7 488b00 mov rax, qword ptr [rax]
0x1409ce0ea 4883f801 cmp rax, 1
0x1409ce0ee 0f85ff030000 jne 0x1409ce4f3
0x1409ce0f4 4883c108 add rcx, 8
0x1409ce0f8 48833900 cmp qword ptr [rcx], 0
0x1409ce0fc 0f852b040000 jne 0x1409ce52d
0x1409ce102 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
0x1409ce109 48898dd80a0000 mov qword ptr [rbp + 0xad8], rcx
0x1409ce110 4883c108 add rcx, 8
0x1409ce114 4c897c2420 mov qword ptr [rsp + 0x20], r15
0x1409ce119 48c744243000000000 mov qword ptr [rsp + 0x30], 0
0x1409ce122 48c744242801000000 mov qword ptr [rsp + 0x28], 1
0x1409ce12b 4c89e2 mov rdx, r12
0x1409ce12e 4c8b85d00a0000 mov r8, qword ptr [rbp + 0xad0]
0x1409ce135 4d89e9 mov r9, r13
0x1409ce138 e8038c2400 call 0x140c16d40
0x1409ce13d 4989c7 mov r15, rax
0x1409ce140 488b85d80a0000 mov rax, qword ptr [rbp + 0xad8]
0x1409ce147 48ff00 inc qword ptr [rax]
0x1409ce14a 4c89bdb80a0000 mov qword ptr [rbp + 0xab8], r15
0x1409ce151 b901000000 mov ecx, 1
0x1409ce156 488b85880b0000 mov rax, qword ptr [rbp + 0xb88]
0x1409ce15d 48898db00a0000 mov qword ptr [rbp + 0xab0], rcx
0x1409ce164 4c8b28 mov r13, qword ptr [rax]
0x1409ce167 4c8b6018 mov r12, qword ptr [rax + 0x18]
0x1409ce16b 4c89e1 mov rcx, r12
0x1409ce16e 4183fd01 cmp r13d, 1
0x1409ce172 7404 je 0x1409ce178
0x1409ce174 488b4808 mov rcx, qword ptr [rax + 8]
0x1409ce178 0f107008 movups xmm6, xmmword ptr [rax + 8]
0x1409ce17c 4883c104 add rcx, 4
0x1409ce180 e80b762000 call 0x140bd5790
0x1409ce185 66490f6ec5 movq xmm0, r13
0x1409ce18a 660fd685a0050000 movq qword ptr [rbp + 0x5a0], xmm0
0x1409ce192 0f11b5a8050000 movups xmmword ptr [rbp + 0x5a8], xmm6
0x1409ce199 4c89a5b8050000 mov qword ptr [rbp + 0x5b8], r12
0x1409ce1a0 c7858809000002000000 mov dword ptr [rbp + 0x988], 2
0x1409ce1aa 4c89742440 mov qword ptr [rsp + 0x40], r14
0x1409ce1af 48895c2438 mov qword ptr [rsp + 0x38], rbx
0x1409ce1b4 4889442430 mov qword ptr [rsp + 0x30], rax
0x1409ce1b9 488d8588090000 lea rax, [rbp + 0x988]
0x1409ce1c0 4889442428 mov qword ptr [rsp + 0x28], rax
0x1409ce1c5 48c744242001000000 mov qword ptr [rsp + 0x20], 1
0x1409ce1ce 488d4dc8 lea rcx, [rbp - 0x38]
0x1409ce1d2 4c8d8da0050000 lea r9, [rbp + 0x5a0]
0x1409ce1d9 4889fa mov rdx, rdi
0x1409ce1dc 4989f0 mov r8, rsi
0x1409ce1df e88c97e9ff call 0x140867970
0x1409ce1e4 488b85900b0000 mov rax, qword ptr [rbp + 0xb90]
0x1409ce1eb 8b4820 mov ecx, dword ptr [rax + 0x20]
0x1409ce1ee 898da00a0000 mov dword ptr [rbp + 0xaa0], ecx
0x1409ce1f4 f30f6f00 movdqu xmm0, xmmword ptr [rax]
0x1409ce1f8 0f104810 movups xmm1, xmmword ptr [rax + 0x10]
0x1409ce1fc 0f298d900a0000 movaps xmmword ptr [rbp + 0xa90], xmm1
0x1409ce203 660f7f85800a0000 movdqa xmmword ptr [rbp + 0xa80], xmm0
0x1409ce20b c685f70a000001 mov byte ptr [rbp + 0xaf7], 1
0x1409ce212 e839852100 call 0x140be6750
0x1409ce217 89c6 mov esi, eax
0x1409ce219 c685f70a000001 mov byte ptr [rbp + 0xaf7], 1
0x1409ce220 31c9 xor ecx, ecx
0x1409ce222 e8f9b870ff call 0x1400d9b20
0x1409ce227 4885c0 test rax, rax
0x1409ce22a 0f8409030000 je 0x1409ce539
0x1409ce230 488b8d800b0000 mov rcx, qword ptr [rbp + 0xb80]
0x1409ce237 89b548060000 mov dword ptr [rbp + 0x648], esi
0x1409ce23d 66c785500600000000 mov word ptr [rbp + 0x650], 0
0x1409ce246 c7854c0600000000803f mov dword ptr [rbp + 0x64c], 0x3f800000
0x1409ce250 48c785a005000000000000 mov qword ptr [rbp + 0x5a0], 0
0x1409ce25b 48898d40060000 mov qword ptr [rbp + 0x640], rcx
0x1409ce262 48c785b005000000000000 mov qword ptr [rbp + 0x5b0], 0
0x1409ce26d 48c785b805000004000000 mov qword ptr [rbp + 0x5b8], 4
0x1409ce278 48c785c005000000000000 mov qword ptr [rbp + 0x5c0], 0
0x1409ce283 f30f6f00 movdqu xmm0, xmmword ptr [rax]
0x1409ce287 48ff00 inc qword ptr [rax]
0x1409ce28a 0f100d9f840c02 movups xmm1, xmmword ptr [rip + 0x20c849f]
0x1409ce291 0f118d20060000 movups xmmword ptr [rbp + 0x620], xmm1
0x1409ce298 0f100d81840c02 movups xmm1, xmmword ptr [rip + 0x20c8481]
0x1409ce29f 0f118d10060000 movups xmmword ptr [rbp + 0x610], xmm1
0x1409ce2a6 f30f7f8530060000 movdqu xmmword ptr [rbp + 0x630], xmm0
0x1409ce2ae 48b80300000001000000 movabs rax, 0x100000003
0x1409ce2b8 488985e0050000 mov qword ptr [rbp + 0x5e0], rax
0x1409ce2bf 48c785c805000000000000 mov qword ptr [rbp + 0x5c8], 0
0x1409ce2ca 48c785d005000001000000 mov qword ptr [rbp + 0x5d0], 1
0x1409ce2d5 48c785d805000000000000 mov qword ptr [rbp + 0x5d8], 0
0x1409ce2e0 c685f70a000001 mov byte ptr [rbp + 0xaf7], 1
0x1409ce2e7 488d8db0030000 lea rcx, [rbp + 0x3b0]
0x1409ce2ee 488db5a0050000 lea rsi, [rbp + 0x5a0]
0x1409ce2f5 4c8d45c8 lea r8, [rbp - 0x38]
0x1409ce2f9 4889f2 mov rdx, rsi
0x1409ce2fc 4531c9 xor r9d, r9d
0x1409ce2ff e8dc78f3ff call 0x140905be0
0x1409ce304 488dbda8040000 lea rdi, [rbp + 0x4a8]
0x1409ce30b 488d95b0030000 lea rdx, [rbp + 0x3b0]
0x1409ce312 41b8f8000000 mov r8d, 0xf8
0x1409ce318 4889f9 mov rcx, rdi
0x1409ce31b e8e058f501 call 0x142923c00
0x1409ce320 48b86518a091ab399f57 movabs rax, 0x579f39ab91a01865
0x1409ce32a 488985e00a0000 mov qword ptr [rbp + 0xae0], rax
0x1409ce331 c785a005000002000000 mov dword ptr [rbp + 0x5a0], 2
0x1409ce33b 4889742420 mov qword ptr [rsp + 0x20], rsi
0x1409ce340 488d8de00a0000 lea rcx, [rbp + 0xae0]
0x1409ce347 49b88c97666903f55b81 movabs r8, 0x815bf5036966978c
0x1409ce351 4889fa mov rdx, rdi
0x1409ce354 41b11d mov r9b, 0x1d
0x1409ce357 e8b4a2f3ff call 0x140908610
0x1409ce35c 488dbd88090000 lea rdi, [rbp + 0x988]
0x1409ce363 488d95a8040000 lea rdx, [rbp + 0x4a8]
0x1409ce36a 41b8f8000000 mov r8d, 0xf8
0x1409ce370 4889f9 mov rcx, rdi
0x1409ce373 e88858f501 call 0x142923c00
0x1409ce378 488d85800a0000 lea rax, [rbp + 0xa80]
0x1409ce37f 488985e00a0000 mov qword ptr [rbp + 0xae0], rax
0x1409ce386 c785a005000002000000 mov dword ptr [rbp + 0x5a0], 2
0x1409ce390 4889742420 mov qword ptr [rsp + 0x20], rsi
0x1409ce395 488d8de00a0000 lea rcx, [rbp + 0xae0]
0x1409ce39c 49b8ddb340cc9756ba11 movabs r8, 0x11ba5697cc40b3dd
0x1409ce3a6 4889fa mov rdx, rdi
0x1409ce3a9 41b11d mov r9b, 0x1d
0x1409ce3ac e84f04f2ff call 0x1408ee800
0x1409ce3b1 488bb588090000 mov rsi, qword ptr [rbp + 0x988]
0x1409ce3b8 488d9590090000 lea rdx, [rbp + 0x990]
0x1409ce3bf 488d8da8050000 lea rcx, [rbp + 0x5a8]
0x1409ce3c6 41b8a0000000 mov r8d, 0xa0
0x1409ce3cc e82f58f501 call 0x142923c00
0x1409ce3d1 0fb685300a0000 movzx eax, byte ptr [rbp + 0xa30]
0x1409ce3d8 0fb78d310a0000 movzx ecx, word ptr [rbp + 0xa31]
0x1409ce3df 0fb695330a0000 movzx edx, byte ptr [rbp + 0xa33]
0x1409ce3e6 0f1085340a0000 movups xmm0, xmmword ptr [rbp + 0xa34]
0x1409ce3ed 0f11854c060000 movups xmmword ptr [rbp + 0x64c], xmm0
0x1409ce3f4 0f1085440a0000 movups xmm0, xmmword ptr [rbp + 0xa44]
0x1409ce3fb 0f11855c060000 movups xmmword ptr [rbp + 0x65c], xmm0
0x1409ce402 0f1085540a0000 movups xmm0, xmmword ptr [rbp + 0xa54]
0x1409ce409 0f11856c060000 movups xmmword ptr [rbp + 0x66c], xmm0
0x1409ce410 0f1085640a0000 movups xmm0, xmmword ptr [rbp + 0xa64]
0x1409ce417 0f11857c060000 movups xmmword ptr [rbp + 0x67c], xmm0
0x1409ce41e f30f6f85700a0000 movdqu xmm0, xmmword ptr [rbp + 0xa70]
0x1409ce426 f30f7f8588060000 movdqu xmmword ptr [rbp + 0x688], xmm0
0x1409ce42e 247f and al, 0x7f
0x1409ce430 80e29f and dl, 0x9f
0x1409ce433 80ca40 or dl, 0x40
0x1409ce436 4889b5a0050000 mov qword ptr [rbp + 0x5a0], rsi
0x1409ce43d 888548060000 mov byte ptr [rbp + 0x648], al
0x1409ce443 66898d49060000 mov word ptr [rbp + 0x649], cx
0x1409ce44a 88954b060000 mov byte ptr [rbp + 0x64b], dl
0x1409ce450 c685f70a000001 mov byte ptr [rbp + 0xaf7], 1
0x1409ce457 488d8da0050000 lea rcx, [rbp + 0x5a0]
0x1409ce45e ba03000000 mov edx, 3
0x1409ce463 e82886f3ff call 0x140906a90
0x1409ce468 488db5a0050000 lea rsi, [rbp + 0x5a0]
0x1409ce46f 488d55c8 lea rdx, [rbp - 0x38]
0x1409ce473 41b8e8030000 mov r8d, 0x3e8
0x1409ce479 4889f1 mov rcx, rsi
0x1409ce47c e87f57f501 call 0x142923c00
0x1409ce481 c685f70a000000 mov byte ptr [rbp + 0xaf7], 0
0x1409ce488 4889f1 mov rcx, rsi
0x1409ce48b e8e09ee9ff call 0x140868370
0x1409ce490 80bde80a000000 cmp byte ptr [rbp + 0xae8], 0
0x1409ce497 743f je 0x1409ce4d8
0x1409ce499 e8929c2400 call 0x140c18130
0x1409ce49e 4889c1 mov rcx, rax
0x1409ce4a1 488b00 mov rax, qword ptr [rax]
0x1409ce4a4 4883f801 cmp rax, 1
0x1409ce4a8 7563 jne 0x1409ce50d
0x1409ce4aa 4883c108 add rcx, 8
0x1409ce4ae 48833900 cmp qword ptr [rcx], 0
0x1409ce4b2 7579 jne 0x1409ce52d
0x1409ce4b4 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
0x1409ce4bb 48898de80a0000 mov qword ptr [rbp + 0xae8], rcx
0x1409ce4c2 4883c108 add rcx, 8
0x1409ce4c6 4c89fa mov rdx, r15
0x1409ce4c9 e8228c2400 call 0x140c170f0
0x1409ce4ce 488b85e80a0000 mov rax, qword ptr [rbp + 0xae8]
0x1409ce4d5 48ff00 inc qword ptr [rax]
0x1409ce4d8 0f28b5000b0000 movaps xmm6, xmmword ptr [rbp + 0xb00]
0x1409ce4df 4881c4980b0000 add rsp, 0xb98
0x1409ce4e6 5b pop rbx
0x1409ce4e7 5f pop rdi
0x1409ce4e8 5e pop rsi
0x1409ce4e9 415c pop r12
0x1409ce4eb 415d pop r13
0x1409ce4ed 415e pop r14
0x1409ce4ef 415f pop r15
0x1409ce4f1 5d pop rbp
0x1409ce4f2 c3 ret 
0x1409ce4f3 4885c0 test rax, rax
0x1409ce4f6 7529 jne 0x1409ce521
0x1409ce4f8 31d2 xor edx, edx
0x1409ce4fa e8d1a2f201 call 0x1428f87d0
0x1409ce4ff 4889c1 mov rcx, rax
0x1409ce502 4885c0 test rax, rax
0x1409ce505 0f85edfbffff jne 0x1409ce0f8
0x1409ce50b eb14 jmp 0x1409ce521
0x1409ce50d 4885c0 test rax, rax
0x1409ce510 750f jne 0x1409ce521
0x1409ce512 31d2 xor edx, edx
0x1409ce514 e8b7a2f201 call 0x1428f87d0
0x1409ce519 4889c1 mov rcx, rax
0x1409ce51c 4885c0 test rax, rax
0x1409ce51f 758d jne 0x1409ce4ae
0x1409ce521 488d0d78c52602 lea rcx, [rip + 0x226c578]
0x1409ce528 e8a39bf001 call 0x1428d80d0
0x1409ce52d 488d0d3cc62602 lea rcx, [rip + 0x226c63c]
0x1409ce534 e817adf001 call 0x1428d9250
0x1409ce539 c685f70a000001 mov byte ptr [rbp + 0xaf7], 1
0x1409ce540 488d0d59c52602 lea rcx, [rip + 0x226c559]
0x1409ce547 e8849bf001 call 0x1428d80d0
0x1409ce54c 0f0b ud2 
0x1409ce54e 6690 nop 
0x1409ce550 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409ce555 55 push rbp
0x1409ce556 4157 push r15
0x1409ce558 4156 push r14
0x1409ce55a 4155 push r13
0x1409ce55c 4154 push r12
0x1409ce55e 56 push rsi
0x1409ce55f 57 push rdi
0x1409ce560 53 push rbx
0x1409ce561 4883ec58 sub rsp, 0x58
0x1409ce565 488daa80000000 lea rbp, [rdx + 0x80]
0x1409ce56c 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409ce571 488b85d80a0000 mov rax, qword ptr [rbp + 0xad8]
0x1409ce578 48ff00 inc qword ptr [rax]
0x1409ce57b 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409ce580 4883c458 add rsp, 0x58
0x1409ce584 5b pop rbx
0x1409ce585 5f pop rdi
0x1409ce586 5e pop rsi
0x1409ce587 415c pop r12
0x1409ce589 415d pop r13
0x1409ce58b 415e pop r14
0x1409ce58d 415f pop r15
0x1409ce58f 5d pop rbp
0x1409ce590 c3 ret 
0x1409ce591 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409ce5a0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409ce5a5 55 push rbp
0x1409ce5a6 4157 push r15
0x1409ce5a8 4156 push r14
0x1409ce5aa 4155 push r13
0x1409ce5ac 4154 push r12
0x1409ce5ae 56 push rsi
0x1409ce5af 57 push rdi
0x1409ce5b0 53 push rbx
0x1409ce5b1 4883ec58 sub rsp, 0x58
0x1409ce5b5 488daa80000000 lea rbp, [rdx + 0x80]
0x1409ce5bc 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409ce5c1 80bde80a000000 cmp byte ptr [rbp + 0xae8], 0
0x1409ce5c8 742f je 0x1409ce5f9
0x1409ce5ca 488d85b80a0000 lea rax, [rbp + 0xab8]
0x1409ce5d1 488985c80a0000 mov qword ptr [rbp + 0xac8], rax
0x1409ce5d8 488d85c80a0000 lea rax, [rbp + 0xac8]
0x1409ce5df 488985c00a0000 mov qword ptr [rbp + 0xac0], rax
0x1409ce5e6 488d0db32e0f02 lea rcx, [rip + 0x20f2eb3]
0x1409ce5ed 488d95c00a0000 lea rdx, [rbp + 0xac0]
0x1409ce5f4 e8d70c9600 call 0x14132f2d0
0x1409ce5f9 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409ce5fe 4883c458 add rsp, 0x58
0x1409ce602 5b pop rbx
0x1409ce603 5f pop rdi
0x1409ce604 5e pop rsi
0x1409ce605 415c pop r12
0x1409ce607 415d pop r13
0x1409ce609 415e pop r14
0x1409ce60b 415f pop r15
0x1409ce60d 5d pop rbp
0x1409ce60e c3 ret 
0x1409ce60f 90 nop 
0x1409ce610 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409ce615 55 push rbp
0x1409ce616 4157 push r15
0x1409ce618 4156 push r14
0x1409ce61a 4155 push r13
0x1409ce61c 4154 push r12
0x1409ce61e 56 push rsi
0x1409ce61f 57 push rdi
0x1409ce620 53 push rbx
0x1409ce621 4883ec58 sub rsp, 0x58
0x1409ce625 488daa80000000 lea rbp, [rdx + 0x80]
0x1409ce62c 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409ce631 488b85e80a0000 mov rax, qword ptr [rbp + 0xae8]
0x1409ce638 48ff00 inc qword ptr [rax]
0x1409ce63b 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409ce640 4883c458 add rsp, 0x58
0x1409ce644 5b pop rbx
0x1409ce645 5f pop rdi
0x1409ce646 5e pop rsi
0x1409ce647 415c pop r12
0x1409ce649 415d pop r13
0x1409ce64b 415e pop r14
0x1409ce64d 415f pop r15
0x1409ce64f 5d pop rbp
0x1409ce650 c3 ret 
0x1409ce651 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409ce660 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409ce665 55 push rbp
0x1409ce666 4157 push r15
0x1409ce668 4156 push r14
0x1409ce66a 4155 push r13
0x1409ce66c 4154 push r12
0x1409ce66e 56 push rsi
0x1409ce66f 57 push rdi
0x1409ce670 53 push rbx
0x1409ce671 4883ec58 sub rsp, 0x58
0x1409ce675 488daa80000000 lea rbp, [rdx + 0x80]
0x1409ce67c 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409ce681 80bdf70a000000 cmp byte ptr [rbp + 0xaf7], 0
0x1409ce688 741f je 0x1409ce6a9
0x1409ce68a 488d4dc8 lea rcx, [rbp - 0x38]
0x1409ce68e e82d3decff call 0x1408923c0
0x1409ce693 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409ce698 4883c458 add rsp, 0x58
0x1409ce69c 5b pop rbx
0x1409ce69d 5f pop rdi
0x1409ce69e 5e pop rsi
0x1409ce69f 415c pop r12
0x1409ce6a1 415d pop r13
0x1409ce6a3 415e pop r14
0x1409ce6a5 415f pop r15
0x1409ce6a7 5d pop rbp
0x1409ce6a8 c3 ret 
0x1409ce6a9 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409ce6ae 4883c458 add rsp, 0x58
0x1409ce6b2 5b pop rbx
0x1409ce6b3 5f pop rdi
0x1409ce6b4 5e pop rsi
0x1409ce6b5 415c pop r12
0x1409ce6b7 415d pop r13
0x1409ce6b9 415e pop r14
0x1409ce6bb 415f pop r15
0x1409ce6bd 5d pop rbp
0x1409ce6be c3 ret 
0x1409ce6bf 90 nop 
0x1409ce6c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409ce6c5 55 push rbp
0x1409ce6c6 4157 push r15
0x1409ce6c8 4156 push r14
0x1409ce6ca 4155 push r13
0x1409ce6cc 4154 push r12
0x1409ce6ce 56 push rsi
0x1409ce6cf 57 push rdi
0x1409ce6d0 53 push rbx
0x1409ce6d1 4883ec58 sub rsp, 0x58
0x1409ce6d5 488daa80000000 lea rbp, [rdx + 0x80]
0x1409ce6dc 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409ce6e1 c685f70a000001 mov byte ptr [rbp + 0xaf7], 1
0x1409ce6e8 488d8da8040000 lea rcx, [rbp + 0x4a8]
0x1409ce6ef e8cc4eecff call 0x1408935c0
0x1409ce6f4 c685f70a000001 mov byte ptr [rbp + 0xaf7], 1
0x1409ce6fb 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409ce700 4883c458 add rsp, 0x58
0x1409ce704 5b pop rbx
0x1409ce705 5f pop rdi
0x1409ce706 5e pop rsi
0x1409ce707 415c pop r12
0x1409ce709 415d pop r13
0x1409ce70b 415e pop r14
0x1409ce70d 415f pop r15
0x1409ce70f 5d pop rbp
0x1409ce710 c3 ret 
0x1409ce711 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409ce720 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409ce725 55 push rbp
0x1409ce726 4157 push r15
0x1409ce728 4156 push r14
0x1409ce72a 4155 push r13
0x1409ce72c 4154 push r12
0x1409ce72e 56 push rsi
0x1409ce72f 57 push rdi
0x1409ce730 53 push rbx
0x1409ce731 4883ec58 sub rsp, 0x58
0x1409ce735 488daa80000000 lea rbp, [rdx + 0x80]
0x1409ce73c 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
0x1409ce741 c685f70a000001 mov byte ptr [rbp + 0xaf7], 1
0x1409ce748 488d8d88090000 lea rcx, [rbp + 0x988]
0x1409ce74f e86c4eecff call 0x1408935c0
0x1409ce754 c685f70a000001 mov byte ptr [rbp + 0xaf7], 1
0x1409ce75b 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1409ce760 4883c458 add rsp, 0x58
0x1409ce764 5b pop rbx
0x1409ce765 5f pop rdi
0x1409ce766 5e pop rsi
0x1409ce767 415c pop r12
0x1409ce769 415d pop r13
0x1409ce76b 415e pop r14
0x1409ce76d 415f pop r15
0x1409ce76f 5d pop rbp
0x1409ce770 c3 ret 
