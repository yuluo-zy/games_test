=== 0x14003bb80 size=1  rhapsody::raw::shader::ComputePipelineDesc::new
    ; ComputePipelineDesc::new: single `ret` — no-op constructor (unit return).  [high]
    0x14003bb80    c3                       ret              

=== 0x140be6750 size=6  rhapsody::raw::shader::impl$7::default
    ; impl$7::default: returns u32 constant 0x1ffc00d0 (packed default flags/discriminant).  [high]
    0x140be6750    b8d000fc1f               mov              eax, 0x1ffc00d0
    0x140be6755    c3                       ret              

=== 0x140be6760 size=22  rhapsody::raw::shader::RasterPipelineDesc::new
    ; RasterPipelineDesc::new: copy 0x18B from src; set u32@0x18:=r8d.  [high]
    0x140be6760    4889c8                   mov              rax, rcx
    0x140be6763    488b4a10                 mov              rcx, qword ptr [rdx + 0x10]
    0x140be6767    48894810                 mov              qword ptr [rax + 0x10], rcx
    0x140be676b    0f1002                   movups           xmm0, xmmword ptr [rdx]
    0x140be676e    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140be6771    44894018                 mov              dword ptr [rax + 0x18], r8d
    0x140be6775    c3                       ret              

=== 0x140be6780 size=18  rhapsody::raw::shader::RenderPassAttachmentDesc::new
    ; RenderPassAttachmentDesc::new(format=edx): u32@0:=edx; qword@4:=0; byte@0xc:=1 (sample_count default 1). ~0x10B.  [high]
    0x140be6780    4889c8                   mov              rax, rcx
    0x140be6783    8911                     mov              dword ptr [rcx], edx
    0x140be6785    48c7410400000000         mov              qword ptr [rcx + 4], 0
    0x140be678d    c6410c01                 mov              byte ptr [rcx + 0xc], 1
    0x140be6791    c3                       ret              

=== 0x140be67a0 size=29  rhapsody::raw::shader::RenderPassAttachmentDesc::clear_input
    ; RenderPassAttachmentDesc::clear_input: copy; set u32@0x04:=1 (tag/bool true).  [high]
    0x140be67a0    4889c8                   mov              rax, rcx
    0x140be67a3    c7420401000000           mov              dword ptr [rdx + 4], 1
    0x140be67aa    8b0a                     mov              ecx, dword ptr [rdx]
    0x140be67ac    8908                     mov              dword ptr [rax], ecx
    0x140be67ae    8b4a04                   mov              ecx, dword ptr [rdx + 4]
    0x140be67b1    894804                   mov              dword ptr [rax + 4], ecx
    0x140be67b4    488b4a08                 mov              rcx, qword ptr [rdx + 8]
    0x140be67b8    48894808                 mov              qword ptr [rax + 8], rcx
    0x140be67bc    c3                       ret              

=== 0x140be67c0 size=42  rhapsody::raw::shader::RenderPassAttachmentDesc::sample_count
    ; RenderPassAttachmentDesc::sample_count(u8): copy; set byte@0x0c:=arg.  [high]
    0x140be67c0    4889c8                   mov              rax, rcx
    0x140be67c3    4488420c                 mov              byte ptr [rdx + 0xc], r8b
    0x140be67c7    488b0a                   mov              rcx, qword ptr [rdx]
    0x140be67ca    488908                   mov              qword ptr [rax], rcx
    0x140be67cd    8b4a08                   mov              ecx, dword ptr [rdx + 8]
    0x140be67d0    894808                   mov              dword ptr [rax + 8], ecx
    0x140be67d3    0fb64a0c                 movzx            ecx, byte ptr [rdx + 0xc]
    0x140be67d7    88480c                   mov              byte ptr [rax + 0xc], cl
    0x140be67da    0fb74a0d                 movzx            ecx, word ptr [rdx + 0xd]
    0x140be67de    6689480d                 mov              word ptr [rax + 0xd], cx
    0x140be67e2    0fb64a0f                 movzx            ecx, byte ptr [rdx + 0xf]
    0x140be67e6    88480f                   mov              byte ptr [rax + 0xf], cl
    0x140be67e9    c3                       ret              

=== 0x140be67f0 size=90  rhapsody::raw::shader::get_vk_blend_mode
    ; get_vk_blend_mode(desc): idx=(byte[desc+0x1b]>>3)&0xc (byte offset 0/4/8/12). Builds VkPipelineColorBlendAttachmentState(0x20B): blendEnable=t0[idx], srcColorFactor@4=1, dstColorFactor=t1[idx], colorOp@0xc=0, srcAlphaFactor@0x10=1, dstAlphaFactor=t2[idx], alphaOp@0x18=0, colorWriteMask@0x1c=0xf. t0@0x142b139f8=[0,1,1,1]; t1@0x142b13a08=[0,1,7,0x10]; t2@0x142b13a18=[0,1,7,0x12] (VkBlendFactor: 0=ZERO,1=ONE,7=ONE_MINUS_SRC_ALPHA,0x10=ONE_MINUS_SRC1_COLOR,0x12=ONE_MINUS_SRC1_ALPHA). idx0=blend off.  [high]
    0x140be67f0    4889c8                   mov              rax, rcx
    0x140be67f3    0fb64a1b                 movzx            ecx, byte ptr [rdx + 0x1b]
    0x140be67f7    488d15fad1f201           lea              rdx, [rip + 0x1f2d1fa] -> 0x142b139f8
    0x140be67fe    c1e903                   shr              ecx, 3
    0x140be6801    83e10c                   and              ecx, 0xc
    0x140be6804    8b1411                   mov              edx, dword ptr [rcx + rdx]
    0x140be6807    4c8d05fad1f201           lea              r8, [rip + 0x1f2d1fa] -> 0x142b13a08
    0x140be680e    468b0401                 mov              r8d, dword ptr [rcx + r8]
    0x140be6812    4c8d0dffd1f201           lea              r9, [rip + 0x1f2d1ff] -> 0x142b13a18
    0x140be6819    428b0c09                 mov              ecx, dword ptr [rcx + r9]
    0x140be681d    8910                     mov              dword ptr [rax], edx
    0x140be681f    c7400401000000           mov              dword ptr [rax + 4], 1
    0x140be6826    44894008                 mov              dword ptr [rax + 8], r8d
    0x140be682a    48ba0000000001000000     movabs           rdx, 0x100000000
    0x140be6834    4889500c                 mov              qword ptr [rax + 0xc], rdx
    0x140be6838    894814                   mov              dword ptr [rax + 0x14], ecx
    0x140be683b    48b9000000000f000000     movabs           rcx, 0xf00000000
    0x140be6845    48894818                 mov              qword ptr [rax + 0x18], rcx
    0x140be6849    c3                       ret              
