// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// xfail-win32 Broken because of LLVM bug: http://llvm.org/bugs/show_bug.cgi?id=16249

// compile-flags:-Z extra-debug-info
// debugger:break zzz
// debugger:run

// STACK BY REF
// debugger:finish
// debugger:print *self
// check:$1 = {100, -100.5}
// debugger:print arg1
// check:$2 = -1
// debugger:print arg2
// check:$3 = -2
// debugger:continue

// STACK BY VAL
// debugger:finish
// d ebugger:print self -- ignored for now because of issue #8512
// c heck:$X = {100, -100.5}
// debugger:print arg1
// check:$4 = -3
// debugger:print arg2
// check:$5 = -4
// debugger:continue

// OWNED BY REF
// debugger:finish
// debugger:print *self
// check:$6 = {200, -200.5}
// debugger:print arg1
// check:$7 = -5
// debugger:print arg2
// check:$8 = -6
// debugger:continue

// OWNED BY VAL
// debugger:finish
// d ebugger:print self -- ignored for now because of issue #8512
// c heck:$X = {200, -200.5}
// debugger:print arg1
// check:$9 = -7
// debugger:print arg2
// check:$10 = -8
// debugger:continue

// OWNED MOVED
// debugger:finish
// debugger:print *self
// check:$11 = {200, -200.5}
// debugger:print arg1
// check:$12 = -9
// debugger:print arg2
// check:$13 = -10
// debugger:continue

// MANAGED BY REF
// debugger:finish
// debugger:print *self
// check:$14 = {300, -300.5}
// debugger:print arg1
// check:$15 = -11
// debugger:print arg2
// check:$16 = -12
// debugger:continue

// MANAGED BY VAL
// debugger:finish
// d ebugger:print self -- ignored for now because of issue #8512
// c heck:$X = {300, -300.5}
// debugger:print arg1
// check:$17 = -13
// debugger:print arg2
// check:$18 = -14
// debugger:continue

// MANAGED SELF
// debugger:finish
// debugger:print self->val
// check:$19 = {300, -300.5}
// debugger:print arg1
// check:$20 = -15
// debugger:print arg2
// check:$21 = -16
// debugger:continue

struct TupleStruct(int, float);

impl TupleStruct {

    fn self_by_ref(&self, arg1: int, arg2: int) -> int {
        zzz();
        arg1 + arg2
    }

    fn self_by_val(self, arg1: int, arg2: int) -> int {
        zzz();
        arg1 + arg2
    }

    fn self_owned(~self, arg1: int, arg2: int) -> int {
        zzz();
        arg1 + arg2
    }

    fn self_managed(@self, arg1: int, arg2: int) -> int {
        zzz();
        arg1 + arg2
    }
}

fn main() {
    let stack = TupleStruct(100, -100.5);
    let _ = stack.self_by_ref(-1, -2);
    let _ = stack.self_by_val(-3, -4);

    let owned = ~TupleStruct(200, -200.5);
    let _ = owned.self_by_ref(-5, -6);
    let _ = owned.self_by_val(-7, -8);
    let _ = owned.self_owned(-9, -10);

    let managed = @TupleStruct(300, -300.5);
    let _ = managed.self_by_ref(-11, -12);
    let _ = managed.self_by_val(-13, -14);
    let _ = managed.self_managed(-15, -16);
}

fn zzz() {()}
