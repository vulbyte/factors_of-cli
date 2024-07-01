/*
 * this program is used to simply list all the factors of a number from x to y. there are a few
 * optional flag options:
 *      -h:
 *          only print half of the results (aka, 300 * 1 and 1 * 300 are the same, so this will
 *      save duplicates from being printed)
 *
 *
 *  this program only uses ints as floats are an undefined infinity
 */

use std::env;

fn main() {
    // ############################################################
    // initialize
    // ############################################################

    // take in args, unicode only
    let args: Vec<String> = env::args().collect();
    //dbg!(&args);

    let num = args[1].parse::<u64>().unwrap();

    let print_all: bool = args.len() > 2 && args[2] == "-all";

    let print_tests: bool = args.len() > 3 && args[3] == "-tests";

    // ############################################################
    // find the GCF
    // ############################################################
    find_gcf(num, print_all, print_tests);
}

fn find_gcf(num: u64, print_all: bool, print_tests: bool) -> () {
    //the cheat line
    println! {"{} * {}", num, 1};
    for i in (1..=(num / 2)).rev() {
        //for testing
        if print_tests {
            println!("i: {}", i);
        }

        //logic
        if print_all == false && num / i > i {
            break;
        }

        //break?
        if num % i == 0 {
            println! {"{} * {}", i, num/i};
        }
    }

    return;
}

//#[cfg(test)]
//mod tests {
//    //import names from outer (for mod tests) scope
//    use super::*;
//
//    #[test]
//    fn gfc_1() {
//        assert_eq!(find_gcf(1, false, false), "1 * 1");
//    }
//}
