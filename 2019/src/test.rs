use crate::solutions::*;
use crate::structures::*;
use crate::generators::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let sample = vec![1, 0, 0, 0, 99];
        assert_eq!(day2_1(&sample), 2);
    }

    #[test]
    fn test2() {
        let sample = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        assert_eq!(day2_1(&sample), 3500);
    }

    #[test]
    fn test3() {
        let hor = Line::new(Pos::new(0, 5), Pos::new(0, -5));
        let ver = Line::new(Pos::new(-5, 0), Pos::new(5, 0));
        let ver2 = Line::new(Pos::new(0, 5), Pos::new(0, 0));
        assert_eq!(hor.intersect(&ver), Some(Pos::new(0, 0)));
        assert_eq!(hor.intersect(&ver2), None); 
    }

    #[test]
    fn test4() {
        assert_eq!(check_input_2(112233), true);
        assert_eq!(check_input_2(123444), false);
        assert_eq!(check_input_2(111122), true);
    }


    #[test]
    fn test5() {
        assert_eq!((12345 % 1000) / 100, 3);
        assert_eq!((12345 % 10000) / 1000, 2);
        assert_eq!((12345 % 100000) / 10000, 1);
    }
}
