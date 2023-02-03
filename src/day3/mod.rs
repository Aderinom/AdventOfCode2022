fn getPrio(char: u8) -> u32 {
    let lowercaseOffset: u32 = 96;
    let uppercaseOffset: u32 = 38;
    if char > 96 {
        return char as u32 - lowercaseOffset;
    } else {
        return char as u32 - uppercaseOffset;
    }
}

#[test]
fn testGetPrio() {
    assert_eq!(getPrio(b'a'), 1);
    assert_eq!(getPrio(b'A'), 27);
}

fn part1() {
    let input = include_str!("input.txt");
    println!("Input: {}", input);

    let mut prio: u32 = 0;

    input.lines().for_each(|line| {
        let offset = b'A' as usize;
        let mut foundItems: [u8; 58] = [0; 58];
        let count = line.len();
        let mut chunks = line.as_bytes().chunks_exact(count / 2);

        chunks
            .next()
            .and_then(|chnk| {
                println!("C1 {:?}", std::str::from_utf8(chnk).unwrap());
                return Some(chnk);
            })
            .unwrap()
            .iter()
            .for_each(|char| {
                foundItems[*char as usize - offset] = 1;
            });

        chunks
            .next()
            .and_then(|chnk| {
                println!("C2 {:?}", std::str::from_utf8(chnk).unwrap());
                return Some(chnk);
            })
            .unwrap()
            .iter()
            .for_each(|char| {
                if foundItems[*char as usize - offset] == 1 {
                    foundItems[*char as usize - offset] = 0;
                    println!("{} exists twice", *char as char);
                    prio += getPrio(*char);
                }
            })
    });

    println!("Prio: {}", prio);
}

fn part2() {
    let input = include_str!("input.txt");
    println!("Input: {}", input);

    // We define the offset between the ASCII values of the characters and the array index
    let offset = b'A' as usize;

    let mut prio: u32 = 0;
    let mut groupIdx = 0;

    // We iterate over the input, in groups of three lines
    input.lines().array_chunks::<3>().for_each(|threeLines| {
        // We define an array to store the number of times each item was found in this array
        let mut foundCharsInThisLine: [u8; 58] = [0; 58];
        // We define an array to store the number of times each item was found in all arrays
        let mut uniqueFoundInAllGroups: [u8; 58] = [0; 58];
        groupIdx += 1;
        println!("GROUP {}", groupIdx);

        let mut lineIdx = 0;
        for line in threeLines.into_iter() {
            lineIdx += 1;
            println!("Line {}: {}", lineIdx, line);
            for thisChar in line.as_bytes().into_iter() {
                // We calculate the offset of the character in the array
                let offset = *thisChar as usize - offset;

                // We check if the character has been found in this line yet
                if foundCharsInThisLine[offset] != lineIdx {
                    // If it was not found we add it to the found list
                    // And increment the number of times the character has been found in all lines
                    foundCharsInThisLine[offset] = lineIdx;
                    uniqueFoundInAllGroups[offset] += 1;

                    // We check if the character has been found three times
                    if uniqueFoundInAllGroups[offset] == 3 {
                        println!("{} exists three times", *thisChar as char);

                        // We increment the priority
                        prio += getPrio(*thisChar);

                        // We break from the loop
                        break;
                    }
                }
            }
        }
    });

    // We print the priority
    println!("Prio: {}", prio);
}

pub fn solve() {
    println!("--- Day 3: Rucksack Reorganization ---");
    part2();
}
