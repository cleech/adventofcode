pub fn main(input: &str) -> Vec<String> {
    let s1 = paper(&input);
    let s2 = ribbon(&input);
    vec![s1.to_string(), s2.to_string()]
}

fn paper(input: &str) -> u32 {
    fn paper_for_present(l: u32, w: u32, h: u32) -> u32 {
        let sides = vec![l * w, w * h, h * l];
        let extra = sides.iter().min().unwrap();
        2 * l * w + 2 * w * h + 2 * h * l + extra
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
        let sides = vec![2 * l + 2 * w, 2 * w + 2 * h, 2 * h + 2 * l];
        let ribbon = sides.iter().min().unwrap();
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
