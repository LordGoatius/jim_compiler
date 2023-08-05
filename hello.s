.global _start
.intel_syntax noprefix

_start: 
    mov rax, 1
    mov rdi, 1
    lea rsi, [message]     
    mov rdx, 14
    syscall             

    mov rax, 60  
    xor rdi, rdi    
    syscall 

message:
    .asciz "Hello, world!\n"
