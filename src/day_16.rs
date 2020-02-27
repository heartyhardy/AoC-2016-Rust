
struct Disk{
    size:usize,
    initial:Vec<u32>,
    fit:Vec<u32>,
}

impl Disk{
    //New Disk with default size
    pub fn new()->Disk{
        let size = 20;
        let sinit = "10000";
        let mut initial:Vec<u32> = sinit.chars().map(|c|c.to_digit(10).unwrap()).collect();
        let fit:Vec<u32>=Vec::new();

        Disk{
            size,
            initial,
            fit
        }
    }

    //fill the disk
    pub fn fill(&mut self){
        if self.initial.len() >= self.size{
            return
        }

        let mut dup:Vec<u32> = self.initial.clone();
        dup.reverse();

        for d in 0..dup.len(){
            match dup[d]{
                0 => dup[d]=1,
                1 => dup[d] =0,
                _ => ()
            }
        }

        self.initial.push(0);
        self.initial.append(&mut dup);

        self.fill();
    }
}


pub fn run(){
    let mut disk = Disk::new();
    disk.fill();
    println!("{:?}", disk.initial );
}