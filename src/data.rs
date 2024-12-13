use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Best Buy Smart Home Hub".to_string(),
            price: 129.99,
            description: "Streamline your smart home with the Best Buy Smart Home Hub. Seamlessly connect and control all your smart devices with a single, user-friendly interface.".to_string(),
            image: "/1.png".to_string()
        },
        Product {
            id: 2,
            name: "UltraSharp 27-inch 4K Monitor".to_string(),
            price: 379.99,
            description: "Experience stunning visuals with the UltraSharp 27-inch 4K Monitor. Perfect for work, gaming, or entertainment, this monitor delivers crisp clarity and vibrant colors.".to_string(),
            image: "/2.png".to_string()
        },
        Product {
            id: 3,
            name: "Noise-Canceling Wireless Headphones".to_string(),
            price: 199.99,
            description: "Immerse yourself in music with Noise-Canceling Wireless Headphones. Enjoy superior sound quality, long battery life, and active noise cancellation for uninterrupted listening.".to_string(),
            image: "/3.png".to_string()
        },
        Product {
            id: 4,
            name: "Best Buy Gaming Laptop".to_string(),
            price: 1299.99,
            description: "Elevate your gaming experience with the Best Buy Gaming Laptop. Featuring a high-performance processor, advanced graphics, and a sleek design for competitive gaming.".to_string(),
            image: "/4.png".to_string()
        }
        
           ]
}