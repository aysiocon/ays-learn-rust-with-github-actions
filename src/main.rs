use std::io;
use std::io::Write;

struct Product {
product_id : u32 ,
product_name : String ,
price : f64 ,
quantity : u32
}
struct Customer {
name : String ,
cart : Vec < String > ,
total_cost : f64
}



fn main() {

    //initialzie ang masterlist ng products with dynamic array(vector) full of "Product"-type elements
    let mut product_masterlist: Vec<Product> = Vec::new(); 

    loop {
        println!("==================== MENU ====================");
        print!("
        [1] Add a Product\n
        [2] Buy a Product\n
        [3] Edit a Product\n
        [4] Delete a Product\n
        [5] View All Products\n
        [6] View All Customers\n
        [7] Exit\n");

        let choice = get_user_input();

        match choice {
            1 => {
                println!("\nAdding a product...");
                // Call add a Product
                add_product(&mut product_masterlist);
            }
            2 => {
                println!("You selected Option 2");
                // Call Buy a Product
            }
            3 => {
                println!("You selected Option 3");
                // Call Edit a Product
            }
            4 => {
                println!("\nDeleting a product...");
                delete_product(&mut product_masterlist);
                // Call Delete a Product
            }
            5 => {
                println!("\nDisplaying all products...");
                // Call View All Product
                display_all_products(&product_masterlist);
            }
            6 => {
                println!("You selected Option 6");
                // Call View All Customers
            }
            7 => {
                println!("Thank you for using!");
                break;
            }
            _ => {//catch if out of list ang pinili
                println!("Invalid choice. Please enter a valid option from the menu (1-7 only).");
            }
        }
    }
}


//this function gets the user input
fn get_user_input() -> usize {
    loop {
        print!("Choice: ");
        io::stdout().flush().unwrap(); // display prompt
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("");

        match input.trim().parse() {
            //checks if the user_input is within the range of menu(1-7)
            Ok(number @ 1..=7) => return number, //catch if string ang input ni user
            _ => {
                println!("Please enter a valid option.");
                continue;
            }
        }
    }
}


//======================================================================


//THREE USER_INPUT STYLE
//gets user input relation with product that requires digits as input
fn ask_product_digits(prompt: &str) -> u32 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        

        //checks if the user input is actually a number
        match input.trim().parse() {
            Ok(number) => return number,
            _ => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        }
    }
}

//gets user input that are String as its datatype
fn ask_string_input(prompt: &str) -> String {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        return input;
    }
}


//======================================================================

//OPTION 1
//this fxn creates a product and checks for any duplicate inside the list
// that returns a "Product"-type of variable
fn add_product_helper(product_masterlist: &Vec<Product>) -> Product {
    println!("Please provide the following details of the product: ");

    //create product id
    let product_id = loop {
        let temp_id = ask_product_digits("Product ID: ");

        //check if product id exists

        //for all product(p),check if temp_id is not equal to any of their product_id
        if product_masterlist.iter().all(|p| p.product_id!=temp_id){
            break temp_id; //this is now the id
        }
        else{
            println!("Product ID already exists in the system. Please choose a different one");
        }
    };

    let product_name = ask_string_input("Product Name: ");
    let price = ask_product_digits("Price: ");
    let quantity = ask_product_digits("Quantity: ");

    Product {
        product_id,
        product_name: String::from(product_name),
        price: price as f64,
        quantity, //convert usize to u32 to match
    }
}

fn add_product(product_masterlist: &mut Vec<Product>) {
    //ask for product id (), name, price
    let new_product = add_product_helper(&product_masterlist);
    product_masterlist.push(new_product); //append new product
    println!("Successfully added the product!\n");
}


//OPTION 4
fn delete_product(product_masterlist: &mut Vec<Product>){
    //ask for Product ID
    let product_id = ask_product_digits("Enter product ID to delete: ");
    //check if product exists in product_masterlist
    if let Some(index) = product_masterlist.iter().position(|p| p.product_id == product_id) {
        //if it exists, delete,
        let deleted_product = product_masterlist.remove(index);
        println!("Product with ID {} has been successfully deleted!", deleted_product.product_id);
    } else {
    //else prompt error and exit function
    println!("!!! Product ID {} does not exist.", product_id);
    }
}






//OPTION 5
fn display_all_products(product_masterlist: &Vec<Product>) {
    if product_masterlist.is_empty() {
        println!("No products available.");
    } else {
        println!("PRODUCT LIST");
        println!("**************************");
        for product in product_masterlist {
            println!("Product ID: {}", product.product_id);
            println!("Product Name: {}", product.product_name);
            println!("Price: P{}", product.price);
            println!("Quantity: {}", product.quantity);
            println!("**************************");
        }
    }
    println!("*****NOTHING FOLLOWS*****\n");
}