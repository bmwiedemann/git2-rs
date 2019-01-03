%"core::fmt::Formatter" = type { [0 x i64], { i64, i64 }, [0 x i64], { i64, i64 }, [0 x i64], { {}*, [3 x i64]* }, [0 x i64], { i64*, i64* }, [0 x i64], { [0 x { i8*, i8* }]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i8], i8, [7 x i8] }
%"core::cell::BorrowMutError" = type { [0 x i8], {}, [0 x i8] }
%"core::fmt::Arguments" = type { [0 x i64], { [0 x { [0 x i8]*, i64 }]*, i64 }, [0 x i64], { i64*, i64 }, [0 x i64], { [0 x { i8*, i8* }]*, i64 }, [0 x i64] }
%"unwind::libunwind::_Unwind_Exception" = type { [0 x i64], i64, [0 x i64], void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [0 x i64], [6 x i64], [0 x i64] }
%"unwind::libunwind::_Unwind_Context" = type { [0 x i8] }
@0 = private unnamed_addr constant <{ [16 x i8] }> <{ [16 x i8] c"already borrowed" }>, align 1
@1 = private unnamed_addr constant <{ [43 x i8] }> <{ [43 x i8] c"called `Option::unwrap()` on a `None` value" }>, align 1
@2 = private unnamed_addr constant <{ [17 x i8] }> <{ [17 x i8] c"libcore/option.rs" }>, align 1
@3 = private unnamed_addr constant <{ i8*, [8 x i8], i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [43 x i8] }>, <{ [43 x i8] }>* @1, i32 0, i32 0, i32 0), [8 x i8] c"+\00\00\00\00\00\00\00", i8* getelementptr inbounds (<{ [17 x i8] }>, <{ [17 x i8] }>* @2, i32 0, i32 0, i32 0), [16 x i8] c"\11\00\00\00\00\00\00\00c\01\00\00\15\00\00\00" }>, align 8
@4 = private unnamed_addr constant <{ [57 x i8] }> <{ [57 x i8] c"cannot access a TLS value during or after it is destroyed" }>, align 1
@5 = private unnamed_addr constant <{ [43 x i8] }> <{ [43 x i8] c"called `Result::unwrap()` on an `Err` value" }>, align 1
@6 = private unnamed_addr constant <{ [0 x i8] }> zeroinitializer, align 8
@7 = private unnamed_addr constant <{ [2 x i8] }> <{ [2 x i8] c": " }>, align 1
@8 = private unnamed_addr constant <{ i8*, [8 x i8], i8*, [8 x i8] }> <{ i8* getelementptr inbounds (<{ [0 x i8] }>, <{ [0 x i8] }>* @6, i32 0, i32 0, i32 0), [8 x i8] zeroinitializer, i8* getelementptr inbounds (<{ [2 x i8] }>, <{ [2 x i8] }>* @7, i32 0, i32 0, i32 0), [8 x i8] c"\02\00\00\00\00\00\00\00" }>, align 8
@vtable.4 = private unnamed_addr constant { void ({ [0 x i8]*, i64 }*)*, i64, i64, i64 ({ [0 x i8]*, i64 }*)* } { void ({ [0 x i8]*, i64 }*)* @_ZN4core3ptr13drop_in_place17h9bd9235bcfc5fc40E, i64 16, i64 8, i64 ({ [0 x i8]*, i64 }*)* @"_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$11get_type_id17h82fc7cfd158950e2E" }, align 8
@vtable.5 = private unnamed_addr constant { void ({}*)*, i64, i64, i64 ({}*)* } { void ({}*)* @_ZN4core3ptr13drop_in_place17h6e528a715c899f9fE, i64 0, i64 1, i64 ({}*)* @"_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$11get_type_id17h96af0782982a05e3E" }, align 8
@_ZN3lib5panic10LAST_ERROR7__getit5__KEY17h3776bf008a82988eE = internal thread_local global <{ [40 x i8] }> zeroinitializer, align 32
define internal i64 @"_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$11get_type_id17h82fc7cfd158950e2E"({ [0 x i8]*, i64 }* noalias nocapture readonly dereferenceable(16) %self) unnamed_addr #0 {
  ret i64 1229646359891580772
}
define internal i64 @"_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$11get_type_id17h96af0782982a05e3E"({}* noalias nocapture nonnull readonly %self) unnamed_addr #0 {
  ret i64 7549865886324542212
}
define internal void @_ZN3std6thread5local4fast13destroy_value17h626edf2a7806eeeaE(i8* nocapture %ptr) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
  %1 = tail call zeroext i1 @_ZN3std3sys4unix17fast_thread_local25requires_move_before_drop17h550ffcf33fedbc57E()
  %_7.sroa.0.0..sroa_idx = bitcast i8* %ptr to i64*
  %_7.sroa.0.0.copyload = load i64, i64* %_7.sroa.0.0..sroa_idx, align 8
  %_7.sroa.28.0..sroa_idx9 = getelementptr inbounds i8, i8* %ptr, i64 16
  %2 = bitcast i8* %_7.sroa.28.0..sroa_idx9 to i64*
  %_7.sroa.28.0.copyload = load i64, i64* %2, align 8
  %_7.sroa.5.0..sroa_idx12 = getelementptr inbounds i8, i8* %ptr, i64 24
  %3 = bitcast i8* %_7.sroa.5.0..sroa_idx12 to i64*
  %_7.sroa.5.0.copyload = load i64, i64* %3, align 8
  %4 = icmp eq i64 %_7.sroa.0.0.copyload, 0
  %5 = icmp eq i64 %_7.sroa.28.0.copyload, 0
  %6 = inttoptr i64 %_7.sroa.28.0.copyload to {}*
  %7 = inttoptr i64 %_7.sroa.5.0.copyload to void ({}*)**
  %8 = load void ({}*)*, void ({}*)** %7, align 8, !invariant.load !1, !nonnull !1
  %9 = inttoptr i64 %_7.sroa.5.0.copyload to i64*
  %10 = getelementptr inbounds i64, i64* %9, i64 1
  %11 = load i64, i64* %10, align 8, !invariant.load !1, !alias.scope !2
  %12 = icmp eq i64 %11, 0
  %13 = inttoptr i64 %_7.sroa.28.0.copyload to i8*
  %14 = getelementptr inbounds i64, i64* %9, i64 2
  %15 = load i64, i64* %14, align 8, !invariant.load !1, !alias.scope !2
  unreachable
}
define internal zeroext i1 @"_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h3a1c279ce18fc823E"({ [0 x i8]*, i64 }* noalias nocapture readonly dereferenceable(16) %self, %"core::fmt::Formatter"* dereferenceable(96) %f) unnamed_addr #1 {
start:
  %0 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %self, i64 0, i32 0
  %1 = load [0 x i8]*, [0 x i8]** %0, align 8, !nonnull !1
  %2 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %self, i64 0, i32 1
  %3 = load i64, i64* %2, align 8
  %4 = tail call zeroext i1 @"_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17h4d06f72123a3bf61E"([0 x i8]* noalias nonnull readonly %1, i64 %3, %"core::fmt::Formatter"* nonnull dereferenceable(96) %f)
  ret i1 %4
}
define internal void @_ZN4core3ptr13drop_in_place17h6e528a715c899f9fE({}* nocapture %arg0) unnamed_addr #0 {
  ret void
}
define internal void @_ZN4core3ptr13drop_in_place17h9bd9235bcfc5fc40E({ [0 x i8]*, i64 }* nocapture %arg0) unnamed_addr #0 {
start:
  %error = alloca %"core::cell::BorrowMutError", align 1
  %_10 = alloca [2 x { i8*, i8* }], align 8
  %_3 = alloca %"core::fmt::Arguments", align 8
  %msg = alloca { [0 x i8]*, i64 }, align 8
  %0 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %msg, i64 0, i32 0
  store [0 x i8]* bitcast (<{ [16 x i8] }>* @0 to [0 x i8]*), [0 x i8]** %0, align 8
  %1 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %msg, i64 0, i32 1
  store i64 16, i64* %1, align 8
  %2 = bitcast %"core::fmt::Arguments"* %_3 to i8*
  call void @llvm.lifetime.start.p0i8(i64 48, i8* nonnull %2)
  %3 = bitcast [2 x { i8*, i8* }]* %_10 to i8*
  %4 = getelementptr inbounds %"core::cell::BorrowMutError", %"core::cell::BorrowMutError"* %error, i64 0, i32 0, i64 0
  %5 = bitcast [2 x { i8*, i8* }]* %_10 to { [0 x i8]*, i64 }**
  store { [0 x i8]*, i64 }* %msg, { [0 x i8]*, i64 }** %5, align 8
  %6 = getelementptr inbounds [2 x { i8*, i8* }], [2 x { i8*, i8* }]* %_10, i64 0, i64 0, i32 1
  store i8* bitcast (i1 ({ [0 x i8]*, i64 }*, %"core::fmt::Formatter"*)* @"_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h3a1c279ce18fc823E" to i8*), i8** %6, align 8
  %7 = getelementptr inbounds [2 x { i8*, i8* }], [2 x { i8*, i8* }]* %_10, i64 0, i64 1, i32 0
  store i8* %4, i8** %7, align 8
  %8 = getelementptr inbounds [2 x { i8*, i8* }], [2 x { i8*, i8* }]* %_10, i64 0, i64 1, i32 1
  store i8* bitcast (i1 (%"core::cell::BorrowMutError"*, %"core::fmt::Formatter"*)* @"_ZN63_$LT$core..cell..BorrowMutError$u20$as$u20$core..fmt..Debug$GT$3fmt17hca7d2710462e2ce1E" to i8*), i8** %8, align 8
  %9 = bitcast %"core::fmt::Arguments"* %_3 to [0 x { [0 x i8]*, i64 }]**
  store [0 x { [0 x i8]*, i64 }]* bitcast (<{ i8*, [8 x i8], i8*, [8 x i8] }>* @8 to [0 x { [0 x i8]*, i64 }]*), [0 x { [0 x i8]*, i64 }]** %9, align 8, !alias.scope !8, !noalias !11
  %10 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %_3, i64 0, i32 1, i32 1
  store i64 2, i64* %10, align 8, !alias.scope !8, !noalias !11
  unreachable
}
define internal { {}*, [3 x i64]* } @"_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$9box_me_up17hea965a0f076813e3E"({ i8*, i64 }* nocapture dereferenceable(16) %self) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %0 = bitcast { i8*, i64 }* %self to i64*
  %1 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %self, i64 0, i32 0
  %tmp.sroa.0.0.copyload.i4.i.i2.i.i = load i8*, i8** %1, align 8
  %tmp.sroa.5.0..sroa_idx2.i.i.i.i.i = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %self, i64 0, i32 1
  %tmp.sroa.5.0.copyload.i.i.i.i.i = load i64, i64* %tmp.sroa.5.0..sroa_idx2.i.i.i.i.i, align 8
  %2 = icmp eq i8* %tmp.sroa.0.0.copyload.i4.i.i2.i.i, null
  br i1 %2, label %bb10, label %bb5
