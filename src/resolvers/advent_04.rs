pub fn resolve(input: &String) {
    let mut sum = 0;
    let mut total_scratchcards = 0;
    let mut next_winning_card: Vec<usize> = Vec::new();
    for _ in input.lines() {
        next_winning_card.push(1);
    }
    let mut row_index: usize = 0;
    for row in input.lines() {
        let parts: Vec<&str> = row.split(":").collect();
        let grid: Vec<&str> = parts[1].split("|").collect();
        let winning_numbers: Vec<&str> = grid[0].trim().split(" ").collect();
        let played_numbers: Vec<&str> = grid[1].trim().split(" ").collect();
        println!("Winning numbers: {:?} played {:?}", winning_numbers, played_numbers);
        let mut nb_valid: usize = 0;
        for played_number in played_numbers {
            if played_number == "" {
                continue;
            }
            if winning_numbers.contains(&played_number) {
                nb_valid = nb_valid + 1;
            }
        }
        if nb_valid <= 1 {
            sum = sum + nb_valid;
        } else {
            sum = sum + 2usize.pow((nb_valid - 1) as u32);
        }

        println!("Nb valid {}, row: {} next_winning_card row {}", nb_valid, row_index, next_winning_card[row_index]);
        for i in 0..nb_valid {
            println!("adding to row {} value {}", row_index + i, 1 * next_winning_card[row_index]);
            next_winning_card[row_index + i + 1] = next_winning_card[row_index + i + 1] + 1 * next_winning_card[row_index];
        }
        total_scratchcards = total_scratchcards + next_winning_card[row_index];

        // println!("next sum: {}", sum);
        println!("next total: {}", total_scratchcards);
        row_index = row_index + 1;
    }

}