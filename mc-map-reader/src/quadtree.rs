//! https://github.com/dmac/rust-quadtree/blob/master/src/quadtree.rs
//! A data structure for inserting and querying elements located in 2D space.

use std::{fmt, ops::Deref};

/// A data structure for inserting and querying elements located in 2D space.
///
/// Querying returns references to the inserted objects.
///
/// # Examples
///
/// ```rust
/// extern crate quadtree;
///
/// use quadtree::{QuadTree, Bounded, Bounds};
///
/// #[deriving(Eq)]
/// struct Rect {
///     x: f32,
///     y: f32,
///     width: f32,
///     height: f32
/// }
///
/// impl Bounded for Rect {
///     fn bounds(&self) -> Bounds {
///         Bounds{x: self.x, y: self.y, width: self.width, height: self.height}
///     }
/// }
///
/// fn main() {
///     let mut qt = QuadTree::new(Bounds{x: 0., y: 0., width: 8., height: 8.});
///     let r1 = &Rect{x: 0., y: 0., width: 2., height: 2.};
///     let r2 = &Rect{x: 3., y: 3., width: 1., height: 1.};
///     let r3 = &Rect{x: 1., y: 1., width: 1., height: 1.};
///     qt.insert(r1);
///     qt.insert(r2);
///     qt.insert(r3);
///     let results: Vec<&Rect> = qt.query(r3).collect();
///     assert!(results.contains(&r1));
/// }
/// ```
pub struct QuadTree<'a, T> {
    capacity: usize,
    depth: usize,
    max_depth: usize,
    bounds: Bounds,
    elements: Vec<&'a T>,
    children: Option<[Box<QuadTree<'a, T>>; 4]>,
}

/// A bounded area represented by x, y, width, and height.
#[derive(PartialEq, Debug)]
pub struct Bounds {
    /// x coordinate
    pub x: f32,
    /// y coordinate
    pub y: f32,
    /// width
    pub width: f32,
    /// height
    pub height: f32,
}

/// Elements of the quadtree must implement this trait.
pub trait Bounded {
    /// Returns a bounded area.
    fn bounds(&self) -> Bounds;
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Quadrant {
    TL,
    TR,
    BR,
    BL,
}

impl<'a, T: Bounded> QuadTree<'a, T> {
    /// Constructs a new quadtree containing the specified bounds.
    pub fn new(bounds: Bounds) -> QuadTree<'a, T> {
        QuadTree {
            capacity: 4,
            max_depth: 10,
            depth: 0,
            bounds: bounds,
            elements: Vec::new(),
            children: None,
        }
    }

    // quadrant:
    //   children:
    //     children[quadrant].push
    //   no children:
    //     self.push
    //     split
    // no quadrant:
    //    children:
    //      self.push
    //    no children:
    //      self.push
    //      split
    /// Inserts an element into the quadtree.
    pub fn insert(&mut self, element: &'a T) {
        match (self.get_quadrant(element), self) {
            (
                Some(q),
                QuadTree {
                    children: Some(children),
                    ..
                },
            ) => {
                children[q as usize].insert(element);
            }
            (
                None,
                _self @ QuadTree {
                    children: Some(_), ..
                },
            ) => {
                _self.elements.push(element);
            }
            (_, _self @ QuadTree { children: None, .. }) => {
                _self.elements.push(element);

                if _self.elements.len() > _self.capacity {
                    _self.split();
                }
            }
        }
    }