bb5:                                              ; preds = %start
  %3 = tail call i8* @__rust_alloc(i64 16, i64 8) #9, !noalias !29
  %4 = icmp eq i8* %3, null
  br i1 %4, label %bb7.i.i, label %bb7
bb7.i.i:                                          ; preds = %bb5
  unreachable
bb7:                                              ; preds = %bb5
  %5 = bitcast i8* %3 to i8**
  store i8* %tmp.sroa.0.0.copyload.i4.i.i2.i.i, i8** %5, align 8, !noalias !29
  %6 = getelementptr inbounds i8, i8* %3, i64 8
  %7 = bitcast i8* %6 to i64*
  store i64 %tmp.sroa.5.0.copyload.i.i.i.i.i, i64* %7, align 8, !noalias !29
  %8 = bitcast i8* %3 to {}*
  br label %bb10
bb10:                                             ; preds = %bb7, %start
  %.sink3 = phi {}* [ %8, %bb7 ], [ inttoptr (i64 1 to {}*), %start ]
  %.sink = phi [3 x i64]* [ bitcast ({ void ({ [0 x i8]*, i64 }*)*, i64, i64, i64 ({ [0 x i8]*, i64 }*)* }* @vtable.4 to [3 x i64]*), %bb7 ], [ bitcast ({ void ({}*)*, i64, i64, i64 ({}*)* }* @vtable.5 to [3 x i64]*), %start ]
  %9 = icmp ne {}* %.sink3, null
  %10 = insertvalue { {}*, [3 x i64]* } undef, {}* %.sink3, 0
  %11 = insertvalue { {}*, [3 x i64]* } %10, [3 x i64]* %.sink, 1
  ret { {}*, [3 x i64]* } %11
}
define void @_ZN3lib5panic5check17h281f861c6ca5df27E() unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %0 = load i8, i8* getelementptr inbounds (<{ [40 x i8] }>, <{ [40 x i8] }>* @_ZN3lib5panic10LAST_ERROR7__getit5__KEY17h3776bf008a82988eE, i64 0, i32 0, i64 33), align 1, !range !32, !noalias !33
  %1 = icmp eq i8 %0, 0
  %2 = load i8, i8* getelementptr inbounds (<{ [40 x i8] }>, <{ [40 x i8] }>* @_ZN3lib5panic10LAST_ERROR7__getit5__KEY17h3776bf008a82988eE, i64 0, i32 0, i64 32), align 32, !range !32, !noalias !33
  %3 = icmp eq i8 %2, 0
  %4 = load i64, i64* bitcast (<{ [40 x i8] }>* @_ZN3lib5panic10LAST_ERROR7__getit5__KEY17h3776bf008a82988eE to i64*), align 32, !range !36, !noalias !33
  %switch.i.i = icmp eq i64 %4, 1
  br i1 %switch.i.i, label %bb16.i.i, label %bb14.i.i
