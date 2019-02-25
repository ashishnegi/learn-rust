// Preface :
// All kind of software has been designed in C.
// All kind of software have been designed in OOPs,like C++, Java.
// So, we have been successful in designing software with all kind of paradigms or language feature set that are provided in language.
// Sum types is one such add on language feature mostly found in functional languages.

// ----------- Next ------------
// What is difference in Product types vs Sum types :

// If we think how many different runtime values can be made.
    // bool val ; // true or false : 2 values
    // struct some_struct {
    //     bool a, b;  // 2 x 2 kind values of some_struct are possible.
    // }

// This 2 x 2 leads to product types.

    // struct some_struct_2 {  // => 2^33  values
    //     Int a; // 2^32 values
    //     Bool x ; // 2 values
    // }

// --------- First ------------
// Here is an example of Sum Types.
enum TrafficLight {
    Red,
    Yello,
    Green
}

// TrafficLight looks like C++ enum.
// Well it is.. and much more.

// --------- Next ------------
// Here is another example :

enum Event {
    ClickEvent(Coordinate),
    PaintEvent(Color)
}

// this is Product Types.. Right ?
struct Color {
    red : u32,
    green : u32,
    blue : u32
}

struct Coordinate {
    x : u32,
    y : u32
}

// test : talk_handleevent

// Similar C code :

// enum EventType {
//   CLICK,
//   PAINT
// };

// struct ClickEvent {
//   int x, y;
// };
// struct PaintEvent {
//   Color color;
// };

// struct Event {
//   enum EventType type;
//   union {
//     struct ClickEvent click;
//     struct PaintEvent paint;
//   };
// };

// // given some Event event
// switch (event.type) {
//   case CLICK:
//     handleClickEvent(&event.click);
//     break;
//   case PAINT:
//     handlePaintEvent(&event.paint);
//     break;
//   // most compilers will let us know if we didn't handle every event type
// }
//
// However, there is some risk here. Nothing prevents code from accessing .paint
// in the case that type is CLICK.
// At all times, every possible field in event is visible to the programmer.

// ---------- Next ------------------

// What can go wrong in this C code ?

// User *CreateUser(int arg1, Arg2 arg2)
// {
//     If (arg1 < 0)
//     {
//         return nullPtr;
//     }

//     return new User(arg1, arg2);
// }

// auto user = CreateUser(arg1, arg2);
// user.Autenticate();

// What can go wrong ?

// ----------- Next ---------------

// How to propogate failures and not have NullPointer/ReferenceException ?
// OR Can i have the cake and eat it too ?
enum ConditionalValue<T> {
    Nothing,
    Just(T),
}

struct User {}

fn createUser(arg1 : i32, arg2 : i32) -> ConditionalValue<User> {
    if arg1 < 0 {
        ConditionalValue::Nothing
    }
    else {
        ConditionalValue::Just(User{})
    }
}

// test : talk_createuser
// What does this mean ?
// A language without pointers or references can return failure from a function
// And still be type-safe fully.
// No NullPointerExceptions if it compiles. This is not a feature.
// It is byproduct of Sum Types in language.

// ------------ Next ---------------------
// Can we do better ?

enum Result<T, E> {
    Ok(T),
    Err(E)
}

fn createUser2(arg1 : i32, arg2 : i32) -> Result<User, String> {
    if arg1 < 0 {
        Result::Err("Arg1 is less than 0".to_string())
    }
    else {
        Result::Ok(User{})
    }
}

// test : talk_createuser2

// ------------ Next ---------------------
// Can we do better ?
#[derive(Debug)]
struct AgeRange {
    min_age : i32,
    max_age : i32
}

#[derive(Debug)]
enum UserCreationError {
    AgeSmall(i32),
    AgeLarge(AgeRange),
    PasswordSmall,
    PasswordWeak,
}

fn createUser3(age: i32, password : String) -> Result<User,UserCreationError> {
    if age < 18 {
        Result::Err(UserCreationError::AgeSmall(18))
    } else if age > 150 {
        Result::Err(UserCreationError::AgeLarge(AgeRange{ min_age : 18, max_age : 150 }))
    } else if password.len() < 6 {
        Result::Err(UserCreationError::PasswordSmall)
    } else if password == "password" {
        Result::Err(UserCreationError::PasswordWeak)
    } else {
        Result::Ok(User{})
    }
}

// test : talk_createuser3

// ----------------- NEXT ------------------------

// It is not just error handling (remember the byproduct), data modelling is easier with SumTypes.

// Here is data model for Json encoder/decoders.
enum Value {
    Null,
    Bool(bool),
    Number(i32),
    String(String),
    Array(Vec<Value>),
    Object(std::collections::BTreeMap<String, Value>),
}

