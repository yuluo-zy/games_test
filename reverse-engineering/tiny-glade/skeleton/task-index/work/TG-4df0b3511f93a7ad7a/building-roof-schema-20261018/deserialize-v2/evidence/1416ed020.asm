0x1416ed020 56 push rsi
0x1416ed021 4883ec60 sub rsp, 0x60
0x1416ed025 4889ce mov rsi, rcx
0x1416ed028 488d4c2448 lea rcx, [rsp + 0x48]
0x1416ed02d e84e54b9ff call 0x141282480
0x1416ed032 0fb6442448 movzx eax, byte ptr [rsp + 0x48]
0x1416ed037 488b542450 mov rdx, qword ptr [rsp + 0x50]
0x1416ed03c 3c02 cmp al, 2
0x1416ed03e 750c jne 0x1416ed04c
0x1416ed040 48895608 mov qword ptr [rsi + 8], rdx
0x1416ed044 c70602000000 mov dword ptr [rsi], 2
0x1416ed04a eb76 jmp 0x1416ed0c2
0x1416ed04c a801 test al, 1
0x1416ed04e 7416 je 0x1416ed066
0x1416ed050 4885d2 test rdx, rdx
0x1416ed053 7409 je 0x1416ed05e
0x1416ed055 803a12 cmp byte ptr [rdx], 0x12
0x1416ed058 0f8591000000 jne 0x1416ed0ef
0x1416ed05e c70601000000 mov dword ptr [rsi], 1
0x1416ed064 eb5c jmp 0x1416ed0c2
0x1416ed066 4885d2 test rdx, rdx
0x1416ed069 7460 je 0x1416ed0cb
0x1416ed06b 488d056e2c5a01 lea rax, [rip + 0x15a2c6e]
0x1416ed072 4889442420 mov qword ptr [rsp + 0x20], rax
0x1416ed077 48c744242802000000 mov qword ptr [rsp + 0x28], 2
0x1416ed080 4c8d05422c5a01 lea r8, [rip + 0x15a2c42]
0x1416ed087 488d4c2438 lea rcx, [rsp + 0x38]
0x1416ed08c 41b90c000000 mov r9d, 0xc
0x1416ed092 e8396ab9ff call 0x141283ad0
0x1416ed097 f644243801 test byte ptr [rsp + 0x38], 1
0x1416ed09c 7410 je 0x1416ed0ae
0x1416ed09e 488b442440 mov rax, qword ptr [rsp + 0x40]
0x1416ed0a3 48894608 mov qword ptr [rsi + 8], rax
0x1416ed0a7 b802000000 mov eax, 2
0x1416ed0ac eb12 jmp 0x1416ed0c0
0x1416ed0ae 8b442444 mov eax, dword ptr [rsp + 0x44]
0x1416ed0b2 89460c mov dword ptr [rsi + 0xc], eax
0x1416ed0b5 488b44243c mov rax, qword ptr [rsp + 0x3c]
0x1416ed0ba 48894604 mov qword ptr [rsi + 4], rax
0x1416ed0be 31c0 xor eax, eax
0x1416ed0c0 8906 mov dword ptr [rsi], eax
0x1416ed0c2 4889f0 mov rax, rsi
0x1416ed0c5 4883c460 add rsp, 0x60
0x1416ed0c9 5e pop rsi
0x1416ed0ca c3 ret 
0x1416ed0cb c64424480d mov byte ptr [rsp + 0x48], 0xd
0x1416ed0d0 488d1599a55301 lea rdx, [rip + 0x153a599]
0x1416ed0d7 4c8d05daa25301 lea r8, [rip + 0x153a2da]
0x1416ed0de 488d4c2448 lea rcx, [rsp + 0x48]
0x1416ed0e3 e818c91d01 call 0x1428c9a00
0x1416ed0e8 4889442440 mov qword ptr [rsp + 0x40], rax
0x1416ed0ed ebaf jmp 0x1416ed09e
0x1416ed0ef 4c8d054aa35301 lea r8, [rip + 0x153a34a]
0x1416ed0f6 488d442448 lea rax, [rsp + 0x48]
0x1416ed0fb 4889d1 mov rcx, rdx
0x1416ed0fe 4889c2 mov rdx, rax
0x1416ed101 e8eaae2001 call 0x1428f7ff0
0x1416ed106 48894608 mov qword ptr [rsi + 8], rax
0x1416ed10a e935ffffff jmp 0x1416ed044