bb14.i.i:                                         ; preds = %bb12.i.i
  %t.0.copyload2.i.i.i.i.i.i.i.i = load <4 x i64>, <4 x i64>* bitcast (<{ [40 x i8] }>* @_ZN3lib5panic10LAST_ERROR7__getit5__KEY17h3776bf008a82988eE to <4 x i64>*), align 32, !noalias !37
  store <4 x i64> <i64 1, i64 0, i64 0, i64 undef>, <4 x i64>* bitcast (<{ [40 x i8] }>* @_ZN3lib5panic10LAST_ERROR7__getit5__KEY17h3776bf008a82988eE to <4 x i64>*), align 32, !noalias !33
  %_10.sroa.0.0.vec.extract.i.i.i = extractelement <4 x i64> %t.0.copyload2.i.i.i.i.i.i.i.i, i32 0
  %_10.sroa.0.16.vec.extract.i.i.i = extractelement <4 x i64> %t.0.copyload2.i.i.i.i.i.i.i.i, i32 2
  %_10.sroa.0.24.vec.extract.i.i.i = extractelement <4 x i64> %t.0.copyload2.i.i.i.i.i.i.i.i, i32 3
  %5 = icmp eq i64 %_10.sroa.0.0.vec.extract.i.i.i, 0
  %6 = icmp eq i64 %_10.sroa.0.16.vec.extract.i.i.i, 0
  %or.cond.i.i.i = or i1 %5, %6
  br i1 %or.cond.i.i.i, label %bb7.i.i.i, label %bb2.i.i.i.i.i.i.i
