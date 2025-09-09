#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Polyline {
    points: Vec<Point>,
}

impl Polyline {
    fn new() -> Self {
        Self { points: Vec::new() }
    }

    fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }

    fn remove_point(&mut self, point: Point) {
        self.points.retain(|&p| p != point);
    }

    fn update_point(&mut self, point_index: usize, new_point: Point) {
        if let Some(point) = self.points.get_mut(point_index) {
            *point = new_point;
        }
    }

    fn get_point(&self, point_index: usize) -> Option<&Point> {
        self.points.get(point_index)
    }

    fn get_points(&self) -> Vec<Point> {
        self.points.clone()
    }
}

fn main() {
    // Polyline
    let mut polyline = Polyline::new();

    // Points
    let point_1 = Point { x: 0, y: 0 };
    let point_2 = Point { x: 10, y: 7 };
    let point_3 = Point { x: -2, y: 3 };
    let point_4 = Point { x: 100, y: -100 };

    // Add a couple points
    polyline.add_point(point_1);
    polyline.add_point(point_2);
    polyline.add_point(point_3);
    polyline.add_point(point_4);

    // Get points from the polyline `get_point` method
    let get_point_1 = polyline.get_point(0);
    let get_point_2 = polyline.get_point(1);
    let get_point_3 = polyline.get_point(2);
    let get_point_4 = polyline.get_point(3);

    // Make some assertions on the gotten points
    assert_eq!(point_1, *get_point_1.unwrap(), "Point 1 should be the same");
    assert_eq!(point_2, *get_point_2.unwrap(), "Point 2 should be the same");
    assert_eq!(point_3, *get_point_3.unwrap(), "Point 3 should be the same");
    assert_eq!(point_4, *get_point_4.unwrap(), "Point 4 should be the same");

    // Get all points
    let all_points = polyline.get_points();

    // Make some assertion on all points
    assert_eq!(all_points.len(), 4, "There should be 4 points");
    assert_eq!(
        all_points,
        vec![point_1, point_2, point_3, point_4],
        "All points should match the array of all points added so far"
    );

    // Updated point
    let new_point_1 = Point { x: 100, y: 100 };
    polyline.update_point(0, new_point_1);

    // Make assertion on the updated point
    assert_eq!(
        new_point_1,
        *polyline.get_point(0).unwrap(),
        "Point should be equal to the new point"
    );

    // Remove some points
    polyline.remove_point(new_point_1);

    // Get all points
    let all_points = polyline.get_points();

    // Make some assertion on all points
    assert_eq!(all_points.len(), 3, "There should be 3 points");
    assert_eq!(
        all_points,
        vec![point_2, point_3, point_4],
        "All points should match the array of updated points"
    );

    // Get an invalid point
    let get_invalid_point = polyline.get_point(12);

    // Assert that the invalid point returns None
    assert!(get_invalid_point.is_none(), "Removed point should be None");
}
