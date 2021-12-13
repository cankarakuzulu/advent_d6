use crate::lanternfish_school::Ocean;

pub fn get_initial_school(source: Option<Vec<String>>) -> Ocean
{
    let line_source = if let Some(t) = source
    {
        t
    } else {
        panic!("Source is unusable");
    };
    let initial_state:Vec<i8> = line_source[0]
        .trim()
        .split(",")
        .map(|x| x
            .trim()
            .parse::<i8>()
            .unwrap())
        .collect();

    Ocean::from(initial_state)
}