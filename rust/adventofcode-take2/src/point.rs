use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }

    #[allow(dead_code)]
    pub fn as_tuple(&self) -> (i64, i64) {
        (self.x, self.y)
    }
}

impl Default for Point {
    fn default() -> Self {
        Point { x: 0, y: 0 }
    }
}

impl ops::Add<Point> for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::Add<i64> for Point {
    type Output = Point;

    fn add(self, rhs: i64) -> Self {
        Self::new(self.x + rhs, self.y + rhs)
    }
}

impl ops::AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::AddAssign<i64> for Point {
    fn add_assign(&mut self, rhs: i64) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl ops::Sub<Point> for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl ops::Sub<i64> for Point {
    type Output = Point;

    fn sub(self, rhs: i64) -> Self {
        Self::new(self.x - rhs, self.y - rhs)
    }
}

impl ops::SubAssign<Point> for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::SubAssign<i64> for Point {
    fn sub_assign(&mut self, rhs: i64) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl ops::Mul<Point> for Point {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl ops::Mul<i64> for Point {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl ops::MulAssign<Point> for Point {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl ops::MulAssign<i64> for Point {
    fn mul_assign(&mut self, rhs: i64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl ops::Div<Point> for Point {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl ops::Div<i64> for Point {
    type Output = Self;

    fn div(self, rhs: i64) -> Self {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl ops::DivAssign<Point> for Point {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl ops::DivAssign<i64> for Point {
    fn div_assign(&mut self, rhs: i64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_point_create() {
        assert_eq!(Point::new(1, 2).as_tuple(), (1, 2));
        assert_eq!(Point::new(3, 5).as_tuple(), (3, 5));
        assert_eq!(Point::default().as_tuple(), (0, 0));
    }

    #[test]
    fn test_point_add() {
        assert_eq!(Point::new(1, 1) + Point::new(2, 3), Point::new(3, 4));
        assert_eq!(Point::new(2, 3) + 1, Point::new(3, 4));

        let mut p = Point::new(5, 6);
        p += Point::new(1, 2);
        assert_eq!(p, Point::new(6, 8));

        let mut p = Point::new(5, 6);
        p += 2;
        assert_eq!(p, Point::new(7, 8));
    }

    #[test]
    fn test_point_sub() {
        assert_eq!(Point::new(1, 1) - Point::new(2, 3), Point::new(-1, -2));
        assert_eq!(Point::new(2, 3) - 1, Point::new(1, 2));

        let mut p = Point::new(5, 6);
        p -= Point::new(1, 2);
        assert_eq!(p, Point::new(4, 4));

        let mut p = Point::new(5, 6);
        p -= 2;
        assert_eq!(p, Point::new(3, 4));
    }

    #[test]
    fn test_point_mul() {
        assert_eq!(Point::new(3, 4) * Point::new(2, 3), Point::new(6, 12));
        assert_eq!(Point::new(3, 4) * 2, Point::new(6, 8));

        let mut p = Point::new(5, 6);
        p *= Point::new(1, 2);
        assert_eq!(p, Point::new(5, 12));

        let mut p = Point::new(5, 6);
        p *= 2;
        assert_eq!(p, Point::new(10, 12));
    }
    #[test]
    fn test_point_div() {
        assert_eq!(Point::new(12, 9) / Point::new(2, 3), Point::new(6, 3));
        assert_eq!(Point::new(12, 9) / 3, Point::new(4, 3));

        let mut p = Point::new(5, 6);
        p /= Point::new(1, 2);
        assert_eq!(p, Point::new(5, 3));

        let mut p = Point::new(10, 6);
        p /= 2;
        assert_eq!(p, Point::new(5, 3));
    }
}
