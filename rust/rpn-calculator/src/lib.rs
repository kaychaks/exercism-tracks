#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

fn add(xs: &mut Vec<i32>) -> Option<i32> {
    Some(xs.pop()? + xs.pop()?)
}

fn sub(xs: &mut Vec<i32>) -> Option<i32> {
    let a = xs.pop()?;
    let b = xs.pop()?;
    Some(b - a)
}

fn mul(xs: &mut Vec<i32>) -> Option<i32> {
    Some(xs.pop()? * xs.pop()?)
}

fn div(xs: &mut Vec<i32>) -> Option<i32> {
    let a = xs.pop()?;
    let b = xs.pop()?;
    Some(b / a)
}

fn eval<F: FnMut(&mut Vec<i32>) -> Option<i32>>(
    mut f: F,
    xs: Option<&mut Vec<i32>>,
) -> Option<&mut Vec<i32>> {
    xs.and_then(|ys| {
        f(ys).map(|x| {
            ys.push(x);
            ys
        })
    })
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    inputs
        .iter()
        .fold(Some(&mut vec![]), |acc, c| match c {
            CalculatorInput::Value(n) => acc.map(|xs| {
                xs.push(*n);
                xs
            }),
            CalculatorInput::Add => eval(add, acc),
            CalculatorInput::Subtract => eval(sub, acc),
            CalculatorInput::Multiply => eval(mul, acc),
            CalculatorInput::Divide => eval(div, acc),
        })
        .and_then(|ys| if ys.len() == 1 { ys.pop() } else { None })
}
