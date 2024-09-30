pub fn raindrops(n: u32) -> String {
    let mut sounds = String::new();
    let dividers = vec![3, 5, 7];
    let sounds_map = vec!["Pling", "Plang", "Plong"];

    // Iterate over dividers and corresponding sounds
    for (divider, sound) in dividers.iter().zip(sounds_map.iter()) {
        if n % divider == 0 {
            sounds.push_str(sound);
        }
    }

    // If no sounds were added, return the number as a string
    if sounds.is_empty() {
        sounds = n.to_string();
    }

    sounds
}
