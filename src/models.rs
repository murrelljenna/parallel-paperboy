struct Destination(Vec2);
struct Path(Vec<Vec2>);


struct Paperboy();

impl Paperboy {
    fn isInRange(self, destination: Destination) -> bool {
        true
    }

    fn start(self, path: Path) {}
}