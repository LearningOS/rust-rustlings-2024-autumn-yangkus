// hashmaps2.rs
//
// We're collecting different fruits to bake a delicious fruit cake. For this,
// we have a basket, which we'll represent in the form of a hash map. The key
// represents the name of each fruit we collect and the value represents how
// many of that particular fruit we have collected. Three types of fruits -
// Apple (4), Mango (2) and Lychee (5) are already in the basket hash map. You
// must add fruit to the basket so that there is at least one of each kind and
// more than 11 in total - we have a lot of mouths to feed. You are not allowed
// to insert any more of these fruits!
//
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps2` or use the `hint` watch subcommand for a
// hint.


use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
enum Fruit {
    Apple,
    Banana,
    Mango,
    Lychee,
    Pineapple,
}

fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
    // Add Banana and Pineapple, each with a count of 1
    basket.insert(Fruit::Banana, 1);
    basket.insert(Fruit::Pineapple, 1);

    // Calculate the current total count of fruits
    let total_count = basket.values().sum::<u32>();

    // If the total count is 11 or less, add more fruits
    if total_count <= 11 {
        let needed = 12 - total_count; // Calculate how many more are needed
        *basket.entry(Fruit::Banana).or_insert(0) += needed; // Add needed count to Banana
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Don't modify this function!
    fn get_fruit_basket() -> HashMap<Fruit, u32> {
        let mut basket = HashMap::<Fruit, u32>::new();
        basket.insert(Fruit::Apple, 4);
        basket.insert(Fruit::Mango, 2);
        basket.insert(Fruit::Lychee, 5);

        basket
    }

    #[test]
    fn test_given_fruits_are_not_modified() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        assert_eq!(*basket.get(&Fruit::Apple).unwrap(), 4);
        assert_eq!(*basket.get(&Fruit::Mango).unwrap(), 2);
        assert_eq!(*basket.get(&Fruit::Lychee).unwrap(), 5);
    }

    #[test]
    fn at_least_five_types_of_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count_fruit_kinds = basket.len();
        assert!(count_fruit_kinds >= 5);
    }

    #[test]
    fn greater_than_eleven_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count = basket.values().sum::<u32>();
        assert!(count > 11);
    }
    
    #[test]
    fn all_fruit_types_in_basket() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        for amount in basket.values() {
            assert_ne!(amount, &0);
        }
    }
}
