struct Present {
    length: i32,
    width: i32,
    height: i32,
    x_area: i32,
    y_area: i32,
    z_area: i32,
    x_circomference: i32,
    y_circomference: i32,
    z_circomference: i32,
}

impl Present {
    fn from_presentstring(text: &str) -> Self {
        let numbers: Vec<i32> = text.split("x").map(|x| x.parse::<i32>().unwrap()).collect();
        if numbers.len() != 3 { panic!() }

        Self { 
            length: numbers[0], 
            width: numbers[1], 
            height: numbers[2],
            x_area: numbers[0]*numbers[1],
            y_area: numbers[1]*numbers[2],
            z_area: numbers[2]*numbers[0],
            x_circomference: (numbers[0]+numbers[1])*2,
            y_circomference: (numbers[1]+numbers[2])*2,
            z_circomference: (numbers[2]+numbers[0])*2,
        }
    }

    fn get_required_paper(&self) -> i32 {
        let mut sides = vec![self.x_area, self.y_area, self.z_area];
        sides.sort();
        let smallest = sides.first().unwrap();

        (2*self.x_area) + (2*self.y_area) + (2*self.z_area) + smallest
    }

    fn get_ribbon_length(&self) -> i32 {
        let mut circomferences = vec![self.x_circomference, self.y_circomference, self.z_circomference];
        circomferences.sort();
        circomferences.first().unwrap().clone() + (self.length * self.width * self.height)
    }
}



fn main() {
    let present_strings: Vec<&str> = include_str!("../input.txt").split("\n").filter(|x| !x.is_empty()).collect();
    let presents: Vec<Present> = present_strings.iter().map(|x| Present::from_presentstring(x)).collect();

    let total_paper: i32 = presents.iter().map(|x| x.get_required_paper() ).sum();
    let total_ribbon_length: i32 = presents.iter().map(|x| x.get_ribbon_length()).sum();

    println!("Total paper: {total_paper}");
    println!("Total ribbon length: {total_ribbon_length}");
}
