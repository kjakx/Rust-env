fn main()
{
    let s = String::from("Hello Rust world!");
    let first_word = first_word(&s[..]);
    let second_word = second_word(&s[..]);
    //s.clear();
    println!("{}", first_word);
    println!("{}", second_word);
}

fn first_word(s: &str) -> &str
{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate()
    {
        if item == b' '
        {
            return &s[..i];
        }
    }

    &s[..]
}

fn second_word(s: &str) -> &str
{
    let bytes = s.as_bytes();
    let mut start: usize = 0;
    let mut end: usize = s.len();
    for (i, &item) in bytes.iter().enumerate()
    {
        if item == b' '
        {
            if start == 0
            {
                start = i + 1;
            }
            else
            {
                end = i;
                break;
            }
        }
    }
    &s[start..end]
}
