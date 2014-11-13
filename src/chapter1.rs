trait Stack<T> {
    fn is_empty(&self) -> bool;
    
    fn cons(self, item: T) -> Self;
    fn head(self) -> Option<T>;
    fn tail(self) -> Self;
}

#[allow(dead_code)]
enum CustomList<T> {
    Cons(T, Box<CustomList<T>>),
    Nil,
}

impl<T> Stack<T> for Vec<T> {
    fn is_empty(&self) -> bool {
        match self.as_slice() {
            [] => return true,
            _ => return false,
        }
    }

    fn cons(mut self, item: T) -> Vec<T> {
        self.push(item);
        self
    }

    fn head(mut self) -> Option<T> {
        self.pop()
    }

    fn tail(mut self) -> Vec<T> {
        self.pop();
        self
    }
}

#[test]
fn vec_test_is_empty() {
    let empty: Vec<uint> = Vec::new();
    assert!(empty.is_empty());
    let not_empty = vec![1u];
    assert!(!not_empty.is_empty());
}

#[test]
fn vec_test_head() {
    let empty: Vec<uint> = Vec::new();
    assert_eq!(empty.head(), None);
    let x = vec![1u].head().unwrap();
    assert_eq!(x, 1u);
}

#[test]

fn vec_test_cons() {
}

#[test]
fn vec_test_tail() {
    let empty: Vec<uint> = Vec::new() ;
    assert!(empty.tail().is_empty());
    let x = vec![1u, 2u].tail().head().unwrap();
    assert_eq!(x, 1u);
}

#[allow(dead_code)]
impl<T> Stack<T> for CustomList<T> {
    fn is_empty(&self) -> bool {
        match *self {
            Nil => return true,
            _ => return false,
        }
    }
    fn cons(self, item: T) -> CustomList<T> {
        return Cons(item, box self);
    }

    fn head(self) -> Option<T> {
        match self {
            Nil => return None,
            Cons(item, _) => return Some(item),
        }
        
    }

    fn tail(self) -> CustomList<T> {
        match self {Nil => return Nil,
            Cons(_, box tail) => return tail,
        }
    }
}

#[test]
fn custom_test_is_empty() {
    let empty_custom_list: CustomList<uint> = Nil;
    assert!(empty_custom_list.is_empty());
    let non_empty_list: CustomList<uint> = Nil.cons(1);
    assert_eq!(non_empty_list.is_empty(), false);
}

#[test]
fn custom_test_head() {
    let nil_list: CustomList<uint> = Nil;
    assert_eq!(nil_list.head(), None);
    let x = Nil.cons(1u).head().unwrap();
    assert_eq!(x, 1u);
}

#[test]
fn custom_test_tail() {
    let nil_list: CustomList<uint> = Nil;
    assert!(nil_list.tail().is_empty());
    let x = Nil.cons(1u).cons(2u).tail().head().unwrap();
    assert_eq!(x, 1u);
}

