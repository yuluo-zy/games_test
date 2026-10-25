=== 0x1406868b0 size=5  rhapsody::raw::buffer::impl$3::byte_count
    ; impl$3::byte_count: returns qword [rcx+8].  [high]
    0x1406868b0    488b4108                 mov              rax, qword ptr [rcx + 8]
    0x1406868b4    c3                       ret              

=== 0x140bd4de0 size=59  rhapsody::raw::buffer::BufferDesc::new
    ; BufferDesc::new(size=rdx,u32 usage=r8d,u8 memloc=r9b): assert size>0 (panic 'assertion failed: size > 0' @0x142b0f830); writes u64 size@0, u32 usage@8, u16 alignment@0xc:=0, u8 memory_location@0xe:=r9b. BufferDesc=16B {size@0,usage@8,alignment@0xc,memory_location@0xe}.  [high]
    0x140bd4de0    4883ec28                 sub              rsp, 0x28
    0x140bd4de4    4885d2                   test             rdx, rdx
    0x140bd4de7    7419                     je               0x140bd4e02
    0x140bd4de9    488911                   mov              qword ptr [rcx], rdx
    0x140bd4dec    44894108                 mov              dword ptr [rcx + 8], r8d
    0x140bd4df0    4488490e                 mov              byte ptr [rcx + 0xe], r9b
    0x140bd4df4    66c7410c0000             mov              word ptr [rcx + 0xc], 0
    0x140bd4dfa    4889c8                   mov              rax, rcx
    0x140bd4dfd    4883c428                 add              rsp, 0x28
    0x140bd4e01    c3                       ret              
    0x140bd4e02    488d0d27aaf301           lea              rcx, [rip + 0x1f3aa27] -> 0x142b0f830
    0x140bd4e09    4c8d0540aaf301           lea              r8, [rip + 0x1f3aa40] -> 0x142b0f850
    0x140bd4e10    ba1a000000               mov              edx, 0x1a
    0x140bd4e15    e81646d001               call             0x1428d9430 -> core::panicking::panic::hb210ae7b817a9f13
    0x140bd4e1a    cc                       int3             

=== 0x140bd4e20 size=59  rhapsody::raw::buffer::BufferDesc::new_gpu_only
    ; BufferDesc::new_gpu_only(size,usage): same as new but memory_location@0xe := 1.  [high]
    0x140bd4e20    4883ec28                 sub              rsp, 0x28
    0x140bd4e24    4885d2                   test             rdx, rdx
    0x140bd4e27    7419                     je               0x140bd4e42
    0x140bd4e29    488911                   mov              qword ptr [rcx], rdx
    0x140bd4e2c    44894108                 mov              dword ptr [rcx + 8], r8d
    0x140bd4e30    c6410e01                 mov              byte ptr [rcx + 0xe], 1
    0x140bd4e34    66c7410c0000             mov              word ptr [rcx + 0xc], 0
    0x140bd4e3a    4889c8                   mov              rax, rcx
    0x140bd4e3d    4883c428                 add              rsp, 0x28
    0x140bd4e41    c3                       ret              
    0x140bd4e42    488d0de7a9f301           lea              rcx, [rip + 0x1f3a9e7] -> 0x142b0f830
    0x140bd4e49    4c8d0518aaf301           lea              r8, [rip + 0x1f3aa18] -> 0x142b0f868
    0x140bd4e50    ba1a000000               mov              edx, 0x1a
    0x140bd4e55    e8d645d001               call             0x1428d9430 -> core::panicking::panic::hb210ae7b817a9f13
    0x140bd4e5a    cc                       int3             

