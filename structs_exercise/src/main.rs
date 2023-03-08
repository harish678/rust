#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


fn normal_area(width: u32, height: u32) -> u32 {
    width * height
}

fn tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn struct_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let width = 30;
    let height = 50;

    println!("Area of rectangle: {}", normal_area(width, height));

    let dimensions = (30, 50);
    println!("Area of rectangle: {}", tuple_area(dimensions));

    let scale = 2;
    let rect = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!("Area of rectangle: {}", struct_area(&rect));

    println!("rect is {:#?}", rect);

    dbg!(&rect);
}