bb2.i.i.i.i.i.i.i:                                ; preds = %bb14.i.i
  %7 = inttoptr i64 %_10.sroa.0.16.vec.extract.i.i.i to {}*
  %8 = inttoptr i64 %_10.sroa.0.24.vec.extract.i.i.i to void ({}*)**
  %9 = load void ({}*)*, void ({}*)** %8, align 8, !invariant.load !1, !noalias !33, !nonnull !1
  br label %bb7.i.i.i
bb7.i.i.i:                                        ; preds = %bb4.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i, %bb14.i.i
  unreachable
bb16.i.i:                                         ; preds = %bb7.i.i.i, %bb12.i.i
  ret void
}
declare i32 @rust_eh_personality(i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*) unnamed_addr #6
declare zeroext i1 @_ZN3std3sys4unix17fast_thread_local25requires_move_before_drop17h550ffcf33fedbc57E() unnamed_addr #1
declare zeroext i1 @"_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17h4d06f72123a3bf61E"([0 x i8]* noalias nonnull readonly, i64, %"core::fmt::Formatter"* dereferenceable(96)) unnamed_addr #1
declare zeroext i1 @"_ZN63_$LT$core..cell..BorrowMutError$u20$as$u20$core..fmt..Debug$GT$3fmt17hca7d2710462e2ce1E"(%"core::cell::BorrowMutError"* noalias nonnull readonly, %"core::fmt::Formatter"* dereferenceable(96)) unnamed_addr #1
declare noalias i8* @__rust_alloc(i64, i64) unnamed_addr #10
declare void @llvm.lifetime.start.p0i8(i64, i8* nocapture) #7
!1 = !{}
!2 = !{!3}
!3 = distinct !{!3, !4, !"_ZN5alloc5alloc8box_free17h215c3c281cc7c6f3E: %ptr.1"}
!4 = distinct !{!4, !"_ZN5alloc5alloc8box_free17h215c3c281cc7c6f3E"}
!8 = !{!9}
!9 = distinct !{!9, !10, !"_ZN4core3fmt9Arguments16new_v1_formatted17hf11ec085fdfcd9c5E: argument 0"}
!10 = distinct !{!10, !"_ZN4core3fmt9Arguments16new_v1_formatted17hf11ec085fdfcd9c5E"}
!11 = !{!12, !13, !14}
!12 = distinct !{!12, !10, !"_ZN4core3fmt9Arguments16new_v1_formatted17hf11ec085fdfcd9c5E: %pieces.0"}
!13 = distinct !{!13, !10, !"_ZN4core3fmt9Arguments16new_v1_formatted17hf11ec085fdfcd9c5E: %args.0"}
!14 = distinct !{!14, !10, !"_ZN4core3fmt9Arguments16new_v1_formatted17hf11ec085fdfcd9c5E: %fmt.0"}
!29 = !{!30}
!30 = distinct !{!30, !31, !"_ZN35_$LT$alloc..boxed..Box$LT$T$GT$$GT$3new17h4fdb857929c782c4E: %x.0"}
!31 = distinct !{!31, !"_ZN35_$LT$alloc..boxed..Box$LT$T$GT$$GT$3new17h4fdb857929c782c4E"}
!32 = !{i8 0, i8 2}
!33 = !{!34}
!34 = distinct !{!34, !35, !"_ZN46_$LT$std..thread..local..LocalKey$LT$T$GT$$GT$8try_with17hecf89db7c9eb249cE: argument 0"}
!35 = distinct !{!35, !"_ZN46_$LT$std..thread..local..LocalKey$LT$T$GT$$GT$8try_with17hecf89db7c9eb249cE"}
!36 = !{i64 0, i64 2}
!37 = !{!38, !40, !34}
!38 = distinct !{!38, !39, !"_ZN4core3mem7replace17h05b6ddc6b02376b9E: argument 0"}
!39 = distinct !{!39, !"_ZN4core3mem7replace17h05b6ddc6b02376b9E"}
!40 = distinct !{!40, !39, !"_ZN4core3mem7replace17h05b6ddc6b02376b9E: %src"}
