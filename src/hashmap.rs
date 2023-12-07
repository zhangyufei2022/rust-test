#[cfg(test)]

mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_update() {
        let teams = [("Chinese", 100), ("American", 10), ("France", 50)];

        let mut teams_map1 = HashMap::new();
        for team in &teams {
            teams_map1.insert(team.0, team.1);
        }

        // 使用两种方法实现 team_map2
        let teams_map2: HashMap<&str, i32> = teams.into_iter().collect();
        assert_eq!(teams_map1, teams_map2);

        let teams_map2: HashMap<&str, i32> = HashMap::from(teams);
        assert_eq!(teams_map1, teams_map2);

        // 覆盖已有的值
        let old = teams_map1.insert("American", 20);
        assert_eq!(old, Some(10));

        // 查询新插入的值
        let new = teams_map1.get("American");
        assert_eq!(new, Some(&20));

        // 查询对应的值，若不存在则插入新值
        let v = teams_map1.entry("France").or_insert(5);
        assert_eq!(*v, 5); // 不存在，插入5

        let v = teams_map1.entry("France").or_insert(50);
        assert_eq!(*v, 5); // 已经存在，因此50没有插入
    }

    #[test]
    fn test_query() {
        let mut scores = HashMap::new();
        scores.insert("Sunface", 98);
        scores.insert("Daniel", 95);
        scores.insert("Ashley", 69);
        scores.insert("Katie", 58);

        // get 返回一个 Option<&V> 枚举值
        let score = scores.get("Sunface");
        assert_eq!(score, Some(&98));

        if scores.contains_key("Daniel") {
            // 索引返回一个值 V
            let score = scores["Daniel"];
            assert_eq!(score, 95);
            scores.remove("Daniel");
        }
        assert_eq!(scores.len(), 3);

        for (name, score) in &scores {
            println!("The score of {} is {}", name, score)
        }
    }

    #[test]
    fn test_ownership() {
        // &str 实现了Copy，如果使用String作为key，就会发生所有权的转移，v在插入hashmap以后不能再使用
        let v = "hello";
        let mut m = HashMap::new();
        m.insert(v, 1);

        assert_eq!(v, "hello");
    }
}
