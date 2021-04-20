/* The Rust compiler has a borrow checker that compares scopes to determine whether all borrows are valid.
{
        let r;                // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          |
    }                         // ---------+
*/

// Only references can have lifetime, non-reference names can just pass ownership

/*
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
*/

/*
When annotating lifetimes in functions, the annotations go in the function signature, not in the function body.
Rust can analyze the code within the function without any help. However, when a function has references to or from code
outside that function, it becomes almost impossible for Rust to figure out the lifetimes of the parameters or return values on its own.
The lifetimes might be different each time the function is called. This is why we need to annotate the lifetimes manually.
*/

// Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.
/*
1. The first rule is that each parameter that is a reference gets its own lifetime parameter.
In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32);
a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.

2. The second rule is if there is exactly one input lifetime parameter,
that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.

3. The third rule is if there are multiple input lifetime parameters,
but one of them is &self or &mut self because this is a method,
the lifetime of self is assigned to all output lifetime parameters.
*/
macro_rules! compilation_error {
    ($s:stmt $(;)?) => {}
}

#[derive(Debug)]
struct Verbose {
    id: i32
}

impl Drop for Verbose {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

impl Clone for Verbose {
    fn clone(&self) -> Self {
        Self { id: self.id }
    }
}

#[allow(unused)]
fn main() {
    {
        fn take_by_reference(v: &Verbose) {
            println!("Taken by reference {:?}", v)
        }
        take_by_reference(&Verbose { id: 1 });
        println!("That is the problem, Verbose instance was dropped right after function call returns. We cannot return it reference from function to use.");
    }
    {
        fn take_by_reference_and_return(v: &Verbose) -> Verbose {
            println!("Taken by reference and returned dereferenced {:?}", v);
            compilation_error!(
                *v // cannot move out of `*v` which is behind a shared reference
            );
            v.clone() // Ok,
        }
        let v = take_by_reference_and_return(&Verbose { id: 2 });
        // previous value was dropped here
        println!("Value moved from function {:?}", v);
    }
    {
        // this function compiles!
        fn take_by_reference_and_return_reference(v: &Verbose) -> &Verbose {
            println!("Taken by reference and returned as is {:?}", v);
            v
        }
        compilation_error!(
            let v = take_by_reference_and_return_reference(&Verbose { id: 3 }); // temporary value dropped while borrowed
        );
        let v = Verbose { id: 3 };
        let v = take_by_reference_and_return_reference(&v); // OK
        println!("Value returned by reference from function {:?}", v);

        let v = take_by_reference_and_return_reference(&Verbose { id: 4 });
        compilation_error!(
            println!("Value returned by reference from function {:?}", v); // temporary value dropped while borrowed
        );
    }
    {
        // borrow two references and return one of them

        compilation_error!(
            fn take_by_reference_and_return_reference(v1: &Verbose, v2: &Verbose) -> &Verbose { // error: missing lifetime specifier
                println!("Taken by reference and returned as is {:?} {:?}", v1, v2);
                if v1.id < v2.id { v1 } else { v2 }
            }
        );

        // 'a is an arbitrary lifetime name, it says that all annotated variables live while the same time in outer scope
        // because function can return ether one or another.
        // And if one of values lives only while function call is not returned it can't be used after function returns it
        fn take_by_reference_and_return_reference<'a>(v1: &'a Verbose, v2: &'a Verbose) -> &'a Verbose {
            println!("Taken by reference and returned as is {:?} {:?}", v1, v2);
            if v1.id < v2.id { v1 } else { v2 }
        }

        println!("--> non of variables survives longer function call");
        let v = take_by_reference_and_return_reference(&Verbose { id: 5 }, &Verbose { id: 6 });
        compilation_error!(
            println!("Returned value {:?}", v); // temporary value dropped while borrowed
        );

        println!("--> now one of variables live longer then before function returns");
        let v = Verbose { id: 7 };
        let vvv = take_by_reference_and_return_reference(&v, &Verbose { id: 8 });
        compilation_error!(
            println!("Returned value {:?}", vvv); // temporary value dropped while borrowed
        );

        // lifetime annotation are about end of lifetime of variables, no the whole lifetime
        println!("--> now tow of variables live longer");
        let v1 = Verbose { id: 9 };
        {
            let v2 = Verbose { id: 10 };
            let vvv = take_by_reference_and_return_reference(&v1, &v2);
            println!("Returned value {:?}", vvv);
        }
    }
    {
        compilation_error!(
            fn generate_verbose_object<'a>() -> &'a Verbose {
                &Verbose { id: 11 } // cannot return reference to temporary value
            }
        );
    }
    {
        fn take_by_reference_and_return_reference<'a>(v1: &'a Verbose, v2: &'a Verbose) -> &'a Verbose {
            println!("Taken by reference and returned as is {:?} {:?}", v1, v2);
            if v1.id < v2.id { v1 } else { v2 }
        }

        // lifetime annotation are about end of lifetime of variables, no the whole lifetime
        println!("--> now tow of variables live longer, but still not enough");
        let v1 = Verbose { id: 9 };
        let vvv;
        {
            let v2 = Verbose { id: 10 };
            vvv = take_by_reference_and_return_reference(&v1, &v2);

        }
        compilation_error!(
            println!("Returned value {:?}", vvv); // `v2` does not live long enough. While v1 does, but it does not matter.
        );
    }
    {
        // structs can have lifetime annotated reference-fields
        compilation_error!(
            struct Something {
                text: &str // missing lifetime specifier
            }
        );
        #[derive(Debug)]
        struct Something<'a> {
            text: &'a str
        }
        let s = "some string";
        let smthn = Something{ text: s };
        println!("The structure {:?}", smthn); // OK, 's' lives more then 'smthn'

        let smthn_late_init;
        {
            let s = "another some string";
            smthn_late_init = Something{ text: s }; // works with string literals, because they have 'static lifetime
        }
        println!("The structure {:?}", smthn_late_init); // OK, 's' lives more then 'smthn_late_init'

        let another_smthn_late_init;
        {
            let s = "another some string".to_string();
            another_smthn_late_init = Something{ text: &s }; // borrowed value does not live long enough
        }
        compilation_error!(
            println!("The structure {:?}", another_smthn_late_init); // `s` does not live long enough
        );
    }
    {
        #[derive(Debug)]
        struct Something<'a> {
            text: &'a Verbose
        }
        let smthn_late_init;
        {
            let s = Verbose{ id: 11 };
            compilation_error!(
                smthn_late_init = Something{ text: &s }; // `s` does not live long enough
            );
        }
        smthn_late_init = Verbose{ id: 12 }; // this line just to keep compiler happy
    }
    {
        // Special lifetime 'static, which means that this reference can live for the entire duration of the program.
        // All string literals have the 'static lifetime, which we can annotate as follows:
        let s: &'static str = "the string";
    }
}
