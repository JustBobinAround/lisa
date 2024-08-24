; ModuleID = 'test.ll'

; Declare the printf function
declare i32 @printf(i8*, ...)

; Function to add two integers
define i32 @add(i32 %a, i32 %b) {
entry:
  %sum = add i32 %a, %b
  ret i32 %sum
}

; Main function
define i32 @main() {
entry:
  ; Define two integers
  %a = alloca i32, align 4
  %b = alloca i32, align 4
  store i32 10, i32* %a, align 4
  store i32 20, i32* %b, align 4
  
  ; Load the integers
  %a_val = load i32, i32* %a, align 4
  %b_val = load i32, i32* %b, align 4
  
  ; Call the add function
  %result = call i32 @add(i32 %a_val, i32 %b_val)
  
  ; Prepare format string
  %fmt = alloca [13 x i8], align 1
  store [13 x i8] c"Result: %d\n\00", [13 x i8]* %fmt, align 1
  
  ; Call printf to print the result
  %fmt_ptr = getelementptr [13 x i8], [13 x i8]* %fmt, i32 0, i32 0
  call i32 @printf(i8* %fmt_ptr, i32 %result)
  
  ; Return 0 from main
  ret i32 0
}


