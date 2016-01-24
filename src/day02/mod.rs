const DATA: &'static str = include_str!("input.txt");

pub fn main() -> Vec<String> {
    let s1 = paper(DATA).unwrap();
    let s2 = ribbon(DATA).unwrap();
    vec![s1.to_string(), s2.to_string()]
}

fn paper(input: &str) -> Result<u32, &str> {
    fn paper_for_present(l: u32, w: u32, h: u32) -> u32 {
        let mut sides = [l * w, w * h, h * l];
        sides.sort();
        let extra = sides[0];
        2 * sides.iter().sum::<u32>() + extra
    }

    input.lines()
         .map(|line| {
             if let (Some(l), Some(w), Some(h)) = scan_fmt!(line, "{d}x{d}x{d}", u32, u32, u32) {
                 Ok(paper_for_present(l, w, h))
             } else {
                 Err("bad input")
             }
         })
         .collect::<Result<Vec<u32>, _>>()
         .map(|v| v.iter().sum())
}

fn ribbon(input: &str) -> Result<u32, &str> {
    fn ribbon_for_present(l: u32, w: u32, h: u32) -> u32 {
        let mut sides = [2 * l + 2 * w, 2 * w + 2 * h, 2 * h + 2 * l];
        sides.sort();
        let ribbon = sides[0];
        let bow = l * w * h;
        ribbon + bow
    }

    input.lines()
         .map(|line| {
             if let (Some(l), Some(w), Some(h)) = scan_fmt!(line, "{d}x{d}x{d}", u32, u32, u32) {
                 Ok(ribbon_for_present(l, w, h))
             } else {
                 Err("bad input")
             }
         })
         .collect::<Result<Vec<u32>, _>>()
         .map(|v| v.iter().sum())
}

#[cfg(test)]
mod test {
    use super::{paper, ribbon};

    #[test]
    fn examples_1() {
        assert_eq!(paper("2x3x4"), Ok(58));
        assert_eq!(paper("1x1x10"), Ok(43));
    }

    #[test]
    fn examples_2() {
        assert_eq!(ribbon("2x3x4"), Ok(34));
        assert_eq!(ribbon("1x1x10"), Ok(14));
    }
}
