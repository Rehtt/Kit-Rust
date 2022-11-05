// mod wireguard;

use std::net::IpAddr;
use std::str::FromStr;
// use wireguard::interface;
// use crate::size::byte::ByteSize;

mod size;

pub fn add(left: usize, right: usize) -> usize {
    // let a = "1 MiB";
    // let by = ByteSize::parse_from_string(a.to_string()).unwrap();
    // println!("{}", by.m_b().to_string());
    let a=std::net::IpAddr::from_str("");
    println!("{:?}",a.unwrap());
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        println!("{}", result);
        assert_eq!(result, 4);
    }
}
