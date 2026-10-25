0x14158e0a0 55 push rbp
0x14158e0a1 4157 push r15
0x14158e0a3 4156 push r14
0x14158e0a5 4154 push r12
0x14158e0a7 56 push rsi
0x14158e0a8 57 push rdi
0x14158e0a9 53 push rbx
0x14158e0aa 4881ec90000000 sub rsp, 0x90
0x14158e0b1 488dac2480000000 lea rbp, [rsp + 0x80]
0x14158e0b9 48c74508feffffff mov qword ptr [rbp + 8], 0xfffffffffffffffe
0x14158e0c1 4c89c7 mov rdi, r8
0x14158e0c4 4d85c0 test r8, r8
0x14158e0c7 0f88f8000000 js 0x14158e1c5
0x14158e0cd 4c89cb mov rbx, r9
0x14158e0d0 4889ce mov rsi, rcx
0x14158e0d3 742e je 0x14158e103
0x14158e0d5 4989d4 mov r12, rdx
0x14158e0d8 0fb6052a6b0a02 movzx eax, byte ptr [rip + 0x20a6b2a]
0x14158e0df 41bf01000000 mov r15d, 1
0x14158e0e5 ba01000000 mov edx, 1
0x14158e0ea 4889f9 mov rcx, rdi
0x14158e0ed e81e5b08ff call 0x140613c10
0x14158e0f2 4885c0 test rax, rax
0x14158e0f5 0f84cd000000 je 0x14158e1c8
0x14158e0fb 4989c6 mov r14, rax
0x14158e0fe 4c89e2 mov rdx, r12
0x14158e101 eb06 jmp 0x14158e109
0x14158e103 41be01000000 mov r14d, 1
0x14158e109 4c89f1 mov rcx, r14
0x14158e10c 4989f8 mov r8, rdi
0x14158e10f e8ec5a3901 call 0x142923c00
0x14158e114 48897dc8 mov qword ptr [rbp - 0x38], rdi
0x14158e118 4c8975d0 mov qword ptr [rbp - 0x30], r14
0x14158e11c 48897dd8 mov qword ptr [rbp - 0x28], rdi
0x14158e120 8b03 mov eax, dword ptr [rbx]
0x14158e122 c645a802 mov byte ptr [rbp - 0x58], 2
0x14158e126 48c745b000000000 mov qword ptr [rbp - 0x50], 0
0x14158e12e 488945b8 mov qword ptr [rbp - 0x48], rax
0x14158e132 4883c618 add rsi, 0x18
0x14158e136 488d4de0 lea rcx, [rbp - 0x20]
0x14158e13a 4c8d45c8 lea r8, [rbp - 0x38]
0x14158e13e 4c8d4da8 lea r9, [rbp - 0x58]
0x14158e142 4889f2 mov rdx, rsi
0x14158e145 e8e6d8eaff call 0x14143ba30
0x14158e14a 0fb645e0 movzx eax, byte ptr [rbp - 0x20]
0x14158e14e 4883f806 cmp rax, 6
0x14158e152 7754 ja 0x14158e1a8
0x14158e154 488d0d71a16e01 lea rcx, [rip + 0x16ea171]
0x14158e15b 48630481 movsxd rax, dword ptr [rcx + rax*4]
0x14158e15f 4801c8 add rax, rcx
0x14158e162 ffe0 jmp rax
0x14158e164 488b55e8 mov rdx, qword ptr [rbp - 0x18]
0x14158e168 4885d2 test rdx, rdx
0x14158e16b 7444 je 0x14158e1b1
0x14158e16d 488b4df0 mov rcx, qword ptr [rbp - 0x10]
0x14158e171 41b801000000 mov r8d, 1
0x14158e177 eb28 jmp 0x14158e1a1
0x14158e179 488b4df0 mov rcx, qword ptr [rbp - 0x10]
0x14158e17d 488b55f8 mov rdx, qword ptr [rbp - 8]
0x14158e181 48894d00 mov qword ptr [rbp], rcx
0x14158e185 e8e6ab2fff call 0x140888d70
0x14158e18a 488b55e8 mov rdx, qword ptr [rbp - 0x18]
0x14158e18e 4885d2 test rdx, rdx
0x14158e191 741e je 0x14158e1b1
0x14158e193 48c1e205 shl rdx, 5
0x14158e197 41b808000000 mov r8d, 8
0x14158e19d 488b4d00 mov rcx, qword ptr [rbp]
0x14158e1a1 e87a5a08ff call 0x140613c20
0x14158e1a6 eb09 jmp 0x14158e1b1
0x14158e1a8 488d4de8 lea rcx, [rbp - 0x18]
0x14158e1ac e88fad30ff call 0x140898f40
0x14158e1b1 31c0 xor eax, eax
0x14158e1b3 4881c490000000 add rsp, 0x90
0x14158e1ba 5b pop rbx
0x14158e1bb 5f pop rdi
0x14158e1bc 5e pop rsi
0x14158e1bd 415c pop r12
0x14158e1bf 415e pop r14
0x14158e1c1 415f pop r15
0x14158e1c3 5d pop rbp
0x14158e1c4 c3 ret 
0x14158e1c5 4531ff xor r15d, r15d
0x14158e1c8 4c8d0509a06e01 lea r8, [rip + 0x16ea009]
0x14158e1cf 4c89f9 mov rcx, r15
0x14158e1d2 4889fa mov rdx, rdi
0x14158e1d5 e809ae3401 call 0x1428d8fe3
0x14158e1da cc int3 
0x14158e1db 0f1f440000 nop dword ptr [rax + rax]
0x14158e1e0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14158e1e5 55 push rbp
0x14158e1e6 4157 push r15
0x14158e1e8 4156 push r14
0x14158e1ea 4154 push r12
0x14158e1ec 56 push rsi
0x14158e1ed 57 push rdi
0x14158e1ee 53 push rbx
0x14158e1ef 4883ec20 sub rsp, 0x20
0x14158e1f3 488daa80000000 lea rbp, [rdx + 0x80]
0x14158e1fa 488b55e8 mov rdx, qword ptr [rbp - 0x18]
0x14158e1fe 4885d2 test rdx, rdx
0x14158e201 7413 je 0x14158e216
0x14158e203 48c1e205 shl rdx, 5
0x14158e207 41b808000000 mov r8d, 8
0x14158e20d 488b4d00 mov rcx, qword ptr [rbp]
0x14158e211 e80a5a08ff call 0x140613c20
0x14158e216 90 nop 
0x14158e217 4883c420 add rsp, 0x20
0x14158e21b 5b pop rbx
0x14158e21c 5f pop rdi
0x14158e21d 5e pop rsi
0x14158e21e 415c pop r12
0x14158e220 415e pop r14
0x14158e222 415f pop r15
0x14158e224 5d pop rbp
0x14158e225 c3 ret 
