fn main() {
    //*********** Basic Variable Assignment Practice **************
    println!("Basic Variable Assignment Practice:");

    let company_string = "TutorialsPoint";  // string type
    let rating_float = 4.5;                 // float type
    let is_growing_boolean = true;          // boolean type
    let icon_char = '♥';                    //unicode character type

    println!("company name is: {}", company_string);
    println!("company rating on 5 is: {}", rating_float);
    println!("company is growing : {}", is_growing_boolean);
    println!("company icon is: {}", icon_char);
    println!("===============================\n");


    //******************* Integer Practice ************************
    println!("Integer Practice:");

    let result = 10;    // i32 by default
    let age:u32 = 20;
    let sum:i32 = 5-15;
    let mark:isize = 10;
    let count:usize = 30;

    println!("result value is {}", result);
    println!("sum is {} and age is {}", sum, age);
    println!("mark is {} and count is {}", mark, count);
    println!("===============================\n");

    //******************* Overflow Practicce ***********************
    /*let age:u8 = 255;

    // 0 to 255 only allowed for u8
    let weight:u8 = 256;   //overflow value is 0
    let height:u8 = 257;   //overflow value is 1
    let score:u8 = 258;    //overflow value is 2

    println!("age is {} ",age);
    println!("weight is {}",weight);
    println!("height is {}",height);
    println!("score is {}",score);
    println!("===============================\n");
    */

    //********************** Float Practice ************************
    println!("Float Practice:");

    let result = 10.00;        //f64 by default
    let interest:f32 = 8.35;
    let cost:f64 = 15000.600;  //double precision
   
    println!("result value is {}",result);
    println!("interest is {}",interest);
    println!("cost is {}",cost);
    println!("===============================\n");

    //****************** Number Seperator Practice ****************
    println!("Number Seperator Practice:");

    let float_with_separator = 11_000.555_001;
    println!("float value {}",float_with_separator);

    let int_with_separator = 50_000;
    println!("int value {}",int_with_separator);
    println!("===============================\n");

    //******************* Character Practice ***********************
    println!("Character Practice:");

    let special_character = '@'; //default
    let alphabet:char = 'A';
    let emoji:char = '😁';
    
    println!("special character is {}",special_character);
    println!("alphabet is {}",alphabet);
    println!("emoji is {}",emoji);
    println!("===============================\n");   

    //***************** Variables in Rust **************************
    // By default, variables in Rust are immutable. Meaning that any variable, once assigned,
    // cannot be changed. However, you can specify that you want a variable to be mutable by
    // using the "mut" tag when initializing a variable 
    
    println!("Variables in Rust:");
   
    let mut fees:i32 = 25_000;
    println!("fees is {} ",fees);
    fees = 35_000;
    println!("fees changed is {}",fees);
    println!("===============================\n");   

    //***************** Constants in Rust *************************
    // Constants in Rust must be defined using a specific variable type, like c. You must also
    // choose the size allocated to that type. Constants, unlike variables can never be mutable. 
    // You can redefine variables and it will overwrite the original value and type used even
    // though it is declared immutable. 

    println!("Constants in Rust:");

    const USER_LIMIT:i32 = 100;    // Declare a integer constant
    const PI:f32 = 3.14;           //Declare a float constant

    println!("user limit is {}",USER_LIMIT);  //Display value of the constant
    println!("pi value is {}",PI);            //Display value of the constant
    println!("===============================\n");   

    
}