    /// Returns an iterator over elements near a given element, which may or may not be in the quadtree.
    pub fn query(&'a self, element: &'a T) -> QueryItems<'a, T> {
        QueryItems {
            qt: self,
            index: 0,
            element: element,
            next_qts: Vec::new(),
        }
    }

    /// Returns an iterator over all elements in the quadtree.
    pub fn iter(&'a self) -> Items<'a, T> {
        Items {
            root: self,
            quadrants: Vec::new(),
            element_index: 0,
        }
    }

    fn split(&mut self) {
        if self.depth >= self.max_depth {
            return;
        }

        match self.children {
            Some(_) => unreachable!(),
            None => {
                let mut children = [
                    Box::new(QuadTree {
                        capacity: self.capacity,
                        depth: self.depth + 1,
                        max_depth: self.max_depth,
                        bounds: Bounds {
                            x: self.bounds.x,
                            y: self.bounds.y,
                            width: self.bounds.width / 2.0,
                            height: self.bounds.height / 2.0,
                        },
                        elements: Vec::new(),
                        children: None,
                    }),
                    Box::new(QuadTree {
                        capacity: self.capacity,
                        depth: self.depth + 1,
                        max_depth: self.max_depth,
                        bounds: Bounds {
                            x: self.bounds.x + self.bounds.width / 2.0,
                            y: self.bounds.y,
                            width: self.bounds.width / 2.0,
                            height: self.bounds.height / 2.0,
                        },
                        elements: Vec::new(),
                        children: None,
                    }),
                    Box::new(QuadTree {
                        capacity: self.capacity,
                        depth: self.depth + 1,
                        max_depth: self.max_depth,
                        bounds: Bounds {
                            x: self.bounds.x + self.bounds.width / 2.0,
                            y: self.bounds.y + self.bounds.height / 2.0,
                            width: self.bounds.width / 2.0,
                            height: self.bounds.height / 2.0,
                        },
                        elements: Vec::new(),
                        children: None,
                    }),
                    Box::new(QuadTree {
                        capacity: self.capacity,
                        depth: self.depth + 1,
                        max_depth: self.max_depth,
                        bounds: Bounds {
                            x: self.bounds.x,
                            y: self.bounds.y + self.bounds.height / 2.0,
                            width: self.bounds.width / 2.0,
                            height: self.bounds.height / 2.0,
                        },
                        elements: Vec::new(),
                        children: None,
                    }),
                ];

                let mut new_elements: Vec<&T> = Vec::new();
                for &element in self.elements.iter() {
                    match self.get_quadrant(element) {
                        Some(i) => children[i as usize].insert(element),
                        None => new_elements.push(element),
                    };
                }

                self.children = Some(children);
                self.elements = new_elements;
            }
        }
    }

    fn get_quadrant(&self, r: &T) -> Option<Quadrant> {
        let half_width = self.bounds.x + (self.bounds.width / 2.0);
        let half_height = self.bounds.y + (self.bounds.height / 2.0);

        let fits_left_half =
            r.bounds().x >= self.bounds.x && r.bounds().x + r.bounds().width < half_width;
        let fits_right_half = r.bounds().x >= half_width
            && r.bounds().x + r.bounds().width < self.bounds.x + self.bounds.width;
        let fits_top_half =
            r.bounds().y >= self.bounds.y && r.bounds().y + r.bounds().height < half_height;
        let fits_bottom_half = r.bounds().y >= half_height
            && r.bounds().y + r.bounds().height < self.bounds.y + self.bounds.height;

        if fits_top_half && fits_left_half {
            Some(Quadrant::TL)
        } else if fits_top_half && fits_right_half {
            Some(Quadrant::TR)
        } else if fits_bottom_half && fits_right_half {
            Some(Quadrant::BR)
        } else if fits_bottom_half && fits_left_half {
            Some(Quadrant::BL)
        } else {
            None
        }
    }

    fn contains(&self, r: &T) -> bool {
        r.bounds().x >= self.bounds.x
            && r.bounds().x + r.bounds().width < self.bounds.width
            && r.bounds().y >= self.bounds.y
            && r.bounds().y + r.bounds().height < self.bounds.height
    }
}

impl<'a, T: Bounded> QuadTree<'a, T> {
    fn len(&self) -> usize {
        let mut count = self.elements.len();
        match self.children {
            Some(ref children) => {
                for child in children.iter() {
                    count += child.len();
                }
            }
            None => {}
        };
        count
    }
}

impl<'a, T: Bounded> QuadTree<'a, T> {
    fn clear(&mut self) {
        self.elements.clear();
        match self {
            QuadTree {
                children: Some(_), ..
            } => {
                match self.children {
                    Some(ref mut children) => {
                        for child in children.iter_mut() {
                            child.clear();
                        }
                    }
                    None => {}
                }
                self.children = None;
            }
            _ => {}
        }
    }
}

impl<'a, T: Bounded + fmt::Debug> fmt::Debug for QuadTree<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for _ in 0..self.depth {
            write!(f, "    ")?
        }

        self.elements.fmt(f)?;

        match self.children {
            Some(ref children) => {
                for child in children.iter() {
                    write!(f, "\n");
                    child.fmt(f);
                }
            }
            None => {}
        };
        Ok(())
    }
}

/// An iterator over all elements in the quadtree.
pub struct Items<'a, T> {
    root: &'a QuadTree<'a, T>,
    quadrants: Vec<Quadrant>,
    element_index: usize,
}

/// An iterator over elements near a given query element.
pub struct QueryItems<'a, T> {
    qt: &'a QuadTree<'a, T>,
    index: usize,
    element: &'a T,
    next_qts: Vec<&'a QuadTree<'a, T>>,
}

