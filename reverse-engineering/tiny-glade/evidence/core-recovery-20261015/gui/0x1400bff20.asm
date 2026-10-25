0x1400bff20 push     r15
0x1400bff22 push     r14
0x1400bff24 push     r13
0x1400bff26 push     r12
0x1400bff28 push     rsi
0x1400bff29 push     rdi
0x1400bff2a push     rbp
0x1400bff2b push     rbx
0x1400bff2c sub      rsp, 0xb8
0x1400bff33 mov      rdi, rdx
0x1400bff36 mov      rsi, rcx
0x1400bff39 mov      rax, qword ptr [rcx]
0x1400bff3c mov      ecx, dword ptr [rax + 0x1c]
0x1400bff3f mov      rdx, qword ptr [rax + 0x10]
0x1400bff43 mov      dword ptr [rdx], ecx
0x1400bff45 mov      r13, qword ptr [rax]
0x1400bff48 mov      r14, qword ptr [rsi + 8]
0x1400bff4c mov      r12, qword ptr [r14]
0x1400bff4f mov      qword ptr [rsp + 0x58], r12
0x1400bff54 lea      rcx, [r13 + 0xd8]
0x1400bff5b lea      rdx, [rsp + 0x58]
0x1400bff60 mov      qword ptr [rsp + 0x50], rcx
0x1400bff65 call     0x1405b0d70
0x1400bff6a mov      rbx, rax
0x1400bff6d shr      rbx, 0x39
0x1400bff71 mov      rcx, qword ptr [r13 + 0xb8]
0x1400bff78 mov      r10, qword ptr [r13 + 0xc0]
0x1400bff7f mov      r8, r10
0x1400bff82 mov      qword ptr [rsp + 0x48], rax
0x1400bff87 and      r8, rax
0x1400bff8a movd     xmm0, ebx
0x1400bff8e punpcklbw xmm0, xmm0
0x1400bff92 pshuflw  xmm0, xmm0, 0
0x1400bff97 pshufd   xmm0, xmm0, 0
0x1400bff9c lea      r9, [rcx - 0x10]
0x1400bffa0 xor      r11d, r11d
0x1400bffa3 pcmpeqd  xmm1, xmm1
0x1400bffa7 mov      rax, r8
0x1400bffaa movdqu   xmm2, xmmword ptr [rcx + rax]
0x1400bffaf movdqa   xmm3, xmm2
0x1400bffb3 pcmpeqb  xmm3, xmm0
0x1400bffb7 pmovmskb ebp, xmm3
0x1400bffbb test     ebp, ebp
0x1400bffbd je       0x1400c0020
0x1400bffbf mov      qword ptr [rsp + 0x40], r11
0x1400bffc4 mov      r11, r8
0x1400bffc7 mov      r8, rbx
0x1400bffca mov      r15, r14
0x1400bffcd mov      rdx, rsi
0x1400bffd0 mov      rsi, rdi
0x1400bffd3 tzcnt    edi, ebp
0x1400bffd7 add      rdi, rax
0x1400bffda and      rdi, r10
0x1400bffdd mov      r14, r10
0x1400bffe0 mov      r10, rdi
0x1400bffe3 shl      r10, 4
0x1400bffe7 mov      rbx, r9
0x1400bffea sub      r9, r10
0x1400bffed cmp      qword ptr [r9], r12
0x1400bfff0 je       0x1400c0047
0x1400bfff2 lea      r9d, [rbp - 1]
0x1400bfff6 and      r9w, bp
0x1400bfffa mov      ebp, r9d
0x1400bfffd mov      rdi, rsi
0x1400c0000 mov      rsi, rdx
0x1400c0003 mov      r9, rbx
0x1400c0006 mov      r10, r14
0x1400c0009 mov      r14, r15
0x1400c000c mov      rbx, r8
0x1400c000f mov      r8, r11
0x1400c0012 mov      r11, qword ptr [rsp + 0x40]
0x1400c0017 jne      0x1400bffc4
0x1400c0019 nop      dword ptr [rax]
0x1400c0020 mov      rdx, r9
0x1400c0023 pcmpeqb  xmm2, xmm1
0x1400c0027 pmovmskb r9d, xmm2
0x1400c002c test     r9d, r9d
0x1400c002f jne      0x1400c005f
0x1400c0031 add      rax, r11
0x1400c0034 add      rax, 0x10
0x1400c0038 add      r11, 0x10
0x1400c003c and      rax, r10
0x1400c003f mov      r9, rdx
0x1400c0042 jmp      0x1400bffaa
0x1400c0047 neg      rdi
0x1400c004a shl      rdi, 4
0x1400c004e add      rcx, rdi
0x1400c0051 mov      rbx, rcx
0x1400c0054 mov      rdi, rsi
0x1400c0057 mov      rsi, rdx
0x1400c005a mov      r14, r15
0x1400c005d jmp      0x1400c00d2
0x1400c005f cmp      qword ptr [r13 + 0xc8], 0
0x1400c0067 je       0x1400c01a1
0x1400c006d movdqu   xmm0, xmmword ptr [rcx + r8]
0x1400c0073 pmovmskb eax, xmm0
0x1400c0077 test     eax, eax
0x1400c0079 je       0x1400c01f3
0x1400c007f tzcnt    eax, eax
0x1400c0083 add      rax, r8
0x1400c0086 and      rax, r10
0x1400c0089 movzx    r8d, byte ptr [rcx + rax]
0x1400c008e test     r8b, r8b
0x1400c0091 jns      0x1400c0217
0x1400c0097 lea      r9, [rax - 0x10]
0x1400c009b and      r9, r10
0x1400c009e mov      byte ptr [rcx + rax], bl
0x1400c00a1 mov      byte ptr [rcx + r9 + 0x10], bl
0x1400c00a6 shl      rax, 4
0x1400c00aa and      r8b, 1
0x1400c00ae movzx    edx, r8b
0x1400c00b2 sub      qword ptr [r13 + 0xc8], rdx
0x1400c00b9 mov      rbx, rcx
0x1400c00bc sub      rbx, rax
0x1400c00bf neg      rax
0x1400c00c2 mov      qword ptr [rcx + rax - 0x10], r12
0x1400c00c7 mov      byte ptr [rbx - 8], 0
0x1400c00cb inc      qword ptr [r13 + 0xd0]
0x1400c00d2 mov      rax, qword ptr [rsi + 0x10]
0x1400c00d6 mov      r15, qword ptr [rax]
0x1400c00d9 mov      dword ptr [rsp + 0x30], 2
0x1400c00e1 movzx    eax, byte ptr [rip + 0x3574b21]
0x1400c00e8 mov      ecx, 0x30
0x1400c00ed mov      edx, 8
0x1400c00f2 call     0x140613c10
0x1400c00f7 test     rax, rax
0x1400c00fa je       0x1400c022d
0x1400c0100 add      rbx, -8
0x1400c0104 lea      rcx, [r14 + 0xc]
0x1400c0108 lea      rdx, [r14 + 8]
0x1400c010c lea      r8, [rsp + 0x30]
0x1400c0111 mov      qword ptr [rax], r8
0x1400c0114 mov      qword ptr [rax + 8], rdx
0x1400c0118 mov      qword ptr [rax + 0x10], rcx
0x1400c011c mov      qword ptr [rax + 0x18], r14
0x1400c0120 mov      qword ptr [rax + 0x20], rbx
0x1400c0124 mov      qword ptr [rax + 0x28], r15
0x1400c0128 lea      rcx, [rip + 0x2890ea9]
0x1400c012f mov      qword ptr [rsp + 0x20], rcx
0x1400c0134 lea      rcx, [rsp + 0x58]
0x1400c0139 mov      rdx, rdi
0x1400c013c xor      r8d, r8d
0x1400c013f mov      r9, rax
0x1400c0142 call     0x1400c4230
0x1400c0147 mov      rax, qword ptr [rsp + 0x68]
0x1400c014c lock dec qword ptr [rax]
0x1400c0150 jne      0x1400c015c
0x1400c0152 lea      rcx, [rsp + 0x68]
0x1400c0157 call     0x140c41170
0x1400c015c mov      eax, dword ptr [rsp + 0x30]
0x1400c0160 mov      rcx, qword ptr [rsp + 0x34]
0x1400c0165 mov      qword ptr [rsp + 0x58], rcx
0x1400c016a mov      ecx, dword ptr [rsp + 0x3c]
0x1400c016e mov      dword ptr [rsp + 0x60], ecx
0x1400c0172 cmp      eax, 2
0x1400c0175 je       0x1400c018d
0x1400c0177 mov      rcx, qword ptr [rsi + 0x18]
0x1400c017b mov      dword ptr [rcx], eax
0x1400c017d mov      rax, qword ptr [rsp + 0x58]
0x1400c0182 mov      qword ptr [rcx + 4], rax
0x1400c0186 mov      eax, dword ptr [rsp + 0x60]
0x1400c018a mov      dword ptr [rcx + 0xc], eax
0x1400c018d add      rsp, 0xb8
0x1400c0194 pop      rbx
0x1400c0195 pop      rbp
0x1400c0196 pop      rdi
0x1400c0197 pop      rsi
0x1400c0198 pop      r12
0x1400c019a pop      r13
0x1400c019c pop      r14
0x1400c019e pop      r15
0x1400c01a0 ret      
0x1400c01a1 lea      rcx, [r13 + 0xb8]
0x1400c01a8 mov      edx, 1
0x1400c01ad mov      r8, qword ptr [rsp + 0x50]
0x1400c01b2 mov      rbp, rsi
0x1400c01b5 mov      rsi, rdi
0x1400c01b8 mov      rdi, qword ptr [rsp + 0x48]
0x1400c01bd call     0x14289bd10
0x1400c01c2 mov      r12, qword ptr [rsp + 0x58]
0x1400c01c7 mov      rcx, qword ptr [r13 + 0xb8]
0x1400c01ce mov      r10, qword ptr [r13 + 0xc0]
0x1400c01d5 and      rdi, r10
0x1400c01d8 mov      r8, rdi
0x1400c01db mov      rdi, rsi
0x1400c01de mov      rsi, rbp
0x1400c01e1 movdqu   xmm0, xmmword ptr [rcx + r8]
0x1400c01e7 pmovmskb eax, xmm0
0x1400c01eb test     eax, eax
0x1400c01ed jne      0x1400c007f
0x1400c01f3 mov      r9d, 0x10
0x1400c01f9 add      r8, r9
0x1400c01fc and      r8, r10
0x1400c01ff movdqu   xmm0, xmmword ptr [rcx + r8]
0x1400c0205 pmovmskb eax, xmm0
0x1400c0209 add      r9, 0x10
0x1400c020d test     eax, eax
0x1400c020f jne      0x1400c007f
0x1400c0215 jmp      0x1400c01f9
0x1400c0217 movdqa   xmm0, xmmword ptr [rcx]
0x1400c021b pmovmskb eax, xmm0
0x1400c021f tzcnt    eax, eax
0x1400c0223 movzx    r8d, byte ptr [rcx + rax]
0x1400c0228 jmp      0x1400c0097
0x1400c022d mov      ecx, 8
0x1400c0232 mov      edx, 0x30
0x1400c0237 call     0x1428d9000