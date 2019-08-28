use std::collections::HashMap;

macro_rules! map {
    (
        $( $key:expr => $value:expr ),*
    ) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

fn main(){
    let mut map1 = HashMap::new();
        map1.insert(1, "one");
        map1.insert(2, "two");
        map1.insert(3, "three");

    let map2 = {
        let mut map = HashMap::new();
            map.insert(1, "one");
            map.insert(2, "two");
            map.insert(3, "three");
        map
    };

    let map3 = map!(
        1 => "one",
        2 => "two",
        3 => "three"
    );

    assert_eq!(map1, map2);
    assert_eq!(map1, map3);
}
