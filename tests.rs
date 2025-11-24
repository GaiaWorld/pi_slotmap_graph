#[cfg(test)]
mod integration_tests {
    use graph_api_lib::{Element, Graph, EdgeReference, EdgeReferenceMut, VertexReference, VertexReferenceMut};
    use crate::slotmap_graph::SlotMapGraph;

    #[derive(Debug, Clone)]
    struct Person {
        name: String,
        age: u32,
    }

    #[derive(Debug, Clone)]
    struct Friendship {
        years_known: u32,
    }

    impl Element for Person {
        type Label = ();

        fn label(&self) -> Self::Label {
            ()
        }
    }

    impl Element for Friendship {
        type Label = ();

        fn label(&self) -> Self::Label {
            ()
        }
    }

    #[test]
    fn test_complete_graph_workflow() {
        let mut graph = SlotMapGraph::<Person, Friendship>::new();

        // 添加人员
        let alice = graph.add_vertex(Person {
            name: "Alice".to_string(),
            age: 30,
        });
        let bob = graph.add_vertex(Person {
            name: "Bob".to_string(),
            age: 25,
        });
        let charlie = graph.add_vertex(Person {
            name: "Charlie".to_string(),
            age: 35,
        });

        // 建立友谊关系
        let alice_bob = graph.add_edge(alice, bob, Friendship { years_known: 5 });
        let bob_charlie = graph.add_edge(bob, charlie, Friendship { years_known: 10 });
        let alice_charlie = graph.add_edge(alice, charlie, Friendship { years_known: 3 });

        // 验证图结构
        assert_eq!(graph.vertex_count(), 3);
        assert_eq!(graph.edge_count(), 3);

        // 验证人员信息
        let alice_ref = graph.vertex(alice).unwrap();
        assert_eq!(alice_ref.weight().name, "Alice");
        assert_eq!(alice_ref.weight().age, 30);

        // 验证友谊关系
        let friendship_ref = graph.edge(alice_bob).unwrap();
        assert_eq!(friendship_ref.weight().years_known, 5);
        assert_eq!(friendship_ref.tail(), alice);
        assert_eq!(friendship_ref.head(), bob);

        // 测试Alice的朋友 - 修复方法调用
        let alice_friends: Vec<_> = graph.outgoing_edges(alice)
            .filter_map(|edge_ref| {
                graph.vertex(edge_ref.head()).map(|v| v.weight().name.clone())
            })
            .collect();
        assert!(alice_friends.contains(&"Bob".to_string()));
        assert!(alice_friends.contains(&"Charlie".to_string()));

        // 测试Bob的入度
        assert_eq!(graph.in_degree(bob), 1);
        assert_eq!(graph.out_degree(bob), 1);

        // 测试遍历所有人员
        let people: Vec<String> = graph.all_vertices()
            .map(|(_, person)| person.name.clone())
            .collect();
        assert_eq!(people.len(), 3);
        assert!(people.contains(&"Alice".to_string()));
        assert!(people.contains(&"Bob".to_string()));
        assert!(people.contains(&"Charlie".to_string()));
    }

    #[test]
    fn test_graph_clear() {
        let mut graph = SlotMapGraph::<Person, Friendship>::new();

        // 添加一些数据
        let v1 = graph.add_vertex(Person { name: "A".to_string(), age: 10 });
        let v2 = graph.add_vertex(Person { name: "B".to_string(), age: 20 });
        graph.add_edge(v1, v2, Friendship { years_known: 1 });

        assert_eq!(graph.vertex_count(), 2);
        assert_eq!(graph.edge_count(), 1);

        // 清空图
        graph.clear();

        assert_eq!(graph.vertex_count(), 0);
        assert_eq!(graph.edge_count(), 0);
        assert!(graph.is_empty());
    }

    #[test]
    fn test_edge_between_query() {
        let mut graph = SlotMapGraph::<Person, Friendship>::new();

        let v1 = graph.add_vertex(Person { name: "A".to_string(), age: 10 });
        let v2 = graph.add_vertex(Person { name: "B".to_string(), age: 20 });

        // 添加多条相同方向的边
        let e1 = graph.add_edge(v1, v2, Friendship { years_known: 1 });
        let e2 = graph.add_edge(v1, v2, Friendship { years_known: 5 });

        // 查询边
        assert!(graph.has_edge(v1, v2));
        assert!(!graph.has_edge(v2, v1));

        let edges_between: Vec<_> = graph.edges_between(v1, v2).collect();
        assert_eq!(edges_between.len(), 2);
    }

    #[test]
    fn test_mutable_operations() {
        let mut graph = SlotMapGraph::<Person, Friendship>::new();

        let v1 = graph.add_vertex(Person {
            name: "Alice".to_string(),
            age: 30,
        });
        let v2 = graph.add_vertex(Person {
            name: "Bob".to_string(),
            age: 25,
        });

        let e1 = graph.add_edge(v1, v2, Friendship { years_known: 5 });

        // 修改顶点数据
        if let Some(mut vertex_ref) = graph.vertex_mut(v1) {
            vertex_ref.weight_mut().age = 31;
        }

        // 验证修改
        assert_eq!(graph.vertex(v1).unwrap().weight().age, 31);

        // 修改边数据
        if let Some(mut edge_ref) = graph.edge_mut(e1) {
            edge_ref.weight_mut().years_known = 6;
        }

        // 验证修改
        assert_eq!(graph.edge(e1).unwrap().weight().years_known, 6);
    }
}