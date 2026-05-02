pub trait ID<T> {
    fn id(&self) -> &T;
}

pub trait ParentNode<T> {
    fn parent(&mut self) -> Option<&T>;
}

pub trait ChildrenNode<T> {
    fn children(&mut self) -> Vec<&T>;
}

#[test]
pub fn tree_test() {
    /// 节点
    #[derive(Debug)]
    pub struct Node1<T> {
        id: T,
        children: Vec<Node1<T>>,
    }

    impl<T> ID<T> for Node1<T> {
        fn id(&self) -> &T {
            &self.id
        }
    }

    impl<T> ChildrenNode<Node1<T>> for Node1<T> {
        fn children(&mut self) -> Vec<&Node1<T>> {
            self.children.iter().collect()
        }
    }

    let node2 = Node1 {
        id: 2,
        children: vec![],
    };
    let mut node1 = Node1 {
        id: 1,
        children: vec![node2],
    };
    let i = node1.id();
    println!("{}", i);
    node1.id = 3;
    node1.children = vec![];
    let vec = node1.children();
    for x in vec {
        println!("{}", x.id());
    }
}
