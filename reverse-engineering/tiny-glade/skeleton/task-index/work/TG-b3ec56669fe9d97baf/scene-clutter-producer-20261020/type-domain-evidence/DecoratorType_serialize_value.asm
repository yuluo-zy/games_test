0x140a47200 55 push rbp
0x140a47201 56 push rsi
0x140a47202 57 push rdi
0x140a47203 4881ecd0000000 sub rsp, 0xd0
0x140a4720a 488dac2480000000 lea rbp, [rsp + 0x80]
0x140a47212 48c74548feffffff mov qword ptr [rbp + 0x48], 0xfffffffffffffffe
0x140a4721a 4889ce mov rsi, rcx
0x140a4721d 0fb602 movzx eax, byte ptr [rdx]
0x140a47220 04c8 add al, 0xc8
0x140a47222 3c08 cmp al, 8
0x140a47224 0fb6c0 movzx eax, al
0x140a47227 b906000000 mov ecx, 6
0x140a4722c 0f42c8 cmovb ecx, eax
0x140a4722f 0fb6c1 movzx eax, cl
0x140a47232 488d0dcf840802 lea rcx, [rip + 0x20884cf]
0x140a47239 48630481 movsxd rax, dword ptr [rcx + rax*4]
0x140a4723d 4801c8 add rax, rcx
0x140a47240 ffe0 jmp rax
0x140a47242 0fb605c0d9be02 movzx eax, byte ptr [rip + 0x2bed9c0]
0x140a47249 b90d000000 mov ecx, 0xd
0x140a4724e ba01000000 mov edx, 1
0x140a47253 e8b8c9bcff call 0x140613c10
0x140a47258 4885c0 test rax, rax
0x140a4725b 0f844d030000 je 0x140a475ae
0x140a47261 48b9676557696e646f77 movabs rcx, 0x776f646e69576567
0x140a4726b 48894805 mov qword ptr [rax + 5], rcx
0x140a4726f 48b9436f747461676557 movabs rcx, 0x5765676174746f43
0x140a47279 488908 mov qword ptr [rax], rcx
0x140a4727c c60603 mov byte ptr [rsi], 3
0x140a4727f 48c746080d000000 mov qword ptr [rsi + 8], 0xd
0x140a47287 48894610 mov qword ptr [rsi + 0x10], rax
0x140a4728b 48c746180d000000 mov qword ptr [rsi + 0x18], 0xd
0x140a47293 e9c6020000 jmp 0x140a4755e
0x140a47298 0fb6056ad9be02 movzx eax, byte ptr [rip + 0x2bed96a]
0x140a4729f b907000000 mov ecx, 7
0x140a472a4 ba01000000 mov edx, 1
0x140a472a9 e862c9bcff call 0x140613c10
0x140a472ae 4885c0 test rax, rax
0x140a472b1 0f84b5020000 je 0x140a4756c
0x140a472b7 c740037465726e mov dword ptr [rax + 3], 0x6e726574
0x140a472be c7004c616e74 mov dword ptr [rax], 0x746e614c
0x140a472c4 eb78 jmp 0x140a4733e
0x140a472c6 0fb6053cd9be02 movzx eax, byte ptr [rip + 0x2bed93c]
0x140a472cd b909000000 mov ecx, 9
0x140a472d2 ba01000000 mov edx, 1
0x140a472d7 e834c9bcff call 0x140613c10
0x140a472dc 4885c0 test rax, rax
0x140a472df 0f849d020000 je 0x140a47582
0x140a472e5 48b94172726f77536c69 movabs rcx, 0x696c53776f727241
0x140a472ef 488908 mov qword ptr [rax], rcx
0x140a472f2 c6400874 mov byte ptr [rax + 8], 0x74
0x140a472f6 c60603 mov byte ptr [rsi], 3
0x140a472f9 48c7460809000000 mov qword ptr [rsi + 8], 9
0x140a47301 48894610 mov qword ptr [rsi + 0x10], rax
0x140a47305 48c7461809000000 mov qword ptr [rsi + 0x18], 9
0x140a4730d e94c020000 jmp 0x140a4755e
0x140a47312 0fb605f0d8be02 movzx eax, byte ptr [rip + 0x2bed8f0]
0x140a47319 b907000000 mov ecx, 7
0x140a4731e ba01000000 mov edx, 1
0x140a47323 e8e8c8bcff call 0x140613c10
0x140a47328 4885c0 test rax, rax
0x140a4732b 0f843b020000 je 0x140a4756c
0x140a47331 c740036d6e6579 mov dword ptr [rax + 3], 0x79656e6d
0x140a47338 c7004368696d mov dword ptr [rax], 0x6d696843
0x140a4733e c60603 mov byte ptr [rsi], 3
0x140a47341 48c7460807000000 mov qword ptr [rsi + 8], 7
0x140a47349 48894610 mov qword ptr [rsi + 0x10], rax
0x140a4734d 48c7461807000000 mov qword ptr [rsi + 0x18], 7
0x140a47355 e904020000 jmp 0x140a4755e
0x140a4735a 0fb605a8d8be02 movzx eax, byte ptr [rip + 0x2bed8a8]
0x140a47361 b906000000 mov ecx, 6
0x140a47366 ba01000000 mov edx, 1
0x140a4736b e8a0c8bcff call 0x140613c10
0x140a47370 4885c0 test rax, rax
0x140a47373 0f841f020000 je 0x140a47598
0x140a47379 66c740047273 mov word ptr [rax + 4], 0x7372
0x140a4737f c70053746169 mov dword ptr [rax], 0x69617453
0x140a47385 c60603 mov byte ptr [rsi], 3
0x140a47388 48c7460806000000 mov qword ptr [rsi + 8], 6
0x140a47390 48894610 mov qword ptr [rsi + 0x10], rax
0x140a47394 48c7461806000000 mov qword ptr [rsi + 0x18], 6
0x140a4739c e9bd010000 jmp 0x140a4755e
0x140a473a1 0fb60561d8be02 movzx eax, byte ptr [rip + 0x2bed861]
0x140a473a8 b90c000000 mov ecx, 0xc
0x140a473ad ba01000000 mov edx, 1
0x140a473b2 e859c8bcff call 0x140613c10
0x140a473b7 4885c0 test rax, rax
0x140a473ba 0f8404020000 je 0x140a475c4
0x140a473c0 48b9476f746869635769 movabs rcx, 0x6957636968746f47
0x140a473ca 488908 mov qword ptr [rax], rcx
0x140a473cd c740086e646f77 mov dword ptr [rax + 8], 0x776f646e
0x140a473d4 c60603 mov byte ptr [rsi], 3
0x140a473d7 48c746080c000000 mov qword ptr [rsi + 8], 0xc
0x140a473df 48894610 mov qword ptr [rsi + 0x10], rax
0x140a473e3 48c746180c000000 mov qword ptr [rsi + 0x18], 0xc
0x140a473eb e96e010000 jmp 0x140a4755e
0x140a473f0 0fb60512d8be02 movzx eax, byte ptr [rip + 0x2bed812]
0x140a473f7 b904000000 mov ecx, 4
0x140a473fc ba01000000 mov edx, 1
0x140a47401 e80ac8bcff call 0x140613c10
0x140a47406 4885c0 test rax, rax
0x140a47409 0f84cb010000 je 0x140a475da
0x140a4740f c700466c6167 mov dword ptr [rax], 0x67616c46
0x140a47415 c60603 mov byte ptr [rsi], 3
0x140a47418 48c7460804000000 mov qword ptr [rsi + 8], 4
0x140a47420 48894610 mov qword ptr [rsi + 0x10], rax
0x140a47424 48c7461804000000 mov qword ptr [rsi + 0x18], 4
0x140a4742c e92d010000 jmp 0x140a4755e
0x140a47431 4889d7 mov rdi, rdx
0x140a47434 488d0545790802 lea rax, [rip + 0x2087945]
0x140a4743b 4889442420 mov qword ptr [rsp + 0x20], rax
0x140a47440 48c744243001000000 mov qword ptr [rsp + 0x30], 1
0x140a47449 48c744242807000000 mov qword ptr [rsp + 0x28], 7
0x140a47452 488d1536790802 lea rdx, [rip + 0x2087936]
0x140a47459 488d4dc0 lea rcx, [rbp - 0x40]
0x140a4745d 41b80d000000 mov r8d, 0xd
0x140a47463 41b906000000 mov r9d, 6
0x140a47469 e8c2ef1300 call 0x140b86430
0x140a4746e 488b4dc0 mov rcx, qword ptr [rbp - 0x40]
0x140a47472 488b45c8 mov rax, qword ptr [rbp - 0x38]
0x140a47476 4889ca mov rdx, rcx
0x140a47479 48f7da neg rdx
0x140a4747c 710c jno 0x140a4748a
0x140a4747e 48894608 mov qword ptr [rsi + 8], rax
0x140a47482 c60606 mov byte ptr [rsi], 6
0x140a47485 e9d4000000 jmp 0x140a4755e
0x140a4748a 0f1045d0 movups xmm0, xmmword ptr [rbp - 0x30]
0x140a4748e 0f104de0 movups xmm1, xmmword ptr [rbp - 0x20]
0x140a47492 0f114d30 movups xmmword ptr [rbp + 0x30], xmm1
0x140a47496 0f114520 movups xmmword ptr [rbp + 0x20], xmm0
0x140a4749a 48894d10 mov qword ptr [rbp + 0x10], rcx
0x140a4749e 48894518 mov qword ptr [rbp + 0x18], rax
0x140a474a2 c6454701 mov byte ptr [rbp + 0x47], 1
0x140a474a6 488d15da780802 lea rdx, [rip + 0x20878da]
0x140a474ad 488d4d10 lea rcx, [rbp + 0x10]
0x140a474b1 41b802000000 mov r8d, 2
0x140a474b7 4989f9 mov r9, rdi
0x140a474ba e8016fb400 call 0x14158e3c0
0x140a474bf 4885c0 test rax, rax
0x140a474c2 7457 je 0x140a4751b
0x140a474c4 48894608 mov qword ptr [rsi + 8], rax
0x140a474c8 c60606 mov byte ptr [rsi], 6
0x140a474cb 488b5510 mov rdx, qword ptr [rbp + 0x10]
0x140a474cf 4885d2 test rdx, rdx
0x140a474d2 740f je 0x140a474e3
0x140a474d4 488b4d18 mov rcx, qword ptr [rbp + 0x18]
0x140a474d8 41b801000000 mov r8d, 1
0x140a474de e83dc7bcff call 0x140613c20
0x140a474e3 488b4d28 mov rcx, qword ptr [rbp + 0x28]
0x140a474e7 4885c9 test rcx, rcx
0x140a474ea 7459 je 0x140a47545
0x140a474ec 488b5530 mov rdx, qword ptr [rbp + 0x30]
0x140a474f0 488b4538 mov rax, qword ptr [rbp + 0x38]
0x140a474f4 48c745c800000000 mov qword ptr [rbp - 0x38], 0
0x140a474fc 48894dd0 mov qword ptr [rbp - 0x30], rcx
0x140a47500 488955d8 mov qword ptr [rbp - 0x28], rdx
0x140a47504 48c745e800000000 mov qword ptr [rbp - 0x18], 0
0x140a4750c 48894df0 mov qword ptr [rbp - 0x10], rcx
0x140a47510 488955f8 mov qword ptr [rbp - 8], rdx
0x140a47514 b901000000 mov ecx, 1
0x140a47519 eb2e jmp 0x140a47549
0x140a4751b 0f104510 movups xmm0, xmmword ptr [rbp + 0x10]
0x140a4751f 0f104d20 movups xmm1, xmmword ptr [rbp + 0x20]
0x140a47523 0f105530 movups xmm2, xmmword ptr [rbp + 0x30]
0x140a47527 0f2955e0 movaps xmmword ptr [rbp - 0x20], xmm2
0x140a4752b 0f294dd0 movaps xmmword ptr [rbp - 0x30], xmm1
0x140a4752f 0f2945c0 movaps xmmword ptr [rbp - 0x40], xmm0
0x140a47533 c6454700 mov byte ptr [rbp + 0x47], 0
0x140a47537 488d55c0 lea rdx, [rbp - 0x40]
0x140a4753b 4889f1 mov rcx, rsi
0x140a4753e e8fdf01300 call 0x140b86640
0x140a47543 eb19 jmp 0x140a4755e
0x140a47545 31c9 xor ecx, ecx
0x140a47547 31c0 xor eax, eax
0x140a47549 48894dc0 mov qword ptr [rbp - 0x40], rcx
0x140a4754d 48894de0 mov qword ptr [rbp - 0x20], rcx
0x140a47551 48894500 mov qword ptr [rbp], rax
0x140a47555 488d4dc0 lea rcx, [rbp - 0x40]
0x140a47559 e86234e3ff call 0x14087a9c0
0x140a4755e 4889f0 mov rax, rsi
0x140a47561 4881c4d0000000 add rsp, 0xd0
0x140a47568 5f pop rdi
0x140a47569 5e pop rsi
0x140a4756a 5d pop rbp
0x140a4756b c3 ret 
0x140a4756c 4c8d0515710802 lea r8, [rip + 0x2087115]
0x140a47573 b901000000 mov ecx, 1
0x140a47578 ba07000000 mov edx, 7
0x140a4757d e8611ae901 call 0x1428d8fe3
0x140a47582 4c8d05ff700802 lea r8, [rip + 0x20870ff]
0x140a47589 b901000000 mov ecx, 1
0x140a4758e ba09000000 mov edx, 9
0x140a47593 e84b1ae901 call 0x1428d8fe3
0x140a47598 4c8d05e9700802 lea r8, [rip + 0x20870e9]
0x140a4759f b901000000 mov ecx, 1
0x140a475a4 ba06000000 mov edx, 6
0x140a475a9 e8351ae901 call 0x1428d8fe3
0x140a475ae 4c8d05d3700802 lea r8, [rip + 0x20870d3]
0x140a475b5 b901000000 mov ecx, 1
0x140a475ba ba0d000000 mov edx, 0xd
0x140a475bf e81f1ae901 call 0x1428d8fe3
0x140a475c4 4c8d05bd700802 lea r8, [rip + 0x20870bd]
0x140a475cb b901000000 mov ecx, 1
0x140a475d0 ba0c000000 mov edx, 0xc
0x140a475d5 e8091ae901 call 0x1428d8fe3
0x140a475da 4c8d05a7700802 lea r8, [rip + 0x20870a7]
0x140a475e1 b901000000 mov ecx, 1
0x140a475e6 ba04000000 mov edx, 4
0x140a475eb e8f319e901 call 0x1428d8fe3
0x140a475f0 cc int3 
0x140a475f1 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a47600 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a47605 55 push rbp
0x140a47606 56 push rsi
0x140a47607 57 push rdi
0x140a47608 4883ec40 sub rsp, 0x40
0x140a4760c 488daa80000000 lea rbp, [rdx + 0x80]
0x140a47613 807d4700 cmp byte ptr [rbp + 0x47], 0
0x140a47617 7508 jne 0x140a47621
0x140a47619 4883c440 add rsp, 0x40
0x140a4761d 5f pop rdi
0x140a4761e 5e pop rsi
0x140a4761f 5d pop rbp
0x140a47620 c3 ret 
0x140a47621 488d4d10 lea rcx, [rbp + 0x10]
0x140a47625 e80653e4ff call 0x14088c930
0x140a4762a ebed jmp 0x140a47619
