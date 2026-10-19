1407ee6f6: 488b8578020000 mov rax, qword ptr [rbp + 0x278]
1407ee6fd: 898510020000 mov dword ptr [rbp + 0x210], eax
1407ee703: 488d05f6c12902 lea rax, [rip + 0x229c1f6]
1407ee70a: 48898518020000 mov qword ptr [rbp + 0x218], rax
1407ee711: 48c785200200000d000000 mov qword ptr [rbp + 0x220], 0xd
1407ee71c: 488d8d10020000 lea rcx, [rbp + 0x210]
1407ee723: e808e5a900 call 0x14128cc30 ; utils::calculate_hash<tuple$<country_core::resources::walls::decorator::DecoratorId,ref$<str$> > >
1407ee728: 48898580020000 mov qword ptr [rbp + 0x280], rax
