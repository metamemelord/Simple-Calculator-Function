use fdk::{Function, RuntimeContext};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(serde::Deserialize)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Deserialize)]
struct CalculatorInput {
    operand1: f64,
    operand2: f64,
    operation: Operation,
}

#[derive(Serialize)]
struct CalculatorResult {
    result: f64,
}

impl CalculatorResult {
    fn new(result: f64) -> Self {
        Self { result }
    }
}

fn simple_calc(
    _: &mut RuntimeContext,
    input: CalculatorInput,
) -> Result<CalculatorResult, Box<dyn Error>> {
    Ok(match input.operation {
        Operation::Add => CalculatorResult::new(input.operand1 + input.operand2),
        Operation::Sub => CalculatorResult::new(input.operand1 - input.operand2),
        Operation::Mul => CalculatorResult::new(input.operand1 * input.operand2),
        Operation::Div => CalculatorResult::new(input.operand1 / input.operand2),
    })
}

#[tokio::main]
async fn main() {
    let function = Function::run(simple_calc);
    if let Err(e) = function.await {
        eprintln!("{}", e);
    };
}