=== 0x140bd4e60 size=59  rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu
    ; BufferDesc::new_cpu_to_gpu(size,usage): memory_location@0xe := 2.  [high]
    0x140bd4e60    4883ec28                 sub              rsp, 0x28
    0x140bd4e64    4885d2                   test             rdx, rdx
    0x140bd4e67    7419                     je               0x140bd4e82
    0x140bd4e69    488911                   mov              qword ptr [rcx], rdx
    0x140bd4e6c    44894108                 mov              dword ptr [rcx + 8], r8d
    0x140bd4e70    c6410e02                 mov              byte ptr [rcx + 0xe], 2
    0x140bd4e74    66c7410c0000             mov              word ptr [rcx + 0xc], 0
    0x140bd4e7a    4889c8                   mov              rax, rcx
    0x140bd4e7d    4883c428                 add              rsp, 0x28
    0x140bd4e81    c3                       ret              
    0x140bd4e82    488d0da7a9f301           lea              rcx, [rip + 0x1f3a9a7] -> 0x142b0f830
    0x140bd4e89    4c8d05f0a9f301           lea              r8, [rip + 0x1f3a9f0] -> 0x142b0f880
    0x140bd4e90    ba1a000000               mov              edx, 0x1a
    0x140bd4e95    e89645d001               call             0x1428d9430 -> core::panicking::panic::hb210ae7b817a9f13
    0x140bd4e9a    cc                       int3             

=== 0x140bd4ea0 size=59  rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu_low_prio
    ; BufferDesc::new_cpu_to_gpu_low_prio(size,usage): memory_location@0xe := 3.  [high]
    0x140bd4ea0    4883ec28                 sub              rsp, 0x28
    0x140bd4ea4    4885d2                   test             rdx, rdx
    0x140bd4ea7    7419                     je               0x140bd4ec2
    0x140bd4ea9    488911                   mov              qword ptr [rcx], rdx
    0x140bd4eac    44894108                 mov              dword ptr [rcx + 8], r8d
    0x140bd4eb0    c6410e03                 mov              byte ptr [rcx + 0xe], 3
    0x140bd4eb4    66c7410c0000             mov              word ptr [rcx + 0xc], 0
    0x140bd4eba    4889c8                   mov              rax, rcx
    0x140bd4ebd    4883c428                 add              rsp, 0x28
    0x140bd4ec1    c3                       ret              
    0x140bd4ec2    488d0d67a9f301           lea              rcx, [rip + 0x1f3a967] -> 0x142b0f830
    0x140bd4ec9    4c8d05c8a9f301           lea              r8, [rip + 0x1f3a9c8] -> 0x142b0f898
    0x140bd4ed0    ba1a000000               mov              edx, 0x1a
    0x140bd4ed5    e85645d001               call             0x1428d9430 -> core::panicking::panic::hb210ae7b817a9f13
    0x140bd4eda    cc                       int3             

=== 0x140bd4ee0 size=59  rhapsody::raw::buffer::BufferDesc::new_gpu_to_cpu
    ; BufferDesc::new_gpu_to_cpu(size,usage): memory_location@0xe := 4. So BufferMemoryLocation enum: 1=GpuOnly,2=CpuToGpu,3=CpuToGpuLowPrio,4=GpuToCpu (0=custom via ::new).  [high]
    0x140bd4ee0    4883ec28                 sub              rsp, 0x28
    0x140bd4ee4    4885d2                   test             rdx, rdx
    0x140bd4ee7    7419                     je               0x140bd4f02
    0x140bd4ee9    488911                   mov              qword ptr [rcx], rdx
    0x140bd4eec    44894108                 mov              dword ptr [rcx + 8], r8d
    0x140bd4ef0    c6410e04                 mov              byte ptr [rcx + 0xe], 4
    0x140bd4ef4    66c7410c0000             mov              word ptr [rcx + 0xc], 0
    0x140bd4efa    4889c8                   mov              rax, rcx
    0x140bd4efd    4883c428                 add              rsp, 0x28
    0x140bd4f01    c3                       ret              
    0x140bd4f02    488d0d27a9f301           lea              rcx, [rip + 0x1f3a927] -> 0x142b0f830
    0x140bd4f09    4c8d05a0a9f301           lea              r8, [rip + 0x1f3a9a0] -> 0x142b0f8b0
    0x140bd4f10    ba1a000000               mov              edx, 0x1a
    0x140bd4f15    e81645d001               call             0x1428d9430 -> core::panicking::panic::hb210ae7b817a9f13
    0x140bd4f1a    cc                       int3             

