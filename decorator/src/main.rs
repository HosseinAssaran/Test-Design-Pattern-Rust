// Define a trait representing the base functionality
trait Component {
    fn operation(&self) -> String;
}

// Define a struct implementing the base functionality
struct ConcreteComponent;

impl Component for ConcreteComponent {
    fn operation(&self) -> String {
        String::from("ConcreteComponent: operation()")
    }
}

// Define a macro to generate decorator structs and their implementations
macro_rules! decorator {
    ($name:ident) => {
        struct $name<T: Component> {
            component: T,
        }

        impl<T: Component> $name<T> {
            fn new(component: T) -> Self {
                $name { component }
            }
        }

        impl<T: Component> Component for $name<T> {
            fn operation(&self) -> String {
                format!("{}: operation({})", stringify!($name), self.component.operation())
            }
        }
    };
}

// Use the macro to generate decorators
decorator!(Decorator1);
decorator!(Decorator2);
decorator!(Decorator3);


fn main() {
    let component = ConcreteComponent;
    let decorated_component =Decorator3::new( Decorator2::new(Decorator1::new(component)));

    println!("{}", decorated_component.operation());
}