// How it is done in C :
// struct JSONValue {
//   JSONBoolean* b;
//   JSONString* s;
//   JSONNumber* n;
//   JSONArray* a;
//   JSONObject* o;
// }

// Want to represent protocols ?
struct Job {}

enum JobQueueCommand {
    Quit,
    LogStatus,
    RunJob(Job)
}

// Want to represent http auth ?
// Http Auth :
enum AuthMethod {
    CookieAuth(Cookie),
    SecretAuth(Secret),
    HeaderAuth(Header)
}

struct Cookie {}
struct Secret {}
struct Header {}

// ----------------- NEXT ------------------
// Anonymous User problem:

// We have a website and like every website we have a User class.
// We have 5 features.
// Now requirement comes for anonymous users to access 2 features and not access 3 features.

// Uncomment below :
// class User
// { ...
// }

// feature1(User user)
// {
//     If (user.IsAnonymous()) // this is runtime check and we can
//     {
//          return;
//     }
// ...
// }

// But real world does not have just 5 places to put a check.
// Sum Types helps us to convert the runtime check (responsibility of the programmer)
// into compile time check. If it compiles, then at places we have checked.

enum Visitor {
    Registered(User),
    Anonymous,
    // uncomment to get the compiler error.
    // same as not all enums handled error.
    // Traacked(FacebookId),
}

fn feature1(visitor : Visitor) {
    match visitor {
        Visitor::Registered(user) => {
            println!("Registered user")
        },
        Visitor::Anonymous => return ()
    }
}

struct FacebookId {}

#[cfg(test)]
mod tests {
    use super::*;

    fn handleEvent(event : Event) {
        // Compile time error if i don't pattern match.
        match event {
            Event::ClickEvent(coordinate) => handleClickEvent(coordinate),
            Event::PaintEvent(color) => handlePaintEvent(color)
        }
    }

    fn handleClickEvent(coordinate : Coordinate) {
        println!("In handle click event")
    }

    fn handlePaintEvent(color : Color) {
        println!("In handle Print event")
    }

    #[test]
    fn talk_handleevent() {
        handleEvent(Event::ClickEvent (Coordinate { x : 10, y : 20 } ));
        handleEvent(Event::PaintEvent(Color { red : 1, green : 2, blue : 3 }))
    }

    #[test]
    fn talk_createuser() {
        let user = createUser(-1, 3);
        match user { // it is compiler error to not check for if user-creation failed or succeeded.
            ConditionalValue::Nothing => println!("User creation failed."),
            ConditionalValue::Just(user) => println!("Created user !!!"),
        }

        // cargo test --tests talk_createuser -- --nocapture
        // running 1 test
        // User creation failed.
        // test talk::tests::talk_createuser ... ok
    }

    #[test]
    fn talk_createuser2() {
        let user = createUser2(-1, 3);
        match user { // it is compiler error to not check for if user-creation failed or succeeded.
            Result::Err(msg) => println!("User creation failed : {0}", msg),
            Result::Ok(user) => println!("Created user !!!"),
        }

        // cargo test --tests talk_createuser2 -- --nocapture
        // running 1 test
        // User creation failed : Arg1 is less than 0
        // test talk::tests::talk_createuser2 ... ok
    }

    #[test]
    fn talk_createuser3() {
        let user = createUser3(21, "awesomePasswORD".to_string());
        match user { // it is compiler error to not check for if user-creation failed or succeeded.
            Result::Err(error) => {
                match error {
                    UserCreationError::AgeSmall(min_age) =>
                        println!("User creation failed : Minimum age : {:?}", min_age),
                    UserCreationError::AgeLarge(ageRange) =>
                        println!("User creation failed : Minimum age : {0}, Maximum age : {1}",
                            ageRange.min_age, ageRange.max_age),
                    UserCreationError::PasswordSmall =>
                        println!("User creation failed : Password is small."),
                    UserCreationError::PasswordWeak =>
                        println!("User creation failed : Password is weak."),
                }
            },
            Result::Ok(user) => println!("Created user !!!"),
        }

        // cargo test --tests talk_createuser3 -- --nocapture
        // running 1 test
        // Created user !!!
        // test talk::tests::talk_createuser3 ... ok
    }
}

// To summarize, sum types:
//   * provide a level of type safety not available otherwise.
//   * have an efficient representation.
//   * give programmers an opportunity to clearly describe possibilities.
//   * with pattern matching, provide excellent safety guarantees.
//   * are an old idea, and are finally coming back into mainstream programming!
