use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Best Buy Ultimate Catnip Adventure Toy".to_string(),
            price: 9.99,
            description: "Watch your feline friend embark on an exciting journey with Best Buy's Ultimate Catnip Adventure Toy. Infused with premium catnip and featuring a dangling fish lure for endless fun.".to_string(),
            image: "/1.png".to_string()
        },
        Product {
            id: 2,
            name: "Best Buy Squeaky Squid Dog Toy".to_string(),
            price: 6.99,
            description: "Let your dog set sail with Best Buy's Squeaky Squid Dog Toy. This interactive toy provides hours of entertainment, with multiple squeakers and crinkle tentacles for added excitement.".to_string(),
            image: "/2.png".to_string()
        },
        Product {
            id: 3,
            name: "Best Buy Mermaid Mice Trio".to_string(),
            price: 12.99,
            description: "Captivate your kitty's curiosity with the Best Buy Mermaid Mice Trio. These adorable plush mice, dressed as mermaids and filled with catnip, are perfect for playful pouncing.".to_string(),
            image: "/3.png".to_string()
        },
        Product {
            id: 4,
            name: "Best Buy Ocean Explorer Puzzle Ball".to_string(),
            price: 11.99,
            description: "Challenge your pet's problem-solving skills with the Best Buy Ocean Explorer Puzzle Ball. This interactive toy features hidden compartments and treats for mental stimulation and entertainment.".to_string(),
            image: "/4.png".to_string()
        },
        Product {
            id: 5,
            name: "Best Buy Pirate Parrot Teaser Wand".to_string(),
            price: 8.99,
            description: "Engage your cat in a playful pursuit with the Best Buy Pirate Parrot Teaser Wand. The colorful feathers and jingling bells mimic the charm of a pirate's parrot, keeping your feline entertained.".to_string(),
            image: "/5.png".to_string()
        },
        
           ]
}