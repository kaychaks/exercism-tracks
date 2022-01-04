fn gett<'a>(xs: &'a [&str]) -> impl Fn((usize, usize)) -> Option<u8> + 'a {
    move |(i, j): (usize, usize)| {
        xs.get(i)?
            .chars()
            .nth(j)
            .map(|x| if x == '*' { 1 } else { 0 })
    }
}
fn neighbourhood(i: usize, j: usize, xs: &[&str]) -> Vec<Option<u8>> {
    let m_range = if i > 0 { i - 1..i + 2 } else { 0..2 };
    let n_range = if j > 0 { j - 1..j + 2 } else { 0..2 };
    let ys: Vec<(usize, usize)> = m_range
        .flat_map(|m| n_range.clone().map(move |n| (m, n)))
        .filter(|&q| (i, j) != q)
        .collect();

    ys.iter().map(|p| gett(xs)(*p)).collect()
}

fn count_neighbours(xs: &[&str], i: usize, j: usize) -> String {
    let n: u8 = neighbourhood(i, j, xs).iter().map(|n| n.unwrap_or(0)).sum();
    if n == 0 {
        String::from(" ")
    } else {
        n.to_string()
    }
}
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    Vec::from(minefield)
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let l: String = x
                .chars()
                .enumerate()
                .map(|(j, y)| {
                    if y == ' ' {
                        count_neighbours(minefield, i, j)
                    } else {
                        String::from(y)
                    }
                })
                .collect();
            l
        })
        .collect()
}
