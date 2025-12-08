.section .text

.global relocator_start
.global relocator_end

relocator_end:

.section .data
.global target_rax
.global target_rbx
.global target_rcx
.global target_rdx
.global target_rsi
.global target_rip

target_rax: .quad 0
target_rbx: .quad 0
target_rcx: .quad 0
target_rdx: .quad 0
target_rsi: .quad 0
target_rip: .quad 0