fn main() {
    print_padovan();

    {
        let point = Box::new((0.625, 0.5));
        let label = format!("{:?}", point);
        assert_eq!(label, "(0.625, 0.5)");
    }

    {
        let mut composers = Vec::new();
        composers.push(Person {
            name: "Palestrina".to_string(),
            birth: 1525,
        });
        composers.push(Person {
            name: "Dowland".to_string(),
            birth: 1563,
        });
        composers.push(Person {
            name: "Lully".to_string(),
            birth: 1632,
        });

        for composer in composers {
            println!("{}, born {}", composer.name, composer.birth);
        }
    }

    {
        let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
        let t = s.clone();
        let u = s;

        println!("{:?}", t);
        println!("{:?}", u);
    }

    {
        let mut s = "Govinda".to_string();
        let t = s;
        s = "Siddhartha".to_string();

        assert_eq!(s, "Siddhartha".to_string());
        assert_eq!(t, "Govinda".to_string());
    }

    {
        let mut v = Vec::new();
        for i in 101..106 {
            v.push(i.to_string())
        }

        let third = &v[2];
        let fifth = &v[4];

        assert_eq!(third, "103");
        assert_eq!(fifth, "105");

        let fifth = v.pop().unwrap();
        assert_eq!(fifth, "105");

        let second = v.swap_remove(1);
        assert_eq!(second, "102");

        let third = std::mem::replace(&mut v[2], "substitute".to_string());
        assert_eq!(third, "103");

        assert_eq!(v, vec!["101", "104", "substitute"]);
    }

    {
        let v = vec![
            "liberte".to_string(),
            "egalite".to_string(),
            "fraternite".to_string(),
        ];
        for mut s in v {
            s.push('!');
            println!("{}", s);
        }
    }

    {
        let mut composers = Vec::new();
        composers.push(Person2 {
            name: Some("Palestrina".to_string()),
        });

        // let first_name = composers[0].name;
        // let first_name = std::mem::replace(&mut composers[0].name, None);
        let first_name = composers[0].name.take();
        assert_eq!(first_name, Some("Palestrina".to_string()));
        assert_eq!(composers[0].name, None);
    }
}

struct Person {
    name: String,
    birth: i32,
}

struct Person2 {
    name: Option<String>,
}

fn print_padovan() {
    let mut padovan = vec![1, 1, 1];
    for i in 3..10 {
        let next = padovan[i - 3] + padovan[i - 2];
        padovan.push(next)
    }

    println!("P(1..10) = {:?}", padovan)
}
