0x14158e230 55 push rbp
0x14158e231 4157 push r15
0x14158e233 4156 push r14
0x14158e235 4154 push r12
0x14158e237 56 push rsi
0x14158e238 57 push rdi
0x14158e239 53 push rbx
0x14158e23a 4881ec90000000 sub rsp, 0x90
0x14158e241 488dac2480000000 lea rbp, [rsp + 0x80]
0x14158e249 48c74508feffffff mov qword ptr [rbp + 8], 0xfffffffffffffffe
0x14158e251 4c89c7 mov rdi, r8
0x14158e254 4d85c0 test r8, r8
0x14158e257 0f88f8000000 js 0x14158e355
0x14158e25d 4c89cb mov rbx, r9
0x14158e260 4889ce mov rsi, rcx
0x14158e263 742e je 0x14158e293
0x14158e265 4989d4 mov r12, rdx
0x14158e268 0fb6059a690a02 movzx eax, byte ptr [rip + 0x20a699a]
0x14158e26f 41bf01000000 mov r15d, 1
0x14158e275 ba01000000 mov edx, 1
0x14158e27a 4889f9 mov rcx, rdi
0x14158e27d e88e5908ff call 0x140613c10
0x14158e282 4885c0 test rax, rax
0x14158e285 0f84cd000000 je 0x14158e358
0x14158e28b 4989c6 mov r14, rax
0x14158e28e 4c89e2 mov rdx, r12
0x14158e291 eb06 jmp 0x14158e299
0x14158e293 41be01000000 mov r14d, 1
0x14158e299 4c89f1 mov rcx, r14
0x14158e29c 4989f8 mov r8, rdi
0x14158e29f e85c593901 call 0x142923c00
0x14158e2a4 48897dc8 mov qword ptr [rbp - 0x38], rdi
0x14158e2a8 4c8975d0 mov qword ptr [rbp - 0x30], r14
0x14158e2ac 48897dd8 mov qword ptr [rbp - 0x28], rdi
0x14158e2b0 8b03 mov eax, dword ptr [rbx]
0x14158e2b2 c645a802 mov byte ptr [rbp - 0x58], 2
0x14158e2b6 48c745b000000000 mov qword ptr [rbp - 0x50], 0
0x14158e2be 488945b8 mov qword ptr [rbp - 0x48], rax
0x14158e2c2 4883c618 add rsi, 0x18
0x14158e2c6 488d4de0 lea rcx, [rbp - 0x20]
0x14158e2ca 4c8d45c8 lea r8, [rbp - 0x38]
0x14158e2ce 4c8d4da8 lea r9, [rbp - 0x58]
0x14158e2d2 4889f2 mov rdx, rsi
0x14158e2d5 e856d7eaff call 0x14143ba30
0x14158e2da 0fb645e0 movzx eax, byte ptr [rbp - 0x20]
0x14158e2de 4883f806 cmp rax, 6
0x14158e2e2 7754 ja 0x14158e338
0x14158e2e4 488d0dfd9f6e01 lea rcx, [rip + 0x16e9ffd]
0x14158e2eb 48630481 movsxd rax, dword ptr [rcx + rax*4]
0x14158e2ef 4801c8 add rax, rcx
0x14158e2f2 ffe0 jmp rax
0x14158e2f4 488b55e8 mov rdx, qword ptr [rbp - 0x18]
0x14158e2f8 4885d2 test rdx, rdx
0x14158e2fb 7444 je 0x14158e341
0x14158e2fd 488b4df0 mov rcx, qword ptr [rbp - 0x10]
0x14158e301 41b801000000 mov r8d, 1
0x14158e307 eb28 jmp 0x14158e331
0x14158e309 488b4df0 mov rcx, qword ptr [rbp - 0x10]
0x14158e30d 488b55f8 mov rdx, qword ptr [rbp - 8]
0x14158e311 48894d00 mov qword ptr [rbp], rcx
0x14158e315 e856aa2fff call 0x140888d70
0x14158e31a 488b55e8 mov rdx, qword ptr [rbp - 0x18]
0x14158e31e 4885d2 test rdx, rdx
0x14158e321 741e je 0x14158e341
0x14158e323 48c1e205 shl rdx, 5
0x14158e327 41b808000000 mov r8d, 8
0x14158e32d 488b4d00 mov rcx, qword ptr [rbp]
0x14158e331 e8ea5808ff call 0x140613c20
0x14158e336 eb09 jmp 0x14158e341
0x14158e338 488d4de8 lea rcx, [rbp - 0x18]
0x14158e33c e8ffab30ff call 0x140898f40
0x14158e341 31c0 xor eax, eax
0x14158e343 4881c490000000 add rsp, 0x90
0x14158e34a 5b pop rbx
0x14158e34b 5f pop rdi
0x14158e34c 5e pop rsi
0x14158e34d 415c pop r12
0x14158e34f 415e pop r14
0x14158e351 415f pop r15
0x14158e353 5d pop rbp
0x14158e354 c3 ret 
0x14158e355 4531ff xor r15d, r15d
0x14158e358 4c8d05799e6e01 lea r8, [rip + 0x16e9e79]
0x14158e35f 4c89f9 mov rcx, r15
0x14158e362 4889fa mov rdx, rdi
0x14158e365 e879ac3401 call 0x1428d8fe3
0x14158e36a cc int3 
0x14158e36b 0f1f440000 nop dword ptr [rax + rax]
0x14158e370 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14158e375 55 push rbp
0x14158e376 4157 push r15
0x14158e378 4156 push r14
0x14158e37a 4154 push r12
0x14158e37c 56 push rsi
0x14158e37d 57 push rdi
0x14158e37e 53 push rbx
0x14158e37f 4883ec20 sub rsp, 0x20
0x14158e383 488daa80000000 lea rbp, [rdx + 0x80]
0x14158e38a 488b55e8 mov rdx, qword ptr [rbp - 0x18]
0x14158e38e 4885d2 test rdx, rdx
0x14158e391 7413 je 0x14158e3a6
0x14158e393 48c1e205 shl rdx, 5
0x14158e397 41b808000000 mov r8d, 8
0x14158e39d 488b4d00 mov rcx, qword ptr [rbp]
0x14158e3a1 e87a5808ff call 0x140613c20
0x14158e3a6 90 nop 
0x14158e3a7 4883c420 add rsp, 0x20
0x14158e3ab 5b pop rbx
0x14158e3ac 5f pop rdi
0x14158e3ad 5e pop rsi
0x14158e3ae 415c pop r12
0x14158e3b0 415e pop r14
0x14158e3b2 415f pop r15
0x14158e3b4 5d pop rbp
0x14158e3b5 c3 ret 
