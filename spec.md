# Language

### Standard Types

- **None**: `()`
- **bool**: `true/false`
- **int**: `(-inf,+inf)`
- **uint**: `[0,+inf)`
- **char**: `'UTF8 GOES HERE'`
- **float**: `f32 etc...`
- **string**: `"char array duh"`
    - This may be extended from array in std lib, idk
- **option**: `<this| or that>`
- **array**: `[T]`
- **struct**: `{ K1: T1, K2:T2 }`
- **tupple**: Extends struct; Key names are implied as numbers
- **functions**: `|T1 -> T2| { expr }`

# Important Feature Ideas Up Front
This is still an early project and is subject to heavy change...
### JIT Cloning
When a non-mutable reference is passed into a function, it has a change
order. If a function on a separate thread attempts to change the value, the current
memory location will split. This allows for the variable to remain in the
same memory location for as long as possible which may avoid a clone all
together. A drop also counts as a mutation. In this way, a variable that is
non-mutable in separate scope will always exist and remain the same for its
current scope.

This is something I just came up with while mowing the lawn today i.e. it might
be a GC strategy or something that already exists lol idk.

Just like rust, variables are always dropped at the end of their scope.

With functionality like this, I'm wondering if it would be possible to split
work into threads until a variable is needed. Basically this would allow for
automatic async scheduling. For instance:
```
|[int] -> int| {
    //doing a lot of stuff in here
}(a_big_array).as(temp_variable);

// do other stuff

*0.as(some_other);

*some_other += temp_variable; // implied await


//
```

### Reason for the weird function design
Functions are designed to always have "one" parameter type and one output type
so that static analysis can produce a graph of all types to perform type
searches for code synthesis.

### Functional Declarations
Variables are declared via `as(NAME)` function which is accessible to all types.
```
0.as("some_variable");

some_variable == 0; // true
```

### Mutability 
Each type has a mutable sibling type that is declared with the `*` symbol
preceding the actual type. This creates a mutable reference that is handled as
a smart pointer. There are no raw pointers in LISA; sorry low-level nerds. Declaration
with `*` allows the ability to use `*` but is still not mutable until declared
so in its instance. This is to allow for better async scheduling by forcing multi
reads and singular writes.
```
0.as(var_a);

var_a += 1; // <-- not allowed
*var_a += 1; // <-- allowed, but not shared between scopes
**var_a += 1; // <-- not allowed, mut use as_shared() method

3.as_shared(var_b);

**var_b += 1; // <-- shared variables may only be mutated when denoted with **
```

### Shared Memory Via Chaining
Lisa is designed to allow for easy parallel processing. The `as(NAME)` method
when used on `*` types can be chained to multiple variable declarations. These
will all share memory even between threads. These declarations all use RwLocks,
so be careful to avoid race conditions when using this functionality. USE WITH
CAUTION
```
1.as_shared(var_a)
  .as_shared(var_b);

**var_a += var_b;  

// var_a==2
// var_b==2
```

### locking shared memory to current value without full clone (JIT clone)
```
1.as_shared(var_a)
 .as_shared(var_b);
 .as(var_c); // var_c is a reference here


**var_a += var_b; // var_c is independent here 

var_a.as(var_d); // var_d is a reference here

**var_a += var_b; // var_d is independent here

// var_a==4
// var_b==4
// var_c==1
// var_d==2
```


### Type Extension
- Types can be built off of each other. 
- Types are declared in order; top down.
- Once a type is declared, it is immutable; a type can not be redeclared.
```
Point: {
    x: int
    y: int
};

PointList: [Point];
```

### Type parameters for functions must be a singleton (kind-of)
```
// this is not allowed:
|(int,int) -> int| {
}.as("add");

// this is allowed:

|[int] -> int| {
    *0.as("sum");
    // strap(VAR) allows for out of closure mutation by creating a shared reference
    // within the closure provided in a map or iterator function
    @.strap(sum)
     .map(|int -> int| {
        *sum += @;
        @
    });

    sum
}.as("array_sum");

array_sum([1,2,3]); // returns 6

// this is also allowed:
|{a: int, b: int} -> int| {
    @.a + @.b
}.as("add");

add({a: 1, b:2}); // returns 3
```

### Parallel Iterator functions
Due to the isolated nature of functions like `map(CLOSURE)` and other iterator
functions, most can be run in parallel. Methods like sort, although an iterator,
will not run in parallel due to ordering concerns and mutability. Parallel
iterator functions are only usable when not run as `*` type. Once `*` is
called, the iterator functions lose parallelized capabilities.  
```
// this will run in parallel
|[int] -> int| {
    *0.as("sum");
    @.strap(sum)
     .map(|int -> int| {
        *sum += @;
        @
     });

    sum
}.as("array_sum");


// this will NOT run in parallel
|[int] -> int| {
    *0.as("sum");
    *@.strap(sum)   // <-- possible mutation could occur...
      .map(|int -> int| {
        *sum += @;
        @
      });

    sum
}.as("array_sum");

([int]) -> [int] {
    // sort_by is only accessible via *
    *@.sort_by(|{a: int, b: int} -> ()| {
        if a > b {
            a
        } else {
            b
        }
    });

    @
}.as("sort_array");

```



### Type Methods - Mutable and Non-Mutable
`*` is a type override; any non-mutable method is accessible for the mutable
type.

```
*Point::|Point -> Point| {
    // can mutate self only when pointing * and fn is declared with &
    *self.x += @.x;
    *self.y += @.y;

    // return self for chain convience
    self.clone()
}.as("add_point_to_self"); // function is immutable, self is not

Point::|Point -> Point| {
    // may only reference variables
    {
        x: self.x + @.x,
        y: self.y + @.y
    }
}.as("add_points");

// usage:

*Point {
    x: 0,
    y: 1
}.as("point_a");

Point {
    x: 1,
    y: 0
}.as("point_b");

// method is not available unless pointed to
*point_a.add_point_to_self(point_b); //point_a is now (1,1)

point_a
    .add_points(point_b)
    .as("point_c");

// point_c is now (2,1) while other points remain the same
```

### implicit returns

The final return can not be implicitly denoted with the `return` keyword. I think
it looks nicer; get over it. Early implicit returns are allowed though:
```
|{a: int, b: int} -> <int, ()>| {
    // if and b are equal, return none without running next statements
    if a==b {
        return <()>;
    }

    
    // implied return is mandatory 
    if a > b {
        a
    } else {
        b
    }
}.as("max_or_none");
```

### optionals
Optionals are nice for things like error handling. They allow for any multiple of
return types; however, each type MUST be different. The only time type may
be "the same" is if the type is ghosted:
```
GhostedInt: int;


*0.as("var_a");
<42>.as("optional_num");
optional_num.is(int)
    .then((int) -> () {
        *var_a = @;
    });

// or you can assign a default
optional_num.is(int)
    .map_or(0)
    .as("var_a");

optional_num.is(GhostedInt) // <-- will not run
    .map_or(0)
    .as("var_a");

GhostedInt::from((int) -> GhostedInt {
    @
}

GhostedInt.from(0)
    .as("ghosted_int");
```
