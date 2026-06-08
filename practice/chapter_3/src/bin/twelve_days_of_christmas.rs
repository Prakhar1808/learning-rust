fn main() {
    println!("The Tweleve Days of Christmas");
    println!("A Capitalist Society
        ");

    let gifts = ["And a partridge in a pear tree.",
"Two turtle doves",
"Three French hens",
"Four calling birds",
"Five golden rings",
"Six geese a-laying",
"Seven swans a-swimming",
"Eight maids a-milking",
"Nine ladies dancing",
"Ten lords a-leaping",
"Eleven pipers piping",
"Twelve drummers drumming"];

    for day in 1..=12 {
        let suffix = if day == 1 {"st"}
            else if day == 2 {"nd"}
            else if day == 3 {"rd"}
            else {"th"};

    println!("On the {day}{suffix} day of Christmas
My true love gave to me");

        for gift_num in (1..=day).rev() {
            let index = (gift_num - 1) as usize;
            if gift_num == 1 && day != 1 {
                // understand this edge case
                println!("A partridge in a pear tree.");
            }else {
                println!("{}", gifts[index]);
            }
        }
        println!();
    }
}
