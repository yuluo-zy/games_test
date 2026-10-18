0x140a97110 55 push rbp
0x140a97111 56 push rsi
0x140a97112 57 push rdi
0x140a97113 53 push rbx
0x140a97114 4881ece8020000 sub rsp, 0x2e8
0x140a9711b 488dac2480000000 lea rbp, [rsp + 0x80]
0x140a97123 48c78560020000feffffff mov qword ptr [rbp + 0x260], 0xfffffffffffffffe
0x140a9712e 4889ce mov rsi, rcx
0x140a97131 0fb605d1dab902 movzx eax, byte ptr [rip + 0x2b9dad1]
0x140a97138 b9c0010000 mov ecx, 0x1c0
0x140a9713d ba08000000 mov edx, 8
0x140a97142 e8c9cab7ff call 0x140613c10
0x140a97147 4885c0 test rax, rax
0x140a9714a 0f84a7030000 je 0x140a974f7
0x140a97150 48898558020000 mov qword ptr [rbp + 0x258], rax
0x140a97157 0f57c0 xorps xmm0, xmm0
0x140a9715a 0f118538010000 movups xmmword ptr [rbp + 0x138], xmm0
0x140a97161 0f118550010000 movups xmmword ptr [rbp + 0x150], xmm0
0x140a97168 0f118568010000 movups xmmword ptr [rbp + 0x168], xmm0
0x140a9716f 0f118580010000 movups xmmword ptr [rbp + 0x180], xmm0
0x140a97176 0f118598010000 movups xmmword ptr [rbp + 0x198], xmm0
0x140a9717d c785a801000000000000 mov dword ptr [rbp + 0x1a8], 0
0x140a97187 0f1185d0010000 movups xmmword ptr [rbp + 0x1d0], xmm0
0x140a9718e 0f1185e8010000 movups xmmword ptr [rbp + 0x1e8], xmm0
0x140a97195 0f118500020000 movups xmmword ptr [rbp + 0x200], xmm0
0x140a9719c 0f118518020000 movups xmmword ptr [rbp + 0x218], xmm0
0x140a971a3 0f118530020000 movups xmmword ptr [rbp + 0x230], xmm0
0x140a971aa c7854002000000000000 mov dword ptr [rbp + 0x240], 0
0x140a971b4 48c785f800000000000000 mov qword ptr [rbp + 0xf8], 0
0x140a971bf 48c7851801000000000000 mov qword ptr [rbp + 0x118], 0
0x140a971ca 48c7852001000008000000 mov qword ptr [rbp + 0x120], 8
0x140a971d5 48c7852801000000000000 mov qword ptr [rbp + 0x128], 0
0x140a971e0 48c7853001000010000000 mov qword ptr [rbp + 0x130], 0x10
0x140a971eb 48c7854801000010000000 mov qword ptr [rbp + 0x148], 0x10
0x140a971f6 48c7856001000010000000 mov qword ptr [rbp + 0x160], 0x10
0x140a97201 48c7857801000010000000 mov qword ptr [rbp + 0x178], 0x10
0x140a9720c 48c7859001000010000000 mov qword ptr [rbp + 0x190], 0x10
0x140a97217 48bb0000000000000080 movabs rbx, 0x8000000000000000
0x140a97221 48899db0010000 mov qword ptr [rbp + 0x1b0], rbx
0x140a97228 488d05093d0402 lea rax, [rip + 0x2043d09]
0x140a9722f 488985b8010000 mov qword ptr [rbp + 0x1b8], rax
0x140a97236 48c785c00100005b000000 mov qword ptr [rbp + 0x1c0], 0x5b
0x140a97241 48c785c801000010000000 mov qword ptr [rbp + 0x1c8], 0x10
0x140a9724c 48c785e001000010000000 mov qword ptr [rbp + 0x1e0], 0x10
0x140a97257 48c785f801000010000000 mov qword ptr [rbp + 0x1f8], 0x10
0x140a97262 48c7851002000010000000 mov qword ptr [rbp + 0x210], 0x10
0x140a9726d 48c7852802000010000000 mov qword ptr [rbp + 0x228], 0x10
0x140a97278 c7854802000000000000 mov dword ptr [rbp + 0x248], 0
0x140a97282 66c7854c0200000100 mov word ptr [rbp + 0x24c], 1
0x140a9728b c7855002000000000000 mov dword ptr [rbp + 0x250], 0
0x140a97295 0fb6056dd9b902 movzx eax, byte ptr [rip + 0x2b9d96d]
0x140a9729c b960010000 mov ecx, 0x160
0x140a972a1 ba08000000 mov edx, 8
0x140a972a6 e865c9b7ff call 0x140613c10
0x140a972ab 4885c0 test rax, rax
0x140a972ae 0f8452020000 je 0x140a97506
0x140a972b4 4889c7 mov rdi, rax
0x140a972b7 488d95f8000000 lea rdx, [rbp + 0xf8]
0x140a972be 41b860010000 mov r8d, 0x160
0x140a972c4 4889c1 mov rcx, rax
0x140a972c7 e834c9e801 call 0x142923c00
0x140a972cc 4c8d057d360402 lea r8, [rip + 0x204367d]
0x140a972d3 488d8d88000000 lea rcx, [rbp + 0x88]
0x140a972da 4889fa mov rdx, rdi
0x140a972dd e82ed82400 call 0x140ce4b10
0x140a972e2 488d4d18 lea rcx, [rbp + 0x18]
0x140a972e6 e8554cf9ff call 0x140a2bf40
0x140a972eb 488d4da8 lea rcx, [rbp - 0x58]
0x140a972ef e85cd4b500 call 0x1415f4750
0x140a972f4 488d8df8000000 lea rcx, [rbp + 0xf8]
0x140a972fb e80013f3ff call 0x1409c8600
0x140a97300 0f1085e8000000 movups xmm0, xmmword ptr [rbp + 0xe8]
0x140a97307 488b8558020000 mov rax, qword ptr [rbp + 0x258]
0x140a9730e 0f114060 movups xmmword ptr [rax + 0x60], xmm0
0x140a97312 0f1085d8000000 movups xmm0, xmmword ptr [rbp + 0xd8]
0x140a97319 0f114050 movups xmmword ptr [rax + 0x50], xmm0
0x140a9731d 0f1085c8000000 movups xmm0, xmmword ptr [rbp + 0xc8]
0x140a97324 0f114040 movups xmmword ptr [rax + 0x40], xmm0
0x140a97328 0f108588000000 movups xmm0, xmmword ptr [rbp + 0x88]
0x140a9732f 0f108d98000000 movups xmm1, xmmword ptr [rbp + 0x98]
0x140a97336 0f1095a8000000 movups xmm2, xmmword ptr [rbp + 0xa8]
0x140a9733d 0f109db8000000 movups xmm3, xmmword ptr [rbp + 0xb8]
0x140a97344 0f115830 movups xmmword ptr [rax + 0x30], xmm3
0x140a97348 0f115020 movups xmmword ptr [rax + 0x20], xmm2
0x140a9734c 0f114810 movups xmmword ptr [rax + 0x10], xmm1
0x140a97350 0f1100 movups xmmword ptr [rax], xmm0
0x140a97353 0f104518 movups xmm0, xmmword ptr [rbp + 0x18]
0x140a97357 0f104d28 movups xmm1, xmmword ptr [rbp + 0x28]
0x140a9735b 0f105538 movups xmm2, xmmword ptr [rbp + 0x38]
0x140a9735f 0f105d48 movups xmm3, xmmword ptr [rbp + 0x48]
0x140a97363 0f114070 movups xmmword ptr [rax + 0x70], xmm0
0x140a97367 0f118880000000 movups xmmword ptr [rax + 0x80], xmm1
0x140a9736e 0f119090000000 movups xmmword ptr [rax + 0x90], xmm2
0x140a97375 0f1198a0000000 movups xmmword ptr [rax + 0xa0], xmm3
0x140a9737c 0f104558 movups xmm0, xmmword ptr [rbp + 0x58]
0x140a97380 0f1180b0000000 movups xmmword ptr [rax + 0xb0], xmm0
0x140a97387 0f104568 movups xmm0, xmmword ptr [rbp + 0x68]
0x140a9738b 0f1180c0000000 movups xmmword ptr [rax + 0xc0], xmm0
0x140a97392 0f104578 movups xmm0, xmmword ptr [rbp + 0x78]
0x140a97396 0f1180d0000000 movups xmmword ptr [rax + 0xd0], xmm0
0x140a9739d 0f104508 movups xmm0, xmmword ptr [rbp + 8]
0x140a973a1 0f118040010000 movups xmmword ptr [rax + 0x140], xmm0
0x140a973a8 0f1045f8 movups xmm0, xmmword ptr [rbp - 8]
0x140a973ac 0f118030010000 movups xmmword ptr [rax + 0x130], xmm0
0x140a973b3 0f1045e8 movups xmm0, xmmword ptr [rbp - 0x18]
0x140a973b7 0f118020010000 movups xmmword ptr [rax + 0x120], xmm0
0x140a973be 0f1045a8 movups xmm0, xmmword ptr [rbp - 0x58]
0x140a973c2 0f104db8 movups xmm1, xmmword ptr [rbp - 0x48]
0x140a973c6 0f1055c8 movups xmm2, xmmword ptr [rbp - 0x38]
0x140a973ca 0f105dd8 movups xmm3, xmmword ptr [rbp - 0x28]
0x140a973ce 0f119810010000 movups xmmword ptr [rax + 0x110], xmm3
0x140a973d5 0f119000010000 movups xmmword ptr [rax + 0x100], xmm2
0x140a973dc 0f1188f0000000 movups xmmword ptr [rax + 0xf0], xmm1
0x140a973e3 0f1180e0000000 movups xmmword ptr [rax + 0xe0], xmm0
0x140a973ea 0f1085f8000000 movups xmm0, xmmword ptr [rbp + 0xf8]
0x140a973f1 0f108d08010000 movups xmm1, xmmword ptr [rbp + 0x108]
0x140a973f8 0f109518010000 movups xmm2, xmmword ptr [rbp + 0x118]
0x140a973ff 0f109d28010000 movups xmm3, xmmword ptr [rbp + 0x128]
0x140a97406 0f118050010000 movups xmmword ptr [rax + 0x150], xmm0
0x140a9740d 0f118860010000 movups xmmword ptr [rax + 0x160], xmm1
0x140a97414 0f119070010000 movups xmmword ptr [rax + 0x170], xmm2
0x140a9741b 0f119880010000 movups xmmword ptr [rax + 0x180], xmm3
0x140a97422 0f108538010000 movups xmm0, xmmword ptr [rbp + 0x138]
0x140a97429 0f118090010000 movups xmmword ptr [rax + 0x190], xmm0
0x140a97430 0f108548010000 movups xmm0, xmmword ptr [rbp + 0x148]
0x140a97437 0f1180a0010000 movups xmmword ptr [rax + 0x1a0], xmm0
0x140a9743e 0f108558010000 movups xmm0, xmmword ptr [rbp + 0x158]
0x140a97445 0f1180b0010000 movups xmmword ptr [rax + 0x1b0], xmm0
0x140a9744c 48c7850001000004000000 mov qword ptr [rbp + 0x100], 4
0x140a97457 48898508010000 mov qword ptr [rbp + 0x108], rax
0x140a9745e 48c7851001000004000000 mov qword ptr [rbp + 0x110], 4
0x140a97469 48c7851801000000000000 mov qword ptr [rbp + 0x118], 0
0x140a97474 48c7852001000008000000 mov qword ptr [rbp + 0x120], 8
0x140a9747f 0f57c0 xorps xmm0, xmm0
0x140a97482 0f118528010000 movups xmmword ptr [rbp + 0x128], xmm0
0x140a97489 48899df8000000 mov qword ptr [rbp + 0xf8], rbx
0x140a97490 488d8d30010000 lea rcx, [rbp + 0x130]
0x140a97497 e884852900 call 0x140d2fa20
0x140a9749c 0f108558010000 movups xmm0, xmmword ptr [rbp + 0x158]
0x140a974a3 0f114660 movups xmmword ptr [rsi + 0x60], xmm0
0x140a974a7 0f108548010000 movups xmm0, xmmword ptr [rbp + 0x148]
0x140a974ae 0f114650 movups xmmword ptr [rsi + 0x50], xmm0
0x140a974b2 0f108538010000 movups xmm0, xmmword ptr [rbp + 0x138]
0x140a974b9 0f114640 movups xmmword ptr [rsi + 0x40], xmm0
0x140a974bd 0f1085f8000000 movups xmm0, xmmword ptr [rbp + 0xf8]
0x140a974c4 0f108d08010000 movups xmm1, xmmword ptr [rbp + 0x108]
0x140a974cb 0f109518010000 movups xmm2, xmmword ptr [rbp + 0x118]
0x140a974d2 0f109d28010000 movups xmm3, xmmword ptr [rbp + 0x128]
0x140a974d9 0f115e30 movups xmmword ptr [rsi + 0x30], xmm3
0x140a974dd 0f115620 movups xmmword ptr [rsi + 0x20], xmm2
0x140a974e1 0f114e10 movups xmmword ptr [rsi + 0x10], xmm1
0x140a974e5 0f1106 movups xmmword ptr [rsi], xmm0
0x140a974e8 4889f0 mov rax, rsi
0x140a974eb 4881c4e8020000 add rsp, 0x2e8
0x140a974f2 5b pop rbx
0x140a974f3 5f pop rdi
0x140a974f4 5e pop rsi
0x140a974f5 5d pop rbp
0x140a974f6 c3 ret 
0x140a974f7 b908000000 mov ecx, 8
0x140a974fc bac0010000 mov edx, 0x1c0
0x140a97501 e8fa1ae401 call 0x1428d9000
0x140a97506 b908000000 mov ecx, 8
0x140a9750b ba60010000 mov edx, 0x160
0x140a97510 e8eb1ae401 call 0x1428d9000
0x140a97515 0f0b ud2 
0x140a97517 660f1f840000000000 nop word ptr [rax + rax]
0x140a97520 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a97525 55 push rbp
0x140a97526 56 push rsi
0x140a97527 57 push rdi
0x140a97528 53 push rbx
0x140a97529 4883ec28 sub rsp, 0x28
0x140a9752d 488daa80000000 lea rbp, [rdx + 0x80]
0x140a97534 488d8df8000000 lea rcx, [rbp + 0xf8]
0x140a9753b e830e9b4ff call 0x1405e5e70
0x140a97540 90 nop 
0x140a97541 4883c428 add rsp, 0x28
0x140a97545 5b pop rbx
0x140a97546 5f pop rdi
0x140a97547 5e pop rsi
0x140a97548 5d pop rbp
0x140a97549 c3 ret 
0x140a9754a 660f1f440000 nop word ptr [rax + rax]
0x140a97550 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a97555 55 push rbp
0x140a97556 56 push rsi
0x140a97557 57 push rdi
0x140a97558 53 push rbx
0x140a97559 4883ec28 sub rsp, 0x28
0x140a9755d 488daa80000000 lea rbp, [rdx + 0x80]
0x140a97564 bac0010000 mov edx, 0x1c0
0x140a97569 41b808000000 mov r8d, 8
0x140a9756f 488b8d58020000 mov rcx, qword ptr [rbp + 0x258]
0x140a97576 e8a5c6b7ff call 0x140613c20
0x140a9757b 90 nop 
0x140a9757c 4883c428 add rsp, 0x28
0x140a97580 5b pop rbx
0x140a97581 5f pop rdi
0x140a97582 5e pop rsi
0x140a97583 5d pop rbp
0x140a97584 c3 ret 
0x140a97585 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a97590 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a97595 55 push rbp
0x140a97596 56 push rsi
0x140a97597 57 push rdi
0x140a97598 53 push rbx
0x140a97599 4883ec28 sub rsp, 0x28
0x140a9759d 488daa80000000 lea rbp, [rdx + 0x80]
0x140a975a4 488d8d88000000 lea rcx, [rbp + 0x88]
0x140a975ab e8d02259ff call 0x140029880
0x140a975b0 90 nop 
0x140a975b1 4883c428 add rsp, 0x28
0x140a975b5 5b pop rbx
0x140a975b6 5f pop rdi
0x140a975b7 5e pop rsi
0x140a975b8 5d pop rbp
0x140a975b9 c3 ret 
0x140a975ba 660f1f440000 nop word ptr [rax + rax]
0x140a975c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a975c5 55 push rbp
0x140a975c6 56 push rsi
0x140a975c7 57 push rdi
0x140a975c8 53 push rbx
0x140a975c9 4883ec28 sub rsp, 0x28
0x140a975cd 488daa80000000 lea rbp, [rdx + 0x80]
0x140a975d4 488d4d18 lea rcx, [rbp + 0x18]
0x140a975d8 e8a32259ff call 0x140029880
0x140a975dd 90 nop 
0x140a975de 4883c428 add rsp, 0x28
0x140a975e2 5b pop rbx
0x140a975e3 5f pop rdi
0x140a975e4 5e pop rsi
0x140a975e5 5d pop rbp
0x140a975e6 c3 ret 
0x140a975e7 660f1f840000000000 nop word ptr [rax + rax]
0x140a975f0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a975f5 55 push rbp
0x140a975f6 56 push rsi
0x140a975f7 57 push rdi
0x140a975f8 53 push rbx
0x140a975f9 4883ec28 sub rsp, 0x28
0x140a975fd 488daa80000000 lea rbp, [rdx + 0x80]
0x140a97604 488d4da8 lea rcx, [rbp - 0x58]
0x140a97608 e8732259ff call 0x140029880
0x140a9760d 90 nop 
0x140a9760e 4883c428 add rsp, 0x28
0x140a97612 5b pop rbx
0x140a97613 5f pop rdi
0x140a97614 5e pop rsi
0x140a97615 5d pop rbp
0x140a97616 c3 ret 
0x140a97617 660f1f840000000000 nop word ptr [rax + rax]
0x140a97620 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a97625 55 push rbp
0x140a97626 56 push rsi
0x140a97627 57 push rdi
0x140a97628 53 push rbx
0x140a97629 4883ec28 sub rsp, 0x28
0x140a9762d 488daa80000000 lea rbp, [rdx + 0x80]
0x140a97634 488d8df8000000 lea rcx, [rbp + 0xf8]
0x140a9763b e8402259ff call 0x140029880
0x140a97640 90 nop 
0x140a97641 4883c428 add rsp, 0x28
0x140a97645 5b pop rbx
0x140a97646 5f pop rdi
0x140a97647 5e pop rsi
0x140a97648 5d pop rbp
0x140a97649 c3 ret 
