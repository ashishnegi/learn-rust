enum TeaError {
    NoMilk,
    // NoCup,
    NoCow,
    // NoGas,
    // NoSomething,
}

fn append(s1: String, s2: String) -> String {
    s1 + s2.as_str()
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
    Ok(get_milk_from_cow(true)?
       + &pour_milk(true)?
       // + &pour_milk(true)?
       // + &pour_milk(true)?
    )
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

    #[test]
    fn append_str() {
        let (s1, s2) = (String::from("ashish "), String::from("negi"));
        assert_eq!(append(s1, s2), "ashish negi")
    }
}
