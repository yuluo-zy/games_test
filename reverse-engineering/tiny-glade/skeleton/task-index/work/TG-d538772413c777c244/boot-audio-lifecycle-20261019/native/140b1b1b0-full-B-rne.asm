140b1b1b0: 55 push rbp
140b1b1b1: 4156 push r14
140b1b1b3: 56 push rsi
140b1b1b4: 57 push rdi
140b1b1b5: 53 push rbx
140b1b1b6: 4881ec10010000 sub rsp, 0x110
140b1b1bd: 488dac2480000000 lea rbp, [rsp + 0x80]
140b1b1c5: 48c78588000000feffffff mov qword ptr [rbp + 0x88], 0xfffffffffffffffe
140b1b1d0: 4c89cf mov rdi, r9
140b1b1d3: 4889d6 mov rsi, rdx
140b1b1d6: 498b00 mov rax, qword ptr [r8]
140b1b1d9: 498b5008 mov rdx, qword ptr [r8 + 8]
140b1b1dd: 4c8b00 mov r8, qword ptr [rax]
140b1b1e0: 4531c9 xor r9d, r9d
140b1b1e3: 4d89c2 mov r10, r8
140b1b1e6: 4c2b5218 sub r10, qword ptr [rdx + 0x18]
140b1b1ea: 4d0f42d1 cmovb r10, r9
140b1b1ee: 4c2b4238 sub r8, qword ptr [rdx + 0x38]
140b1b1f2: 4d0f42c1 cmovb r8, r9
140b1b1f6: 4c8b5a30 mov r11, qword ptr [rdx + 0x30]
140b1b1fa: 4d29c3 sub r11, r8
140b1b1fd: 4d0f42d9 cmovb r11, r9
140b1b201: 410f96c0 setbe r8b
140b1b205: 488b5a10 mov rbx, qword ptr [rdx + 0x10]
140b1b209: 4c29d3 sub rbx, r10
140b1b20c: 490f42d9 cmovb rbx, r9
140b1b210: 488b5240 mov rdx, qword ptr [rdx + 0x40]
140b1b214: 410f96c1 setbe r9b
140b1b218: 4c01db add rbx, r11
140b1b21b: 4989d2 mov r10, rdx
140b1b21e: 4929da sub r10, rbx
140b1b221: 4c8910 mov qword ptr [rax], r10
140b1b224: 4584c1 test r9b, r8b
140b1b227: 0f8549010000 jne 0x140b1b376
140b1b22d: 488910 mov qword ptr [rax], rdx
140b1b230: 488b01 mov rax, qword ptr [rcx]
140b1b233: 488b10 mov rdx, qword ptr [rax]
140b1b236: 4885d2 test rdx, rdx
140b1b239: 745a je 0x140b1b295
140b1b23b: 4883c210 add rdx, 0x10
140b1b23f: 4c8d05cae2fc01 lea r8, [rip + 0x1fce2ca] ; 0x142ae9510 'event:/UI/Loading'
140b1b246: 488d4d10 lea rcx, [rbp + 0x10]
140b1b24a: 41b911000000 mov r9d, 0x11
140b1b250: e8eb91f3ff call 0x140a54440 ; country_core::resources::audio_resources::AudioCoreInner::instantiate_audio
140b1b255: 48837d1002 cmp qword ptr [rbp + 0x10], 2
140b1b25a: 0f8582000000 jne 0x140b1b2e2
140b1b260: 488b4518 mov rax, qword ptr [rbp + 0x18]
140b1b264: 488945d8 mov qword ptr [rbp - 0x28], rax
140b1b268: 488d05b9e2fc01 lea rax, [rip + 0x1fce2b9]
140b1b26f: 4889442420 mov qword ptr [rsp + 0x20], rax
140b1b274: 488d0d0ddefc01 lea rcx, [rip + 0x1fcde0d] ; 0x142ae9088 'called `Result::unwrap()` on an `Err` value'
140b1b27b: 4c8d0de6ddfc01 lea r9, [rip + 0x1fcdde6]
140b1b282: 4c8d45d8 lea r8, [rbp - 0x28]
140b1b286: ba2b000000 mov edx, 0x2b
140b1b28b: e8d0e4db01 call 0x1428d9760 ; core::result::unwrap_failed
140b1b290: e9af030000 jmp 0x140b1b644
140b1b295: 0fb6056d99b102 movzx eax, byte ptr [rip + 0x2b1996d]
140b1b29c: b911000000 mov ecx, 0x11
140b1b2a1: ba01000000 mov edx, 1
140b1b2a6: e86589afff call 0x140613c10 ; 
140b1b2ab: 4885c0 test rax, rax
140b1b2ae: 0f84e8020000 je 0x140b1b59c
140b1b2b4: 0f100555e2fc01 movups xmm0, xmmword ptr [rip + 0x1fce255] ; 0x142ae9510 'event:/UI/Loading'
140b1b2bb: 0f1100 movups xmmword ptr [rax], xmm0
140b1b2be: c6401067 mov byte ptr [rax + 0x10], 0x67
140b1b2c2: 48c7451000000000 mov qword ptr [rbp + 0x10], 0
140b1b2ca: 48c7452011000000 mov qword ptr [rbp + 0x20], 0x11
140b1b2d2: 48894528 mov qword ptr [rbp + 0x28], rax
140b1b2d6: 48c7453011000000 mov qword ptr [rbp + 0x30], 0x11
140b1b2de: c6453802 mov byte ptr [rbp + 0x38], 2
140b1b2e2: 488b4510 mov rax, qword ptr [rbp + 0x10]
140b1b2e6: 488b4d18 mov rcx, qword ptr [rbp + 0x18]
140b1b2ea: 488b5520 mov rdx, qword ptr [rbp + 0x20]
140b1b2ee: 4c8b4528 mov r8, qword ptr [rbp + 0x28]
140b1b2f2: 4c8b4d30 mov r9, qword ptr [rbp + 0x30]
140b1b2f6: 4c894d70 mov qword ptr [rbp + 0x70], r9
140b1b2fa: 440fb64d38 movzx r9d, byte ptr [rbp + 0x38]
140b1b2ff: 44884d78 mov byte ptr [rbp + 0x78], r9b
140b1b303: 448b4d39 mov r9d, dword ptr [rbp + 0x39]
140b1b307: 44894d79 mov dword ptr [rbp + 0x79], r9d
140b1b30b: 440fb74d3d movzx r9d, word ptr [rbp + 0x3d]
140b1b310: 6644894d7d mov word ptr [rbp + 0x7d], r9w
140b1b315: 440fb64d3f movzx r9d, byte ptr [rbp + 0x3f]
140b1b31a: 44884d7f mov byte ptr [rbp + 0x7f], r9b
140b1b31e: 48895560 mov qword ptr [rbp + 0x60], rdx
140b1b322: 4c894568 mov qword ptr [rbp + 0x68], r8
140b1b326: 48894550 mov qword ptr [rbp + 0x50], rax
140b1b32a: 48894d58 mov qword ptr [rbp + 0x58], rcx
140b1b32e: 837d5001 cmp dword ptr [rbp + 0x50], 1
140b1b332: 7521 jne 0x140b1b355
140b1b334: 488d5558 lea rdx, [rbp + 0x58]
140b1b338: 488d4dd8 lea rcx, [rbp - 0x28]
140b1b33c: e88fbf9100 call 0x1414372d0 ; libfmod::EventInstance::start
140b1b341: 48b80400000000000080 movabs rax, 0x8000000000000004
140b1b34b: 483945d8 cmp qword ptr [rbp - 0x28], rax
140b1b34f: 0f85fa010000 jne 0x140b1b54f
140b1b355: 488d4d50 lea rcx, [rbp + 0x50]
140b1b359: e822efd7ff call 0x14089a280 ; country_core::resources::audio_resources::impl$13::drop
140b1b35e: 488b5560 mov rdx, qword ptr [rbp + 0x60]
140b1b362: 4885d2 test rdx, rdx
140b1b365: 740f je 0x140b1b376
140b1b367: 488b4d68 mov rcx, qword ptr [rbp + 0x68]
140b1b36b: 41b801000000 mov r8d, 1
140b1b371: e8aa88afff call 0x140613c20 ; 
140b1b376: 488b07 mov rax, qword ptr [rdi]
140b1b379: 488b4f08 mov rcx, qword ptr [rdi + 8]
140b1b37d: 488b10 mov rdx, qword ptr [rax]
140b1b380: 4531c0 xor r8d, r8d
140b1b383: 4989d1 mov r9, rdx
140b1b386: 4c2b4918 sub r9, qword ptr [rcx + 0x18]
140b1b38a: 4d0f42c8 cmovb r9, r8
140b1b38e: 482b5138 sub rdx, qword ptr [rcx + 0x38]
140b1b392: 490f42d0 cmovb rdx, r8
140b1b396: 4c8b5130 mov r10, qword ptr [rcx + 0x30]
140b1b39a: 4929d2 sub r10, rdx
140b1b39d: 4d0f42d0 cmovb r10, r8
140b1b3a1: 0f97c2 seta dl
140b1b3a4: 4c8b5910 mov r11, qword ptr [rcx + 0x10]
140b1b3a8: 4d29cb sub r11, r9
140b1b3ab: 4d0f42d8 cmovb r11, r8
140b1b3af: 488b4940 mov rcx, qword ptr [rcx + 0x40]
140b1b3b3: 410f97c0 seta r8b
140b1b3b7: 4d01d3 add r11, r10
140b1b3ba: 4989c9 mov r9, rcx
140b1b3bd: 4d29d9 sub r9, r11
140b1b3c0: 4c8908 mov qword ptr [rax], r9
140b1b3c3: 4108d0 or r8b, dl
140b1b3c6: 750f jne 0x140b1b3d7
140b1b3c8: 8b7e1c mov edi, dword ptr [rsi + 0x1c]
140b1b3cb: 488b1e mov rbx, qword ptr [rsi]
140b1b3ce: 4c8b7610 mov r14, qword ptr [rsi + 0x10]
140b1b3d2: e99d000000 jmp 0x140b1b474
140b1b3d7: 488908 mov qword ptr [rax], rcx
140b1b3da: 8b7e1c mov edi, dword ptr [rsi + 0x1c]
140b1b3dd: 4c8b7610 mov r14, qword ptr [rsi + 0x10]
140b1b3e1: 41893e mov dword ptr [r14], edi
140b1b3e4: 488b1e mov rbx, qword ptr [rsi]
140b1b3e7: 488b03 mov rax, qword ptr [rbx]
140b1b3ea: 488b4b28 mov rcx, qword ptr [rbx + 0x28]
140b1b3ee: 48894dd0 mov qword ptr [rbp - 0x30], rcx
140b1b3f2: 0f104318 movups xmm0, xmmword ptr [rbx + 0x18]
140b1b3f6: 0f2945c0 movaps xmmword ptr [rbp - 0x40], xmm0
140b1b3fa: 0f104308 movups xmm0, xmmword ptr [rbx + 8]
140b1b3fe: 0f2945b0 movaps xmmword ptr [rbp - 0x50], xmm0
140b1b402: 48c70302000000 mov qword ptr [rbx], 2
140b1b409: 4883f802 cmp rax, 2
140b1b40d: 7465 je 0x140b1b474
140b1b40f: 48894550 mov qword ptr [rbp + 0x50], rax
140b1b413: 0f2845b0 movaps xmm0, xmmword ptr [rbp - 0x50]
140b1b417: 0f284dc0 movaps xmm1, xmmword ptr [rbp - 0x40]
140b1b41b: 0f114558 movups xmmword ptr [rbp + 0x58], xmm0
140b1b41f: 0f114d68 movups xmmword ptr [rbp + 0x68], xmm1
140b1b423: 488b4dd0 mov rcx, qword ptr [rbp - 0x30]
140b1b427: 48894d78 mov qword ptr [rbp + 0x78], rcx
140b1b42b: a801 test al, 1
140b1b42d: 7424 je 0x140b1b453
140b1b42f: 488d5558 lea rdx, [rbp + 0x58]
140b1b433: 488d4dd8 lea rcx, [rbp - 0x28]
140b1b437: 4531c0 xor r8d, r8d
140b1b43a: e801c09100 call 0x141437440 ; libfmod::EventInstance::stop
140b1b43f: 48b80400000000000080 movabs rax, 0x8000000000000004
140b1b449: 483945d8 cmp qword ptr [rbp - 0x28], rax
140b1b44d: 0f855f010000 jne 0x140b1b5b2
140b1b453: 488d4d50 lea rcx, [rbp + 0x50]
140b1b457: e824eed7ff call 0x14089a280 ; country_core::resources::audio_resources::impl$13::drop
140b1b45c: 488b5560 mov rdx, qword ptr [rbp + 0x60]
140b1b460: 4885d2 test rdx, rdx
140b1b463: 740f je 0x140b1b474
140b1b465: 488b4d68 mov rcx, qword ptr [rbp + 0x68]
140b1b469: 41b801000000 mov r8d, 1
140b1b46f: e8ac87afff call 0x140613c20 ; 
140b1b474: 41893e mov dword ptr [r14], edi
140b1b477: f6433001 test byte ptr [rbx + 0x30], 1
140b1b47b: 0f84c0000000 je 0x140b1b541
140b1b481: 488b85e0000000 mov rax, qword ptr [rbp + 0xe0]
140b1b488: 488b00 mov rax, qword ptr [rax]
140b1b48b: f30f104028 movss xmm0, dword ptr [rax + 0x28]
140b1b490: f30f584334 addss xmm0, dword ptr [rbx + 0x34]
140b1b495: f30f114334 movss dword ptr [rbx + 0x34], xmm0
140b1b49a: 0f2e0563a4e001 ucomiss xmm0, dword ptr [rip + 0x1e0a463]
140b1b4a1: 0f869a000000 jbe 0x140b1b541
140b1b4a7: 41893e mov dword ptr [r14], edi
140b1b4aa: 488b03 mov rax, qword ptr [rbx]
140b1b4ad: 488b4b28 mov rcx, qword ptr [rbx + 0x28]
140b1b4b1: 48894dd0 mov qword ptr [rbp - 0x30], rcx
140b1b4b5: 0f104318 movups xmm0, xmmword ptr [rbx + 0x18]
140b1b4b9: 0f2945c0 movaps xmmword ptr [rbp - 0x40], xmm0
140b1b4bd: 0f104308 movups xmm0, xmmword ptr [rbx + 8]
140b1b4c1: 0f2945b0 movaps xmmword ptr [rbp - 0x50], xmm0
140b1b4c5: 48c70302000000 mov qword ptr [rbx], 2
140b1b4cc: 4883f802 cmp rax, 2
140b1b4d0: 7465 je 0x140b1b537
140b1b4d2: 48894550 mov qword ptr [rbp + 0x50], rax
140b1b4d6: 0f2845b0 movaps xmm0, xmmword ptr [rbp - 0x50]
140b1b4da: 0f284dc0 movaps xmm1, xmmword ptr [rbp - 0x40]
140b1b4de: 0f114558 movups xmmword ptr [rbp + 0x58], xmm0
140b1b4e2: 0f114d68 movups xmmword ptr [rbp + 0x68], xmm1
140b1b4e6: 488b4dd0 mov rcx, qword ptr [rbp - 0x30]
140b1b4ea: 48894d78 mov qword ptr [rbp + 0x78], rcx
140b1b4ee: a801 test al, 1
140b1b4f0: 7424 je 0x140b1b516
140b1b4f2: 488d5558 lea rdx, [rbp + 0x58]
140b1b4f6: 488d4dd8 lea rcx, [rbp - 0x28]
140b1b4fa: 4531c0 xor r8d, r8d
140b1b4fd: e83ebf9100 call 0x141437440 ; libfmod::EventInstance::stop
140b1b502: 48b80400000000000080 movabs rax, 0x8000000000000004
140b1b50c: 483945d8 cmp qword ptr [rbp - 0x28], rax
140b1b510: 0f85e6000000 jne 0x140b1b5fc
140b1b516: 488d4d50 lea rcx, [rbp + 0x50]
140b1b51a: e861edd7ff call 0x14089a280 ; country_core::resources::audio_resources::impl$13::drop
140b1b51f: 488b5560 mov rdx, qword ptr [rbp + 0x60]
140b1b523: 4885d2 test rdx, rdx
140b1b526: 740f je 0x140b1b537
140b1b528: 488b4d68 mov rcx, qword ptr [rbp + 0x68]
140b1b52c: 41b801000000 mov r8d, 1
140b1b532: e8e986afff call 0x140613c20 ; 
140b1b537: 41893e mov dword ptr [r14], edi
140b1b53a: c7433000000000 mov dword ptr [rbx + 0x30], 0
140b1b541: 4881c410010000 add rsp, 0x110
140b1b548: 5b pop rbx
140b1b549: 5f pop rdi
140b1b54a: 5e pop rsi
140b1b54b: 415e pop r14
140b1b54d: 5d pop rbp
140b1b54e: c3 ret 
140b1b54f: 488b4508 mov rax, qword ptr [rbp + 8]
140b1b553: 48894540 mov qword ptr [rbp + 0x40], rax
140b1b557: 0f1045d8 movups xmm0, xmmword ptr [rbp - 0x28]
140b1b55b: 0f104de8 movups xmm1, xmmword ptr [rbp - 0x18]
140b1b55f: 0f1055f8 movups xmm2, xmmword ptr [rbp - 8]
140b1b563: 0f295530 movaps xmmword ptr [rbp + 0x30], xmm2
140b1b567: 0f294d20 movaps xmmword ptr [rbp + 0x20], xmm1
140b1b56b: 0f294510 movaps xmmword ptr [rbp + 0x10], xmm0
140b1b56f: 488d054a76fb01 lea rax, [rip + 0x1fb764a]
140b1b576: 4889442420 mov qword ptr [rsp + 0x20], rax
140b1b57b: 488d0d7e50fb01 lea rcx, [rip + 0x1fb507e] ; 0x142ad0600 'called `Result::unwrap()` on an `Err` value'
140b1b582: 4c8d0d3750fb01 lea r9, [rip + 0x1fb5037]
140b1b589: 4c8d4510 lea r8, [rbp + 0x10]
140b1b58d: ba2b000000 mov edx, 0x2b
140b1b592: e8c9e1db01 call 0x1428d9760 ; core::result::unwrap_failed
140b1b597: e9a8000000 jmp 0x140b1b644
140b1b59c: 4c8d050d52fb01 lea r8, [rip + 0x1fb520d]
140b1b5a3: b901000000 mov ecx, 1
140b1b5a8: ba11000000 mov edx, 0x11
140b1b5ad: e831dadb01 call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140b1b5b2: 488b4508 mov rax, qword ptr [rbp + 8]
140b1b5b6: 48894540 mov qword ptr [rbp + 0x40], rax
140b1b5ba: 0f1045d8 movups xmm0, xmmword ptr [rbp - 0x28]
140b1b5be: 0f104de8 movups xmm1, xmmword ptr [rbp - 0x18]
140b1b5c2: 0f1055f8 movups xmm2, xmmword ptr [rbp - 8]
140b1b5c6: 0f295530 movaps xmmword ptr [rbp + 0x30], xmm2
140b1b5ca: 0f294d20 movaps xmmword ptr [rbp + 0x20], xmm1
140b1b5ce: 0f294510 movaps xmmword ptr [rbp + 0x10], xmm0
140b1b5d2: 488d052f76fb01 lea rax, [rip + 0x1fb762f]
140b1b5d9: 4889442420 mov qword ptr [rsp + 0x20], rax
140b1b5de: 488d0d1b50fb01 lea rcx, [rip + 0x1fb501b] ; 0x142ad0600 'called `Result::unwrap()` on an `Err` value'
140b1b5e5: 4c8d0dd44ffb01 lea r9, [rip + 0x1fb4fd4]
140b1b5ec: 4c8d4510 lea r8, [rbp + 0x10]
140b1b5f0: ba2b000000 mov edx, 0x2b
140b1b5f5: e866e1db01 call 0x1428d9760 ; core::result::unwrap_failed
140b1b5fa: eb48 jmp 0x140b1b644
140b1b5fc: 488b4508 mov rax, qword ptr [rbp + 8]
140b1b600: 48894540 mov qword ptr [rbp + 0x40], rax
140b1b604: 0f1045d8 movups xmm0, xmmword ptr [rbp - 0x28]
140b1b608: 0f104de8 movups xmm1, xmmword ptr [rbp - 0x18]
140b1b60c: 0f1055f8 movups xmm2, xmmword ptr [rbp - 8]
140b1b610: 0f295530 movaps xmmword ptr [rbp + 0x30], xmm2
140b1b614: 0f294d20 movaps xmmword ptr [rbp + 0x20], xmm1
140b1b618: 0f294510 movaps xmmword ptr [rbp + 0x10], xmm0
140b1b61c: 488d05e575fb01 lea rax, [rip + 0x1fb75e5]
140b1b623: 4889442420 mov qword ptr [rsp + 0x20], rax
140b1b628: 488d0dd14ffb01 lea rcx, [rip + 0x1fb4fd1] ; 0x142ad0600 'called `Result::unwrap()` on an `Err` value'
140b1b62f: 4c8d0d8a4ffb01 lea r9, [rip + 0x1fb4f8a]
140b1b636: 4c8d4510 lea r8, [rbp + 0x10]
140b1b63a: ba2b000000 mov edx, 0x2b
140b1b63f: e81ce1db01 call 0x1428d9760 ; core::result::unwrap_failed
140b1b644: 0f0b ud2 
140b1b646: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
140b1b650: 4889542410 mov qword ptr [rsp + 0x10], rdx
140b1b655: 55 push rbp
140b1b656: 4156 push r14
140b1b658: 56 push rsi
140b1b659: 57 push rdi
140b1b65a: 53 push rbx
140b1b65b: 4883ec30 sub rsp, 0x30
140b1b65f: 488daa80000000 lea rbp, [rdx + 0x80]
140b1b666: 488d4d10 lea rcx, [rbp + 0x10]
140b1b66a: e84170f3ff call 0x140a526b0 ; core::ptr::drop_in_place<enum2$<libfmod::Error> >
140b1b66f: 90 nop 
140b1b670: 4883c430 add rsp, 0x30
140b1b674: 5b pop rbx
140b1b675: 5f pop rdi
140b1b676: 5e pop rsi
140b1b677: 415e pop r14
140b1b679: 5d pop rbp
140b1b67a: c3 ret 
140b1b67b: 0f1f440000 nop dword ptr [rax + rax]
140b1b680: 4889542410 mov qword ptr [rsp + 0x10], rdx
140b1b685: 55 push rbp
140b1b686: 4156 push r14
140b1b688: 56 push rsi
140b1b689: 57 push rdi
140b1b68a: 53 push rbx
140b1b68b: 4883ec30 sub rsp, 0x30
140b1b68f: 488daa80000000 lea rbp, [rdx + 0x80]
140b1b696: 488d4d50 lea rcx, [rbp + 0x50]
140b1b69a: e8e1ef62ff call 0x14014a680 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::critters::sheep::SheepAudio> | core::ptr::drop_in_place<tiny_glade::rollercoaster::CoasterAudioInstance> | core::ptr::drop_in_place<country_core::resources::audio_resources::AudioInstance> | core::ptr::drop_in_place<country_core::resources::critters::ButterflyAudio> | core::ptr::drop_in_place<country_core::resources::critters::sheep::SheepAudio> | core::ptr::drop_in_place<country_core::resources::audio_resources::AudioInstance> | bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::critters::ButterflyAudio> | core::ptr::drop_in_place<country_core::resources::critters::Wren> | core::ptr::drop_in_place<country_core::resources::critters::ButterflyAudio> | bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::critters::Wren>
140b1b69f: 90 nop 
140b1b6a0: 4883c430 add rsp, 0x30
140b1b6a4: 5b pop rbx
140b1b6a5: 5f pop rdi
140b1b6a6: 5e pop rsi
140b1b6a7: 415e pop r14
140b1b6a9: 5d pop rbp
140b1b6aa: c3 ret 
140b1b6ab: 0f1f440000 nop dword ptr [rax + rax]
140b1b6b0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140b1b6b5: 55 push rbp
140b1b6b6: 4156 push r14
140b1b6b8: 56 push rsi
140b1b6b9: 57 push rdi
140b1b6ba: 53 push rbx
140b1b6bb: 4883ec30 sub rsp, 0x30
140b1b6bf: 488daa80000000 lea rbp, [rdx + 0x80]
140b1b6c6: 488b5560 mov rdx, qword ptr [rbp + 0x60]
140b1b6ca: 4885d2 test rdx, rdx
140b1b6cd: 740f je 0x140b1b6de
140b1b6cf: 488b4d68 mov rcx, qword ptr [rbp + 0x68]
140b1b6d3: 41b801000000 mov r8d, 1
140b1b6d9: e84285afff call 0x140613c20 ; 
140b1b6de: 90 nop 
140b1b6df: 4883c430 add rsp, 0x30
140b1b6e3: 5b pop rbx
140b1b6e4: 5f pop rdi
140b1b6e5: 5e pop rsi
140b1b6e6: 415e pop r14
140b1b6e8: 5d pop rbp
140b1b6e9: c3 ret 
140b1b6ea: 660f1f440000 nop word ptr [rax + rax]
140b1b6f0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140b1b6f5: 55 push rbp
140b1b6f6: 4156 push r14
140b1b6f8: 56 push rsi
140b1b6f9: 57 push rdi
140b1b6fa: 53 push rbx
140b1b6fb: 4883ec30 sub rsp, 0x30
140b1b6ff: 488daa80000000 lea rbp, [rdx + 0x80]
140b1b706: 488d4d50 lea rcx, [rbp + 0x50]
140b1b70a: e871ef62ff call 0x14014a680 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::critters::sheep::SheepAudio> | core::ptr::drop_in_place<tiny_glade::rollercoaster::CoasterAudioInstance> | core::ptr::drop_in_place<country_core::resources::audio_resources::AudioInstance> | core::ptr::drop_in_place<country_core::resources::critters::ButterflyAudio> | core::ptr::drop_in_place<country_core::resources::critters::sheep::SheepAudio> | core::ptr::drop_in_place<country_core::resources::audio_resources::AudioInstance> | bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::critters::ButterflyAudio> | core::ptr::drop_in_place<country_core::resources::critters::Wren> | core::ptr::drop_in_place<country_core::resources::critters::ButterflyAudio> | bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::critters::Wren>
140b1b70f: 90 nop 
140b1b710: 4883c430 add rsp, 0x30
140b1b714: 5b pop rbx
140b1b715: 5f pop rdi
140b1b716: 5e pop rsi
140b1b717: 415e pop r14
140b1b719: 5d pop rbp
140b1b71a: c3 ret 
140b1b71b: 0f1f440000 nop dword ptr [rax + rax]
140b1b720: 4889542410 mov qword ptr [rsp + 0x10], rdx
140b1b725: 55 push rbp
140b1b726: 4156 push r14
140b1b728: 56 push rsi
140b1b729: 57 push rdi
140b1b72a: 53 push rbx
140b1b72b: 4883ec30 sub rsp, 0x30
140b1b72f: 488daa80000000 lea rbp, [rdx + 0x80]
140b1b736: 488b5560 mov rdx, qword ptr [rbp + 0x60]
140b1b73a: 4885d2 test rdx, rdx
140b1b73d: 740f je 0x140b1b74e
140b1b73f: 488b4d68 mov rcx, qword ptr [rbp + 0x68]
140b1b743: 41b801000000 mov r8d, 1
140b1b749: e8d284afff call 0x140613c20 ; 
140b1b74e: 90 nop 
140b1b74f: 4883c430 add rsp, 0x30
140b1b753: 5b pop rbx
140b1b754: 5f pop rdi
140b1b755: 5e pop rsi
140b1b756: 415e pop r14
140b1b758: 5d pop rbp
140b1b759: c3 ret 
140b1b75a: 660f1f440000 nop word ptr [rax + rax]
140b1b760: 4889542410 mov qword ptr [rsp + 0x10], rdx
140b1b765: 55 push rbp
140b1b766: 4156 push r14
140b1b768: 56 push rsi
140b1b769: 57 push rdi
140b1b76a: 53 push rbx
140b1b76b: 4883ec30 sub rsp, 0x30
140b1b76f: 488daa80000000 lea rbp, [rdx + 0x80]
140b1b776: 488d4d50 lea rcx, [rbp + 0x50]
140b1b77a: e801ef62ff call 0x14014a680 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::critters::sheep::SheepAudio> | core::ptr::drop_in_place<tiny_glade::rollercoaster::CoasterAudioInstance> | core::ptr::drop_in_place<country_core::resources::audio_resources::AudioInstance> | core::ptr::drop_in_place<country_core::resources::critters::ButterflyAudio> | core::ptr::drop_in_place<country_core::resources::critters::sheep::SheepAudio> | core::ptr::drop_in_place<country_core::resources::audio_resources::AudioInstance> | bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::critters::ButterflyAudio> | core::ptr::drop_in_place<country_core::resources::critters::Wren> | core::ptr::drop_in_place<country_core::resources::critters::ButterflyAudio> | bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::critters::Wren>
140b1b77f: 90 nop 
140b1b780: 4883c430 add rsp, 0x30
140b1b784: 5b pop rbx
140b1b785: 5f pop rdi
140b1b786: 5e pop rsi
140b1b787: 415e pop r14
140b1b789: 5d pop rbp
140b1b78a: c3 ret 
140b1b78b: 0f1f440000 nop dword ptr [rax + rax]
140b1b790: 4889542410 mov qword ptr [rsp + 0x10], rdx
140b1b795: 55 push rbp
140b1b796: 4156 push r14
140b1b798: 56 push rsi
140b1b799: 57 push rdi
140b1b79a: 53 push rbx
140b1b79b: 4883ec30 sub rsp, 0x30
140b1b79f: 488daa80000000 lea rbp, [rdx + 0x80]
140b1b7a6: 488b5560 mov rdx, qword ptr [rbp + 0x60]
140b1b7aa: 4885d2 test rdx, rdx
140b1b7ad: 740f je 0x140b1b7be
140b1b7af: 488b4d68 mov rcx, qword ptr [rbp + 0x68]
140b1b7b3: 41b801000000 mov r8d, 1
140b1b7b9: e86284afff call 0x140613c20 ; 
140b1b7be: 90 nop 
140b1b7bf: 4883c430 add rsp, 0x30
140b1b7c3: 5b pop rbx
140b1b7c4: 5f pop rdi
140b1b7c5: 5e pop rsi
140b1b7c6: 415e pop r14
140b1b7c8: 5d pop rbp
140b1b7c9: c3 ret 
140b1b7ca: 660f1f440000 nop word ptr [rax + rax]
140b1b7d0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140b1b7d5: 55 push rbp
140b1b7d6: 4156 push r14
140b1b7d8: 56 push rsi
140b1b7d9: 57 push rdi
140b1b7da: 53 push rbx
140b1b7db: 4883ec30 sub rsp, 0x30
140b1b7df: 488daa80000000 lea rbp, [rdx + 0x80]
140b1b7e6: 488d4dd8 lea rcx, [rbp - 0x28]
140b1b7ea: e841142300 call 0x140d4cc30 ; anyhow::error::impl$6::drop
140b1b7ef: 90 nop 
140b1b7f0: 4883c430 add rsp, 0x30
140b1b7f4: 5b pop rbx
140b1b7f5: 5f pop rdi
140b1b7f6: 5e pop rsi
140b1b7f7: 415e pop r14
140b1b7f9: 5d pop rbp
140b1b7fa: c3 ret 
140b1b7fb: 0f1f440000 nop dword ptr [rax + rax]
140b1b800: 4889542410 mov qword ptr [rsp + 0x10], rdx
140b1b805: 55 push rbp
140b1b806: 4156 push r14
140b1b808: 56 push rsi
140b1b809: 57 push rdi
140b1b80a: 53 push rbx
140b1b80b: 4883ec30 sub rsp, 0x30
140b1b80f: 488daa80000000 lea rbp, [rdx + 0x80]
140b1b816: 488d4d10 lea rcx, [rbp + 0x10]
140b1b81a: e8916ef3ff call 0x140a526b0 ; core::ptr::drop_in_place<enum2$<libfmod::Error> >
140b1b81f: 90 nop 
140b1b820: 4883c430 add rsp, 0x30
140b1b824: 5b pop rbx
140b1b825: 5f pop rdi
140b1b826: 5e pop rsi
140b1b827: 415e pop r14
140b1b829: 5d pop rbp
140b1b82a: c3 ret 
140b1b82b: 0f1f440000 nop dword ptr [rax + rax]
140b1b830: 4889542410 mov qword ptr [rsp + 0x10], rdx
140b1b835: 55 push rbp
140b1b836: 4156 push r14
140b1b838: 56 push rsi
140b1b839: 57 push rdi
140b1b83a: 53 push rbx
140b1b83b: 4883ec30 sub rsp, 0x30
140b1b83f: 488daa80000000 lea rbp, [rdx + 0x80]
140b1b846: 488d4d10 lea rcx, [rbp + 0x10]
140b1b84a: e8616ef3ff call 0x140a526b0 ; core::ptr::drop_in_place<enum2$<libfmod::Error> >
140b1b84f: 90 nop 
140b1b850: 4883c430 add rsp, 0x30
140b1b854: 5b pop rbx
140b1b855: 5f pop rdi
140b1b856: 5e pop rsi
140b1b857: 415e pop r14
140b1b859: 5d pop rbp
140b1b85a: c3 ret 
