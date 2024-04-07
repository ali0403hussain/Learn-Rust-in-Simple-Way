# Variables and Constants
* Type of Variable
    * Rust not need to put type of variable, rust will automatically get type by its value e.g. let x =10;
* Mutability
    * By default in rust variables are immutable.
    * Immutable values are assigned once, any where in code.
    * For mutable use key word mut.
* Scope 
    * Every variable in rust retains in a specific scope that can be defined by {}.
* Shadowing
    * Its actually is the creation of two variables.
    * e.g let a = 20; let a = a + 10;
    * In the above example there are two variable, one is created first and then second is created by shadowing the other.
    * Use case e.g want to change variable after performing specific computation.
    * Use case nested scope.
* Constants
    * These are created by const key word and cant be mutated. Use Capital letters with _ for constants.