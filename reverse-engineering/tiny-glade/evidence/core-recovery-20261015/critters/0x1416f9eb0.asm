0x1416f9eb0 push     rsi
0x1416f9eb1 push     rdi
0x1416f9eb2 push     rbx
0x1416f9eb3 sub      rsp, 0x40
0x1416f9eb7 mov      rsi, rcx
0x1416f9eba mov      rdi, qword ptr [rdx]
0x1416f9ebd mov      rax, qword ptr [rdi + 0x10]
0x1416f9ec1 cmp      qword ptr [rdi], rax
0x1416f9ec4 je       0x1416f9f76
0x1416f9eca mov      rcx, qword ptr [rdi + 8]
0x1416f9ece mov      byte ptr [rcx + rax], 0x7b
0x1416f9ed2 inc      rax
0x1416f9ed5 mov      qword ptr [rdi + 0x10], rax
0x1416f9ed9 mov      qword ptr [rsp + 0x30], rdx
0x1416f9ede mov      byte ptr [rsp + 0x38], 1
0x1416f9ee3 lea      r9, [rsi + 0x1c]
0x1416f9ee7 lea      rdx, [rip + 0x159805e] ; 0x142c91f4c: b'positiongoalmove_state\x00\x00\x00\x00\x00\x00L\x1f\xc9B\x01\x00\x00\x00\x08\x00\x00\x00\x00\x00\x00\x00T\x1f\xc9B'
0x1416f9eee lea      rcx, [rsp + 0x30]
0x1416f9ef3 mov      r8d, 8
0x1416f9ef9 call     0x141583080 ; serde::ser::SerializeMap::serialize_entry::h0ce3d6de0fae5387
0x1416f9efe test     rax, rax
0x1416f9f01 jne      0x1416f9f6e
0x1416f9f03 lea      rdx, [rip + 0x159804a] ; 0x142c91f54: b'goalmove_state\x00\x00\x00\x00\x00\x00L\x1f\xc9B\x01\x00\x00\x00\x08\x00\x00\x00\x00\x00\x00\x00T\x1f\xc9B\x01\x00\x00\x00\x04\x00\x00\x00'
0x1416f9f0a lea      rcx, [rsp + 0x30]
0x1416f9f0f mov      r8d, 4
0x1416f9f15 mov      r9, rsi
0x1416f9f18 call     0x141587b60 ; serde::ser::SerializeMap::serialize_entry::h7939b6c7b1d18f25
0x1416f9f1d test     rax, rax
0x1416f9f20 jne      0x1416f9f6e
0x1416f9f22 add      rsi, 0x10
0x1416f9f26 lea      rdx, [rip + 0x159802b] ; 0x142c91f58: b'move_state\x00\x00\x00\x00\x00\x00L\x1f\xc9B\x01\x00\x00\x00\x08\x00\x00\x00\x00\x00\x00\x00T\x1f\xc9B\x01\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00'
0x1416f9f2d lea      rcx, [rsp + 0x30]
0x1416f9f32 mov      r8d, 0xa
0x1416f9f38 mov      r9, rsi
0x1416f9f3b call     0x141587910 ; serde::ser::SerializeMap::serialize_entry::h77709d8e9335feca
0x1416f9f40 test     rax, rax
0x1416f9f43 jne      0x1416f9f6e
0x1416f9f45 cmp      byte ptr [rsp + 0x38], 0
0x1416f9f4a je       0x1416f9f6c
0x1416f9f4c mov      rax, qword ptr [rsp + 0x30]
0x1416f9f51 mov      rsi, qword ptr [rax]
0x1416f9f54 mov      rdx, qword ptr [rsi + 0x10]
0x1416f9f58 cmp      qword ptr [rsi], rdx
0x1416f9f5b je       0x1416f9fa5
0x1416f9f5d mov      rax, qword ptr [rsi + 8]
0x1416f9f61 mov      byte ptr [rax + rdx], 0x7d
0x1416f9f65 inc      rdx
0x1416f9f68 mov      qword ptr [rsi + 0x10], rdx
0x1416f9f6c xor      eax, eax
0x1416f9f6e add      rsp, 0x40
0x1416f9f72 pop      rbx
0x1416f9f73 pop      rdi
0x1416f9f74 pop      rsi
0x1416f9f75 ret      
0x1416f9f76 mov      qword ptr [rsp + 0x20], 1
0x1416f9f7f mov      r8d, 1
0x1416f9f85 mov      r9d, 1
0x1416f9f8b mov      rcx, rdi
0x1416f9f8e mov      rbx, rdx
0x1416f9f91 mov      rdx, rax
0x1416f9f94 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x1416f9f99 mov      rdx, rbx
0x1416f9f9c mov      rax, qword ptr [rdi + 0x10]
0x1416f9fa0 jmp      0x1416f9eca
0x1416f9fa5 mov      qword ptr [rsp + 0x20], 1
0x1416f9fae mov      r8d, 1
0x1416f9fb4 mov      r9d, 1
0x1416f9fba mov      rcx, rsi
0x1416f9fbd call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x1416f9fc2 mov      rdx, qword ptr [rsi + 0x10]
0x1416f9fc6 jmp      0x1416f9f5d
0x1416f9fc8 int3     
0x1416f9fc9 int3     
0x1416f9fca int3     
0x1416f9fcb int3     
0x1416f9fcc int3     
0x1416f9fcd int3     
0x1416f9fce int3     
0x1416f9fcf int3     