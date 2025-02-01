fn count_sheep(sheep: &[bool]) -> u8 {

 sheep.iter().filter(|&&present| present == true).count() as u8 


}