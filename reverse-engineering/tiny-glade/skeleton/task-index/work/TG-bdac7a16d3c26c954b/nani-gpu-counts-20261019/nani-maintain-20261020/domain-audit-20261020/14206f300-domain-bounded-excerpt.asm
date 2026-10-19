14206fad0: 488b5108 mov rdx, qword ptr [rcx + 8]
14206fad4: 488955f0 mov qword ptr [rbp - 0x10], rdx
14206fad8: 488b4d28 mov rcx, qword ptr [rbp + 0x28]
14206fadc: e86f869cfe call 0x140a38150 ; country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::get_by_name<utils::name_hash::NameHash>
14206fae1: 4c8b4558 mov r8, qword ptr [rbp + 0x58]
14206fae5: 450fb64810 movzx r9d, byte ptr [r8 + 0x10]
14206faea: 4983c010 add r8, 0x10
14206faee: 4883c0f8 add rax, -8
14206faf2: 48c1e203 shl rdx, 3
14206faf6: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
14206fb00: 4885d2 test rdx, rdx
14206fb03: 0f84140b0000 je 0x14207061d
14206fb09: 488d4808 lea rcx, [rax + 8]
14206fb0d: 4883c2f8 add rdx, -8
14206fb11: 4438480c cmp byte ptr [rax + 0xc], r9b
14206fb15: 4889c8 mov rax, rcx
14206fb18: 75e6 jne 0x14206fb00
14206fb1a: 8b19 mov ebx, dword ptr [rcx]
142070474: 48899508010000 mov qword ptr [rbp + 0x108], rdx
14207047b: 4c8985f0000000 mov qword ptr [rbp + 0xf0], r8
142070482: 498b5008 mov rdx, qword ptr [r8 + 8]
142070486: 488b4d28 mov rcx, qword ptr [rbp + 0x28]
14207048a: e8c17c9cfe call 0x140a38150 ; country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::get_by_name<utils::name_hash::NameHash>
14207048f: 488955e8 mov qword ptr [rbp - 0x18], rdx
142070493: 4883fa01 cmp rdx, 1
142070497: 0f851f010000 jne 0x1420705bc
14207049d: 448b00 mov r8d, dword ptr [rax]
1420704a0: 0fb64004 movzx eax, byte ptr [rax + 4]
1420704a4: 44898510010000 mov dword ptr [rbp + 0x110], r8d
1420704ab: 888514010000 mov byte ptr [rbp + 0x114], al
1420704b1: 4c8b95f0000000 mov r10, qword ptr [rbp + 0xf0]
1420704b8: 413a4210 cmp al, byte ptr [r10 + 0x10]
1420704bc: 0f8529010000 jne 0x1420705eb
