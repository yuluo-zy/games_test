140a7a651: 8b471c mov eax, dword ptr [rdi + 0x1c]
140a7a654: 488b4f10 mov rcx, qword ptr [rdi + 0x10]
140a7a658: 8901 mov dword ptr [rcx], eax
140a7a65a: 488b3f mov rdi, qword ptr [rdi]
140a7a65d: 4889f9 mov rcx, rdi
140a7a660: e85bee0f00 call 0x140b794c0 ; hashbrown::raw::RawTable::clear<tuple$<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,alloc::alloc::Global>
140a7a665: c6473801 mov byte ptr [rdi + 0x38], 1
