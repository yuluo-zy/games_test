==== 0x140be2ad0  file_off=0xbe1ed0
  140be2ad0  4883ec38                 sub rsp, 0x38
  140be2ad4  0f1001                   movups xmm0, xmmword ptr [rcx]
  140be2ad7  0f29442420               movaps xmmword ptr [rsp + 0x20], xmm0
  140be2adc  89542430                 mov dword ptr [rsp + 0x30], edx
  140be2ae0  c64424342a               mov byte ptr [rsp + 0x34], 0x2a
  140be2ae5  488d0d5423a502           lea rcx, [rip + 0x2a52354] -> 0x143634e40
  140be2aec  488d542420               lea rdx, [rsp + 0x20]
  140be2af1  41b815000000             mov r8d, 0x15
  140be2af7  e884e8cd00               call 0x1418c1380
  140be2afc  90                       nop 
  140be2afd  4883c438                 add rsp, 0x38
  140be2b01  c3                       ret 
  140be2b02  cc                       int3 
  140be2b03  cc                       int3 
  140be2b04  cc                       int3 
  140be2b05  cc                       int3 
  140be2b06  cc                       int3 
  140be2b07  cc                       int3 
  140be2b08  cc                       int3 
  140be2b09  cc                       int3 
  140be2b0a  cc                       int3 
  140be2b0b  cc                       int3 
  140be2b0c  cc                       int3 
  140be2b0d  cc                       int3 
  140be2b0e  cc                       int3 
  140be2b0f  cc                       int3 
