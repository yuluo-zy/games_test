00000001409499c0: 55                               push rbp
00000001409499c1: 56                               push rsi
00000001409499c2: 57                               push rdi
00000001409499c3: 4881ecf0000000                   sub rsp, 0xf0
00000001409499ca: 488dac2480000000                 lea rbp, [rsp + 0x80]
00000001409499d2: 48c74568feffffff                 mov qword ptr [rbp + 0x68], 0xfffffffffffffffe
00000001409499da: 4889ce                           mov rsi, rcx
00000001409499dd: 488d4da0                         lea rcx, [rbp - 0x60]
00000001409499e1: e85af74000                       call 0x140d59140 ; std::fs::read_to_string::inner
00000001409499e6: 488b4da0                         mov rcx, qword ptr [rbp - 0x60]
00000001409499ea: 488b7da8                         mov rdi, qword ptr [rbp - 0x58]
00000001409499ee: 4889c8                           mov rax, rcx
00000001409499f1: 48f7d8                           neg rax
00000001409499f4: 7167                             jno 0x140949a5d
00000001409499f6: 48897d48                         mov qword ptr [rbp + 0x48], rdi
00000001409499fa: c6456701                         mov byte ptr [rbp + 0x67], 1
00000001409499fe: 488d4dd0                         lea rcx, [rbp - 0x30]
0000000140949a02: e8f9cc4000                       call 0x140d56700 ; std::backtrace::Backtrace::capture
0000000140949a07: 0f1045d0                         movups xmm0, xmmword ptr [rbp - 0x30]
0000000140949a0b: 0f104de0                         movups xmm1, xmmword ptr [rbp - 0x20]
0000000140949a0f: 0f1055f0                         movups xmm2, xmmword ptr [rbp - 0x10]
0000000140949a13: 0f295530                         movaps xmmword ptr [rbp + 0x30], xmm2
0000000140949a17: 0f294d20                         movaps xmmword ptr [rbp + 0x20], xmm1
0000000140949a1b: 0f294510                         movaps xmmword ptr [rbp + 0x10], xmm0
0000000140949a1f: 488d05ca9b1602                   lea rax, [rip + 0x2169bca]
0000000140949a26: 488945d0                         mov qword ptr [rbp - 0x30], rax
0000000140949a2a: 48c745d817000000                 mov qword ptr [rbp - 0x28], 0x17
0000000140949a32: 48897de0                         mov qword ptr [rbp - 0x20], rdi
0000000140949a36: c6456700                         mov byte ptr [rbp + 0x67], 0
0000000140949a3a: 488d4dd0                         lea rcx, [rbp - 0x30]
0000000140949a3e: 488d5510                         lea rdx, [rbp + 0x10]
0000000140949a42: e839f4f501                       call 0x1428a8e80 ; anyhow::Error::construct<anyhow::error::ContextError<ref$<str$>,std::io::error::Error> >
0000000140949a47: 48894608                         mov qword ptr [rsi + 8], rax
0000000140949a4b: 48b80000000000000080             movabs rax, 0x8000000000000000
0000000140949a55: 488906                           mov qword ptr [rsi], rax
0000000140949a58: e9e2000000                       jmp 0x140949b3f ; 
0000000140949a5d: 48894d58                         mov qword ptr [rbp + 0x58], rcx
0000000140949a61: 4c8b45b0                         mov r8, qword ptr [rbp - 0x50]
0000000140949a65: 488d4dd0                         lea rcx, [rbp - 0x30]
0000000140949a69: 48897d50                         mov qword ptr [rbp + 0x50], rdi
0000000140949a6d: 4889fa                           mov rdx, rdi
0000000140949a70: e83bb9ab00                       call 0x1414053b0 ; serde_json::de::from_str<country_core::input::input_stream::InputStream>
0000000140949a75: 488b45d0                         mov rax, qword ptr [rbp - 0x30]
0000000140949a79: 488b7dd8                         mov rdi, qword ptr [rbp - 0x28]
0000000140949a7d: 4889c1                           mov rcx, rax
0000000140949a80: 48f7d9                           neg rcx
0000000140949a83: 716d                             jno 0x140949af2
0000000140949a85: 48897d48                         mov qword ptr [rbp + 0x48], rdi
0000000140949a89: c6456601                         mov byte ptr [rbp + 0x66], 1
0000000140949a8d: 488d4d10                         lea rcx, [rbp + 0x10]
0000000140949a91: e86acc4000                       call 0x140d56700 ; std::backtrace::Backtrace::capture
0000000140949a96: 0f104510                         movups xmm0, xmmword ptr [rbp + 0x10]
0000000140949a9a: 0f104d20                         movups xmm1, xmmword ptr [rbp + 0x20]
0000000140949a9e: 0f105530                         movups xmm2, xmmword ptr [rbp + 0x30]
0000000140949aa2: 0f2955c0                         movaps xmmword ptr [rbp - 0x40], xmm2
0000000140949aa6: 0f294db0                         movaps xmmword ptr [rbp - 0x50], xmm1
0000000140949aaa: 0f2945a0                         movaps xmmword ptr [rbp - 0x60], xmm0
0000000140949aae: 488d05529b1602                   lea rax, [rip + 0x2169b52]
0000000140949ab5: 48894510                         mov qword ptr [rbp + 0x10], rax
0000000140949ab9: 48c7451815000000                 mov qword ptr [rbp + 0x18], 0x15
0000000140949ac1: 48897d20                         mov qword ptr [rbp + 0x20], rdi
0000000140949ac5: c6456600                         mov byte ptr [rbp + 0x66], 0
0000000140949ac9: 488d4d10                         lea rcx, [rbp + 0x10]
0000000140949acd: 488d55a0                         lea rdx, [rbp - 0x60]
0000000140949ad1: e82af7f501                       call 0x1428a9200 ; anyhow::Error::construct<anyhow::error::ContextError<ref$<str$>,serde_json::error::Error> >
0000000140949ad6: 48894608                         mov qword ptr [rsi + 8], rax
0000000140949ada: 48b80000000000000080             movabs rax, 0x8000000000000000
0000000140949ae4: 488906                           mov qword ptr [rsi], rax
0000000140949ae7: 488b5558                         mov rdx, qword ptr [rbp + 0x58]
0000000140949aeb: 4885d2                           test rdx, rdx
0000000140949aee: 7540                             jne 0x140949b30
0000000140949af0: eb4d                             jmp 0x140949b3f ; 
0000000140949af2: 0f1045e0                         movups xmm0, xmmword ptr [rbp - 0x20]
0000000140949af6: 0f104df0                         movups xmm1, xmmword ptr [rbp - 0x10]
0000000140949afa: 0f105500                         movups xmm2, xmmword ptr [rbp]
0000000140949afe: 0f115630                         movups xmmword ptr [rsi + 0x30], xmm2
0000000140949b02: 0f114e20                         movups xmmword ptr [rsi + 0x20], xmm1
0000000140949b06: 0f114610                         movups xmmword ptr [rsi + 0x10], xmm0
0000000140949b0a: 488906                           mov qword ptr [rsi], rax
0000000140949b0d: 48897e08                         mov qword ptr [rsi + 8], rdi
0000000140949b11: 48b80200000000000080             movabs rax, 0x8000000000000002
0000000140949b1b: 48894640                         mov qword ptr [rsi + 0x40], rax
0000000140949b1f: 48c74648ffffffff                 mov qword ptr [rsi + 0x48], 0xffffffffffffffff
0000000140949b27: 488b5558                         mov rdx, qword ptr [rbp + 0x58]
0000000140949b2b: 4885d2                           test rdx, rdx
0000000140949b2e: 740f                             je 0x140949b3f
0000000140949b30: 41b801000000                     mov r8d, 1
0000000140949b36: 488b4d50                         mov rcx, qword ptr [rbp + 0x50]
0000000140949b3a: e8e1a0ccff                       call 0x140613c20 ; 
0000000140949b3f: 4889f0                           mov rax, rsi
0000000140949b42: 4881c4f0000000                   add rsp, 0xf0
0000000140949b49: 5f                               pop rdi
0000000140949b4a: 5e                               pop rsi
0000000140949b4b: 5d                               pop rbp
0000000140949b4c: c3                               ret 
0000000140949b4d: 0f1f00                           nop dword ptr [rax]
0000000140949b50: 4889542410                       mov qword ptr [rsp + 0x10], rdx
0000000140949b55: 55                               push rbp
0000000140949b56: 56                               push rsi
0000000140949b57: 57                               push rdi
0000000140949b58: 4883ec20                         sub rsp, 0x20
0000000140949b5c: 488daa80000000                   lea rbp, [rdx + 0x80]
0000000140949b63: 807d6700                         cmp byte ptr [rbp + 0x67], 0
0000000140949b67: 7508                             jne 0x140949b71
0000000140949b69: 4883c420                         add rsp, 0x20
0000000140949b6d: 5f                               pop rdi
0000000140949b6e: 5e                               pop rsi
0000000140949b6f: 5d                               pop rbp
0000000140949b70: c3                               ret 
0000000140949b71: 488d4d48                         lea rcx, [rbp + 0x48]
0000000140949b75: e886dec9ff                       call 0x1405e7a00 ; core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::option::Option<std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::option::Option<std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error>
0000000140949b7a: ebed                             jmp 0x140949b69 ; 
0000000140949b7c: 0f1f4000                         nop dword ptr [rax]
0000000140949b80: 4889542410                       mov qword ptr [rsp + 0x10], rdx
0000000140949b85: 55                               push rbp
0000000140949b86: 56                               push rsi
0000000140949b87: 57                               push rdi
0000000140949b88: 4883ec20                         sub rsp, 0x20
0000000140949b8c: 488daa80000000                   lea rbp, [rdx + 0x80]
0000000140949b93: 48837d5800                       cmp qword ptr [rbp + 0x58], 0
0000000140949b98: 7413                             je 0x140949bad
0000000140949b9a: 41b801000000                     mov r8d, 1
0000000140949ba0: 488b4d50                         mov rcx, qword ptr [rbp + 0x50]
0000000140949ba4: 488b5558                         mov rdx, qword ptr [rbp + 0x58]
0000000140949ba8: e873a0ccff                       call 0x140613c20 ; 
0000000140949bad: 90                               nop 
0000000140949bae: 4883c420                         add rsp, 0x20
0000000140949bb2: 5f                               pop rdi
0000000140949bb3: 5e                               pop rsi
0000000140949bb4: 5d                               pop rbp
0000000140949bb5: c3                               ret 
0000000140949bb6: 662e0f1f840000000000             nop word ptr cs:[rax + rax]
0000000140949bc0: 4889542410                       mov qword ptr [rsp + 0x10], rdx
0000000140949bc5: 55                               push rbp
0000000140949bc6: 56                               push rsi
0000000140949bc7: 57                               push rdi
0000000140949bc8: 4883ec20                         sub rsp, 0x20
0000000140949bcc: 488daa80000000                   lea rbp, [rdx + 0x80]
0000000140949bd3: 807d6600                         cmp byte ptr [rbp + 0x66], 0
0000000140949bd7: 7409                             je 0x140949be2
0000000140949bd9: 488d4d48                         lea rcx, [rbp + 0x48]
0000000140949bdd: e8beecc9ff                       call 0x1405e88a0 ; core::ptr::drop_in_place<serde_json::error::Error> | core::ptr::drop_in_place<serde_json::error::Error> | core::ptr::drop_in_place<serde_json::error::Error> | core::ptr::drop_in_place<serde_json::error::Error> | core::ptr::drop_in_place<serde_json::error::Error> | core::ptr::drop_in_place<serde_json::error::Error> | core::ptr::drop_in_place<serde_json::error::Error> | core::ptr::drop_in_place<serde_json::error::Error>
0000000140949be2: 90                               nop 
0000000140949be3: 4883c420                         add rsp, 0x20
0000000140949be7: 5f                               pop rdi
0000000140949be8: 5e                               pop rsi
0000000140949be9: 5d                               pop rbp
0000000140949bea: c3                               ret 
