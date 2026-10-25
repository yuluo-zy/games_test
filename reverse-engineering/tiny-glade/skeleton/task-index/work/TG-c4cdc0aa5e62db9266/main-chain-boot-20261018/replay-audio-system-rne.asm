0x140ad5800 55 push rbp
0x140ad5801 4157 push r15
0x140ad5803 4156 push r14
0x140ad5805 4155 push r13
0x140ad5807 4154 push r12
0x140ad5809 56 push rsi
0x140ad580a 57 push rdi
0x140ad580b 53 push rbx
0x140ad580c 4881ec48010000 sub rsp, 0x148
0x140ad5813 488dac2480000000 lea rbp, [rsp + 0x80]
0x140ad581b 0f29b5b0000000 movaps xmmword ptr [rbp + 0xb0], xmm6
0x140ad5822 48c785a8000000feffffff mov qword ptr [rbp + 0xa8], 0xfffffffffffffffe
0x140ad582d 4c89cb mov rbx, r9
0x140ad5830 4d89c7 mov r15, r8
0x140ad5833 4989d4 mov r12, rdx
0x140ad5836 4989ce mov r14, rcx
0x140ad5839 e8c2111400 call 0x140c16a00 ; puffin::are_scopes_on
0x140ad583e 84c0 test al, al
0x140ad5840 8885a7000000 mov byte ptr [rbp + 0xa7], al
0x140ad5846 48895de0 mov qword ptr [rbp - 0x20], rbx
0x140ad584a 7466 je 0x140ad58b2
0x140ad584c 4c89a598000000 mov qword ptr [rbp + 0x98], r12
0x140ad5853 4c8d25b6b80002 lea r12, [rip + 0x200b8b6]
0x140ad585a bf3f000000 mov edi, 0x3f
0x140ad585f ba3f000000 mov edx, 0x3f
0x140ad5864 4c89e1 mov rcx, r12
0x140ad5867 e854c3ffff call 0x140ad1bc0 ; core::str::impl$0::rfind<ref$<str$> >
0x140ad586c 4883f801 cmp rax, 1
0x140ad5870 0f85b1000000 jne 0x140ad5927
0x140ad5876 4989d1 mov r9, rdx
0x140ad5879 4885d2 test rdx, rdx
0x140ad587c 743d je 0x140ad58bb
0x140ad587e 4983f93f cmp r9, 0x3f
0x140ad5882 7335 jae 0x140ad58b9
0x140ad5884 488d0585b80002 lea rax, [rip + 0x200b885]
0x140ad588b 41803c01bf cmp byte ptr [r9 + rax], 0xbf
0x140ad5890 7f29 jg 0x140ad58bb
0x140ad5892 488d05d7ba0002 lea rax, [rip + 0x200bad7]
0x140ad5899 4889442420 mov qword ptr [rsp + 0x20], rax
0x140ad589e 488d0d6bb80002 lea rcx, [rip + 0x200b86b]
0x140ad58a5 ba3f000000 mov edx, 0x3f
0x140ad58aa 4531c0 xor r8d, r8d
0x140ad58ad e88e40e001 call 0x1428d9940 ; core::str::slice_error_fail
0x140ad58b2 31c0 xor eax, eax
0x140ad58b4 e9d6010000 jmp 0x140ad5a8f
0x140ad58b9 75d7 jne 0x140ad5892
0x140ad58bb 4c8d254eb80002 lea r12, [rip + 0x200b84e]
0x140ad58c2 4c89e1 mov rcx, r12
0x140ad58c5 4c89ca mov rdx, r9
0x140ad58c8 e8f3c2ffff call 0x140ad1bc0 ; core::str::impl$0::rfind<ref$<str$> >
0x140ad58cd 4883f801 cmp rax, 1
0x140ad58d1 7554 jne 0x140ad5927
0x140ad58d3 4989d0 mov r8, rdx
0x140ad58d6 4983c002 add r8, 2
0x140ad58da 7439 je 0x140ad5915
0x140ad58dc 4983f83f cmp r8, 0x3f
0x140ad58e0 7331 jae 0x140ad5913
0x140ad58e2 488d0527b80002 lea rax, [rip + 0x200b827]
0x140ad58e9 41803c00bf cmp byte ptr [r8 + rax], 0xbf
0x140ad58ee 7f25 jg 0x140ad5915
0x140ad58f0 488d0591ba0002 lea rax, [rip + 0x200ba91]
0x140ad58f7 4889442420 mov qword ptr [rsp + 0x20], rax
0x140ad58fc 488d0d0db80002 lea rcx, [rip + 0x200b80d]
0x140ad5903 ba3f000000 mov edx, 0x3f
0x140ad5908 41b93f000000 mov r9d, 0x3f
0x140ad590e e82d40e001 call 0x1428d9940 ; core::str::slice_error_fail
0x140ad5913 75db jne 0x140ad58f0
0x140ad5915 bf3d000000 mov edi, 0x3d
0x140ad591a 4829d7 sub rdi, rdx
0x140ad591d 4c8d25ecb70002 lea r12, [rip + 0x200b7ec]
0x140ad5924 4d01c4 add r12, r8
0x140ad5927 488d05edbd0002 lea rax, [rip + 0x200bded]
0x140ad592e 488d35b1bd0002 lea rsi, [rip + 0x200bdb1]
0x140ad5935 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140ad5940 0fbe48ff movsx ecx, byte ptr [rax - 1]
0x140ad5944 85c9 test ecx, ecx
0x140ad5946 7818 js 0x140ad5960
0x140ad5948 48ffc8 dec rax
0x140ad594b 83f95c cmp ecx, 0x5c
0x140ad594e 7569 jne 0x140ad59b9
0x140ad5950 eb7c jmp 0x140ad59ce
0x140ad5952 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140ad5960 0fb650fe movzx edx, byte ptr [rax - 2]
0x140ad5964 80fac0 cmp dl, 0xc0
0x140ad5967 7d25 jge 0x140ad598e
0x140ad5969 440fb640fd movzx r8d, byte ptr [rax - 3]
0x140ad596e 4180f8c0 cmp r8b, 0xc0
0x140ad5972 7d23 jge 0x140ad5997
0x140ad5974 440fb648fc movzx r9d, byte ptr [rax - 4]
0x140ad5979 4883c0fc add rax, -4
0x140ad597d 4183e107 and r9d, 7
0x140ad5981 41c1e106 shl r9d, 6
0x140ad5985 4183e03f and r8d, 0x3f
0x140ad5989 4509c8 or r8d, r9d
0x140ad598c eb11 jmp 0x140ad599f
0x140ad598e 4883c0fe add rax, -2
0x140ad5992 83e21f and edx, 0x1f
0x140ad5995 eb12 jmp 0x140ad59a9
0x140ad5997 4883c0fd add rax, -3
0x140ad599b 4183e00f and r8d, 0xf
0x140ad599f 41c1e006 shl r8d, 6
0x140ad59a3 83e23f and edx, 0x3f
0x140ad59a6 4409c2 or edx, r8d
0x140ad59a9 c1e206 shl edx, 6
0x140ad59ac 80e13f and cl, 0x3f
0x140ad59af 0fb6c9 movzx ecx, cl
0x140ad59b2 09d1 or ecx, edx
0x140ad59b4 83f95c cmp ecx, 0x5c
0x140ad59b7 7415 je 0x140ad59ce
0x140ad59b9 83f92f cmp ecx, 0x2f
0x140ad59bc 7410 je 0x140ad59ce
0x140ad59be 4839f0 cmp rax, rsi
0x140ad59c1 0f8579ffffff jne 0x140ad5940
0x140ad59c7 bb35000000 mov ebx, 0x35
0x140ad59cc eb4f jmp 0x140ad5a1d
0x140ad59ce 488d3511bd0002 lea rsi, [rip + 0x200bd11]
0x140ad59d5 4829f0 sub rax, rsi
0x140ad59d8 4989c0 mov r8, rax
0x140ad59db 49ffc0 inc r8
0x140ad59de 7432 je 0x140ad5a12
0x140ad59e0 4983f835 cmp r8, 0x35
0x140ad59e4 732a jae 0x140ad5a10
0x140ad59e6 41803c30bf cmp byte ptr [r8 + rsi], 0xbf
0x140ad59eb 7f25 jg 0x140ad5a12
0x140ad59ed 488d055cb90002 lea rax, [rip + 0x200b95c]
0x140ad59f4 4889442420 mov qword ptr [rsp + 0x20], rax
0x140ad59f9 488d0de6bc0002 lea rcx, [rip + 0x200bce6]
0x140ad5a00 ba35000000 mov edx, 0x35
0x140ad5a05 41b935000000 mov r9d, 0x35
0x140ad5a0b e8303fe001 call 0x1428d9940 ; core::str::slice_error_fail
0x140ad5a10 75db jne 0x140ad59ed
0x140ad5a12 bb34000000 mov ebx, 0x34
0x140ad5a17 4829c3 sub rbx, rax
0x140ad5a1a 4c01c6 add rsi, r8
0x140ad5a1d e80e271400 call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
0x140ad5a22 4889c1 mov rcx, rax
0x140ad5a25 488b00 mov rax, qword ptr [rax]
0x140ad5a28 4883f801 cmp rax, 1
0x140ad5a2c 0f850d040000 jne 0x140ad5e3f
0x140ad5a32 4883c108 add rcx, 8
0x140ad5a36 48833900 cmp qword ptr [rcx], 0
0x140ad5a3a 0f853d040000 jne 0x140ad5e7d
0x140ad5a40 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
0x140ad5a47 48894d58 mov qword ptr [rbp + 0x58], rcx
0x140ad5a4b 4883c108 add rcx, 8
0x140ad5a4f 48895c2420 mov qword ptr [rsp + 0x20], rbx
0x140ad5a54 48c744243000000000 mov qword ptr [rsp + 0x30], 0
0x140ad5a5d 48c744242801000000 mov qword ptr [rsp + 0x28], 1
0x140ad5a66 4c89e2 mov rdx, r12
0x140ad5a69 4989f8 mov r8, rdi
0x140ad5a6c 4989f1 mov r9, rsi
0x140ad5a6f e8cc121400 call 0x140c16d40 ; puffin::ThreadProfiler::begin_scope
0x140ad5a74 488b4d58 mov rcx, qword ptr [rbp + 0x58]
0x140ad5a78 48ff01 inc qword ptr [rcx]
0x140ad5a7b 488945e8 mov qword ptr [rbp - 0x18], rax
0x140ad5a7f 488945c8 mov qword ptr [rbp - 0x38], rax
0x140ad5a83 b801000000 mov eax, 1
0x140ad5a88 4c8ba598000000 mov r12, qword ptr [rbp + 0x98]
0x140ad5a8f 488945c0 mov qword ptr [rbp - 0x40], rax
0x140ad5a93 498b3f mov rdi, qword ptr [r15]
0x140ad5a96 498b5708 mov rdx, qword ptr [r15 + 8]
0x140ad5a9a 4c8b0f mov r9, qword ptr [rdi]
0x140ad5a9d 4531c0 xor r8d, r8d
0x140ad5aa0 4c89c9 mov rcx, r9
0x140ad5aa3 482b4a18 sub rcx, qword ptr [rdx + 0x18]
0x140ad5aa7 490f42c8 cmovb rcx, r8
0x140ad5aab 4c2b4a38 sub r9, qword ptr [rdx + 0x38]
0x140ad5aaf 4d0f42c8 cmovb r9, r8
0x140ad5ab3 4889c8 mov rax, rcx
0x140ad5ab6 48c1e004 shl rax, 4
0x140ad5aba 48034208 add rax, qword ptr [rdx + 8]
0x140ad5abe 488b7210 mov rsi, qword ptr [rdx + 0x10]
0x140ad5ac2 4829ce sub rsi, rcx
0x140ad5ac5 490f42f0 cmovb rsi, r8
0x140ad5ac9 41ba08000000 mov r10d, 8
0x140ad5acf 490f42c2 cmovb rax, r10
0x140ad5ad3 4c89c9 mov rcx, r9
0x140ad5ad6 48c1e104 shl rcx, 4
0x140ad5ada 48034a28 add rcx, qword ptr [rdx + 0x28]
0x140ad5ade 4c8b6a30 mov r13, qword ptr [rdx + 0x30]
0x140ad5ae2 4d29cd sub r13, r9
0x140ad5ae5 4d0f42e8 cmovb r13, r8
0x140ad5ae9 490f42ca cmovb rcx, r10
0x140ad5aed 488b5240 mov rdx, qword ptr [rdx + 0x40]
0x140ad5af1 4c29ea sub rdx, r13
0x140ad5af4 4829f2 sub rdx, rsi
0x140ad5af7 488917 mov qword ptr [rdi], rdx
0x140ad5afa 48c1e604 shl rsi, 4
0x140ad5afe 4801c6 add rsi, rax
0x140ad5b01 49c1e504 shl r13, 4
0x140ad5b05 4901cd add r13, rcx
0x140ad5b08 498b1e mov rbx, qword ptr [r14]
0x140ad5b0b 4d8b3424 mov r14, qword ptr [r12]
0x140ad5b0f 498d5638 lea rdx, [r14 + 0x38]
0x140ad5b13 48899598000000 mov qword ptr [rbp + 0x98], rdx
0x140ad5b1a 498d5608 lea rdx, [r14 + 8]
0x140ad5b1e 48895558 mov qword ptr [rbp + 0x58], rdx
0x140ad5b22 eb12 jmp 0x140ad5b36
0x140ad5b24 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140ad5b30 4c89e0 mov rax, r12
0x140ad5b33 4c89f9 mov rcx, r15
0x140ad5b36 4885c0 test rax, rax
0x140ad5b39 7455 je 0x140ad5b90
0x140ad5b3b 4839f0 cmp rax, rsi
0x140ad5b3e 7450 je 0x140ad5b90
0x140ad5b40 4c8d6010 lea r12, [rax + 0x10]
0x140ad5b44 4989cf mov r15, rcx
0x140ad5b47 48ff07 inc qword ptr [rdi]
0x140ad5b4a 0fb64008 movzx eax, byte ptr [rax + 8]
0x140ad5b4e 85c0 test eax, eax
0x140ad5b50 7465 je 0x140ad5bb7
0x140ad5b52 83f801 cmp eax, 1
0x140ad5b55 0f8595000000 jne 0x140ad5bf0
0x140ad5b5b 41837e3001 cmp dword ptr [r14 + 0x30], 1
0x140ad5b60 4c89e0 mov rax, r12
0x140ad5b63 4c89f9 mov rcx, r15
0x140ad5b66 75ce jne 0x140ad5b36
0x140ad5b68 488d4d20 lea rcx, [rbp + 0x20]
0x140ad5b6c 488b9598000000 mov rdx, qword ptr [rbp + 0x98]
0x140ad5b73 e858179600 call 0x1414372d0 ; libfmod::EventInstance::start
0x140ad5b78 48b80400000000000080 movabs rax, 0x8000000000000004
0x140ad5b82 48394520 cmp qword ptr [rbp + 0x20], rax
0x140ad5b86 74a8 je 0x140ad5b30
0x140ad5b88 e984030000 jmp 0x140ad5f11
0x140ad5b8d 0f1f00 nop dword ptr [rax]
0x140ad5b90 4885c9 test rcx, rcx
0x140ad5b93 0f8487010000 je 0x140ad5d20
0x140ad5b99 4c39e9 cmp rcx, r13
0x140ad5b9c 0f847e010000 je 0x140ad5d20
0x140ad5ba2 4c8d7910 lea r15, [rcx + 0x10]
0x140ad5ba6 4531e4 xor r12d, r12d
0x140ad5ba9 4889c8 mov rax, rcx
0x140ad5bac 48ff07 inc qword ptr [rdi]
0x140ad5baf 0fb64008 movzx eax, byte ptr [rax + 8]
0x140ad5bb3 85c0 test eax, eax
0x140ad5bb5 759b jne 0x140ad5b52
0x140ad5bb7 41833e01 cmp dword ptr [r14], 1
0x140ad5bbb 4c89e0 mov rax, r12
0x140ad5bbe 4c89f9 mov rcx, r15
0x140ad5bc1 0f856fffffff jne 0x140ad5b36
0x140ad5bc7 488d4d20 lea rcx, [rbp + 0x20]
0x140ad5bcb 488b5558 mov rdx, qword ptr [rbp + 0x58]
0x140ad5bcf e8fc169600 call 0x1414372d0 ; libfmod::EventInstance::start
0x140ad5bd4 48b80400000000000080 movabs rax, 0x8000000000000004
0x140ad5bde 48394520 cmp qword ptr [rbp + 0x20], rax
0x140ad5be2 0f8448ffffff je 0x140ad5b30
0x140ad5be8 e974030000 jmp 0x140ad5f61
0x140ad5bed 0f1f00 nop dword ptr [rax]
0x140ad5bf0 488b13 mov rdx, qword ptr [rbx]
0x140ad5bf3 4885d2 test rdx, rdx
0x140ad5bf6 7426 je 0x140ad5c1e
0x140ad5bf8 4883c210 add rdx, 0x10
0x140ad5bfc 41b913000000 mov r9d, 0x13
0x140ad5c02 488d4d60 lea rcx, [rbp + 0x60]
0x140ad5c06 4c8d0575bb0002 lea r8, [rip + 0x200bb75]
0x140ad5c0d e82ee8f7ff call 0x140a54440 ; country_core::resources::audio_resources::AudioCoreInner::instantiate_audio
0x140ad5c12 48837d6002 cmp qword ptr [rbp + 0x60], 2
0x140ad5c17 755b jne 0x140ad5c74
0x140ad5c19 e96b020000 jmp 0x140ad5e89
0x140ad5c1e 0fb605e4efb502 movzx eax, byte ptr [rip + 0x2b5efe4]
0x140ad5c25 b913000000 mov ecx, 0x13
0x140ad5c2a ba01000000 mov edx, 1
0x140ad5c2f e8dcdfb3ff call 0x140613c10 ; ?
0x140ad5c34 4885c0 test rax, rax
0x140ad5c37 0f8474030000 je 0x140ad5fb1
0x140ad5c3d 0f10053ebb0002 movups xmm0, xmmword ptr [rip + 0x200bb3e]
0x140ad5c44 0f1100 movups xmmword ptr [rax], xmm0
0x140ad5c47 c7400f6e707574 mov dword ptr [rax + 0xf], 0x7475706e
0x140ad5c4e 48c7456000000000 mov qword ptr [rbp + 0x60], 0
0x140ad5c56 48c7457013000000 mov qword ptr [rbp + 0x70], 0x13
0x140ad5c5e 48894578 mov qword ptr [rbp + 0x78], rax
0x140ad5c62 48c7858000000013000000 mov qword ptr [rbp + 0x80], 0x13
0x140ad5c6d c6858800000002 mov byte ptr [rbp + 0x88], 2
0x140ad5c74 488b4560 mov rax, qword ptr [rbp + 0x60]
0x140ad5c78 488b4d68 mov rcx, qword ptr [rbp + 0x68]
0x140ad5c7c 488b5570 mov rdx, qword ptr [rbp + 0x70]
0x140ad5c80 4c8b4578 mov r8, qword ptr [rbp + 0x78]
0x140ad5c84 4c8b8d80000000 mov r9, qword ptr [rbp + 0x80]
0x140ad5c8b 4c894d10 mov qword ptr [rbp + 0x10], r9
0x140ad5c8f 440fb68d88000000 movzx r9d, byte ptr [rbp + 0x88]
0x140ad5c97 44884d18 mov byte ptr [rbp + 0x18], r9b
0x140ad5c9b 448b8d89000000 mov r9d, dword ptr [rbp + 0x89]
0x140ad5ca2 44894d19 mov dword ptr [rbp + 0x19], r9d
0x140ad5ca6 440fb78d8d000000 movzx r9d, word ptr [rbp + 0x8d]
0x140ad5cae 6644894d1d mov word ptr [rbp + 0x1d], r9w
0x140ad5cb3 440fb68d8f000000 movzx r9d, byte ptr [rbp + 0x8f]
0x140ad5cbb 44884d1f mov byte ptr [rbp + 0x1f], r9b
0x140ad5cbf 48895500 mov qword ptr [rbp], rdx
0x140ad5cc3 4c894508 mov qword ptr [rbp + 8], r8
0x140ad5cc7 488945f0 mov qword ptr [rbp - 0x10], rax
0x140ad5ccb 48894df8 mov qword ptr [rbp - 8], rcx
0x140ad5ccf 837df001 cmp dword ptr [rbp - 0x10], 1
0x140ad5cd3 7521 jne 0x140ad5cf6
0x140ad5cd5 488d4d20 lea rcx, [rbp + 0x20]
0x140ad5cd9 488d55f8 lea rdx, [rbp - 8]
0x140ad5cdd e8ee159600 call 0x1414372d0 ; libfmod::EventInstance::start
0x140ad5ce2 48b80400000000000080 movabs rax, 0x8000000000000004
0x140ad5cec 48394520 cmp qword ptr [rbp + 0x20], rax
0x140ad5cf0 0f85c8010000 jne 0x140ad5ebe
0x140ad5cf6 488d4df0 lea rcx, [rbp - 0x10]
0x140ad5cfa e88145dcff call 0x14089a280 ; country_core::resources::audio_resources::impl$13::drop
0x140ad5cff 488b5500 mov rdx, qword ptr [rbp]
0x140ad5d03 4885d2 test rdx, rdx
0x140ad5d06 0f8424feffff je 0x140ad5b30
0x140ad5d0c 488b4d08 mov rcx, qword ptr [rbp + 8]
0x140ad5d10 41b801000000 mov r8d, 1
0x140ad5d16 e805dfb3ff call 0x140613c20 ; ?
0x140ad5d1b e910feffff jmp 0x140ad5b30
0x140ad5d20 488b45e0 mov rax, qword ptr [rbp - 0x20]
0x140ad5d24 488b00 mov rax, qword ptr [rax]
0x140ad5d27 0f57db xorps xmm3, xmm3
0x140ad5d2a 83380d cmp dword ptr [rax], 0xd
0x140ad5d2d 7550 jne 0x140ad5d7f
0x140ad5d2f 488b8530010000 mov rax, qword ptr [rbp + 0x130]
0x140ad5d36 488b00 mov rax, qword ptr [rax]
0x140ad5d39 4883781800 cmp qword ptr [rax + 0x18], 0
0x140ad5d3e 0f84e5000000 je 0x140ad5e29
0x140ad5d44 488b4810 mov rcx, qword ptr [rax + 0x10]
0x140ad5d48 488b10 mov rdx, qword ptr [rax]
0x140ad5d4b 488b4008 mov rax, qword ptr [rax + 8]
0x140ad5d4f 4531c0 xor r8d, r8d
0x140ad5d52 4839d1 cmp rcx, rdx
0x140ad5d55 4c0f43c2 cmovae r8, rdx
0x140ad5d59 4c29c1 sub rcx, r8
0x140ad5d5c 48c1e106 shl rcx, 6
0x140ad5d60 807c083800 cmp byte ptr [rax + rcx + 0x38], 0
0x140ad5d65 f30f103597fbe401 movss xmm6, dword ptr [rip + 0x1e4fb97]
0x140ad5d6d 0f28de movaps xmm3, xmm6
0x140ad5d70 7510 jne 0x140ad5d82
0x140ad5d72 0f57db xorps xmm3, xmm3
0x140ad5d75 488b0b mov rcx, qword ptr [rbx]
0x140ad5d78 4885c9 test rcx, rcx
0x140ad5d7b 750d jne 0x140ad5d8a
0x140ad5d7d eb42 jmp 0x140ad5dc1
0x140ad5d7f 0f57f6 xorps xmm6, xmm6
0x140ad5d82 488b0b mov rcx, qword ptr [rbx]
0x140ad5d85 4885c9 test rcx, rcx
0x140ad5d88 7437 je 0x140ad5dc1
0x140ad5d8a 4883c110 add rcx, 0x10
0x140ad5d8e 488d15e3b90002 lea rdx, [rip + 0x200b9e3]
0x140ad5d95 41b804000000 mov r8d, 4
0x140ad5d9b e810e0f7ff call 0x140a53db0 ; country_core::resources::audio_resources::AudioCoreInner::set_parameter
0x140ad5da0 488b0b mov rcx, qword ptr [rbx]
0x140ad5da3 4885c9 test rcx, rcx
0x140ad5da6 7419 je 0x140ad5dc1
0x140ad5da8 4883c110 add rcx, 0x10
0x140ad5dac 488d15c9b90002 lea rdx, [rip + 0x200b9c9]
0x140ad5db3 41b806000000 mov r8d, 6
0x140ad5db9 0f28de movaps xmm3, xmm6
0x140ad5dbc e8efdff7ff call 0x140a53db0 ; country_core::resources::audio_resources::AudioCoreInner::set_parameter
0x140ad5dc1 80bda700000000 cmp byte ptr [rbp + 0xa7], 0
0x140ad5dc8 7444 je 0x140ad5e0e
0x140ad5dca e861231400 call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
0x140ad5dcf 4889c1 mov rcx, rax
0x140ad5dd2 488b00 mov rax, qword ptr [rax]
0x140ad5dd5 4883f801 cmp rax, 1
0x140ad5dd9 757e jne 0x140ad5e59
0x140ad5ddb 4883c108 add rcx, 8
0x140ad5ddf 48833900 cmp qword ptr [rcx], 0
0x140ad5de3 0f8594000000 jne 0x140ad5e7d
0x140ad5de9 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
0x140ad5df0 48898d98000000 mov qword ptr [rbp + 0x98], rcx
0x140ad5df7 4883c108 add rcx, 8
0x140ad5dfb 488b55e8 mov rdx, qword ptr [rbp - 0x18]
0x140ad5dff e8ec121400 call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
0x140ad5e04 488b8598000000 mov rax, qword ptr [rbp + 0x98]
0x140ad5e0b 48ff00 inc qword ptr [rax]
0x140ad5e0e 0f28b5b0000000 movaps xmm6, xmmword ptr [rbp + 0xb0]
0x140ad5e15 4881c448010000 add rsp, 0x148
0x140ad5e1c 5b pop rbx
0x140ad5e1d 5f pop rdi
0x140ad5e1e 5e pop rsi
0x140ad5e1f 415c pop r12
0x140ad5e21 415d pop r13
0x140ad5e23 415e pop r14
0x140ad5e25 415f pop r15
0x140ad5e27 5d pop rbp
0x140ad5e28 c3 ret 
0x140ad5e29 f30f1035d3fae401 movss xmm6, dword ptr [rip + 0x1e4fad3]
0x140ad5e31 488b0b mov rcx, qword ptr [rbx]
0x140ad5e34 4885c9 test rcx, rcx
0x140ad5e37 0f854dffffff jne 0x140ad5d8a
0x140ad5e3d eb82 jmp 0x140ad5dc1
0x140ad5e3f 4885c0 test rax, rax
0x140ad5e42 752d jne 0x140ad5e71
0x140ad5e44 31d2 xor edx, edx
0x140ad5e46 e88529e201 call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
0x140ad5e4b 4889c1 mov rcx, rax
0x140ad5e4e 4885c0 test rax, rax
0x140ad5e51 0f85dffbffff jne 0x140ad5a36
0x140ad5e57 eb18 jmp 0x140ad5e71
0x140ad5e59 4885c0 test rax, rax
0x140ad5e5c 7513 jne 0x140ad5e71
0x140ad5e5e 31d2 xor edx, edx
0x140ad5e60 e86b29e201 call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
0x140ad5e65 4889c1 mov rcx, rax
0x140ad5e68 4885c0 test rax, rax
0x140ad5e6b 0f856effffff jne 0x140ad5ddf
0x140ad5e71 488d0d284c1602 lea rcx, [rip + 0x2164c28]
0x140ad5e78 e85322e001 call 0x1428d80d0 ; std::thread::local::panic_access_error
0x140ad5e7d 488d0dec4c1602 lea rcx, [rip + 0x2164cec]
0x140ad5e84 e8c733e001 call 0x1428d9250 ; core::cell::panic_already_borrowed
0x140ad5e89 488b4568 mov rax, qword ptr [rbp + 0x68]
0x140ad5e8d 48894520 mov qword ptr [rbp + 0x20], rax
0x140ad5e91 488d0500b90002 lea rax, [rip + 0x200b900]
0x140ad5e98 4889442420 mov qword ptr [rsp + 0x20], rax
0x140ad5e9d 488d0d04b00002 lea rcx, [rip + 0x200b004]
0x140ad5ea4 4c8d0dddaf0002 lea r9, [rip + 0x200afdd]
0x140ad5eab 4c8d4520 lea r8, [rbp + 0x20]
0x140ad5eaf ba2b000000 mov edx, 0x2b
0x140ad5eb4 e8a738e001 call 0x1428d9760 ; core::result::unwrap_failed
0x140ad5eb9 e909010000 jmp 0x140ad5fc7
0x140ad5ebe 488b4550 mov rax, qword ptr [rbp + 0x50]
0x140ad5ec2 48898590000000 mov qword ptr [rbp + 0x90], rax
0x140ad5ec9 0f104520 movups xmm0, xmmword ptr [rbp + 0x20]
0x140ad5ecd 0f104d30 movups xmm1, xmmword ptr [rbp + 0x30]
0x140ad5ed1 0f105540 movups xmm2, xmmword ptr [rbp + 0x40]
0x140ad5ed5 0f299580000000 movaps xmmword ptr [rbp + 0x80], xmm2
0x140ad5edc 0f294d70 movaps xmmword ptr [rbp + 0x70], xmm1
0x140ad5ee0 0f294560 movaps xmmword ptr [rbp + 0x60], xmm0
0x140ad5ee4 488d05d5ccff01 lea rax, [rip + 0x1ffccd5]
0x140ad5eeb 4889442420 mov qword ptr [rsp + 0x20], rax
0x140ad5ef0 488d0d09a7ff01 lea rcx, [rip + 0x1ffa709]
0x140ad5ef7 4c8d0dc2a6ff01 lea r9, [rip + 0x1ffa6c2]
0x140ad5efe 4c8d4560 lea r8, [rbp + 0x60]
0x140ad5f02 ba2b000000 mov edx, 0x2b
0x140ad5f07 e85438e001 call 0x1428d9760 ; core::result::unwrap_failed
0x140ad5f0c e9b6000000 jmp 0x140ad5fc7
0x140ad5f11 488b4550 mov rax, qword ptr [rbp + 0x50]
0x140ad5f15 48898590000000 mov qword ptr [rbp + 0x90], rax
0x140ad5f1c 0f104520 movups xmm0, xmmword ptr [rbp + 0x20]
0x140ad5f20 0f104d30 movups xmm1, xmmword ptr [rbp + 0x30]
0x140ad5f24 0f105540 movups xmm2, xmmword ptr [rbp + 0x40]
0x140ad5f28 0f299580000000 movaps xmmword ptr [rbp + 0x80], xmm2
0x140ad5f2f 0f294d70 movaps xmmword ptr [rbp + 0x70], xmm1
0x140ad5f33 0f294560 movaps xmmword ptr [rbp + 0x60], xmm0
0x140ad5f37 488d0582ccff01 lea rax, [rip + 0x1ffcc82]
0x140ad5f3e 4889442420 mov qword ptr [rsp + 0x20], rax
0x140ad5f43 488d0db6a6ff01 lea rcx, [rip + 0x1ffa6b6]
0x140ad5f4a 4c8d0d6fa6ff01 lea r9, [rip + 0x1ffa66f]
0x140ad5f51 4c8d4560 lea r8, [rbp + 0x60]
0x140ad5f55 ba2b000000 mov edx, 0x2b
0x140ad5f5a e80138e001 call 0x1428d9760 ; core::result::unwrap_failed
0x140ad5f5f eb66 jmp 0x140ad5fc7
0x140ad5f61 488b4550 mov rax, qword ptr [rbp + 0x50]
0x140ad5f65 48898590000000 mov qword ptr [rbp + 0x90], rax
0x140ad5f6c 0f104520 movups xmm0, xmmword ptr [rbp + 0x20]
0x140ad5f70 0f104d30 movups xmm1, xmmword ptr [rbp + 0x30]
0x140ad5f74 0f105540 movups xmm2, xmmword ptr [rbp + 0x40]
0x140ad5f78 0f299580000000 movaps xmmword ptr [rbp + 0x80], xmm2
0x140ad5f7f 0f294d70 movaps xmmword ptr [rbp + 0x70], xmm1
0x140ad5f83 0f294560 movaps xmmword ptr [rbp + 0x60], xmm0
0x140ad5f87 488d0532ccff01 lea rax, [rip + 0x1ffcc32]
0x140ad5f8e 4889442420 mov qword ptr [rsp + 0x20], rax
0x140ad5f93 488d0d66a6ff01 lea rcx, [rip + 0x1ffa666]
0x140ad5f9a 4c8d0d1fa6ff01 lea r9, [rip + 0x1ffa61f]
0x140ad5fa1 4c8d4560 lea r8, [rbp + 0x60]
0x140ad5fa5 ba2b000000 mov edx, 0x2b
0x140ad5faa e8b137e001 call 0x1428d9760 ; core::result::unwrap_failed
0x140ad5faf eb16 jmp 0x140ad5fc7
0x140ad5fb1 4c8d05f8a7ff01 lea r8, [rip + 0x1ffa7f8]
0x140ad5fb8 b901000000 mov ecx, 1
0x140ad5fbd ba13000000 mov edx, 0x13
0x140ad5fc2 e81c30e001 call 0x1428d8fe3 ; alloc::raw_vec::handle_error
0x140ad5fc7 0f0b ud2 
0x140ad5fc9 0f1f8000000000 nop dword ptr [rax]
0x140ad5fd0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140ad5fd5 55 push rbp
0x140ad5fd6 4157 push r15
0x140ad5fd8 4156 push r14
0x140ad5fda 4155 push r13
0x140ad5fdc 4154 push r12
0x140ad5fde 56 push rsi
0x140ad5fdf 57 push rdi
0x140ad5fe0 53 push rbx
0x140ad5fe1 4883ec48 sub rsp, 0x48
0x140ad5fe5 488daa80000000 lea rbp, [rdx + 0x80]
0x140ad5fec 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
0x140ad5ff1 488b4558 mov rax, qword ptr [rbp + 0x58]
0x140ad5ff5 48ff00 inc qword ptr [rax]
0x140ad5ff8 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
0x140ad5ffd 4883c448 add rsp, 0x48
0x140ad6001 5b pop rbx
0x140ad6002 5f pop rdi
0x140ad6003 5e pop rsi
0x140ad6004 415c pop r12
0x140ad6006 415d pop r13
0x140ad6008 415e pop r14
0x140ad600a 415f pop r15
0x140ad600c 5d pop rbp
0x140ad600d c3 ret 
0x140ad600e 6690 nop 
0x140ad6010 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140ad6015 55 push rbp
0x140ad6016 4157 push r15
0x140ad6018 4156 push r14
0x140ad601a 4155 push r13
0x140ad601c 4154 push r12
0x140ad601e 56 push rsi
0x140ad601f 57 push rdi
0x140ad6020 53 push rbx
0x140ad6021 4883ec48 sub rsp, 0x48
0x140ad6025 488daa80000000 lea rbp, [rdx + 0x80]
0x140ad602c 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
0x140ad6031 80bda700000000 cmp byte ptr [rbp + 0xa7], 0
0x140ad6038 7420 je 0x140ad605a
0x140ad603a 488d45c8 lea rax, [rbp - 0x38]
0x140ad603e 488945d8 mov qword ptr [rbp - 0x28], rax
0x140ad6042 488d45d8 lea rax, [rbp - 0x28]
0x140ad6046 488945d0 mov qword ptr [rbp - 0x30], rax
0x140ad604a 488d0d9fb20002 lea rcx, [rip + 0x200b29f]
0x140ad6051 488d55d0 lea rdx, [rbp - 0x30]
0x140ad6055 e876928500 call 0x14132f2d0 ; std::thread::local::LocalKey::with<core::cell::RefCell<puffin::ThreadProfiler>,puffin::impl$6::call::closure_env$0<tuple$<>,puffin::impl$10::drop::closure_env$0>,tuple$<> >
0x140ad605a 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
0x140ad605f 4883c448 add rsp, 0x48
0x140ad6063 5b pop rbx
0x140ad6064 5f pop rdi
0x140ad6065 5e pop rsi
0x140ad6066 415c pop r12
0x140ad6068 415d pop r13
0x140ad606a 415e pop r14
0x140ad606c 415f pop r15
0x140ad606e 5d pop rbp
0x140ad606f c3 ret 
0x140ad6070 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140ad6075 55 push rbp
0x140ad6076 4157 push r15
0x140ad6078 4156 push r14
0x140ad607a 4155 push r13
0x140ad607c 4154 push r12
0x140ad607e 56 push rsi
0x140ad607f 57 push rdi
0x140ad6080 53 push rbx
0x140ad6081 4883ec48 sub rsp, 0x48
0x140ad6085 488daa80000000 lea rbp, [rdx + 0x80]
0x140ad608c 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
0x140ad6091 488b8598000000 mov rax, qword ptr [rbp + 0x98]
0x140ad6098 48ff00 inc qword ptr [rax]
0x140ad609b 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
0x140ad60a0 4883c448 add rsp, 0x48
0x140ad60a4 5b pop rbx
0x140ad60a5 5f pop rdi
0x140ad60a6 5e pop rsi
0x140ad60a7 415c pop r12
0x140ad60a9 415d pop r13
0x140ad60ab 415e pop r14
0x140ad60ad 415f pop r15
0x140ad60af 5d pop rbp
0x140ad60b0 c3 ret 
0x140ad60b1 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140ad60c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140ad60c5 55 push rbp
0x140ad60c6 4157 push r15
0x140ad60c8 4156 push r14
0x140ad60ca 4155 push r13
0x140ad60cc 4154 push r12
0x140ad60ce 56 push rsi
0x140ad60cf 57 push rdi
0x140ad60d0 53 push rbx
0x140ad60d1 4883ec48 sub rsp, 0x48
0x140ad60d5 488daa80000000 lea rbp, [rdx + 0x80]
0x140ad60dc 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
0x140ad60e1 488d4d60 lea rcx, [rbp + 0x60]
0x140ad60e5 e8c6c5f7ff call 0x140a526b0 ; core::ptr::drop_in_place<enum2$<libfmod::Error> >
0x140ad60ea 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
0x140ad60ef 4883c448 add rsp, 0x48
0x140ad60f3 5b pop rbx
0x140ad60f4 5f pop rdi
0x140ad60f5 5e pop rsi
0x140ad60f6 415c pop r12
0x140ad60f8 415d pop r13
0x140ad60fa 415e pop r14
0x140ad60fc 415f pop r15
0x140ad60fe 5d pop rbp
0x140ad60ff c3 ret 
0x140ad6100 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140ad6105 55 push rbp
0x140ad6106 4157 push r15
0x140ad6108 4156 push r14
0x140ad610a 4155 push r13
0x140ad610c 4154 push r12
0x140ad610e 56 push rsi
0x140ad610f 57 push rdi
0x140ad6110 53 push rbx
0x140ad6111 4883ec48 sub rsp, 0x48
0x140ad6115 488daa80000000 lea rbp, [rdx + 0x80]
0x140ad611c 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
0x140ad6121 488d4d20 lea rcx, [rbp + 0x20]
0x140ad6125 e8066b2700 call 0x140d4cc30 ; anyhow::error::impl$6::drop
0x140ad612a 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
0x140ad612f 4883c448 add rsp, 0x48
0x140ad6133 5b pop rbx
0x140ad6134 5f pop rdi
0x140ad6135 5e pop rsi
0x140ad6136 415c pop r12
0x140ad6138 415d pop r13
0x140ad613a 415e pop r14
0x140ad613c 415f pop r15
0x140ad613e 5d pop rbp
0x140ad613f c3 ret 
0x140ad6140 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140ad6145 55 push rbp
0x140ad6146 4157 push r15
0x140ad6148 4156 push r14
0x140ad614a 4155 push r13
0x140ad614c 4154 push r12
0x140ad614e 56 push rsi
0x140ad614f 57 push rdi
0x140ad6150 53 push rbx
0x140ad6151 4883ec48 sub rsp, 0x48
0x140ad6155 488daa80000000 lea rbp, [rdx + 0x80]
0x140ad615c 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
0x140ad6161 488d4d60 lea rcx, [rbp + 0x60]
0x140ad6165 e846c5f7ff call 0x140a526b0 ; core::ptr::drop_in_place<enum2$<libfmod::Error> >
0x140ad616a 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
0x140ad616f 4883c448 add rsp, 0x48
0x140ad6173 5b pop rbx
0x140ad6174 5f pop rdi
0x140ad6175 5e pop rsi
0x140ad6176 415c pop r12
0x140ad6178 415d pop r13
0x140ad617a 415e pop r14
0x140ad617c 415f pop r15
0x140ad617e 5d pop rbp
0x140ad617f c3 ret 
0x140ad6180 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140ad6185 55 push rbp
0x140ad6186 4157 push r15
0x140ad6188 4156 push r14
0x140ad618a 4155 push r13
0x140ad618c 4154 push r12
0x140ad618e 56 push rsi
0x140ad618f 57 push rdi
0x140ad6190 53 push rbx
0x140ad6191 4883ec48 sub rsp, 0x48
0x140ad6195 488daa80000000 lea rbp, [rdx + 0x80]
0x140ad619c 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
0x140ad61a1 488d4df0 lea rcx, [rbp - 0x10]
0x140ad61a5 e8d64467ff call 0x14014a680 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::critters::Wren>
0x140ad61aa 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
0x140ad61af 4883c448 add rsp, 0x48
0x140ad61b3 5b pop rbx
0x140ad61b4 5f pop rdi
0x140ad61b5 5e pop rsi
0x140ad61b6 415c pop r12
0x140ad61b8 415d pop r13
0x140ad61ba 415e pop r14
0x140ad61bc 415f pop r15
0x140ad61be 5d pop rbp
0x140ad61bf c3 ret 
0x140ad61c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140ad61c5 55 push rbp
0x140ad61c6 4157 push r15
0x140ad61c8 4156 push r14
0x140ad61ca 4155 push r13
0x140ad61cc 4154 push r12
0x140ad61ce 56 push rsi
0x140ad61cf 57 push rdi
0x140ad61d0 53 push rbx
0x140ad61d1 4883ec48 sub rsp, 0x48
0x140ad61d5 488daa80000000 lea rbp, [rdx + 0x80]
0x140ad61dc 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
0x140ad61e1 488b5500 mov rdx, qword ptr [rbp]
0x140ad61e5 4885d2 test rdx, rdx
0x140ad61e8 740f je 0x140ad61f9
0x140ad61ea 488b4d08 mov rcx, qword ptr [rbp + 8]
0x140ad61ee 41b801000000 mov r8d, 1
0x140ad61f4 e827dab3ff call 0x140613c20 ; ?
0x140ad61f9 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
0x140ad61fe 4883c448 add rsp, 0x48
0x140ad6202 5b pop rbx
0x140ad6203 5f pop rdi
0x140ad6204 5e pop rsi
0x140ad6205 415c pop r12
0x140ad6207 415d pop r13
0x140ad6209 415e pop r14
0x140ad620b 415f pop r15
0x140ad620d 5d pop rbp
0x140ad620e c3 ret 
0x140ad620f 90 nop 
0x140ad6210 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140ad6215 55 push rbp
0x140ad6216 4157 push r15
0x140ad6218 4156 push r14
0x140ad621a 4155 push r13
0x140ad621c 4154 push r12
0x140ad621e 56 push rsi
0x140ad621f 57 push rdi
0x140ad6220 53 push rbx
0x140ad6221 4883ec48 sub rsp, 0x48
0x140ad6225 488daa80000000 lea rbp, [rdx + 0x80]
0x140ad622c 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
0x140ad6231 488d4d60 lea rcx, [rbp + 0x60]
0x140ad6235 e876c4f7ff call 0x140a526b0 ; core::ptr::drop_in_place<enum2$<libfmod::Error> >
0x140ad623a 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
0x140ad623f 4883c448 add rsp, 0x48
0x140ad6243 5b pop rbx
0x140ad6244 5f pop rdi
0x140ad6245 5e pop rsi
0x140ad6246 415c pop r12
0x140ad6248 415d pop r13
0x140ad624a 415e pop r14
0x140ad624c 415f pop r15
0x140ad624e 5d pop rbp
0x140ad624f c3 ret 
