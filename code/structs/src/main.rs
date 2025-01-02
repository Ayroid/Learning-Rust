struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // Default way

    let width1: u32 = 30;
    let height1: u32 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // Using tuple
    // Tuple is a way to group together a number of values with a variety of types into one compound type.
    // This organizes data in a way that is easy to access and manipulate. but it does not name each piece of data, making it less readable and harder to work with.

    let rect1: (u32, u32) = (30, 50);

    println!(
        "The area_tuple of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );

    // Using struct
    // Structs are a way to create more complex data types. They allow you to name and package together multiple related values that make up a meaningful group.
    // Structs are similar to tuples. Like tuples, the pieces of a struct can be different types. Unlike with tuples, you’ll name each piece of data so it’s clear what the values mean.

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    // using & we are passing a reference to the struct instead of taking ownership of it.

    println!(
        "The area_struct of the rectangle is {} square pixels.",
        area_struct(&rect2)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
