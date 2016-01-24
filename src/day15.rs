use std::cmp::max;
use std::iter;
use std::iter::Iterator;

const DATA: &'static str = include_str!("../data/input_15.txt");

pub fn main() -> Vec<String> {
    let budget: i32 = 100;
    if let Some((ingredients, calories)) = parse_input(DATA) {

        // generate all possible recipes
        let recipes = cookie_recipes(budget, ingredients.len());

        // score all the cookies
        let cookies = recipes.map(|r| {
                                 // a recipe is a list of 4 amounts
                                 // for each amount pair with the ingredient data
                                 let score = score_recipe(&r, &ingredients);
                                 let calories = get_calories(&r, &calories);
                                 (score, calories)
                             })
                             .collect::<Vec<(i32, i32)>>();

        vec![cookies.iter().map(|c| c.0).max().unwrap_or(0).to_string(),
             cookies.iter().filter(|c| c.1 == 500).map(|c| c.0).max().unwrap_or(0).to_string()]
    } else {
        vec![]
    }
}

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

// return an iterator over all "recipes"
// which are just 'count' numbers that add up to the total ingredient budget
fn cookie_recipes(budget: i32, count: usize) -> Box<Iterator<Item = Vec<i32>>> {
    if count == 1 {
        box iter::once(vec![budget])
    } else {
        box (0..budget + 1).flat_map(move |n| {
            cookie_recipes(budget - n, count - 1).map(move |mut v| {
                v.push(n);
                v
            })
        })
    }
}

fn score_recipe(r: &[i32], ingredients: &[[i32; 4]]) -> i32 {
    Iterator::zip(r.iter(), ingredients.iter())
        .map(|(i, w)| {
            // multiply the ingredient properties by the amount
            [i * w[0], i * w[1], i * w[2], i * w[3]]
        })
        .fold([0; 4], |acc, i| {
            // add up the weighted properties of each ingredient
            [acc[0] + i[0], acc[1] + i[1], acc[2] + i[2], acc[3] + i[3]]
        })
        .iter()
        .fold(1, |acc, &i| {
            // each recipe has been reduced to scores for 4 properties
            // drop any negative scores, and multiply the rest together for
            // a total cookie score
            acc * max(0, i)
        })
}

fn get_calories(r: &[i32], calories: &[i32]) -> i32 {
    // multiply amount by ingredient calorie info and sum
    Iterator::zip(r.iter(), calories.iter())
        .map(|(i, c)| i * c)
        .sum::<i32>()
}

#[cfg(test)]
mod test {
    use super::{parse_input, cookie_recipes, score_recipe, get_calories};

    const EXAMPLE_DATA: [&'static str; 2] = ["Butterscotch: capacity -1, durability -2, flavor \
                                              6, texture 3, calories 8",
                                             "Cinnamon: capacity 2, durability 3, flavor -2, \
                                              texture -1, calories 3"];

    #[test]
    fn examples_1() {
        if let Some((ingredients, _)) = parse_input(&EXAMPLE_DATA.join("\n")) {
            let recipes = cookie_recipes(100, ingredients.len());
            let cookies = recipes.map(|r| score_recipe(&r, &ingredients));
            assert_eq!(cookies.max(), Some(62842880));
        } else {
            panic!("EXAMPLE_DATA failed to parse");
        }
    }

    #[test]
    fn examples_2() {
        if let Some((ingredients, calories)) = parse_input(&EXAMPLE_DATA.join("\n")) {
            let recipes = cookie_recipes(100, ingredients.len());
            let cookies = recipes.map(|r| {
                (score_recipe(&r, &ingredients), get_calories(&r, &calories))
            });
            let soln = cookies.filter(|c| c.1 == 500)
                              .map(|c| c.0)
                              .max();
            assert_eq!(soln, Some(57600000));
        } else {
            panic!("EXAMPLE_DATA failed to parse");
        }
    }
}
