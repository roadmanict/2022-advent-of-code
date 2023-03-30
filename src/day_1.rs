use crate::model::{food::Food, elf::Elf, food_bag::FoodBag};
use std::{fs::File, io::Read, path::Path, vec};

pub fn find_elf_carrying_most_calories() -> usize {
    let data_file_path = Path::new("resources/day_1.txt");
    let mut data_file = match File::open(data_file_path) {
        Err(_) => panic!("Error opening file"),
        Ok(file) => file,
    };
    let mut content = String::new();
    match data_file.read_to_string(&mut content) {
        Err(_) => panic!("Error reading file"),
        Ok(_) => {}
    }
    let splitted_string = content.split("\\d");
    for part in splitted_string {
        if part == "" {
            println!("Empty line");

            continue;
        }

        print!("{}", part);
    }
    let elfs: Vec<Elf> = vec![
        Elf {
            food_bag: FoodBag {
                items: vec![
                    Food { calories: 1000 },
                    Food { calories: 2000 },
                    Food { calories: 3000 },
                ],
            },
        },
        Elf {
            food_bag: FoodBag {
                items: vec![Food { calories: 4000 }],
            },
        },
        Elf {
            food_bag: FoodBag {
                items: vec![Food { calories: 5000 }, Food { calories: 6000 }],
            },
        },
        Elf {
            food_bag: FoodBag {
                items: vec![
                    Food { calories: 7000 },
                    Food { calories: 8000 },
                    Food { calories: 9000 },
                ],
            },
        },
        Elf {
            food_bag: FoodBag {
                items: vec![Food { calories: 10000 }],
            },
        },
    ];

    let mut elf_with_most_calories = 0;
    let mut most_calories = 0;

    for (index, elf) in elfs.iter().enumerate() {
        if elf.total_calories() > most_calories {
            elf_with_most_calories = index;
            most_calories = elf.total_calories();
        }
    }

    elf_with_most_calories
}