=== 0x140bd51a0 size=13  rhapsody::raw::buffer::impl$4::byte_count
    ; impl$4::byte_count: if [rcx]!=0 then return qword [rcx+8] else return 0. (Option-like accessor; not BufferDesc itself.)  [medium]
    0x140bd51a0    488b01                   mov              rax, qword ptr [rcx]
    0x140bd51a3    4885c0                   test             rax, rax
    0x140bd51a6    7404                     je               0x140bd51ac
    0x140bd51a8    488b4108                 mov              rax, qword ptr [rcx + 8]
    0x140bd51ac    c3                       ret              

=== 0x1415a6090 size=224  rhapsody::raw::buffer::impl$10::fmt
    ; impl$10::fmt: Debug for BufferDesc. struct 'BufferDesc'; fields 'size','usage','memory_location','alignment'. Confirms the 4 fields+names.  [high]
    0x1415a6090    4881eca8000000           sub              rsp, 0xa8
    0x1415a6097    4889d0                   mov              rax, rdx
    0x1415a609a    488d5108                 lea              rdx, [rcx + 8]
    0x1415a609e    4c8d410e                 lea              r8, [rcx + 0xe]
    0x1415a60a2    4c8d490c                 lea              r9, [rcx + 0xc]
    0x1415a60a6    4c898c24a0000000         mov              qword ptr [rsp + 0xa0], r9
    0x1415a60ae    4c8d0d6b366d01           lea              r9, [rip + 0x16d366b] -> 0x142c79720
    0x1415a60b5    4c898c2490000000         mov              qword ptr [rsp + 0x90], r9
    0x1415a60bd    4c8d8c24a0000000         lea              r9, [rsp + 0xa0]
    0x1415a60c5    4c898c2488000000         mov              qword ptr [rsp + 0x88], r9
    0x1415a60cd    4c8d0d8e366d01           lea              r9, [rip + 0x16d368e] -> 0x142c79762
    0x1415a60d4    4c894c2478               mov              qword ptr [rsp + 0x78], r9
    0x1415a60d9    4c8d0d20366d01           lea              r9, [rip + 0x16d3620] -> 0x142c79700
    0x1415a60e0    4c894c2470               mov              qword ptr [rsp + 0x70], r9
    0x1415a60e5    4c89442468               mov              qword ptr [rsp + 0x68], r8
    0x1415a60ea    4c8d0562366d01           lea              r8, [rip + 0x16d3662] -> 0x142c79753
    0x1415a60f1    4c89442458               mov              qword ptr [rsp + 0x58], r8
    0x1415a60f6    4c8d05e3356d01           lea              r8, [rip + 0x16d35e3] -> 0x142c796e0
    0x1415a60fd    4c89442450               mov              qword ptr [rsp + 0x50], r8
    0x1415a6102    4889542448               mov              qword ptr [rsp + 0x48], rdx
    0x1415a6107    488d1540366d01           lea              rdx, [rip + 0x16d3640] -> 0x142c7974e
    0x1415a610e    4889542438               mov              qword ptr [rsp + 0x38], rdx
    0x1415a6113    488d15a6356d01           lea              rdx, [rip + 0x16d35a6] -> 0x142c796c0
    0x1415a611a    4889542430               mov              qword ptr [rsp + 0x30], rdx
    0x1415a611f    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x1415a6124    48c784248000000009000000 mov              qword ptr [rsp + 0x80], 9
    0x1415a6130    48c74424600f000000       mov              qword ptr [rsp + 0x60], 0xf
    0x1415a6139    48c744244005000000       mov              qword ptr [rsp + 0x40], 5
    0x1415a6142    48c744242004000000       mov              qword ptr [rsp + 0x20], 4
    0x1415a614b    488d15ee356d01           lea              rdx, [rip + 0x16d35ee] -> 0x142c79740
    0x1415a6152    4c8d0df1356d01           lea              r9, [rip + 0x16d35f1] -> 0x142c7974a
    0x1415a6159    41b80a000000             mov              r8d, 0xa
    0x1415a615f    4889c1                   mov              rcx, rax
    0x1415a6162    e8490d7eff               call             0x140d86eb0 -> core::fmt::Formatter::debug_struct_field4_finish::hed15c7072f5fd9d1
    0x1415a6167    90                       nop              
    0x1415a6168    4881c4a8000000           add              rsp, 0xa8
    0x1415a616f    c3                       ret              
