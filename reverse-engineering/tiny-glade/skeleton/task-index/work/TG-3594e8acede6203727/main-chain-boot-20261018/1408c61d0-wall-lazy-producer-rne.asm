0x1408c61d0 55 push rbp
0x1408c61d1 4156 push r14
0x1408c61d3 56 push rsi
0x1408c61d4 57 push rdi
0x1408c61d5 53 push rbx
0x1408c61d6 4881ecc0000000 sub rsp, 0xc0
0x1408c61dd 488dac2480000000 lea rbp, [rsp + 0x80]
0x1408c61e5 48c74538feffffff mov qword ptr [rbp + 0x38], 0xfffffffffffffffe
0x1408c61ed bb01000000 mov ebx, 1
0x1408c61f2 f00fc19ab0050000 lock xadd dword ptr [rdx + 0x5b0], ebx
0x1408c61fa 31c0 xor eax, eax
0x1408c61fc 483b8148010000 cmp rax, qword ptr [rcx + 0x148]
0x1408c6203 0f80dd010000 jo 0x1408c63e6
0x1408c6209 4889ce mov rsi, rcx
0x1408c620c 488b8138010000 mov rax, qword ptr [rcx + 0x138]
0x1408c6213 48398218020000 cmp qword ptr [rdx + 0x218], rax
0x1408c621a 0f8634010000 jbe 0x1408c6354
0x1408c6220 488b8a10020000 mov rcx, qword ptr [rdx + 0x210]
0x1408c6227 488b04c1 mov rax, qword ptr [rcx + rax*8]
0x1408c622b 4885c0 test rax, rax
0x1408c622e 0f8420010000 je 0x1408c6354
0x1408c6234 4c8bb2e0010000 mov r14, qword ptr [rdx + 0x1e0]
0x1408c623b 48f7d0 not rax
0x1408c623e 488d0440 lea rax, [rax + rax*2]
0x1408c6242 48c1e005 shl rax, 5
0x1408c6246 49837c062000 cmp qword ptr [r14 + rax + 0x20], 0
0x1408c624c 0f8402010000 je 0x1408c6354
0x1408c6252 4901c6 add r14, rax
0x1408c6255 498b7e10 mov rdi, qword ptr [r14 + 0x10]
0x1408c6259 8b82b0050000 mov eax, dword ptr [rdx + 0x5b0]
0x1408c625f 488b8640010000 mov rax, qword ptr [rsi + 0x140]
0x1408c6266 48398218020000 cmp qword ptr [rdx + 0x218], rax
0x1408c626d 0f86fd000000 jbe 0x1408c6370
0x1408c6273 488b8a10020000 mov rcx, qword ptr [rdx + 0x210]
0x1408c627a 488b0cc1 mov rcx, qword ptr [rcx + rax*8]
0x1408c627e 4885c9 test rcx, rcx
0x1408c6281 0f84e9000000 je 0x1408c6370
0x1408c6287 488b82e0010000 mov rax, qword ptr [rdx + 0x1e0]
0x1408c628e 48f7d1 not rcx
0x1408c6291 488d0c49 lea rcx, [rcx + rcx*2]
0x1408c6295 48c1e105 shl rcx, 5
0x1408c6299 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1408c629f 0f84cb000000 je 0x1408c6370
0x1408c62a5 448b8630010000 mov r8d, dword ptr [rsi + 0x130]
0x1408c62ac 4801c8 add rax, rcx
0x1408c62af 4d8d4e58 lea r9, [r14 + 0x58]
0x1408c62b3 488b4810 mov rcx, qword ptr [rax + 0x10]
0x1408c62b7 448b92b0050000 mov r10d, dword ptr [rdx + 0x5b0]
0x1408c62be 48897da0 mov qword ptr [rbp - 0x60], rdi
0x1408c62c2 4c894da8 mov qword ptr [rbp - 0x58], r9
0x1408c62c6 4d8d4e5c lea r9, [r14 + 0x5c]
0x1408c62ca 4c894db0 mov qword ptr [rbp - 0x50], r9
0x1408c62ce 4c8d4858 lea r9, [rax + 0x58]
0x1408c62d2 448945b8 mov dword ptr [rbp - 0x48], r8d
0x1408c62d6 895dbc mov dword ptr [rbp - 0x44], ebx
0x1408c62d9 48894dc0 mov qword ptr [rbp - 0x40], rcx
0x1408c62dd 4c894dc8 mov qword ptr [rbp - 0x38], r9
0x1408c62e1 4c8d485c lea r9, [rax + 0x5c]
0x1408c62e5 4c894dd0 mov qword ptr [rbp - 0x30], r9
0x1408c62e9 448945d8 mov dword ptr [rbp - 0x28], r8d
0x1408c62ed 895ddc mov dword ptr [rbp - 0x24], ebx
0x1408c62f0 488955e0 mov qword ptr [rbp - 0x20], rdx
0x1408c62f4 488d9648010000 lea rdx, [rsi + 0x148]
0x1408c62fb 48c745e800000000 mov qword ptr [rbp - 0x18], 0
0x1408c6303 488955f0 mov qword ptr [rbp - 0x10], rdx
0x1408c6307 89585c mov dword ptr [rax + 0x5c], ebx
0x1408c630a e8d1f51400 call 0x140a158e0
0x1408c630f 48894530 mov qword ptr [rbp + 0x30], rax
0x1408c6313 48894510 mov qword ptr [rbp + 0x10], rax
0x1408c6317 41895e5c mov dword ptr [r14 + 0x5c], ebx
0x1408c631b 4883c718 add rdi, 0x18
0x1408c631f 4889f9 mov rcx, rdi
0x1408c6322 e869dfb800 call 0x141454290
0x1408c6327 488d4de0 lea rcx, [rbp - 0x20]
0x1408c632b 4c8d0d163f2102 lea r9, [rip + 0x2213f16]
0x1408c6332 488b5530 mov rdx, qword ptr [rbp + 0x30]
0x1408c6336 4989c0 mov r8, rax
0x1408c6339 e8c2a9a100 call 0x1412e0d00
0x1408c633e 899e30010000 mov dword ptr [rsi + 0x130], ebx
0x1408c6344 31c0 xor eax, eax
0x1408c6346 4881c4c0000000 add rsp, 0xc0
0x1408c634d 5b pop rbx
0x1408c634e 5f pop rdi
0x1408c634f 5e pop rsi
0x1408c6350 415e pop r14
0x1408c6352 5d pop rbp
0x1408c6353 c3 ret 
0x1408c6354 4881c698000000 add rsi, 0x98
0x1408c635b 488d054ec41d02 lea rax, [rip + 0x21dc44e]
0x1408c6362 48894500 mov qword ptr [rbp], rax
0x1408c6366 48c7450877000000 mov qword ptr [rbp + 8], 0x77
0x1408c636e eb1a jmp 0x1408c638a
0x1408c6370 4881c698000000 add rsi, 0x98
0x1408c6377 488d05dfc31d02 lea rax, [rip + 0x21dc3df]
0x1408c637e 48894500 mov qword ptr [rbp], rax
0x1408c6382 48c7450836000000 mov qword ptr [rbp + 8], 0x36
0x1408c638a 48897510 mov qword ptr [rbp + 0x10], rsi
0x1408c638e 488d053b5777ff lea rax, [rip - 0x88a8c5]
0x1408c6395 48894518 mov qword ptr [rbp + 0x18], rax
0x1408c6399 4889e8 mov rax, rbp
0x1408c639c 48894520 mov qword ptr [rbp + 0x20], rax
0x1408c63a0 488d05e93a84ff lea rax, [rip - 0x7bc517]
0x1408c63a7 48894528 mov qword ptr [rbp + 0x28], rax
0x1408c63ab 488d05dec11d02 lea rax, [rip + 0x21dc1de]
0x1408c63b2 488945a0 mov qword ptr [rbp - 0x60], rax
0x1408c63b6 48c745a802000000 mov qword ptr [rbp - 0x58], 2
0x1408c63be 48c745c000000000 mov qword ptr [rbp - 0x40], 0
0x1408c63c6 488d4510 lea rax, [rbp + 0x10]
0x1408c63ca 488945b0 mov qword ptr [rbp - 0x50], rax
0x1408c63ce 48c745b802000000 mov qword ptr [rbp - 0x48], 2
0x1408c63d6 488d15bbc31d02 lea rdx, [rip + 0x21dc3bb]
0x1408c63dd 488d4da0 lea rcx, [rbp - 0x60]
0x1408c63e1 e8aa2f0102 call 0x1428d9390
0x1408c63e6 488d0d63c51d02 lea rcx, [rip + 0x21dc563]
0x1408c63ed 4c8d05bcc51d02 lea r8, [rip + 0x21dc5bc]
0x1408c63f4 ba59000000 mov edx, 0x59
0x1408c63f9 e8322f0102 call 0x1428d9330
0x1408c63fe cc int3 
0x1408c63ff 90 nop 
0x1408c6400 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1408c6405 55 push rbp
0x1408c6406 4156 push r14
0x1408c6408 56 push rsi
0x1408c6409 57 push rdi
0x1408c640a 53 push rbx
0x1408c640b 4883ec20 sub rsp, 0x20
0x1408c640f 488daa80000000 lea rbp, [rdx + 0x80]
0x1408c6416 488b4530 mov rax, qword ptr [rbp + 0x30]
0x1408c641a f048ff08 lock dec qword ptr [rax]
0x1408c641e 7509 jne 0x1408c6429
0x1408c6420 488d4d10 lea rcx, [rbp + 0x10]
0x1408c6424 e8176c2900 call 0x140b5d040
0x1408c6429 90 nop 
0x1408c642a 4883c420 add rsp, 0x20
0x1408c642e 5b pop rbx
0x1408c642f 5f pop rdi
0x1408c6430 5e pop rsi
0x1408c6431 415e pop r14
0x1408c6433 5d pop rbp
0x1408c6434 c3 ret 
