0x1416f9fd0 push     rsi
0x1416f9fd1 push     rdi
0x1416f9fd2 push     rbx
0x1416f9fd3 sub      rsp, 0x40
0x1416f9fd7 mov      rsi, rcx
0x1416f9fda mov      rdi, qword ptr [rdx]
0x1416f9fdd mov      rax, qword ptr [rdi + 0x10]
0x1416f9fe1 cmp      qword ptr [rdi], rax
0x1416f9fe4 je       0x1416fa04d
0x1416f9fe6 mov      rcx, qword ptr [rdi + 8]
0x1416f9fea mov      byte ptr [rcx + rax], 0x7b
0x1416f9fee inc      rax
0x1416f9ff1 mov      qword ptr [rdi + 0x10], rax
0x1416f9ff5 mov      qword ptr [rsp + 0x30], rdx
0x1416f9ffa mov      byte ptr [rsp + 0x38], 1
0x1416f9fff lea      rdx, [rip + 0x1597fa6] ; 0x142c91fac: b'sheep\x00\x00\x00\x00\x00\x00\x00\xac\x1f\xc9B\x01\x00\x00\x00\x05\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00'
0x1416fa006 lea      rcx, [rsp + 0x30]
0x1416fa00b mov      r8d, 5
0x1416fa011 mov      r9, rsi
0x1416fa014 call     0x14158aa70 ; serde::ser::SerializeMap::serialize_entry::hb83806878d107454
0x1416fa019 test     rax, rax
0x1416fa01c jne      0x1416fa045
0x1416fa01e cmp      byte ptr [rsp + 0x38], 0
0x1416fa023 je       0x1416fa045
0x1416fa025 mov      rcx, qword ptr [rsp + 0x30]
0x1416fa02a mov      rsi, qword ptr [rcx]
0x1416fa02d mov      rdx, qword ptr [rsi + 0x10]
0x1416fa031 cmp      qword ptr [rsi], rdx
0x1416fa034 je       0x1416fa07c
0x1416fa036 mov      rcx, qword ptr [rsi + 8]
0x1416fa03a mov      byte ptr [rcx + rdx], 0x7d
0x1416fa03e inc      rdx
0x1416fa041 mov      qword ptr [rsi + 0x10], rdx
0x1416fa045 add      rsp, 0x40
0x1416fa049 pop      rbx
0x1416fa04a pop      rdi
0x1416fa04b pop      rsi
0x1416fa04c ret      
0x1416fa04d mov      qword ptr [rsp + 0x20], 1
0x1416fa056 mov      r8d, 1
0x1416fa05c mov      r9d, 1
0x1416fa062 mov      rcx, rdi
0x1416fa065 mov      rbx, rdx
0x1416fa068 mov      rdx, rax
0x1416fa06b call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x1416fa070 mov      rdx, rbx
0x1416fa073 mov      rax, qword ptr [rdi + 0x10]
0x1416fa077 jmp      0x1416f9fe6
0x1416fa07c mov      qword ptr [rsp + 0x20], 1
0x1416fa085 mov      r8d, 1
0x1416fa08b mov      r9d, 1
0x1416fa091 mov      rcx, rsi
0x1416fa094 mov      rdi, rax
0x1416fa097 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x1416fa09c mov      rax, rdi
0x1416fa09f mov      rdx, qword ptr [rsi + 0x10]
0x1416fa0a3 jmp      0x1416fa036