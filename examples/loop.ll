


define i32 @main({i32, i8**} %_19577) {
main_19575:
    %argc_19587 = extractvalue {i32, i8**} %_19577, 0
    br label %loop_19583

loop_19583:
    %i_19586 = phi i32 [ 0, %main_19575 ], [ %inc_19608, %body_19600 ]
    %acc_19596 = phi i32 [ 0, %main_19575 ], [ %acc_19610, %body_19600 ]
    %cond_19592 = icmp ult i32 %i_19586, %argc_19587
    br i1 %cond_19592, label %body_19600, label %exit_19594

body_19600:
    %inc_19608 = add i32 1, %i_19586
    %acc_19610 = add i32 %i_19586, %acc_19596
    br label %loop_19583

exit_19594:
    ret i32 %acc_19596

}


