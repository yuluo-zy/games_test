1407ec570: 488d8de0000000 lea rcx, [rbp + 0xe0]
1407ec577: e8f4a33300 call 0x140b26970 ; country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
1407ec57c: 888580020000 mov byte ptr [rbp + 0x280], al
1407ec582: 0fb68580020000 movzx eax, byte ptr [rbp + 0x280]
1407ec589: 88858f020000 mov byte ptr [rbp + 0x28f], al
1407ec58f: 3c01 cmp al, 1
1407ec591: 0f8793feffff ja 0x1407ec42a
