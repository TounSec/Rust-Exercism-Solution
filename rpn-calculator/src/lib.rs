#[derive(Debug, Clone, Copy)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
   let mut stack: Vec<i32> = vec![];

   for input in inputs {
    match input {
        CalculatorInput::Value(i) => stack.push(*i),
        operator => {
            if stack.len() < 2 {
                return None;

            } else {
                let op2 = stack.pop().unwrap();
                let op1 = stack.pop().unwrap();

                stack.push(calculate_expression(operator, op1, op2));
            }
        },
    }
   }
   if stack.len() != 1 {
    None

   } else {
    stack.pop()

   }
}

fn calculate_expression(operator: &CalculatorInput, op1: i32, op2: i32) -> i32 {
    match operator {
        &CalculatorInput::Add => op1 + op2,
        &CalculatorInput::Subtract => op1 - op2,
        &CalculatorInput::Multiply => op1 * op2,
        &CalculatorInput::Divide => op1 / op2,
        _ => unreachable!(),
    }
}