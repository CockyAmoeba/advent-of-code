use day_2::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(include_str!(
        "../input1.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn part1_nom() {
    part1_nom::process(divan::black_box(include_str!(
        "../input1.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn part1_struct() {
    part1_struct::process(divan::black_box(include_str!(
        "../input1.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn part2() {
    part2::process(divan::black_box(include_str!(
        "../input2.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn part2_nom() {
    part2_nom::process(divan::black_box(include_str!(
        "../input2.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn part2_struct() {
    part2_struct::process(divan::black_box(include_str!(
        "../input2.txt",
    )))
    .unwrap();
}