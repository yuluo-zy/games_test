0x1400bdc80 push     rbp
0x1400bdc81 push     r15
0x1400bdc83 push     r14
0x1400bdc85 push     r12
0x1400bdc87 push     rsi
0x1400bdc88 push     rdi
0x1400bdc89 push     rbx
0x1400bdc8a sub      rsp, 0x360
0x1400bdc91 lea      rbp, [rsp + 0x80]
0x1400bdc99 mov      qword ptr [rbp + 0x2d8], 0xfffffffffffffffe
0x1400bdca4 mov      rsi, rdx
0x1400bdca7 mov      r14, qword ptr [rcx]
0x1400bdcaa mov      ebx, dword ptr [r14 + 0x1c]
0x1400bdcae mov      r15, qword ptr [r14 + 0x10]
0x1400bdcb2 mov      dword ptr [r15], ebx
0x1400bdcb5 mov      rdi, qword ptr [r14]
0x1400bdcb8 movzx    eax, byte ptr [rip + 0x3576f4a]
0x1400bdcbf mov      ecx, 0x1a
0x1400bdcc4 mov      edx, 1
0x1400bdcc9 call     0x140613c10
0x1400bdcce test     rax, rax
0x1400bdcd1 je       0x1400be88e
0x1400bdcd7 lea      rcx, [rdi + 0xf8]
0x1400bdcde movups   xmm0, xmmword ptr [rip + 0x287782d] ; @0x142935512 'an scale ref (R)Show debug threshold curveShow debug spatial gridShow debug raycast geoShow debu'
0x1400bdce5 movups   xmmword ptr [rax + 0xa], xmm0
0x1400bdce9 movups   xmm0, xmmword ptr [rip + 0x2877818] ; @0x142935508 'Enable human scale ref (R)Show debug threshold curveShow debug spatial gridShow debug raycast ge'
0x1400bdcf0 movups   xmmword ptr [rax], xmm0
0x1400bdcf3 mov      qword ptr [rbp + 0x1a0], 0x1a
0x1400bdcfe mov      qword ptr [rbp + 0x1a8], rax
0x1400bdd05 mov      qword ptr [rbp + 0x1b0], 0x1a
0x1400bdd10 mov      dword ptr [rbp + 0x1b8], 0
0x1400bdd1a mov      qword ptr [rbp + 0x1c8], 0
0x1400bdd25 mov      dword ptr [rbp + 0x1c0], 0
0x1400bdd2f mov      qword ptr [rbp + 0x170], 3
0x1400bdd3a mov      qword ptr [rbp + 0x188], 6
0x1400bdd45 mov      byte ptr [rbp + 0x1d0], 0
0x1400bdd4c mov      dword ptr [rbp + 0x1d5], 0
0x1400bdd56 mov      dword ptr [rbp + 0x1d8], 0
0x1400bdd60 mov      qword ptr [rbp + 0x1e0], rcx
0x1400bdd67 mov      byte ptr [rbp + 0x1e8], 0
0x1400bdd6e lea      rcx, [rbp - 0x50]
0x1400bdd72 lea      rdx, [rbp + 0x170]
0x1400bdd79 mov      r8, rsi
0x1400bdd7c call     0x140c48930
0x1400bdd81 mov      rax, qword ptr [rbp - 0x40]
0x1400bdd85 lock dec qword ptr [rax]
0x1400bdd89 jne      0x1400bdd94
0x1400bdd8b lea      rcx, [rbp - 0x40]
0x1400bdd8f call     0x140c41170
0x1400bdd94 mov      dword ptr [r15], ebx
0x1400bdd97 movzx    eax, byte ptr [rip + 0x3576e6b]
0x1400bdd9e mov      ecx, 0x1a
0x1400bdda3 mov      edx, 1
0x1400bdda8 call     0x140613c10
0x1400bddad test     rax, rax
0x1400bddb0 je       0x1400be88e
0x1400bddb6 lea      rcx, [rdi + 0xf3]
0x1400bddbd movups   xmm0, xmmword ptr [rip + 0x2877768] ; @0x14293552c ' threshold curveShow debug spatial gridShow debug raycast geoShow debug trigger proxiesViz prese'
0x1400bddc4 movups   xmmword ptr [rax + 0xa], xmm0
0x1400bddc8 movups   xmm0, xmmword ptr [rip + 0x2877753] ; @0x142935522 'Show debug threshold curveShow debug spatial gridShow debug raycast geoShow debug trigger proxie'
0x1400bddcf movups   xmmword ptr [rax], xmm0
0x1400bddd2 mov      qword ptr [rbp + 0x1a0], 0x1a
0x1400bdddd mov      qword ptr [rbp + 0x1a8], rax
0x1400bdde4 mov      qword ptr [rbp + 0x1b0], 0x1a
0x1400bddef mov      dword ptr [rbp + 0x1b8], 0
0x1400bddf9 mov      qword ptr [rbp + 0x1c8], 0
0x1400bde04 mov      dword ptr [rbp + 0x1c0], 0
0x1400bde0e mov      qword ptr [rbp + 0x170], 3
0x1400bde19 mov      qword ptr [rbp + 0x188], 6
0x1400bde24 mov      byte ptr [rbp + 0x1d0], 0
0x1400bde2b mov      dword ptr [rbp + 0x1d5], 0
0x1400bde35 mov      dword ptr [rbp + 0x1d8], 0
0x1400bde3f mov      qword ptr [rbp + 0x1e0], rcx
0x1400bde46 mov      byte ptr [rbp + 0x1e8], 0
0x1400bde4d lea      rcx, [rbp - 0x50]
0x1400bde51 lea      rdx, [rbp + 0x170]
0x1400bde58 mov      r8, rsi
0x1400bde5b call     0x140c48930
0x1400bde60 mov      rax, qword ptr [rbp - 0x40]
0x1400bde64 lock dec qword ptr [rax]
0x1400bde68 jne      0x1400bde73
0x1400bde6a lea      rcx, [rbp - 0x40]
0x1400bde6e call     0x140c41170
0x1400bde73 mov      dword ptr [r15], ebx
0x1400bde76 movzx    eax, byte ptr [rip + 0x3576d8c]
0x1400bde7d mov      ecx, 0x17
0x1400bde82 mov      edx, 1
0x1400bde87 call     0x140613c10
0x1400bde8c test     rax, rax
0x1400bde8f je       0x1400be8a4
0x1400bde95 lea      rcx, [rdi + 0xf1]
0x1400bde9c movups   xmm0, xmmword ptr [rip + 0x2877699] ; @0x14293553c 'Show debug spatial gridShow debug raycast geoShow debug trigger proxiesViz presets'
0x1400bdea3 movups   xmmword ptr [rax], xmm0
0x1400bdea6 movabs   rdx, 0x64697267206c6169
0x1400bdeb0 mov      qword ptr [rax + 0xf], rdx
0x1400bdeb4 mov      qword ptr [rbp + 0x1a0], 0x17
0x1400bdebf mov      qword ptr [rbp + 0x1a8], rax
0x1400bdec6 mov      qword ptr [rbp + 0x1b0], 0x17
0x1400bded1 mov      dword ptr [rbp + 0x1b8], 0
0x1400bdedb mov      qword ptr [rbp + 0x1c8], 0
0x1400bdee6 mov      dword ptr [rbp + 0x1c0], 0
0x1400bdef0 mov      qword ptr [rbp + 0x170], 3
0x1400bdefb mov      qword ptr [rbp + 0x188], 6
0x1400bdf06 mov      byte ptr [rbp + 0x1d0], 0
0x1400bdf0d mov      dword ptr [rbp + 0x1d5], 0
0x1400bdf17 mov      dword ptr [rbp + 0x1d8], 0
0x1400bdf21 mov      qword ptr [rbp + 0x1e0], rcx
0x1400bdf28 mov      byte ptr [rbp + 0x1e8], 0
0x1400bdf2f lea      rcx, [rbp - 0x50]
0x1400bdf33 lea      rdx, [rbp + 0x170]
0x1400bdf3a mov      r8, rsi
0x1400bdf3d call     0x140c48930
0x1400bdf42 mov      rax, qword ptr [rbp - 0x40]
0x1400bdf46 lock dec qword ptr [rax]
0x1400bdf4a jne      0x1400bdf55
0x1400bdf4c lea      rcx, [rbp - 0x40]
0x1400bdf50 call     0x140c41170
0x1400bdf55 lea      rdx, [rip + 0x28775f7] ; @0x142935553 'Show debug raycast geoShow debug trigger proxiesViz presets'
0x1400bdf5c lea      rcx, [rbp + 0x170]
0x1400bdf63 mov      r8d, 0x16
0x1400bdf69 call     0x1400ff0b0
0x1400bdf6e add      rdi, 0x104
0x1400bdf75 mov      byte ptr [rbp + 0x2d7], 1
0x1400bdf7c mov      rcx, rdi
0x1400bdf7f call     0x140940170
0x1400bdf84 mov      r15, rdx
0x1400bdf87 test     rdx, rdx
0x1400bdf8a js       0x1400be8ba
0x1400bdf90 mov      rdi, rax
0x1400bdf93 test     r15, r15
0x1400bdf96 je       0x1400bdfbf
0x1400bdf98 movzx    eax, byte ptr [rip + 0x3576c6a]
0x1400bdf9f mov      ebx, 1
0x1400bdfa4 mov      edx, 1
0x1400bdfa9 mov      rcx, r15
0x1400bdfac call     0x140613c10
0x1400bdfb1 test     rax, rax
0x1400bdfb4 je       0x1400be8bc
0x1400bdfba mov      r12, rax
0x1400bdfbd jmp      0x1400bdfc5
0x1400bdfbf mov      r12d, 1
0x1400bdfc5 mov      rcx, r12
0x1400bdfc8 mov      rdx, rdi
0x1400bdfcb mov      r8, r15
0x1400bdfce call     0x142923c00
0x1400bdfd3 mov      dword ptr [rbp + 0x2c1], 0
0x1400bdfdd mov      dword ptr [rbp + 0x2c4], 0
0x1400bdfe7 mov      rdi, qword ptr [rbp + 0x170]
0x1400bdfee mov      ecx, edi
0x1400bdff0 and      ecx, 0xfffffffe
0x1400bdff3 lea      rdx, [rdi - 3]
0x1400bdff7 xor      eax, eax
0x1400bdff9 cmp      ecx, 4
0x1400bdffc cmove    rax, rdx
0x1400be000 test     rax, rax
0x1400be003 mov      qword ptr [rbp + 0x2c8], r15
0x1400be00a mov      qword ptr [rbp + 0x168], r12
0x1400be011 je       0x1400be04c
0x1400be013 cmp      rax, 1
0x1400be017 jne      0x1400be0ba
0x1400be01d mov      rdx, qword ptr [rbp + 0x178]
0x1400be024 test     rdx, rdx
0x1400be027 je       0x1400be03b
0x1400be029 mov      rcx, qword ptr [rbp + 0x180]
0x1400be030 mov      r8d, 1
0x1400be036 call     0x140613c20
0x1400be03b lea      rcx, [rbp + 0x190]
0x1400be042 call     0x140608f80
0x1400be047 jmp      0x1400be0d3
0x1400be04c mov      rdx, qword ptr [rbp + 0x1a0]
0x1400be053 test     rdx, rdx
0x1400be056 je       0x1400be06a
0x1400be058 mov      rcx, qword ptr [rbp + 0x1a8]
0x1400be05f mov      r8d, 1
0x1400be065 call     0x140613c20
0x1400be06a cmp      edi, 3
0x1400be06d je       0x1400be08d
0x1400be06f cmp      edi, 2
0x1400be072 jb       0x1400be08d
0x1400be074 mov      rax, qword ptr [rbp + 0x178]
0x1400be07b lock dec qword ptr [rax]
0x1400be07f jne      0x1400be08d
0x1400be081 lea      rcx, [rbp + 0x178]
0x1400be088 call     0x14064eb30
0x1400be08d mov      rax, qword ptr [rbp + 0x188]
0x1400be094 cmp      rax, 6
0x1400be098 je       0x1400be0d3
0x1400be09a cmp      eax, 5
0x1400be09d jb       0x1400be0d3
0x1400be09f mov      rax, qword ptr [rbp + 0x190]
0x1400be0a6 lock dec qword ptr [rax]
0x1400be0aa jne      0x1400be0d3
0x1400be0ac lea      rcx, [rbp + 0x190]
0x1400be0b3 call     0x14064eb30
0x1400be0b8 jmp      0x1400be0d3
0x1400be0ba mov      rax, qword ptr [rbp + 0x178]
0x1400be0c1 lock dec qword ptr [rax]
0x1400be0c5 jne      0x1400be0d3
0x1400be0c7 lea      rcx, [rbp + 0x178]
0x1400be0ce call     0x140c7de90
0x1400be0d3 mov      qword ptr [rbp + 0x170], 3
0x1400be0de movups   xmm0, xmmword ptr [rbp - 0x50]
0x1400be0e2 movups   xmmword ptr [rbp + 0x178], xmm0
0x1400be0e9 mov      qword ptr [rbp + 0x188], 6
0x1400be0f4 movups   xmm0, xmmword ptr [rbp + 0xf8]
0x1400be0fb movups   xmmword ptr [rbp + 0x190], xmm0
0x1400be102 mov      rax, qword ptr [rbp + 0x2c8]
0x1400be109 mov      qword ptr [rbp + 0x1a0], rax
0x1400be110 mov      rcx, qword ptr [rbp + 0x168]
0x1400be117 mov      qword ptr [rbp + 0x1a8], rcx
0x1400be11e mov      qword ptr [rbp + 0x1b0], rax
0x1400be125 mov      dword ptr [rbp + 0x1b8], 0
0x1400be12f mov      dword ptr [rbp + 0x1c0], 0
0x1400be139 mov      qword ptr [rbp + 0x1c8], 0
0x1400be144 mov      byte ptr [rbp + 0x1d0], 0
0x1400be14b mov      rax, qword ptr [rbp + 0x2bd]
0x1400be152 mov      qword ptr [rbp + 0x1d1], rax
0x1400be159 mov      eax, dword ptr [rbp + 0x2c4]
0x1400be15f mov      dword ptr [rbp + 0x1d8], eax
0x1400be165 lea      rdi, [rbp - 0x50]
0x1400be169 lea      rdx, [rbp + 0x170]
0x1400be170 mov      r8d, 0x110
0x1400be176 mov      rcx, rdi
0x1400be179 call     0x142923c00
0x1400be17e lea      rcx, [rbp + 0x170]
0x1400be185 mov      r8d, 0x110
0x1400be18b mov      rdx, rdi
0x1400be18e call     0x142923c00
0x1400be193 movzx    eax, byte ptr [rip + 0x3576a6f]
0x1400be19a mov      ecx, 8
0x1400be19f mov      edx, 8
0x1400be1a4 call     0x140613c10
0x1400be1a9 test     rax, rax
0x1400be1ac mov      qword ptr [rbp + 0x2c8], rax
0x1400be1b3 je       0x1400be8d0
0x1400be1b9 mov      r9, rax
0x1400be1bc mov      qword ptr [rax], r14
0x1400be1bf lea      rax, [rip + 0x288ea52]
0x1400be1c6 mov      qword ptr [rsp + 0x20], rax
0x1400be1cb lea      rcx, [rbp + 0xf8]
0x1400be1d2 lea      rdx, [rbp - 0x50]
0x1400be1d6 mov      r8, rsi
0x1400be1d9 call     0x1400ff3e0
0x1400be1de mov      rax, qword ptr [rbp + 0x108]
0x1400be1e5 lock dec qword ptr [rax]
0x1400be1e9 jne      0x1400be1f7
0x1400be1eb lea      rcx, [rbp + 0x108]
0x1400be1f2 call     0x140c41170
0x1400be1f7 mov      eax, dword ptr [r14 + 0x1c]
0x1400be1fb mov      rcx, qword ptr [r14 + 0x10]
0x1400be1ff mov      dword ptr [rcx], eax
0x1400be201 mov      rdi, qword ptr [r14]
0x1400be204 movzx    eax, byte ptr [rip + 0x35769fe]
0x1400be20b mov      ecx, 0x1a
0x1400be210 mov      edx, 1
0x1400be215 call     0x140613c10
0x1400be21a test     rax, rax
0x1400be21d je       0x1400be88e
0x1400be223 add      rdi, 0xf2
0x1400be22a movups   xmm0, xmmword ptr [rip + 0x2877342] ; @0x142935573 ' trigger proxiesViz presets'
0x1400be231 movups   xmmword ptr [rax + 0xa], xmm0
0x1400be235 movups   xmm0, xmmword ptr [rip + 0x287732d] ; @0x142935569 'Show debug trigger proxiesViz presets'
0x1400be23c movups   xmmword ptr [rax], xmm0
0x1400be23f mov      qword ptr [rbp + 0x1a0], 0x1a
0x1400be24a mov      qword ptr [rbp + 0x1a8], rax
0x1400be251 mov      qword ptr [rbp + 0x1b0], 0x1a
0x1400be25c mov      dword ptr [rbp + 0x1b8], 0
0x1400be266 mov      qword ptr [rbp + 0x1c8], 0
0x1400be271 mov      dword ptr [rbp + 0x1c0], 0
0x1400be27b mov      qword ptr [rbp + 0x170], 3
0x1400be286 mov      qword ptr [rbp + 0x188], 6
0x1400be291 mov      byte ptr [rbp + 0x1d0], 0
0x1400be298 mov      dword ptr [rbp + 0x1d5], 0
0x1400be2a2 mov      dword ptr [rbp + 0x1d8], 0
0x1400be2ac mov      qword ptr [rbp + 0x1e0], rdi
0x1400be2b3 mov      byte ptr [rbp + 0x1e8], 0
0x1400be2ba lea      rcx, [rbp - 0x50]
0x1400be2be lea      rdx, [rbp + 0x170]
0x1400be2c5 mov      r8, rsi
0x1400be2c8 call     0x140c48930
0x1400be2cd mov      rax, qword ptr [rbp - 0x40]
0x1400be2d1 lock dec qword ptr [rax]
0x1400be2d5 jne      0x1400be2e0
0x1400be2d7 lea      rcx, [rbp - 0x40]
0x1400be2db call     0x140c41170
0x1400be2e0 mov      eax, dword ptr [r14 + 0x1c]
0x1400be2e4 mov      rcx, qword ptr [r14 + 0x10]
0x1400be2e8 mov      dword ptr [rcx], eax
0x1400be2ea mov      rdi, qword ptr [r14]
0x1400be2ed movzx    eax, byte ptr [rip + 0x3576915]
0x1400be2f4 mov      ecx, 0xe
0x1400be2f9 mov      edx, 1
0x1400be2fe call     0x140613c10
0x1400be303 test     rax, rax
0x1400be306 je       0x1400be8e1
0x1400be30c add      rdi, 0xf4
0x1400be313 movabs   rcx, 0x6469726720797a61
0x1400be31d mov      qword ptr [rax + 6], rcx
0x1400be321 movabs   rcx, 0x7a616c20776f6853
0x1400be32b mov      qword ptr [rax], rcx
0x1400be32e mov      qword ptr [rbp + 0x1a0], 0xe
0x1400be339 mov      qword ptr [rbp + 0x1a8], rax
0x1400be340 mov      qword ptr [rbp + 0x1b0], 0xe
0x1400be34b mov      dword ptr [rbp + 0x1b8], 0
0x1400be355 mov      qword ptr [rbp + 0x1c8], 0
0x1400be360 mov      dword ptr [rbp + 0x1c0], 0
0x1400be36a mov      qword ptr [rbp + 0x170], 3
0x1400be375 mov      qword ptr [rbp + 0x188], 6
0x1400be380 mov      byte ptr [rbp + 0x1d0], 0
0x1400be387 mov      dword ptr [rbp + 0x1d5], 0
0x1400be391 mov      dword ptr [rbp + 0x1d8], 0
0x1400be39b mov      qword ptr [rbp + 0x1e0], rdi
0x1400be3a2 mov      byte ptr [rbp + 0x1e8], 0
0x1400be3a9 lea      rcx, [rbp - 0x50]
0x1400be3ad lea      rdx, [rbp + 0x170]
0x1400be3b4 mov      r8, rsi
0x1400be3b7 call     0x140c48930
0x1400be3bc mov      rax, qword ptr [rbp - 0x40]
0x1400be3c0 lock dec qword ptr [rax]
0x1400be3c4 jne      0x1400be3cf
0x1400be3c6 lea      rcx, [rbp - 0x40]
0x1400be3ca call     0x140c41170
0x1400be3cf mov      rax, qword ptr [r14]
0x1400be3d2 cmp      byte ptr [rax + 0xf2], 0
0x1400be3d9 je       0x1400be87b
0x1400be3df lea      rdx, [rip + 0x287719d] ; @0x142935583 'Viz presets'
0x1400be3e6 lea      rcx, [rbp + 0x170]
0x1400be3ed mov      r8d, 0xb
0x1400be3f3 call     0x1400ff0b0
0x1400be3f8 mov      edx, 0x102
0x1400be3fd add      rdx, qword ptr [r14]
0x1400be400 mov      byte ptr [rbp + 0x2d6], 1
0x1400be407 lea      rcx, [rbp + 0xf8]
0x1400be40e call     0x14093ffd0
0x1400be413 mov      byte ptr [rbp + 0x2d6], 0
0x1400be41a lea      rcx, [rbp - 0x50]
0x1400be41e lea      rdx, [rbp + 0x170]
0x1400be425 lea      r8, [rbp + 0xf8]
0x1400be42c call     0x1400c0fd0
0x1400be431 lea      rcx, [rbp + 0x170]
0x1400be438 lea      rdx, [rbp - 0x50]
0x1400be43c mov      r8d, 0x110
0x1400be442 call     0x142923c00
0x1400be447 movzx    eax, byte ptr [rip + 0x35767bb]
0x1400be44e mov      ecx, 8
0x1400be453 mov      edx, 8
0x1400be458 call     0x140613c10
0x1400be45d test     rax, rax
0x1400be460 mov      qword ptr [rbp + 0x2c8], rax
0x1400be467 je       0x1400be8f7
0x1400be46d mov      r9, rax
0x1400be470 mov      qword ptr [rax], r14
0x1400be473 lea      rax, [rip + 0x288e7de]
0x1400be47a mov      qword ptr [rsp + 0x20], rax
0x1400be47f lea      rcx, [rbp + 0xf8]
0x1400be486 lea      rdx, [rbp - 0x50]
0x1400be48a mov      r8, rsi
0x1400be48d call     0x1400ff3e0
0x1400be492 mov      rax, qword ptr [rbp + 0x108]
0x1400be499 lock dec qword ptr [rax]
0x1400be49d jne      0x1400be4ab
0x1400be49f lea      rcx, [rbp + 0x108]
0x1400be4a6 call     0x140c41170
0x1400be4ab mov      rdi, qword ptr [r14]
0x1400be4ae cmp      byte ptr [rdi + 0x102], 3
0x1400be4b5 jne      0x1400be87b
0x1400be4bb mov      eax, dword ptr [r14 + 0x1c]
0x1400be4bf mov      rcx, qword ptr [r14 + 0x10]
0x1400be4c3 mov      dword ptr [rcx], eax
0x1400be4c5 movzx    eax, byte ptr [rip + 0x357673d]
0x1400be4cc mov      ecx, 8
0x1400be4d1 mov      edx, 8
0x1400be4d6 call     0x140613c10
0x1400be4db test     rax, rax
0x1400be4de je       0x1400be908
0x1400be4e4 add      rdi, 0xe8
0x1400be4eb mov      qword ptr [rbp + 0x2c8], rax
0x1400be4f2 mov      qword ptr [rax], rdi
0x1400be4f5 lea      rcx, [rbp + 0xf8]
0x1400be4fc call     0x140c34750
0x1400be501 mov      qword ptr [rbp + 0x210], 0
0x1400be50c mov      qword ptr [rbp + 0x218], 1
0x1400be517 xorps    xmm0, xmm0
0x1400be51a movups   xmmword ptr [rbp + 0x220], xmm0
0x1400be521 mov      qword ptr [rbp + 0x230], 1
0x1400be52c mov      qword ptr [rbp + 0x238], 0
0x1400be537 movups   xmm1, xmmword ptr [rbp + 0xf8]
0x1400be53e movups   xmm2, xmmword ptr [rbp + 0x108]
0x1400be545 movups   xmm3, xmmword ptr [rbp + 0x118]
0x1400be54c movups   xmm4, xmmword ptr [rbp + 0x128]
0x1400be553 movups   xmmword ptr [rbp - 0x48], xmm1
0x1400be557 movups   xmmword ptr [rbp - 0x38], xmm2
0x1400be55b movups   xmmword ptr [rbp - 0x28], xmm3
0x1400be55f movups   xmmword ptr [rbp - 0x18], xmm4
0x1400be563 movups   xmm1, xmmword ptr [rbp + 0x138]
0x1400be56a movups   xmmword ptr [rbp - 8], xmm1
0x1400be56e movups   xmm1, xmmword ptr [rbp + 0x148]
0x1400be575 movups   xmmword ptr [rbp + 8], xmm1
0x1400be579 movups   xmm1, xmmword ptr [rbp + 0x158]
0x1400be580 movups   xmmword ptr [rbp + 0x18], xmm1
0x1400be584 movups   xmmword ptr [rbp + 0x288], xmm0
0x1400be58b mov      qword ptr [rbp + 0x170], 0
0x1400be596 mov      qword ptr [rbp + 0x180], 0
0x1400be5a1 mov      qword ptr [rbp + 0x190], 0
0x1400be5ac movups   xmm0, xmmword ptr [rbp - 0x50]
0x1400be5b0 movups   xmm1, xmmword ptr [rbp - 0x40]
0x1400be5b4 movups   xmm2, xmmword ptr [rbp - 0x30]
0x1400be5b8 movups   xmm3, xmmword ptr [rbp - 0x20]
0x1400be5bc movups   xmmword ptr [rbp + 0x198], xmm0
0x1400be5c3 movups   xmmword ptr [rbp + 0x1a8], xmm1
0x1400be5ca movups   xmmword ptr [rbp + 0x1b8], xmm2
0x1400be5d1 movups   xmmword ptr [rbp + 0x1c8], xmm3
0x1400be5d8 movups   xmm0, xmmword ptr [rbp - 0x10]
0x1400be5dc movups   xmmword ptr [rbp + 0x1d8], xmm0
0x1400be5e3 movups   xmm0, xmmword ptr [rbp]
0x1400be5e7 movups   xmmword ptr [rbp + 0x1e8], xmm0
0x1400be5ee movups   xmm0, xmmword ptr [rbp + 0x10]
0x1400be5f2 movups   xmmword ptr [rbp + 0x1f8], xmm0
0x1400be5f9 mov      rax, qword ptr [rbp + 0x20]
0x1400be5fd mov      qword ptr [rbp + 0x208], rax
0x1400be604 mov      dword ptr [rbp + 0x240], 2
0x1400be60e movaps   xmm0, xmmword ptr [rip + 0x287606b]
0x1400be615 movups   xmmword ptr [rbp + 0x248], xmm0
0x1400be61c mov      byte ptr [rbp + 0x258], 0
0x1400be623 movaps   xmm0, xmmword ptr [rip + 0x2875fa6]
0x1400be62a movups   xmmword ptr [rbp + 0x260], xmm0
0x1400be631 mov      byte ptr [rbp + 0x270], 0
0x1400be638 mov      rax, qword ptr [rbp + 0x2c8]
0x1400be63f mov      qword ptr [rbp + 0x278], rax
0x1400be646 lea      rax, [rip + 0x2943a4b]
0x1400be64d mov      qword ptr [rbp + 0x280], rax
0x1400be654 mov      qword ptr [rbp + 0x2a0], 0
0x1400be65f mov      dword ptr [rbp + 0x2b0], 0x10101
0x1400be669 mov      byte ptr [rbp + 0x2b4], 2
0x1400be670 movzx    eax, byte ptr [rip + 0x3576592]
0x1400be677 mov      ecx, 0xd
0x1400be67c mov      edx, 1
0x1400be681 call     0x140613c10
0x1400be686 test     rax, rax
0x1400be689 je       0x1400be917
0x1400be68f movabs   rcx, 0x6874676e656c206c
0x1400be699 mov      qword ptr [rax + 5], rcx
0x1400be69d movabs   rcx, 0x6c206c616d726f4e
0x1400be6a7 mov      qword ptr [rax], rcx
0x1400be6aa mov      dword ptr [rbp + 0x2c4], 0
0x1400be6b4 mov      dword ptr [rbp + 0x2c1], 0
0x1400be6be mov      rdi, qword ptr [rbp + 0x1a0]
0x1400be6c5 mov      ecx, edi
0x1400be6c7 and      ecx, 0xfffffffe
0x1400be6ca lea      rdx, [rdi - 3]
0x1400be6ce xor      r8d, r8d
0x1400be6d1 cmp      ecx, 4
0x1400be6d4 cmove    r8, rdx
0x1400be6d8 test     r8, r8
0x1400be6db mov      qword ptr [rbp + 0x2c8], rax
0x1400be6e2 je       0x1400be71d
0x1400be6e4 cmp      r8, 1
0x1400be6e8 jne      0x1400be78b
0x1400be6ee mov      rdx, qword ptr [rbp + 0x1a8]
0x1400be6f5 test     rdx, rdx
0x1400be6f8 je       0x1400be70c
0x1400be6fa mov      rcx, qword ptr [rbp + 0x1b0]
0x1400be701 mov      r8d, 1
0x1400be707 call     0x140613c20
0x1400be70c lea      rcx, [rbp + 0x1c0]
0x1400be713 call     0x140608f80
0x1400be718 jmp      0x1400be7a4
0x1400be71d mov      rdx, qword ptr [rbp + 0x1d0]
0x1400be724 test     rdx, rdx
0x1400be727 je       0x1400be73b
0x1400be729 mov      rcx, qword ptr [rbp + 0x1d8]
0x1400be730 mov      r8d, 1
0x1400be736 call     0x140613c20
0x1400be73b cmp      edi, 3
0x1400be73e je       0x1400be75e
0x1400be740 cmp      edi, 2
0x1400be743 jb       0x1400be75e
0x1400be745 mov      rax, qword ptr [rbp + 0x1a8]
0x1400be74c lock dec qword ptr [rax]
0x1400be750 jne      0x1400be75e
0x1400be752 lea      rcx, [rbp + 0x1a8]
0x1400be759 call     0x14064eb30
0x1400be75e mov      rax, qword ptr [rbp + 0x1b8]
0x1400be765 cmp      rax, 6
0x1400be769 je       0x1400be7a4
0x1400be76b cmp      eax, 5
0x1400be76e jb       0x1400be7a4
0x1400be770 mov      rax, qword ptr [rbp + 0x1c0]
0x1400be777 lock dec qword ptr [rax]
0x1400be77b jne      0x1400be7a4
0x1400be77d lea      rcx, [rbp + 0x1c0]
0x1400be784 call     0x14064eb30
0x1400be789 jmp      0x1400be7a4
0x1400be78b mov      rax, qword ptr [rbp + 0x1a8]
0x1400be792 lock dec qword ptr [rax]
0x1400be796 jne      0x1400be7a4
0x1400be798 lea      rcx, [rbp + 0x1a8]
0x1400be79f call     0x140c7de90
0x1400be7a4 mov      qword ptr [rbp + 0x1a0], 3
0x1400be7af movups   xmm0, xmmword ptr [rbp - 0x50]
0x1400be7b3 movups   xmmword ptr [rbp + 0x1a8], xmm0
0x1400be7ba mov      qword ptr [rbp + 0x1b8], 6
0x1400be7c5 movups   xmm0, xmmword ptr [rbp + 0xf8]
0x1400be7cc movups   xmmword ptr [rbp + 0x1c0], xmm0
0x1400be7d3 mov      qword ptr [rbp + 0x1d0], 0xd
0x1400be7de mov      rax, qword ptr [rbp + 0x2c8]
0x1400be7e5 mov      qword ptr [rbp + 0x1d8], rax
0x1400be7ec mov      qword ptr [rbp + 0x1e0], 0xd
0x1400be7f7 mov      dword ptr [rbp + 0x1e8], 0
0x1400be801 mov      dword ptr [rbp + 0x1f0], 0
0x1400be80b mov      qword ptr [rbp + 0x1f8], 0
0x1400be816 mov      byte ptr [rbp + 0x200], 0
0x1400be81d mov      rax, qword ptr [rbp + 0x2bd]
0x1400be824 mov      qword ptr [rbp + 0x201], rax
0x1400be82b mov      eax, dword ptr [rbp + 0x2c4]
0x1400be831 mov      dword ptr [rbp + 0x208], eax
0x1400be837 lea      rdi, [rbp - 0x50]
0x1400be83b lea      rdx, [rbp + 0x170]
0x1400be842 mov      r8d, 0x148
0x1400be848 mov      rcx, rdi
0x1400be84b call     0x142923c00
0x1400be850 lea      rcx, [rbp + 0x170]
0x1400be857 mov      rdx, rdi
0x1400be85a mov      r8, rsi
0x1400be85d call     0x140c77160
0x1400be862 mov      rax, qword ptr [rbp + 0x180]
0x1400be869 lock dec qword ptr [rax]
0x1400be86d jne      0x1400be87b
0x1400be86f lea      rcx, [rbp + 0x180]
0x1400be876 call     0x140c41170
0x1400be87b nop      
0x1400be87c add      rsp, 0x360
0x1400be883 pop      rbx
0x1400be884 pop      rdi
0x1400be885 pop      rsi
0x1400be886 pop      r12
0x1400be888 pop      r14
0x1400be88a pop      r15
0x1400be88c pop      rbp
0x1400be88d ret      
0x1400be88e lea      r8, [rip + 0x28764b3]
0x1400be895 mov      ecx, 1
0x1400be89a mov      edx, 0x1a
0x1400be89f call     0x1428d8fe3
0x1400be8a4 lea      r8, [rip + 0x287649d]
0x1400be8ab mov      ecx, 1
0x1400be8b0 mov      edx, 0x17
0x1400be8b5 call     0x1428d8fe3
0x1400be8ba xor      ebx, ebx
0x1400be8bc lea      r8, [rip + 0x2876485]
0x1400be8c3 mov      rcx, rbx
0x1400be8c6 mov      rdx, r15
0x1400be8c9 call     0x1428d8fe3
0x1400be8ce jmp      0x1400be92d
0x1400be8d0 mov      ecx, 8
0x1400be8d5 mov      edx, 8
0x1400be8da call     0x1428d9000
0x1400be8df jmp      0x1400be92d
0x1400be8e1 lea      r8, [rip + 0x2876460]
0x1400be8e8 mov      ecx, 1
0x1400be8ed mov      edx, 0xe
0x1400be8f2 call     0x1428d8fe3
0x1400be8f7 mov      ecx, 8
0x1400be8fc mov      edx, 8
0x1400be901 call     0x1428d9000
0x1400be906 jmp      0x1400be92d
0x1400be908 mov      ecx, 8
0x1400be90d mov      edx, 8
0x1400be912 call     0x1428d9000
0x1400be917 lea      r8, [rip + 0x287642a]
0x1400be91e mov      ecx, 1