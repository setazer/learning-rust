mod numbers;
mod pig_latin;
mod allocations;
fn main() {
    dbg!(numbers::number_stats(&vec![100,200,300,300,300,1,1,2,3,5,8,10]));
    println!("{}", pig_latin::to_pig_latin("What's up mate aren't you pleased to see me"));
    allocations::run();
}
