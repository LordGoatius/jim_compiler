.intel_syntax noprefix
.extern printf
.global main

main:
    mov rax, 4
    push rax

    mov rax, 1
    mov rdi, 1
    lea rsi, rax     
    mov rdx, 8
    syscall             

    mov rax, 60  
    xor rdi, rdi    
    syscall