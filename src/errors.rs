enum TeaError {
    NoMilk,
    NoCup,
    NoCow,
    NoGas,
    NoSomething,
}

fn get_milk_from_cow(cow: bool) -> Result<String, TeaError> {
    if cow {
        Ok(String::from("get milk from cow"))
    } else {
        Err(TeaError::NoCow)
    }
}

fn pour_milk(milk: bool) -> Result<String, TeaError> {
    if milk {
        Ok(String::from("poured milk"))
    } else {
        Err(TeaError::NoMilk)
    }
}

fn make_tea() -> Result<String, TeaError> {
    let mut process = String::new();
    let step_cow = get_milk_from_cow(true)?;
    let step_milk = pour_milk(true)?;
    process.push_str(step_cow.as_str());
    process.push_str(step_milk.as_str());
    Ok(process)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attemp_tea_one() {
        match pour_milk(false) {
            Err(v) => match v {
                TeaError::NoMilk => assert!(true),
                _ => assert!(false)
            },
            Ok(_) => assert!(false)
        };
    }

    #[test]
    fn attemp_tea_two() {
        match make_tea() {
            Err(_) => assert!(false),
            Ok(_) => assert!(true)
        };
    }
}
