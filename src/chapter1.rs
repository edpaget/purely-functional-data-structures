trait Stack<T> {
    fn is_empty(self) -> bool;
    
    fn cons(self, item: T) -> Stack<T>;
    fn head(self) -> T;
    fn tail(self) -> Stack<T>;
}

enum CustomList<T> {
    ConsCell(T, Box<CustomList<T>>),
    Nil,
}

impl<T> Stack<T> for CustomList<T> {
    fn is_empty(self) -> bool {
        match self {
            Nil => return true,
            _ => return false,
        }
    }
    fn cons(self, item: T) -> CustomList<T> {
        return ConsCell(item, box self);
    }

    fn head(self) -> T {
        match self {
            Nil => return Nil,
            ConsCell(item, _) => return item,
        }
        
    }

    fn tail(self) -> CustomList<T> {
        match self {
            Nil => return Nil,
            ConsCell(_, tail) => return tail,
        }
    }
}

#[test]
fn test_is_empty() {
    assert(Nil.is_empty());
}
