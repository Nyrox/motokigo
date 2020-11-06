

# 元木語 - Motoki shading language

Motoki is yet another programming language targeted at programming GPUs, aiming to address the state of tooling and debugging for Shading Languages. It facilitates this by providing a simple language framework to build advanced tools with, including a VM implementation for use in debuggers.  
  
Motoki does not aim to revolutionize the way shaders are written, instead providing a simple language that should be familiar to graphic programmers and only improving on existing languages where those improvements do not pose a barrier to entry.

## Building

1. Clone the repo, cd into it
2. If you do not have cargo-script installed: `cargo install script`
3. Run `cargo script regen_globals.rs`
4. Run `cargo build`

## Features

- [x] VM Implementation
- [x] Debugging support (in-language)
- [x] GLSL Backend
- [ ] HLSL Backend (not planned)


## Language

This is what the language looks like currently. Everything open to change.

```c
in Vec3 normal

Vec3 main() {
    let L = normalize(Vec3(-0.5, 1.0, -1.0))
    let C = Vec3(1.0, 0.5, 0.5)
    
    let cos_a = dot(L, normal)
    let ambient = 0.3

    return cos_a * C + ambient * C
}
```

The basic structure of the language is a play on single-assignment form. As shaders are by their nature equivalent to pure functions, mutability in shaders is not strictly necceary. The language therefore does not allow for arbitrary writes to variables. It is likely this will be added in the future, possibly through a `mut` keyword or similar.  
  
The type system is designed to be similar to C's. The type of any expression (e.g. `foo(x, y) + 5`) is intentionally umambigious due to limited inference. As a consequence, it is not required to state the type of a variable when declaring it, the type of a variable is equal to the type of the expression assigned to it.  
  
Inputs to the shader are declared with the `in` and `uniform` keywords. These are kept for ease of conforming to standards for real shading languages. Control flow, once implement, must only depend on dynamically uniform expressions. The output of a shader is determined by the return type of main. If the return type constitutes a Struct, each field of the returned struct is converted to one shader output.

#### Language-Features

- [x] Functions, Function Overloading
- [x] Operators, Operator Overloading
- [x] Basic Type-Checking, including multiple built in types
- [x] Conditional statements (If-Else)
- [ ] Product Types (Structs)
- [ ] Loops
- [ ] Opague Types (Samplers)

## GUI-Debugger

Currently there is no standalone visual debugger, but Motoki is included in [Moto-Forestry](https://github.com/Nyrox/moto-forestry) as the primary shading language.  
A standalone debugger is likely to come in the future (maybe in the form of a shadertoy-like utility).
