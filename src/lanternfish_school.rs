use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

pub struct Lanternfish
{
    pub remaining_until_next_cycle: i8,
}

impl Lanternfish {
    pub fn new() -> Lanternfish
    {
        Lanternfish { remaining_until_next_cycle: 8 }
    }

    pub fn from_remaining(days:i8) -> Lanternfish
    {
        Lanternfish{remaining_until_next_cycle:(days)}
    }

    pub fn reset(&mut self)
    {
        self.remaining_until_next_cycle=6;
    }

    pub fn process_the_day(&mut self) -> Option<Lanternfish>
    {
        self.remaining_until_next_cycle -= 1;
        if self.remaining_until_next_cycle < 0
        {
            self.reset();
            Some(Lanternfish::new())
        } else {
            None
        }
    }
}

pub struct Ocean
{
    pub fishes:Vec<Lanternfish>,
    pub days_passed:u32
}

impl Ocean {
    pub fn from(source:Vec<i8>) -> Ocean
    {
        Ocean{ fishes:source.iter().map(|x| Lanternfish::from_remaining(*x)).collect(), days_passed:0}
    }

    pub fn spend_one_day(&mut self)
    {
        let new:Vec<Lanternfish> = self.fishes.par_iter_mut().map(|f| f.process_the_day()).filter(|x| x.is_some()).map(|x| match x {
            Some(t)=>{
                t
            },
            None => Lanternfish{ remaining_until_next_cycle: 0 }}).collect();
        self.fishes.splice(1..1, new);
        self.days_passed+=1;
    }

    pub fn count(&self) -> usize
    {
        self.fishes.len()
    }
}