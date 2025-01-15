use std::collections::HashMap;

fn cakes(recipe: &HashMap<&str, u32>, available: &HashMap<&str, u32>) -> u32 {
    let mut minimum = u32::MAX;

    for (&ing, &needed) in recipe.iter() {
        if !available.contains_key(ing) {
            return 0;
        }
        for (&avail, &amount) in available.iter() {
            if avail.eq(ing) {
                match amount / needed {
                    x if x == 0 => {
                        return 0;
                    }
                    x if x > 0 && minimum > x => minimum = x,
                    _ => {}
                }
            }
        }
    }
    minimum
}