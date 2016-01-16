use std::cmp::max;
use std::iter::Iterator;

const DATA: &'static str = include_str!("../data/input_15.txt");

fn parse_ingredient(s: &str) -> Option<([i32; 4], i32)> {
    if let (Some(capacity),
            Some(durability),
            Some(flavor),
            Some(texture),
            Some(calories)) = scan_fmt!(s,
                                        "{*}: capacity {d}, durability {d}, flavor {d}, texture \
                                         {d}, calories {d}",
                                        i32,
                                        i32,
                                        i32,
                                        i32,
                                        i32) {
        Some(([capacity, durability, flavor, texture], calories))
    } else {
        None
    }
}

fn parse_input(input: &str) -> Option<(Vec<[i32; 4]>, Vec<i32>)> {
    input.lines()
         .map(parse_ingredient)
         .collect::<Option<Vec<_>>>()
         .map(|v| v.iter().cloned().unzip())
}

pub fn main() -> Vec<String> {
    let budget: i32 = 100;
    if let Some((ingredients, calories)) = parse_input(DATA) {

        // generate all possible recipes
        let recipes = (0..budget + 1).flat_map(move |a| {
            (0..budget + 1 - a).flat_map(move |b| {
                (0..budget + 1 - a - b).map(move |c| {
                    let d = budget - a - b - c;
                    [a, b, c, d]
                })
            })
        });

        // score all the cookies
        let cookies = recipes.map(|r| {
                                 // a recipe is a list of 4 amounts
                                 // for each amount pair with the ingredient data
                                 let scores =
                                 Iterator::zip(r.iter(), ingredients.iter())
                                     .map(|(i, w)| {
                                         // and multiply the ingredient properties by the amount
                                         [i * w[0], i * w[1], i * w[2], i * w[3]]
                                     })
                                     .fold([0; 4], |acc, i| {
                                         // now add up the weighted properties of each ingredient
                                         [acc[0] + i[0],
                                          acc[1] + i[1],
                                          acc[2] + i[2],
                                          acc[3] + i[3]]
                                     })
                                     .iter()
                                     .fold(1, |acc, &i| {
                                         // now we've reduced each recipe to scores for 4 properties
                                         // drop any negative scores, and multiply them together for
                                         // a total cookie score
                                         acc * max(0, i)
                                     });

                                 // for calories, multiply amount by ingredient info and sum
                                 let calories = Iterator::zip(r.iter(), calories.iter())
                                                    .map(|(i, c)| i * c)
                                                    .sum::<i32>();
                                 (scores, calories)
                             })
                             .collect::<Vec<(i32, i32)>>();

        vec![cookies.iter().map(|c| c.0).max().unwrap_or(0).to_string(),
             cookies.iter().filter(|c| c.1 == 500).map(|c| c.0).max().unwrap_or(0).to_string()]
    } else {
        vec![]
    }
}

#[cfg(test)]
mod test {
}
