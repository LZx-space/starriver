pub trait ID<T> {
    fn id(&self) -> &T;
}

pub trait ParentNode<T> {
    fn parent(&mut self) -> Option<&T>;
}

pub trait ChildrenNode<T> {
    fn children(&mut self) -> Vec<&T>;
}

/// 节点
#[derive(Debug)]
pub struct Node1<T> {
    id: T,
    name: String,
    children: Vec<Node1<T>>,
}

impl<T> ID<T> for Node1<T> {
    fn id(&self) -> &T {
        &self.id
    }
}

impl<T> ChildrenNode<Node1<T>> for Node1<T> {
    fn children(&mut self) -> Vec<&Node1<T>> {
        self.children.iter().map(|e| e).collect()
    }
}

/// 节点
#[derive(Debug)]
pub struct Node2<T> {
    id: T,
}

impl<T> ID<T> for Node2<T> {
    fn id(&self) -> &T {
        &self.id
    }
}

impl<T> ParentNode<Node2<T>> for Node2<T> {
    fn parent(&mut self) -> Option<&Node2<T>> {
        todo!()
    }
}

impl<T> ChildrenNode<Node2<T>> for Node2<T> {
    fn children(&mut self) -> Vec<&Node2<T>> {
        todo!()
    }
}

#[test]
pub fn tree_test() {
    let mut node2 = Node1 {
        id: 2,
        name: "".to_string(),
        children: vec![],
    };
    let mut node1 = Node1 {
        id: 1,
        name: "".to_string(),
        children: vec![node2],
    };
    let i = node1.id();
    println!("{}", i);
    node1.id = 3;
    node1.children = vec![];
    let vec = node1.children();
    for mut x in vec {
        println!("{}", x.id());
    }
    let mut t2 = T2 {
        id: "".to_string(),
        t_vec: vec![],
    };
    let vec1 = t2.try_1();
    let mut vec2 = t2.try_1();
    println!("{:?}", vec1);
    println!("{:?}", vec2);
    vec2[0] = "";
}

pub struct T2 {
    id: String,
    t_vec: Vec<String>,
}

impl T2 {
    pub fn t_1(&self) -> &String {
        &self.id
    }

    pub fn try_1(&self) -> Vec<&str> {
        self.t_vec.iter().map(|e| e.as_str()).collect()
    }
}
