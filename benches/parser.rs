#![feature(test)]

extern crate eve;
extern crate test;

use eve::compiler::*;
use eve::ops::Program;
use test::Bencher;

#[bench]
pub fn parse_clock(b: &mut Bencher) {
    b.iter(|| {
        let mut program = Program::new("parse_clock");
        let blocks = parse_file(
            &mut program.state.interner,
            "/users/ibdknox/scratch/eve-starter/programs/test.eve",
            false,
            false,
        );
        println!("blocks {:?}", blocks.len());
    });
}

// #[bench]
// pub fn parse_run_clock(b:&mut Bencher) {
//     let mut program = Program::new("parse_run_clock");
//     let blocks = parse_file(&mut program.state.interner, "/users/ibdknox/scratch/eve-starter/programs/test.eve", false, false);
//     println!("blocks {:?}", blocks.len());
//     let mut names = vec![];
//     for block in blocks {
//         names.push(block.name.clone());
//         program.raw_block(block);
//     }

//     let mut iter_pool = eve::ops::EstimateIterPool::new();
//     let mut txn = eve::ops::Transaction::new(iter_pool);
//     let mut ix = 0;
//     let interner = program.state.interner;
//     txn.input(interner.string_id("time|system/timer|"), interner.string_id("minutes"), interner.number_id(10.0), 1);
//     txn.input(interner.string_id("time|system/timer|"), interner.string_id("hours"), interner.number_id(10.0), 1);
//     b.iter(|| {
//         // program.clear();
//         txn.input(interner.string_id("time|system/timer|"), interner.string_id("seconds"), interner.number_id(ix as f32), 1);
//         txn.exec(&mut program, );
//         txn.clear();
//         ix += 1;
//         // let mut txn = CodeTransaction::new();
//         // for name in names.iter() {
//         //     txn.exec(&mut program, name, true);
//         // }
//     });
//}
