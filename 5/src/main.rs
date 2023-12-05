
#[derive(Debug)]
struct MapRecord {
    source: i64,
    destination: i64,
    length: i64
}

impl MapRecord {
    fn from_string(text: &str) -> Result<Self, &'static str> {
        let num_strings: Vec<&str> = text.split(" ").collect();
        let mut nums: Vec<i64> = Vec::new();

        for ns in num_strings {
            match ns.parse::<i64>() {
                Ok(v) => nums.push(v),
                Err(e) => continue
            }
        }

        if nums.len() != 3 {
            return Err("Record does not contain 3 numbers.")
        }



        Ok(Self { source: nums[1], destination: nums[0], length: nums[2] })
    }

    fn is_in_record(&self, num: i64) -> bool {
        if (self.source <= num) && (num < self.source+self.length) {
            return true
        } 
        return false
    }

    fn get_value(&self, num: i64) -> Option<i64> {
        if !self.is_in_record(num) {
            return None ;
        }
        let offset = num- self.source;


        return Some(self.destination+offset);


    }
}

#[test]
fn in_record_test() {
    let test_rec = MapRecord { source: 20, destination: 30, length: 4};

    assert!(test_rec.is_in_record(20));
    assert!(test_rec.is_in_record(23));
    assert!(!test_rec.is_in_record(25));
    assert!(!test_rec.is_in_record(19));
}
#[test]
fn get_value_test() {
    let test_rec = MapRecord { source: 20, destination: 30, length: 4};
    
    assert_eq!(test_rec.get_value(19), None);
    assert_eq!(test_rec.get_value(20), Some(30));
    assert_eq!(test_rec.get_value(22), Some(32));
    assert_eq!(test_rec.get_value(23), Some(33));
    assert_eq!(test_rec.get_value(24), None);
}

#[derive(Debug)]
struct Map {
    from: String,
    to: String,
    records: Vec<MapRecord>
}

impl Map {
    fn from_string(text: &str) -> Self {
        let (name, mapdata) = text.split_once(" map:\n").unwrap();
        let (from, to) = name.split_once("-to-").unwrap();

        let mut records: Vec<MapRecord> = Vec::new();
        for d in mapdata.split("\n") {
            match MapRecord::from_string(d) {
                Ok(v) => records.push(v),
                Err(e) => println!("{e}")
            }
        }


        Self { from: from.to_string(), to: to.to_string(), records }
    }

    fn translate_value(&self, num: i64) -> i64 {
        for i in &self.records {
            if i.is_in_record(num) {
                return i.get_value(num).unwrap()
            }
        }

        num
    }
}

fn main() {
    let mut maps_str: Vec<&str> = include_str!("../input.txt").split("\n\n").collect();
    let seed_list_str: &str = maps_str[0].strip_prefix("seeds: ").unwrap();
    let mut seeds: Vec<i64> = Vec::new();
    maps_str.remove(0);

    for seed in seed_list_str.split(" ") {
        match seed.parse::<i64>() {
            Ok(v) => seeds.push(v),
            Err(_) => continue
        }
    }

    let mut maps: Vec<Map> = Vec::new();
    for i in maps_str {
        maps.push(Map::from_string(i));
    }

    let mut map_idx:usize = 0;
    for map_idx in 0..maps.len() {
        for i in 0..seeds.len() {
            let translated = maps[map_idx].translate_value(seeds[i]);
            seeds[i] = translated;
        }
    }

    let mut min: i64 = seeds[0].clone();
    for i in seeds {
        if i < min {
            min=i;
        }
    }
   
    println!("{}", min)
    
}
