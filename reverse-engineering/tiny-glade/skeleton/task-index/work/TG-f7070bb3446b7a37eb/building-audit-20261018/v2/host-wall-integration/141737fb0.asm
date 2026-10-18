0x141737fb0 55 push rbp
0x141737fb1 56 push rsi
0x141737fb2 57 push rdi
0x141737fb3 53 push rbx
0x141737fb4 4881ec28040000 sub rsp, 0x428
0x141737fbb 488dac2480000000 lea rbp, [rsp + 0x80]
0x141737fc3 48c785a0030000feffffff mov qword ptr [rbp + 0x3a0], 0xfffffffffffffffe
0x141737fce 4889ce mov rsi, rcx
0x141737fd1 0fb60531ccef01 movzx eax, byte ptr [rip + 0x1efcc31]
0x141737fd8 b9a0020000 mov ecx, 0x2a0
0x141737fdd ba08000000 mov edx, 8
0x141737fe2 e829bcedfe call 0x140613c10
0x141737fe7 4885c0 test rax, rax
0x141737fea 0f8483040000 je 0x141738473
0x141737ff0 48898598030000 mov qword ptr [rbp + 0x398], rax
0x141737ff7 0f57c0 xorps xmm0, xmm0
0x141737ffa 0f118578020000 movups xmmword ptr [rbp + 0x278], xmm0
0x141738001 0f118590020000 movups xmmword ptr [rbp + 0x290], xmm0
0x141738008 0f1185a8020000 movups xmmword ptr [rbp + 0x2a8], xmm0
0x14173800f 0f1185c0020000 movups xmmword ptr [rbp + 0x2c0], xmm0
0x141738016 0f1185d8020000 movups xmmword ptr [rbp + 0x2d8], xmm0
0x14173801d c785e802000000000000 mov dword ptr [rbp + 0x2e8], 0
0x141738027 0f118510030000 movups xmmword ptr [rbp + 0x310], xmm0
0x14173802e 0f118528030000 movups xmmword ptr [rbp + 0x328], xmm0
0x141738035 0f118540030000 movups xmmword ptr [rbp + 0x340], xmm0
0x14173803c 0f118558030000 movups xmmword ptr [rbp + 0x358], xmm0
0x141738043 0f118570030000 movups xmmword ptr [rbp + 0x370], xmm0
0x14173804a c7858003000000000000 mov dword ptr [rbp + 0x380], 0
0x141738054 48c785d801000000000000 mov qword ptr [rbp + 0x1d8], 0
0x14173805f 48c7855802000000000000 mov qword ptr [rbp + 0x258], 0
0x14173806a 48c7856002000008000000 mov qword ptr [rbp + 0x260], 8
0x141738075 48c7856802000000000000 mov qword ptr [rbp + 0x268], 0
0x141738080 48c7857002000010000000 mov qword ptr [rbp + 0x270], 0x10
0x14173808b 48c7858802000010000000 mov qword ptr [rbp + 0x288], 0x10
0x141738096 48c785a002000010000000 mov qword ptr [rbp + 0x2a0], 0x10
0x1417380a1 48c785b802000010000000 mov qword ptr [rbp + 0x2b8], 0x10
0x1417380ac 48c785d002000010000000 mov qword ptr [rbp + 0x2d0], 0x10
0x1417380b7 48bb0000000000000080 movabs rbx, 0x8000000000000000
0x1417380c1 48899df0020000 mov qword ptr [rbp + 0x2f0], rbx
0x1417380c8 488d05c5545901 lea rax, [rip + 0x15954c5]
0x1417380cf 488985f8020000 mov qword ptr [rbp + 0x2f8], rax
0x1417380d6 48c785000300004b000000 mov qword ptr [rbp + 0x300], 0x4b
0x1417380e1 48c7850803000010000000 mov qword ptr [rbp + 0x308], 0x10
0x1417380ec 48c7852003000010000000 mov qword ptr [rbp + 0x320], 0x10
0x1417380f7 48c7853803000010000000 mov qword ptr [rbp + 0x338], 0x10
0x141738102 48c7855003000010000000 mov qword ptr [rbp + 0x350], 0x10
0x14173810d 48c7856803000010000000 mov qword ptr [rbp + 0x368], 0x10
0x141738118 c7858803000000000000 mov dword ptr [rbp + 0x388], 0
0x141738122 66c7858c0300000100 mov word ptr [rbp + 0x38c], 1
0x14173812b c7859003000000000000 mov dword ptr [rbp + 0x390], 0
0x141738135 0fb605cdcaef01 movzx eax, byte ptr [rip + 0x1efcacd]
0x14173813c b9c0010000 mov ecx, 0x1c0
0x141738141 ba08000000 mov edx, 8
0x141738146 e8c5baedfe call 0x140613c10
0x14173814b 4885c0 test rax, rax
0x14173814e 0f842e030000 je 0x141738482
0x141738154 4889c7 mov rdi, rax
0x141738157 488d95d8010000 lea rdx, [rbp + 0x1d8]
0x14173815e 41b8c0010000 mov r8d, 0x1c0
0x141738164 4889c1 mov rcx, rax
0x141738167 e894ba1e01 call 0x142923c00
0x14173816c 4c8d055d505901 lea r8, [rip + 0x159505d]
0x141738173 488d8d68010000 lea rcx, [rbp + 0x168]
0x14173817a 4889fa mov rdx, rdi
0x14173817d e88ec95aff call 0x140ce4b10
0x141738182 488d8df8000000 lea rcx, [rbp + 0xf8]
0x141738189 e88234d0ff call 0x14143b610
0x14173818e 488d8d88000000 lea rcx, [rbp + 0x88]
0x141738195 e8f630d0ff call 0x14143b290
0x14173819a 488d4d18 lea rcx, [rbp + 0x18]
0x14173819e e89d7036ff call 0x140a9f240
0x1417381a3 488d4da8 lea rcx, [rbp - 0x58]
0x1417381a7 e8b4820000 call 0x141740460
0x1417381ac 488d8dd8010000 lea rcx, [rbp + 0x1d8]
0x1417381b3 e888e02eff call 0x140a26240
0x1417381b8 0f1085c8010000 movups xmm0, xmmword ptr [rbp + 0x1c8]
0x1417381bf 488b8598030000 mov rax, qword ptr [rbp + 0x398]
0x1417381c6 0f114060 movups xmmword ptr [rax + 0x60], xmm0
0x1417381ca 0f1085b8010000 movups xmm0, xmmword ptr [rbp + 0x1b8]
0x1417381d1 0f114050 movups xmmword ptr [rax + 0x50], xmm0
0x1417381d5 0f1085a8010000 movups xmm0, xmmword ptr [rbp + 0x1a8]
0x1417381dc 0f114040 movups xmmword ptr [rax + 0x40], xmm0
0x1417381e0 0f108568010000 movups xmm0, xmmword ptr [rbp + 0x168]
0x1417381e7 0f108d78010000 movups xmm1, xmmword ptr [rbp + 0x178]
0x1417381ee 0f109588010000 movups xmm2, xmmword ptr [rbp + 0x188]
0x1417381f5 0f109d98010000 movups xmm3, xmmword ptr [rbp + 0x198]
0x1417381fc 0f115830 movups xmmword ptr [rax + 0x30], xmm3
0x141738200 0f115020 movups xmmword ptr [rax + 0x20], xmm2
0x141738204 0f114810 movups xmmword ptr [rax + 0x10], xmm1
0x141738208 0f1100 movups xmmword ptr [rax], xmm0
0x14173820b 0f1085f8000000 movups xmm0, xmmword ptr [rbp + 0xf8]
0x141738212 0f108d08010000 movups xmm1, xmmword ptr [rbp + 0x108]
0x141738219 0f109518010000 movups xmm2, xmmword ptr [rbp + 0x118]
0x141738220 0f109d28010000 movups xmm3, xmmword ptr [rbp + 0x128]
0x141738227 0f114070 movups xmmword ptr [rax + 0x70], xmm0
0x14173822b 0f118880000000 movups xmmword ptr [rax + 0x80], xmm1
0x141738232 0f119090000000 movups xmmword ptr [rax + 0x90], xmm2
0x141738239 0f1198a0000000 movups xmmword ptr [rax + 0xa0], xmm3
0x141738240 0f108538010000 movups xmm0, xmmword ptr [rbp + 0x138]
0x141738247 0f1180b0000000 movups xmmword ptr [rax + 0xb0], xmm0
0x14173824e 0f108548010000 movups xmm0, xmmword ptr [rbp + 0x148]
0x141738255 0f1180c0000000 movups xmmword ptr [rax + 0xc0], xmm0
0x14173825c 0f108558010000 movups xmm0, xmmword ptr [rbp + 0x158]
0x141738263 0f1180d0000000 movups xmmword ptr [rax + 0xd0], xmm0
0x14173826a 0f1085e8000000 movups xmm0, xmmword ptr [rbp + 0xe8]
0x141738271 0f118040010000 movups xmmword ptr [rax + 0x140], xmm0
0x141738278 0f1085d8000000 movups xmm0, xmmword ptr [rbp + 0xd8]
0x14173827f 0f118030010000 movups xmmword ptr [rax + 0x130], xmm0
0x141738286 0f1085c8000000 movups xmm0, xmmword ptr [rbp + 0xc8]
0x14173828d 0f118020010000 movups xmmword ptr [rax + 0x120], xmm0
0x141738294 0f108588000000 movups xmm0, xmmword ptr [rbp + 0x88]
0x14173829b 0f108d98000000 movups xmm1, xmmword ptr [rbp + 0x98]
0x1417382a2 0f1095a8000000 movups xmm2, xmmword ptr [rbp + 0xa8]
0x1417382a9 0f109db8000000 movups xmm3, xmmword ptr [rbp + 0xb8]
0x1417382b0 0f119810010000 movups xmmword ptr [rax + 0x110], xmm3
0x1417382b7 0f119000010000 movups xmmword ptr [rax + 0x100], xmm2
0x1417382be 0f1188f0000000 movups xmmword ptr [rax + 0xf0], xmm1
0x1417382c5 0f1180e0000000 movups xmmword ptr [rax + 0xe0], xmm0
0x1417382cc 0f104518 movups xmm0, xmmword ptr [rbp + 0x18]
0x1417382d0 0f104d28 movups xmm1, xmmword ptr [rbp + 0x28]
0x1417382d4 0f105538 movups xmm2, xmmword ptr [rbp + 0x38]
0x1417382d8 0f105d48 movups xmm3, xmmword ptr [rbp + 0x48]
0x1417382dc 0f118050010000 movups xmmword ptr [rax + 0x150], xmm0
0x1417382e3 0f118860010000 movups xmmword ptr [rax + 0x160], xmm1
0x1417382ea 0f119070010000 movups xmmword ptr [rax + 0x170], xmm2
0x1417382f1 0f119880010000 movups xmmword ptr [rax + 0x180], xmm3
0x1417382f8 0f104558 movups xmm0, xmmword ptr [rbp + 0x58]
0x1417382fc 0f118090010000 movups xmmword ptr [rax + 0x190], xmm0
0x141738303 0f104568 movups xmm0, xmmword ptr [rbp + 0x68]
0x141738307 0f1180a0010000 movups xmmword ptr [rax + 0x1a0], xmm0
0x14173830e 0f104578 movups xmm0, xmmword ptr [rbp + 0x78]
0x141738312 0f1180b0010000 movups xmmword ptr [rax + 0x1b0], xmm0
0x141738319 0f1045a8 movups xmm0, xmmword ptr [rbp - 0x58]
0x14173831d 0f104db8 movups xmm1, xmmword ptr [rbp - 0x48]
0x141738321 0f1055c8 movups xmm2, xmmword ptr [rbp - 0x38]
0x141738325 0f105dd8 movups xmm3, xmmword ptr [rbp - 0x28]
0x141738329 0f1180c0010000 movups xmmword ptr [rax + 0x1c0], xmm0
0x141738330 0f1188d0010000 movups xmmword ptr [rax + 0x1d0], xmm1
0x141738337 0f1190e0010000 movups xmmword ptr [rax + 0x1e0], xmm2
0x14173833e 0f1198f0010000 movups xmmword ptr [rax + 0x1f0], xmm3
0x141738345 0f1045e8 movups xmm0, xmmword ptr [rbp - 0x18]
0x141738349 0f118000020000 movups xmmword ptr [rax + 0x200], xmm0
0x141738350 0f104508 movups xmm0, xmmword ptr [rbp + 8]
0x141738354 0f118020020000 movups xmmword ptr [rax + 0x220], xmm0
0x14173835b 0f1045f8 movups xmm0, xmmword ptr [rbp - 8]
0x14173835f 0f118010020000 movups xmmword ptr [rax + 0x210], xmm0
0x141738366 0f108538020000 movups xmm0, xmmword ptr [rbp + 0x238]
0x14173836d 0f118090020000 movups xmmword ptr [rax + 0x290], xmm0
0x141738374 0f108528020000 movups xmm0, xmmword ptr [rbp + 0x228]
0x14173837b 0f118080020000 movups xmmword ptr [rax + 0x280], xmm0
0x141738382 0f108518020000 movups xmm0, xmmword ptr [rbp + 0x218]
0x141738389 0f118070020000 movups xmmword ptr [rax + 0x270], xmm0
0x141738390 0f1085d8010000 movups xmm0, xmmword ptr [rbp + 0x1d8]
0x141738397 0f108de8010000 movups xmm1, xmmword ptr [rbp + 0x1e8]
0x14173839e 0f1095f8010000 movups xmm2, xmmword ptr [rbp + 0x1f8]
0x1417383a5 0f109d08020000 movups xmm3, xmmword ptr [rbp + 0x208]
0x1417383ac 0f119860020000 movups xmmword ptr [rax + 0x260], xmm3
0x1417383b3 0f119050020000 movups xmmword ptr [rax + 0x250], xmm2
0x1417383ba 0f118030020000 movups xmmword ptr [rax + 0x230], xmm0
0x1417383c1 0f118840020000 movups xmmword ptr [rax + 0x240], xmm1
0x1417383c8 48c785e001000006000000 mov qword ptr [rbp + 0x1e0], 6
0x1417383d3 488985e8010000 mov qword ptr [rbp + 0x1e8], rax
0x1417383da 48c785f001000006000000 mov qword ptr [rbp + 0x1f0], 6
0x1417383e5 48c785f801000000000000 mov qword ptr [rbp + 0x1f8], 0
0x1417383f0 48c7850002000008000000 mov qword ptr [rbp + 0x200], 8
0x1417383fb 0f57c0 xorps xmm0, xmm0
0x1417383fe 0f118508020000 movups xmmword ptr [rbp + 0x208], xmm0
0x141738405 48899dd8010000 mov qword ptr [rbp + 0x1d8], rbx
0x14173840c 488d8d10020000 lea rcx, [rbp + 0x210]
0x141738413 e8887cf6ff call 0x1416a00a0
0x141738418 0f108538020000 movups xmm0, xmmword ptr [rbp + 0x238]
0x14173841f 0f114660 movups xmmword ptr [rsi + 0x60], xmm0
0x141738423 0f108528020000 movups xmm0, xmmword ptr [rbp + 0x228]
0x14173842a 0f114650 movups xmmword ptr [rsi + 0x50], xmm0
0x14173842e 0f108518020000 movups xmm0, xmmword ptr [rbp + 0x218]
0x141738435 0f114640 movups xmmword ptr [rsi + 0x40], xmm0
0x141738439 0f1085d8010000 movups xmm0, xmmword ptr [rbp + 0x1d8]
0x141738440 0f108de8010000 movups xmm1, xmmword ptr [rbp + 0x1e8]
0x141738447 0f1095f8010000 movups xmm2, xmmword ptr [rbp + 0x1f8]
0x14173844e 0f109d08020000 movups xmm3, xmmword ptr [rbp + 0x208]
0x141738455 0f115e30 movups xmmword ptr [rsi + 0x30], xmm3
0x141738459 0f115620 movups xmmword ptr [rsi + 0x20], xmm2
0x14173845d 0f114e10 movups xmmword ptr [rsi + 0x10], xmm1
0x141738461 0f1106 movups xmmword ptr [rsi], xmm0
0x141738464 4889f0 mov rax, rsi
0x141738467 4881c428040000 add rsp, 0x428
0x14173846e 5b pop rbx
0x14173846f 5f pop rdi
0x141738470 5e pop rsi
0x141738471 5d pop rbp
0x141738472 c3 ret 
0x141738473 b908000000 mov ecx, 8
0x141738478 baa0020000 mov edx, 0x2a0
0x14173847d e87e0b1a01 call 0x1428d9000
0x141738482 b908000000 mov ecx, 8
0x141738487 bac0010000 mov edx, 0x1c0
0x14173848c e86f0b1a01 call 0x1428d9000
0x141738491 0f0b ud2 
0x141738493 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1417384a0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1417384a5 55 push rbp
0x1417384a6 56 push rsi
0x1417384a7 57 push rdi
0x1417384a8 53 push rbx
0x1417384a9 4883ec28 sub rsp, 0x28
0x1417384ad 488daa80000000 lea rbp, [rdx + 0x80]
0x1417384b4 488d8dd8010000 lea rcx, [rbp + 0x1d8]
0x1417384bb e81055eafe call 0x1405dd9d0
0x1417384c0 90 nop 
0x1417384c1 4883c428 add rsp, 0x28
0x1417384c5 5b pop rbx
0x1417384c6 5f pop rdi
0x1417384c7 5e pop rsi
0x1417384c8 5d pop rbp
0x1417384c9 c3 ret 
0x1417384ca 660f1f440000 nop word ptr [rax + rax]
0x1417384d0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1417384d5 55 push rbp
0x1417384d6 56 push rsi
0x1417384d7 57 push rdi
0x1417384d8 53 push rbx
0x1417384d9 4883ec28 sub rsp, 0x28
0x1417384dd 488daa80000000 lea rbp, [rdx + 0x80]
0x1417384e4 baa0020000 mov edx, 0x2a0
0x1417384e9 41b808000000 mov r8d, 8
0x1417384ef 488b8d98030000 mov rcx, qword ptr [rbp + 0x398]
0x1417384f6 e825b7edfe call 0x140613c20
0x1417384fb 90 nop 
0x1417384fc 4883c428 add rsp, 0x28
0x141738500 5b pop rbx
0x141738501 5f pop rdi
0x141738502 5e pop rsi
0x141738503 5d pop rbp
0x141738504 c3 ret 
0x141738505 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x141738510 4889542410 mov qword ptr [rsp + 0x10], rdx
0x141738515 55 push rbp
0x141738516 56 push rsi
0x141738517 57 push rdi
0x141738518 53 push rbx
0x141738519 4883ec28 sub rsp, 0x28
0x14173851d 488daa80000000 lea rbp, [rdx + 0x80]
0x141738524 488d8d68010000 lea rcx, [rbp + 0x168]
0x14173852b e850138ffe call 0x140029880
0x141738530 90 nop 
0x141738531 4883c428 add rsp, 0x28
0x141738535 5b pop rbx
0x141738536 5f pop rdi
0x141738537 5e pop rsi
0x141738538 5d pop rbp
0x141738539 c3 ret 
0x14173853a 660f1f440000 nop word ptr [rax + rax]
0x141738540 4889542410 mov qword ptr [rsp + 0x10], rdx
0x141738545 55 push rbp
0x141738546 56 push rsi
0x141738547 57 push rdi
0x141738548 53 push rbx
0x141738549 4883ec28 sub rsp, 0x28
0x14173854d 488daa80000000 lea rbp, [rdx + 0x80]
0x141738554 488d8df8000000 lea rcx, [rbp + 0xf8]
0x14173855b e820138ffe call 0x140029880
0x141738560 90 nop 
0x141738561 4883c428 add rsp, 0x28
0x141738565 5b pop rbx
0x141738566 5f pop rdi
0x141738567 5e pop rsi
0x141738568 5d pop rbp
0x141738569 c3 ret 
0x14173856a 660f1f440000 nop word ptr [rax + rax]
0x141738570 4889542410 mov qword ptr [rsp + 0x10], rdx
0x141738575 55 push rbp
0x141738576 56 push rsi
0x141738577 57 push rdi
0x141738578 53 push rbx
0x141738579 4883ec28 sub rsp, 0x28
0x14173857d 488daa80000000 lea rbp, [rdx + 0x80]
0x141738584 488d8d88000000 lea rcx, [rbp + 0x88]
0x14173858b e8f0128ffe call 0x140029880
0x141738590 90 nop 
0x141738591 4883c428 add rsp, 0x28
0x141738595 5b pop rbx
0x141738596 5f pop rdi
0x141738597 5e pop rsi
0x141738598 5d pop rbp
0x141738599 c3 ret 
0x14173859a 660f1f440000 nop word ptr [rax + rax]
0x1417385a0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1417385a5 55 push rbp
0x1417385a6 56 push rsi
0x1417385a7 57 push rdi
0x1417385a8 53 push rbx
0x1417385a9 4883ec28 sub rsp, 0x28
0x1417385ad 488daa80000000 lea rbp, [rdx + 0x80]
0x1417385b4 488d4d18 lea rcx, [rbp + 0x18]
0x1417385b8 e8c3128ffe call 0x140029880
0x1417385bd 90 nop 
0x1417385be 4883c428 add rsp, 0x28
0x1417385c2 5b pop rbx
0x1417385c3 5f pop rdi
0x1417385c4 5e pop rsi
0x1417385c5 5d pop rbp
0x1417385c6 c3 ret 
0x1417385c7 660f1f840000000000 nop word ptr [rax + rax]
0x1417385d0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1417385d5 55 push rbp
0x1417385d6 56 push rsi
0x1417385d7 57 push rdi
0x1417385d8 53 push rbx
0x1417385d9 4883ec28 sub rsp, 0x28
0x1417385dd 488daa80000000 lea rbp, [rdx + 0x80]
0x1417385e4 488d4da8 lea rcx, [rbp - 0x58]
0x1417385e8 e893128ffe call 0x140029880
0x1417385ed 90 nop 
0x1417385ee 4883c428 add rsp, 0x28
0x1417385f2 5b pop rbx
0x1417385f3 5f pop rdi
0x1417385f4 5e pop rsi
0x1417385f5 5d pop rbp
0x1417385f6 c3 ret 
0x1417385f7 660f1f840000000000 nop word ptr [rax + rax]
0x141738600 4889542410 mov qword ptr [rsp + 0x10], rdx
0x141738605 55 push rbp
0x141738606 56 push rsi
0x141738607 57 push rdi
0x141738608 53 push rbx
0x141738609 4883ec28 sub rsp, 0x28
0x14173860d 488daa80000000 lea rbp, [rdx + 0x80]
0x141738614 488d8dd8010000 lea rcx, [rbp + 0x1d8]
0x14173861b e860128ffe call 0x140029880
0x141738620 90 nop 
0x141738621 4883c428 add rsp, 0x28
0x141738625 5b pop rbx
0x141738626 5f pop rdi
0x141738627 5e pop rsi
0x141738628 5d pop rbp
0x141738629 c3 ret 