impl<'a, T: Bounded> Iterator for Items<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        let mut node = self.root;
        for quadrant in self.quadrants.iter() {
            match &node.children {
                &Some(ref children) => node = &*children[*quadrant as usize],
                &None => unreachable!(),
            }
        }

        if self.element_index < node.elements.len() {
            let element = node.elements.get(self.element_index);
            self.element_index += 1;
            return element.map(Deref::deref);
        }

        match node.children {
            Some(_) => {
                self.quadrants.push(Quadrant::TL);
                self.element_index = 0;
                self.next()
            }
            None => {
                let mut last_index = Quadrant::BL;
                while last_index == Quadrant::BL {
                    match self.quadrants.pop() {
                        Some(i) => last_index = i,
                        None => return None,
                    };
                }
                self.quadrants.push(match last_index {
                    TL => Quadrant::TR,
                    TR => Quadrant::BR,
                    BR => Quadrant::BL,
                    BL => unreachable!(),
                });
                self.element_index = 0;
                self.next()
            }
        }
    }
}

impl<'a, T: Bounded> Iterator for QueryItems<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        if self.index < self.qt.elements.len() {
            let e = self.qt.elements.get(self.index);
            self.index += 1;
            return e.map(Deref::deref);
        }

        match (self.qt.get_quadrant(self.element), &self) {
            (
                Some(q),
                QueryItems {
                    qt:
                        &QuadTree {
                            children: Some(ref children),
                            ..
                        },
                    ..
                },
            ) => {
                self.qt = &*children[q as usize];
                self.index = 0;
                self.next()
            }
            (
                None,
                QueryItems {
                    qt:
                        &QuadTree {
                            children: Some(ref children),
                            ..
                        },
                    ..
                },
            ) => {
                // This handles the case where the query element lies outside the bounds of the entire
                // quadtree.
                if !self.qt.contains(self.element) {
                    return None;
                }
                self.qt = &*children[Quadrant::TL as usize];
                self.next_qts.push(&*children[Quadrant::TR as usize]);
                self.next_qts.push(&*children[Quadrant::BR as usize]);
                self.next_qts.push(&*children[Quadrant::BL as usize]);
                self.index = 0;
                self.next()
            }
            (
                _,
                QueryItems {
                    qt: &QuadTree { children: None, .. },
                    ..
                },
            ) => match self.next_qts.pop() {
                Some(qt) => {
                    self.qt = qt;
                    self.index = 0;
                    self.next()
                }
                None => None,
            },
        }
    }
}

#[cfg(test)]
#[derive(PartialEq, Debug)]
struct TestRect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[cfg(test)]
impl Bounded for TestRect {
    fn bounds(&self) -> Bounds {
        Bounds {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }
}

#[cfg(test)]
fn new_test_quadtree<'a>() -> QuadTree<'a, TestRect> {
    QuadTree::new(Bounds {
        x: 0.0,
        y: 0.0,
        width: 16.0,
        height: 16.0,
    })
}

#[test]
fn insert() {
    let mut qt = new_test_quadtree();
    let r1 = &TestRect {
        x: 0.0,
        y: 0.0,
        width: 1.0,
        height: 1.0,
    };
    let r2 = &TestRect {
        x: 14.0,
        y: 14.0,
        width: 1.0,
        height: 1.0,
    };
    qt.insert(r1);
    qt.insert(r2);
    let rects: Vec<&TestRect> = qt.iter().collect();
    assert!(rects == vec!(r1, r2));
    assert!(2 == qt.len());
}

#[test]
fn query() {
    let mut qt = new_test_quadtree();
    let r1 = &TestRect {
        x: 0.0,
        y: 0.0,
        width: 1.0,
        height: 1.0,
    };
    let r2 = &TestRect {
        x: 4.0,
        y: 4.0,
        width: 8.0,
        height: 8.0,
    };
    let r3 = &TestRect {
        x: 9.0,
        y: 9.0,
        width: 1.0,
        height: 1.0,
    };
    let r4 = &TestRect {
        x: 14.0,
        y: 14.0,
        width: 1.0,
        height: 1.0,
    };
    qt.insert(r1);
    qt.insert(r2);
    qt.insert(r3);
    let results: Vec<&TestRect> = qt.query(r4).collect();
    assert!(results.contains(&r2) && results.contains(&r3));
}

#[test]
fn clear() {
    let mut qt = new_test_quadtree();
    let r1 = &TestRect {
        x: 0.0,
        y: 0.0,
        width: 1.0,
        height: 1.0,
    };
    let r2 = &TestRect {
        x: 4.0,
        y: 4.0,
        width: 8.0,
        height: 8.0,
    };
    qt.insert(r1);
    qt.insert(r2);
    assert!(2 == qt.len());
    qt.clear();
    assert!(0 == qt.len());
}
