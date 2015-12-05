pub fn main(input: &str) -> Vec<String> {
    let s1 = paper(&input);
    let s2 = ribbon(&input);
    vec![s1.to_string(), s2.to_string()]
}

fn paper(input: &str) -> u32 {
    fn paper_for_present(l: u32, w: u32, h: u32) -> u32 {
        let mut sides = [l * w, w * h, h * l];
        sides.sort();
        let extra = sides[0];
        2 * sides.iter().sum::<u32>() + extra
    }

    input.lines()
         .map(|line| {
             let (l, w, h) = scan_fmt!(line, "{d}x{d}x{d}", u32, u32, u32);
             paper_for_present(l.unwrap(), w.unwrap(), h.unwrap())
         })
         .sum()
}

#[test]
fn test_paper() {
    assert_eq!(paper("2x3x4"), 58);
    assert_eq!(paper("1x1x10"), 43);
}

fn ribbon(input: &str) -> u32 {
    fn ribbon_for_present(l: u32, w: u32, h: u32) -> u32 {
        let mut sides = [2 * l + 2 * w, 2 * w + 2 * h, 2 * h + 2 * l];
        sides.sort();
        let ribbon = sides[0];
        let bow = l * w * h;
        ribbon + bow
    }

    input.lines()
         .map(|line| {
             let (l, w, h) = scan_fmt!(line, "{d}x{d}x{d}", u32, u32, u32);
             ribbon_for_present(l.unwrap(), w.unwrap(), h.unwrap())
         })
         .sum()
}

#[test]
fn test_ribbon() {
    assert_eq!(ribbon("2x3x4"), 34);
    assert_eq!(ribbon("1x1x10"), 14);
}
